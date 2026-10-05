use super::{block_table, path::PathEntity, Entity, EntityKind};
use crate::{
    block::*,
    math::{self, floor_double as floor},
    World,
};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    Player(usize),
    Entity(i32),
}
#[derive(Clone, Debug)]
pub struct LivingState {
    pub effects: Vec<super::potion::PotionEffect>,
    pub effect_capacity: usize,
    pub flee_ticks: i32,
    pub experience_value: i32,
    pub recently_hit: i32,
    pub attacking_player: Option<i32>,
    pub arrows_stuck: i32,
    pub arrow_timer: i32,
    pub ground_acceleration: f32,
    pub air_acceleration: f32,
    pub health: i32,
    pub prev_health: i32,
    pub hearts_life: i32,
    pub hearts_halves_life: i32,
    pub hurt_time: i32,
    pub max_hurt_time: i32,
    pub death_time: i32,
    pub attack_time: i32,
    pub attacked_at_yaw: f32,
    pub living_sound_time: i32,
    pub entity_age: i32,
    pub last_damage: i32,
    pub dying: bool,
    pub render_yaw_offset: f32,
    pub prev_render_yaw_offset: f32,
    pub limb_swing: f32,
    pub limb_amount: f32,
    pub prev_limb_amount: f32,
    pub swing_progress: f32,
    pub prev_swing_progress: f32,
    pub walk_animation: f32,
    pub prev_walk_animation: f32,
    pub walk_phase: f32,
    pub prev_walk_phase: f32,
    pub random_animation: f32,
    pub animation_speed: f32,
    pub random_scale: f32,
    pub move_strafing: f32,
    pub move_forward: f32,
    pub random_yaw_velocity: f32,
    pub is_jumping: bool,
    pub default_pitch: f32,
    pub move_speed: f32,
    pub current_target: Option<Target>,
    pub chase_ticks: i32,
    pub target: Option<Target>,
    pub has_attacked: bool,
    pub path: Option<PathEntity>,
}
impl Default for LivingState {
    fn default() -> Self {
        Self {
            effects: Vec::new(),
            effect_capacity: 16,
            flee_ticks: 0,
            experience_value: 0,
            recently_hit: 0,
            attacking_player: None,
            arrows_stuck: 0,
            arrow_timer: 0,
            ground_acceleration: 0.1,
            air_acceleration: 0.02,
            health: 10,
            prev_health: 0,
            hearts_life: 0,
            hearts_halves_life: 20,
            hurt_time: 0,
            max_hurt_time: 0,
            death_time: 0,
            attack_time: 0,
            attacked_at_yaw: 0.0,
            living_sound_time: 0,
            entity_age: 0,
            last_damage: 0,
            dying: false,
            render_yaw_offset: 0.0,
            prev_render_yaw_offset: 0.0,
            limb_swing: 0.0,
            limb_amount: 0.0,
            prev_limb_amount: 0.0,
            swing_progress: 0.0,
            prev_swing_progress: 0.0,
            walk_animation: 0.0,
            prev_walk_animation: 0.0,
            walk_phase: 0.0,
            prev_walk_phase: 0.0,
            random_animation: 0.0,
            animation_speed: 0.0,
            random_scale: 0.0,
            move_strafing: 0.0,
            move_forward: 0.0,
            random_yaw_velocity: 0.0,
            is_jumping: false,
            default_pitch: 0.0,
            move_speed: 0.7,
            current_target: None,
            chase_ticks: 0,
            target: None,
            has_attacked: false,
            path: None,
        }
    }
}
impl Entity {
    pub fn living(w: &mut World, name: &str) -> Option<Self> {
        let kind = match name {
            "Player" => EntityKind::Player(Box::default()),
            "Pig" => EntityKind::Pig { saddled: false },
            "Sheep" => EntityKind::Sheep {
                sheared: false,
                fleece_color: 0,
            },
            "Cow" => EntityKind::Cow,
            "Chicken" => EntityKind::Chicken {
                flap: 0.0,
                prev_flap: 0.0,
                flap_amount: 0.0,
                prev_flap_amount: 0.0,
                flap_speed: 1.0,
                egg_timer: 0,
            },
            "Ghast" => EntityKind::Ghast(Default::default()),
            "Wolf" => EntityKind::Wolf(Default::default()),
            "Zombie" => EntityKind::Zombie,
            "Skeleton" => EntityKind::Skeleton,
            "Spider" => EntityKind::Spider,
            "CaveSpider" => EntityKind::CaveSpider,
            "Enderman" => EntityKind::Enderman(Default::default()),
            "Silverfish" => EntityKind::Silverfish { ally_timer: 0 },
            "Giant" => EntityKind::Giant,
            "PigZombie" => EntityKind::PigZombie {
                anger: 0,
                sound_delay: 0,
            },
            "Creeper" => EntityKind::Creeper {
                ignition: 0,
                prev_ignition: 0,
                state: -1,
                powered: false,
            },
            "Squid" => EntityKind::Squid(Default::default()),
            "Slime" => EntityKind::Slime {
                size: 1,
                squish: 0.0,
                prev_squish: 0.0,
                jump_delay: 0,
            },
            _ => return None,
        };
        let mut e = Self::base(w, kind);
        e.living.random_scale =
            (w.math_random.next_double() * 0.9f32 as f64 + 0.1f32 as f64) as f32;
        e.prevent_entity_spawning = true;
        e.living.animation_speed = (w.math_random.next_double() + 1.0) as f32 * 0.01;
        e.set_position(e.pos.x, e.pos.y, e.pos.z);
        e.living.random_animation = w.math_random.next_double() as f32 * 12398.0;
        e.yaw = (w.math_random.next_double() * math::PI_F32 as f64 * 2.0) as f32;
        e.step_height = 0.5;
        match &mut e.kind {
            EntityKind::Pig { .. } => {
                e.width = 0.9;
                e.height = 0.9;
            }
            EntityKind::Sheep { .. } | EntityKind::Cow => {
                e.width = 0.9;
                e.height = 1.3;
            }
            EntityKind::Chicken { egg_timer, .. } => {
                e.width = 0.3;
                e.height = 0.7;
                e.living.health = 4;
                *egg_timer = e.rand.next_int(6000) + 6000;
            }
            _ => {}
        }
        if e.kind.is_mob() {
            e.living.health = 20;
            e.living.experience_value = 5;
        }
        match &mut e.kind {
            EntityKind::Ghast(..) => {
                e.width = 4.0;
                e.height = 4.0;
                e.immune_to_fire = true;
                e.living.experience_value = 5;
            }
            EntityKind::Wolf(..) => {
                e.width = 0.8;
                e.height = 0.8;
                e.living.move_speed = 1.1;
                e.living.health = 8;
            }
            EntityKind::Zombie => e.living.move_speed = 0.5,
            EntityKind::PigZombie { .. } => {
                e.living.move_speed = 0.5;
                e.immune_to_fire = true;
            }
            EntityKind::Spider | EntityKind::CaveSpider => {
                e.width = 1.4;
                e.height = 0.9;
                e.living.move_speed = 0.8;
            }
            EntityKind::Giant => {
                e.living.move_speed = 0.5;
                e.living.health *= 10;
                e.y_offset *= 6.0;
                e.width *= 6.0;
                e.height *= 6.0;
            }
            EntityKind::Squid(s) => {
                e.width = 0.95;
                e.height = 0.95;
                s.rotation_speed = 1.0 / (e.rand.next_float() + 1.0) * 0.2;
            }
            EntityKind::Slime { jump_delay, .. } => {
                let size = 1 << e.rand.next_int(3);
                e.y_offset = 0.0;
                *jump_delay = e.rand.next_int(20) + 10;
                e.set_slime_size(size);
            }
            _ => {}
        }
        match e.kind {
            EntityKind::CaveSpider => {
                e.width = 0.7;
                e.height = 0.5;
            }
            EntityKind::Enderman(..) => {
                e.width = 0.6;
                e.height = 2.9;
                e.step_height = 1.0;
                e.living.move_speed = 0.2;
            }
            EntityKind::Silverfish { .. } => {
                e.width = 0.3;
                e.height = 0.7;
                e.living.move_speed = 0.6;
            }
            EntityKind::Slime { size, .. } => e.living.experience_value = size,
            _ => {}
        }
        Some(e)
    }
    pub fn eye_height(&self) -> f32 {
        if matches!(self.kind, EntityKind::Player(..)) {
            0.12
        } else if matches!(self.kind, EntityKind::Wolf(..)) {
            self.height * 0.8
        } else if self.kind.is_living() {
            self.height * 0.85
        } else {
            0.0
        }
    }
    pub fn is_inside_material(&self, w: &mut World, material: u8) -> bool {
        let eye = self.pos.y + self.eye_height() as f64;
        let (x, y, z) = (
            floor(self.pos.x),
            math::floor_float(floor(eye) as f32),
            floor(self.pos.z),
        );
        if w.get_block_material(x, y, z) != material {
            return false;
        }
        let m = w.get_block_metadata(x, y, z);
        let air = (if m >= 8 { 1 } else { m + 1 }) as f32 / 9.0 - 1.0f32 / 9.0;
        eye < ((y + 1) as f32 - air) as f64
    }
    fn inside_opaque_block(&self, w: &mut World) -> bool {
        if self.player_state().is_some_and(|s| s.sleeping) {
            return false;
        }
        for i in 0..8 {
            let x = ((i % 2) as f32 - 0.5) * self.width * 0.9;
            let y = (((i / 2) % 2) as f32 - 0.5) * 0.1;
            let z = (((i / 4) % 2) as f32 - 0.5) * self.width * 0.9;
            if w.is_block_normal_cube(
                floor(self.pos.x + x as f64),
                floor(self.pos.y + self.eye_height() as f64 + y as f64),
                floor(self.pos.z + z as f64),
            ) {
                return true;
            }
        }
        false
    }
    pub(super) fn living_entity_update(&mut self, w: &mut World) {
        self.living.prev_swing_progress = self.living.swing_progress;
        self.on_entity_update(w);
        let sound = self.living.living_sound_time;
        self.living.living_sound_time += 1;
        if self.rand.next_int(1000) < sound {
            self.living.living_sound_time = if self.kind.is_mob()
                || matches!(
                    self.kind,
                    EntityKind::Slime { .. } | EntityKind::Ghast(..) | EntityKind::Player(..)
                ) {
                -80
            } else {
                -120
            };
            if let Some(name) = self.living_sound() {
                let pitch = (self.rand.next_float() - self.rand.next_float()) * 0.2 + 1.0;
                w.play_sound_at_entity(self, name, self.sound_volume(), pitch);
            }
        }
        if !self.dead && self.living.health > 0 && self.inside_opaque_block(w) {
            self.attack_from(
                w,
                super::damage::DamageSource::new(super::damage::DamageKind::InWall),
                1,
            );
        }
        if self.immune_to_fire {
            self.fire = 0;
        }
        if !self.dead
            && self.living.health > 0
            && self.is_inside_material(w, MAT_WATER)
            && !matches!(self.kind, EntityKind::Squid(..))
            && self.potion_effect(super::potion::WATER_BREATHING).is_none()
        {
            self.air -= 1;
            if self.air == -20 {
                self.air = 0;
                for _ in 0..8 {
                    let x = self.rand.next_float() - self.rand.next_float();
                    let y = self.rand.next_float() - self.rand.next_float();
                    let z = self.rand.next_float() - self.rand.next_float();
                    w.spawn_particle(
                        "bubble",
                        self.pos.add(x as f64, y as f64, z as f64),
                        self.motion,
                    );
                }
                self.attack_from(
                    w,
                    super::damage::DamageSource::new(super::damage::DamageKind::Drown),
                    2,
                );
            }
            self.fire = 0;
        } else {
            self.air = self.max_air;
        }
        if self.living.attack_time > 0 {
            self.living.attack_time -= 1;
        }
        if self.living.hurt_time > 0 {
            self.living.hurt_time -= 1;
        }
        if self.living.hearts_life > 0 {
            self.living.hearts_life -= 1;
        }
        if self.living.health <= 0 {
            self.living.death_time += 1;
            if self.living.death_time > 20 {
                if self.living.recently_hit > 0 || self.player_state().is_some() {
                    let mut xp = if let Some(p) = self.player_state() {
                        p.experience.total >> 1
                    } else if matches!(
                        self.kind,
                        EntityKind::Pig { .. }
                            | EntityKind::Sheep { .. }
                            | EntityKind::Cow
                            | EntityKind::Chicken { .. }
                            | EntityKind::Wolf(..)
                            | EntityKind::Squid(..)
                    ) {
                        1 + w.rand.next_int(3)
                    } else {
                        self.living.experience_value
                    };
                    while xp > 0 {
                        let value = super::xp::split_xp(xp);
                        xp -= value;
                        let orb = Self::xp_orb(w, self.pos, value);
                        w.entity_joined_world(orb);
                    }
                }
                self.set_dead(w);
                self.death_particles(w);
            }
        }
        if self.living.recently_hit > 0 {
            self.living.recently_hit -= 1;
        } else {
            self.living.attacking_player = None;
        }
        self.tick_potion_effects(w);
        self.living.prev_walk_phase = self.living.walk_phase;
        self.living.prev_render_yaw_offset = self.living.render_yaw_offset;
        self.prev_yaw = self.yaw;
        self.prev_pitch = self.pitch;
    }
    pub fn explosion_particle(&mut self, w: &mut World) {
        self.emit_explosion_particles(w, 10.0);
    }
    fn death_particles(&mut self, w: &mut World) {
        self.emit_explosion_particles(w, 0.0);
    }
    fn emit_explosion_particles(&mut self, w: &mut World, offset: f64) {
        for _ in 0..20 {
            let v = super::Vec3D::new(
                self.rand.next_gaussian() * 0.02,
                self.rand.next_gaussian() * 0.02,
                self.rand.next_gaussian() * 0.02,
            );
            let p = super::Vec3D::new(
                self.pos.x + (self.rand.next_float() * self.width * 2.0) as f64
                    - self.width as f64
                    - v.x * offset,
                self.pos.y + (self.rand.next_float() * self.height) as f64 - v.y * offset,
                self.pos.z + (self.rand.next_float() * self.width * 2.0) as f64
                    - self.width as f64
                    - v.z * offset,
            );
            w.spawn_particle("explode", p, v);
        }
    }
    pub(super) fn update_living(&mut self, w: &mut World) {
        let was_on_ground = self.on_ground;
        if let EntityKind::Slime {
            squish,
            prev_squish,
            ..
        } = &mut self.kind
        {
            *prev_squish = *squish;
        }
        self.living_entity_update(w);
        if self.living.arrows_stuck > 0 {
            if self.living.arrow_timer <= 0 {
                self.living.arrow_timer = 60;
            }
            self.living.arrow_timer -= 1;
            if self.living.arrow_timer <= 0 {
                self.living.arrows_stuck -= 1;
            }
        }

        self.living_update(w);
        let x = self.pos.x - self.prev_pos.x;
        let z = self.pos.z - self.prev_pos.z;
        let distance = (x * x + z * z).sqrt() as f32;
        let mut body = self.living.render_yaw_offset;
        let mut phase = 0.0;
        let mut amount = 0.0;
        self.living.prev_walk_animation = self.living.walk_animation;
        if distance > 0.05 {
            amount = 1.0;
            phase = distance * 3.0;
            body = crate::jmath::java_atan2(z, x) as f32 * 180.0 / math::PI_F32 - 90.0;
        }
        if self.living.swing_progress > 0.0 {
            body = self.yaw;
        }
        if !self.on_ground {
            amount = 0.0;
        }
        self.living.walk_animation += (amount - self.living.walk_animation) * 0.3;
        let diff = wrap(body - self.living.render_yaw_offset);
        self.living.render_yaw_offset += diff * 0.3;
        let mut diff = wrap(self.yaw - self.living.render_yaw_offset);
        let reverse = diff < -90.0 || diff >= 90.0;
        if diff < -75.0 {
            diff = -75.0;
        }
        if diff >= 75.0 {
            diff = 75.0;
        }
        self.living.render_yaw_offset = self.yaw - diff;
        if diff * diff > 2500.0 {
            self.living.render_yaw_offset += diff * 0.2;
        }
        if reverse {
            phase *= -1.0;
        }
        while self.yaw - self.prev_yaw < -180.0 {
            self.prev_yaw -= 360.0;
        }
        while self.yaw - self.prev_yaw >= 180.0 {
            self.prev_yaw += 360.0;
        }
        while self.living.render_yaw_offset - self.living.prev_render_yaw_offset < -180.0 {
            self.living.prev_render_yaw_offset -= 360.0;
        }
        while self.living.render_yaw_offset - self.living.prev_render_yaw_offset >= 180.0 {
            self.living.prev_render_yaw_offset += 360.0;
        }
        while self.pitch - self.prev_pitch < -180.0 {
            self.prev_pitch -= 360.0;
        }
        while self.pitch - self.prev_pitch >= 180.0 {
            self.prev_pitch += 360.0;
        }
        self.living.walk_phase += phase;
        self.slime_after_update(w, was_on_ground);
    }
    fn living_update(&mut self, w: &mut World) {
        if matches!(self.kind, EntityKind::Player(..)) {
            self.player_before_living(w);
        }
        self.enderman_before_living(w);
        self.mob_before_living(w);
        if self.living.health <= 0 || self.player_state().is_some_and(|s| s.sleeping) {
            self.living.is_jumping = false;
            self.living.move_strafing = 0.0;
            self.living.move_forward = 0.0;
            self.living.random_yaw_velocity = 0.0;
        } else {
            match self.kind {
                EntityKind::Player(..) => self.player_action(),
                EntityKind::Ghast(..) => self.ghast_action(w),
                EntityKind::Squid(..) => self.squid_action_state(w),
                EntityKind::Slime { .. } => self.slime_action_state(w),
                _ => {
                    self.creature_action_state(w);
                    self.silverfish_after_action(w);
                }
            }
        }
        if self.living.health > 0 && matches!(self.kind, EntityKind::Wolf(..)) {
            self.wolf_after_action(w);
        }
        let water = self.is_in_water_query(w);
        let lava = self.handle_lava_movement(w);
        if self.living.is_jumping {
            if water || lava {
                self.motion.y += 0.04f32 as f64;
            } else if self.on_ground {
                self.motion.y = 0.42f32 as f64;
                if self.sprinting {
                    let angle = self.yaw * (math::PI_F32 / 180.0);
                    self.motion.x -= (math::sin(angle) * 0.2) as f64;
                    self.motion.z += (math::cos(angle) * 0.2) as f64;
                }
                self.velocity_changed = true;
                self.add_stat(crate::entity::player::stat::JUMP, 1);
                if self.player_state().is_some() {
                    self.add_exhaustion(if self.sprinting { 0.8 } else { 0.2 });
                }
            }
        }
        self.living.move_strafing *= 0.98;
        self.living.move_forward *= 0.98;
        self.living.random_yaw_velocity *= 0.9;
        let acceleration = self.living.ground_acceleration;
        self.living.ground_acceleration *= self.potion_speed_multiplier();
        self.move_entity_with_heading(w, self.living.move_strafing, self.living.move_forward);
        self.living.ground_acceleration = acceleration;
        for id in w.entities_within_aabb(
            Some(self.entity_id),
            self.bounding_box.expand(0.2f32 as f64, 0.0, 0.2f32 as f64),
        ) {
            if let Some(mut other) = w.entities.remove(&id) {
                if other.can_be_pushed() {
                    other.collide_entity(w, self);
                }
                w.entities.insert(id, other);
            }
        }
        if matches!(self.kind, EntityKind::Squid(..)) {
            self.squid_living_update(w);
        }
        self.wolf_living(w);
        if matches!(self.kind, EntityKind::Player(..)) {
            self.player_after_living(w);
        }
        if let EntityKind::Chicken {
            flap,
            prev_flap,
            flap_amount,
            prev_flap_amount,
            flap_speed,
            egg_timer,
        } = &mut self.kind
        {
            *prev_flap = *flap;
            *prev_flap_amount = *flap_amount;
            *flap_amount =
                (*flap_amount as f64 + (if self.on_ground { -1 } else { 4 }) as f64 * 0.3) as f32;
            *flap_amount = flap_amount.clamp(0.0, 1.0);
            if !self.on_ground && *flap_speed < 1.0 {
                *flap_speed = 1.0;
            }
            *flap_speed = (*flap_speed as f64 * 0.9) as f32;
            if !self.on_ground && self.motion.y < 0.0 {
                self.motion.y *= 0.6;
            }
            *flap += *flap_speed * 2.0;
            *egg_timer -= 1;
            if *egg_timer <= 0 {
                let pitch = (self.rand.next_float() - self.rand.next_float()) * 0.2 + 1.0;
                w.play_sound_at_entity(self, "mob.chickenplop", 1.0, pitch);
                self.drop_item(w, 344, 1, 0, 0.0);
                let next = self.rand.next_int(6000) + 6000;
                if let EntityKind::Chicken { egg_timer, .. } = &mut self.kind {
                    *egg_timer = next;
                }
            }
        }
    }
    pub fn apply_entity_collision(&mut self, other: &mut Entity) {
        if self.ridden_by == Some(other.entity_id) || self.riding == Some(other.entity_id) {
            return;
        }
        let mut x = other.pos.x - self.pos.x;
        let mut z = other.pos.z - self.pos.z;
        let mut largest = if x < 0.0 { -x } else { x };
        let az = if z < 0.0 { -z } else { z };
        if az > largest {
            largest = az;
        }
        if largest >= 0.01f32 as f64 {
            largest = largest.sqrt() as f32 as f64;
            x /= largest;
            z /= largest;
            let inverse = (1.0 / largest).min(1.0);
            x *= inverse;
            z *= inverse;
            x *= 0.05f32 as f64;
            z *= 0.05f32 as f64;
            x *= (1.0 - self.collision_reduction) as f64;
            z *= (1.0 - self.collision_reduction) as f64;
            self.motion.x += -x;
            self.motion.y += 0.0;
            self.motion.z += -z;
            other.motion.x += x;
            other.motion.y += 0.0;
            other.motion.z += z;
        }
    }
    pub fn move_entity_with_heading(&mut self, w: &mut World, strafe: f32, forward: f32) {
        if self.player_state().is_some() {
            let before = self.pos;
            // EntityPlayer.moveEntityWithHeading: flying uses jumpMovementFactor 0.05 and keeps
            // 60% of the vertical motion.
            let flying = self.player_state().unwrap().capabilities.flying;
            let (vertical, air) = (self.motion.y, self.living.air_acceleration);
            if flying {
                self.living.air_acceleration = 0.05;
            }
            self.living_move_entity_with_heading(w, strafe, forward);
            if flying {
                self.motion.y = vertical * 0.6;
                self.living.air_acceleration = air;
            }
            let d = (
                self.pos.x - before.x,
                self.pos.y - before.y,
                self.pos.z - before.z,
            );
            self.add_movement_stat(w, d.0, d.1, d.2);
        } else {
            self.living_move_entity_with_heading(w, strafe, forward);
        }
    }
    fn living_move_entity_with_heading(&mut self, w: &mut World, strafe: f32, forward: f32) {
        if matches!(self.kind, EntityKind::Ghast(..)) {
            self.flying_heading(w, strafe, forward);
            return;
        }
        if matches!(self.kind, EntityKind::Squid(..)) {
            self.move_entity(w, self.motion.x, self.motion.y, self.motion.z);
            return;
        }
        if self.in_water || self.handle_lava_movement(w) {
            let water = self.in_water;
            let old_y = self.pos.y;
            self.move_flying(strafe, forward, 0.02);
            self.move_entity(w, self.motion.x, self.motion.y, self.motion.z);
            let drag = if water { 0.8f32 as f64 } else { 0.5 };
            self.motion.x *= drag;
            self.motion.y *= drag;
            self.motion.z *= drag;
            self.motion.y -= 0.02;
            if self.collided_horizontally {
                let b = self.bounding_box.translated(
                    self.motion.x,
                    self.motion.y + 0.6f32 as f64 - self.pos.y + old_y,
                    self.motion.z,
                );
                if w.get_colliding_bounding_boxes(Some(self.entity_id), b)
                    .is_empty()
                    && !w.get_is_any_liquid(b)
                {
                    self.motion.y = 0.3f32 as f64;
                }
            }
        } else {
            let friction = self.ground_friction(w);
            let accel = 0.16277136 / (friction * friction * friction);
            self.move_flying(
                strafe,
                forward,
                if self.on_ground {
                    self.living.ground_acceleration * accel
                } else {
                    self.living.air_acceleration
                },
            );
            let friction = self.ground_friction(w);
            if self.is_on_ladder(w) {
                let speed = 0.15f32 as f64;
                self.motion.x = self.motion.x.clamp(-speed, speed);
                self.motion.z = self.motion.z.clamp(-speed, speed);
                self.fall_distance = 0.0;
                if self.motion.y < -0.15 {
                    self.motion.y = -0.15;
                }
                if self.sneaking && self.motion.y < 0.0 {
                    self.motion.y = 0.0;
                }
            }
            self.move_entity(w, self.motion.x, self.motion.y, self.motion.z);
            if self.collided_horizontally && self.is_on_ladder(w) {
                self.motion.y = 0.2;
            }
            self.motion.y -= 0.08;
            self.motion.y *= 0.98f32 as f64;
            self.motion.x *= friction as f64;
            self.motion.z *= friction as f64;
        }
        self.living.prev_limb_amount = self.living.limb_amount;
        let x = self.pos.x - self.prev_pos.x;
        let z = self.pos.z - self.prev_pos.z;
        let amount = ((x * x + z * z).sqrt() as f32 * 4.0).min(1.0);
        self.living.limb_amount += (amount - self.living.limb_amount) * 0.4;
        self.living.limb_swing += self.living.limb_amount;
    }
    pub(super) fn ground_friction(&self, w: &mut World) -> f32 {
        if !self.on_ground {
            return 0.91;
        }
        let id = w.get_block_id(
            floor(self.pos.x),
            floor(self.bounding_box.min_y) - 1,
            floor(self.pos.z),
        );
        if id > 0 {
            block_table::SLIPPERINESS[id as usize] * 0.91
        } else {
            546.0f32 * 0.1f32 * 0.1f32 * 0.1f32
        }
    }
    pub fn is_on_ladder(&self, w: &mut World) -> bool {
        if matches!(self.kind, EntityKind::Spider | EntityKind::CaveSpider) {
            return self.collided_horizontally;
        }
        w.get_block_id(
            floor(self.pos.x),
            floor(self.bounding_box.min_y),
            floor(self.pos.z),
        ) == LADDER
    }
    pub(super) fn living_attack(
        &mut self,
        w: &mut World,
        source: super::damage::DamageSource,
        damage: i32,
    ) -> bool {
        let attacker = source.attacker;
        self.living.entity_age = 0;
        if self.living.health <= 0 {
            return false;
        }
        self.living.limb_amount = 1.5;
        let fresh;
        if self.living.hearts_life as f32 > self.living.hearts_halves_life as f32 / 2.0 {
            if damage <= self.living.last_damage {
                return false;
            }
            self.damage_living_from(source, damage - self.living.last_damage);
            self.living.last_damage = damage;
            fresh = false;
        } else {
            self.living.last_damage = damage;
            self.living.prev_health = self.living.health;
            self.living.hearts_life = self.living.hearts_halves_life;
            self.damage_living_from(source, damage);
            self.living.hurt_time = 10;
            self.living.max_hurt_time = 10;
            fresh = true;
        }
        self.living.attacked_at_yaw = 0.0;
        if let Some(id) = attacker {
            if id < 0
                || w.entities
                    .get(&id)
                    .is_some_and(|e| e.player_state().is_some())
            {
                self.living.recently_hit = 60;
                self.living.attacking_player = Some(id);
            } else if w
                .entities
                .get(&id)
                .is_some_and(|e| matches!(&e.kind, EntityKind::Wolf(s) if s.flags & 4 != 0))
            {
                self.living.recently_hit = 60;
                self.living.attacking_player = None;
            }
        }
        if fresh {
            self.been_attacked = true;
            if let Some(pos) = attacker.and_then(|id| {
                if id < 0 {
                    w.players
                        .get((-1 - id) as usize)
                        .map(|&(x, y, z)| super::Vec3D::new(x, y, z))
                } else {
                    w.entities.get(&id).map(|e| e.pos)
                }
            }) {
                let mut x = pos.x - self.pos.x;
                let mut z = pos.z - self.pos.z;
                while x * x + z * z < 1.0e-4 {
                    x = (w.math_random.next_double() - w.math_random.next_double()) * 0.01;
                    z = (w.math_random.next_double() - w.math_random.next_double()) * 0.01;
                }
                self.living.attacked_at_yaw = (crate::jmath::java_atan2(z, x) * 180.0
                    / math::PI_F32 as f64) as f32
                    - self.yaw;
                let length = (x * x + z * z).sqrt() as f32;
                self.velocity_changed = true;
                self.motion.x /= 2.0;
                self.motion.y /= 2.0;
                self.motion.z /= 2.0;
                self.motion.x -= x / length as f64 * 0.4f32 as f64;
                self.motion.y += 0.4f32 as f64;
                self.motion.z -= z / length as f64 * 0.4f32 as f64;
                self.motion.y = self.motion.y.min(0.4f32 as f64);
            } else {
                self.living.attacked_at_yaw =
                    (w.math_random.next_double() * 2.0) as i32 as f32 * 180.0;
            }
            let pitch = (self.rand.next_float() - self.rand.next_float()) * 0.2 + 1.0;
            if matches!(self.kind, EntityKind::Squid(..)) {
                w.play_null_sound_at_entity(self, self.sound_volume(), pitch);
            } else {
                w.play_sound_at_entity(
                    self,
                    self.damage_sound(self.living.health <= 0),
                    self.sound_volume(),
                    pitch,
                );
            }
        }
        if self.living.health <= 0 {
            self.living.dying = true;
            if let Some(mut a) = attacker
                .filter(|&id| id >= 0)
                .and_then(|id| w.entities.remove(&id))
            {
                if let Some(s) = a.player_state_mut() {
                    // addToPlayerScore (scoreValue is 0 for every mob in Beta)
                    s.score += 0;
                    let kills = if self.player_state().is_some() {
                        crate::entity::player::stat::PLAYER_KILLS
                    } else {
                        crate::entity::player::stat::MOB_KILLS
                    };
                    a.add_stat(kills, 1);
                    // onKillEntity
                    if self.kind.is_mob() {
                        a.add_stat(crate::entity::player::stat::KILL_ENEMY, 1);
                    }
                }
                w.entities.insert(a.entity_id, a);
            }
            self.drop_few_items(w);
            if matches!(self.kind, EntityKind::Player(..)) {
                self.player_death(w, attacker);
            }
            if matches!(self.kind, EntityKind::Creeper { .. })
                && attacker
                    .and_then(|id| w.entities.get(&id))
                    .is_some_and(|e| matches!(e.kind, EntityKind::Skeleton))
            {
                let item = 2256 + self.rand.next_int(2);
                self.drop_item(w, item, 1, 0, 0.0);
            }
            // Vanilla's Skeleton sniper achievement is unreachable in this build:
            // DamageSource.func_35526_e delegates to func_35532_a, so both
            // accessors return the owner. One entity cannot be Arrow and Player.
        }
        true
    }
    pub fn drop_few_items(&mut self, w: &mut World) {
        if matches!(self.kind, EntityKind::Squid(..)) {
            let n = self.rand.next_int(3) + 1;
            for _ in 0..n {
                self.drop_item(w, 351, 1, 0, 0.0);
            }
            return;
        }
        if let EntityKind::Sheep {
            sheared,
            fleece_color,
        } = self.kind
        {
            if !sheared {
                self.drop_item(w, 35, 1, fleece_color as i32, 0.0);
            }
            return;
        }
        if matches!(self.kind, EntityKind::Skeleton) {
            for id in [262, 352] {
                let n = self.rand.next_int(3);
                for _ in 0..n {
                    self.drop_item(w, id, 1, 0, 0.0);
                }
            }
            return;
        }
        if matches!(self.kind, EntityKind::Cow) {
            let n = self.rand.next_int(3);
            for _ in 0..n {
                self.drop_item(w, 334, 1, 0, 0.0);
            }
            let n = self.rand.next_int(3) + 1;
            for _ in 0..n {
                self.drop_item(w, if self.fire > 0 { 364 } else { 363 }, 1, 0, 0.0);
            }
            return;
        }
        if matches!(self.kind, EntityKind::Chicken { .. }) {
            let n = self.rand.next_int(3);
            for _ in 0..n {
                self.drop_item(w, 288, 1, 0, 0.0);
            }
            self.drop_item(w, if self.fire > 0 { 366 } else { 365 }, 1, 0, 0.0);
            return;
        }
        let id = match self.kind {
            EntityKind::Pig { .. } => {
                if self.fire > 0 {
                    320
                } else {
                    319
                }
            }
            EntityKind::Cow => 334,
            EntityKind::Chicken { .. } => 288,
            EntityKind::Zombie => 367,
            EntityKind::Spider | EntityKind::CaveSpider => 287,
            EntityKind::Enderman(..) => 368,
            EntityKind::Creeper { .. } | EntityKind::Ghast(..) => 289,
            EntityKind::PigZombie { .. } => 320,
            EntityKind::Slime { size: 1, .. } => 341,
            _ => 0,
        };
        if id > 0 {
            let n = self
                .rand
                .next_int(if matches!(self.kind, EntityKind::Enderman(..)) {
                    2
                } else {
                    3
                });
            for _ in 0..n {
                self.drop_item(w, id, 1, 0, 0.0);
            }
        }
    }
}
pub fn wrap(mut angle: f32) -> f32 {
    while angle < -180.0 {
        angle += 360.0;
    }
    while angle >= 180.0 {
        angle -= 360.0;
    }
    angle
}
