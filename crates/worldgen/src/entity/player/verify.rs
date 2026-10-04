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
pub fn state_bytes(s:&super::PlayerState,e:&crate::entity::Entity,out:&mut Vec<u8>) {
    for v in [s.prev_camera_yaw,s.camera_yaw,s.prev_camera_pitch,s.camera_pitch,s.time_in_portal,s.prev_time_in_portal] {out.extend_from_slice(&v.to_bits().to_le_bytes());}
    for v in [s.swing_ticks,s.portal_cooldown,s.sleep_timer,s.inventory.armor_value(),s.inventory.current_item,s.damage_remainder] {out.extend_from_slice(&v.to_le_bytes());}
    for v in [s.is_swinging,s.sleeping,e.sneaking] {out.push(v as u8);}
    for v in [s.prev_cloak.x,s.prev_cloak.y,s.prev_cloak.z,s.cloak.x,s.cloak.y,s.cloak.z] {out.extend_from_slice(&v.to_bits().to_le_bytes());}
    for s in s.inventory.main.iter().chain(&s.inventory.armor) {write(out,*s);}
}
pub fn player_bytes(scenario:&str,ticks:i32)->Vec<u8> {
    use crate::{World,entity::{Entity,verify::{snapshot,append_world}},world::chunk::{Chunk,idx}};
    let mut w=World::new(0,300);w.set_allowed_mob_spawns(false,false);w.set_entity_determinism(100,200,0);
    for cx in -3..=3 {for cz in -3..=3 {
        let mut c=Chunk::new(cx,cz,vec![0;32768].into_boxed_slice().try_into().unwrap());
        for x in 0..16 {for z in 0..16 {c.blocks[idx(x,63,z)]=1;}}
        c.loaded=true;c.populated=true;w.chunks.insert((cx,cz),Box::new(c));w.chunk_list.push((cx,cz));
    }}
    fn raw(w:&mut World,x:i32,y:i32,z:i32,id:u8,meta:i32) {let c=w.chunk_mut(x>>4,z>>4);c.blocks[idx(x&15,y,z&15)]=id;c.data.set(x&15,y,z&15,meta);}
    let mut p=Entity::player(&mut w,"Player");p.set_location_and_angles(0.5,64.0,0.5,0.0,0.0);let id=p.entity_id;w.entity_joined_world(p);
    for z in 3..=5 {for x in -2..=2 {raw(&mut w,x,64,z,1,0);}}
    for x in 4..=8 {for z in -2..=2 {raw(&mut w,x,64,z,53,0);}}
    for x in 10..=15 {for y in 64..=67 {for z in -2..=2 {raw(&mut w,x,y,z,9,0);}}}
    for x in -15..=-10 {for y in 64..=67 {for z in -2..=2 {raw(&mut w,x,y,z,11,0);}}}
    raw(&mut w,1,64,-5,30,0);for y in 64..=71 {raw(&mut w,0,y,-8,65,3);raw(&mut w,0,y,-9,1,0);}
    let mut out=Vec::new();snapshot(&w,&mut out);
    for t in 0..ticks {
        if let Some(mut p)=w.entities.remove(&id) {
            if t%120==0 {let phase=t/120%8;p.motion=Default::default();let x=match phase {2=>4.5,3=>11.5,4=>-12.5,_=>0.5};let z=match phase {6=>-7.5,7=>-4.5,_=>0.5};let y=match phase {5=>76.0,3|4=>66.0,_=>64.0};p.set_location_and_angles(x,y,z,if phase==2 {-90.0}else if phase>=6 {180.0}else{0.0},0.0);if scenario=="movement" {p.living.health=20;p.living.hearts_life=0;}}
            if scenario=="damage"&&t==1 {for a in 0..4 {p.player_state_mut().unwrap().inventory.armor[a]=Some(ItemStack::new(310+a as i32,1,a as i32*40));}p.attack(&mut w,None,3);}
            if scenario=="damage"&&t==30 {let mut mob=Entity::living(&mut w,"Zombie").unwrap();mob.set_location_and_angles(0.5,64.0,1.5,0.0,0.0);let source=mob.entity_id;w.entity_joined_world(mob);p.attack(&mut w,Some(source),5);}
            if scenario=="pickup"&&t%20==0 {let e=Entity::item(&mut w,p.pos.x,p.pos.y-1.3,p.pos.z,crate::world::tile::ItemStack::new(if t%40==0{35}else{263},40,t%3));w.entity_joined_world(e);}
            p.set_player_input(super::PlayerInput {forward:t%120<80,left:t%120>=80,jump:t%31<8,sneak:t%120>=90,..Default::default()});
            if t%13==0 {p.swing_item();}w.entities.insert(id,p);
        }
        w.update_entities();snapshot(&w,&mut out);
    }
    append_world(&w,&mut out);out
}
