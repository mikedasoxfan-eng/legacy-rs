use worldgen::entity::{Entity, player::inventory::ItemStack, potion::PotionEffect, Vec3D};
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mut w = worldgen::entity::verify181::world(); let mut out = Vec::new();
    let weapons = [-1,268,272,267,276,283,257,258,256,329];
    for c in 0..600 {
        let mut p = Entity::player(&mut w,"Player"); let mut target = Entity::living(&mut w,"Pig").unwrap();
        p.set_location_and_angles(0.5,64.0,0.5,(c*17%360) as f32,0.0);
        target.set_location_and_angles(0.5,64.0,1.5,0.0,0.0);target.living.health=1000;
        let weapon = weapons[c as usize%weapons.len()];
        if weapon >= 0 { let mut item=ItemStack::new(weapon,1,0);if c%3==0 {item.damage=item.max_damage();}p.player_state_mut().unwrap().inventory.main[0]=Some(item); }
        p.motion=Vec3D::new(0.3,if c%2==0 {-0.1} else {0.1},-0.4);p.on_ground=c%4==0;p.in_water=c%5==0;
        raw(&mut w,if c%7==0 {65}else{0},3);
        p.set_sprinting(c%3==0);if c%11==0 {p.set_creative(true);}
        target.living.hearts_life=if c%6==0 {20}else{0};target.living.last_damage=c%13;
        let id=target.entity_id;w.entities.insert(id,target);p.attack_with_item(&mut w,id);let target=w.entities.remove(&id).unwrap();
        for v in [target.living.health,target.living.hearts_life,target.living.hurt_time,target.living.last_damage] {out.extend(v.to_le_bytes());}
        for v in [p.motion.x,p.motion.y,p.motion.z,target.motion.x,target.motion.y,target.motion.z] {out.extend(v.to_le_bytes());}
        out.push(p.player_state().unwrap().sprinting as u8);out.push(target.velocity_changed as u8);
        out.extend(p.player_state().unwrap().food.exhaustion.to_le_bytes());
        stack(&mut out,p.held_item());
        let stats=&p.player_state().unwrap().stat_log;out.extend((stats.len() as i32).to_le_bytes());for &(id,n) in stats {out.extend(id.to_le_bytes());out.extend(n.to_le_bytes());}
        out.extend(p.rand.state().to_le_bytes());out.extend(target.rand.state().to_le_bytes());
        worldgen::world::events::write_events(&w.drain_events(), &mut out);
    }
    let mut p=Entity::player(&mut w,"Player");p.on_ground=true;
    for block in [1,3,12,14,49,7,18,30] {for weapon in [-1,257,269,359] {for amp in -1..8 {for effect in [3,4] {for state in 0..4 {
        p.living.effects.clear();if amp>=0 {p.add_potion_effect(PotionEffect::new(effect,100,amp));}
        p.player_state_mut().unwrap().inventory.main[0]=if weapon<0 {None}else{Some(ItemStack::new(weapon,1,0))};p.on_ground=state&1==0;
        raw(&mut w,if state&2!=0 {9}else{0},0);p.set_location_and_angles(0.5,64.0,0.5,0.0,0.0);
        out.extend(p.relative_block_hardness(&mut w,block).to_le_bytes());
    }}}}}
    std::fs::write(&args[1],&out).unwrap();let expected=std::fs::read(&args[2]).unwrap();
    if out!=expected {let at=out.iter().zip(&expected).position(|(a,b)|a!=b).unwrap_or(out.len().min(expected.len()));eprintln!("PLAYER COMBAT DIFFERS byte {at}, lengths {}/{}",out.len(),expected.len());std::process::exit(1);}
    println!("PLAYER COMBAT EXACT: 600 attacks and 2304 potion mining cases ({} bytes)",out.len());
}
fn raw(w:&mut worldgen::World,id:u8,meta:i32) {let c=w.chunks.get_mut(&(0,0)).unwrap();let index=worldgen::world::chunk::idx(0,64,0);c.blocks[index]=id;c.data.set(0,64,0,meta);}
fn stack(out:&mut Vec<u8>,s:Option<ItemStack>){for v in s.map_or([-1,0,0,0],|s|[s.id,s.count,s.damage,s.animations]) {out.extend(v.to_le_bytes());}}
