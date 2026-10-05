//! Original-bytecode whole-player replay. No terrain/world serialization dependency.
use super::*;
/// Reuse the entities owner's serializer for the local queued-drop fixture.
pub fn dropped_entity_snapshot(w: &mut World, entity: &Entity, out: &mut Vec<u8>) {
    super::super::verify181::state(w, entity, out);
}
fn i(o: &mut Vec<u8>, v: i32) { o.extend(v.to_le_bytes()); }
fn f(o: &mut Vec<u8>, v: f32) { o.extend(v.to_le_bytes()); }
fn frame(w: &mut World, p: &mut Entity, o: &mut Vec<u8>) {
    let mut bytes = Vec::new();
    super::super::verify181::state(w,p,&mut bytes);
    verify::state_bytes(p.player_state().unwrap(),p,&mut bytes);
    p.player_state_mut().unwrap().stat_log.clear();
    let s=p.player_state().unwrap();let food=&s.food;
    for v in [food.level,food.prev_level,food.tick_timer,s.experience.points,s.experience.level,s.experience.total,s.xp_pickup_cooldown,s.sprint_ticks,s.sprint_toggle_timer,s.fly_toggle_timer,s.item_use_ticks] {i(&mut bytes,v);}
    for v in [food.saturation,food.exhaustion,p.fov_multiplier(),p.living.swing_progress,p.living.prev_swing_progress,s.previous_forward,s.render_arm_yaw,s.render_arm_pitch,s.prev_render_arm_yaw,s.prev_render_arm_pitch] {f(&mut bytes,v);}
    for v in [s.sprinting,s.capabilities.disable_damage,s.capabilities.flying,s.capabilities.allow_flying,s.capabilities.creative,s.previous_jump,p.is_blocking()] {bytes.push(v as u8);}
    verify::write(&mut bytes,s.item_in_use);
    let arrows:Vec<_> = w.loaded_entity_list.iter().filter_map(|id|w.entities.get(id)).filter(|e|matches!(e.kind,EntityKind::Arrow(_))).collect();
    i(&mut bytes,arrows.len() as i32);
    for e in arrows {
        if let EntityKind::Arrow(a)=&e.kind {bytes.push(a.critical as u8);}
        for v in [e.motion.x,e.motion.y,e.motion.z] { bytes.extend(v.to_le_bytes()); }
    }
    i(&mut bytes,s.controller.block_hit_wait);
    i(&mut bytes,w.get_block_id(4,64,4) as i32);
    i(&mut bytes,w.get_block_id(4,64,5) as i32);
    i(o,bytes.len() as i32);o.extend(bytes);
}
fn food(p:&mut Entity,level:i32,saturation:f32) {
    let s=p.player_state_mut().unwrap();s.food.level=level;s.food.saturation=saturation;s.food.exhaustion=0.0;s.food.tick_timer=0;
}
pub fn replay(kind:&str,ticks:i32)->Vec<u8> {
    use crate::entity::{damage::{DamageKind,DamageSource},potion::PotionEffect};
    let mut w=super::super::verify181::world();w.item_random=crate::JavaRandom::new(600);
    let mut p=Entity::player(&mut w,"Player");p.set_location_and_angles(0.5,64.0,0.5,0.0,0.0);
    if kind=="flight" {p.set_creative(true);}
    if kind=="creativecontroller" {
        p.player_state_mut().unwrap().inventory.main[0]=Some(ItemStack::new(260,37,0));
        p.initialize_player_mode(true);
    }
    if kind=="starve" {food(&mut p,0,0.0);p.living.health=12;for j in 0..4 {p.player_state_mut().unwrap().inventory.armor[j]=Some(ItemStack::new(310+j as i32,1,0));}}
    if kind=="regen" {food(&mut p,20,5.0);p.living.health=10;}
    let mut o=vec![];frame(&mut w,&mut p,&mut o);
    for t in 0..ticks {
        let input=PlayerInput {
            forward:(kind=="movement"&&t%80<60)||(kind=="sprint"&&(t%60==0||t%60>=2&&t%60<45)),
            left:kind=="movement"&&t%80>=60,
            jump:(kind=="movement"&&t%31<8)||(kind=="flight"&&(t==1||t==3||(10..18).contains(&t)||t==40||t==42)),
            sneak:(kind=="movement"&&t%80>=70)||(kind=="flight"&&(22..30).contains(&t)),..Default::default()
        };
        p.set_player_input(input);
        if kind=="flight"&&t==0 {p.set_location_and_angles(0.5,75.0,0.5,0.0,0.0);}
        if kind=="sprint"&&t==90 {food(&mut p,6,0.0);}
        if kind=="sprint"&&t==150 {food(&mut p,20,5.0);}
        if kind.starts_with("eat")&&t%80==0 {let id=kind[3..].parse().unwrap();food(&mut p,10,0.0);let s=p.player_state_mut().unwrap();s.inventory.main[0]=Some(ItemStack::new(id,3,0));s.inventory.current_item=0;p.use_item(&mut w);}
        if kind=="cancel"&&t%40==0 {food(&mut p,10,0.0);let item=ItemStack::new(260,3,0);let s=p.player_state_mut().unwrap();s.inventory.main[0]=Some(item);s.inventory.current_item=0;p.start_item_use(item,32);}
        if kind=="cancel"&&t%40==10 {p.player_state_mut().unwrap().inventory.current_item=1;}
        if kind=="xp"&&t%3==0 {let orb=Entity::xp_orb(&mut w,Vec3D::new(p.pos.x,p.bounding_box.min_y,p.pos.z),1+t%37);w.entity_joined_world(orb);}
        if kind=="damage"&&t%30==0 {p.living.health=20;p.living.hearts_life=0;for j in 0..4 {p.player_state_mut().unwrap().inventory.armor[j]=Some(ItemStack::new(310+j as i32,1,j as i32*40));}let item=ItemStack::new(267,1,0);p.player_state_mut().unwrap().inventory.main[0]=Some(item);p.start_item_use(item,72000);}
        if kind=="damage"&&t%5==0 {p.attack_from(&mut w,DamageSource::new(if t%10==0 {DamageKind::Starve}else {DamageKind::Cactus}),5);}
        if kind=="creativevoid"&&t==0 {p.set_creative(true);p.attack_from(&mut w,DamageSource::new(DamageKind::OutOfWorld),4);}
        if kind=="speed"&&t==0 {p.add_potion_effect(PotionEffect::new(1,80,1));}
        if kind=="haste"&&t%100==0 {p.add_potion_effect(PotionEffect::new(3,80,t/100%8));}
        if kind=="fatigue"&&t%100==0 {p.add_potion_effect(PotionEffect::new(4,80,t/100%4));}
        if kind=="nausea"&&t==0 {p.add_potion_effect(PotionEffect::new(9,300,0));}
        if kind=="creativedamage" {
            p.set_creative(true);
            if t%5==0 {p.attack_from(&mut w,DamageSource::new(if t%10==0 {DamageKind::Starve}else {DamageKind::Cactus}),5);}
        }
        if kind=="identity" {
            if t%80==0 {food(&mut p,10,0.0);let s=p.player_state_mut().unwrap();let item=ItemStack::new(260,3,0);s.inventory.main[0]=Some(item);s.inventory.current_item=0;p.start_item_use(item,32);}
            if t%80==4 {p.start_item_use(p.held_item().unwrap(),32);}
            if t%80==8 {p.player_state_mut().unwrap().inventory.main[0].as_mut().unwrap().count=2;}
            if t%80==12 {let s=p.player_state_mut().unwrap();s.inventory.main[1]=s.inventory.main[0].take();s.inventory.current_item=1;}
            if t%80==18 {p.player_state_mut().unwrap().inventory.main[1]=Some(ItemStack::new(260,2,0));}
        }
        if kind=="bow" {
            if t%40==0 {let s=p.player_state_mut().unwrap();s.inventory.main[0]=Some(ItemStack::new(261,1,0));s.inventory.main[1]=Some(ItemStack::new(262,16,0));s.inventory.current_item=0;p.use_item(&mut w);}
            if t%40==[1,3,10,20,30][(t/40) as usize%5] {p.stop_item_use(&mut w);}
        }
        if kind=="creativecontroller" {
            w.set_block_with_notify(4,64,4,1);
            if t%11==0 {w.set_block_with_notify(4,64,5,51);}
            let mut controller=std::mem::take(&mut p.player_state_mut().unwrap().controller);
            if t%13==0 {controller.click_block(&mut p,&mut w,4,64,4,3);}
            else if t%13==4 {/* Creative resetBlockRemoving is deliberately a no-op. */}
            else {controller.dig(&mut p,&mut w,4,64,4,3);}
            p.player_state_mut().unwrap().controller=controller;
        }
        if t%13==0 {p.swing_item();}
        p.on_update(&mut w);frame(&mut w,&mut p,&mut o);
    }
    o
}
