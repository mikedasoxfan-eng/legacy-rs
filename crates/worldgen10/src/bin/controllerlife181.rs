use worldgen::{entity::player::controller::PlayerControllerSP, entity::Entity, world::Dimension, JavaRandom};
fn i(o:&mut Vec<u8>,n:i32){o.extend(n.to_le_bytes());}
fn f(o:&mut Vec<u8>,n:f32){o.extend(if n.is_nan(){0x7fc00000u32}else{n.to_bits()}.to_le_bytes());}
fn state(p:&Entity,o:&mut Vec<u8>){
    let b=p.bounding_box;
    for n in [p.pos.x,p.pos.y,p.pos.z,p.prev_pos.x,p.prev_pos.y,p.prev_pos.z,b.min_x,b.min_y,b.min_z,b.max_x,b.max_y,b.max_z]{o.extend(n.to_le_bytes());}
    for n in [p.yaw,p.pitch,p.prev_yaw,p.prev_pitch,p.y_offset,p.width,p.height]{f(o,n);}
    let s=p.player_state().unwrap();
    for n in [s.dimension,p.living.health,p.fire_resistance,s.inventory.current_item]{i(o,n);}
    i(o,s.username.len() as i32);o.extend(s.username.as_bytes());o.push(1);o.push(1);
    for stack in s.inventory.main {for n in stack.map_or([-1,0,0,0],|s|[s.id,s.count,s.damage,s.animations]){i(o,n);}}
}
fn main(){
    let a:Vec<_>=std::env::args().collect();let mut o=Vec::new();let mut r=JavaRandom::new(7181);
    let mut w=worldgen::entity::verify181::world();
    // The original owner is constructed before the factory cases.
    let mut owner=Entity::player(&mut w,"Player");
    for c in 0..1000 {
        let creative=c%2==0;
        w.spawn=(r.next_int(600001)-300000,r.next_int(256)-64,r.next_int(600001)-300000);
        w.dimension=[Dimension::Overworld,Dimension::Nether,Dimension::Sky][c%3];
        let mut p=PlayerControllerSP::create_player(&mut w,["Player","","Notch","Jörg"][c%4]);
        state(&p,&mut o);o.push(1);
        p.yaw=(r.next_int(200001)-100000) as f32/317.0;p.pitch=(r.next_int(200001)-100000) as f32/319.0;
        p.prev_yaw=123.25;p.prev_pitch=-47.75;
        PlayerControllerSP::flip_player(&mut p,creative);
        for n in [p.yaw,p.pitch,p.prev_yaw,p.prev_pitch]{f(&mut o,n);}
    }
    let bounds=[-f32::MAX,-1.0,-0.0,0.0,f32::from_bits(1),1.0,f32::MAX,f32::NEG_INFINITY,f32::INFINITY,f32::NAN];
    let mut controller=PlayerControllerSP::default();
    for c in 0..20000 {
        controller.cur_block_damage=if c%3==0 {bounds[c%bounds.len()]} else {(r.next_int(200001)-100000) as f32/317.0};
        controller.prev_block_damage=if c%5==0 {bounds[c/5%bounds.len()]} else {(r.next_int(200001)-100000) as f32/319.0};
        let partial=if c%7==0 {bounds[c/7%bounds.len()]} else {(r.next_int(200001)-100000) as f32/331.0};
        controller.step_sound_counter=17.25;controller.block_hit_wait=-13;
        let value=controller.damage_partial(partial);f(&mut o,value);f(&mut o,value);
        controller.reset();
        for n in [controller.cur_block_damage,controller.prev_block_damage,controller.step_sound_counter]{f(&mut o,n);}i(&mut o,controller.block_hit_wait);
    }
    w.set_block(4,64,4,0);
    let hook=std::panic::take_hook();std::panic::set_hook(Box::new(|_|{}));
    let failed=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||controller.remove_block(&mut owner,&mut w,4,64,4,1))).is_err();
    std::panic::set_hook(hook);o.push(failed as u8);
    for c in 0..256 {
        use worldgen::entity::player::ItemStack;
        let item=ItemStack::new([267,260,261,1][c as usize%4],c%9-3,c%17);
        owner.player_state_mut().unwrap().inventory.main[0]=Some(item);
        if c%5!=0 {owner.start_item_use(item,[-1,0,32,72000][c as usize%4]);}
        owner.stop_item_use(&mut w);
        let s=owner.player_state().unwrap();
        for n in s.item_in_use.map_or([-1,0,0,0],|s|[s.id,s.count,s.damage,s.animations]){i(&mut o,n);}
        i(&mut o,s.item_use_ticks);o.push(s.item_in_use.is_some() as u8);o.push(owner.is_blocking() as u8);
        for n in s.inventory.main[0].map_or([-1,0,0,0],|s|[s.id,s.count,s.damage,s.animations]){i(&mut o,n);}
    }
    std::fs::write(&a[1],&o).unwrap();let expected=std::fs::read(&a[2]).unwrap();
    if o!=expected {let at=o.iter().zip(&expected).position(|(a,b)|a!=b).unwrap_or(o.len().min(expected.len()));eprintln!("CONTROLLER LIFECYCLE DIFFERS byte {at}, lengths {}/{}",o.len(),expected.len());std::process::exit(1);}
    println!("CONTROLLER LIFECYCLE EXACT: 1000 factory/flip cases, 20000 partial/reset boundaries, original air failure ({} bytes)",o.len());
}
