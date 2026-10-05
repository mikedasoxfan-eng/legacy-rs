//! Beta 1.8.1 Enderman and Silverfish special AI.
use super::{
    damage::{DamageKind, DamageSource},
    living::Target,
    Entity, EntityKind, Vec3D,
};
use crate::{math::floor_double as floor, World};
#[derive(Clone, Debug, Default)]
pub struct EndermanState {
    pub carried: i8,
    pub carried_data: i8,
    pub screaming: bool,
    pub teleport_ticks: i32,
    pub stare_ticks: i32,
}
pub fn enderman_can_carry(block: u8) -> bool {
    matches!(
        block,
        1 | 2
            | 3
            | 4
            | 5
            | 12
            | 13
            | 14
            | 15
            | 16
            | 17
            | 18
            | 19
            | 20
            | 21
            | 22
            | 24
            | 35
            | 37
            | 38
            | 39
            | 40
            | 41
            | 42
            | 45
            | 46
            | 47
            | 48
            | 56
            | 57
            | 58
            | 73
            | 74
            | 79
            | 81
            | 82
            | 86
            | 87
            | 88
            | 89
            | 91
            | 98
            | 99
            | 100
            | 103
    )
}
impl Entity {
    pub fn enderman_is_stared_at(&self, w: &mut World, target: Target) -> bool {
        let Some(id) = w.target_entity_id(target) else {
            return false;
        };
        let Some(player) = w.entities.get(&id) else {
            return false;
        };
        if player
            .player_state()
            .is_some_and(|p| p.inventory.armor[3].as_ref().is_some_and(|s| s.id == 86))
        {
            return false;
        }
        let Some(look) = player.look_vector() else {
            return false;
        };
        let look = look.normalize();
        let direction = Vec3D::new(
            self.pos.x - player.pos.x,
            self.bounding_box.min_y + (self.height / 2.0) as f64 - player.pos.y
                + player.eye_height() as f64,
            self.pos.z - player.pos.z,
        );
        let distance = direction.length();
        let direction = direction.normalize();
        look.x * direction.x + look.y * direction.y + look.z * direction.z > 1.0 - 0.025 / distance
            && self.can_see_target(w, target)
    }
    pub(super) fn enderman_find_target(&mut self, w: &mut World) -> Option<Target> {
        let target = w.closest_player(self.pos, 64.0).map(Target::Player)?;
        let stared = self.enderman_is_stared_at(w, target);
        if let EntityKind::Enderman(s) = &mut self.kind {
            if stared {
                let count = s.stare_ticks;
                s.stare_ticks += 1;
                if count == 5 {
                    s.stare_ticks = 0;
                    return Some(target);
                }
            } else {
                s.stare_ticks = 0;
            }
        }
        None
    }
    pub(super) fn enderman_before_living(&mut self, w: &mut World) {
        if !matches!(self.kind, EntityKind::Enderman(..)) {
            return;
        }
        if self.is_wet(w) {
            self.attack_from(w, DamageSource::new(DamageKind::Drown), 1);
        }
        if let EntityKind::Enderman(s) = &mut self.kind {
            s.screaming = self.living.target.is_some();
        }
        self.living.move_speed = if self.living.target.is_some() {
            4.5
        } else {
            0.3
        };
        let carried = if let EntityKind::Enderman(s) = &self.kind {
            s.carried
        } else {
            0
        };
        if carried == 0 {
            if self.rand.next_int(20) == 0 {
                let x = floor(self.pos.x - 2.0 + self.rand.next_double() * 4.0);
                let y = floor(self.pos.y + self.rand.next_double() * 3.0);
                let z = floor(self.pos.z - 2.0 + self.rand.next_double() * 4.0);
                let block = w.get_block_id(x, y, z);
                if enderman_can_carry(block) {
                    let data = w.get_block_metadata(x, y, z);
                    if let EntityKind::Enderman(s) = &mut self.kind {
                        s.carried = block as i8;
                        s.carried_data = data as i8;
                    }
                    w.set_block_with_notify(x, y, z, 0);
                }
            }
        } else if self.rand.next_int(2000) == 0 {
            let x = floor(self.pos.x - 1.0 + self.rand.next_double() * 2.0);
            let y = floor(self.pos.y + self.rand.next_double() * 2.0);
            let z = floor(self.pos.z - 1.0 + self.rand.next_double() * 2.0);
            let block = w.get_block_id(x, y, z);
            let below = w.get_block_id(x, y - 1, z);
            if block == 0 && below > 0 && crate::block::props(below).normal_render {
                let data = if let EntityKind::Enderman(s) = &self.kind {
                    s.carried_data as i32
                } else {
                    0
                };
                w.set_block_and_metadata_with_notify(x, y, z, carried as u8, data);
                if let EntityKind::Enderman(s) = &mut self.kind {
                    s.carried = 0;
                }
            }
        }
        for _ in 0..2 {
            let p = Vec3D::new(
                self.pos.x + (self.rand.next_double() - 0.5) * self.width as f64,
                self.pos.y + self.rand.next_double() * self.height as f64 - 0.25,
                self.pos.z + (self.rand.next_double() - 0.5) * self.width as f64,
            );
            let v = Vec3D::new(
                (self.rand.next_double() - 0.5) * 2.0,
                -self.rand.next_double(),
                (self.rand.next_double() - 0.5) * 2.0,
            );
            w.spawn_particle("portal", p, v);
        }
        if w.skylight_subtracted < 4 {
            let light = self.brightness(w);
            if light > 0.5
                && w.can_block_see_the_sky(floor(self.pos.x), floor(self.pos.y), floor(self.pos.z))
                && self.rand.next_float() * 30.0 < (light - 0.4) * 2.0
            {
                self.fire = 300;
            }
        }
        self.living.is_jumping = false;
        if let Some(target) = self.living.target {
            self.face_target(w, target, 100.0, 100.0);
            if matches!(target, Target::Player(_)) && self.enderman_is_stared_at(w, target) {
                self.living.move_strafing = 0.0;
                self.living.move_forward = 0.0;
                self.living.move_speed = 0.0;
                if self
                    .target_position(w, target)
                    .is_some_and(|(p, _, _)| p.square_distance(self.pos) < 16.0)
                {
                    self.enderman_teleport_random(w);
                }
                if let EntityKind::Enderman(s) = &mut self.kind {
                    s.teleport_ticks = 0;
                }
            } else if self
                .target_position(w, target)
                .is_some_and(|(p, _, _)| p.square_distance(self.pos) > 256.0)
            {
                let ticks = if let EntityKind::Enderman(s) = &mut self.kind {
                    let t = s.teleport_ticks;
                    s.teleport_ticks += 1;
                    t
                } else {
                    0
                };
                if ticks >= 30 && self.enderman_teleport_to_target(w, target) {
                    if let EntityKind::Enderman(s) = &mut self.kind {
                        s.teleport_ticks = 0;
                    }
                }
            }
        } else if let EntityKind::Enderman(s) = &mut self.kind {
            s.teleport_ticks = 0;
        }
    }
    pub fn enderman_teleport_random(&mut self, w: &mut World) -> bool {
        let x = self.pos.x + (self.rand.next_double() - 0.5) * 64.0;
        let y = self.pos.y + (self.rand.next_int(64) - 32) as f64;
        let z = self.pos.z + (self.rand.next_double() - 0.5) * 64.0;
        self.enderman_teleport(w, Vec3D::new(x, y, z))
    }
    pub fn enderman_teleport_to_target(&mut self, w: &mut World, target: Target) -> bool {
        let Some((p, eye, _)) = self.target_position(w, target) else {
            return false;
        };
        let direction = Vec3D::new(
            self.pos.x - p.x,
            self.bounding_box.min_y + (self.height / 2.0) as f64 - p.y + eye as f64,
            self.pos.z - p.z,
        )
        .normalize();
        let x = self.pos.x + (self.rand.next_double() - 0.5) * 8.0 - direction.x * 16.0;
        let y = self.pos.y + (self.rand.next_int(16) - 8) as f64 - direction.y * 16.0;
        let z = self.pos.z + (self.rand.next_double() - 0.5) * 8.0 - direction.z * 16.0;
        self.enderman_teleport(w, Vec3D::new(x, y, z))
    }
    pub fn enderman_teleport(&mut self, w: &mut World, destination: Vec3D) -> bool {
        let old = self.pos;
        self.pos = destination;
        let x = floor(self.pos.x);
        let mut y = floor(self.pos.y);
        let z = floor(self.pos.z);
        let mut success = false;
        if w.block_exists(x, y, z) {
            let mut ground = false;
            while !ground && y > 0 {
                let block = w.get_block_id(x, y - 1, z);
                if block != 0 && crate::block::mat(w.get_block_material(x, y - 1, z)).is_solid {
                    ground = true;
                } else {
                    self.pos.y -= 1.0;
                    y -= 1;
                }
            }
            if ground {
                self.set_position(self.pos.x, self.pos.y, self.pos.z);
                success = w
                    .get_colliding_bounding_boxes(Some(self.entity_id), self.bounding_box)
                    .is_empty()
                    && !w.get_is_any_liquid(self.bounding_box);
            }
        }
        if !success {
            self.set_position(old.x, old.y, old.z);
            return false;
        }
        for i in 0..128 {
            let t = i as f64 / 127.0;
            let v = Vec3D::new(
                ((self.rand.next_float() - 0.5) * 0.2) as f64,
                ((self.rand.next_float() - 0.5) * 0.2) as f64,
                ((self.rand.next_float() - 0.5) * 0.2) as f64,
            );
            let p = Vec3D::new(
                old.x
                    + (self.pos.x - old.x) * t
                    + (self.rand.next_double() - 0.5) * self.width as f64 * 2.0,
                old.y + (self.pos.y - old.y) * t + self.rand.next_double() * self.height as f64,
                old.z
                    + (self.pos.z - old.z) * t
                    + (self.rand.next_double() - 0.5) * self.width as f64 * 2.0,
            );
            w.spawn_particle("portal", p, v);
        }
        true
    }
    pub(super) fn silverfish_after_action(&mut self, w: &mut World) {
        let EntityKind::Silverfish { ally_timer } = &mut self.kind else {
            return;
        };
        let mut summon = false;
        if *ally_timer > 0 {
            *ally_timer -= 1;
            summon = *ally_timer == 0;
        }
        if summon {
            let x = floor(self.pos.x);
            let y = floor(self.pos.y);
            let z = floor(self.pos.z);
            let alternating = |limit: i32| -> Vec<i32> {
                let mut v = vec![0];
                for i in 1..=limit {
                    v.push(i);
                    v.push(-i);
                }
                v
            };
            'search: for dy in alternating(5) {
                for dx in alternating(10) {
                    for dz in alternating(10) {
                        let (x, y, z) = (x + dx, y + dy, z + dz);
                        if w.get_block_id(x, y, z) == 97 {
                            let data = w.get_block_metadata(x, y, z);
                            w.auxiliary_effect(2001, (x, y, z), 97 + data * 256);
                            w.set_block_with_notify(x, y, z, 0);
                            let mut fish = Self::living(w, "Silverfish").unwrap();
                            fish.set_location_and_angles(
                                x as f64 + 0.5,
                                y as f64,
                                z as f64 + 0.5,
                                0.0,
                                0.0,
                            );
                            let id = fish.entity_id;
                            if w.entity_joined_world(fish.clone()) {
                                let mut joined = w.entities.remove(&id).unwrap();
                                joined.explosion_particle(w);
                                w.entities.insert(id, joined);
                            } else {
                                fish.explosion_particle(w);
                            }
                            if self.rand.next_bool() {
                                break 'search;
                            }
                        }
                    }
                }
            }
        }
        if self.living.target.is_none() && self.living.path.is_none() {
            let x = floor(self.pos.x);
            let y = floor(self.pos.y + 0.5);
            let z = floor(self.pos.z);
            let side = self.rand.next_int(6) as usize;
            let (x, y, z) = (
                x + [0, 0, 0, 0, -1, 1][side],
                y + [-1, 1, 0, 0, 0, 0][side],
                z + [0, 0, -1, 1, 0, 0][side],
            );
            let block = w.get_block_id(x, y, z);
            if matches!(block, 1 | 4 | 98) {
                let data = match block {
                    1 => 0,
                    4 => 1,
                    _ => 2,
                };
                w.set_block_and_metadata_with_notify(x, y, z, 97, data);
                self.explosion_particle(w);
                self.set_dead(w);
            } else {
                self.wander(w);
            }
        } else if self.living.target.is_some() && self.living.path.is_none() {
            self.living.target = None;
        }
    }
}
