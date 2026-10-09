use worldgen::{entity::player::{InventoryPlayer,ItemStack},nbt::{Compound,Tag}};
fn i(o:&mut Vec<u8>,n:i32){o.extend(n.to_le_bytes());}
fn stack(o:&mut Vec<u8>,s:Option<ItemStack>){for n in s.map_or([-1,0,0,0],|s|[s.id,s.count,s.damage,s.animations]){i(o,n);}}
fn tag(slot:i32,id:i32,count:i32,damage:i32)->Tag{
    let mut n=Compound::new();n.put("Slot",Tag::Byte(slot as i8));n.put("id",Tag::Short(id as i16));
    n.put("Count",Tag::Byte(count as i8));n.put("Damage",Tag::Short(damage as i16));Tag::Compound(n)
}
fn main(){
    let a:Vec<_>=std::env::args().collect();let mut out=Vec::new();let mut inv=InventoryPlayer::default();
    let hook=std::panic::take_hook();std::panic::set_hook(Box::new(|_|{}));
    for c in 0..65536 {
        inv.current_item=c%9;inv.changed=c&1!=0;inv.cursor=Some(ItemStack::new(35,-3,14));inv.main[20]=Some(ItemStack::new(264,5,0));
        let tags=[tag(0,260,3,0),tag(c*37,c,c,c*97),tag(100,310,1,7)];
        let failed=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||inv.read_nbt(&tags))).is_err();
        out.push(failed as u8);i(&mut out,40);
        for s in inv.main.into_iter().chain(inv.armor){stack(&mut out,s);}
        stack(&mut out,inv.cursor);i(&mut out,inv.current_item);out.push(inv.changed as u8);
        for id in [c as i16 as i32,260,310]{out.push(inv.contains_item(id) as u8);}
    }
    std::panic::set_hook(hook);std::fs::write(&a[1],&out).unwrap();let expected=std::fs::read(&a[2]).unwrap();
    if out!=expected{let at=out.iter().zip(&expected).position(|(a,b)|a!=b).unwrap_or(out.len().min(expected.len()));eprintln!("INVENTORY BOUNDS DIFFERS byte {at}, lengths {}/{}",out.len(),expected.len());std::process::exit(1);}
    println!("INVENTORY BOUNDS EXACT: all 65536 signed-short IDs, exception/partial-state/slot/cursor/membership semantics ({} bytes)",out.len());
}
