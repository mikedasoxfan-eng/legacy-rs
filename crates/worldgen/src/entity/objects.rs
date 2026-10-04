use super::{block_table, Entity, EntityKind};
use crate::{block::*, math::floor_double as floor, World};
impl Entity {
    pub(super) fn update_item(&mut self, w: &mut World) {
        self.on_entity_update(w);
        if let EntityKind::Item { pickup_delay, .. } = &mut self.kind {
            if *pickup_delay > 0 {
                *pickup_delay -= 1;
            }
        }
        self.prev_pos = self.pos;
        self.motion.y -= 0.04f32 as f64;
        if w.get_block_material(floor(self.pos.x), floor(self.pos.y), floor(self.pos.z)) == MAT_LAVA
        {
            self.motion.y = 0.2f32 as f64;
            self.motion.x = ((self.rand.next_float() - self.rand.next_float()) * 0.2) as f64;
            self.motion.z = ((self.rand.next_float() - self.rand.next_float()) * 0.2) as f64;
            self.rand.next_float();
        }
        self.push_out_of_blocks(
            w,
            self.pos.x,
            (self.bounding_box.min_y + self.bounding_box.max_y) / 2.0,
            self.pos.z,
        );
        self.move_entity(w, self.motion.x, self.motion.y, self.motion.z);
        let mut friction = 0.98f32;
        if self.on_ground {
            friction = 0.1f32 * 0.1f32 * 58.8f32;
            let id = w.get_block_id(
                floor(self.pos.x),
                floor(self.bounding_box.min_y) - 1,
                floor(self.pos.z),
            );
            if id > 0 {
                friction = block_table::SLIPPERINESS[id as usize] * 0.98;
            }
        }
        self.motion.x *= friction as f64;
        self.motion.y *= 0.98f32 as f64;
        self.motion.z *= friction as f64;
        if self.on_ground {
            self.motion.y *= -0.5;
        }
        if let EntityKind::Item { age, updates, .. } = &mut self.kind {
            *updates += 1;
            *age += 1;
            if *age >= 6000 {
                self.dead = true;
            }
        }
    }
    pub(super) fn update_falling_sand(&mut self, w: &mut World) {
        let (block, fall_time) =
            if let EntityKind::FallingSand { block, fall_time } = &mut self.kind {
                if *block == 0 {
                    self.dead = true;
                    return;
                }
                *fall_time += 1;
                (*block, *fall_time)
            } else {
                unreachable!()
            };
        self.prev_pos = self.pos;
        self.motion.y -= 0.04f32 as f64;
        self.move_entity(w, self.motion.x, self.motion.y, self.motion.z);
        self.motion.x *= 0.98f32 as f64;
        self.motion.y *= 0.98f32 as f64;
        self.motion.z *= 0.98f32 as f64;
        let (x, y, z) = (floor(self.pos.x), floor(self.pos.y), floor(self.pos.z));
        if w.get_block_id(x, y, z) == block {
            w.set_block_with_notify(x, y, z, 0);
        }
        if self.on_ground {
            self.motion.x *= 0.7f32 as f64;
            self.motion.z *= 0.7f32 as f64;
            self.motion.y *= -0.5;
            self.dead = true;
            let cur = w.get_block_id(x, y, z);
            let replace = cur == 0
                || matches!(
                    cur,
                    WATER_MOVING | WATER_STILL | LAVA_MOVING | LAVA_STILL | FIRE | SNOW
                );
            if !replace
                || crate::world::behavior::can_fall_below(w, x, y - 1, z)
                || !w.set_block_with_notify(x, y, z, block)
            {
                self.drop_item(w, block as i32, 1, 0, 0.0);
            }
        } else if fall_time > 100 {
            self.drop_item(w, block as i32, 1, 0, 0.0);
            self.dead = true;
        }
    }
}

impl super::Entity {
    pub fn tnt(w: &mut crate::World, x: f64, y: f64, z: f64) -> Self {
        let mut e = Self::base(w, super::EntityKind::Tnt { fuse: 80 });
        e.prevent_entity_spawning = true;
        e.width = 0.98;
        e.height = 0.98;
        e.y_offset = e.height / 2.0;
        e.set_position(x, y, z);
        let angle = (w.math_random.next_double() * crate::math::PI_F32 as f64 * 2.0) as f32;
        e.motion.x = (-crate::math::sin(angle * crate::math::PI_F32 / 180.0) * 0.02) as f64;
        e.motion.y = 0.2f32 as f64;
        e.motion.z = (-crate::math::cos(angle * crate::math::PI_F32 / 180.0) * 0.02) as f64;
        e.prev_pos = e.pos;
        e
    }
    pub(super) fn update_tnt(&mut self, w: &mut crate::World) {
        self.prev_pos = self.pos;
        self.motion.y -= 0.04f32 as f64;
        self.move_entity(w, self.motion.x, self.motion.y, self.motion.z);
        self.motion.x *= 0.98f32 as f64;
        self.motion.y *= 0.98f32 as f64;
        self.motion.z *= 0.98f32 as f64;
        if self.on_ground {
            self.motion.x *= 0.7f32 as f64;
            self.motion.z *= 0.7f32 as f64;
            self.motion.y *= -0.5;
        }
        if let super::EntityKind::Tnt { fuse } = &mut self.kind {
            let old = *fuse;
            *fuse -= 1;
            if old <= 0 {
                self.dead = true;
                w.create_explosion(None, self.pos, 4.0);
            }
        }
    }
    pub fn lightning(w: &mut crate::World, x: f64, y: f64, z: f64) -> Self {
        let mut e = Self::base(
            w,
            super::EntityKind::Lightning {
                state: 2,
                seed: 0,
                remaining: 0,
            },
        );
        e.set_location_and_angles(x, y, z, 0.0, 0.0);
        let seed = e.rand.next_long();
        let remaining = e.rand.next_int(3) + 1;
        e.kind = super::EntityKind::Lightning {
            state: 2,
            seed,
            remaining,
        };
        let (bx, by, bz) = (
            crate::math::floor_double(x),
            crate::math::floor_double(y),
            crate::math::floor_double(z),
        );
        if w.difficulty >= 2
            && w.check_chunks_exist(bx - 10, by - 10, bz - 10, bx + 10, by + 10, bz + 10)
        {
            e.lightning_fire(w, bx, by, bz);
            for _ in 0..4 {
                let x = bx + e.rand.next_int(3) - 1;
                let y = by + e.rand.next_int(3) - 1;
                let z = bz + e.rand.next_int(3) - 1;
                e.lightning_fire(w, x, y, z);
            }
        }
        e
    }
    fn lightning_fire(&self, w: &mut crate::World, x: i32, y: i32, z: i32) {
        if w.get_block_id(x, y, z) == 0 && crate::world::ticks_blocks::fire_can_place(w, x, y, z) {
            w.set_block_with_notify(x, y, z, crate::block::FIRE);
        }
    }
    pub(super) fn update_lightning(&mut self, w: &mut crate::World) {
        self.on_entity_update(w);
        if let super::EntityKind::Lightning {
            state,
            seed,
            remaining,
        } = &mut self.kind
        {
            if *state == 2 {
                self.rand.next_float();
                self.rand.next_float();
            }
            *state -= 1;
            let mut fire = false;
            if *state < 0 {
                if *remaining == 0 {
                    self.dead = true;
                } else if *state < -self.rand.next_int(10) {
                    *remaining -= 1;
                    *state = 1;
                    *seed = self.rand.next_long();
                    fire = true;
                }
            }
            let active = *state >= 0;
            if fire {
                let (x, y, z) = (
                    crate::math::floor_double(self.pos.x),
                    crate::math::floor_double(self.pos.y),
                    crate::math::floor_double(self.pos.z),
                );
                if w.check_chunks_exist(x - 10, y - 10, z - 10, x + 10, y + 10, z + 10) {
                    self.lightning_fire(w, x, y, z);
                }
            }
            if active {
                let b = super::AxisAlignedBB::new(
                    self.pos.x - 3.0,
                    self.pos.y - 3.0,
                    self.pos.z - 3.0,
                    self.pos.x + 3.0,
                    self.pos.y + 9.0,
                    self.pos.z + 3.0,
                );
                for id in w.entities_within_aabb(Some(self.entity_id), b) {
                    if let Some(mut e) = w.entities.remove(&id) {
                        e.struck_by_lightning(w);
                        w.entities.insert(id, e);
                    }
                }
                w.lightning_sky_flash = 2;
            }
        }
    }
    pub fn struck_by_lightning(&mut self, w: &mut crate::World) {
        if let super::EntityKind::Pig { .. } = self.kind {
            let mut pig = Self::living(w, "PigZombie").unwrap();
            pig.set_location_and_angles(self.pos.x, self.pos.y, self.pos.z, self.yaw, self.pitch);
            w.entity_joined_world(pig);
            self.dead = true;
            return;
        }
        if !self.immune_to_fire {
            self.attack(w, None, 5);
        }
        self.fire += 1;
        if self.fire == 0 {
            self.fire = 300;
        }
        if let super::EntityKind::Creeper { powered, .. } = &mut self.kind {
            *powered = true;
        }
    }
}
