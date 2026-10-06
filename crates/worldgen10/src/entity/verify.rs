//! Oracle geometry replay; invoked by compare and the mandatory verify suite.
use super::{AxisAlignedBB as BB, MovingObjectPosition, Vec3D};
use super::{Entity, EntityKind, EntitySeedSource};
use crate::JavaRandom;
use crate::{
    World,
    block::*,
    world::chunk::{Chunk, idx},
    world::tile::ItemStack,
};
pub fn geometry_bytes() -> Vec<u8> {
    let mut r = JavaRandom::new(1729);
    let mut out = Vec::new();
    fn d(out: &mut Vec<u8>, x: f64) {
        out.extend_from_slice(&x.to_bits().to_le_bytes());
    }
    for _ in 0..10000 {
        let x = r.next_double() * 200.0 - 100.0;
        let y = r.next_double() * 200.0 - 100.0;
        let z = r.next_double() * 200.0 - 100.0;
        let a = BB::new(
            x,
            y,
            z,
            x + r.next_double() * 4.0,
            y + r.next_double() * 4.0,
            z + r.next_double() * 4.0,
        );
        let mut v = Vec3D::new(
            r.next_double() * 200.0 - 100.0,
            r.next_double() * 200.0 - 100.0,
            r.next_double() * 200.0 - 100.0,
        );
        let u = Vec3D::new(
            r.next_double() * 200.0 - 100.0,
            r.next_double() * 200.0 - 100.0,
            r.next_double() * 200.0 - 100.0,
        );
        let n = v.normalize();
        let c = v.cross(u);
        for x in [
            n.x,
            n.y,
            n.z,
            c.x,
            c.y,
            c.z,
            v.distance(u),
            v.square_distance(u),
            v.length(),
        ] {
            d(&mut out, x);
        }
        let angle = r.next_float() * 20.0 - 10.0;
        v.rotate_x(angle);
        v.rotate_y(angle);
        for x in [v.x, v.y, v.z] {
            d(&mut out, x);
        }
        match a.ray_trace(v, u) {
            None => out.extend_from_slice(&(-1i32).to_le_bytes()),
            Some(MovingObjectPosition::Block { side, hit, .. }) => {
                out.extend_from_slice(&side.to_le_bytes());
                for x in [hit.x, hit.y, hit.z] {
                    d(&mut out, x);
                }
            }
            _ => unreachable!(),
        }
        let b = a.translated(
            r.next_double() * 8.0 - 4.0,
            r.next_double() * 8.0 - 4.0,
            r.next_double() * 8.0 - 4.0,
        );
        let delta = r.next_double() * 8.0 - 4.0;
        for x in [
            a.calculate_x_offset(b, delta),
            a.calculate_y_offset(b, delta),
            a.calculate_z_offset(b, delta),
            a.average_edge_length(),
        ] {
            d(&mut out, x);
        }
        out.push(a.intersects(b) as u8);
        out.push(a.contains(v) as u8);
        d(&mut out, r.next_gaussian());
        out.extend_from_slice(&r.state().to_le_bytes());
        d(&mut out, crate::jmath::java_atan2(v.x, u.z));
    }
    out
}

pub fn block_bytes() -> Vec<u8> {
    let mut w = World::new(0, 300);
    for cx in -3..=3 {
        for cz in -3..=3 {
            let mut c = Chunk::new(
                cx,
                cz,
                vec![0; 32768].into_boxed_slice().try_into().unwrap(),
            );
            for x in 0..16 {
                for z in 0..16 {
                    c.blocks[idx(x, 63, z)] = 1;
                }
            }
            c.loaded = true;
            c.populated = true;
            w.chunks.insert((cx, cz), Box::new(c));
        }
    }
    fn i(o: &mut Vec<u8>, v: i32) {
        o.extend_from_slice(&v.to_le_bytes());
    }
    fn d(o: &mut Vec<u8>, v: f64) {
        o.extend_from_slice(
            &(if v.is_nan() {
                0x7ff8000000000000
            } else {
                v.to_bits()
            })
            .to_le_bytes(),
        );
    }
    let mut r = JavaRandom::new(979);
    let mut out = Vec::new();
    for _ in 0..10000 {
        let id = 1 + r.next_int(96);
        let meta = r.next_int(16);
        let c = w.chunk_mut(0, 0);
        c.blocks[idx(8, 64, 8)] = id as u8;
        c.data.set(8, 64, 8, meta);
        let x = 8.5 + r.next_double() * 3.0 - 1.5;
        let y = 64.5 + r.next_double() * 3.0 - 1.5;
        let z = 8.5 + r.next_double() * 3.0 - 1.5;
        let b = BB::new(
            x,
            y,
            z,
            x + r.next_double() * 2.0,
            y + r.next_double() * 2.0,
            z + r.next_double() * 2.0,
        );
        let boxes = w.get_colliding_bounding_boxes(None, b);
        i(&mut out, boxes.len() as i32);
        for b in boxes {
            for v in [b.min_x, b.min_y, b.min_z, b.max_x, b.max_y, b.max_z] {
                d(&mut out, v);
            }
        }
        let start = Vec3D::new(
            8.5 + r.next_double() * 4.0 - 2.0,
            64.5 + r.next_double() * 4.0 - 2.0,
            8.5 + r.next_double() * 4.0 - 2.0,
        );
        let end = Vec3D::new(
            8.5 + r.next_double() * 4.0 - 2.0,
            64.5 + r.next_double() * 4.0 - 2.0,
            8.5 + r.next_double() * 4.0 - 2.0,
        );
        let liquids = r.next_bool();
        let no_box = r.next_bool();
        match w.ray_trace_blocks_options(start, end, liquids, no_box) {
            None => i(&mut out, -1),
            Some(MovingObjectPosition::Block { x, y, z, side, hit }) => {
                for v in [side, x, y, z] {
                    i(&mut out, v);
                }
                for v in [hit.x, hit.y, hit.z] {
                    d(&mut out, v);
                }
            }
            _ => unreachable!(),
        }
    }
    out
}

pub fn snapshot(w: &mut World, out: &mut Vec<u8>) {
    fn i(o: &mut Vec<u8>, v: i32) {
        o.extend_from_slice(&v.to_le_bytes());
    }
    fn l(o: &mut Vec<u8>, v: i64) {
        o.extend_from_slice(&v.to_le_bytes());
    }
    fn d(o: &mut Vec<u8>, v: f64) {
        o.extend_from_slice(
            &(if v.is_nan() {
                0x7ff8000000000000
            } else {
                v.to_bits()
            })
            .to_le_bytes(),
        );
    }
    fn f(o: &mut Vec<u8>, v: f32) {
        o.extend_from_slice(&(if v.is_nan() { 0x7fc00000 } else { v.to_bits() }).to_le_bytes());
    }
    let mut entities: Vec<_> = w
        .loaded_entity_list
        .iter()
        .filter_map(|id| w.entities.get(id))
        .chain(w.weather_effects.iter())
        .collect();
    entities.sort_by_key(|e| e.entity_id);
    i(out, entities.len() as i32);
    for e in entities {
        i(out, e.entity_id);
        let s = e.kind.name().as_bytes();
        out.extend_from_slice(&(s.len() as u16).to_be_bytes());
        out.extend_from_slice(s);
        for v in [e.pos, e.motion, e.prev_pos, e.last_tick_pos] {
            for v in [v.x, v.y, v.z] {
                d(out, v);
            }
        }
        let b = e.bounding_box;
        for v in [b.min_x, b.min_y, b.min_z, b.max_x, b.max_y, b.max_z] {
            d(out, v);
        }
        for v in [
            e.yaw,
            e.pitch,
            e.prev_yaw,
            e.prev_pitch,
            e.fall_distance,
            e.y_size,
            e.distance_walked,
        ] {
            f(out, v);
        }
        for v in [
            e.fire,
            e.air,
            e.ticks_existed,
            e.chunk_x,
            e.chunk_y,
            e.chunk_z,
        ] {
            i(out, v);
        }
        for v in [e.on_ground, e.dead, e.in_water, e.added_to_chunk] {
            out.push(v as u8);
        }
        match e.kind {
            EntityKind::Item {
                item,
                age,
                pickup_delay,
                health,
                hover_start,
                ..
            } => {
                for v in [
                    age,
                    pickup_delay,
                    item.id as i32,
                    item.count as i32,
                    item.damage as i32,
                ] {
                    i(out, v);
                }
                f(out, hover_start);
                i(out, health);
            }
            EntityKind::FallingSand { block, fall_time } => {
                i(out, block as i32);
                i(out, fall_time);
            }
            _ => {}
        }
        if e.kind.is_living() {
            let a = &e.living;
            for v in [
                a.health,
                a.prev_health,
                a.hearts_life,
                a.hearts_halves_life,
                a.hurt_time,
                a.max_hurt_time,
                a.death_time,
                a.attack_time,
                a.living_sound_time,
                a.entity_age,
                a.last_damage,
            ] {
                i(out, v);
            }
            for v in [
                a.attacked_at_yaw,
                a.render_yaw_offset,
                a.prev_render_yaw_offset,
                a.limb_swing,
                a.limb_amount,
                a.prev_limb_amount,
                a.swing_progress,
                a.prev_swing_progress,
                a.walk_animation,
                a.prev_walk_animation,
                a.walk_phase,
                a.prev_walk_phase,
                a.random_animation,
                a.animation_speed,
                a.random_scale,
                a.move_strafing,
                a.move_forward,
                a.random_yaw_velocity,
            ] {
                f(out, v);
            }
            out.push(a.is_jumping as u8);
            match e.kind {
                EntityKind::Pig { saddled } => out.push(saddled as u8),
                EntityKind::Sheep {
                    sheared,
                    fleece_color,
                } => {
                    out.push(sheared as u8);
                    i(out, fleece_color as i32);
                }
                EntityKind::Chicken {
                    flap,
                    prev_flap,
                    flap_amount,
                    prev_flap_amount,
                    flap_speed,
                    egg_timer,
                } => {
                    for v in [flap, prev_flap, flap_amount, prev_flap_amount, flap_speed] {
                        f(out, v);
                    }
                    i(out, egg_timer);
                }
                _ => {}
            }
        }
        if let EntityKind::Tnt { fuse } = e.kind {
            i(out, fuse);
        }
        if let EntityKind::Snowball(s) | EntityKind::Egg(s) = &e.kind {
            for v in [s.tile[0], s.tile[1], s.tile[2], s.block as i32] {
                i(out, v);
            }
            out.push(s.in_ground as u8);
            for v in [s.shake, s.owner.unwrap_or(-1), s.ticks_ground, s.ticks_air] {
                i(out, v);
            }
        }
        if let EntityKind::Arrow(s) = &e.kind {
            for v in [s.tile[0], s.tile[1], s.tile[2], s.block as i32, s.metadata] {
                i(out, v);
            }
            out.push(s.in_ground as u8);
            out.push(s.belongs_to_player as u8);
            for v in [s.shake, s.owner.unwrap_or(-1), s.ticks_ground, s.ticks_air] {
                i(out, v);
            }
        }
        if let EntityKind::Creeper {
            ignition,
            prev_ignition,
            state,
            powered,
        } = e.kind
        {
            i(out, ignition);
            i(out, prev_ignition);
            i(out, state);
            out.push(powered as u8);
        }
        if let EntityKind::PigZombie { anger, sound_delay } = e.kind {
            i(out, anger);
            i(out, sound_delay);
        }
        if let EntityKind::Wolf(s) = &e.kind {
            i(out, s.flags as i32);
            let bytes = s.owner.as_bytes();
            out.extend_from_slice(&(bytes.len() as u16).to_be_bytes());
            out.extend_from_slice(bytes);
            i(out, s.watched_health);
            for v in [s.interested, s.shaking, s.shake_active] {
                out.push(v as u8);
            }
            for v in [s.interest, s.prev_interest, s.shake_time, s.prev_shake_time] {
                f(out, v);
            }
        }
        if let EntityKind::Ghast(s) = &e.kind {
            i(out, s.course_cooldown);
            for v in [s.waypoint.x, s.waypoint.y, s.waypoint.z] {
                d(out, v);
            }
            for v in [s.aggro_cooldown, s.prev_attack_counter, s.attack_counter] {
                i(out, v);
            }
            out.push(s.attacking as u8);
        }
        if let EntityKind::Fireball(s) = &e.kind {
            for v in [
                s.arrow.tile[0],
                s.arrow.tile[1],
                s.arrow.tile[2],
                s.arrow.block as i32,
            ] {
                i(out, v);
            }
            out.push(s.arrow.in_ground as u8);
            for v in [
                s.arrow.shake,
                s.arrow.owner.unwrap_or(-1),
                s.arrow.ticks_ground,
                s.arrow.ticks_air,
            ] {
                i(out, v);
            }
            for v in [s.acceleration.x, s.acceleration.y, s.acceleration.z] {
                d(out, v);
            }
        }
        if let EntityKind::Slime {
            size,
            squish,
            prev_squish,
            jump_delay,
        } = e.kind
        {
            i(out, size);
            f(out, squish);
            f(out, prev_squish);
            i(out, jump_delay);
        }
        if let EntityKind::Squid(s) = &e.kind {
            for v in [
                s.pitch,
                s.prev_pitch,
                s.yaw,
                s.prev_yaw,
                s.rotation,
                s.prev_rotation,
                s.tentacle_angle,
                s.prev_tentacle_angle,
                s.speed,
                s.rotation_speed,
                s.yaw_speed,
                s.motion_x,
                s.motion_y,
                s.motion_z,
            ] {
                f(out, v);
            }
        }
        if let EntityKind::Lightning {
            state,
            seed,
            remaining,
        } = e.kind
        {
            i(out, state);
            l(out, seed);
            i(out, remaining);
        }
        if let Some(s) = e.player_state() {
            super::player::verify::state_bytes(s, e, out);
        }
        if let EntityKind::Minecart(s)=&e.kind {
            for v in [s.cart_type,s.damage,s.time_since_hit,s.rock_direction,s.fuel] { i(out,v); }
            d(out,s.push_x);d(out,s.push_z);out.push(s.reversed as u8);
            i(out,s.items.len() as i32);
            for item in &s.items { super::player::verify::write(out,*item); }
        }
        if let EntityKind::Boat(s)=&e.kind { for v in [s.damage,s.time_since_hit,s.rock_direction] { i(out,v); } }
        if matches!(e.kind,EntityKind::Boat(..)|EntityKind::Minecart(..)) {
            for v in [e.collided_horizontally,e.collided_vertically,e.collided,e.been_attacked,e.first_update,e.prevent_entity_spawning] {out.push(v as u8);}
            for v in [e.collision_reduction,e.width,e.height,e.y_offset,e.step_height,e.prev_distance_walked] {f(out,v);}
        }
        i(out,e.riding.unwrap_or(-1));i(out,e.ridden_by.unwrap_or(-1));d(out,e.rider_yaw_delta);d(out,e.rider_pitch_delta);
        l(out, e.rand.state());
    }
    l(out, w.rand.state());
    l(out, w.math_random.state());
    l(
        out,
        match w.entity_seed_source {
            EntitySeedSource::Deterministic(n) => n,
            _ => panic!("deterministic scenario required"),
        },
    );
    l(
        out,
        match w.explosion_seed_source {
            EntitySeedSource::Deterministic(n) => n,
            _ => panic!("deterministic explosion source required"),
        },
    );
    l(out, w.world_time);
    for v in [
        w.update_lcg,
        w.sound_counter,
        w.skylight_subtracted,
        w.lightning_flash,
        w.lightning_sky_flash,
        w.info.rain_time,
        w.info.thunder_time,
    ] {
        i(out, v);
    }
    out.push(w.info.raining as u8);
    out.push(w.info.thundering as u8);
    for v in [
        w.raining_strength,
        w.prev_raining_strength,
        w.thundering_strength,
        w.prev_thundering_strength,
    ] {
        f(out, v);
    }
    let mut spawners = Vec::new();
    for c in w.chunks.values() {
        for (&(x, y, z), te) in &c.tile_entities {
            if let crate::world::tile::TileEntity::MobSpawner { delay, .. } = te {
                spawners.push((c.x * 16 + x, y, c.z * 16 + z, *delay));
            }
        }
    }
    spawners.sort();
    i(out, spawners.len() as i32);
    for (x, y, z, delay) in spawners {
        for v in [x, y, z, delay] {
            i(out, v);
        }
        let (yaw, prev) = w
            .spawner_animations
            .get(&(x, y, z))
            .copied()
            .unwrap_or_default();
        d(out, yaw);
        d(out, prev);
    }
    crate::world::events::write_events(&w.drain_events(), out);
    for e in w.entities.values_mut() {
        if let Some(s) = e.player_state_mut() {
            s.stat_log.clear();
        }
    }
}
pub fn object_bytes(scenario: &str, ticks: i32) -> Vec<u8> {
    let _remote_eye = super::creature::stand_in_eye_height(1.82);
    let mut w = World::with_dimension(
        0,
        300,
        if scenario == "nether" {
            crate::world::Dimension::Nether
        } else {
            crate::world::Dimension::Overworld
        },
    );
    w.set_allowed_mob_spawns(false, false);
    w.set_entity_determinism(100, 200, 0);
    w.players.push((0.5, 65.62, 0.5));
    let passive = scenario == "combat"
        || scenario == "nbtall"
        || scenario == "slimes"
        || scenario == "passive"
        || scenario == "aquatic"
        || scenario == "hostile"
        || scenario == "special"
        || scenario == "spawning"
        || scenario == "spawner"
        || matches!(scenario, "client" | "stormclient")
        || scenario == "nbt"
        || scenario == "nether"
        || scenario == "lightning"
        || scenario == "rider";
    let radius = if passive { 12 } else { 3 };
    for cx in -radius..=radius {
        for cz in -radius..=radius {
            let mut c = Chunk::new(
                cx,
                cz,
                vec![0; 32768].into_boxed_slice().try_into().unwrap(),
            );
            for x in 0..16 {
                for z in 0..16 {
                    c.blocks[idx(x, if scenario == "slimes" { 7 } else { 63 }, z)] =
                        if passive { 2 } else { 1 };
                }
            }
            if passive {
                c.sky_light.0.fill(255);
                c.height_map.fill(if scenario == "slimes" { 8 } else { 64 });
            }
            // new Chunk(world, ...) reads world.worldProvider.hasNoSky (1.0 Chunk.getBlockLightValue).
            c.has_no_sky10 = w.dimension.has_no_sky();
            c.loaded = true;
            c.populated = true;
            w.chunks.insert((cx, cz), Box::new(c));
            w.chunk_list.push((cx, cz));
        }
    }
    fn raw(w: &mut World, x: i32, y: i32, z: i32, id: u8, meta: i32) {
        let c = w.chunk_mut(x >> 4, z >> 4);
        c.blocks[idx(x & 15, y, z & 15)] = id;
        c.data.set(x & 15, y, z & 15, meta);
    }
    for x in 10..=15 {
        for y in 64..=67 {
            for z in -2..=2 {
                raw(&mut w, x, y, z, WATER_STILL, (x + y + z) & 7);
            }
        }
    }
    for x in -15..=-10 {
        for y in 64..=67 {
            for z in -2..=2 {
                raw(&mut w, x, y, z, LAVA_STILL, 0);
            }
        }
    }
    if scenario == "hostile" {
        w.skylight_subtracted = 11;
        w.world_time = 18000;
    }
    let ground = [
        STONE,
        ICE,
        SOUL_SAND,
        STAIRS_WOOD,
        STAIRS_COBBLESTONE,
        FENCE,
        CACTUS,
        SNOW,
    ];
    for k in 0..if scenario == "nbtall" {
        18
    } else if scenario == "spawning" || scenario == "spawner" {
        0
    } else if passive {
        8
    } else {
        16
    } {
        let x = -24 + k * 3;
        raw(&mut w, x, 63, 5, ground[k as usize % 8], k % 4);
        if scenario == "hooks" {
            raw(
                &mut w,
                x,
                64,
                5,
                [30, 81, 70, 72, 90, 0, 0, 0][k as usize % 8],
                0,
            );
            if k % 8 >= 5 {
                raw(&mut w, x, 63, 5, [60, 73, 74][k as usize % 8 - 5], 0);
            }
        }
        let e = if scenario == "slimes" {
            let mut e = Entity::living(&mut w, "Slime").unwrap();
            e.set_slime_size(4);
            e.set_location_and_angles((-14 + k * 4) as f64 + 0.5, 8.0, 8.5, (k * 30) as f32, 0.0);
            e
        } else if scenario == "nbtall" {
            let mut e = match k {
                0 => Entity::item(&mut w, x as f64 + 0.5, 70.0, 5.5, ItemStack::new(35, 3, 7)),
                1 => Entity::falling_sand(&mut w, x as f64 + 0.5, 77.0, 5.5, 12),
                2 => Entity::tnt(&mut w, x as f64 + 0.5, 70.0, 5.5),
                3 => Entity::arrow(&mut w, x as f64 + 0.5, 70.0, 5.5),
                _ => Entity::living(
                    &mut w,
                    [
                        "Item",
                        "FallingSand",
                        "PrimedTnt",
                        "Arrow",
                        "Pig",
                        "Cow",
                        "Sheep",
                        "Chicken",
                        "Squid",
                        "Slime",
                        "Wolf",
                        "Zombie",
                        "Skeleton",
                        "Spider",
                        "Creeper",
                        "PigZombie",
                        "Giant",
                        "Ghast",
                    ][k as usize],
                )
                .unwrap(),
            };
            e.set_location_and_angles(
                (-34 + k * 4) as f64 + 0.5,
                if k == 17 { 78.0 } else { 64.0 },
                12.5,
                (k * 30) as f32,
                0.0,
            );
            if let EntityKind::Pig { saddled } = &mut e.kind {
                *saddled = true;
            }
            if let EntityKind::Sheep {
                sheared,
                fleece_color,
            } = &mut e.kind
            {
                *sheared = true;
                *fleece_color = 14;
            }
            if let EntityKind::Creeper { powered, .. } = &mut e.kind {
                *powered = true;
            }
            if let EntityKind::Wolf(s) = &mut e.kind {
                s.flags = 5;
                s.owner = "Player".into();
            }
            e
        } else if scenario == "aquatic" {
            let mut e = Entity::living(&mut w, if k < 4 { "Squid" } else { "Slime" }).unwrap();
            e.set_location_and_angles(
                if k < 4 {
                    10.5 + k as f64
                } else {
                    (-14 + (k - 4) * 4) as f64 + 0.5
                },
                if k < 4 { 65.0 } else { 64.0 },
                if k < 4 { 0.5 } else { 8.5 },
                (k * 30) as f32,
                0.0,
            );
            e
        } else if scenario == "rider" {
            let mut e =
                Entity::living(&mut w, if k % 2 == 0 { "Spider" } else { "Skeleton" }).unwrap();
            e.set_location_and_angles(
                (-14 + (k / 2) * 6) as f64 + 0.5,
                64.0,
                8.5,
                (k * 30) as f32,
                0.0,
            );
            if k % 2 == 1 {
                let parent = *w.loaded_entity_list.last().unwrap();
                e.riding = Some(parent);
                w.entities.get_mut(&parent).unwrap().ridden_by = Some(e.entity_id);
            }
            e
        } else if scenario == "nether" {
            let mut e = Entity::living(&mut w, if k < 4 { "Ghast" } else { "PigZombie" }).unwrap();
            e.set_location_and_angles(
                (-14 + k * 4) as f64 + 0.5,
                if k < 4 { 78.0 } else { 64.0 },
                8.5,
                (k * 30) as f32,
                0.0,
            );
            e
        } else if scenario == "nbt" {
            let mut e = if k == 0 {
                Entity::item(&mut w, x as f64 + 0.5, 70.0, 5.5, ItemStack::new(35, 3, 7))
            } else if k == 1 {
                Entity::falling_sand(&mut w, x as f64 + 0.5, 77.0, 5.5, 12)
            } else {
                Entity::living(
                    &mut w,
                    [
                        "Item",
                        "FallingSand",
                        "Pig",
                        "Cow",
                        "Sheep",
                        "Chicken",
                        "Squid",
                        "Slime",
                    ][k as usize],
                )
                .unwrap()
            };
            e.set_location_and_angles((-14 + k * 4) as f64 + 0.5, 64.0, 8.5, (k * 30) as f32, 0.0);
            e
        } else if scenario == "combat" {
            let mut e = Entity::living(&mut w, if k < 4 { "Wolf" } else { "PigZombie" }).unwrap();
            e.set_location_and_angles((-14 + k * 4) as f64 + 0.5, 64.0, 8.5, (k * 30) as f32, 0.0);
            e
        } else if scenario == "special" {
            let mut e = Entity::living(&mut w, if k < 4 { "Wolf" } else { "Ghast" }).unwrap();
            e.set_location_and_angles(
                (-14 + k * 4) as f64 + 0.5,
                if k < 4 { 64.0 } else { 78.0 },
                8.5,
                (k * 30) as f32,
                0.0,
            );
            e
        } else if scenario == "hostile" {
            let mut e = Entity::living(
                &mut w,
                [
                    "Zombie",
                    "Skeleton",
                    "Spider",
                    "Creeper",
                    "PigZombie",
                    "Giant",
                    "Zombie",
                    "Skeleton",
                ][k as usize],
            )
            .unwrap();
            e.set_location_and_angles((-14 + k * 4) as f64 + 0.5, 64.0, 8.5, (k * 30) as f32, 0.0);
            e
        } else if scenario == "tnt" {
            Entity::tnt(&mut w, x as f64 + 0.5, (70 + k % 4) as f64, 5.5)
        } else if scenario == "arrows" {
            let mut e = Entity::arrow(
                &mut w,
                x as f64 + 0.5,
                (70 + k % 4) as f64,
                if k < 8 { 0.5 } else { 5.5 },
            );
            e.set_arrow_heading(0.2 + k as f64 * 0.03, -0.2, 0.1, 1.5, 1.0);
            e
        } else if passive {
            let mut e = Entity::living(
                &mut w,
                if scenario == "lightning" && k == 4 {
                    "Creeper"
                } else {
                    ["Pig", "Cow", "Sheep", "Chicken"][k as usize % 4]
                },
            )
            .unwrap();
            e.set_location_and_angles((-14 + k * 4) as f64 + 0.5, 64.0, 8.5, (k * 30) as f32, 0.0);
            e
        } else if scenario == "sand" {
            let block = if k % 2 == 0 { SAND } else { GRAVEL };
            raw(&mut w, x, 77 + k % 4, 5, block, 0);
            Entity::falling_sand(
                &mut w,
                x as f64 + 0.5,
                (77 + k % 4) as f64 + 0.5,
                5.5,
                block,
            )
        } else {
            Entity::item(
                &mut w,
                x as f64 + 0.5,
                if scenario == "hooks" {
                    64.2
                } else {
                    (70 + k % 4) as f64
                },
                if scenario == "hooks" {
                    5.5
                } else if k < 8 {
                    0.5
                } else {
                    5.5
                },
                ItemStack::new(1 + k, 1 + k % 3, k % 4),
            )
        };
        w.entity_joined_world(e);
    }
    if scenario == "spawning" || scenario == "spawner" {
        w.skylight_subtracted = 11;
        w.world_time = 18000;
    }
    if scenario == "spawner" {
        for c in w.chunks.values_mut() {
            c.sky_light.0.fill(0);
        }
        w.set_block_with_notify(4, 64, 4, 52);
        if let Some(crate::world::tile::TileEntity::MobSpawner { mob, .. }) =
            w.get_block_tile_entity(4, 64, 4)
        {
            *mob = "Zombie".into();
        }
    }
    if matches!(scenario, "lightning" | "stormclient") {
        w.entities
            .get_mut(&0)
            .unwrap()
            .set_location_and_angles(-0.5, 64.0, 8.5, 0.0, 0.0);
        w.info.raining = true;
        w.info.thundering = true;
        w.info.rain_time = 1000000;
        w.info.thunder_time = 1000000;
        w.raining_strength = 1.0;
        w.prev_raining_strength = 1.0;
        w.thundering_strength = 1.0;
        w.prev_thundering_strength = 1.0;
        let bolt = Entity::lightning(&mut w, 0.5, 64.0, 8.5);
        w.weather_effects.push(bolt);
    }
    if scenario == "hooks" {
        w.set_block_with_notify(0, 64, 12, 54);
        if let Some(crate::world::tile::TileEntity::Chest(slots)) =
            w.get_block_tile_entity(0, 64, 12)
        {
            slots[0] = Some(ItemStack::new(35, 64, 7));
            slots[5] = Some(ItemStack::new(264, 43, 0));
        }
        w.set_block_with_notify(8, 64, 12, 46);
        w.set_block_with_notify(-8, 68, 12, 1);
        w.set_block_with_notify(-8, 69, 12, 12);
    }
    if matches!(scenario, "client" | "stormclient") {
        w.set_allowed_mob_spawns(true, true);
    }
    if scenario == "slimes" {
        w.players[0].1 = 9.62;
        w.skylight_subtracted = 11;
        w.world_time = 18000;
    }
    let mut out = Vec::new();
    snapshot(&mut w, &mut out);
    for t in 0..ticks {
        if scenario == "combat" && (t == 50 || t == 100) {
            w.damage_entity(
                if t == 50 { 2 } else { 5 },
                Some(super::living::Target::Player(0)),
                1,
            );
        }
        if scenario == "hooks" && t == 0 {
            w.set_block_with_notify(0, 64, 12, 0);
            w.ignite_tnt(8, 64, 12);
            w.set_block_with_notify(8, 64, 12, 0);
            w.set_block_with_notify(-8, 68, 12, 0);
        }
        if scenario == "slimes" && t == 100 {
            let ids = w.loaded_entity_list.clone();
            for id in ids {
                w.damage_entity(id, None, 4);
            }
        }
        if matches!(scenario, "client" | "stormclient") {
            w.client_tick(0, t + 1);
        } else {
            w.update_entities();
            if scenario == "hooks" {
                w.world_time += 1;
                w.tick_updates(false);
                w.random_display_updates(0, 65, 0);
            }
            if matches!(scenario, "slimes" | "spawning" | "nether") {
                w.perform_spawning(true, true);
            }
        }
        if (scenario == "nbt" && t % 100 == 99) || (scenario == "nbtall" && t % 25 == 24) {
            let tags: Vec<_> = w
                .loaded_entity_list
                .iter()
                .filter_map(|id| w.entities[id].to_nbt())
                .collect();
            w.entities.clear();
            w.loaded_entity_list.clear();
            for c in w.chunks.values_mut() {
                for list in &mut c.entity_lists {
                    list.clear();
                }
            }
            for n in tags {
                let e = Entity::from_nbt(&mut w, &n).unwrap();
                w.entity_joined_world(e);
            }
        }
        snapshot(&mut w, &mut out);
    }
    append_world(&w, &mut out);
    out
}

pub fn append_world(w: &World, out: &mut Vec<u8>) {
    fn i(o: &mut Vec<u8>, v: i32) {
        o.extend_from_slice(&v.to_le_bytes());
    }
    out.extend_from_slice(b"B173WLD1");
    for _ in 0..3 {
        i(out, 0);
    }
    i(out, w.chunks.len() as i32);
    let mut keys: Vec<_> = w.chunks.keys().copied().collect();
    keys.sort();
    for (cx, cz) in keys {
        let c = w.chunk(cx, cz);
        i(out, cx);
        i(out, cz);
        out.push(c.populated as u8);
        out.extend_from_slice(&c.blocks[..]);
        out.extend_from_slice(&c.data.0[..]);
        out.extend_from_slice(&c.sky_light.0[..]);
        out.extend_from_slice(&c.block_light.0[..]);
        out.extend_from_slice(&c.height_map);
        let mut tiles: Vec<_> = c.tile_entities.keys().copied().collect();
        tiles.sort();
        let raw: Vec<_> = c
            .raw_tile_entities
            .iter()
            .filter(|n| matches!(n.string("id").as_str(), "Furnace" | "Trap"))
            .collect();
        tiles.extend(
            raw.iter()
                .map(|n| (n.int("x") - cx * 16, n.int("y"), n.int("z") - cz * 16)),
        );
        tiles.sort();
        i(out, tiles.len() as i32);
        for (x, y, z) in tiles {
            if let Some(n) = raw
                .iter()
                .find(|n| n.int("x") == cx * 16 + x && n.int("y") == y && n.int("z") == cz * 16 + z)
            {
                for v in [cx * 16 + x, y, cz * 16 + z] {
                    i(out, v);
                }
                let furnace = n.string("id") == "Furnace";
                out.push(if furnace { 3 } else { 4 });
                for slot in 0..if furnace { 3 } else { 9 } {
                    let item = n
                        .list("Items")
                        .iter()
                        .find_map(|t| {
                            if let crate::nbt::Tag::Compound(c) = t {
                                if c.byte("Slot") as i32 == slot {
                                    Some(ItemStack::new(
                                        c.short("id") as i32,
                                        c.byte("Count") as i32,
                                        c.short("Damage") as i32,
                                    ))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .unwrap_or(ItemStack::new(-1, 0, 0));
                    out.extend_from_slice(&(item.id as i16).to_le_bytes());
                    out.push(item.count as u8);
                    out.extend_from_slice(&item.damage.to_le_bytes());
                }
                if furnace {
                    out.extend_from_slice(&n.short("BurnTime").to_le_bytes());
                    out.extend_from_slice(&n.short("CookTime").to_le_bytes());
                }
                continue;
            }
            for v in [cx * 16 + x, y, cz * 16 + z] {
                i(out, v);
            }
            match &c.tile_entities[&(x, y, z)] {
                crate::world::tile::TileEntity::BrewingStand(_)|crate::world::tile::TileEntity::EnchantmentTable(_)|crate::world::tile::TileEntity::EndPortal=>panic!("legacy worldBytes does not encode release tiles"),
                // Shared Oracle.worldBytes also rejects RecordPlayer tiles.
                // Their original-bytecode persistence is covered by record4.
                crate::world::tile::TileEntity::RecordPlayer { .. } => {
                    panic!("shared worldBytes does not encode RecordPlayer")
                }
                crate::world::tile::TileEntity::Note(n)=> {
                    out.push(5); out.push(n.note as u8); out.push(n.powered as u8);
                }
                crate::world::tile::TileEntity::Piston(p)=> {
                    out.push(6);
                    for v in [p.block_id as i32,p.block_data,p.facing,p.progress.to_bits() as i32,p.last_progress.to_bits() as i32] { i(out,v); }
                    out.push(p.extending as u8);out.push(p.render_head as u8);
                }
                crate::world::tile::TileEntity::Sign(sign) => {
                    out.push(7);
                    for s in &sign.text { i(out,s.len() as i32);out.extend_from_slice(s.as_bytes()); }
                }
                crate::world::tile::TileEntity::Chest(slots) => {
                    out.push(1);
                    for slot in slots.iter() {
                        let item = slot.unwrap_or(ItemStack::new(-1, 0, 0));
                        out.extend_from_slice(&(item.id as i16).to_le_bytes());
                        out.push(item.count as u8);
                        out.extend_from_slice(&item.damage.to_le_bytes());
                    }
                }
                crate::world::tile::TileEntity::MobSpawner { mob, .. } => {
                    out.push(2);
                    out.push(mob.len() as u8);
                    out.extend_from_slice(mob.as_bytes());
                }
                crate::world::tile::TileEntity::Furnace(f) => {
                    out.push(3);
                    for s in f.items {
                        let s: ItemStack = s
                            .unwrap_or(crate::entity::player::ItemStack::new(-1, 0, 0))
                            .into();
                        out.extend_from_slice(&(s.id as i16).to_le_bytes());
                        out.push(s.count as u8);
                        out.extend_from_slice(&s.damage.to_le_bytes());
                    }
                    out.extend_from_slice(&(f.burn as i16).to_le_bytes());
                    out.extend_from_slice(&(f.cook as i16).to_le_bytes());
                }
                crate::world::tile::TileEntity::Dispenser(items) => {
                    out.push(4);
                    for s in items.iter() {
                        let s: ItemStack = s
                            .unwrap_or(crate::entity::player::ItemStack::new(-1, 0, 0))
                            .into();
                        out.extend_from_slice(&(s.id as i16).to_le_bytes());
                        out.push(s.count as u8);
                        out.extend_from_slice(&s.damage.to_le_bytes());
                    }
                }
            }
        }
    }
    i(out, w.tick_order.len() as i32);
    for &(time, _, key) in &w.tick_order {
        for v in [key.x, key.y, key.z, key.block] {
            i(out, v);
        }
        out.extend_from_slice(&time.to_le_bytes());
    }
    i(out, w.light_queue.len() as i32);
}

/// Every ambient Block.randomDisplayTick override, all metadata and neighbouring faces.
pub fn display_event_bytes() -> Vec<u8> {
    let mut w = World::new(0, 300);
    for cx in -3..=3 {
        for cz in -3..=3 {
            let mut c = Chunk::new(
                cx,
                cz,
                vec![0; 32768].into_boxed_slice().try_into().unwrap(),
            );
            for x in 0..16 {
                for z in 0..16 {
                    c.blocks[idx(x, 63, z)] = 1;
                }
            }
            c.loaded = true;
            c.populated = true;
            w.chunks.insert((cx, cz), Box::new(c));
        }
    }
    fn raw(w: &mut World, x: i32, y: i32, z: i32, id: u8, meta: i32) {
        let c = w.chunk_mut(x >> 4, z >> 4);
        c.blocks[idx(x & 15, y, z & 15)] = id;
        c.data.set(x & 15, y, z & 15, meta);
    }
    let mut o = Vec::new();
    for id in [8, 9, 10, 11, 50, 51, 55, 62, 74, 75, 76, 90, 93, 94] {
        for meta in 0..16 {
            for scene in 0..4 {
                let x = if scene == 3 { -16 } else { 8 };
                let y = if scene == 1 { 65 } else { 64 };
                let z = if scene == 3 { -16 } else { 8 };
                for [dx, dy, dz] in [
                    [0, -1, 0],
                    [0, 1, 0],
                    [-1, 0, 0],
                    [1, 0, 0],
                    [0, 0, -1],
                    [0, 0, 1],
                ] {
                    raw(
                        &mut w,
                        x + dx,
                        y + dy,
                        z + dz,
                        if scene == 2 {
                            17
                        } else if scene == 0 && dy < 0 {
                            1
                        } else {
                            0
                        },
                        0,
                    );
                }
                if id == 90 && scene == 2 {
                    raw(&mut w, x - 1, y, z, 90, 0);
                }
                raw(&mut w, x, y, z, id, meta);
                let mut r = JavaRandom::new(1000 + meta as i64);
                w.rand.set_seed(2000 + id as i64);
                for _ in 0..100 {
                    w.block_display_tick(id, x, y, z, &mut r);
                    crate::world::events::write_events(&w.drain_events(), &mut o);
                    o.extend(w.rand.state().to_le_bytes());
                    o.extend(r.state().to_le_bytes());
                }
            }
        }
    }
    o
}

pub fn music_event_bytes() -> Vec<u8> {
    let mut w = World::new(0, 300);
    w.set_entity_determinism(100, 200, 0);
    for cx in -3..=3 {
        for cz in -3..=3 {
            let mut c = Chunk::new(
                cx,
                cz,
                vec![0; 32768].into_boxed_slice().try_into().unwrap(),
            );
            for x in 0..16 {
                for z in 0..16 {
                    c.blocks[idx(x, 63, z)] = 1;
                }
            }
            c.loaded = true;
            c.populated = true;
            w.chunks.insert((cx, cz), Box::new(c));
        }
    }
    fn raw(w: &mut World, x: i32, y: i32, z: i32, id: u8) {
        w.chunk_mut(x >> 4, z >> 4).blocks[idx(x & 15, y, z & 15)] = id;
        w.chunk_mut(x >> 4, z >> 4).data.set(x & 15, y, z & 15, 0);
    }
    fn emit(w: &mut World, o: &mut Vec<u8>) {
        crate::world::events::write_events(&w.drain_events(), o);
    }
    let mut o = Vec::new();
    for below in [0, 1, 12, 20, 5] {
        w.remove_block_tile_entity(8, 64, 8);
        raw(&mut w, 8, 64, 8, 25);
        raw(&mut w, 8, 63, 8, below);
        for action in 0..30 {
            raw(&mut w, 8, 65, 8, if action == 27 { 1 } else { 0 });
            if action != 26 {
                w.activate_note(8, 64, 8);
            } else {
                w.trigger_note(8, 64, 8);
            }
            emit(&mut w, &mut o);
            let note = match w.get_block_tile_entity(8, 64, 8) {
                Some(crate::world::tile::TileEntity::Note(n)) => n.note,
                _ => 0,
            };
            o.extend((note as i32).to_le_bytes());
        }
    }
    w.remove_block_tile_entity(8, 64, 8);
    raw(&mut w, 8, 64, 8, 84);
    for record in [2256, 2257] {
        w.insert_record(8, 64, 8, record);
        emit(&mut w, &mut o);
        o.extend(w.get_block_metadata(8, 64, 8).to_le_bytes());
        o.extend(record.to_le_bytes());
        w.eject_record(8, 64, 8);
        emit(&mut w, &mut o);
        o.extend(w.get_block_metadata(8, 64, 8).to_le_bytes());
        o.extend(0i32.to_le_bytes());
        o.extend(w.rand.state().to_le_bytes());
        let e = &w.entities[w.loaded_entity_list.last().unwrap()];
        for v in [
            e.pos.x, e.pos.y, e.pos.z, e.motion.x, e.motion.y, e.motion.z,
        ] {
            o.extend(v.to_bits().to_le_bytes());
        }
        o.extend(e.rand.state().to_le_bytes());
        if let EntityKind::Item {
            item,
            pickup_delay,
            hover_start,
            ..
        } = &e.kind
        {
            o.extend(hover_start.to_bits().to_le_bytes());
            o.extend(pickup_delay.to_le_bytes());
            o.extend((item.id as i32).to_le_bytes());
            o.extend((item.count as i32).to_le_bytes());
        }
    }
    w.remove_block_tile_entity(8, 64, 8);
    raw(&mut w, 8, 64, 8, 25);
    raw(&mut w, 8, 63, 8, 1);
    for action in 0..16 {
        raw(&mut w, 8, 65, 8, if action >= 12 { 1 } else { 0 });
        raw(&mut w, 8, 64, 7, 77);
        w.chunk_mut(0, 0)
            .data
            .set(8, 64, 7, if action % 4 < 2 { 13 } else { 5 });
        crate::world::behavior::on_neighbor_block_change(
            &mut w,
            25,
            8,
            64,
            8,
            if action == 5 { 0 } else { 77 },
        );
        emit(&mut w, &mut o);
        w.get_block_tile_entity(8, 64, 8);
        let (note, powered) = match w.get_block_tile_entity(8, 64, 8) {
            Some(crate::world::tile::TileEntity::Note(n)) => (n.note, n.powered),
            _ => (0, false),
        };
        o.extend((note as i32).to_le_bytes());
        o.push(powered as u8);
    }
    for id in 1..=96 {
        let (a, v, p) = crate::world::events::step_sound(id);
        let (b, _, _) = crate::world::events::break_sound(id);
        for text in [a, b] {
            o.extend((text.len() as i32).to_le_bytes());
            o.extend(text.as_bytes());
        }
        o.extend(v.to_bits().to_le_bytes());
        o.extend(p.to_bits().to_le_bytes());
    }
    o
}
