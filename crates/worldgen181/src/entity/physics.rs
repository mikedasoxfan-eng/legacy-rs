use super::{Entity, EntityKind, Vec3D};
use crate::{
    block::*,
    math::{self, floor_double as floor},
    World,
};
impl Entity {
    pub fn handle_water_movement(&mut self, w: &mut World) -> bool {
        let b = if matches!(self.kind, EntityKind::Item { .. }) {
            self.bounding_box
        } else {
            self.bounding_box
                .expand(0.0, -0.4f32 as f64, 0.0)
                .contract(0.001, 0.001, 0.001)
        };
        w.handle_material_acceleration(b, MAT_WATER, &mut self.motion)
    }
    pub fn handle_lava_movement(&mut self, w: &mut World) -> bool {
        w.is_material_in_bb(
            self.bounding_box
                .expand(-0.1f32 as f64, -0.4f32 as f64, -0.1f32 as f64),
            MAT_LAVA,
        )
    }
    pub fn is_wet(&self, w: &mut World) -> bool {
        self.in_water
            || w.can_block_be_rained_on(floor(self.pos.x), floor(self.pos.y), floor(self.pos.z))
    }
    pub fn on_entity_update(&mut self, w: &mut World) {
        if self.riding.and_then(|id|w.entities.get(&id)).is_some_and(|p|p.dead) {self.riding=None;}
        self.ticks_existed = self.ticks_existed.wrapping_add(1);
        self.prev_distance_walked = self.distance_walked;
        self.prev_pos = self.pos;
        self.prev_yaw = self.yaw;
        self.prev_pitch = self.pitch;
        if self.handle_water_movement(w) {
            if !self.in_water && !self.first_update {
                let volume = ((self.motion.x * self.motion.x * 0.2f32 as f64
                    + self.motion.y * self.motion.y
                    + self.motion.z * self.motion.z * 0.2f32 as f64)
                    .sqrt() as f32
                    * 0.2)
                    .min(1.0);
                let pitch = 1.0 + (self.rand.next_float() - self.rand.next_float()) * 0.4;
                w.play_sound_at_entity(self, "random.splash", volume, pitch);
                let y = floor(self.bounding_box.min_y) as f32;
                let mut i = 0;
                while (i as f32) < 1.0 + self.width * 20.0 {
                    let x = (self.rand.next_float() * 2.0 - 1.0) * self.width;
                    let z = (self.rand.next_float() * 2.0 - 1.0) * self.width;
                    let vy = self.motion.y - (self.rand.next_float() * 0.2) as f64;
                    w.spawn_particle(
                        "bubble",
                        super::Vec3D::new(
                            self.pos.x + x as f64,
                            (y + 1.0) as f64,
                            self.pos.z + z as f64,
                        ),
                        super::Vec3D::new(self.motion.x, vy, self.motion.z),
                    );
                    i += 1;
                }
                let mut i = 0;
                while (i as f32) < 1.0 + self.width * 20.0 {
                    let x = (self.rand.next_float() * 2.0 - 1.0) * self.width;
                    let z = (self.rand.next_float() * 2.0 - 1.0) * self.width;
                    w.spawn_particle(
                        "splash",
                        super::Vec3D::new(
                            self.pos.x + x as f64,
                            (y + 1.0) as f64,
                            self.pos.z + z as f64,
                        ),
                        self.motion,
                    );
                    i += 1;
                }
            }
            self.fall_distance = 0.0;
            self.in_water = true;
            self.fire = 0;
        } else {
            self.in_water = false;
        }
        if self.fire > 0 {
            if self.immune_to_fire {
                self.fire -= 4;
                if self.fire < 0 {
                    self.fire = 0;
                }
            } else {
                if self.fire % 20 == 0 {
                    self.attack(w, None, 1);
                }
                self.fire -= 1;
            }
        }
        if self.handle_lava_movement(w) && !self.immune_to_fire {
            self.attack(w, None, 4);
            self.fire = 600;
        }
        if self.pos.y < -64.0 {
            if self.kind.is_living() {
                self.attack(w, None, 4);
            } else {
                self.set_entity_dead(w);
            }
        }
        self.first_update = false;
    }
    pub fn move_flying(&mut self, mut strafe: f32, mut forward: f32, friction: f32) {
        let mut f = ((strafe * strafe + forward * forward) as f64).sqrt() as f32;
        if f >= 0.01 {
            if f < 1.0 {
                f = 1.0;
            }
            f = friction / f;
            strafe *= f;
            forward *= f;
            let sin = math::sin(self.yaw * math::PI_F32 / 180.0);
            let cos = math::cos(self.yaw * math::PI_F32 / 180.0);
            self.motion.x += (strafe * cos - forward * sin) as f64;
            self.motion.z += (forward * cos + strafe * sin) as f64;
        }
    }
    fn sync_position_from_box(&mut self) {
        self.pos.x = (self.bounding_box.min_x + self.bounding_box.max_x) / 2.0;
        self.pos.y = self.bounding_box.min_y + self.y_offset as f64 - self.y_size as f64;
        self.pos.z = (self.bounding_box.min_z + self.bounding_box.max_z) / 2.0;
    }
    pub fn move_entity(&mut self, w: &mut World, mut dx: f64, mut dy: f64, mut dz: f64) {
        if matches!(self.kind,super::EntityKind::Painting(_)) {if dx*dx+dy*dy+dz*dz>0.0{self.break_painting(w,false);}return;}
        if self.no_clip {
            self.bounding_box.offset(dx, dy, dz);
            self.sync_position_from_box();
            return;
        }
        self.y_size *= 0.4;
        let old_x = self.pos.x;
        let old_z = self.pos.z;
        if self.in_web {
            self.in_web = false;
            dx *= 0.25;
            dy *= 0.05f32 as f64;
            dz *= 0.25;
            self.motion = Vec3D::default();
        }
        let mut original_x = dx;
        let original_y = dy;
        let mut original_z = dz;
        let original_box = self.bounding_box;
        let sneak = self.on_ground && self.sneaking;
        if sneak {
            while dx != 0.0
                && w.get_colliding_bounding_boxes(
                    Some(self.entity_id),
                    self.bounding_box.translated(dx, -1.0, 0.0),
                )
                .is_empty()
            {
                dx = if dx < 0.05 && dx >= -0.05 {
                    0.0
                } else if dx > 0.0 {
                    dx - 0.05
                } else {
                    dx + 0.05
                };
                original_x = dx;
            }
            while dz != 0.0
                && w.get_colliding_bounding_boxes(
                    Some(self.entity_id),
                    self.bounding_box.translated(0.0, -1.0, dz),
                )
                .is_empty()
            {
                dz = if dz < 0.05 && dz >= -0.05 {
                    0.0
                } else if dz > 0.0 {
                    dz - 0.05
                } else {
                    dz + 0.05
                };
                original_z = dz;
            }
        }
        let mut boxes = w.get_colliding_bounding_boxes(
            Some(self.entity_id),
            self.bounding_box.add_coord(dx, dy, dz),
        );
        for b in &boxes {
            dy = b.calculate_y_offset(self.bounding_box, dy);
        }
        self.bounding_box.offset(0.0, dy, 0.0);
        if !self.slide_on_collision && original_y != dy {
            dx = 0.0;
            dy = 0.0;
            dz = 0.0;
        }
        let grounded = self.on_ground || original_y != dy && original_y < 0.0;
        for b in &boxes {
            dx = b.calculate_x_offset(self.bounding_box, dx);
        }
        self.bounding_box.offset(dx, 0.0, 0.0);
        if !self.slide_on_collision && original_x != dx {
            dx = 0.0;
            dy = 0.0;
            dz = 0.0;
        }
        for b in &boxes {
            dz = b.calculate_z_offset(self.bounding_box, dz);
        }
        self.bounding_box.offset(0.0, 0.0, dz);
        if !self.slide_on_collision && original_z != dz {
            dx = 0.0;
            dy = 0.0;
            dz = 0.0;
        }
        if self.step_height > 0.0
            && grounded
            && (sneak || self.y_size < 0.05)
            && (original_x != dx || original_z != dz)
        {
            let (saved_x, saved_y, saved_z) = (dx, dy, dz);
            let saved_box = self.bounding_box;
            dx = original_x;
            dy = self.step_height as f64;
            dz = original_z;
            self.bounding_box = original_box;
            boxes = w.get_colliding_bounding_boxes(
                Some(self.entity_id),
                self.bounding_box.add_coord(original_x, dy, original_z),
            );
            for b in &boxes {
                dy = b.calculate_y_offset(self.bounding_box, dy);
            }
            self.bounding_box.offset(0.0, dy, 0.0);
            if !self.slide_on_collision && original_y != dy {
                dx = 0.0;
                dy = 0.0;
                dz = 0.0;
            }
            for b in &boxes {
                dx = b.calculate_x_offset(self.bounding_box, dx);
            }
            self.bounding_box.offset(dx, 0.0, 0.0);
            if !self.slide_on_collision && original_x != dx {
                dx = 0.0;
                dy = 0.0;
                dz = 0.0;
            }
            for b in &boxes {
                dz = b.calculate_z_offset(self.bounding_box, dz);
            }
            self.bounding_box.offset(0.0, 0.0, dz);
            if !self.slide_on_collision && original_z != dz {
                dx = 0.0;
                dy = 0.0;
                dz = 0.0;
            }
            if !self.slide_on_collision && original_y != dy {
                dx = 0.0;
                dy = 0.0;
                dz = 0.0;
            } else {
                dy = (-self.step_height) as f64;
                for b in &boxes {
                    dy = b.calculate_y_offset(self.bounding_box, dy);
                }
                self.bounding_box.offset(0.0, dy, 0.0);
            }
            if saved_x * saved_x + saved_z * saved_z >= dx * dx + dz * dz {
                dx = saved_x;
                dy = saved_y;
                dz = saved_z;
                self.bounding_box = saved_box;
            } else {
                let fraction = self.bounding_box.min_y - (self.bounding_box.min_y as i32) as f64;
                if fraction > 0.0 {
                    self.y_size = (self.y_size as f64 + fraction + 0.01) as f32;
                }
            }
        }
        self.sync_position_from_box();
        self.collided_horizontally = original_x != dx || original_z != dz;
        self.collided_vertically = original_y != dy;
        self.on_ground = original_y != dy && original_y < 0.0;
        self.collided = self.collided_horizontally || self.collided_vertically;
        if self.on_ground {
            if self.fall_distance > 0.0 {
                let distance = self.fall_distance;
                self.fall(w, distance);
                self.fall_distance = 0.0;
            }
        } else if dy < 0.0 {
            self.fall_distance = (self.fall_distance as f64 - dy) as f32;
        }
        if original_x != dx {
            self.motion.x = 0.0;
        }
        if original_y != dy {
            self.motion.y = 0.0;
        }
        if original_z != dz {
            self.motion.z = 0.0;
        }
        if self.can_trigger_walking() && !sneak && self.riding.is_none() {
            let x = self.pos.x - old_x;
            let z = self.pos.z - old_z;
            self.distance_walked =
                (self.distance_walked as f64 + ((x * x + z * z).sqrt() as f32) as f64 * 0.6) as f32;
            let (x, y, z) = (
                floor(self.pos.x),
                floor(self.pos.y - 0.2f32 as f64 - self.y_offset as f64),
                floor(self.pos.z),
            );
            let mut id = w.get_block_id(x, y, z);
            if w.get_block_id(x, y - 1, z) == FENCE {
                id = FENCE;
            }
            if self.distance_walked > self.next_step_distance as f32 && id > 0 {
                self.next_step_distance += 1;
                if w.get_block_id(x, y + 1, z) == SNOW {
                    self.play_step_sound(w, SNOW, 0.15, 1.0);
                } else if !mat(material_of(id)).is_liquid {
                    self.play_step_sound(w, id, 0.15, 1.0);
                }
                self.on_entity_walking(w, id, x, y, z);
            }
        }
        let b = self.bounding_box;
        let (x0, y0, z0, x1, y1, z1) = (
            floor(b.min_x + 0.001),
            floor(b.min_y + 0.001),
            floor(b.min_z + 0.001),
            floor(b.max_x - 0.001),
            floor(b.max_y - 0.001),
            floor(b.max_z - 0.001),
        );
        if w.check_chunks_exist(x0, y0, z0, x1, y1, z1) {
            for x in x0..=x1 {
                for y in y0..=y1 {
                    for z in z0..=z1 {
                        let id = w.get_block_id(x, y, z);
                        self.on_entity_collided_with_block(w, id, x, y, z);
                    }
                }
            }
        }
        let wet = self.is_wet(w);
        if w.is_bounding_box_burning(self.bounding_box.contract(0.001, 0.001, 0.001)) {
            if !self.immune_to_fire {
                self.attack(w, None, 1);
            }
            if !wet {
                self.fire += 1;
                if self.fire == 0 {
                    self.fire = 300;
                }
            }
        } else if self.fire <= 0 {
            self.fire = -self.fire_resistance;
        }
        if wet && self.fire > 0 {
            let pitch = 1.6 + (self.rand.next_float() - self.rand.next_float()) * 0.4;
            w.play_sound_at_entity(self, "random.fizz", 0.7, pitch);
            self.fire = -self.fire_resistance;
        }
    }
    pub fn can_trigger_walking(&self) -> bool {
        matches!(self.kind, EntityKind::Fish(_)) || (self.kind.is_living() && !matches!(self.kind, EntityKind::Spider | EntityKind::Wolf(..)))
    }
    fn on_entity_walking(&mut self, w: &mut World, id: u8, x: i32, y: i32, z: i32) {
        if id == TILLED_FIELD && w.rand.next_int(4) == 0 {
            w.set_block_with_notify(x, y, z, DIRT);
        }
        if matches!(id, ORE_REDSTONE | ORE_REDSTONE_GLOWING) {
            w.redstone_display(x, y, z);
            if id == ORE_REDSTONE {
                w.set_block_with_notify(x, y, z, ORE_REDSTONE_GLOWING);
            }
        }
    }
    fn on_entity_collided_with_block(&mut self, w: &mut World, id: u8, x: i32, y: i32, z: i32) {
        match id {
            28 => {
                if w.get_block_metadata(x,y,z)&8==0 {
                    w.entities.insert(self.entity_id,self.clone());
                    w.update_detector_rail(x,y,z);
                    w.entities.remove(&self.entity_id);
                }
            }
            PORTAL => self.set_in_portal(),
            CACTUS => {
                self.attack(w, None, 1);
            }
            WEB => {
                self.in_web = true;
            }
            SOUL_SAND => {
                self.motion.x *= 0.4;
                self.motion.z *= 0.4;
            }
            PRESSURE_PLATE_WOOD | PRESSURE_PLATE_STONE => {
                if w.get_block_metadata(x, y, z) != 1 {
                    w.entities.insert(self.entity_id, self.clone());
                    w.update_pressure_plate(id, x, y, z);
                    w.entities.remove(&self.entity_id);
                }
            }
            _ => {}
        }
    }
    pub fn push_out_of_blocks(&mut self, w: &mut World, x: f64, y: f64, z: f64) {
        let (bx, by, bz) = (floor(x), floor(y), floor(z));
        if !w.is_block_normal_cube(bx, by, bz) {
            return;
        }
        let empty = [
            !w.is_block_normal_cube(bx - 1, by, bz),
            !w.is_block_normal_cube(bx + 1, by, bz),
            !w.is_block_normal_cube(bx, by - 1, bz),
            !w.is_block_normal_cube(bx, by + 1, bz),
            !w.is_block_normal_cube(bx, by, bz - 1),
            !w.is_block_normal_cube(bx, by, bz + 1),
        ];
        let d = [
            x - bx as f64,
            1.0 - (x - bx as f64),
            y - by as f64,
            1.0 - (y - by as f64),
            z - bz as f64,
            1.0 - (z - bz as f64),
        ];
        let mut side = None;
        let mut nearest = 9999.0;
        for i in 0..6 {
            if empty[i] && d[i] < nearest {
                nearest = d[i];
                side = Some(i);
            }
        }
        let push = self.rand.next_float() * 0.2 + 0.1;
        match side {
            Some(0) => self.motion.x = (-push) as f64,
            Some(1) => self.motion.x = push as f64,
            Some(2) => self.motion.y = (-push) as f64,
            Some(3) => self.motion.y = push as f64,
            Some(4) => self.motion.z = (-push) as f64,
            Some(5) => self.motion.z = push as f64,
            _ => {}
        }
    }
}

impl Entity {
    /// `fall(distance)` with its overrides: `EntityPlayer` counts the distance, `Entity` passes
    /// the fall to the rider, `EntityLiving` takes damage, `EntityPig` gives its rider "When
    /// Pigs Fly"; chickens and ghasts ignore falls (rider included).
    pub fn fall(&mut self, w: &mut World, distance: f32) {
        if matches!(self.kind, EntityKind::Ghast(..) | EntityKind::Chicken { .. }) {
            return;
        }
        if self.player_state().is_some() && distance >= 2.0 {
            let n = crate::entity::player::java_round_d(distance as f64 * 100.0);
            self.add_stat(crate::entity::player::stat::FALL, n);
        }
        // Entity.fall: the rider falls with its mount.
        if let Some(id) = self.ridden_by {
            if let Some(mut rider) = w.entities.remove(&id) {
                rider.fall(w, distance);
                w.entities.insert(id, rider);
            }
        }
        if self.kind.is_living() {
            let damage = (distance - 3.0).ceil() as i32;
            if damage > 0 {
                self.attack(w, None, damage);
                let id = w.get_block_id(
                    floor(self.pos.x),
                    floor(self.pos.y - 0.2f32 as f64 - self.y_offset as f64),
                    floor(self.pos.z),
                );
                if id > 0 {
                    self.play_step_sound(w, id, 0.5, 12.0 / 16.0);
                }
            }
        }
        if matches!(self.kind, EntityKind::Pig { .. }) && distance > 5.0 {
            if let Some(rider) = self.ridden_by.and_then(|id| w.entities.get_mut(&id)) {
                rider.add_stat(crate::entity::player::stat::FLY_PIG, 1);
            }
        }
    }
}
