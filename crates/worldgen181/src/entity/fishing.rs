//! EntityFish and ItemFishingRod. Tile coordinates deliberately stay -1 on impact.
use super::{
    AxisAlignedBB as BB, Entity, EntityKind, MovingObjectPosition as Hit, Vec3D,
    projectile::ArrowState,
};
use crate::{
    World,
    block::MAT_WATER,
    entity::player::ItemStack,
    math::{self, PI_F32 as PI, floor_double as floor},
};
#[derive(Clone, Debug, Default)]
pub struct FishState {
    pub arrow: ArrowState,
    pub catchable: i32,
    pub hooked: Option<i32>,
    pub interpolation_ticks: i32,
    pub target: Vec3D,
    pub target_yaw: f64,
    pub target_pitch: f64,
    pub velocity: Vec3D,
}
impl World {
    /// Java's angler keeps its hook object after World removes it from the loaded list.
    pub(crate) fn fish_has_owner_reference(&self, fish: &Entity) -> bool {
        let EntityKind::Fish(s) = &fish.kind else {
            return false;
        };
        s.arrow
            .owner
            .and_then(|id| self.entities.get(&id))
            .and_then(|p| p.player_state())
            .is_some_and(|p| p.fish_entity == Some(fish.entity_id))
    }
}
impl Entity {
    pub fn fish_base(w: &mut World) -> Self {
        let mut e = Self::base(w, EntityKind::Fish(Default::default()));
        e.width = 0.25;
        e.height = 0.25;
        e
    }
    pub fn fish_from_owner(w: &mut World, owner: &mut Entity) -> Self {
        let mut e = Self::fish_base(w);
        if let EntityKind::Fish(s) = &mut e.kind {
            s.arrow.owner = Some(owner.entity_id);
        }
        owner.player_state_mut().unwrap().fish_entity = Some(e.entity_id);
        e.set_location_and_angles(
            owner.pos.x,
            owner.pos.y + 1.62 - owner.y_offset as f64,
            owner.pos.z,
            owner.yaw,
            owner.pitch,
        );
        e.pos.x -= (math::cos(e.yaw / 180.0 * PI) * 0.16) as f64;
        e.pos.y -= 0.1f32 as f64;
        e.pos.z -= (math::sin(e.yaw / 180.0 * PI) * 0.16) as f64;
        e.set_position(e.pos.x, e.pos.y, e.pos.z);
        e.y_offset = 0.0;
        let x = (-math::sin(e.yaw / 180.0 * PI) * math::cos(e.pitch / 180.0 * PI) * 0.4) as f64;
        let z = (math::cos(e.yaw / 180.0 * PI) * math::cos(e.pitch / 180.0 * PI) * 0.4) as f64;
        let y = (-math::sin(e.pitch / 180.0 * PI) * 0.4) as f64;
        e.set_arrow_heading(x, y, z, 1.5, 1.0);
        e
    }
    pub fn set_fish_velocity(&mut self, velocity: Vec3D) {
        if let EntityKind::Fish(s) = &mut self.kind {
            s.velocity = velocity;
        }
        self.motion = velocity;
    }
    pub fn interpolate_fish(&mut self, target: Vec3D, yaw: f32, pitch: f32, ticks: i32) {
        if let EntityKind::Fish(s) = &mut self.kind {
            s.target = target;
            s.target_yaw = yaw as f64;
            s.target_pitch = pitch as f64;
            s.interpolation_ticks = ticks;
            self.motion = s.velocity;
        }
    }
    pub(super) fn update_fish(&mut self, w: &mut World) {
        self.on_entity_update(w);
        let EntityKind::Fish(mut s) = self.kind.clone() else {
            return;
        };
        if s.interpolation_ticks > 0 {
            let n = s.interpolation_ticks as f64;
            let p = self.pos.add(
                (s.target.x - self.pos.x) / n,
                (s.target.y - self.pos.y) / n,
                (s.target.z - self.pos.z) / n,
            );
            let mut yaw = s.target_yaw - self.yaw as f64;
            while yaw < -180.0 {
                yaw += 360.0;
            }
            while yaw >= 180.0 {
                yaw -= 360.0;
            }
            self.yaw = (self.yaw as f64 + yaw / n) as f32;
            self.pitch = (self.pitch as f64 + (s.target_pitch - self.pitch as f64) / n) as f32;
            s.interpolation_ticks -= 1;
            self.set_position(p.x, p.y, p.z);
            self.yaw %= 360.0;
            self.pitch %= 360.0;
            self.kind = EntityKind::Fish(s);
            return;
        }
        let owner = s.arrow.owner.and_then(|id| w.entities.get(&id));
        if !owner.is_some_and(|p| {
            !p.dead
                && p.living.health > 0
                && p.held_item().is_some_and(|s| s.id == 346)
                && self.pos.square_distance(p.pos) <= 1024.0
        }) {
            self.dead = true;
            if let Some(p) = s
                .arrow
                .owner
                .and_then(|id| w.entities.get_mut(&id))
                .and_then(|e| e.player_state_mut())
            {
                p.fish_entity = None;
            }
            return;
        }
        if let Some(id) = s.hooked {
            if let Some(target) = w.entities.get(&id).filter(|e| !e.dead) {
                self.pos = Vec3D::new(
                    target.pos.x,
                    target.bounding_box.min_y + target.height as f64 * 0.8,
                    target.pos.z,
                );
                return;
            }
            s.hooked = None;
        }
        if s.arrow.shake > 0 {
            s.arrow.shake -= 1;
        }
        if s.arrow.in_ground {
            let [x, y, z] = s.arrow.tile;
            if w.get_block_id(x, y, z) == s.arrow.block {
                s.arrow.ticks_ground += 1;
                if s.arrow.ticks_ground == 1200 {
                    self.dead = true;
                }
                self.kind = EntityKind::Fish(s);
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
        let start = self.pos;
        let mut end = self.pos.add(self.motion.x, self.motion.y, self.motion.z);
        let mut hit = w.ray_trace_blocks(start, end);
        if let Some(Hit::Block { hit: p, .. }) = &hit {
            end = *p;
        }
        let mut closest = None;
        let mut distance = 0.0;
        for id in w.entities_within_aabb(
            Some(self.entity_id),
            self.bounding_box
                .add_coord(self.motion.x, self.motion.y, self.motion.z)
                .expand(1.0, 1.0, 1.0),
        ) {
            let target = &w.entities[&id];
            if target.can_be_collided_with()
                && (Some(id) != s.arrow.owner || s.arrow.ticks_air >= 5)
            {
                let bb = target
                    .bounding_box
                    .expand(0.3f32 as f64, 0.3f32 as f64, 0.3f32 as f64);
                if let Some(Hit::Block { hit: p, .. }) = bb.ray_trace(start, end) {
                    let d = start.distance(p);
                    if d < distance || distance == 0.0 {
                        closest = Some(id);
                        distance = d;
                    }
                }
            }
        }
        if let Some(id) = closest {
            hit = Some(Hit::Entity {
                id,
                hit: w.entities[&id].pos,
            });
        }
        if let Some(hit) = hit {
            match hit {
                Hit::Entity { id, .. } => {
                    if let Some(mut target) = w.entities.remove(&id) {
                        if target.attack_from(w,super::damage::DamageSource::indirect(super::damage::DamageKind::Thrown,self.entity_id,s.arrow.owner),0) {
                            s.hooked = Some(id);
                        }
                        w.entities.insert(id, target);
                    }
                }
                Hit::Block { .. } => s.arrow.in_ground = true,
            }
        }
        if !s.arrow.in_ground {
            self.move_entity(w, self.motion.x, self.motion.y, self.motion.z);
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
            let mut friction = if self.on_ground || self.collided_horizontally {
                0.5f32
            } else {
                0.92
            };
            let mut water = 0.0;
            for layer in 0..5 {
                let bb = self.bounding_box;
                let min = bb.min_y + (bb.max_y - bb.min_y) * layer as f64 / 5.0 - 0.125 + 0.125;
                let max =
                    bb.min_y + (bb.max_y - bb.min_y) * (layer + 1) as f64 / 5.0 - 0.125 + 0.125;
                if w.is_aabb_in_material(
                    BB::new(bb.min_x, min, bb.min_z, bb.max_x, max, bb.max_z),
                    MAT_WATER,
                ) {
                    water += 1.0 / 5.0;
                }
            }
            if water > 0.0 {
                if s.catchable > 0 {
                    s.catchable -= 1;
                } else {
                    let chance = if w.can_block_be_rained_on(
                        floor(self.pos.x),
                        floor(self.pos.y) + 1,
                        floor(self.pos.z),
                    ) {
                        300
                    } else {
                        500
                    };
                    if self.rand.next_int(chance) == 0 {
                        s.catchable = self.rand.next_int(30) + 10;
                        self.motion.y -= 0.2f32 as f64;
                        let pitch = 1.0 + (self.rand.next_float() - self.rand.next_float()) * 0.4;
                        w.play_sound_at_entity(self, "random.splash", 0.25, pitch);
                        let y = floor(self.bounding_box.min_y) as f32;
                        let mut i = 0;
                        while (i as f32) < 1.0 + self.width * 20.0 {
                            let x = (self.rand.next_float() * 2.0 - 1.0) * self.width;
                            let z = (self.rand.next_float() * 2.0 - 1.0) * self.width;
                            let vy = self.motion.y - (self.rand.next_float() * 0.2) as f64;
                            w.spawn_particle(
                                "bubble",
                                Vec3D::new(
                                    self.pos.x + x as f64,
                                    (y + 1.0) as f64,
                                    self.pos.z + z as f64,
                                ),
                                Vec3D::new(self.motion.x, vy, self.motion.z),
                            );
                            i += 1;
                        }
                        let mut i = 0;
                        while (i as f32) < 1.0 + self.width * 20.0 {
                            let x = (self.rand.next_float() * 2.0 - 1.0) * self.width;
                            let z = (self.rand.next_float() * 2.0 - 1.0) * self.width;
                            w.spawn_particle(
                                "splash",
                                Vec3D::new(
                                    self.pos.x + x as f64,
                                    (y + 1.0) as f64,
                                    self.pos.z + z as f64,
                                ),
                                self.motion,
                            );
                            i += 1;
                        }
                    }
                }
            }
            if s.catchable > 0 {
                self.motion.y -= (self.rand.next_float()
                    * self.rand.next_float()
                    * self.rand.next_float()) as f64
                    * 0.2;
            }
            self.motion.y += 0.04f32 as f64 * (water * 2.0 - 1.0);
            if water > 0.0 {
                friction = (friction as f64 * 0.9) as f32;
                self.motion.y *= 0.8;
            }
            self.motion.x *= friction as f64;
            self.motion.y *= friction as f64;
            self.motion.z *= friction as f64;
            self.set_position(self.pos.x, self.pos.y, self.pos.z);
        }
        self.kind = EntityKind::Fish(s);
    }
    pub fn catch_fish(&mut self, w: &mut World, angler: &mut Entity) -> i32 {
        let EntityKind::Fish(s) = &self.kind else {
            return 0;
        };
        let s = s.clone();
        let mut damage = 0;
        let pull = Vec3D::new(
            angler.pos.x - self.pos.x,
            angler.pos.y - self.pos.y,
            angler.pos.z - self.pos.z,
        );
        let distance = pull.length();
        let motion = Vec3D::new(
            pull.x * 0.1,
            pull.y * 0.1 + (distance.sqrt() as f32 as f64) * 0.08,
            pull.z * 0.1,
        );
        if let Some(id) = s.hooked {
            if let Some(target) = w.entities.get_mut(&id) {
                target.motion.x += motion.x;
                target.motion.y += motion.y;
                target.motion.z += motion.z;
            }
            damage = 3;
        } else if s.catchable > 0 {
            let mut item = Self::item(
                w,
                self.pos.x,
                self.pos.y,
                self.pos.z,
                crate::world::tile::ItemStack::new(349, 1, 0),
            );
            item.motion = motion;
            w.entity_joined_world(item);
            angler.add_stat(crate::entity::player::stat::FISH_CAUGHT, 1);
            damage = 1;
        }
        if s.arrow.in_ground {
            damage = 2;
        }
        self.dead = true;
        angler.player_state_mut().unwrap().fish_entity = None;
        damage
    }
    pub fn use_fishing_rod(&mut self, w: &mut World, stack: &mut ItemStack) {
        if let Some(id) = self.player_state().unwrap().fish_entity {
            if let Some(mut fish) = w.entities.remove(&id) {
                let damage = fish.catch_fish(w, self);
                let count = stack.count;
                stack.damage_item(damage);
                if stack.count < count {
                    self.add_stat(crate::entity::player::stat::broken(346), 1);
                }
                if w.loaded_entity_list.contains(&id) {
                    w.entities.insert(id, fish);
                }
            }
        } else {
            let pitch = 0.4 / (w.item_random.next_float() * 0.4 + 0.8);
            w.play_sound_at_entity(self, "random.bow", 0.5, pitch);
            let fish = Self::fish_from_owner(w, self);
            w.entity_joined_world(fish);
        }
        self.swing_item();
    }
}
