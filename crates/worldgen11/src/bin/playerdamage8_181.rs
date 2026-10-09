use worldgen::entity::{
    damage::{DamageKind, DamageSource},
    Entity, Vec3D,
};
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mut w = worldgen::entity::verify181::world();
    let mut out = Vec::new();
    let sources = [
        DamageKind::InFire,
        DamageKind::OnFire,
        DamageKind::Lava,
        DamageKind::InWall,
        DamageKind::Drown,
        DamageKind::Starve,
        DamageKind::Cactus,
        DamageKind::Fall,
        DamageKind::OutOfWorld,
        DamageKind::Generic,
        DamageKind::Explosion,
        DamageKind::Magic,
    ];
    for c in 0..2400 {
        w.entities.clear();
        w.loaded_entity_list.clear();
        let mut p = Entity::player(&mut w, "Player");
        p.set_location_and_angles(0.5, 64.0, 0.5, (c * 17 % 360) as f32, 0.0);
        p.motion = Vec3D::new(0.3, -0.1, -0.4);
        p.living.health = if c % 3 == 0 { 2 } else { 20 };
        p.living.prev_health = 17;
        p.living.hearts_life = if c % 5 == 0 { 20 } else { 0 };
        p.living.last_damage = c % 7;
        let damage = c % 23 + 1;
        let source = if c % 2 == 0 {
            DamageSource::new(sources[(c / 2) as usize % sources.len()])
        } else {
            DamageSource::indirect(DamageKind::Arrow, -99, Some(p.entity_id))
        };
        let hit = p.attack_from(&mut w, source, damage);
        out.push(hit as u8);
        for n in [
            p.living.health,
            p.living.prev_health,
            p.living.hearts_life,
            p.living.hurt_time,
            p.living.max_hurt_time,
            p.living.last_damage,
            p.living.recently_hit,
        ] {
            out.extend(n.to_le_bytes());
        }
        for n in [
            p.motion.x,
            p.motion.y,
            p.motion.z,
            p.bounding_box.min_y,
            p.bounding_box.max_y,
        ] {
            out.extend(n.to_le_bytes());
        }
        for n in [
            p.living.attacked_at_yaw,
            p.width,
            p.height,
            p.y_offset,
            p.player_state().unwrap().food.read().exhaustion,
        ] {
            out.extend(n.to_le_bytes());
        }
        out.push(p.been_attacked as u8);
        out.push(p.velocity_changed as u8);
        out.extend(p.rand.state().to_le_bytes());
        out.extend(w.math_random.state().to_le_bytes());
        let log = &p.player_state().unwrap().stat_log;
        out.extend((log.len() as i32).to_le_bytes());
        for &(id, n) in log {
            out.extend(id.to_le_bytes());
            out.extend(n.to_le_bytes());
        }
        worldgen::world::events::write_events(&w.drain_events(), &mut out);
    }
    std::fs::write(&args[1], &out).unwrap();
    let expected = std::fs::read(&args[2]).unwrap();
    if out != expected {
        let at = out
            .iter()
            .zip(&expected)
            .position(|(a, b)| a != b)
            .unwrap_or(out.len().min(expected.len()));
        eprintln!(
            "PLAYER DAMAGE ROUND 8 DIFFERS byte {at}, lengths {}/{}",
            out.len(),
            expected.len()
        );
        std::process::exit(1);
    }
    println!("PLAYER DAMAGE ROUND 8 EXACT: 2400 environmental/self-owned-arrow hits, lethal and hurt cooldown cases ({} bytes)",out.len());
}
