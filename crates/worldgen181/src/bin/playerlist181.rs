use worldgen::{JavaRandom,entity::player::list::PlayerList};
fn int(out:&mut Vec<u8>,v:i32){out.extend(v.to_le_bytes());}
fn value(out:&mut Vec<u8>,v:Option<i32>){int(out,v.unwrap_or(i32::MIN));}
fn main() {
    let args:Vec<_>=std::env::args().collect();let mut out=vec![];
    let mut list=PlayerList::<Option<i32>>::default();let mut random=JavaRandom::new(181);
    for tick in 0..3000 {
        let op=random.next_int(5);let key=(random.next_int(128)-64) as i64 | (((random.next_int(5)-2) as i64)<<32);
        let v=if tick%7==0 {None} else {Some(random.next_int_unbounded())};
        match op {
            0|1=>list.insert(key,v),
            2=>value(&mut out,list.remove(key).flatten()),
            3=>value(&mut out,list.get(key).copied().flatten()),
            _=>out.push(list.contains(key) as u8),
        }
        for v in [list.len(),list.threshold(),list.modifications(),list.buckets().len() as i32] {int(&mut out,v);}
        for bucket in list.buckets() {
            int(&mut out,bucket.len() as i32);
            for entry in bucket {out.extend(entry.key.to_le_bytes());value(&mut out,entry.value);int(&mut out,entry.hash);}
        }
    }
    std::fs::write(&args[1],&out).unwrap();
    if let Some(expected)=args.get(2) {assert_eq!(out,std::fs::read(expected).unwrap(),"PlayerList differs");println!("PLAYER LIST EXACT: 3000 operations, duplicates, nulls, collisions, resizes and entries");}
}
