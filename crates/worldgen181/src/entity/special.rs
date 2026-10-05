use super::{living::Target, AxisAlignedBB as BB, Entity, EntityKind, Vec3D};
use crate::{
    math::{self, floor_double as floor, PI_F32 as PI},
    World,
};
#[derive(Clone, Debug, Default)]
pub struct GhastState {
    pub course_cooldown: i32,
    pub waypoint: Vec3D,
    pub target: Option<Target>,
    pub aggro_cooldown: i32,
    pub prev_attack_counter: i32,
    pub attack_counter: i32,
    pub attacking: bool,
}
#[derive(Clone, Debug)]
pub struct WolfState {
    pub flags: u8,
    pub owner: String,
    pub watched_health: i32,
    pub interested: bool,
    pub interest: f32,
    pub prev_interest: f32,
    pub shaking: bool,
    pub shake_active: bool,
    pub shake_time: f32,
    pub prev_shake_time: f32,
}
impl Default for WolfState {
    fn default() -> Self {
        Self {
            flags: 0,
            owner: String::new(),
            watched_health: 0,
            interested: false,
            interest: 0.0,
            prev_interest: 0.0,
            shaking: false,
            shake_active: false,
            shake_time: 0.0,
            prev_shake_time: 0.0,
        }
    }
}
impl Entity {
    pub(super) fn prepare_special_attack(
        &mut self,
        w: &mut World,
        attacker: Option<i32>,
        damage: &mut i32,
    ) {
        if matches!(self.kind, EntityKind::PigZombie { .. })
            && attacker.is_some_and(|id| {
                id < 0
                    || w.entities
                        .get(&id)
                        .is_some_and(|e| e.player_state().is_some())
            })
        {
            let id = attacker.unwrap();
            let target = if id < 0 {
                Target::Player((-1 - id) as usize)
            } else {
                Target::Entity(id)
            };
            for id in w.entities_within_aabb(
                Some(self.entity_id),
                self.bounding_box.expand(32.0, 32.0, 32.0),
            ) {
                if let Some(e) = w.entities.get_mut(&id) {
                    if matches!(e.kind, EntityKind::PigZombie { .. }) {
                        e.become_angry_at(target);
                    }
                }
            }
            self.become_angry_at(target);
        }
        if let EntityKind::Wolf(s) = &mut self.kind {
            s.flags &= !1;
            if attacker.is_some_and(|id| {
                id >= 0
                    && !w.entities.get(&id).is_some_and(|e| {
                        matches!(e.kind, EntityKind::Arrow(..) | EntityKind::Player(..))
                    })
            }) {
                *damage = (*damage + 1) / 2;
            }
        }
    }
    pub(super) fn after_special_attack(&mut self, w: &mut World, attacker: Option<i32>, hit: bool) {
        if !hit {
            return;
        }
        let Some(mut id) = attacker else {
            return;
        };
        let source_player = id < 0
            || w.entities
                .get(&id)
                .is_some_and(|e| e.player_state().is_some());
        let target = if id < 0 {
            Target::Player((-1 - id) as usize)
        } else {
            Target::Entity(id)
        };
        let EntityKind::Wolf(s) = &mut self.kind else {
            return;
        };
        if s.flags & 6 == 0 {
            if source_player {
                s.flags |= 2;
                self.living.target = Some(target);
            }
            if let Some(EntityKind::Arrow(a)) = w.entities.get(&id).map(|e| &e.kind) {
                if let Some(owner) = a.owner {
                    id = owner;
                }
            }
            let target = if id < 0 {
                Target::Player((-1 - id) as usize)
            } else {
                Target::Entity(id)
            };
            let source_player = id < 0
                || w.entities
                    .get(&id)
                    .is_some_and(|e| e.player_state().is_some());
            let source_living = id < 0 || w.entities.get(&id).is_some_and(|e| e.kind.is_living());
            if source_living {
                if w.loaded_entity_list.contains(&self.entity_id) && self.living.target.is_none() {
                    self.living.target = Some(target);
                }
                let b = BB::new(
                    self.pos.x,
                    self.pos.y,
                    self.pos.z,
                    self.pos.x + 1.0,
                    self.pos.y + 1.0,
                    self.pos.z + 1.0,
                )
                .expand(16.0, 4.0, 16.0);
                for eid in w.entities_within_aabb(None, b) {
                    if let Some(e) = w.entities.get_mut(&eid) {
                        if let EntityKind::Wolf(s) = &mut e.kind {
                            if s.flags & 4 == 0 && e.living.target.is_none() {
                                e.living.target = Some(target);
                                if source_player {
                                    s.flags |= 2;
                                }
                            }
                        }
                    }
                }
            }
        } else if id != self.entity_id {
            let owner = if id < 0 {
                Some("Player")
            } else {
                w.entities
                    .get(&id)
                    .and_then(|e| e.player_state())
                    .map(|s| s.username.as_str())
            };
            if !(s.flags & 4 != 0 && owner.is_some_and(|name| s.owner.eq_ignore_ascii_case(name))) {
                self.living.target = Some(target);
            }
        }
    }
    pub(super) fn update_ridden(&mut self, w: &mut World) {
        let before = self.pos;
        let Some(parent) = self.riding.and_then(|id| w.entities.get(&id)).cloned() else {
            self.riding = None;
            return;
        };
        if parent.dead {
            self.riding = None;
            if self.kind.is_living() {
                self.living.prev_walk_animation=self.living.walk_animation;
                self.living.walk_animation=0.0;
            }
            if let Some(s)=self.player_state_mut() {s.prev_camera_yaw=s.camera_yaw;s.camera_yaw=0.0;}
            return;
        }
        self.motion = Vec3D::default();
        self.on_update(w);
        if self.riding.is_some() {
            let parent=self.riding.and_then(|id|w.entities.get(&id)).cloned().unwrap_or(parent);
            parent.update_rider_position(self);
            self.rider_yaw_delta += (parent.yaw - parent.prev_yaw) as f64;
            self.rider_pitch_delta += (parent.pitch - parent.prev_pitch) as f64;
            while self.rider_yaw_delta >= 180.0 {
                self.rider_yaw_delta -= 360.0;
            }
            while self.rider_yaw_delta < -180.0 {
                self.rider_yaw_delta += 360.0;
            }
            while self.rider_pitch_delta >= 180.0 {
                self.rider_pitch_delta -= 360.0;
            }
            while self.rider_pitch_delta < -180.0 {
                self.rider_pitch_delta += 360.0;
            }
            let yaw = (self.rider_yaw_delta * 0.5).clamp(-10.0, 10.0);
            let pitch = (self.rider_pitch_delta * 0.5).clamp(-10.0, 10.0);
            self.rider_yaw_delta -= yaw;
            self.rider_pitch_delta -= pitch;
            self.yaw = (self.yaw as f64 + yaw) as f32;
            self.pitch = (self.pitch as f64 + pitch) as f32;
        }
        if self.kind.is_living() {
            self.living.prev_walk_animation=self.living.walk_animation;
            self.living.walk_animation=0.0;
        }
        if let Some(s) = self.player_state_mut() {
            s.prev_camera_yaw = s.camera_yaw;
            s.camera_yaw = 0.0;
            let d = (self.pos.x - before.x, self.pos.y - before.y, self.pos.z - before.z);
            self.add_mounted_movement_stat(w, d.0, d.1, d.2);
        }
    }
    pub(super) fn wolf_attack_target(&mut self, w: &mut World, target: Target, distance: f32) {
        let Some((p, _, _)) = self.target_position(w, target) else {
            return;
        };
        if distance > 2.0 && distance < 6.0 && self.rand.next_int(10) == 0 {
            if self.on_ground {
                let dx = p.x - self.pos.x;
                let dz = p.z - self.pos.z;
                let len = (dx * dx + dz * dz).sqrt() as f32 as f64;
                self.motion.x = dx / len * 0.5 * 0.8f32 as f64 + self.motion.x * 0.2f32 as f64;
                self.motion.z = dz / len * 0.5 * 0.8f32 as f64 + self.motion.z * 0.2f32 as f64;
                self.motion.y = 0.4f32 as f64;
            }
        } else if (distance as f64) < 1.5 {
            let (min, max) = match target {
                Target::Player(_) => w
                    .target_entity_id(target)
                    .and_then(|id| w.entities.get(&id))
                    .map_or((p.y, p.y + 1.8f32 as f64), |e| {
                        (e.bounding_box.min_y, e.bounding_box.max_y)
                    }),
                Target::Entity(id) => {
                    let e = &w.entities[&id];
                    (e.bounding_box.min_y, e.bounding_box.max_y)
                }
            };
            if max > self.bounding_box.min_y && min < self.bounding_box.max_y {
                self.living.attack_time = 20;
                let damage = if matches!(&self.kind,EntityKind::Wolf(s) if s.flags&4!=0) {
                    4
                } else {
                    2
                };
                if let Some(id) = w.target_entity_id(target) {
                    if let Some(mut e) = w.entities.remove(&id) {
                        w.entities.insert(self.entity_id, self.clone());
                        e.attack(w, Some(self.entity_id), damage);
                        w.entities.remove(&self.entity_id);
                        w.entities.insert(id, e);
                    }
                }
            }
        }
    }
    pub fn set_dead(&mut self, w: &mut World) {
        if let EntityKind::Slime { size, .. } = self.kind {
            if size > 1 && self.living.health == 0 {
                for k in 0..4 {
                    let x = (k % 2) as f32 - 0.5;
                    let z = (k / 2) as f32 - 0.5;
                    let mut child = Self::living(w, "Slime").unwrap();
                    child.set_slime_size(size / 2);
                    child.set_location_and_angles(
                        self.pos.x + (x * size as f32 / 4.0) as f64,
                        self.pos.y + 0.5,
                        self.pos.z + (z * size as f32 / 4.0) as f64,
                        self.rand.next_float() * 360.0,
                        0.0,
                    );
                    w.entity_joined_world(child);
                }
            }
        }
        self.dead = true;
    }
    pub(super) fn ghast_action(&mut self, w: &mut World) {
        if w.difficulty == 0 {
            self.dead = true;
        }
        self.despawn(w);
        let EntityKind::Ghast(mut s) = self.kind.clone() else {
            return;
        };
        s.prev_attack_counter = s.attack_counter;
        let d = Vec3D::new(
            s.waypoint.x - self.pos.x,
            s.waypoint.y - self.pos.y,
            s.waypoint.z - self.pos.z,
        );
        let length = d.length();
        if length < 1.0 || length > 60.0 {
            s.waypoint = Vec3D::new(
                self.pos.x + ((self.rand.next_float() * 2.0 - 1.0) * 16.0) as f64,
                self.pos.y + ((self.rand.next_float() * 2.0 - 1.0) * 16.0) as f64,
                self.pos.z + ((self.rand.next_float() * 2.0 - 1.0) * 16.0) as f64,
            );
        }
        let cooldown = s.course_cooldown;
        s.course_cooldown -= 1;
        if cooldown <= 0 {
            s.course_cooldown += self.rand.next_int(5) + 2;
            let step = Vec3D::new(
                (s.waypoint.x - self.pos.x) / length,
                (s.waypoint.y - self.pos.y) / length,
                (s.waypoint.z - self.pos.z) / length,
            );
            let mut b = self.bounding_box;
            let mut clear = true;
            let mut k = 1;
            while (k as f64) < length {
                b.offset(step.x, step.y, step.z);
                if !w
                    .get_colliding_bounding_boxes(Some(self.entity_id), b)
                    .is_empty()
                {
                    clear = false;
                    break;
                }
                k += 1;
            }
            if clear {
                self.motion.x += d.x / length * 0.1;
                self.motion.y += d.y / length * 0.1;
                self.motion.z += d.z / length * 0.1;
            } else {
                s.waypoint = self.pos;
            }
        }
        if s.target.is_some_and(|t| {
            self.target_position(w, t)
                .map_or(true, |(_, _, alive)| !alive)
        }) {
            s.target = None;
        }
        let search = if s.target.is_none() {
            true
        } else {
            let old = s.aggro_cooldown;
            s.aggro_cooldown -= 1;
            old <= 0
        };
        if search {
            s.target = w.closest_player(self.pos, 100.0).map(Target::Player);
            if s.target.is_some() {
                s.aggro_cooldown = 20;
            }
        }
        let target = s
            .target
            .and_then(|t| self.target_position(w, t).map(|v| (t, v)));
        if let Some((t, (p, _, _))) =
            target.filter(|(_, (p, _, _))| p.square_distance(self.pos) < 4096.0)
        {
            let target_y = match t {
                Target::Player(_) => p.y + 1.8f32 as f64 / 2.0,
                Target::Entity(id) => {
                    let e = &w.entities[&id];
                    e.bounding_box.min_y + (e.height / 2.0) as f64
                }
            };
            let aim = Vec3D::new(
                p.x - self.pos.x,
                target_y - (self.pos.y + (self.height / 2.0) as f64),
                p.z - self.pos.z,
            );
            self.yaw = -(crate::jmath::java_atan2(aim.x, aim.z) as f32) * 180.0 / PI;
            self.living.render_yaw_offset = self.yaw;
            if self.can_see_target(w, t) {
                if s.attack_counter == 10 {
                    let pitch = (self.rand.next_float() - self.rand.next_float()) * 0.2 + 1.0;
                    w.play_sound_at_entity(self, "mob.ghast.charge", 10.0, pitch);
                }
                s.attack_counter += 1;
                if s.attack_counter == 20 {
                    let pitch = (self.rand.next_float() - self.rand.next_float()) * 0.2 + 1.0;
                    w.play_sound_at_entity(self, "mob.ghast.fireball", 10.0, pitch);
                    let mut fireball = Self::fireball_from_owner(w, self, aim);
                    let look = self.look_vector().unwrap();
                    fireball.pos = Vec3D::new(
                        self.pos.x + look.x * 4.0,
                        self.pos.y + (self.height / 2.0) as f64 + 0.5,
                        self.pos.z + look.z * 4.0,
                    );
                    w.entity_joined_world(fireball);
                    s.attack_counter = -40;
                }
            } else if s.attack_counter > 0 {
                s.attack_counter -= 1;
            }
        } else {
            self.yaw =
                -(crate::jmath::java_atan2(self.motion.x, self.motion.z) as f32) * 180.0 / PI;
            self.living.render_yaw_offset = self.yaw;
            if s.attack_counter > 0 {
                s.attack_counter -= 1;
            }
        }
        s.attacking = s.attack_counter > 10;
        self.kind = EntityKind::Ghast(s);
    }
    pub(super) fn flying_heading(&mut self, w: &mut World, strafe: f32, forward: f32) {
        let drag;
        if self.in_water || self.handle_lava_movement(w) {
            drag = if self.in_water { 0.8 } else { 0.5 };
            self.move_flying(strafe, forward, 0.02);
        } else {
            let friction = self.ground_friction(w);
            let accel = 0.16277136 / (friction * friction * friction);
            self.move_flying(
                strafe,
                forward,
                if self.on_ground { 0.1 * accel } else { 0.02 },
            );
            drag = self.ground_friction(w);
        }
        self.move_entity(w, self.motion.x, self.motion.y, self.motion.z);
        self.motion.x *= drag as f64;
        self.motion.y *= drag as f64;
        self.motion.z *= drag as f64;
        self.living.prev_limb_amount = self.living.limb_amount;
        let dx = self.pos.x - self.prev_pos.x;
        let dz = self.pos.z - self.prev_pos.z;
        let a = ((dx * dx + dz * dz).sqrt() as f32 * 4.0).min(1.0);
        self.living.limb_amount += (a - self.living.limb_amount) * 0.4;
        self.living.limb_swing += self.living.limb_amount;
    }
    pub(super) fn wolf_after_action(&mut self, w: &mut World) {
        let EntityKind::Wolf(s) = &self.kind else {
            return;
        };
        let tamed = s.flags & 4 != 0;
        if !self.living.has_attacked && self.living.path.is_none() && tamed && self.riding.is_none()
        {
            let owner = w
                .player_entities
                .iter()
                .flatten()
                .filter_map(|id| w.entities.get(id))
                .find(|e| e.player_state().is_some_and(|p| p.username == s.owner))
                .cloned();
            let target = owner.as_ref().map(|e| (e.pos, e.bounding_box.min_y));
            let target = target.or_else(|| {
                if s.owner == "Player" {
                    w.players
                        .iter()
                        .enumerate()
                        .find(|(i, _)| w.player_entities.get(*i).copied().flatten().is_none())
                        .map(|(_, p)| (Vec3D::new(p.0, p.1, p.2), p.1))
                } else {
                    None
                }
            });
            if let Some((target, feet)) = target {
                let distance = target.distance(self.pos) as f32;
                if distance > 5.0 {
                    self.follow_owner(w, Vec3D::new(target.x, feet, target.z), distance);
                }
            } else if !self.in_water {
                if let EntityKind::Wolf(s) = &mut self.kind {
                    s.flags |= 1;
                }
            }
        } else if self.living.target.is_none()
            && self.living.path.is_none()
            && !tamed
            && w.rand.next_int(100) == 0
        {
            let b = BB::new(
                self.pos.x,
                self.pos.y,
                self.pos.z,
                self.pos.x + 1.0,
                self.pos.y + 1.0,
                self.pos.z + 1.0,
            )
            .expand(16.0, 4.0, 16.0);
            let sheep: Vec<_> = w
                .entities_within_aabb(None, b)
                .into_iter()
                .filter(|id| matches!(w.entities[id].kind, EntityKind::Sheep { .. }))
                .collect();
            if !sheep.is_empty() {
                self.living.target = Some(Target::Entity(
                    sheep[w.rand.next_int(sheep.len() as i32) as usize],
                ));
            }
        }
        if let EntityKind::Wolf(s) = &mut self.kind {
            if self.in_water {
                s.flags &= !1;
            }
            if !w.multiplayer_world {
                s.watched_health = self.living.health;
            }
        }
    }
    pub(super) fn follow_owner(&mut self, w: &mut World, target: Vec3D, distance: f32) {
        let path = w.entity_path(self, target, 16.0, 16.0);
        if path.is_none() && distance > 12.0 {
            let x = floor(target.x) - 2;
            let z = floor(target.z) - 2;
            let y = floor(target.y);
            for dx in 0..=4 {
                for dz in 0..=4 {
                    if (dx < 1 || dz < 1 || dx > 3 || dz > 3)
                        && w.is_block_normal_cube(x + dx, y - 1, z + dz)
                        && !w.is_block_normal_cube(x + dx, y, z + dz)
                        && !w.is_block_normal_cube(x + dx, y + 1, z + dz)
                    {
                        self.set_location_and_angles(
                            ((x + dx) as f32 + 0.5) as f64,
                            y as f64,
                            ((z + dz) as f32 + 0.5) as f64,
                            self.yaw,
                            self.pitch,
                        );
                        return;
                    }
                }
            }
        } else {
            self.living.path = path;
        }
    }
    pub(super) fn wolf_living(&mut self, w: &World) {
        let held = self
            .living
            .current_target
            .and_then(|target| match target {
                Target::Entity(id) => w.entities.get(&id).and_then(Entity::player_state),
                Target::Player(index) => w
                    .player_entities
                    .get(index)
                    .copied()
                    .flatten()
                    .and_then(|id| w.entities.get(&id))
                    .and_then(Entity::player_state),
            })
            .and_then(|p| p.inventory.held())
            .map(|s| s.id);
        if let EntityKind::Wolf(s) = &mut self.kind {
            s.interested = self.living.path.is_none()
                && s.flags & 2 == 0
                && held.is_some_and(|id| {
                    if s.flags & 4 == 0 {
                        id == 352
                    } else {
                        matches!(id, 319 | 320 | 363 | 364 | 365 | 366 | 367)
                    }
                });
            if !self.network.multiplayer_entity && s.shaking && !s.shake_active && self.living.path.is_none() && self.on_ground {
                s.shake_active = true;
                s.shake_time = 0.0;
                s.prev_shake_time = 0.0;
            }
        }
    }
    pub(super) fn wolf_hearts_or_smoke(&mut self, w: &mut World, hearts: bool) {
        for _ in 0..7 {
            let velocity = Vec3D::new(self.rand.next_gaussian() * 0.02,
                self.rand.next_gaussian() * 0.02, self.rand.next_gaussian() * 0.02);
            let position = Vec3D::new(
                self.pos.x + (self.rand.next_float() * self.width * 2.0) as f64 - self.width as f64,
                self.pos.y + 0.5 + (self.rand.next_float() * self.height) as f64,
                self.pos.z + (self.rand.next_float() * self.width * 2.0) as f64 - self.width as f64);
            w.spawn_particle(if hearts { "heart" } else { "smoke" }, position, velocity);
        }
    }
    pub(super) fn wolf_update(&mut self, w: &mut World) {
        let wet = self.is_wet(w);
        if let EntityKind::Wolf(s) = &mut self.kind {
            s.prev_interest = s.interest;
            s.interest += (if s.interested { 1.0 } else { 0.0 } - s.interest) * 0.4;
            if s.interested {
                self.living.chase_ticks = 10;
            }
            if wet {
                s.shaking = true;
                s.shake_active = false;
                s.shake_time = 0.0;
                s.prev_shake_time = 0.0;
            } else if (s.shaking || s.shake_active) && s.shake_active {
                if s.shake_time == 0.0 {
                    let pitch = (self.rand.next_float() - self.rand.next_float()) * 0.2 + 1.0;
                    w.play_sound(
                        "mob.wolf.shake",
                        self.pos.add(0.0, -(self.y_offset as f64), 0.0),
                        0.4,
                        pitch,
                    );
                }
                s.prev_shake_time = s.shake_time;
                s.shake_time += 0.05;
                if s.prev_shake_time >= 2.0 {
                    s.shaking = false;
                    s.shake_active = false;
                    s.prev_shake_time = 0.0;
                    s.shake_time = 0.0;
                }
                if s.shake_time > 0.4 {
                    let n = (math::sin((s.shake_time - 0.4) * PI) * 7.0) as i32;
                    for _ in 0..n {
                        let dx = (self.rand.next_float() * 2.0 - 1.0) * self.width * 0.5;
                        let dz = (self.rand.next_float() * 2.0 - 1.0) * self.width * 0.5;
                        w.spawn_particle(
                            "splash",
                            super::Vec3D::new(
                                self.pos.x + dx as f64,
                                (self.bounding_box.min_y as f32 + 0.8) as f64,
                                self.pos.z + dz as f64,
                            ),
                            self.motion,
                        );
                    }
                }
            }
        }
    }
}
