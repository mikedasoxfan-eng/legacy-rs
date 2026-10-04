//! Original-bytecode replay, shared by compare and verify.sh.
use super::inventory::{InventoryPlayer, ItemStack};
use crate::JavaRandom;
pub fn inventory_bytes()->Vec<u8> {
    let mut out=Vec::new(); let mut r=JavaRandom::new(1729); let mut inv=InventoryPlayer::default();
    let ids=[1,35,263,256,257,298,299,300,301,332,351,357,2256];
    for _ in 0..10000 {
        let op=r.next_int(7); let id=ids[r.next_int(ids.len() as i32) as usize]; let count=r.next_int(80)+1; let damage=r.next_int(150);
        let result=match op {
            0|1=>{let mut s=ItemStack::new(id,count,damage); let ok=inv.add(&mut s); write(&mut out,Some(s)); ok as i32},
            2=>inv.consume(id) as i32,
            3=>{let slot=r.next_int(40) as usize;write(&mut out,inv.decrement(slot,count));0},
            4=>{inv.change_current_item(count-40);inv.current_item},
            5=>{let slot=r.next_int(4) as usize;inv.armor[slot]=Some(ItemStack::new(298+r.next_int(20),1,damage));inv.damage_armor(count);inv.armor_value()},
            _=>{inv.tick_animations();if let crate::nbt::Tag::List(_,tags)=inv.to_nbt(){inv.read_nbt(&tags);}0},
        };
        out.extend_from_slice(&result.to_le_bytes());
        for s in inv.main.iter().chain(&inv.armor) {write(&mut out,*s);}
        out.extend_from_slice(&inv.current_item.to_le_bytes()); out.extend_from_slice(&inv.armor_value().to_le_bytes());
    }
    out
}
fn write(out:&mut Vec<u8>,s:Option<ItemStack>) {let a=s.map_or([-1,0,0,0],|s|[s.id,s.count,s.damage,s.animations]);for n in a {out.extend_from_slice(&n.to_le_bytes());}}
