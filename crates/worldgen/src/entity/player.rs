//! Beta 1.7.3 singleplayer simulation and client-facing inputs.
pub mod activation;
pub mod api;
pub mod block_hooks;
pub mod controller;
pub mod dye;
pub mod interactions;
pub mod inventory;
pub mod items;
pub mod persistence;
pub mod picking;
pub mod placement;
pub mod registry;
pub mod sleep_world;
pub mod throwable;
pub mod verify;
use super::{Entity, EntityKind, Vec3D};
use crate::{
    World,
    math::{self, floor_double as floor},
};
pub use api::{PlayerHud, PlayerTarget, PlayerTickInput};
pub use inventory::{InventoryPlayer, ItemStack};

#[derive(Clone, Copy, Debug, Default)]
pub struct PlayerInput {
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub sneak: bool,
    pub yaw_delta: f32,
    pub pitch_delta: f32,
}
impl PlayerInput {
    pub fn movement(self) -> (f32, f32) {
        let mut strafe = (self.left as i32 - self.right as i32) as f32;
        let mut forward = (self.forward as i32 - self.back as i32) as f32;
        if self.sneak {
            strafe = (strafe as f64 * 0.3) as f32;
            forward = (forward as f64 * 0.3) as f32;
        }
        (strafe, forward)
    }
}
#[derive(Clone, Debug)]
pub enum PlayerEvent {
    EditSign { position: (i32, i32, i32), block: u8, metadata: i32, sign: crate::sign::Sign },
    Container(crate::container::ContainerEvent),
    Pickup {
        entity: i32,
        count: i32,
        source: Box<Entity>,
    },
    ChangeDimension,
    OpenContainer {
        block: u8,
        position: (i32, i32, i32),
    },
    SpawnVehicle {
        vehicle: VehicleKind,
        position: Vec3D,
    },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VehicleKind {
    Boat,
    Minecart,
    StorageMinecart,
    PoweredMinecart,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SleepStatus {
    Ok,
    OtherProblem,
    NotPossibleHere,
    NotPossibleNow,
    TooFarAway,
}
#[derive(Clone, Debug)]
pub struct PlayerState {
    pub left_click_counter: i32,
    pub last_click_tick: i32,
    pub controller: controller::PlayerControllerSP,
    pub inventory: InventoryPlayer,
    pub input: PlayerInput,
    pub username: String,
    pub camera_yaw: f32,
    pub prev_camera_yaw: f32,
    pub camera_pitch: f32,
    pub prev_camera_pitch: f32,
    pub is_swinging: bool,
    pub swing_ticks: i32,
    pub score: i32,
    pub dimension: i32,
    pub cloak: Vec3D,
    pub prev_cloak: Vec3D,
    pub sleeping: bool,
    pub sleep_timer: i32,
    pub bed: Option<(i32, i32, i32)>,
    pub spawn: Option<(i32, i32, i32)>,
    pub bed_offset: Vec3D,
    pub portal_cooldown: i32,
    pub in_portal: bool,
    pub time_in_portal: f32,
    pub prev_time_in_portal: f32,
    pub damage_remainder: i32,
    pub events: Vec<PlayerEvent>,
    pub player_index: Option<usize>,
}
impl Default for PlayerState {
    fn default() -> Self {
        Self {
            left_click_counter: 0,
            last_click_tick: 0,
            controller: Default::default(),
            inventory: Default::default(),
            input: Default::default(),
            username: "Player".into(),
            camera_yaw: 0.0,
            prev_camera_yaw: 0.0,
            camera_pitch: 0.0,
            prev_camera_pitch: 0.0,
            is_swinging: false,
            swing_ticks: 0,
            score: 0,
            dimension: 0,
            cloak: Vec3D::default(),
            prev_cloak: Vec3D::default(),
            sleeping: false,
            sleep_timer: 0,
            bed: None,
            spawn: None,
            bed_offset: Vec3D::default(),
            portal_cooldown: 20,
            in_portal: false,
            time_in_portal: 0.0,
            prev_time_in_portal: 0.0,
            damage_remainder: 0,
            events: Vec::new(),
            player_index: None,
        }
    }
}
impl Entity {
    pub fn player(w: &mut World, username: impl Into<String>) -> Self {
        let mut e = Self::living(w, "Player").unwrap();
        e.player_state_mut().unwrap().username = username.into();
        e.y_offset = 1.62;
        e.fire_resistance = 20;
        e.living.health = 20;
        let (x, y, z) = w.spawn;
        e.set_location_and_angles(x as f64 + 0.5, (y + 1) as f64, z as f64 + 0.5, 0.0, 0.0);
        e
    }
    pub fn player_state(&self) -> Option<&PlayerState> {
        if let EntityKind::Player(s) = &self.kind {
            Some(s)
        } else {
            None
        }
    }
    pub fn player_state_mut(&mut self) -> Option<&mut PlayerState> {
        if let EntityKind::Player(s) = &mut self.kind {
            Some(s)
        } else {
            None
        }
    }
    pub fn set_player_input(&mut self, input: PlayerInput) {
        self.set_angles(input.yaw_delta, input.pitch_delta);
        if let Some(s) = self.player_state_mut() {
            s.input = input;
            let sleeping = s.sleeping;
            self.sneaking = input.sneak && !sleeping;
        }
    }
    /// Entity.setAngles: mouse deltas are scaled in double precision before narrowing.
    pub fn set_angles(&mut self, yaw: f32, pitch: f32) {
        let (old_pitch, old_yaw) = (self.pitch, self.yaw);
        self.yaw = (self.yaw as f64 + yaw as f64 * 0.15) as f32;
        self.pitch = (self.pitch as f64 - pitch as f64 * 0.15) as f32;
        self.pitch = self.pitch.clamp(-90.0, 90.0);
        self.prev_pitch += self.pitch - old_pitch;
        self.prev_yaw += self.yaw - old_yaw;
    }
    pub fn swing_item(&mut self) {
        if let Some(s) = self.player_state_mut() {
            s.swing_ticks = -1;
            s.is_swinging = true;
        }
    }
    pub fn heal(&mut self, amount: i32) {
        if self.living.health > 0 {
            self.living.health = (self.living.health + amount).min(20);
            self.living.hearts_life = self.living.hearts_halves_life / 2;
        }
    }
    pub(super) fn player_action(&mut self) {
        let s = self.player_state_mut().unwrap();
        if s.is_swinging {
            s.swing_ticks += 1;
            if s.swing_ticks >= 8 {
                s.swing_ticks = 0;
                s.is_swinging = false;
            }
        } else {
            s.swing_ticks = 0;
        }
        let swing = s.swing_ticks as f32 / 8.0;
        let (strafe, forward) = s.input.movement();
        let jump = s.input.jump;
        self.living.swing_progress = swing;
        self.living.move_strafing = strafe;
        self.living.move_forward = forward;
        self.living.is_jumping = jump;
    }
    pub(super) fn player_before_living(&mut self, w: &mut World) {
        if self.player_state().unwrap().in_portal && self.riding.is_some() {
            self.mount_entity(w, None);
        }
        let s = self.player_state_mut().unwrap();
        s.prev_time_in_portal = s.time_in_portal;
        let mut sounds = Vec::new();
        if s.in_portal {
            if s.time_in_portal == 0.0 {
                sounds.push("portal.trigger");
            }
            s.time_in_portal += 0.0125;
            if s.time_in_portal >= 1.0 {
                s.time_in_portal = 1.0;
                s.portal_cooldown = 10;
                sounds.push("portal.travel");
                s.events.push(PlayerEvent::ChangeDimension);
            }
            s.in_portal = false;
        } else {
            if s.time_in_portal > 0.0 {
                s.time_in_portal -= 0.05;
            }
            if s.time_in_portal < 0.0 {
                s.time_in_portal = 0.0;
            }
        }
        if s.portal_cooldown > 0 {
            s.portal_cooldown -= 1;
        }
        let sneak = s.input.sneak;
        self.sneaking = sneak && !s.sleeping;
        for name in sounds {
            let pitch = self.rand.next_float() * 0.4 + 0.8;
            w.direct_sound(name, None, 1.0, pitch);
        }
        if sneak && self.y_size < 0.2 {
            self.y_size = 0.2;
        }
        for (x, z) in [(-1.0, 1.0), (-1.0, -1.0), (1.0, -1.0), (1.0, 1.0)] {
            self.player_push_out(
                w,
                self.pos.x + x * self.width as f64 * 0.35,
                self.bounding_box.min_y + 0.5,
                self.pos.z + z * self.width as f64 * 0.35,
            );
        }
        if w.difficulty == 0 && self.living.health < 20 && self.ticks_existed % 20 * 12 == 0 {
            self.heal(1);
        }
        let s = self.player_state_mut().unwrap();
        s.inventory.tick_animations();
        s.prev_camera_yaw = s.camera_yaw;
    }
    fn player_push_out(&mut self, w: &mut World, x: f64, y: f64, z: f64) {
        let (bx, by, bz) = (floor(x), floor(y), floor(z));
        if w.is_block_normal_cube(bx, by, bz) || w.is_block_normal_cube(bx, by + 1, bz) {
            let (dx, dz) = (x - bx as f64, z - bz as f64);
            let mut closest = 9999.0;
            let mut side = -1;
            for (sx, sz, d, s) in [
                (-1, 0, dx, 0),
                (1, 0, 1.0 - dx, 1),
                (0, -1, dz, 4),
                (0, 1, 1.0 - dz, 5),
            ] {
                if !w.is_block_normal_cube(bx + sx, by, bz + sz)
                    && !w.is_block_normal_cube(bx + sx, by + 1, bz + sz)
                    && d < closest
                {
                    closest = d;
                    side = s;
                }
            }
            match side {
                0 => self.motion.x = -0.1f32 as f64,
                1 => self.motion.x = 0.1f32 as f64,
                4 => self.motion.z = -0.1f32 as f64,
                5 => self.motion.z = 0.1f32 as f64,
                _ => {}
            }
        }
    }
    pub(super) fn player_after_living(&mut self, w: &mut World) {
        let mut yaw = (self.motion.x * self.motion.x + self.motion.z * self.motion.z).sqrt() as f32;
        let mut pitch = crate::jmath::java_atan(-self.motion.y * 0.2f32 as f64) as f32 * 15.0;
        if yaw > 0.1 {
            yaw = 0.1;
        }
        if !self.on_ground || self.living.health <= 0 {
            yaw = 0.0;
        }
        if self.on_ground || self.living.health <= 0 {
            pitch = 0.0;
        }
        let s = self.player_state_mut().unwrap();
        s.camera_yaw += (yaw - s.camera_yaw) * 0.4;
        s.camera_pitch += (pitch - s.camera_pitch) * 0.8;
        if self.living.health > 0 {
            for id in w.entities_within_aabb(
                Some(self.entity_id),
                self.bounding_box.expand(1.0, 0.0, 1.0),
            ) {
                if let Some(mut e) = w.entities.remove(&id) {
                    if !e.dead {
                        self.player_pickup(w, &mut e);
                    }
                    w.entities.insert(id, e);
                }
            }
        }
    }
    fn player_pickup(&mut self, w: &mut World, e: &mut Entity) {
        // EntitySlime.onCollideWithPlayer runs in the same player collision pass as pickups.
        if let EntityKind::Slime { size, .. } = e.kind {
            let delta = Vec3D::new(
                e.pos.x - self.pos.x,
                e.pos.y - self.pos.y,
                e.pos.z - self.pos.z,
            );
            let distance =
                (delta.x * delta.x + delta.y * delta.y + delta.z * delta.z).sqrt() as f32;
            if size > 1
                && w.ray_trace_blocks(
                    e.pos.add(0.0, e.eye_height() as f64, 0.0),
                    self.pos.add(0.0, self.eye_height() as f64, 0.0),
                )
                .is_none()
                && (distance as f64) < 0.6 * size as f64
            {
                w.entities.insert(e.entity_id, e.clone());
                let hit = self.attack(w, Some(e.entity_id), size);
                w.entities.remove(&e.entity_id);
                if hit {
                    let pitch = (e.rand.next_float() - e.rand.next_float()) * 0.2 + 1.0;
                    w.play_sound_at_entity(e, "mob.slimeattack", 1.0, pitch);
                }
            }
        }
        let mut pickup = None;
        if let EntityKind::Item {
            item, pickup_delay, ..
        } = &mut e.kind
        {
            if *pickup_delay == 0 {
                let original = item.count as i32;
                let mut stack = (*item).into();
                if self.player_state_mut().unwrap().inventory.add(&mut stack) {
                    pickup = Some(original);
                }
                *item = stack.into();
                if pickup.is_some() && item.count == 0 {
                    e.dead = true;
                }
            }
        } else if let EntityKind::Arrow(a) = &e.kind {
            if a.in_ground && a.belongs_to_player && a.shake <= 0 {
                if self
                    .player_state_mut()
                    .unwrap()
                    .inventory
                    .add(&mut ItemStack::new(262, 1, 0))
                {
                    pickup = Some(1);
                    e.dead = true;
                }
            }
        }
        if let Some(count) = pickup {
            let pitch = ((e.rand.next_float() - e.rand.next_float()) * 0.7 + 1.0) * 2.0;
            w.play_sound_at_entity(&e, "random.pop", 0.2, pitch);
            self.player_state_mut()
                .unwrap()
                .events
                .push(PlayerEvent::Pickup {
                    entity: e.entity_id,
                    count,
                    source: Box::new(e.clone()),
                });
        }
    }
    pub(super) fn update_player(&mut self, w: &mut World) {
        let s = self.player_state_mut().unwrap();
        if s.sleeping {
            s.sleep_timer = (s.sleep_timer + 1).min(100);
        } else if s.sleep_timer > 0 {
            s.sleep_timer += 1;
            if s.sleep_timer >= 110 {
                s.sleep_timer = 0;
            }
        }
        if let Some(bed) = self
            .player_state()
            .filter(|s| s.sleeping)
            .and_then(|s| s.bed)
        {
            if w.get_block_id(bed.0, bed.1, bed.2) != 26 {
                self.wake_up(w, true, false);
            } else if w.skylight_subtracted < 4 {
                self.wake_up(w, false, true);
            }
        }
        let previous_pitch = self.player_state().unwrap().camera_pitch;
        self.player_state_mut().unwrap().prev_camera_pitch = previous_pitch;
        self.update_living(w);
        w.validate_container_entity(self);
        let pos = self.pos;
        let s = self.player_state_mut().unwrap();
        s.prev_cloak = s.cloak;
        let delta = s.cloak.subtract(pos);
        if delta.x > 10.0 || delta.x < -10.0 {
            s.prev_cloak.x = pos.x;
            s.cloak.x = pos.x;
        }
        if delta.z > 10.0 || delta.z < -10.0 {
            s.prev_cloak.z = pos.z;
            s.cloak.z = pos.z;
        }
        if delta.y > 10.0 || delta.y < -10.0 {
            s.prev_cloak.y = pos.y;
            s.cloak.y = pos.y;
        }
        s.cloak.x += delta.x * 0.25;
        s.cloak.z += delta.z * 0.25;
        s.cloak.y += delta.y * 0.25;
    }
    pub(super) fn player_damage(
        &mut self,
        w: &mut World,
        attacker: Option<i32>,
        damage: &mut i32,
    ) -> bool {
        self.living.entity_age = 0;
        if self.living.health <= 0 {
            return false;
        }
        if self.player_state().unwrap().sleeping {
            self.wake_up(w, true, false);
        }
        if attacker
            .and_then(|id| w.entities.get(&id))
            .is_some_and(|e| e.kind.is_mob() || matches!(e.kind, EntityKind::Arrow(..)))
        {
            *damage = match w.difficulty {
                0 => 0,
                1 => *damage / 3 + 1,
                3 => *damage * 3 / 2,
                _ => *damage,
            };
        }
        if *damage != 0 {
            if let Some(mut source) = attacker.and_then(|id| w.entities.get(&id)).cloned() {
                if let EntityKind::Arrow(a) = &source.kind {
                    if let Some(owner) = a.owner.and_then(|id| w.entities.get(&id)).cloned() {
                        source = owner;
                    }
                }
                if source.kind.is_living() {
                    self.alert_wolves(w, &source, false);
                }
            }
        }
        *damage != 0
    }
    pub(super) fn damage_living(&mut self, mut amount: i32) {
        if let Some(s) = self.player_state_mut() {
            let damage = amount * (25 - s.inventory.armor_value()) + s.damage_remainder;
            s.inventory.damage_armor(amount);
            amount = damage / 25;
            s.damage_remainder = damage % 25;
        }
        self.living.health -= amount;
    }
    pub(super) fn player_death(&mut self, w: &mut World, attacker: Option<i32>) {
        self.width = 0.2;
        self.height = 0.2;
        self.set_position(self.pos.x, self.pos.y, self.pos.z);
        self.motion.y = 0.1f32 as f64;
        if self.player_state().unwrap().username == "Notch" {
            self.drop_player_item(w, ItemStack::new(260, 1, 0), true);
        }
        for slot in 0..40 {
            if let Some(stack) = self
                .player_state_mut()
                .unwrap()
                .inventory
                .decrement(slot, i32::MAX)
            {
                self.drop_player_item(w, stack, true);
            }
        }
        if attacker.is_some() {
            let angle = (self.living.attacked_at_yaw + self.yaw) * math::PI_F32 / 180.0;
            self.motion.x = (-math::cos(angle) * 0.1) as f64;
            self.motion.z = (-math::sin(angle) * 0.1) as f64;
        } else {
            self.motion.x = 0.0;
            self.motion.z = 0.0;
        }
        self.y_offset = 0.1;
    }
    pub fn drop_player_item(&mut self, w: &mut World, stack: ItemStack, random: bool) {
        let mut e = Entity::item(
            w,
            self.pos.x,
            self.pos.y - 0.3f32 as f64 + self.eye_height() as f64,
            self.pos.z,
            stack.into(),
        );
        if let EntityKind::Item { pickup_delay, .. } = &mut e.kind {
            *pickup_delay = 40;
        }
        if random {
            let speed = self.rand.next_float() * 0.5;
            let angle = self.rand.next_float() * math::PI_F32 * 2.0;
            e.motion.x = (-math::sin(angle) * speed) as f64;
            e.motion.z = (math::cos(angle) * speed) as f64;
            e.motion.y = 0.2f32 as f64;
        } else {
            let yaw = self.yaw / 180.0 * math::PI_F32;
            let pitch = self.pitch / 180.0 * math::PI_F32;
            e.motion.x = (-math::sin(yaw) * math::cos(pitch) * 0.3) as f64;
            e.motion.z = (math::cos(yaw) * math::cos(pitch) * 0.3) as f64;
            e.motion.y = (-math::sin(pitch) * 0.3 + 0.1) as f64;
            let angle = self.rand.next_float() * math::PI_F32 * 2.0;
            let speed = 0.02 * self.rand.next_float();
            e.motion.x += crate::jmath::java_cos(angle as f64) * speed as f64;
            e.motion.y += ((self.rand.next_float() - self.rand.next_float()) * 0.1) as f64;
            e.motion.z += crate::jmath::java_sin(angle as f64) * speed as f64;
        }
        w.entity_joined_world(e);
    }
    pub fn set_in_portal(&mut self) {
        if let Some(s) = self.player_state_mut() {
            if s.portal_cooldown > 0 {
                s.portal_cooldown = 10;
            } else {
                s.in_portal = true;
            }
        }
    }
    pub fn wake_up(&mut self, w: &mut World, reset_timer: bool, set_spawn: bool) {
        self.width = 0.6;
        self.height = 1.8;
        self.y_offset = 1.62;
        self.sneaking = self.player_state().unwrap().input.sneak;
        if let Some((x, y, z)) = self.player_state().unwrap().bed {
            if w.get_block_id(x, y, z) == 26 {
                let meta = w.get_block_metadata(x, y, z);
                w.set_block_metadata_with_notify(x, y, z, meta & !4);
                let (x, y, z) = Self::bed_exit(w, x, y, z, 0).unwrap_or((x, y + 1, z));
                self.set_position(
                    (x as f32 + 0.5) as f64,
                    (y as f32 + self.y_offset + 0.1) as f64,
                    (z as f32 + 0.5) as f64,
                );
            }
        }
        let s = self.player_state_mut().unwrap();
        s.sleeping = false;
        s.sleep_timer = if reset_timer { 0 } else { 100 };
        if set_spawn {
            s.spawn = s.bed;
        }
    }
    pub fn sleep_in_bed(&mut self, w: &mut World, x: i32, y: i32, z: i32) -> SleepStatus {
        if self.player_state().unwrap().sleeping || self.living.health <= 0 || self.dead {
            return SleepStatus::OtherProblem;
        }
        if w.dimension.is_hell_world() {
            return SleepStatus::NotPossibleHere;
        }
        if w.skylight_subtracted < 4 {
            return SleepStatus::NotPossibleNow;
        }
        if (self.pos.x - x as f64).abs() > 3.0
            || (self.pos.y - y as f64).abs() > 2.0
            || (self.pos.z - z as f64).abs() > 3.0
        {
            return SleepStatus::TooFarAway;
        }
        self.width = 0.2;
        self.height = 0.2;
        self.y_offset = 0.2;
        self.sneaking = false;
        let direction = w.get_block_metadata(x, y, z) & 3;
        let (ox, oz) = match direction {
            0 => (0.5f32, 0.9f32),
            1 => (0.1, 0.5),
            2 => (0.5, 0.1),
            _ => (0.9, 0.5),
        };
        self.set_position(
            (x as f32 + ox) as f64,
            (y as f32 + 15.0 / 16.0) as f64,
            (z as f32 + oz) as f64,
        );
        let s = self.player_state_mut().unwrap();
        s.bed_offset = match direction {
            0 => Vec3D::new(0.0, 0.0, -1.8f32 as f64),
            1 => Vec3D::new(1.8f32 as f64, 0.0, 0.0),
            2 => Vec3D::new(0.0, 0.0, 1.8f32 as f64),
            _ => Vec3D::new(-1.8f32 as f64, 0.0, 0.0),
        };
        s.sleeping = true;
        s.sleep_timer = 0;
        s.bed = Some((x, y, z));
        self.motion = Vec3D::default();
        SleepStatus::Ok
    }
    pub fn bed_exit(
        w: &mut World,
        x: i32,
        y: i32,
        z: i32,
        mut skip: i32,
    ) -> Option<(i32, i32, i32)> {
        let (dx, dz) =
            [(0, 1), (-1, 0), (0, -1), (1, 0)][(w.get_block_metadata(x, y, z) & 3) as usize];
        for half in 0..=1 {
            for bx in x - dx * half - 1..=x - dx * half + 1 {
                for bz in z - dz * half - 1..=z - dz * half + 1 {
                    if w.is_block_normal_cube(bx, y - 1, bz)
                        && w.get_block_id(bx, y, bz) == 0
                        && w.get_block_id(bx, y + 1, bz) == 0
                    {
                        if skip <= 0 {
                            return Some((bx, y, bz));
                        }
                        skip -= 1;
                    }
                }
            }
        }
        None
    }
}
