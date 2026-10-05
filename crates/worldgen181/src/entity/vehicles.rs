//! EntityMinecart and EntityBoat, singleplayer paths from Beta 1.7.3.
use super::{AxisAlignedBB, Entity, EntityKind, Vec3D};
use crate::{World, math::floor_double as floor, world::tile::ItemStack};

pub const RAIL_ENDPOINTS: [[[i32; 3]; 2]; 10] = [
    [[0, 0, -1], [0, 0, 1]],
    [[-1, 0, 0], [1, 0, 0]],
    [[-1, -1, 0], [1, 0, 0]],
    [[-1, 0, 0], [1, -1, 0]],
    [[0, 0, -1], [0, -1, 1]],
    [[0, -1, -1], [0, 0, 1]],
    [[0, 0, 1], [1, 0, 0]],
    [[0, 0, 1], [-1, 0, 0]],
    [[0, 0, -1], [-1, 0, 0]],
    [[0, 0, -1], [1, 0, 0]],
];
pub fn is_rail(id: u8) -> bool {
    matches!(id, 27 | 28 | 66)
}
#[derive(Clone, Debug)]
pub struct MinecartState {
    pub cart_type: i32,
    pub damage: i32,
    pub time_since_hit: i32,
    pub rock_direction: i32,
    pub reversed: bool,
    pub fuel: i32,
    pub push_x: f64,
    pub push_z: f64,
    // Vanilla constructs 36 slots, exposes 27, and loads chest carts with 27.
    pub items: Vec<Option<super::player::ItemStack>>,
}
impl Default for MinecartState {
    fn default() -> Self {
        Self {
            cart_type: 0,
            damage: 0,
            time_since_hit: 0,
            rock_direction: 1,
            reversed: false,
            fuel: 0,
            push_x: 0.0,
            push_z: 0.0,
            items: vec![None; 36],
        }
    }
}
impl MinecartState {
    pub fn inventory_size(&self) -> usize {
        27
    }
    pub fn stack_in_slot(&self, slot: usize) -> Option<super::player::ItemStack> {
        self.items[slot]
    }
    pub fn decrement_stack(&mut self, slot: usize, count: i32) -> Option<super::player::ItemStack> {
        let s = self.items[slot].as_mut()?;
        if s.count <= count {
            self.items[slot].take()
        } else {
            let result = s.split(count);
            if s.count == 0 {
                self.items[slot] = None;
            }
            Some(result)
        }
    }
    pub fn set_stack(&mut self, slot: usize, mut item: Option<super::player::ItemStack>) {
        if let Some(s) = &mut item {
            if s.count > 64 {
                s.count = 64;
            }
        }
        self.items[slot] = item;
    }
}
#[derive(Clone, Debug)]
pub struct BoatState {
    pub damage: i32,
    pub time_since_hit: i32,
    pub rock_direction: i32,
}
impl Default for BoatState {
    fn default() -> Self {
        Self {
            damage: 0,
            time_since_hit: 0,
            rock_direction: 1,
        }
    }
}
impl Entity {
    pub fn minecart(w: &mut World, x: f64, y: f64, z: f64, cart_type: i32) -> Self {
        let mut e = Self::minecart_base(w);
        if let EntityKind::Minecart(s) = &mut e.kind {
            s.cart_type = cart_type;
        }
        e.set_position(x, y + e.y_offset as f64, z);
        e.prev_pos = Vec3D::new(x, y, z);
        e
    }
    pub(super) fn minecart_base(w: &mut World) -> Self {
        let mut e = Self::base(w, EntityKind::Minecart(Default::default()));
        e.prevent_entity_spawning = true;
        e.width = 0.98;
        e.height = 0.7;
        e.y_offset = e.height / 2.0;
        e
    }
    pub fn boat(w: &mut World, x: f64, y: f64, z: f64) -> Self {
        let mut e = Self::boat_base(w);
        e.set_position(x, y + e.y_offset as f64, z);
        e.prev_pos = Vec3D::new(x, y, z);
        e
    }
    pub(super) fn boat_base(w: &mut World) -> Self {
        let mut e = Self::base(w, EntityKind::Boat(Default::default()));
        e.prevent_entity_spawning = true;
        e.width = 1.5;
        e.height = 0.6;
        e.y_offset = e.height / 2.0;
        e
    }
    pub fn mounted_y_offset(&self) -> f64 {
        match self.kind {
            EntityKind::Boat(..) | EntityKind::Minecart(..) => {
                self.height as f64 * 0.0 - 0.3f32 as f64
            }
            EntityKind::Spider => self.height as f64 * 0.75 - 0.5,
            _ => self.height as f64 * 0.75,
        }
    }
    pub fn rider_y_offset(&self) -> f64 {
        if self.player_state().is_some() {
            (self.y_offset - 0.5) as f64
        } else {
            self.y_offset as f64
        }
    }
    pub fn update_rider_position(&self, rider: &mut Entity) {
        let (dx, dz) = if matches!(self.kind, EntityKind::Boat(..)) {
            let angle = self.yaw as f64 * std::f64::consts::PI / 180.0;
            (
                crate::jmath::java_cos(angle) * 0.4,
                crate::jmath::java_sin(angle) * 0.4,
            )
        } else {
            (0.0, 0.0)
        };
        rider.set_position(
            self.pos.x + dx,
            self.pos.y + self.mounted_y_offset() + rider.rider_y_offset(),
            self.pos.z + dz,
        );
    }
    pub fn can_be_pushed(&self) -> bool {
        matches!(self.kind, EntityKind::Minecart(..) | EntityKind::Boat(..))
            || self.kind.is_living() && !self.dead
    }
    pub fn can_interact_with_cart(&self, player: &Entity) -> bool {
        !self.dead && player.pos.square_distance(self.pos) <= 64.0
    }
    /// Entity.addVelocity for carts and what they push.
    pub fn add_motion(&mut self, x: f64, y: f64, z: f64) {
        self.motion.x += x;
        self.motion.y += y;
        self.motion.z += z;
    }
    fn dismount_vehicle_rider(&mut self, w: &mut World) {
        if let Some(id) = self.ridden_by {
            if let Some(mut rider) = w.entities.remove(&id) {
                w.entities.insert(self.entity_id, self.clone());
                rider.mount_entity(w, Some(self.entity_id));
                self.ridden_by = w.entities.remove(&self.entity_id).unwrap().ridden_by;
                w.entities.insert(id, rider);
            }
        }
    }
    pub fn set_entity_dead(&mut self, w: &mut World) {
        if matches!(self.kind, EntityKind::Minecart(..)) {
            self.drop_cart_contents(w);
        }
        self.dead = true;
    }
    fn drop_cart_contents(&mut self, w: &mut World) {
        for slot in 0..27 {
            let stack = match &self.kind {
                EntityKind::Minecart(s) => s.items[slot],
                _ => return,
            };
            if let Some(mut stack) = stack {
                let x = self.rand.next_float() * 0.8 + 0.1;
                let y = self.rand.next_float() * 0.8 + 0.1;
                let z = self.rand.next_float() * 0.8 + 0.1;
                while stack.count > 0 {
                    let n = (self.rand.next_int(21) + 10).min(stack.count as i32);
                    stack.count -= n;
                    let mut item = Self::item(
                        w,
                        self.pos.x + x as f64,
                        self.pos.y + y as f64,
                        self.pos.z + z as f64,
                        ItemStack::new(stack.id as i32, n, stack.damage as i32),
                    );
                    item.motion.x = (self.rand.next_gaussian() as f32 * 0.05) as f64;
                    item.motion.y = (self.rand.next_gaussian() as f32 * 0.05 + 0.2) as f64;
                    item.motion.z = (self.rand.next_gaussian() as f32 * 0.05) as f64;
                    w.entity_joined_world(item);
                }
                if let EntityKind::Minecart(s) = &mut self.kind {
                    s.items[slot] = Some(stack);
                }
            }
        }
    }
    pub(super) fn attack_vehicle(&mut self, w: &mut World, damage: i32) -> bool {
        if self.dead || w.multiplayer_world {
            return true;
        }
        let (broken, cart_type) = match &mut self.kind {
            EntityKind::Minecart(s) => {
                s.rock_direction = -s.rock_direction;
                s.time_since_hit = 10;
                s.damage = s.damage.wrapping_add(damage.wrapping_mul(10));
                (s.damage > 40, Some(s.cart_type))
            }
            EntityKind::Boat(s) => {
                s.rock_direction = -s.rock_direction;
                s.time_since_hit = 10;
                s.damage = s.damage.wrapping_add(damage.wrapping_mul(10));
                (s.damage > 40, None)
            }
            _ => return false,
        };
        self.been_attacked = true;
        if broken {
            self.dismount_vehicle_rider(w);
            if let Some(t) = cart_type {
                self.set_entity_dead(w);
                self.drop_item(w, 328, 1, 0, 0.0);
                if t == 1 {
                    self.drop_cart_contents(w);
                    self.drop_item(w, 54, 1, 0, 0.0);
                }
                if t == 2 {
                    self.drop_item(w, 61, 1, 0, 0.0);
                }
            } else {
                self.drop_boat_parts(w);
                self.set_entity_dead(w);
            }
        }
        true
    }
    fn drop_boat_parts(&mut self, w: &mut World) {
        for _ in 0..3 {
            self.drop_item(w, 5, 1, 0, 0.0);
        }
        for _ in 0..2 {
            self.drop_item(w, 280, 1, 0, 0.0);
        }
    }
    pub fn perform_vehicle_hurt_animation(&mut self) {
        let (damage, time, direction) = match &mut self.kind {
            EntityKind::Minecart(s) => {
                (&mut s.damage, &mut s.time_since_hit, &mut s.rock_direction)
            }
            EntityKind::Boat(s) => (&mut s.damage, &mut s.time_since_hit, &mut s.rock_direction),
            _ => return,
        };
        *direction = -*direction;
        *time = 10;
        *damage = damage.wrapping_add(damage.wrapping_mul(10));
    }
    pub(super) fn update_boat(&mut self, w: &mut World) {
        self.on_entity_update(w);
        if let EntityKind::Boat(s) = &mut self.kind {
            if s.time_since_hit > 0 {
                s.time_since_hit -= 1;
            }
            if s.damage > 0 {
                s.damage -= 1;
            }
        }
        self.prev_pos = self.pos;
        let b = self.bounding_box;
        let mut submerged = 0.0;
        for i in 0..5 {
            let min = b.min_y + (b.max_y - b.min_y) * i as f64 / 5.0 - 0.125;
            let max = b.min_y + (b.max_y - b.min_y) * (i + 1) as f64 / 5.0 - 0.125;
            if w.is_aabb_in_material(
                AxisAlignedBB::new(b.min_x, min, b.min_z, b.max_x, max, b.max_z),
                crate::block::MAT_WATER,
            ) {
                submerged += 1.0 / 5.0;
            }
        }
        if w.multiplayer_world {
            if self.network.increments > 0 {
                self.interpolate_network(w, false);
            } else {
                self.set_position(self.pos.x + self.motion.x, self.pos.y + self.motion.y, self.pos.z + self.motion.z);
                if self.on_ground { self.motion.x *= 0.5; self.motion.y *= 0.5; self.motion.z *= 0.5; }
                self.motion.x *= 0.99f32 as f64; self.motion.y *= 0.95f32 as f64; self.motion.z *= 0.99f32 as f64;
            }
            return;
        }
        if submerged < 1.0 {
            self.motion.y += 0.04f32 as f64 * (submerged * 2.0 - 1.0);
        } else {
            if self.motion.y < 0.0 {
                self.motion.y /= 2.0;
            }
            self.motion.y += 0.007f32 as f64;
        }
        if let Some(rider) = self.ridden_by.and_then(|id| w.entities.get(&id)) {
            self.motion.x += rider.motion.x * 0.2;
            self.motion.z += rider.motion.z * 0.2;
        }
        self.motion.x = self.motion.x.clamp(-0.4, 0.4);
        self.motion.z = self.motion.z.clamp(-0.4, 0.4);
        if self.on_ground {
            self.motion.x *= 0.5;
            self.motion.y *= 0.5;
            self.motion.z *= 0.5;
        }
        self.move_vehicle(w, self.motion.x, self.motion.y, self.motion.z);
        let speed = (self.motion.x * self.motion.x + self.motion.z * self.motion.z).sqrt();
        if speed > 0.15 {
            let a = self.yaw as f64 * std::f64::consts::PI / 180.0;
            let cos = crate::jmath::java_cos(a);
            let sin = crate::jmath::java_sin(a);
            let mut i = 0;
            while (i as f64) < 1.0 + speed * 60.0 {
                let spread = (self.rand.next_float() * 2.0 - 1.0) as f64;
                let side = (self.rand.next_int(2) * 2 - 1) as f64 * 0.7;
                let (x, z) = if self.rand.next_bool() {
                    (
                        self.pos.x - cos * spread * 0.8 + sin * side,
                        self.pos.z - sin * spread * 0.8 - cos * side,
                    )
                } else {
                    (
                        self.pos.x + cos + sin * spread * 0.7,
                        self.pos.z + sin - cos * spread * 0.7,
                    )
                };
                w.spawn_particle("splash", Vec3D::new(x, self.pos.y - 0.125, z), self.motion);
                i += 1;
            }
        }
        if self.collided_horizontally && speed > 0.15 {
            self.set_entity_dead(w);
            self.drop_boat_parts(w);
        } else {
            self.motion.x *= 0.99f32 as f64;
            self.motion.y *= 0.95f32 as f64;
            self.motion.z *= 0.99f32 as f64;
        }
        self.pitch = 0.0;
        let mut yaw = self.yaw as f64;
        let x = self.prev_pos.x - self.pos.x;
        let z = self.prev_pos.z - self.pos.z;
        if x * x + z * z > 0.001 {
            yaw = (crate::jmath::java_atan2(z, x) * 180.0 / std::f64::consts::PI) as f32 as f64;
        }
        let mut delta = yaw - self.yaw as f64;
        while delta >= 180.0 {
            delta -= 360.0;
        }
        while delta < -180.0 {
            delta += 360.0;
        }
        self.yaw = (self.yaw as f64 + delta.clamp(-20.0, 20.0)) as f32;
        self.yaw %= 360.0;
        self.pitch %= 360.0;
        self.collide_vehicles(w, false);
        for i in 0..4 {
            let x = floor(self.pos.x + ((i % 2) as f64 - 0.5) * 0.8);
            let z = floor(self.pos.z + ((i / 2) as f64 - 0.5) * 0.8);
            let y = floor(self.pos.y);
            if w.get_block_id(x, y, z) == 78 {
                w.set_block_with_notify(x, y, z, 0);
            }
        }
        self.clear_dead_rider(w);
    }
    fn move_vehicle(&mut self, w: &mut World, x: f64, y: f64, z: f64) {
        w.entities.insert(self.entity_id, self.clone());
        self.move_entity(w, x, y, z);
        w.entities.remove(&self.entity_id);
    }
    fn clear_dead_rider(&mut self, w: &World) {
        if self
            .ridden_by
            .and_then(|id| w.entities.get(&id))
            .is_some_and(|e| e.dead)
        {
            self.ridden_by = None;
        }
    }
    fn collide_vehicles(&mut self, w: &mut World, carts: bool) {
        for id in w.entities_within_aabb(
            Some(self.entity_id),
            self.bounding_box.expand(0.2f32 as f64, 0.0, 0.2f32 as f64),
        ) {
            if self.ridden_by == Some(id) {
                continue;
            }
            if let Some(mut e) = w.entities.remove(&id) {
                if e.can_be_pushed()
                    && if carts {
                        matches!(e.kind, EntityKind::Minecart(..))
                    } else {
                        matches!(e.kind, EntityKind::Boat(..))
                    }
                {
                    e.collide_entity(w, self);
                }
                w.entities.insert(id, e);
            }
        }
    }
    pub(super) fn update_minecart(&mut self, w: &mut World) {
        if let EntityKind::Minecart(s) = &mut self.kind {
            if s.time_since_hit > 0 {
                s.time_since_hit -= 1;
            }
            if s.damage > 0 {
                s.damage -= 1;
            }
        }
        if w.multiplayer_world && self.network.increments > 0 {
            self.interpolate_network(w, false);
            return;
        }
        // EntityMinecart deliberately does not call Entity.onUpdate.
        self.prev_pos = self.pos;
        self.motion.y -= 0.04f32 as f64;
        let x = floor(self.pos.x);
        let mut y = floor(self.pos.y);
        let z = floor(self.pos.z);
        if is_rail(w.get_block_id(x, y - 1, z)) {
            y -= 1;
        }
        let id = w.get_block_id(x, y, z);
        let mut burning = false;
        if is_rail(id) {
            let old = Self::rail_position(w, self.pos.x, self.pos.y, self.pos.z);
            let mut meta = w.get_block_metadata(x, y, z);
            self.pos.y = y as f64;
            let boost = id == 27 && meta & 8 != 0;
            let brake = id == 27 && !boost;
            if id != 66 {
                meta &= 7;
            }
            if (2..=5).contains(&meta) {
                self.pos.y = (y + 1) as f64;
            }
            match meta {
                2 => self.motion.x -= 1.0 / 128.0,
                3 => self.motion.x += 1.0 / 128.0,
                4 => self.motion.z += 1.0 / 128.0,
                5 => self.motion.z -= 1.0 / 128.0,
                _ => {}
            }
            let ends = RAIL_ENDPOINTS[meta as usize];
            let mut dx = (ends[1][0] - ends[0][0]) as f64;
            let mut dz = (ends[1][2] - ends[0][2]) as f64;
            let length = (dx * dx + dz * dz).sqrt();
            if self.motion.x * dx + self.motion.z * dz < 0.0 {
                dx = -dx;
                dz = -dz;
            }
            let speed = (self.motion.x * self.motion.x + self.motion.z * self.motion.z).sqrt();
            self.motion.x = speed * dx / length;
            self.motion.z = speed * dz / length;
            if brake {
                let speed = (self.motion.x * self.motion.x + self.motion.z * self.motion.z).sqrt();
                let f = if speed < 0.03 { 0.0 } else { 0.5 };
                self.motion.x *= f;
                self.motion.y *= 0.0;
                self.motion.z *= f;
            }
            let ax = x as f64 + 0.5 + ends[0][0] as f64 * 0.5;
            let az = z as f64 + 0.5 + ends[0][2] as f64 * 0.5;
            let bx = x as f64 + 0.5 + ends[1][0] as f64 * 0.5;
            let bz = z as f64 + 0.5 + ends[1][2] as f64 * 0.5;
            dx = bx - ax;
            dz = bz - az;
            let t = if dx == 0.0 {
                self.pos.x = x as f64 + 0.5;
                self.pos.z - z as f64
            } else if dz == 0.0 {
                self.pos.z = z as f64 + 0.5;
                self.pos.x - x as f64
            } else {
                ((self.pos.x - ax) * dx + (self.pos.z - az) * dz) * 2.0
            };
            self.pos.x = ax + dx * t;
            self.pos.z = az + dz * t;
            self.set_position(self.pos.x, self.pos.y + self.y_offset as f64, self.pos.z);
            let rider = if self.ridden_by.is_some() { 0.75 } else { 1.0 };
            self.move_vehicle(
                w,
                (self.motion.x * rider).clamp(-0.4, 0.4),
                0.0,
                (self.motion.z * rider).clamp(-0.4, 0.4),
            );
            for end in ends {
                if end[1] != 0 && floor(self.pos.x) - x == end[0] && floor(self.pos.z) - z == end[2]
                {
                    self.set_position(self.pos.x, self.pos.y + end[1] as f64, self.pos.z);
                    break;
                }
            }
            if self.ridden_by.is_some() {
                self.motion.x *= 0.997f32 as f64;
                self.motion.y *= 0.0;
                self.motion.z *= 0.997f32 as f64;
            } else {
                if let EntityKind::Minecart(s) = &mut self.kind {
                    if s.cart_type == 2 {
                        let len = (s.push_x * s.push_x + s.push_z * s.push_z).sqrt() as f32 as f64;
                        if len > 0.01 {
                            burning = true;
                            s.push_x /= len;
                            s.push_z /= len;
                            self.motion.x *= 0.8f32 as f64;
                            self.motion.y *= 0.0;
                            self.motion.z *= 0.8f32 as f64;
                            self.motion.x += s.push_x * 0.04;
                            self.motion.z += s.push_z * 0.04;
                        } else {
                            self.motion.x *= 0.9f32 as f64;
                            self.motion.y *= 0.0;
                            self.motion.z *= 0.9f32 as f64;
                        }
                    }
                }
                self.motion.x *= 0.96f32 as f64;
                self.motion.y *= 0.0;
                self.motion.z *= 0.96f32 as f64;
            }
            if let (Some(new), Some(old)) = (
                Self::rail_position(w, self.pos.x, self.pos.y, self.pos.z),
                old,
            ) {
                let slope = (old.y - new.y) * 0.05;
                let speed = (self.motion.x * self.motion.x + self.motion.z * self.motion.z).sqrt();
                if speed > 0.0 {
                    self.motion.x = self.motion.x / speed * (speed + slope);
                    self.motion.z = self.motion.z / speed * (speed + slope);
                }
                self.set_position(self.pos.x, new.y, self.pos.z);
            }
            let nx = floor(self.pos.x);
            let nz = floor(self.pos.z);
            if nx != x || nz != z {
                let speed = (self.motion.x * self.motion.x + self.motion.z * self.motion.z).sqrt();
                self.motion.x = speed * (nx - x) as f64;
                self.motion.z = speed * (nz - z) as f64;
            }
            if let EntityKind::Minecart(s) = &mut self.kind {
                if s.cart_type == 2 {
                    let len = (s.push_x * s.push_x + s.push_z * s.push_z).sqrt() as f32 as f64;
                    if len > 0.01
                        && self.motion.x * self.motion.x + self.motion.z * self.motion.z > 0.001
                    {
                        s.push_x /= len;
                        s.push_z /= len;
                        if s.push_x * self.motion.x + s.push_z * self.motion.z < 0.0 {
                            s.push_x = 0.0;
                            s.push_z = 0.0;
                        } else {
                            s.push_x = self.motion.x;
                            s.push_z = self.motion.z;
                        }
                    }
                }
            }
            if boost {
                let speed = (self.motion.x * self.motion.x + self.motion.z * self.motion.z).sqrt();
                if speed > 0.01 {
                    self.motion.x += self.motion.x / speed * 0.06;
                    self.motion.z += self.motion.z / speed * 0.06;
                } else if meta == 1 {
                    if w.is_block_normal_cube(x - 1, y, z) {
                        self.motion.x = 0.02;
                    } else if w.is_block_normal_cube(x + 1, y, z) {
                        self.motion.x = -0.02;
                    }
                } else if meta == 0 {
                    if w.is_block_normal_cube(x, y, z - 1) {
                        self.motion.z = 0.02;
                    } else if w.is_block_normal_cube(x, y, z + 1) {
                        self.motion.z = -0.02;
                    }
                }
            }
        } else {
            self.motion.x = self.motion.x.clamp(-0.4, 0.4);
            self.motion.z = self.motion.z.clamp(-0.4, 0.4);
            if self.on_ground {
                self.motion.x *= 0.5;
                self.motion.y *= 0.5;
                self.motion.z *= 0.5;
            }
            self.move_vehicle(w, self.motion.x, self.motion.y, self.motion.z);
            if !self.on_ground {
                self.motion.x *= 0.95f32 as f64;
                self.motion.y *= 0.95f32 as f64;
                self.motion.z *= 0.95f32 as f64;
            }
        }
        self.pitch = 0.0;
        let x = self.prev_pos.x - self.pos.x;
        let z = self.prev_pos.z - self.pos.z;
        if x * x + z * z > 0.001 {
            self.yaw = (crate::jmath::java_atan2(z, x) * 180.0 / std::f64::consts::PI) as f32;
            if matches!(&self.kind,EntityKind::Minecart(s) if s.reversed) {
                self.yaw += 180.0;
            }
        }
        let mut delta = (self.yaw - self.prev_yaw) as f64;
        while delta >= 180.0 {
            delta -= 360.0;
        }
        while delta < -180.0 {
            delta += 360.0;
        }
        if delta < -170.0 || delta >= 170.0 {
            self.yaw += 180.0;
            if let EntityKind::Minecart(s) = &mut self.kind {
                s.reversed = !s.reversed;
            }
        }
        self.yaw %= 360.0;
        self.pitch %= 360.0;
        self.collide_vehicles(w, true);
        self.clear_dead_rider(w);
        if burning && self.rand.next_int(4) == 0 {
            if let EntityKind::Minecart(s) = &mut self.kind {
                s.fuel -= 1;
                if s.fuel < 0 {
                    s.push_x = 0.0;
                    s.push_z = 0.0;
                }
            }
            w.spawn_particle(
                "largesmoke",
                Vec3D::new(self.pos.x, self.pos.y + 0.8, self.pos.z),
                Vec3D::default(),
            );
        }
    }
    /// EntityMinecart.func_514_g (also used by RenderMinecart).
    pub fn rail_position(w: &mut World, px: f64, py: f64, pz: f64) -> Option<Vec3D> {
        Self::rail_position_with(
            &mut |x, y, z| (w.get_block_id(x, y, z), w.get_block_metadata(x, y, z)),
            px,
            py,
            pz,
        )
    }
    pub fn rail_position_with(
        block: &mut impl FnMut(i32, i32, i32) -> (u8, i32),
        mut px: f64,
        py: f64,
        mut pz: f64,
    ) -> Option<Vec3D> {
        let x = floor(px);
        let mut y = floor(py);
        let z = floor(pz);
        if is_rail(block(x, y - 1, z).0) {
            y -= 1;
        }
        let id = block(x, y, z).0;
        if !is_rail(id) {
            return None;
        }
        let mut m = block(x, y, z).1;
        if id != 66 {
            m &= 7;
        }
        let ends = RAIL_ENDPOINTS[m as usize];
        let ax = x as f64 + 0.5 + ends[0][0] as f64 * 0.5;
        let ay = y as f64 + 0.5 + ends[0][1] as f64 * 0.5;
        let az = z as f64 + 0.5 + ends[0][2] as f64 * 0.5;
        let bx = x as f64 + 0.5 + ends[1][0] as f64 * 0.5;
        let by = y as f64 + 0.5 + ends[1][1] as f64 * 0.5;
        let bz = z as f64 + 0.5 + ends[1][2] as f64 * 0.5;
        let dx = bx - ax;
        let dy = (by - ay) * 2.0;
        let dz = bz - az;
        let t = if dx == 0.0 {
            pz - z as f64
        } else if dz == 0.0 {
            px - x as f64
        } else {
            ((px - ax) * dx + (pz - az) * dz) * 2.0
        };
        px = ax + dx * t;
        let mut py = ay + dy * t;
        pz = az + dz * t;
        if dy < 0.0 {
            py += 1.0;
        }
        if dy > 0.0 {
            py += 0.5;
        }
        Some(Vec3D::new(px, py, pz))
    }
    /// EntityMinecart.func_515_a.
    pub fn rail_offset(w: &mut World, px: f64, py: f64, pz: f64, offset: f64) -> Option<Vec3D> {
        Self::rail_offset_with(
            &mut |x, y, z| (w.get_block_id(x, y, z), w.get_block_metadata(x, y, z)),
            px,
            py,
            pz,
            offset,
        )
    }
    pub fn rail_offset_with(
        block: &mut impl FnMut(i32, i32, i32) -> (u8, i32),
        mut px: f64,
        py: f64,
        mut pz: f64,
        offset: f64,
    ) -> Option<Vec3D> {
        let x = floor(px);
        let mut y = floor(py);
        let z = floor(pz);
        if is_rail(block(x, y - 1, z).0) {
            y -= 1;
        }
        let id = block(x, y, z).0;
        if !is_rail(id) {
            return None;
        }
        let mut m = block(x, y, z).1;
        if id != 66 {
            m &= 7;
        }
        let mut py = if (2..=5).contains(&m) {
            (y + 1) as f64
        } else {
            y as f64
        };
        let ends = RAIL_ENDPOINTS[m as usize];
        let mut dx = (ends[1][0] - ends[0][0]) as f64;
        let mut dz = (ends[1][2] - ends[0][2]) as f64;
        let len = (dx * dx + dz * dz).sqrt();
        dx /= len;
        dz /= len;
        px += dx * offset;
        pz += dz * offset;
        for end in ends {
            if end[1] != 0 && floor(px) - x == end[0] && floor(pz) - z == end[2] {
                py += end[1] as f64;
                break;
            }
        }
        Self::rail_position_with(block, px, py, pz)
    }
    /// Dispatch virtual applyEntityCollision; ordinary entities keep their existing path.
    pub fn collide_entity(&mut self, w: &mut World, other: &mut Entity) {
        let t = match &self.kind {
            EntityKind::Minecart(s) => s.cart_type,
            _ => {
                self.apply_entity_collision(other);
                return;
            }
        };
        if self.ridden_by == Some(other.entity_id) {
            return;
        }
        if other.kind.is_living()
            && other.player_state().is_none()
            && t == 0
            && self.motion.x * self.motion.x + self.motion.z * self.motion.z > 0.01
            && self.ridden_by.is_none()
            && other.riding.is_none()
        {
            w.entities.insert(self.entity_id, self.clone());
            other.mount_entity(w, Some(self.entity_id));
            self.ridden_by = w.entities.remove(&self.entity_id).unwrap().ridden_by;
        }
        let mut x = other.pos.x - self.pos.x;
        let mut z = other.pos.z - self.pos.z;
        let len = x * x + z * z;
        if len < 1.0e-4f32 as f64 {
            return;
        }
        let len = len.sqrt() as f32 as f64;
        x /= len;
        z /= len;
        let inv = (1.0 / len).min(1.0);
        x *= inv;
        z *= inv;
        x *= 0.1f32 as f64;
        z *= 0.1f32 as f64;
        x *= (1.0 - self.collision_reduction) as f64;
        z *= (1.0 - self.collision_reduction) as f64;
        x *= 0.5;
        z *= 0.5;
        if let EntityKind::Minecart(s) = &other.kind {
            // Vanilla bug: prevPosX, not motionX, in the alignment test.
            let align = (other.pos.x - self.pos.x) * other.motion.z
                + (other.pos.z - self.pos.z) * other.prev_pos.x;
            if align * align > 5.0 {
                return;
            }
            let vx = other.motion.x + self.motion.x;
            let vz = other.motion.z + self.motion.z;
            if s.cart_type == 2 && t != 2 {
                self.motion.x *= 0.2f32 as f64;
                self.motion.z *= 0.2f32 as f64;
                self.add_motion(other.motion.x - x, 0.0, other.motion.z - z);
                other.motion.x *= 0.7f32 as f64;
                other.motion.z *= 0.7f32 as f64;
            } else if s.cart_type != 2 && t == 2 {
                other.motion.x *= 0.2f32 as f64;
                other.motion.z *= 0.2f32 as f64;
                other.add_motion(self.motion.x + x, 0.0, self.motion.z + z);
                self.motion.x *= 0.7f32 as f64;
                self.motion.z *= 0.7f32 as f64;
            } else {
                self.motion.x *= 0.2f32 as f64;
                self.motion.z *= 0.2f32 as f64;
                self.add_motion(vx / 2.0 - x, 0.0, vz / 2.0 - z);
                other.motion.x *= 0.2f32 as f64;
                other.motion.z *= 0.2f32 as f64;
                other.add_motion(vx / 2.0 + x, 0.0, vz / 2.0 + z);
            }
        } else {
            self.add_motion(-x, 0.0, -z);
            other.add_motion(x / 4.0, 0.0, z / 4.0);
        }
    }
}
