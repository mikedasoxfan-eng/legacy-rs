//! Release 1.0 EntityDragon, EntityDragonBase, DragonPart and EntityEnderCrystal.
use super::{
    damage::{DamageKind, DamageSource},
    data_watcher::Value,
    living::Target,
    AxisAlignedBB, Entity, EntityKind, Vec3D,
};
use crate::{
    math::{self, floor_double as floor, PI_F32 as PI},
    World,
};

#[derive(Clone, Debug)]
pub struct DragonState {
    pub target_position: Vec3D,
    pub history: [[f64; 3]; 64],
    pub history_index: i32,
    pub parts: [i32; 7],
    pub previous_animation: f32,
    pub animation: f32,
    pub force_new_target: bool,
    pub slowed: bool,
    pub target: Option<Target>,
    pub death_ticks: i32,
    pub healing_crystal: Option<i32>,
}
impl Default for DragonState {
    fn default() -> Self {
        Self {
            target_position: Vec3D::new(0.0, 100.0, 0.0),
            history: [[0.0; 3]; 64],
            history_index: -1,
            parts: [0; 7],
            previous_animation: 0.0,
            animation: 0.0,
            force_new_target: false,
            slowed: false,
            target: None,
            death_ticks: 0,
            healing_crystal: None,
        }
    }
}
impl DragonState {
    pub fn movement_offsets(&self, offset: i32, mut partial: f32, health: i32) -> [f64; 3] {
        if health <= 0 {
            partial = 0.0;
        }
        partial = 1.0 - partial;
        let a = self.history[((self.history_index - offset) & 63) as usize];
        let b = self.history[((self.history_index - offset - 1) & 63) as usize];
        [
            a[0] + wrap(b[0] - a[0]) * partial as f64,
            a[1] + (b[1] - a[1]) * partial as f64,
            a[2] + (b[2] - a[2]) * partial as f64,
        ]
    }
}
fn wrap(mut a: f64) -> f64 {
    while a < -180.0 {
        a += 360.0;
    }
    while a >= 180.0 {
        a -= 360.0;
    }
    a
}
impl Entity {
    pub(super) fn init_dragon(&mut self, w: &mut World) {
        let mut parts = [0; 7];
        for (i, (name, width, height)) in [
            ("head", 6.0, 6.0),
            ("body", 8.0, 8.0),
            ("tail", 4.0, 4.0),
            ("tail", 4.0, 4.0),
            ("tail", 4.0, 4.0),
            ("wing", 4.0, 4.0),
            ("wing", 4.0, 4.0),
        ]
        .into_iter()
        .enumerate()
        {
            let mut part = Self::base(
                w,
                EntityKind::DragonPart {
                    parent: self.entity_id,
                    name,
                },
            );
            part.width = width;
            part.height = height;
            parts[i] = part.entity_id;
            // Parts have IDs and Randoms, but are not independently joined or ticked.
            w.entities.insert(part.entity_id, part);
        }
        if let EntityKind::Dragon(s) = &mut self.kind {
            s.parts = parts;
        }
        self.living.health = 200;
        self.width = 16.0;
        self.height = 8.0;
        self.no_clip = true;
        self.immune_to_fire = true;
    }
    /// EntityEnderCrystal(World, x, y, z).
    pub fn ender_crystal(w: &mut World, p: Vec3D) -> Self {
        let mut e = Self::ender_crystal_base(w);
        e.set_position(p.x, p.y, p.z);
        e
    }
    /// EntityEnderCrystal(World): setSize does not move the box Entity's constructor placed.
    pub fn ender_crystal_base(w: &mut World) -> Self {
        let mut e = Self::base(
            w,
            EntityKind::EnderCrystal {
                rotation: 0,
                health: 5,
            },
        );
        e.prevent_entity_spawning = true;
        e.width = 2.0;
        e.height = 2.0;
        e.y_offset = 1.0;
        let rotation = e.rand.next_int(100000);
        e.kind = EntityKind::EnderCrystal {
            rotation,
            health: 5,
        };
        e
    }
    pub(super) fn update_ender_crystal(&mut self, w: &mut World) {
        self.prev_pos = self.pos;
        if let EntityKind::EnderCrystal { rotation, health } = &mut self.kind {
            *rotation = rotation.wrapping_add(1);
            self.data_watcher.update(8, Value::Int(*health)).unwrap();
        }
        let (x, y, z) = (floor(self.pos.x), floor(self.pos.y), floor(self.pos.z));
        if w.get_block_id(x, y, z) != 51 {
            w.set_block_with_notify(x, y, z, 51);
        }
    }
    pub fn attack_dragon_part(
        &mut self,
        w: &mut World,
        head: bool,
        source: DamageSource,
        mut damage: i32,
    ) -> bool {
        if !head {
            damage = damage / 4 + 1;
        }
        let angle = self.yaw * PI / 180.0;
        let p = Vec3D::new(
            self.pos.x
                + (math::sin(angle) * 5.0) as f64
                + ((self.rand.next_float() - 0.5) * 2.0) as f64,
            self.pos.y + (self.rand.next_float() * 3.0) as f64 + 1.0,
            self.pos.z - (math::cos(angle) * 5.0) as f64
                + ((self.rand.next_float() - 0.5) * 2.0) as f64,
        );
        if let EntityKind::Dragon(s) = &mut self.kind {
            s.target_position = p;
            s.target = None;
        }
        if source.kind == DamageKind::Explosion
            || source.attacker.is_some_and(|id| {
                id < 0
                    || w.entities
                        .get(&id)
                        .is_some_and(|e| e.player_state().is_some())
            })
        {
            self.living_attack(w, source, damage);
        }
        true
    }
    fn dragon_new_target(&mut self, w: &World, s: &mut DragonState) {
        s.force_new_target = false;
        if self.rand.next_int(2) == 0 && !w.players.is_empty() {
            s.target = Some(Target::Player(
                self.rand.next_int(w.players.len() as i32) as usize
            ));
        } else {
            loop {
                s.target_position =
                    Vec3D::new(0.0, (70.0 + self.rand.next_float() * 50.0) as f64, 0.0);
                s.target_position.x += (self.rand.next_float() * 120.0 - 60.0) as f64;
                s.target_position.z += (self.rand.next_float() * 120.0 - 60.0) as f64;
                if self.pos.square_distance(s.target_position) > 100.0 {
                    break;
                }
            }
            s.target = None;
        }
    }
    pub(super) fn dragon_living_update(&mut self, w: &mut World) {
        let EntityKind::Dragon(state) = &self.kind else {
            return;
        };
        let mut s = (**state).clone();
        s.previous_animation = s.animation;
        if !w.multiplayer_world {
            self.data_watcher
                .update(16, Value::Int(self.living.health))
                .unwrap();
        }
        if self.living.health <= 0 {
            let p = self.pos.add(
                ((self.rand.next_float() - 0.5) * 8.0) as f64,
                2.0 + ((self.rand.next_float() - 0.5) * 4.0) as f64,
                ((self.rand.next_float() - 0.5) * 8.0) as f64,
            );
            w.spawn_particle("largeexplode", p, Vec3D::default());
            self.kind = EntityKind::Dragon(Box::new(s));
            return;
        }
        if let Some(id) = s.healing_crystal {
            if w.entities.get(&id).is_none_or(|e| e.dead) {
                if !w.multiplayer_world {
                    self.attack_dragon_part(w, true, DamageSource::new(DamageKind::Explosion), 10);
                    if let EntityKind::Dragon(current) = &self.kind {
                        s.target_position = current.target_position;
                        s.target = current.target;
                    }
                }
                s.healing_crystal = None;
            } else if self.ticks_existed % 10 == 0 && self.living.health < 200 {
                self.living.health += 1;
            }
        }
        if self.rand.next_int(10) == 0 {
            let mut best = f64::MAX;
            s.healing_crystal = None;
            for id in w.entities_within_aabb(
                Some(self.entity_id),
                self.bounding_box.expand(32.0, 32.0, 32.0),
            ) {
                let e = &w.entities[&id];
                if matches!(e.kind, EntityKind::EnderCrystal { .. }) {
                    let d = e.pos.square_distance(self.pos);
                    if d < best {
                        best = d;
                        s.healing_crystal = Some(id);
                    }
                }
            }
        }
        let mut speed = 0.2
            / (((self.motion.x * self.motion.x + self.motion.z * self.motion.z).sqrt() as f32)
                * 10.0
                + 1.0);
        speed *= 2.0f64.powf(self.motion.y) as f32;
        s.animation += if s.slowed { speed * 0.5 } else { speed };
        while self.yaw >= 180.0 {
            self.yaw -= 360.0;
        }
        while self.yaw < -180.0 {
            self.yaw += 360.0;
        }
        if s.history_index < 0 {
            for h in &mut s.history {
                h[0] = self.yaw as f64;
                h[1] = self.pos.y;
            }
        }
        s.history_index += 1;
        if s.history_index == 64 {
            s.history_index = 0;
        }
        s.history[s.history_index as usize][0] = self.yaw as f64;
        s.history[s.history_index as usize][1] = self.pos.y;
        if w.multiplayer_world {
            self.interpolate_network(w, false);
        } else {
            let dx = s.target_position.x - self.pos.x;
            let mut dy = s.target_position.y - self.pos.y;
            let dz = s.target_position.z - self.pos.z;
            let distance = dx * dx + dy * dy + dz * dz;
            if let Some(t) = s.target {
                if let Some((p, _, _)) = self.target_position(w, t) {
                    s.target_position.x = p.x;
                    s.target_position.z = p.z;
                    let x = p.x - self.pos.x;
                    let z = p.z - self.pos.z;
                    let offset = (0.4f32 as f64 + (x * x + z * z).sqrt() / 80.0 - 1.0).min(10.0);
                    let bottom = w
                        .target_entity_id(t)
                        .and_then(|id| w.entities.get(&id))
                        .map_or(p.y, |e| e.bounding_box.min_y);
                    s.target_position.y = bottom + offset;
                }
            } else {
                s.target_position.x += self.rand.next_gaussian() * 2.0;
                s.target_position.z += self.rand.next_gaussian() * 2.0;
            }
            if s.force_new_target
                || distance < 100.0
                || distance > 22500.0
                || self.collided_horizontally
                || self.collided_vertically
            {
                self.dragon_new_target(w, &mut s);
            }
            dy /= (dx * dx + dz * dz).sqrt() as f32 as f64;
            let limit = 0.6f32 as f64;
            if dy < -limit {
                dy = -limit;
            }
            if dy > limit {
                dy = limit;
            }
            self.motion.y += dy * 0.1f32 as f64;
            while self.yaw < -180.0 {
                self.yaw += 360.0;
            }
            while self.yaw >= 180.0 {
                self.yaw -= 360.0;
            }
            let desired = 180.0 - crate::jmath::java_atan2(dx, dz) * 180.0 / PI as f64;
            let turn = wrap(desired - self.yaw as f64).clamp(-50.0, 50.0);
            let direction = Vec3D::new(
                s.target_position.x - self.pos.x,
                s.target_position.y - self.pos.y,
                s.target_position.z - self.pos.z,
            )
            .normalize();
            let facing = Vec3D::new(
                math::sin(self.yaw * PI / 180.0) as f64,
                self.motion.y,
                (-math::cos(self.yaw * PI / 180.0)) as f64,
            )
            .normalize();
            let mut alignment = (facing.dot(direction) + 0.5) as f32 / 1.5;
            if alignment < 0.0 {
                alignment = 0.0;
            }
            self.living.random_yaw_velocity *= 0.8;
            let horizontal = self.motion.x * self.motion.x + self.motion.z * self.motion.z;
            let fspeed = horizontal.sqrt() as f32 * 1.0 + 1.0;
            let dspeed = (horizontal.sqrt() * 1.0 + 1.0).min(40.0);
            self.living.random_yaw_velocity = (self.living.random_yaw_velocity as f64
                + turn * (0.7f32 as f64 / dspeed / fspeed as f64))
                as f32;
            self.yaw += self.living.random_yaw_velocity * 0.1;
            let scale = (2.0 / (dspeed + 1.0)) as f32;
            self.move_flying(0.0, -1.0, 0.06 * (alignment * scale + (1.0 - scale)));
            let slow = if s.slowed { 0.8f32 as f64 } else { 1.0 };
            self.move_entity(
                w,
                self.motion.x * slow,
                self.motion.y * slow,
                self.motion.z * slow,
            );
            let mut drag = (self.motion.normalize().dot(facing) + 1.0) as f32 / 2.0;
            drag = 0.8 + 0.15 * drag;
            self.motion.x *= drag as f64;
            self.motion.z *= drag as f64;
            self.motion.y *= 0.91f32 as f64;
        }
        self.living.render_yaw_offset = self.yaw;
        let h5 = s.movement_offsets(5, 1.0, self.living.health);
        let pitch =
            (h5[1] - s.movement_offsets(10, 1.0, self.living.health)[1]) as f32 * 10.0 / 180.0 * PI;
        let c = math::cos(pitch);
        let sp = -math::sin(pitch);
        let angle = self.yaw * PI / 180.0;
        let sin = math::sin(angle);
        let cos = math::cos(angle);
        self.position_dragon_part(
            w,
            s.parts[1],
            5.0,
            3.0,
            self.pos.add((sin * 0.5) as f64, 0.0, (-cos * 0.5) as f64),
        );
        self.position_dragon_part(
            w,
            s.parts[5],
            4.0,
            2.0,
            self.pos.add((cos * 4.5) as f64, 2.0, (sin * 4.5) as f64),
        );
        self.position_dragon_part(
            w,
            s.parts[6],
            4.0,
            3.0,
            self.pos.add((-cos * 4.5) as f64, 2.0, (-sin * 4.5) as f64),
        );
        if !w.multiplayer_world && self.living.max_hurt_time == 0 {
            let body = w.entities[&s.parts[1]].bounding_box;
            let center_x = (body.min_x + body.max_x) / 2.0;
            let center_z = (body.min_z + body.max_z) / 2.0;
            for part in [s.parts[5], s.parts[6]] {
                let b = w.entities[&part]
                    .bounding_box
                    .expand(4.0, 2.0, 4.0)
                    .translated(0.0, -2.0, 0.0);
                for id in w.entities_within_aabb(Some(self.entity_id), b) {
                    if let Some(e) = w.entities.get_mut(&id) {
                        if e.kind.is_living() {
                            let x = e.pos.x - center_x;
                            let z = e.pos.z - center_z;
                            let d = x * x + z * z;
                            e.motion.x += x / d * 4.0;
                            e.motion.y += 0.2f32 as f64;
                            e.motion.z += z / d * 4.0;
                            e.velocity_changed = true;
                        }
                    }
                }
            }
            let b = w.entities[&s.parts[0]].bounding_box.expand(1.0, 1.0, 1.0);
            for id in w.entities_within_aabb(Some(self.entity_id), b) {
                if let Some(mut e) = w.entities.remove(&id) {
                    if e.kind.is_living() {
                        w.entities.insert(self.entity_id, self.clone());
                        e.attack_from(
                            w,
                            DamageSource::entity(DamageKind::Mob, Some(self.entity_id)),
                            10,
                        );
                        w.entities.remove(&self.entity_id);
                    }
                    w.entities.insert(id, e);
                }
            }
        }
        let h0 = s.movement_offsets(0, 1.0, self.living.health);
        let head_angle = angle - self.living.random_yaw_velocity * 0.01;
        self.position_dragon_part(
            w,
            s.parts[0],
            3.0,
            3.0,
            Vec3D::new(
                self.pos.x + (math::sin(head_angle) * 5.5 * c) as f64,
                self.pos.y + (h0[1] - h5[1]) * 1.0 + (sp * 5.5) as f64,
                self.pos.z - (math::cos(head_angle) * 5.5 * c) as f64,
            ),
        );
        for i in 0..3 {
            let h = s.movement_offsets(12 + i as i32 * 2, 1.0, self.living.health);
            let a = angle + wrap(h[0] - h5[0]) as f32 * PI / 180.0 * 1.0;
            let length = (i + 1) as f32 * 2.0;
            self.position_dragon_part(
                w,
                s.parts[2 + i],
                2.0,
                2.0,
                Vec3D::new(
                    self.pos.x - ((sin * 1.5 + math::sin(a) * length) * c) as f64,
                    self.pos.y + (h[1] - h5[1]) * 1.0 - ((length + 1.5) * sp) as f64 + 1.5,
                    self.pos.z + ((cos * 1.5 + math::cos(a) * length) * c) as f64,
                ),
            );
        }
        if !w.multiplayer_world {
            // Java uses |: both boxes must be scanned, including both RNG draws.
            let head = w.entities[&s.parts[0]].bounding_box;
            let body = w.entities[&s.parts[1]].bounding_box;
            s.slowed = self.dragon_destroy_blocks(w, head) | self.dragon_destroy_blocks(w, body);
        }
        self.kind = EntityKind::Dragon(Box::new(s));
    }
    fn position_dragon_part(&mut self, w: &mut World, id: i32, width: f32, height: f32, p: Vec3D) {
        if let Some(mut part) = w.entities.remove(&id) {
            part.width = width;
            part.height = height;
            part.on_entity_update(w);
            part.set_location_and_angles(p.x, p.y, p.z, 0.0, 0.0);
            w.entities.insert(id, part);
        }
    }
    fn dragon_destroy_blocks(&mut self, w: &mut World, b: AxisAlignedBB) -> bool {
        let mut blocked = false;
        let mut destroyed = false;
        for x in floor(b.min_x)..=floor(b.max_x) {
            for y in floor(b.min_y)..=floor(b.max_y) {
                for z in floor(b.min_z)..=floor(b.max_z) {
                    let block = w.get_block_id(x, y, z);
                    if block != 0 {
                        if matches!(block, 49 | 121 | 7) {
                            blocked = true;
                        } else {
                            destroyed = true;
                            w.set_block_with_notify(x, y, z, 0);
                        }
                    }
                }
            }
        }
        if destroyed {
            let p = Vec3D::new(
                b.min_x + (b.max_x - b.min_x) * self.rand.next_float() as f64,
                b.min_y + (b.max_y - b.min_y) * self.rand.next_float() as f64,
                b.min_z + (b.max_z - b.min_z) * self.rand.next_float() as f64,
            );
            w.spawn_particle("largeexplode", p, Vec3D::default());
        }
        blocked
    }
    pub(super) fn dragon_death_update(&mut self, w: &mut World) {
        let EntityKind::Dragon(s) = &mut self.kind else {
            return;
        };
        s.death_ticks += 1;
        let tick = s.death_ticks;
        if (180..=200).contains(&tick) {
            let p = self.pos.add(
                ((self.rand.next_float() - 0.5) * 8.0) as f64,
                2.0 + ((self.rand.next_float() - 0.5) * 4.0) as f64,
                ((self.rand.next_float() - 0.5) * 8.0) as f64,
            );
            w.spawn_particle("hugeexplosion", p, Vec3D::default());
        }
        if !w.multiplayer_world && tick > 150 && tick % 5 == 0 {
            self.dragon_drop_xp(w, 1000);
        }
        self.move_entity(w, 0.0, 0.1f32 as f64, 0.0);
        self.yaw += 20.0;
        self.living.render_yaw_offset = self.yaw;
        if tick == 200 {
            self.dragon_drop_xp(w, 10000);
            self.rand.next_int(2);
            self.rand.next_int(2);
            self.rand.next_int(2);
            self.dragon_exit_portal(w, floor(self.pos.x), floor(self.pos.z));
            self.set_dead(w);
        }
    }
    fn dragon_drop_xp(&self, w: &mut World, mut amount: i32) {
        while amount > 0 {
            let value = super::xp::split_xp(amount);
            amount -= value;
            let orb = Self::xp_orb(w, self.pos, value);
            w.entity_joined_world(orb);
        }
    }
    fn dragon_exit_portal(&self, w: &mut World, x: i32, z: i32) {
        let level = 64; // World.field_35472_c is 128 in release 1.0.
        for y in level - 1..=level + 32 {
            for bx in x - 4..=x + 4 {
                for bz in z - 4..=z + 4 {
                    let dx = (bx - x) as f64;
                    let dz = (bz - z) as f64;
                    let radius = (dx * dx + dz * dz).sqrt() as f32 as f64;
                    if radius <= 3.5 {
                        let block = if y < level {
                            if radius <= 2.5 {
                                Some(7)
                            } else {
                                None
                            }
                        } else if y > level {
                            Some(0)
                        } else if radius > 2.5 {
                            Some(7)
                        } else {
                            Some(119)
                        };
                        if let Some(block) = block {
                            w.set_block_with_notify(bx, y, bz, block);
                        }
                    }
                }
            }
        }
        for (dx, dy, dz, block) in [
            (0, 0, 0, 7),
            (0, 1, 0, 7),
            (0, 2, 0, 7),
            (-1, 2, 0, 50),
            (1, 2, 0, 50),
            (0, 2, -1, 50),
            (0, 2, 1, 50),
            (0, 3, 0, 7),
            (0, 4, 0, 122),
        ] {
            w.set_block_with_notify(x + dx, level + dy, z + dz, block);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn multipart_constructor_and_damage_routing() {
        let mut w = super::super::verify181::world();
        let mut dragon = Entity::living(&mut w, "EnderDragon").unwrap();
        dragon.set_position(0.0, 100.0, 0.0);
        let EntityKind::Dragon(s) = &dragon.kind else {
            panic!()
        };
        assert_eq!(
            s.parts,
            [
                dragon.entity_id + 1,
                dragon.entity_id + 2,
                dragon.entity_id + 3,
                dragon.entity_id + 4,
                dragon.entity_id + 5,
                dragon.entity_id + 6,
                dragon.entity_id + 7
            ]
        );
        assert_eq!(dragon.living.health, 200);
        assert_eq!(dragon.data_watcher.int(16).unwrap(), 0);
        assert!(!dragon.can_be_collided_with());
        assert!(!dragon.attack_from(&mut w, DamageSource::new(DamageKind::Explosion), 12));
        assert!(dragon.attack_dragon_part(
            &mut w,
            false,
            DamageSource::new(DamageKind::Explosion),
            12
        ));
        assert_eq!(dragon.living.health, 196);
        dragon.living.hearts_life = 0;
        dragon.attack_dragon_part(&mut w, true, DamageSource::new(DamageKind::Explosion), 12);
        assert_eq!(dragon.living.health, 184);
    }
    #[test]
    fn history_wraps_yaw_and_death_forces_previous_sample() {
        let mut s = DragonState::default();
        s.history_index = 0;
        s.history[0] = [179.0, 80.0, 2.0];
        s.history[63] = [-179.0, 100.0, 4.0];
        assert_eq!(s.movement_offsets(0, 0.5, 200), [180.0, 90.0, 3.0]);
        assert_eq!(s.movement_offsets(0, 0.5, 0), [181.0, 100.0, 4.0]);
    }
    #[test]
    fn crystal_tick_does_not_run_entity_base_tick() {
        let mut w = super::super::verify181::world();
        let mut e = Entity::ender_crystal(&mut w, Vec3D::new(0.0, 64.0, 0.0));
        let EntityKind::EnderCrystal { rotation, .. } = e.kind else {
            panic!()
        };
        assert_eq!(e.data_watcher.int(8).unwrap(), 0);
        e.on_update(&mut w);
        assert_eq!(e.ticks_existed, 0);
        assert_eq!(e.data_watcher.int(8).unwrap(), 5);
        assert!(matches!(e.kind,EntityKind::EnderCrystal {rotation:r,..} if r==rotation+1));
        assert_eq!(w.get_block_id(0, 64, 0), 51);
    }
    #[test]
    fn death_rewards_and_exit_portal_at_tick_200() {
        let mut w = super::super::verify181::world();
        let mut dragon = Entity::living(&mut w, "EnderDragon").unwrap();
        dragon.set_position(0.0, 80.0, 0.0);
        dragon.living.health = 0;
        for _ in 0..199 {
            dragon.dragon_death_update(&mut w);
            assert!(!dragon.dead);
        }
        let total: i32 = w
            .entities
            .values()
            .filter_map(|e| {
                if let EntityKind::XPOrb(s) = &e.kind {
                    Some(s.value)
                } else {
                    None
                }
            })
            .sum();
        assert_eq!(total, 9000);
        dragon.dragon_death_update(&mut w);
        assert!(dragon.dead);
        let total: i32 = w
            .entities
            .values()
            .filter_map(|e| {
                if let EntityKind::XPOrb(s) = &e.kind {
                    Some(s.value)
                } else {
                    None
                }
            })
            .sum();
        assert_eq!(total, 20000);
        assert_eq!(w.get_block_id(0, 68, 0), 122);
        assert_eq!(w.get_block_id(1, 64, 0), 119);
        assert_eq!(w.get_block_id(3, 64, 0), 7);
        assert_eq!(w.get_block_id(0, 67, 0), 7);
    }
}
