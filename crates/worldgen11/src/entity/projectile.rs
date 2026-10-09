use super::{Entity, EntityKind, MovingObjectPosition as Hit, Vec3D};
use crate::{
    math::{self, PI_F32 as PI},
    World,
};

#[cfg(test)]
mod release_11_impact_tests {
    use super::*;
    #[test]
    fn arrow_impact_matches_original_11_bytecode() {
        let mut world = crate::entity::verify181::world();
        for line in include_str!("../fixtures/arrowhits11.txt").lines() {
            let v: Vec<i64> = line.split_whitespace().map(|s|s.parse().unwrap()).collect();
            world.entities.clear(); world.loaded_entity_list.clear(); world.events.clear();
            let mut target = Entity::living(&mut world, "Cow").unwrap();
            target.set_position(0.0, 64.0, 1.0); target.living.health=1000;
            if v[0]==0 {target.living.hearts_life=20;target.living.last_damage=100000;}
            let target_id = target.entity_id;
            world.entity_joined_world(target);
            let mut arrow = Entity::arrow(&mut world, 0.0, 64.5, 0.0);
            arrow.motion=Vec3D::new(0.2,0.1,1.25);arrow.rand.set_seed(42);
            if v[1]!=0 {arrow.set_fire_seconds(10);}
            if let EntityKind::Arrow(state)=&mut arrow.kind {
                state.critical=v[2]!=0;state.knockback=v[3] as i32;state.damage=f64::from_bits(v[4] as u64);
            }
            arrow.update_arrow(&mut world);
            let target=&world.entities[&target_id];
            if v[0]!=0 {assert_eq!(target.living.last_damage,v[5] as i32,"damage: {line}");}
            assert_eq!(target.fire,v[6] as i32,"burn before acceptance: {line}");
            assert_eq!((target.motion.x.to_bits(),target.motion.y.to_bits(),target.motion.z.to_bits()),
                (v[7] as u64,v[8] as u64,v[9] as u64),"punch velocity: {line}");
            assert_eq!(arrow.dead,v[10]!=0,"accepted impact: {line}");
            assert_eq!(arrow.rand.next_long(),v[11],"critical and hit-sound RNG tail: {line}");
        }
    }
}
#[derive(Clone, Debug)]
pub struct ArrowState {
    pub tile: [i32; 3],
    pub block: u8,
    pub metadata: i32,
    pub in_ground: bool,
    pub belongs_to_player: bool,
    pub shake: i32,
    pub owner: Option<i32>,
    pub ticks_ground: i32,
    pub ticks_air: i32,
    pub critical: bool,
    pub damage: f64,
    pub knockback: i32,
}
impl Default for ArrowState {
    fn default() -> Self {
        Self {
            tile: [-1; 3],
            block: 0,
            metadata: 0,
            in_ground: false,
            belongs_to_player: false,
            shake: 0,
            owner: None,
            ticks_ground: 0,
            ticks_air: 0,
            critical: false,
            damage: 2.0,
            knockback: 0,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct FireballState {
    pub arrow: ArrowState,
    pub acceleration: Vec3D,
}
impl Entity {
    pub fn can_be_collided_with(&self) -> bool {
        if matches!(
            self.kind,
            EntityKind::Dragon(..) | EntityKind::SmallFireball(..)
        ) {
            return false;
        }
        if matches!(
            self.kind,
            EntityKind::DragonPart { .. } | EntityKind::EnderCrystal { .. }
        ) {
            return true;
        }
        matches!(
            self.kind,
            EntityKind::Fireball(..) | EntityKind::Painting(..)
        ) || (!self.dead
            && (self.kind.is_living()
                || matches!(
                    self.kind,
                    EntityKind::FallingSand { .. }
                        | EntityKind::Tnt { .. }
                        | EntityKind::Minecart(..)
                        | EntityKind::Boat(..)
                )))
    }
    pub fn arrow(w: &mut World, x: f64, y: f64, z: f64) -> Self {
        let mut e = Self::base(w, EntityKind::Arrow(Default::default()));
        e.width = 0.5;
        e.height = 0.5;
        e.set_position(x, y, z);
        e
    }
    pub fn arrow_from_owner(w: &mut World, owner: &Entity) -> Self {
        Self::arrow_from_owner_power(w, owner, 1.0)
    }
    pub fn arrow_from_owner_power(w: &mut World, owner: &Entity, power: f32) -> Self {
        let mut e = Self::base(
            w,
            EntityKind::Arrow(ArrowState {
                owner: Some(owner.entity_id),
                belongs_to_player: owner.player_state().is_some(),
                ..Default::default()
            }),
        );
        e.width = 0.5;
        e.height = 0.5;
        e.set_location_and_angles(
            owner.pos.x,
            owner.pos.y + owner.eye_height() as f64,
            owner.pos.z,
            owner.yaw,
            owner.pitch,
        );
        e.pos.x -= (math::cos(e.yaw / 180.0 * PI) * 0.16) as f64;
        e.pos.y -= 0.1f32 as f64;
        e.pos.z -= (math::sin(e.yaw / 180.0 * PI) * 0.16) as f64;
        e.set_position(e.pos.x, e.pos.y, e.pos.z);
        e.set_arrow_heading(
            (-math::sin(e.yaw / 180.0 * PI) * math::cos(e.pitch / 180.0 * PI)) as f64,
            (-math::sin(e.pitch / 180.0 * PI)) as f64,
            (math::cos(e.yaw / 180.0 * PI) * math::cos(e.pitch / 180.0 * PI)) as f64,
            power * 1.5,
            1.0,
        );
        e
    }
    pub fn set_arrow_heading(
        &mut self,
        mut x: f64,
        mut y: f64,
        mut z: f64,
        velocity: f32,
        spread: f32,
    ) {
        let length = (x * x + y * y + z * z).sqrt() as f32 as f64;
        x /= length;
        y /= length;
        z /= length;
        x += self.rand.next_gaussian() * 0.0075f32 as f64 * spread as f64;
        y += self.rand.next_gaussian() * 0.0075f32 as f64 * spread as f64;
        z += self.rand.next_gaussian() * 0.0075f32 as f64 * spread as f64;
        self.motion = Vec3D::new(
            x * velocity as f64,
            y * velocity as f64,
            z * velocity as f64,
        );
        self.projectile_angles();
        self.prev_yaw = self.yaw;
        self.prev_pitch = self.pitch;
        if let EntityKind::Arrow(s) = &mut self.kind {
            s.ticks_ground = 0;
        }
        if let EntityKind::Fish(s) = &mut self.kind {
            s.arrow.ticks_ground = 0;
        }
    }
    pub fn fireball(w: &mut World, pos: Vec3D, aim: Vec3D) -> Self {
        let mut e = Self::base(w, EntityKind::Fireball(Default::default()));
        e.width = 1.0;
        e.height = 1.0;
        e.set_location_and_angles(pos.x, pos.y, pos.z, 0.0, 0.0);
        e.set_position(pos.x, pos.y, pos.z);
        let n = aim.length();
        if let EntityKind::Fireball(s) = &mut e.kind {
            s.acceleration = Vec3D::new(aim.x / n * 0.1, aim.y / n * 0.1, aim.z / n * 0.1);
        }
        e
    }
    pub fn fireball_from_owner(w: &mut World, owner: &Entity, mut aim: Vec3D) -> Self {
        let mut e = Self::base(w, EntityKind::Fireball(Default::default()));
        e.width = 1.0;
        e.height = 1.0;
        e.set_location_and_angles(
            owner.pos.x,
            owner.pos.y,
            owner.pos.z,
            owner.yaw,
            owner.pitch,
        );
        e.set_position(e.pos.x, e.pos.y, e.pos.z);
        aim.x += e.rand.next_gaussian() * 0.4;
        aim.y += e.rand.next_gaussian() * 0.4;
        aim.z += e.rand.next_gaussian() * 0.4;
        let length = aim.length();
        if let EntityKind::Fireball(s) = &mut e.kind {
            s.arrow.owner = Some(owner.entity_id);
            s.acceleration = Vec3D::new(
                aim.x / length * 0.1,
                aim.y / length * 0.1,
                aim.z / length * 0.1,
            );
        }
        e
    }
    pub fn look_vector(&self) -> Option<Vec3D> {
        if !self.kind.is_living() {
            return None;
        }
        // 1.0 EntityLiving.getLook(1.0F): angle * ((float)Math.PI / 180.0F), a folded constant.
        let cy = math::cos(-self.yaw * (PI / 180.0) - PI);
        let sy = math::sin(-self.yaw * (PI / 180.0) - PI);
        let cp = -math::cos(-self.pitch * (PI / 180.0));
        let sp = math::sin(-self.pitch * (PI / 180.0));
        Some(Vec3D::new((sy * cp) as f64, sp as f64, (cy * cp) as f64))
    }
    pub(super) fn fireball_attack(&mut self, w: &World, attacker: Option<i32>) -> bool {
        self.been_attacked = true;
        if let Some(id) = attacker {
            if let Some(v) = w.entities.get(&id).and_then(|e| e.look_vector()) {
                self.motion = v;
                if let EntityKind::Fireball(s) = &mut self.kind {
                    s.acceleration = Vec3D::new(v.x * 0.1, v.y * 0.1, v.z * 0.1);
                    if w.entities.get(&id).is_some_and(|e| e.kind.is_living()) {
                        s.arrow.owner = Some(id);
                    }
                }
            }
            true
        } else {
            false
        }
    }
    pub(super) fn projectile_angles(&mut self) {
        let horizontal =
            (self.motion.x * self.motion.x + self.motion.z * self.motion.z).sqrt() as f32 as f64;
        self.yaw =
            (crate::jmath::java_atan2(self.motion.x, self.motion.z) * 180.0 / PI as f64) as f32;
        self.pitch =
            (crate::jmath::java_atan2(self.motion.y, horizontal) * 180.0 / PI as f64) as f32;
    }
    fn projectile_intersection(
        &self,
        w: &mut World,
        state: &ArrowState,
        fireball: bool,
    ) -> Option<Hit> {
        let start = self.pos;
        let end = self.pos.add(self.motion.x, self.motion.y, self.motion.z);
        let mut hit = w.ray_trace_blocks_options(start, end, false, !fireball);
        let end = match hit {
            Some(Hit::Block { hit, .. }) => hit,
            _ => end,
        };
        let mut closest = 0.0;
        for id in w.entities_within_aabb(
            Some(self.entity_id),
            self.bounding_box
                .add_coord(self.motion.x, self.motion.y, self.motion.z)
                .expand(1.0, 1.0, 1.0),
        ) {
            let e = &w.entities[&id];
            if e.can_be_collided_with()
                && ((!fireball &&Some(id) != state.owner)
                    || (fireball && !e.entity_identity_matches(state.owner)) || state.ticks_air >= if fireball { 25 } else { 5 })
            {
                if let Some(Hit::Block { hit: p, .. }) = e
                    .bounding_box
                    .expand(0.3f32 as f64, 0.3f32 as f64, 0.3f32 as f64)
                    .ray_trace(start, end)
                {
                    let distance = start.distance(p);
                    if distance < closest || closest == 0.0 {
                        closest = distance;
                        hit = Some(Hit::Entity { id, hit: e.pos });
                    }
                }
            }
        }
        hit
    }
    pub(super) fn projectile_advance(
        &mut self,
        w: &mut World,
        drag: f32,
        gravity: f32,
        accel: Vec3D,
    ) {
        self.pos.x += self.motion.x;
        self.pos.y += self.motion.y;
        self.pos.z += self.motion.z;
        self.projectile_angles();
        while self.pitch - self.prev_pitch < -180.0 {
            self.prev_pitch -= 360.0;
        }
        while self.pitch - self.prev_pitch >= 180.0 {
            self.prev_pitch += 360.0;
        }
        while self.yaw - self.prev_yaw < -180.0 {
            self.prev_yaw -= 360.0;
        }
        while self.yaw - self.prev_yaw >= 180.0 {
            self.prev_yaw += 360.0;
        }
        self.pitch = self.prev_pitch + (self.pitch - self.prev_pitch) * 0.2;
        self.yaw = self.prev_yaw + (self.yaw - self.prev_yaw) * 0.2;
        if self.in_water {
            for _ in 0..4 {
                let t = 0.25f32 as f64;
                let p = self.pos.add(-self.motion.x * t, -self.motion.y * t, -self.motion.z * t);
                let v = self.motion;
                w.spawn_particle("bubble", p, v);
            }
        }
        // Arrows do not add zero acceleration: signed zero is observable.
        if matches!(self.kind, EntityKind::Fireball(..) | EntityKind::SmallFireball(..)) {
            self.motion.x += accel.x;
            self.motion.y += accel.y;
            self.motion.z += accel.z;
        }
        self.motion.x *= drag as f64;
        self.motion.y *= drag as f64;
        self.motion.z *= drag as f64;
        if gravity != 0.0 {
            self.motion.y -= gravity as f64;
        }
        self.set_position(self.pos.x, self.pos.y, self.pos.z);
    }
    pub(super) fn update_arrow(&mut self, w: &mut World) {
        self.on_entity_update(w);
        if self.prev_pitch == 0.0 && self.prev_yaw == 0.0 {
            self.projectile_angles();
            self.prev_yaw = self.yaw;
            self.prev_pitch = self.pitch;
        }
        let EntityKind::Arrow(mut s) = self.kind.clone() else {
            return;
        };
        let [x, y, z] = s.tile;
        let id = w.get_block_id(x, y, z);
        if id > 0 {
            w.set_block_bounds_based_on_state(id, x, y, z);
            if w.block_collision_box(id, x, y, z)
                .is_some_and(|b| b.contains(self.pos))
            {
                s.in_ground = true;
            }
        }
        if s.shake > 0 {
            s.shake -= 1;
        }
        if s.in_ground {
            if w.get_block_id(x, y, z) == s.block && w.get_block_metadata(x, y, z) == s.metadata {
                s.ticks_ground += 1;
                if s.ticks_ground == 1200 {
                    self.dead = true;
                }
            } else {
                s.in_ground = false;
                self.motion.x *= (self.rand.next_float() * 0.2) as f64;
                self.motion.y *= (self.rand.next_float() * 0.2) as f64;
                self.motion.z *= (self.rand.next_float() * 0.2) as f64;
                s.ticks_ground = 0;
                s.ticks_air = 0;
            }
        } else {
            s.ticks_air += 1;
            if let Some(hit) = self.projectile_intersection(w, &s, false) {
                match hit {
                    Hit::Entity { id, .. } => {
                        let success = if let Some(mut target) = w.entities.remove(&id) {
                            let speed = self.motion.length() as f32;
                            let mut damage = (speed as f64 * s.damage).ceil() as i32;
                            if s.critical {
                                damage += self.rand.next_int( damage / 2 + 2);
                            }
                            if self.is_burning(w) { target.set_fire_seconds(5); }
                            let hit = target.attack_from(
                                w,
                                super::damage::DamageSource::indirect(
                                    super::damage::DamageKind::Arrow,
                                    self.entity_id,
                                    s.owner.or(Some(self.entity_id)),
                                ),
                                damage,
                            );
                            if hit && target.kind.is_living() {
                                target.living.arrows_stuck += 1;
                                if s.knockback > 0 {
                                    let horizontal = math::sqrt_double(self.motion.x*self.motion.x+self.motion.z*self.motion.z);
                                    if horizontal > 0.0 {
                                        target.add_velocity(w,
                                            self.motion.x * s.knockback as f64 * 0.6f32 as f64 / horizontal as f64,
                                            0.1,
                                            self.motion.z * s.knockback as f64 * 0.6f32 as f64 / horizontal as f64);
                                    }
                                }
                            }
                            w.entities.insert(id, target);
                            hit
                        } else {
                            false
                        };
                        if success {
                            let pitch = 1.2 / (self.rand.next_float() * 0.2 + 0.9);
                            w.play_sound_at_entity(self, "random.bowhit", 1.0, pitch);
                            self.dead = true;
                        } else {
                            self.motion.x *= -0.1f32 as f64;
                            self.motion.y *= -0.1f32 as f64;
                            self.motion.z *= -0.1f32 as f64;
                            self.yaw += 180.0;
                            self.prev_yaw += 180.0;
                            s.ticks_air = 0;
                        }
                    }
                    Hit::Block { x, y, z, hit, .. } => {
                        s.tile = [x, y, z];
                        s.block = w.get_block_id(x, y, z);
                        s.metadata = w.get_block_metadata(x, y, z);
                        self.motion = Vec3D::new(
                            (hit.x - self.pos.x) as f32 as f64,
                            (hit.y - self.pos.y) as f32 as f64,
                            (hit.z - self.pos.z) as f32 as f64,
                        );
                        let n = self.motion.length();
                        self.pos.x -= self.motion.x / n * 0.05f32 as f64;
                        self.pos.y -= self.motion.y / n * 0.05f32 as f64;
                        self.pos.z -= self.motion.z / n * 0.05f32 as f64;
                        let pitch = 1.2 / (self.rand.next_float() * 0.2 + 0.9);
                        w.play_sound_at_entity(self, "random.bowhit", 1.0, pitch);
                        s.in_ground = true;
                        s.shake = 7;
                        s.critical = false;
                    }
                }
            }
            if s.critical {
                for i in 0..4 {
                    let t = i as f64 / 4.0;
                    w.spawn_particle(
                        "crit",
                        self.pos
                            .add(self.motion.x * t, self.motion.y * t, self.motion.z * t),
                        Vec3D::new(-self.motion.x, -self.motion.y + 0.2, -self.motion.z),
                    );
                }
            }
            self.projectile_advance(
                w,
                if self.in_water { 0.8 } else { 0.99 },
                0.05,
                Vec3D::default(),
            );
        }
        self.kind = EntityKind::Arrow(s);
    }
    pub(super) fn update_fireball(&mut self, w: &mut World) {
        self.on_entity_update(w);
        self.set_fire_seconds(1);
        let small = matches!(self.kind, EntityKind::SmallFireball(..));
        let mut s = match &self.kind { EntityKind::Fireball(s) | EntityKind::SmallFireball( s) => s.clone(),
            _ =>
            return,
        };
        if !w.multiplayer_world
            && s.arrow.owner
                .is_none_or(|id| w.entities.get(&id).is_none_or(|e| e.dead)) {
            self.dead= true;
        }
        if s.arrow.in_ground {
            let [x, y, z] = s.arrow.tile;
            if w.get_block_id(x, y, z) == s.arrow.block {
                s.arrow.ticks_ground += 1;
                if s.arrow.ticks_ground == 1200 {
                    self.dead = true;
                }
                self.kind = if small {
                    EntityKind::SmallFireball(s)
                } else { EntityKind::Fireball(s)
                };
                return;
            }
            s.arrow.in_ground = false;
            self.motion.x *= (self.rand.next_float() * 0.2) as f64;
            self.motion.y *= (self.rand.next_float() * 0.2) as f64;
            self.motion.z *= (self.rand.next_float() * 0.2) as f64;
            s.arrow.ticks_ground = 0;
            s.arrow.ticks_air = 0;
        } else {
            s.arrow.ticks_air += 1;
        }
        let impact = self.projectile_intersection(w, &s.arrow, true);
        if let Some(hit) = impact {
            self.fireball_impact(w, hit, &s);
        }
        self.projectile_advance(
            w,
            if self.in_water { 0.8 } else { 0.95 },
            0.0,
            s.acceleration,
        );
        w.spawn_particle("smoke", self.pos.add(0.0, 0.5, 0.0), Vec3D::default());
        self.kind = if small {
            EntityKind::SmallFireball(s)
        } else { EntityKind::Fireball(s)
        };
    }
    pub(super) fn fireball_impact(&mut self, w: &mut World, hit: Hit, s: &FireballState) {
        if matches!(self.kind, EntityKind::SmallFireball(..)) {
            self.small_fireball_impact(w, hit, s);
            return;
        }
        if w.multiplayer_world {
            return;
        }
        if let Hit::Entity { id, .. } = hit {
            if let Some(mut e) = w.entities.remove(&id) {
                e.attack_from(
                    w,
                    super::damage::DamageSource::indirect(
                        super::damage::DamageKind::Fireball,
                        self.entity_id,
                        s.arrow.owner,
                    ),
                    4,
                );
                w.entities.insert(id, e);
            }
        }
        // The fireball is still in the world during its own explosion: at zero distance the
        // knockback direction is NaN, and that NaN reaches its later motion and smoke.
        let me = self.entity_id;
        let mut copy = self.clone();
        copy.kind = EntityKind::Fireball(s.clone());
        w.entities.insert(me, copy);
        w.new_explosion(None, self.pos, 1.0, true);
        if let Some(back) = w.entities.remove(&me) {
            self.motion = back.motion;
            self.velocity_changed = back.velocity_changed;
        }
        self.dead = true;
    }
}
