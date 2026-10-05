//! Singleplayer Beta 1.8.1 entity simulation. Arithmetic follows the client source.
pub mod aquatic;
pub mod block_hooks;
pub mod block_table;
pub mod blocks;
pub mod client_tick;
pub mod creature;
pub mod damage;
pub mod explosion;
pub mod fishing;
pub mod geometry;
pub mod hostile;
pub mod living;
pub mod mobs181;
pub mod nbt;
pub mod objects;
pub mod painting;
pub mod path;
pub mod physics;
pub mod player;
pub mod potion;
pub mod projectile;
mod sounds;
pub mod spawning;
pub mod special;
pub mod vehicle_verify;
pub mod vehicles;
pub mod verify;
pub mod verify181;
pub mod world;
pub mod xp;
use crate::world::tile::ItemStack;
use crate::{JavaRandom, World};

/// Seed source is independent of World.rand and Math.random, as in the JVM.
pub enum EntitySeedSource {
    Deterministic(i64),
    Time,
    Custom(Box<dyn FnMut() -> i64>),
}
impl EntitySeedSource {
    pub fn next_seed(&mut self) -> i64 {
        match self {
            Self::Deterministic(n) => {
                let s = *n;
                *n = n.wrapping_add(1);
                s
            }
            Self::Time => {
                use std::sync::atomic::{AtomicI64, Ordering};
                static UNIQUE: AtomicI64 = AtomicI64::new(8682522807148012);
                let mut old = UNIQUE.load(Ordering::Relaxed);
                let n = loop {
                    let n = old.wrapping_mul(181783497276652981);
                    match UNIQUE.compare_exchange_weak(old, n, Ordering::Relaxed, Ordering::Relaxed)
                    {
                        Ok(_) => break n,
                        Err(v) => old = v,
                    }
                };
                n ^ std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos() as i64
            }
            Self::Custom(f) => f(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum EntityKind {
    Fish(fishing::FishState),
    Painting(painting::PaintingState),
    Minecart(vehicles::MinecartState),
    Boat(vehicles::BoatState),
    Player(Box<player::PlayerState>),
    Item {
        item: ItemStack,
        age: i32,
        pickup_delay: i32,
        health: i32,
        hover_start: f32,
        updates: i32,
    },
    FallingSand {
        block: u8,
        fall_time: i32,
    },
    Tnt {
        fuse: i32,
    },
    Arrow(projectile::ArrowState),
    Snowball(projectile::ArrowState),
    Egg(projectile::ArrowState),
    Fireball(projectile::FireballState),
    Lightning {
        state: i32,
        seed: i64,
        remaining: i32,
    },
    Ghast(special::GhastState),
    Wolf(special::WolfState),
    Zombie,
    Skeleton,
    Spider,
    CaveSpider,
    Enderman(mobs181::EndermanState),
    Silverfish {
        ally_timer: i32,
    },
    XPOrb(xp::XPOrbState),
    Creeper {
        ignition: i32,
        prev_ignition: i32,
        state: i32,
        powered: bool,
    },
    PigZombie {
        anger: i32,
        sound_delay: i32,
    },
    Giant,
    Pig {
        saddled: bool,
    },
    Sheep {
        sheared: bool,
        fleece_color: u8,
    },
    Cow,
    Chicken {
        flap: f32,
        prev_flap: f32,
        flap_amount: f32,
        prev_flap_amount: f32,
        flap_speed: f32,
        egg_timer: i32,
    },
    Squid(aquatic::SquidState),
    Slime {
        size: i32,
        squish: f32,
        prev_squish: f32,
        jump_delay: i32,
    },
}
impl EntityKind {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Fish(..) => "FishingHook",
            Self::Painting(..) => "Painting",
            Self::Minecart(..) => "Minecart",
            Self::Boat(..) => "Boat",
            Self::Player(..) => "Player",
            Self::Ghast(..) => "Ghast",
            Self::Wolf(..) => "Wolf",
            Self::Tnt { .. } => "PrimedTnt",
            Self::Arrow(..) => "Arrow",
            Self::Snowball(..) => "Snowball",
            Self::Egg(..) => "ThrownEgg",
            Self::Fireball(..) => "Fireball",
            Self::Lightning { .. } => "LightningBolt",
            Self::Zombie => "Zombie",
            Self::Skeleton => "Skeleton",
            Self::Spider => "Spider",
            Self::CaveSpider => "CaveSpider",
            Self::Enderman(..) => "Enderman",
            Self::Silverfish { .. } => "Silverfish",
            Self::XPOrb(..) => "XPOrb",
            Self::Creeper { .. } => "Creeper",
            Self::PigZombie { .. } => "PigZombie",
            Self::Giant => "Giant",
            Self::Item { .. } => "Item",
            Self::FallingSand { .. } => "FallingSand",
            Self::Pig { .. } => "Pig",
            Self::Sheep { .. } => "Sheep",
            Self::Cow => "Cow",
            Self::Chicken { .. } => "Chicken",
            Self::Squid(..) => "Squid",
            Self::Slime { .. } => "Slime",
        }
    }
    pub fn is_mob(&self) -> bool {
        matches!(
            self,
            Self::Zombie
                | Self::Skeleton
                | Self::Spider
                | Self::CaveSpider
                | Self::Enderman(..)
                | Self::Silverfish { .. }
                | Self::Creeper { .. }
                | Self::PigZombie { .. }
                | Self::Giant
        )
    }
    pub fn is_living(&self) -> bool {
        !matches!(
            self,
            Self::Fish(..)
                | Self::Painting(..)
                | Self::Minecart(..)
                | Self::Boat(..)
                | Self::XPOrb(..)
                | Self::Item { .. }
                | Self::FallingSand { .. }
                | Self::Tnt { .. }
                | Self::Arrow(..)
                | Self::Snowball(..)
                | Self::Egg(..)
                | Self::Fireball(..)
                | Self::Lightning { .. }
        )
    }
}

#[derive(Clone, Debug)]
pub struct Entity {
    pub living: living::LivingState,
    pub entity_id: i32,
    pub kind: EntityKind,
    pub rand: JavaRandom,
    pub pos: Vec3D,
    pub prev_pos: Vec3D,
    pub last_tick_pos: Vec3D,
    pub motion: Vec3D,
    pub yaw: f32,
    pub pitch: f32,
    pub prev_yaw: f32,
    pub prev_pitch: f32,
    pub bounding_box: AxisAlignedBB,
    pub width: f32,
    pub height: f32,
    pub y_offset: f32,
    pub y_size: f32,
    pub step_height: f32,
    pub on_ground: bool,
    pub collided_horizontally: bool,
    pub collided_vertically: bool,
    pub collided: bool,
    pub been_attacked: bool,
    pub in_web: bool,
    pub slide_on_collision: bool,
    pub dead: bool,
    pub no_clip: bool,
    pub sneaking: bool,
    pub sprinting: bool,
    pub velocity_changed: bool,
    pub prevent_entity_spawning: bool,
    pub fall_distance: f32,
    pub distance_walked: f32,
    pub prev_distance_walked: f32,
    pub next_step_distance: i32,
    pub ticks_existed: i32,
    pub fire_resistance: i32,
    pub fire: i32,
    pub max_air: i32,
    pub air: i32,
    pub in_water: bool,
    pub first_update: bool,
    pub immune_to_fire: bool,
    pub added_to_chunk: bool,
    pub chunk_x: i32,
    pub chunk_y: i32,
    pub chunk_z: i32,
    pub riding: Option<i32>,
    pub ridden_by: Option<i32>,
    pub collision_reduction: f32,
    pub rider_yaw_delta: f64,
    pub rider_pitch_delta: f64,
}
impl Entity {
    /// Vanilla interpolation used by living entity renderers.
    pub fn swing_progress(&self, partial: f32) -> f32 {
        let mut delta = self.living.swing_progress - self.living.prev_swing_progress;
        if delta < 0.0 {
            delta += 1.0;
        }
        self.living.prev_swing_progress + delta * partial
    }
    pub fn creeper_flash(&self, partial: f32) -> f32 {
        if let EntityKind::Creeper {
            ignition,
            prev_ignition,
            ..
        } = self.kind
        {
            (prev_ignition as f32 + (ignition - prev_ignition) as f32 * partial) / 28.0
        } else {
            0.0
        }
    }
    pub fn wolf_interested_angle(&self, partial: f32) -> f32 {
        if let EntityKind::Wolf(s) = &self.kind {
            (s.prev_interest + (s.interest - s.prev_interest) * partial)
                * 0.15
                * crate::math::PI_F32
        } else {
            0.0
        }
    }
    pub fn wolf_shake_angle(&self, partial: f32, offset: f32) -> f32 {
        if let EntityKind::Wolf(s) = &self.kind {
            let phase =
                ((s.prev_shake_time + (s.shake_time - s.prev_shake_time) * partial + offset) / 1.8)
                    .clamp(0.0, 1.0);
            crate::math::sin(phase * crate::math::PI_F32)
                * crate::math::sin(phase * crate::math::PI_F32 * 11.0)
                * 0.15
                * crate::math::PI_F32
        } else {
            0.0
        }
    }
    pub fn wolf_shading_while_shaking(&self, partial: f32) -> f32 {
        if let EntityKind::Wolf(s) = &self.kind {
            0.75 + (s.prev_shake_time + (s.shake_time - s.prev_shake_time) * partial) / 2.0 * 0.25
        } else {
            1.0
        }
    }
    fn base(w: &mut World, kind: EntityKind) -> Self {
        let id = w.next_entity_id;
        w.next_entity_id = id.wrapping_add(1);
        let mut e = Self {
            living: Default::default(),
            entity_id: id,
            kind,
            rand: JavaRandom::new(w.entity_seed_source.next_seed()),
            pos: Vec3D::default(),
            prev_pos: Vec3D::default(),
            last_tick_pos: Vec3D::default(),
            motion: Vec3D::default(),
            yaw: 0.0,
            pitch: 0.0,
            prev_yaw: 0.0,
            prev_pitch: 0.0,
            bounding_box: AxisAlignedBB::default(),
            width: 0.6,
            height: 1.8,
            y_offset: 0.0,
            y_size: 0.0,
            step_height: 0.0,
            on_ground: false,
            collided_horizontally: false,
            collided_vertically: false,
            collided: false,
            been_attacked: false,
            in_web: false,
            slide_on_collision: true,
            dead: false,
            no_clip: false,
            sneaking: false,
            sprinting: false,
            velocity_changed: false,
            prevent_entity_spawning: false,
            fall_distance: 0.0,
            distance_walked: 0.0,
            prev_distance_walked: 0.0,
            next_step_distance: 1,
            ticks_existed: 0,
            fire_resistance: 1,
            fire: 0,
            max_air: 300,
            air: 300,
            in_water: false,
            first_update: true,
            immune_to_fire: false,
            added_to_chunk: false,
            chunk_x: 0,
            chunk_y: 0,
            chunk_z: 0,
            riding: None,
            ridden_by: None,
            collision_reduction: 0.0,
            rider_yaw_delta: 0.0,
            rider_pitch_delta: 0.0,
        };
        e.set_position(0.0, 0.0, 0.0);
        e
    }
    pub fn item(w: &mut World, x: f64, y: f64, z: f64, item: ItemStack) -> Self {
        // The hover initializer runs after Entity's constructor and before the body.
        let mut e = Self::base(
            w,
            EntityKind::Item {
                item,
                age: 0,
                pickup_delay: 0,
                health: 5,
                hover_start: 0.0,
                updates: 0,
            },
        );
        let hover = (w.math_random.next_double() * std::f64::consts::PI * 2.0) as f32;
        if let EntityKind::Item { hover_start, .. } = &mut e.kind {
            *hover_start = hover;
        }
        e.width = 0.25;
        e.height = 0.25;
        e.y_offset = e.height / 2.0;
        e.set_position(x, y, z);
        e.yaw = (w.math_random.next_double() * 360.0) as f32;
        e.motion.x = (w.math_random.next_double() * 0.2f32 as f64 - 0.1f32 as f64) as f32 as f64;
        e.motion.y = 0.2f32 as f64;
        e.motion.z = (w.math_random.next_double() * 0.2f32 as f64 - 0.1f32 as f64) as f32 as f64;
        e
    }
    pub fn falling_sand(w: &mut World, x: f64, y: f64, z: f64, block: u8) -> Self {
        let mut e = Self::base(
            w,
            EntityKind::FallingSand {
                block,
                fall_time: 0,
            },
        );
        e.prevent_entity_spawning = true;
        e.width = 0.98;
        e.height = 0.98;
        e.y_offset = e.height / 2.0;
        e.set_position(x, y, z);
        e.prev_pos = e.pos;
        e
    }
    pub fn set_position(&mut self, x: f64, y: f64, z: f64) {
        self.pos = Vec3D::new(x, y, z);
        let r = (self.width / 2.0) as f64;
        let bottom = y - self.y_offset as f64 + self.y_size as f64;
        self.bounding_box = AxisAlignedBB::new(
            x - r,
            bottom,
            z - r,
            x + r,
            bottom + self.height as f64,
            z + r,
        );
    }
    pub fn set_location_and_angles(&mut self, x: f64, y: f64, z: f64, yaw: f32, pitch: f32) {
        self.set_position(x, y + self.y_offset as f64, z);
        self.prev_pos = self.pos;
        self.last_tick_pos = self.pos;
        self.yaw = yaw;
        self.pitch = pitch;
    }
    pub fn attack(&mut self, w: &mut World, attacker: Option<i32>, damage: i32) -> bool {
        let kind = if attacker.is_some() {
            damage::DamageKind::Mob
        } else {
            damage::DamageKind::Generic
        };
        self.attack_from(w, damage::DamageSource::entity(kind, attacker), damage)
    }
    pub fn attack_from(
        &mut self,
        w: &mut World,
        source: damage::DamageSource,
        mut damage: i32,
    ) -> bool {
        let attacker = source.attacker;
        if matches!(self.kind, EntityKind::Silverfish { .. }) && source.is_entity() {
            if let EntityKind::Silverfish { ally_timer } = &mut self.kind {
                if *ally_timer <= 0 {
                    *ally_timer = 20;
                }
            }
        }
        if matches!(
            self.kind,
            EntityKind::Pig { .. }
                | EntityKind::Sheep { .. }
                | EntityKind::Cow
                | EntityKind::Chicken { .. }
                | EntityKind::Wolf(..)
        ) {
            self.living.flee_ticks = 60;
        }

        if matches!(self.kind, EntityKind::Painting(_)) {
            if !self.dead {
                self.break_painting(w, true);
            }
            return true;
        }
        if matches!(self.kind, EntityKind::Minecart(..) | EntityKind::Boat(..)) {
            return self.attack_vehicle(w, damage);
        }
        if matches!(self.kind, EntityKind::Player(..))
            && !self.player_damage(w, source, &mut damage)
        {
            return false;
        }
        self.prepare_special_attack(w, attacker, &mut damage);
        if self.kind.is_living() {
            let hit = self.living_attack(w, source, damage);
            if hit
                && self.kind.is_mob()
                && attacker.is_some()
                && attacker != Some(self.entity_id)
                && attacker != self.riding
                && attacker != self.ridden_by
            {
                self.living.target = attacker.map(|id| {
                    if id < 0 {
                        living::Target::Player((-1 - id) as usize)
                    } else {
                        living::Target::Entity(id)
                    }
                });
            }
            self.after_special_attack(w, attacker, hit);
            return hit;
        }
        if matches!(self.kind, EntityKind::Fireball(..)) {
            return self.fireball_attack(w, attacker);
        }
        self.been_attacked = true;
        if let EntityKind::XPOrb(s) = &mut self.kind {
            s.health -= damage;
            if s.health <= 0 {
                self.dead = true;
            }
        }
        if let EntityKind::Item { health, .. } = &mut self.kind {
            self.been_attacked = true;
            *health -= damage;
            if *health <= 0 {
                self.dead = true;
            }
        }
        false
    }
    pub fn drop_item(
        &mut self,
        w: &mut World,
        id: i32,
        count: i32,
        damage: i32,
        offset: f32,
    ) -> i32 {
        let mut e = Self::item(
            w,
            self.pos.x,
            self.pos.y + offset as f64,
            self.pos.z,
            ItemStack::new(id, count, damage),
        );
        if let EntityKind::Item { pickup_delay, .. } = &mut e.kind {
            *pickup_delay = 10;
        }
        let id = e.entity_id;
        w.entity_joined_world(e);
        id
    }
    pub fn on_update(&mut self, w: &mut World) {
        match self.kind {
            EntityKind::Fish(..) => self.update_fish(w),
            EntityKind::Painting(..) => self.update_painting(w),
            EntityKind::Minecart(..) => self.update_minecart(w),
            EntityKind::Boat(..) => self.update_boat(w),
            EntityKind::Player(..) => self.update_player(w),
            EntityKind::Item { .. } => self.update_item(w),
            EntityKind::XPOrb(..) => self.update_xp_orb(w),
            EntityKind::FallingSand { .. } => self.update_falling_sand(w),
            EntityKind::Tnt { .. } => self.update_tnt(w),
            EntityKind::Arrow(..) => self.update_arrow(w),
            EntityKind::Snowball(..) | EntityKind::Egg(..) => self.update_throwable(w),
            EntityKind::Fireball(..) => self.update_fireball(w),
            EntityKind::Lightning { .. } => self.update_lightning(w),
            _ => self.update_hostile_living(w),
        }
    }
}
pub use geometry::{AxisAlignedBB, MovingObjectPosition, Vec3D};
