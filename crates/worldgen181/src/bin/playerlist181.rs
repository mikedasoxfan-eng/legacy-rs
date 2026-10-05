use worldgen::{
    entity::player::list::{hash, PlayerList, PlayerListEntry},
    JavaRandom,
};
fn int(out: &mut Vec<u8>, v: i32) {
    out.extend(v.to_le_bytes());
}
fn value(out: &mut Vec<u8>, v: Option<i32>) {
    int(out, v.unwrap_or(i32::MIN));
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mut out = vec![];
    let mut list = PlayerList::<Option<i32>>::default();
    let mut random = JavaRandom::new(181);
    for tick in 0..3000 {
        let op = random.next_int(5);
        let key = (random.next_int(128) - 64) as i64 | (((random.next_int(5) - 2) as i64) << 32);
        let v = if tick % 7 == 0 {
            None
        } else {
            Some(random.next_int_unbounded())
        };
        match op {
            0 | 1 => list.insert(key, v),
            2 => value(&mut out, list.remove(key).flatten()),
            3 => value(&mut out, list.get(key).copied().flatten()),
            _ => out.push(list.contains(key) as u8),
        }
        for v in [
            list.len(),
            list.threshold(),
            list.modifications(),
            list.buckets().len() as i32,
        ] {
            int(&mut out, v);
        }
        for bucket in list.buckets() {
            int(&mut out, bucket.len() as i32);
            for entry in bucket {
                out.extend(entry.key.to_le_bytes());
                value(&mut out, entry.value);
                int(&mut out, entry.hash);
            }
        }
    }
    for c in 0..10000 {
        let key = random.next_long();
        let v = if c % 7 == 0 {
            None
        } else {
            Some(random.next_int_unbounded())
        };
        let first = PlayerListEntry {
            key,
            value: v,
            hash: c,
        };
        let second = PlayerListEntry {
            key: if c % 3 == 0 { key } else { key.wrapping_add(1) },
            value: if c % 2 == 0 { v } else { Some(c) },
            hash: -c,
        };
        out.push((first == first) as u8);
        out.push((first == second) as u8);
        int(&mut out, hash(key));
        let text = first.to_string();
        int(&mut out, text.len() as i32);
        out.extend(text.as_bytes());
    }
    std::fs::write(&args[1], &out).unwrap();
    if let Some(expected) = args.get(2) {
        let expected = std::fs::read(expected).unwrap();
        if out != expected {
            let at = out
                .iter()
                .zip(&expected)
                .position(|(a, b)| a != b)
                .unwrap_or(out.len().min(expected.len()));
            eprintln!(
                "PLAYER LIST DIFFERS at byte {at}, lengths {}/{}",
                out.len(),
                expected.len()
            );
            std::process::exit(1);
        }
        println!(
            "PLAYER LIST EXACT: 3000 table operations and 10000 entry equality/hash/string cases"
        );
    }
}
