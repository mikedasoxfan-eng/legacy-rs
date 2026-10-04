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
    let mut p=Entity::player(&mut w,"Player");p.set_location_and_angles(0.5,64.0,0.5,0.0,0.0);let mut id=p.entity_id;w.entity_joined_world(p);
    for z in 3..=5 {for x in -2..=2 {raw(&mut w,x,64,z,1,0);}}
    for x in 4..=8 {for z in -2..=2 {raw(&mut w,x,64,z,53,0);}}
    for x in 10..=15 {for y in 64..=67 {for z in -2..=2 {raw(&mut w,x,y,z,9,0);}}}
    for x in -15..=-10 {for y in 64..=67 {for z in -2..=2 {raw(&mut w,x,y,z,11,0);}}}
    raw(&mut w,1,64,-5,30,0);for y in 64..=71 {raw(&mut w,0,y,-8,65,3);raw(&mut w,0,y,-9,1,0);}
    let interaction=matches!(scenario,"dig"|"place"|"use"|"shoot"|"interact"|"fertilize"|"activate");let mut result=0;let mut controller=super::controller::PlayerControllerSP::default();
    let mut out=Vec::new();snapshot(&w,&mut out);if scenario=="look" {picking_bytes(&mut w,id,&mut out);}if interaction {controller_bytes(&controller,result,&mut out);}
    for t in 0..ticks {
        if let Some(mut p)=w.entities.remove(&id) {
            if t%120==0 {let phase=t/120%8;p.motion=Default::default();let x=match phase {2=>4.5,3=>11.5,4=>-12.5,_=>0.5};let z=match phase {6=>-7.5,7=>-4.5,_=>0.5};let y=match phase {5=>76.0,3|4=>66.0,_=>64.0};p.set_location_and_angles(x,y,z,if phase==2 {-90.0}else if phase>=6 {180.0}else{0.0},0.0);if scenario=="movement" {p.living.health=20;p.living.hearts_life=0;}}
            if scenario=="look"&&t%120==0 {p.living.health=20;p.living.hearts_life=0;}
            if scenario=="damage"&&t==1 {for a in 0..4 {p.player_state_mut().unwrap().inventory.armor[a]=Some(ItemStack::new(310+a as i32,1,a as i32*40));}p.attack(&mut w,None,3);}
            if scenario=="damage"&&t==30 {let mut mob=Entity::living(&mut w,"Zombie").unwrap();mob.set_location_and_angles(0.5,64.0,1.5,0.0,0.0);let source=mob.entity_id;w.entity_joined_world(mob);p.attack(&mut w,Some(source),5);}
            if scenario=="pickup"&&t%20==0 {let e=Entity::item(&mut w,p.pos.x,p.pos.y-1.3,p.pos.z,crate::world::tile::ItemStack::new(if t%40==0{35}else{263},40,t%3));w.entity_joined_world(e);}
            p.set_player_input(super::PlayerInput {forward:t%120<80,left:t%120>=80,jump:t%31<8,sneak:t%120>=90,..Default::default()});
            if t%13==0 {p.swing_item();}
            if scenario=="sleep" {if t==0 {w.skylight_subtracted=11;w.world_time=18000;p.set_location_and_angles(0.5,64.0,0.5,0.0,0.0);raw(&mut w,0,64,0,26,0);raw(&mut w,0,64,1,26,8);p.sleep_in_bed(&mut w,0,64,1);}if t==120 {p.wake_up(&mut w,false,true);}}
            if scenario=="portal" {let s=p.player_state_mut().unwrap();s.input.forward=false;s.input.left=false;s.input.jump=false;if t<110||(150..160).contains(&t) {p.set_in_portal();}}
            if scenario=="look" {p.living.health=20;p.living.hearts_life=0;p.set_angles((t%7-3) as f32*33.7,(t%11-5) as f32*9.8);}
            if interaction {
                p.set_location_and_angles(0.5,64.0,0.5,t as f32*33.7,0.0);p.motion=Default::default();p.set_player_input(Default::default());p.on_ground=true;controller.update();result=0;
                if scenario=="dig" {
                    let phase=(t/100%10) as usize;let blocks=[1,17,56,49,35,18,78,79,4,3];let tools=[0,271,257,278,359,359,269,278,0,0];
                    if t%100==0 {raw(&mut w,8,64,8,blocks[phase],if phase==4{7}else{0});p.player_state_mut().unwrap().inventory.main[0]=if tools[phase]==0{None}else{Some(ItemStack::new(tools[phase],1,0))};controller.reset();controller.click_block(&mut p,&mut w,8,64,8,1);}
                    controller.dig(&mut p,&mut w,8,64,8,1);
                }else if scenario=="place" {
                    let blocks=[1,5,35,44,53,67,86,50,65,69,77,324,330,355,54];let block=blocks[t as usize%blocks.len()];let x=8+t%6*3;let z=8+t/6%6*3;
                    raw(&mut w,x,64,z,1,0);p.player_state_mut().unwrap().inventory.main[0]=Some(ItemStack::new(block,10,t%4));result=p.use_on_block(&mut w,x,64,z,t%6) as i32;
                }else if scenario=="interact" {
                    let phase=t/30%7;let item=[359,351,325,329,352,276,278][phase as usize];p.player_state_mut().unwrap().inventory.main[0]=Some(ItemStack::new(item,10,if phase==1{t%16}else{0}));
                    if t%30==0 {let mut mob=Entity::living(&mut w,if phase<2{"Sheep"}else if phase==2{"Cow"}else if phase==3{"Pig"}else if phase==4{"Wolf"}else{"Zombie"}).unwrap();mob.set_location_and_angles(0.5,64.0,1.5,0.0,0.0);w.entity_joined_world(mob);}
                    let mut mob=*w.loaded_entity_list.last().unwrap();if matches!(w.entities.get(&mob).map(|e|&e.kind),Some(crate::entity::EntityKind::Item{..})) {for &i in &w.loaded_entity_list {if i!=id&&w.entities.get(&i).is_some_and(|e|e.kind.is_living()) {mob=i;}}}
                    if phase>=5 {p.attack_with_item(&mut w,mob);}else{p.use_on_entity(&mut w,mob);}
                }else if scenario=="fertilize" {
                    let phase=t%5;let block=if phase<3{6}else if phase==3{59}else{2};raw(&mut w,8,63,8,if phase==3{60}else{2},0);raw(&mut w,8,64,8,block,if phase<3{phase}else{0});p.player_state_mut().unwrap().inventory.main[0]=Some(ItemStack::new(351,10,15));result=p.use_on_block(&mut w,8,64,8,1) as i32;
                }else if scenario=="activate" {
                    let block=[64,69,77,96,93,92,58,54,61,23][t as usize%10];w.remove_block_tile_entity(8,64,8);raw(&mut w,8,64,8,block,if block==69||block==77{5}else{0});if block==64 {raw(&mut w,8,65,8,block,8);}p.living.health=5;p.player_state_mut().unwrap().inventory.main[0]=None;result=p.use_on_block(&mut w,8,64,8,1) as i32;
                }else if scenario=="shoot" {
                    let item=if t%3==0{261}else if t%3==1{332}else{344};p.player_state_mut().unwrap().inventory.main[0]=Some(ItemStack::new(item,16,0));p.player_state_mut().unwrap().inventory.main[1]=Some(ItemStack::new(262,64,0));result=p.use_item(&mut w) as i32;
                }else {
                    let items=[260,282,297,319,320,322,349,350,357,259,290,325,326,327,335];let item=items[t as usize%items.len()];p.living.health=5;p.player_state_mut().unwrap().inventory.main[0]=Some(ItemStack::new(item,1,0));
                    if item==259||item==290 {raw(&mut w,8,64,8,3,0);result=p.use_on_block(&mut w,8,64,8,t%6) as i32;}else{result=p.use_item(&mut w) as i32;}
                }
            }
            if scenario=="nbt"&&t%100==99 {let n=p.player_nbt().unwrap();w.loaded_entity_list.retain(|&v|v!=id);for c in w.chunks.values_mut(){for list in &mut c.entity_lists {list.retain(|&v|v!=id);}}w.players.clear();w.player_entities.clear();p=Entity::player_from_nbt(&mut w,&n,"Player").unwrap();id=p.entity_id;w.entity_joined_world(p);}else{w.entities.insert(id,p);}
        }
        w.update_entities();snapshot(&w,&mut out);if scenario=="look" {picking_bytes(&mut w,id,&mut out);}if interaction {controller_bytes(&controller,result,&mut out);}
    }
    append_world(&w,&mut out);out
}
fn controller_bytes(c:&super::controller::PlayerControllerSP,result:i32,out:&mut Vec<u8>) {out.extend_from_slice(&result.to_le_bytes());for v in [c.cur_block_damage,c.prev_block_damage,c.step_sound_counter]{out.extend_from_slice(&v.to_bits().to_le_bytes());}for v in [c.target.0,c.target.1,c.target.2,c.block_hit_wait]{out.extend_from_slice(&v.to_le_bytes());}}
fn picking_bytes(w:&mut crate::World,id:i32,out:&mut Vec<u8>) {
    use crate::entity::MovingObjectPosition;
    let Some(p)=w.entities.get(&id).cloned() else {return;};
    for partial in [0.0,0.5,1.0] {
        let look=p.look(partial);for v in [look.x,look.y,look.z] {out.extend_from_slice(&v.to_bits().to_le_bytes());}
        let hit=p.mouse_over(w,partial);let kind:i32=match hit {None=>-1,Some(MovingObjectPosition::Entity{..})=>1,_=>0};out.extend_from_slice(&kind.to_le_bytes());
        match hit {Some(MovingObjectPosition::Entity{id,..})=>out.extend_from_slice(&id.to_le_bytes()),Some(MovingObjectPosition::Block{x,y,z,side,hit})=>{for v in [x,y,z,side]{out.extend_from_slice(&v.to_le_bytes());}for v in [hit.x,hit.y,hit.z]{out.extend_from_slice(&v.to_bits().to_le_bytes());}},_=>{}}
    }
}
