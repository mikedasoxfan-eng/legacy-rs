use worldgen::{entity::{player::{controller::PlayerControllerSP, inventory::ItemStack}, Entity, MovingObjectPosition as Hit}, World};
fn i(o:&mut Vec<u8>,v:i32){o.extend(v.to_le_bytes());}
fn f(o:&mut Vec<u8>,v:f32){o.extend(v.to_le_bytes());}
fn stack(o:&mut Vec<u8>,s:Option<ItemStack>){for v in s.map_or([-1,0,0,0],|s|[s.id,s.count,s.damage,s.animations]){i(o,v);}}
fn raw(w:&mut World,x:i32,y:i32,z:i32,id:u8,meta:i32){let c=w.chunks.get_mut(&(x>>4,z>>4)).unwrap();let at=worldgen::world::chunk::idx(x&15,y,z&15);c.blocks[at]=id;c.data.set(x&15,y,z&15,meta);}
fn state(w:&mut World,p:&mut Entity,c:&PlayerControllerSP,creative:bool,result:i32,o:&mut Vec<u8>){
    i(o,result);
    if creative {i(o,c.block_hit_wait);} else {for v in [c.cur_block_damage,c.prev_block_damage,c.step_sound_counter]{f(o,v);}for v in [c.target.0,c.target.1,c.target.2,c.block_hit_wait]{i(o,v);}}
    for y in 64..=65 {for x in 3..=5 {for z in 3..=5 {i(o,w.get_block_id(x,y,z)as i32);i(o,w.get_block_metadata(x,y,z));}}}
    for s in p.player_state().unwrap().inventory.main {stack(o,s);}
    f(o,p.player_state().unwrap().food.exhaustion);
    let log=std::mem::take(&mut p.player_state_mut().unwrap().stat_log);i(o,log.len()as i32);for (id,n) in log {i(o,id);i(o,n);}
    o.extend(w.rand.state().to_le_bytes());o.extend(w.item_random.state().to_le_bytes());
    worldgen::world::events::write_events(&w.drain_events(),o);
    i(o,w.loaded_entity_list.len()as i32);for id in w.loaded_entity_list.clone(){let e=w.entities[&id].clone();worldgen::entity::player::verify181::dropped_entity_snapshot(w,&e,o);}
}
fn main(){
    let args:Vec<_>=std::env::args().collect();let mut o=Vec::new();
    for creative in [false,true] {
        let mut w=worldgen::entity::verify181::world();w.item_random=worldgen::JavaRandom::new(600);
        let mut p=Entity::player(&mut w,"Player");p.set_location_and_angles(0.5,64.0,0.5,0.0,0.0);p.on_ground=true;p.set_creative(creative);
        let mut c=PlayerControllerSP::default();f(&mut o,PlayerControllerSP::block_reach(creative));for b in [creative,PlayerControllerSP::should_draw_hud(creative),false,!creative,!creative,creative,PlayerControllerSP::has_extended_entity_reach(creative)]{o.push(b as u8);}
        for t in 0..1200 {
            let phase=t%60;let mut result=-1;
            if phase==0 {w.loaded_entity_list.clear();w.entities.clear();raw(&mut w,4,64,4,[1,3,7,18,31,106][t/60%6],(t/60%4)as i32);p.player_state_mut().unwrap().inventory.main[0]=Some(ItemStack::new([278,359,268,257][t/60%4],1,0));}
            if phase==0||phase==20 {c.click_block(&mut p,&mut w,4,64,4,3);}else if phase==10 {if !creative {c.reset();}}else {c.dig(&mut p,&mut w,4,64,4,3);}
            if phase==30 {raw(&mut w,4,64,4,1,0);raw(&mut w,4,64,5,51,0);c.click_block(&mut p,&mut w,4,64,4,3);}
            if phase==40 {raw(&mut w,4,64,4,3,0);raw(&mut w,4,65,4,0,0);p.player_state_mut().unwrap().inventory.main[0]=Some(ItemStack::new([5,35,259,290,351][t/60%5],3,if t/60%5==4 {0}else {2}));result=p.use_on_block(&mut w,4,64,4,1)as i32;}
            if phase==45 {p.player_state_mut().unwrap().inventory.main[0]=Some(ItemStack::new(267,1,0));result=p.use_item(&mut w)as i32;}
            if phase==46 {p.stop_item_use(&mut w);}
            if !creative {c.update();}
            let mut frame=Vec::new();state(&mut w,&mut p,&c,creative,result,&mut frame);i(&mut o,frame.len()as i32);o.extend(frame);
        }
    }
    for creative in [false,true] {for using in [false,true] {
        let mut w=worldgen::entity::verify181::world();w.item_random=worldgen::JavaRandom::new(601);
        let mut p=Entity::player(&mut w,"Player");p.set_location_and_angles(0.5,64.0,0.5,0.0,0.0);p.on_ground=true;p.set_creative(creative);
        p.player_state_mut().unwrap().inventory.main[8]=Some(ItemStack::new(1,4,0));let sword=ItemStack::new(267,1,0);p.player_state_mut().unwrap().inventory.main[0]=Some(sword);if using {p.start_item_use(sword,72000);}
        let id=p.entity_id;w.entities.insert(id,p);
        for t in 0..200 {
            if t%20==0 {raw(&mut w,4,64,4,1,0);}
            let hit=if t%20<15 {Some(Hit::Block{x:4,y:64,z:4,side:1,hit:worldgen::entity::Vec3D::new(4.0,65.0,4.0)})}else{None};
            w.apply_player_tick_input(id,worldgen::entity::player::api::PlayerTickInput{target:worldgen::entity::player::api::PlayerTarget::ObjectMouseOver(hit),left_held:t%20<18,right_held:using&&t<100,pick_presses:if t%20==5||t%20==6 {1}else{0},..Default::default()},t);
            let mut p=w.entities.remove(&id).unwrap();let c=p.player_state().unwrap().controller.clone();let mut frame=Vec::new();state(&mut w,&mut p,&c,creative,-1,&mut frame);let s=p.player_state().unwrap();i(&mut frame,s.swing_ticks);frame.push(s.is_swinging as u8);i(&mut frame,s.item_use_ticks);i(&mut frame,s.inventory.current_item);i(&mut frame,s.pending_pick_presses);i(&mut o,frame.len()as i32);o.extend(frame);w.entities.insert(id,p);
        }
    }}
    for creative in [false,true] {for block in [0,1,2,7,35,43,44,97] {for slot in [0,8,9,35] {for hit in 0..3 {
        let mut w=worldgen::entity::verify181::world();let mut p=Entity::player(&mut w,"Player");p.set_creative(creative);
        p.player_state_mut().unwrap().inventory.main[slot]=Some(ItemStack::new(match block {2=>3,7=>1,43=>44,b=>b},1,0));p.player_state_mut().unwrap().inventory.current_item=5;
        raw(&mut w,4,64,4,block as u8,0);raw(&mut w,0,0,0,block as u8,0);
        p.pick_block(&mut w,match hit {0=>None,1=>Some(Hit::Block{x:4,y:64,z:4,side:1,hit:worldgen::entity::Vec3D::new(4.0,65.0,4.0)}),_=>Some(Hit::Entity{id:p.entity_id,hit:p.pos})});i(&mut o,p.player_state().unwrap().inventory.current_item);
    }}}}
    for creative in [false,true] {for distance in 2..=35 {for obstruction in 0..2 {
        let mut w=worldgen::entity::verify181::world();let mut p=Entity::player(&mut w,"Player");p.set_location_and_angles(0.5,64.0,0.5,0.0,0.0);p.set_creative(creative);
        let mut pig=Entity::living(&mut w,"Pig").unwrap();pig.set_location_and_angles(0.5,64.0,distance as f64+0.5,0.0,0.0);w.entity_joined_world(pig);
        if obstruction!=0 {raw(&mut w,0,64,4,1,0);}
        let h=p.mouse_over(&mut w,1.0);i(&mut o,match h {None=>-1,Some(Hit::Entity{..})=>1,_=>0});if let Some(h)=h {match h {Hit::Entity{id,..}=>i(&mut o,id),Hit::Block{x,y,z,side,hit}=>{for v in [x,y,z,side]{i(&mut o,v);}for v in [hit.x,hit.y,hit.z]{o.extend(v.to_le_bytes());}}}}
    }}}
    std::fs::write(&args[1],&o).unwrap();let expected=std::fs::read(&args[2]).unwrap();if expected!=o {let at=expected.iter().zip(&o).position(|(a,b)|a!=b).unwrap_or(expected.len().min(o.len()));eprintln!("CONTROLLER DIFFERS at byte {at}, lengths {}/{}",expected.len(),o.len());std::process::exit(1);}println!("CONTROLLER EXACT: 2400 controller and 800 held-input transitions, 192 picks, 136 reach cases ({} bytes)",o.len());
}
