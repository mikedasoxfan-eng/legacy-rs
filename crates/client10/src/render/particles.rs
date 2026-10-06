//! Literal Beta EntityFX constructors, tick methods and Tessellator quads.
//! Entity randoms, Math.random and EffectRenderer.rand are independent of World.rand.
use super::{
    rprops,
    weather::{Snapshot, WeatherVertex},
};
use worldgen::{math, JavaRandom};
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    Base,
    Smoke,
    LargeSmoke,
    Flame,
    Explode,
    Splash,
    Rain,
    Bubble,
    Portal,
    Note,
    RedDust,
    Heart,
    Lava,
    Snowball,
    Slime,
    SnowShovel,
    Digging { block: u8, metadata: i32 },
    Footstep,
    Pickup,
    Crit,
    MagicCrit,
    Cloud,
    Aura,
    Suspend,
    LargeExplode,
    HugeExplode,
    CritEmitter,
}
impl Kind {
    pub fn named(name: &str) -> Option<Self> {
        Some(match name {
            "smoke" => Self::Smoke,
            "largesmoke" => Self::LargeSmoke,
            "flame" => Self::Flame,
            "explode" => Self::Explode,
            "splash" => Self::Splash,
            "bubble" => Self::Bubble,
            "portal" => Self::Portal,
            "note" => Self::Note,
            "reddust" => Self::RedDust,
            "heart" => Self::Heart,
            "lava" => Self::Lava,
            "snowballpoof" => Self::Snowball,
            "slime" => Self::Slime,
            "snowshovel" => Self::SnowShovel,
            "footstep" => Self::Footstep,
            "crit" => Self::Crit,

            "cloud" => Self::Cloud,
            "townaura" | "depthsuspend" => Self::Aura,
            "suspended" => Self::Suspend,
            "largeexplode" => Self::LargeExplode,
            "hugeexplosion" => Self::HugeExplode,
            _ => return None,
        })
    }
}
#[derive(Clone)]
pub struct Particle {
    pub kind: Kind,
    pub pos: [f64; 3],
    pub prev: [f64; 3],
    pub motion: [f64; 3],
    pub bounds: [f64; 6],
    pub color: [f32; 3],
    pub scale: f32,
    pub original_scale: f32,
    pub gravity: f32,
    pub texture: i32,
    pub jitter: [f32; 2],
    pub age: i32,
    pub max_age: i32,
    pub dead: bool,
    pub on_ground: bool,
    pub no_clip: bool,
    pub in_web: bool,
    pub width: f32,
    pub height: f32,
    pub y_offset: f32,
    pub origin: [f64; 3],
    pub timer: i32,
    pub rand: JavaRandom,
    pub pickup: Option<Box<worldgen::entity::Entity>>,
    pub target: i32,
    pub cloud_player: Option<[f64; 5]>,
    pub duration: i32,
    pub emitted: Vec<(&'static str, [f64; 3], [f64; 3])>,
}
impl Particle {
    /// Entity.func_35115_a and the three particle overrides. The partial tick
    /// affects Flame, but Portal deliberately uses the integer age alone.
    pub fn packed_brightness(&self, terrain: &Snapshot, partial: f32) -> i32 {
        let base = terrain.packed_brightness([self.pos[0],
            self.pos[1] - self.y_offset as f64 + (self.bounds[4] - self.bounds[1]) * 0.66,
            self.pos[2]]);
        self.packed_brightness_from(base, partial)
    }
    pub fn packed_brightness_from(&self, base: i32, partial: f32) -> i32 {
        let block = base & 255;
        let sky = (base >> 16) & 255;
        match self.kind {
            Kind::LargeExplode => 61680,
            Kind::Lava => 240 | sky << 16,
            Kind::Flame => {
                let t = ((self.age as f32 + partial) / self.max_age as f32).clamp(0.0, 1.0);
                (block + (t * 15.0 * 16.0) as i32).min(240) | sky << 16
            }
            Kind::Portal => {
                let mut t = self.age as f32 / self.max_age as f32;
                t *= t;
                t *= t;
                block | (sky + (t * 15.0 * 16.0) as i32).min(240) << 16
            }
            _ => base,
        }
    }
    pub fn new(
        kind: Kind,
        pos: [f64; 3],
        velocity: [f64; 3],
        math_rand: &mut JavaRandom,
        seed: i64,
    ) -> Self {
        Self::new_in(kind, pos, velocity, math_rand, seed, &Snapshot::default())
    }
    pub fn new_in(
        kind: Kind,
        mut pos: [f64; 3],
        velocity: [f64; 3],
        math_rand: &mut JavaRandom,
        seed: i64,
        terrain: &Snapshot,
    ) -> Self {
        if kind == Kind::Suspend {
            pos[1] -= 0.125;
        }
        let m = math_rand;
        let mut r = JavaRandom::new(seed);
        let input = if matches!(
            kind,
            Kind::Crit
                | Kind::MagicCrit
                | Kind::Cloud
                | Kind::Smoke
                | Kind::LargeSmoke
                | Kind::RedDust
                | Kind::Note
                | Kind::Heart
                | Kind::Lava
                | Kind::Rain
                | Kind::Splash
                | Kind::Slime
                | Kind::Snowball
                | Kind::Footstep
                | Kind::LargeExplode
                | Kind::HugeExplode
        ) {
            [0.0; 3]
        } else {
            velocity
        };
        let mut motion =
            std::array::from_fn(|i| input[i] + ((m.next_double() * 2.0 - 1.0) as f32 * 0.4) as f64);
        let speed = (m.next_double() + m.next_double() + 1.0) as f32 * 0.15;
        let length = (motion.iter().map(|v| v * v).sum::<f64>()).sqrt() as f32;
        for i in 0..3 {
            motion[i] = motion[i] / length as f64 * speed as f64 * 0.4f32 as f64;
        }
        motion[1] += 0.1f32 as f64;
        let jitter = [r.next_float() * 3.0, r.next_float() * 3.0];
        let scale = (r.next_float() * 0.5 + 0.5) * 2.0;
        let max_age = (4.0 / (r.next_float() * 0.9 + 0.1)) as i32;
        let half = (0.2f32 / 2.0) as f64;
        let mut p = Self {
            kind,
            pos,
            prev: [0.0; 3],
            motion,
            bounds: [
                pos[0] - half,
                pos[1] - half,
                pos[2] - half,
                pos[0] + half,
                pos[1] - half + 0.2f32 as f64,
                pos[2] + half,
            ],
            color: [1.0; 3],
            scale,
            original_scale: scale,
            gravity: 0.0,
            texture: 0,
            jitter,
            age: 0,
            max_age,
            dead: false,
            on_ground: false,
            no_clip: false,
            in_web: false,
            width: 0.2,
            height: 0.2,
            y_offset: 0.1,
            origin: pos,
            timer: 0,
            rand: r,
            pickup: None,
            target: 0,
            cloud_player: None,
            duration: 0,
            emitted: Vec::new(),
        };
        match kind {
            Kind::LargeExplode => {
                p.duration = 6 + p.rand.next_int(4);
                p.color = [p.rand.next_float() * 0.6 + 0.4; 3];
                p.original_scale = 1.0 - velocity[0] as f32 * 0.5;
            }
            Kind::HugeExplode => p.duration = 8,
            Kind::CritEmitter => p.duration = 3,
            Kind::Crit | Kind::MagicCrit | Kind::Cloud => {
                for i in 0..3 {
                    p.motion[i] *= 0.1f32 as f64;
                    p.motion[i] += velocity[i] * if kind == Kind::Cloud { 1.0 } else { 0.4 };
                }
                p.color = [if kind == Kind::Cloud {
                    1.0 - (m.next_double() * 0.3f32 as f64) as f32
                } else {
                    (m.next_double() * 0.3f32 as f64 + 0.6f32 as f64) as f32
                }; 3];
                p.scale *= 12.0 / 16.0;
                p.scale *= if kind == Kind::Cloud { 2.5 } else { 1.0 };
                p.max_age = if kind == Kind::Cloud {
                    (8.0 / (m.next_double() * 0.8 + 0.3)) as i32
                } else {
                    (6.0 / (m.next_double() * 0.8 + 0.6)) as i32
                };
                if kind == Kind::Cloud {
                    p.max_age = (p.max_age as f32 * 2.5) as i32;
                } else {
                    p.texture = 65;
                }
            }
            Kind::Aura | Kind::Suspend => {
                if kind == Kind::Aura {
                    p.color = [p.rand.next_float() * 0.1 + 0.2; 3];
                    p.width = 0.02;
                    p.height = 0.02;
                    p.scale *= p.rand.next_float() * 0.6 + 0.5;
                    for v in &mut p.motion {
                        *v *= 0.02f32 as f64;
                    }
                    p.max_age = (20.0 / (m.next_double() * 0.8 + 0.2)) as i32;
                    p.no_clip = true;
                } else {
                    p.color = [0.4, 0.4, 0.7];
                    p.width = 0.01;
                    p.height = 0.01;
                    p.scale *= p.rand.next_float() * 0.6 + 0.2;
                    p.motion = velocity.map(|v| v * 0.0);
                    p.max_age = (16.0 / (m.next_double() * 0.8 + 0.2)) as i32;
                }
            }

            Kind::Smoke | Kind::LargeSmoke | Kind::SnowShovel => {
                for i in 0..3 {
                    p.motion[i] *= 0.1f32 as f64;
                    p.motion[i] += velocity[i];
                }
                let gray = (m.next_double() * 0.3f32 as f64) as f32;
                p.color = [if kind == Kind::SnowShovel {
                    1.0 - gray
                } else {
                    gray
                }; 3];
                p.scale *= 12.0 / 16.0;
                let size = if kind == Kind::LargeSmoke { 2.5 } else { 1.0 };
                p.scale *= size;
                p.max_age = (8.0 / (m.next_double() * 0.8 + 0.2)) as i32;
                p.max_age = (p.max_age as f32 * size) as i32;
            }
            Kind::Flame => {
                for i in 0..3 {
                    p.motion[i] = p.motion[i] * 0.01f32 as f64 + velocity[i];
                    for _ in 0..2 {
                        p.rand.next_float();
                    }
                }
                p.max_age = (8.0 / (m.next_double() * 0.8 + 0.2)) as i32 + 4;
                p.no_clip = true;
                p.texture = 48;
            }
            Kind::Explode => {
                for i in 0..3 {
                    p.motion[i] =
                        velocity[i] + ((m.next_double() * 2.0 - 1.0) as f32 * 0.05) as f64;
                }
                p.color = [p.rand.next_float() * 0.3 + 0.7; 3];
                p.scale = p.rand.next_float() * p.rand.next_float() * 6.0 + 1.0;
                p.max_age = (16.0 / (p.rand.next_float() as f64 * 0.8 + 0.2)) as i32 + 2;
            }
            Kind::Bubble => {
                p.texture = 32;
                p.width = 0.02;
                p.height = 0.02;
                p.scale *= p.rand.next_float() * 0.6 + 0.2;
                for i in 0..3 {
                    p.motion[i] = velocity[i] * 0.2f32 as f64
                        + ((m.next_double() * 2.0 - 1.0) as f32 * 0.02) as f64;
                }
                p.max_age = (8.0 / (m.next_double() * 0.8 + 0.2)) as i32;
            }
            Kind::Rain | Kind::Splash => {
                p.motion[0] *= 0.3f32 as f64;
                p.motion[1] = (m.next_double() as f32 * 0.2 + 0.1) as f64;
                p.motion[2] *= 0.3f32 as f64;
                p.texture = 19 + p.rand.next_int(4);
                p.width = 0.01;
                p.height = 0.01;
                p.gravity = 0.06;
                p.max_age = (8.0 / (m.next_double() * 0.8 + 0.2)) as i32;
                if kind == Kind::Splash {
                    p.gravity = 0.04;
                    p.texture += 1;
                    if velocity[1] == 0.0 && (velocity[0] != 0.0 || velocity[2] != 0.0) {
                        p.motion = [velocity[0], velocity[1] + 0.1, velocity[2]];
                    }
                }
            }
            Kind::Portal => {
                p.motion = velocity;
                let bright = p.rand.next_float() * 0.6 + 0.4;
                p.scale = p.rand.next_float() * 0.2 + 0.5;
                p.color = [1.0 * bright; 3];
                p.color[1] *= 0.3;
                p.color[0] *= 0.9;
                p.max_age = (m.next_double() * 10.0) as i32 + 40;
                p.no_clip = true;
                p.texture = (m.next_double() * 8.0) as i32;
            }
            Kind::Note | Kind::Heart => {
                for v in &mut p.motion {
                    *v *= 0.01f32 as f64;
                }
                p.motion[1] += if kind == Kind::Note { 0.2 } else { 0.1 };
                if kind == Kind::Note {
                    for i in 0..3 {
                        let phase = [0.0, 1.0 / 3.0, 2.0 / 3.0][i];
                        p.color[i] = math::sin((velocity[0] as f32 + phase) * math::PI_F32 * 2.0)
                            * 0.65
                            + 0.35;
                    }
                }
                p.scale *= 12.0 / 16.0;
                p.scale *= 2.0;
                p.max_age = if kind == Kind::Note { 6 } else { 16 };
                p.texture = if kind == Kind::Note { 64 } else { 80 };
            }
            Kind::RedDust => {
                for v in &mut p.motion {
                    *v *= 0.1f32 as f64;
                }
                let bright = m.next_double() as f32 * 0.4 + 0.6;
                for i in 0..3 {
                    let mut c = velocity[i] as f32;
                    if i == 0 && c == 0.0 {
                        c = 1.0;
                    }
                    p.color[i] = ((m.next_double() * 0.2f32 as f64) as f32 + 0.8) * c * bright;
                }
                p.scale *= 12.0 / 16.0;
                p.scale *= 1.0;
                p.max_age = (8.0 / (m.next_double() * 0.8 + 0.2)) as i32;
                p.max_age = (p.max_age as f32 * 1.0) as i32;
            }
            Kind::Lava => {
                for v in &mut p.motion {
                    *v *= 0.8f32 as f64;
                }
                p.motion[1] = (p.rand.next_float() * 0.4 + 0.05) as f64;
                p.scale *= p.rand.next_float() * 2.0 + 0.2;
                p.max_age = (16.0 / (m.next_double() * 0.8 + 0.2)) as i32;
                p.texture = 49;
            }
            Kind::Snowball | Kind::Slime => {
                p.texture = if kind == Kind::Snowball { 14 } else { 30 };
                p.gravity = 1.0;
                p.scale /= 2.0;
            }
            Kind::Digging { block, metadata } => {
                p.texture = if block == 2 {
                    2
                } else {
                    rprops(block).textures[(metadata & 15) as usize][0] as i32
                };
                p.gravity = 1.0;
                p.color = [0.6; 3];
                p.scale /= 2.0;
            }
            Kind::Footstep => {
                p.motion = [0.0; 3];
            }
            Kind::Pickup | Kind::Base => {}
        }
        if kind != Kind::LargeExplode {
            p.original_scale = p.scale;
        }
        if matches!(kind, Kind::Crit | Kind::MagicCrit) {
            p.tick(terrain, m);
            if kind == Kind::MagicCrit {
                p.color[0] *= 0.3;
                p.color[1] *= 0.8;
                p.texture += 1;
            }
        }
        p
    }
    pub fn layer(&self) -> usize {
        match self.kind {
            Kind::Digging { .. } | Kind::HugeExplode => 1,
            Kind::Slime | Kind::Snowball => 2,
            Kind::Footstep | Kind::Pickup | Kind::LargeExplode | Kind::CritEmitter => 3,
            _ => 0,
        }
    }
    pub fn multiply_velocity(&mut self, f: f32) {
        self.motion[0] *= f as f64;
        self.motion[1] = (self.motion[1] - 0.1f32 as f64) * f as f64 + 0.1f32 as f64;
        self.motion[2] *= f as f64;
    }
    pub fn multiply_scale(&mut self, f: f32) {
        self.width = 0.2 * f;
        self.height = 0.2 * f;
        self.scale *= f;
        self.original_scale = self.scale;
    }
    pub fn tick(&mut self, terrain: &Snapshot, m: &mut JavaRandom) -> Option<([f64; 3], [f64; 3])> {
        if matches!(self.kind, Kind::LargeExplode | Kind::HugeExplode | Kind::CritEmitter) {
            if self.kind == Kind::LargeExplode {
                self.prev = self.pos;
            } else if self.kind == Kind::HugeExplode {
                for _ in 0..6 {
                    let pos = std::array::from_fn(|i| self.pos[i] + (self.rand.next_double() - self.rand.next_double()) * 4.0);
                    self.emitted.push(("largeexplode", pos, [(self.timer as f32 / self.duration as f32) as f64, 0.0, 0.0]));
                }
            } else if let Some(target) = &self.pickup {
                for _ in 0..16 {
                    let v: [f64; 3] = std::array::from_fn(|_| (self.rand.next_float() * 2.0 - 1.0) as f64);
                    if v.iter().map(|v| v * v).sum::<f64>() <= 1.0 {
                        let pos = [target.pos.x + v[0] * target.width as f64 / 4.0,
                            target.bounding_box.min_y + (target.height / 2.0) as f64 + v[1] * target.height as f64 / 4.0,
                            target.pos.z + v[2] * target.width as f64 / 4.0];
                        self.emitted.push(("crit", pos, [v[0], v[1] + 0.2, v[2]]));
                    }
                }
            }
            self.timer += 1;
            self.dead = if self.kind == Kind::CritEmitter { self.timer >= self.duration } else { self.timer == self.duration };
            return None;
        }
        if self.kind == Kind::Pickup {
            self.timer += 1;
            if self.timer == 3 {
                self.dead = true;
            }
            return None;
        }
        if self.kind == Kind::Footstep {
            self.timer += 1;
            if self.timer == 200 {
                self.dead = true;
            }
            return None;
        }
        self.prev = self.pos;
        let mut smoke = None;
        if self.kind == Kind::Portal {
            let mut t = self.age as f32 / self.max_age as f32;
            let old = t;
            t = -t + t * t * 2.0;
            t = 1.0 - t;
            self.pos = std::array::from_fn(|i| self.origin[i] + self.motion[i] * t as f64);
            self.pos[1] += (1.0 - old) as f64;
            if self.age >= self.max_age {
                self.dead = true;
            }
            self.age += 1;
            return None;
        }
        if matches!(self.kind, Kind::Aura | Kind::Suspend) {
            self.on_ground = move_particle(self, terrain);
            if self.kind == Kind::Aura {
                for v in &mut self.motion {
                    *v *= 0.99;
                }
            } else if material(terrain, self.pos) != worldgen::block::MAT_WATER {
                self.dead = true;
            }
            if self.max_age <= 0 {
                self.dead = true;
            }
            self.max_age -= 1;
            return None;
        }
        if matches!(self.kind, Kind::Crit | Kind::MagicCrit) {
            if self.age >= self.max_age {
                self.dead = true;
            }
            self.age += 1;
            self.on_ground = move_particle(self, terrain);
            self.color[1] = (self.color[1] as f64 * 0.96) as f32;
            self.color[2] = (self.color[2] as f64 * 0.9) as f32;
            for v in &mut self.motion {
                *v *= 0.7f32 as f64;
            }
            self.motion[1] -= 0.02f32 as f64;
            if self.on_ground {
                self.motion[0] *= 0.7f32 as f64;
                self.motion[2] *= 0.7f32 as f64;
            }
            return None;
        }
        if !matches!(self.kind, Kind::Rain | Kind::Splash | Kind::Bubble) {
            if self.age >= self.max_age {
                self.dead = true;
            }
            self.age += 1;
        }
        if matches!(
            self.kind,
            Kind::Cloud
                | Kind::Smoke
                | Kind::LargeSmoke
                | Kind::Explode
                | Kind::RedDust
                | Kind::SnowShovel
        ) {
            self.texture = 7 - self.age * 8 / self.max_age;
        }
        match self.kind {
            Kind::Smoke | Kind::LargeSmoke | Kind::Explode => self.motion[1] += 0.004,
            Kind::Bubble => self.motion[1] += 0.002,
            Kind::Rain | Kind::Splash => self.motion[1] -= self.gravity as f64,
            Kind::SnowShovel => self.motion[1] -= 0.03,
            Kind::Lava => {
                if self.rand.next_float() > self.age as f32 / self.max_age as f32 {
                    smoke = Some((self.pos, self.motion));
                }
                self.motion[1] -= 0.03;
            }
            Kind::Base | Kind::Digging { .. } | Kind::Slime | Kind::Snowball => {
                self.motion[1] -= 0.04 * self.gravity as f64
            }
            _ => {}
        }
        self.on_ground = move_particle(self, terrain);
        if matches!(
            self.kind,
            Kind::Smoke | Kind::LargeSmoke | Kind::RedDust | Kind::Note | Kind::Heart
        ) && self.pos[1] == self.prev[1]
        {
            self.motion[0] *= 1.1;
            self.motion[2] *= 1.1;
        }
        let friction: f32 = match self.kind {
            Kind::Cloud | Kind::Smoke | Kind::LargeSmoke | Kind::Flame | Kind::RedDust => 0.96,
            Kind::Explode => 0.9,
            Kind::Bubble => 0.85,
            Kind::Note => 0.66,
            Kind::Heart => 0.86,
            Kind::Lava => 0.999,
            Kind::SnowShovel => 0.99,
            _ => 0.98,
        };
        for v in &mut self.motion {
            *v *= friction as f64;
        }
        if self.kind == Kind::Cloud {
            if let Some(player) = self.cloud_player {
                let distance = (0..3)
                    .map(|i| (self.pos[i] - player[i]).powi(2))
                    .sum::<f64>();
                if distance < 4.0 && self.pos[1] > player[3] {
                    self.pos[1] += (player[3] - self.pos[1]) * 0.2;
                    self.motion[1] += (player[4] - self.motion[1]) * 0.2;
                    let half = (self.width / 2.0) as f64;
                    self.bounds = [
                        self.pos[0] - half,
                        self.pos[1] - self.y_offset as f64,
                        self.pos[2] - half,
                        self.pos[0] + half,
                        self.pos[1] - self.y_offset as f64 + self.height as f64,
                        self.pos[2] + half,
                    ];
                }
            }
        }
        if self.kind == Kind::Bubble {
            if material(terrain, self.pos) != worldgen::block::MAT_WATER {
                self.dead = true;
            }
            if self.max_age <= 0 {
                self.dead = true;
            }
            self.max_age -= 1;
        }
        if matches!(self.kind, Kind::Rain | Kind::Splash) {
            if self.max_age <= 0 {
                self.dead = true;
            }
            self.max_age -= 1;
        }
        if self.on_ground {
            if matches!(self.kind, Kind::Rain | Kind::Splash) && m.next_double() < 0.5 {
                self.dead = true;
            }
            self.motion[0] *= 0.7f32 as f64;
            self.motion[2] *= 0.7f32 as f64;
        }
        if matches!(self.kind, Kind::Rain | Kind::Splash) {
            let y = math::floor_double(self.pos[1]);
            let mat = worldgen::block::mat(material(terrain, self.pos));
            if mat.is_liquid || mat.is_solid {
                let meta = terrain
                    .column(
                        math::floor_double(self.pos[0]),
                        math::floor_double(self.pos[2]),
                    )
                    .map_or(0, |c| {
                        if (0..128).contains(&y) {
                            c.meta[y as usize] as i32
                        } else {
                            0
                        }
                    });
                let air = (if meta >= 8 { 1 } else { meta + 1 }) as f32 / 9.0;
                if self.pos[1] < ((y + 1) as f32 - air) as f64 {
                    self.dead = true;
                }
            }
        }
        smoke
    }
    pub fn vertices(
        &mut self,
        terrain: &Snapshot,
        cam: [f64; 3],
        rotation: [f32; 5],
        partial: f32,
    ) -> Vec<WeatherVertex> {
        if matches!(self.kind, Kind::Pickup | Kind::HugeExplode | Kind::CritEmitter) {
            return Vec::new();
        }
        if self.kind == Kind::Footstep {
            let mut t = (self.timer as f32 + partial) / 200.0;
            t *= t;
            let alpha = (2.0 - t * 2.0).min(1.0) * 0.2;
            let bright = terrain.brightness(self.pos);
            let color = packed_color([bright; 3], alpha);
            let p: [f32; 3] = std::array::from_fn(|i| (self.pos[i] - cam[i]) as f32);
            let size = 2.0 / 16.0;
            return [
                [p[0] - size, p[1], p[2] + size],
                [p[0] + size, p[1], p[2] + size],
                [p[0] + size, p[1], p[2] - size],
                [p[0] - size, p[1], p[2] - size],
            ]
            .into_iter()
            .zip([[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]])
            .map(|(pos, uv)| WeatherVertex { pos, uv, color })
            .collect();
        }
        let t = (self.age as f32 + partial) / self.max_age as f32;
        match self.kind {
            Kind::Crit
            | Kind::MagicCrit
            | Kind::Cloud
            | Kind::Smoke
            | Kind::LargeSmoke
            | Kind::RedDust
            | Kind::Note
            | Kind::Heart
            | Kind::SnowShovel => self.scale = self.original_scale * (t * 32.0).clamp(0.0, 1.0),
            Kind::Flame => self.scale = self.original_scale * (1.0 - t * t * 0.5),
            Kind::Lava => self.scale = self.original_scale * (1.0 - t * t),
            Kind::Portal => {
                let mut f = 1.0 - t;
                f *= f;
                f = 1.0 - f;
                self.scale = self.original_scale * f;
            }
            _ => {}
        }
        let pos: [f32; 3] = std::array::from_fn(|i| {
            (self.prev[i] + (self.pos[i] - self.prev[i]) * partial as f64 - cam[i]) as f32
        });
        let color = packed_color(self.color, 1.0);
        let fragment = matches!(
            self.kind,
            Kind::Digging { .. } | Kind::Snowball | Kind::Slime
        );
        let frame = ((self.timer as f32 + partial) * 15.0 / self.duration as f32) as i32;
        if self.kind == Kind::LargeExplode && frame > 15 { return Vec::new(); }
        let u0 = if self.kind == Kind::LargeExplode {
            (frame % 4) as f32 * 4.0
        } else if fragment {
            (self.texture % 16) as f32 + self.jitter[0] / 4.0
        } else {
            (self.texture % 16) as f32
        } / 16.0;
        let v0 = if self.kind == Kind::LargeExplode {
            (frame / 4) as f32 * 4.0
        } else if fragment {
            (self.texture / 16) as f32 + self.jitter[1] / 4.0
        } else {
            (self.texture / 16) as f32
        } / 16.0;
        let width = 0.999 / if self.kind == Kind::LargeExplode { 4.0 } else if fragment { 64.0 } else { 16.0 };
        let u1 = u0 + width;
        let v1 = v0 + width;
        let [rx, ry, rz, ux, uz] = rotation;
        let size = if self.kind == Kind::LargeExplode { 2.0 * self.original_scale } else { 0.1 * self.scale };
        let points = [
            [
                pos[0] - rx * size - ux * size,
                pos[1] - ry * size,
                pos[2] - rz * size - uz * size,
            ],
            [
                pos[0] - rx * size + ux * size,
                pos[1] + ry * size,
                pos[2] - rz * size + uz * size,
            ],
            [
                pos[0] + rx * size + ux * size,
                pos[1] + ry * size,
                pos[2] + rz * size + uz * size,
            ],
            [
                pos[0] + rx * size - ux * size,
                pos[1] - ry * size,
                pos[2] + rz * size - uz * size,
            ],
        ];
        let uv = if fragment {
            [[u0, v1], [u0, v0], [u1, v0], [u1, v1]]
        } else {
            [[u1, v1], [u1, v0], [u0, v0], [u0, v1]]
        };
        points
            .into_iter()
            .zip(uv)
            .map(|(pos, uv)| WeatherVertex { pos, uv, color })
            .collect()
    }
}
pub fn rotation(yaw: f32, pitch: f32) -> [f32; 5] {
    let cy = math::cos(yaw * math::PI_F32 / 180.0);
    let sy = math::sin(yaw * math::PI_F32 / 180.0);
    let sp = math::sin(pitch * math::PI_F32 / 180.0);
    [
        cy,
        math::cos(pitch * math::PI_F32 / 180.0),
        sy,
        -sy * sp,
        cy * sp,
    ]
}
fn packed_color(rgb: [f32; 3], alpha: f32) -> [f32; 4] {
    let c = |v: f32| ((v * 255.0) as i32).clamp(0, 255) as f32 / 255.0;
    [c(rgb[0]), c(rgb[1]), c(rgb[2]), c(alpha)]
}
fn material(s: &Snapshot, pos: [f64; 3]) -> u8 {
    let y = math::floor_double(pos[1]);
    s.column(math::floor_double(pos[0]), math::floor_double(pos[2]))
        .map_or(worldgen::block::MAT_AIR, |c| {
            if (0..128).contains(&y) {
                worldgen::block::material_of(c.blocks[y as usize])
            } else {
                worldgen::block::MAT_AIR
            }
        })
}

pub struct EffectRenderer {
    pub layers: [Vec<Particle>; 4],
    /// EntityFX.interpPos: only renderParticles updates it. The lit pass and
    /// clearEffects deliberately retain the previous regular pass's origin.
    pub interp_pos: [f64; 3],
    pub creation_terrain: Snapshot,
    pub cloud_player: Option<[f64; 5]>,
    pub math_random: JavaRandom,
    pub next_seed: i64,
    pub rand: JavaRandom,
    pub seed_source: Option<worldgen::entity::EntitySeedSource>,
}
impl EffectRenderer {
    /// EntityCrit2FX constructs at the target's center and emits its first
    /// burst immediately, then follows the target for two more updates.
    pub fn critical_hit(&mut self, target: &worldgen::entity::Entity, view: [f64; 3]) {
        let pos = [target.pos.x, target.bounding_box.min_y + (target.height / 2.0) as f64, target.pos.z];
        let mut p = self.create(Kind::CritEmitter, pos, [target.motion.x, target.motion.y, target.motion.z]);
        p.pickup = Some(Box::new(target.clone()));
        p.target = target.entity_id;
        p.tick(&self.creation_terrain, &mut self.math_random);
        for (name, pos, motion) in std::mem::take(&mut p.emitted) { self.spawn(name, pos, motion, view); }
        self.add(p);
    }
    /// Keep the emitter's target current without borrowing the simulation world.
    pub fn update_critical_target(&mut self, target: &worldgen::entity::Entity) {
        for p in &mut self.layers[3] {
            if p.kind == Kind::CritEmitter && p.target == target.entity_id { p.pickup = Some(Box::new(target.clone())); }
        }
    }
    /// Parallel packed-light output for integrators using the lightmap texture.
    pub fn lightmapped_vertices(&mut self, s: &Snapshot, cam: [f64; 3], yaw: f32, pitch: f32, partial: f32) -> [Vec<(WeatherVertex, i32)>; 3] {
        let mut out: [Vec<_>; 3] = std::array::from_fn(|_| Vec::new());
        self.render_particles(s, cam, yaw, pitch, partial, |layer, p, origin, rot, light, _color| {
            // These FX set the Tessellator color array, which replaces GL's
            // current material color rather than multiplying it.
            out[layer].extend(p.vertices(s, origin, rot, partial).into_iter().map(|v| (v, light)));
        });
        out
    }
    /// Original EffectRenderer.renderParticles dispatch. Brightness is evaluated
    /// immediately before each callback, including callbacks that emit no quads.
    pub fn render_particles(&mut self, s: &Snapshot, cam: [f64; 3], yaw: f32, pitch: f32, partial: f32,
        mut draw: impl FnMut(usize, &mut Particle, [f64; 3], [f32; 5], i32, Option<[f32; 4]>)) {
        let rot = rotation(yaw, pitch);
        self.interp_pos = cam;
        for layer in 0..3 {
            for (index, p) in self.layers[layer].iter_mut().enumerate() {
                let light = p.packed_brightness(s, partial);
                // b1.8.1 resets the GL material color at each nonempty layer;
                // scalar world brightness must not tint the lightmapped quads.
                draw(layer, p, cam, rot, light, (index == 0).then_some([1.0; 4]));
            }
        }
    }
    /// Original func_1187_b: insertion order in layer 3, current camera rotation,
    /// and the retained interpPos. It does not update the interpolation origin.
    pub fn render_lit_particles(&mut self, s: &Snapshot, yaw: f32, pitch: f32, partial: f32,
        mut draw: impl FnMut(&mut Particle, [f64; 3], [f32; 5], i32)) {
        let rot = rotation(yaw, pitch);
        for p in &mut self.layers[3] {
            let light = p.packed_brightness(s, partial);
            draw(p, self.interp_pos, rot, light);
        }
    }
    pub fn new(seed: i64) -> Self {
        Self {
            layers: std::array::from_fn(|_| Vec::new()),
            interp_pos: [0.0; 3],
            creation_terrain: Snapshot::default(),
            cloud_player: None,
            math_random: JavaRandom::new(seed),
            next_seed: seed.wrapping_add(1),
            rand: JavaRandom::new(seed.wrapping_add(2)),
            seed_source: None,
        }
    }
    pub fn new_runtime(seed: i64) -> Self {
        let mut r = Self::new(seed);
        r.math_random
            .set_seed(worldgen::entity::EntitySeedSource::Time.next_seed());
        r.rand
            .set_seed(worldgen::entity::EntitySeedSource::Time.next_seed());
        r.seed_source = Some(worldgen::entity::EntitySeedSource::Time);
        r
    }
    pub fn len(&self) -> usize {
        self.layers[..3].iter().map(Vec::len).sum()
    }
    pub fn clear(&mut self) {
        for l in &mut self.layers {
            l.clear();
        }
    }
    /// clearEffects also replaces the world used by subsequent constructors.
    /// Its RNGs and EntityFX's interpolation origin remain unchanged.
    pub fn clear_world(&mut self, terrain: Snapshot) {
        self.clear();
        self.creation_terrain = terrain;
    }
    pub fn create(&mut self, kind: Kind, p: [f64; 3], v: [f64; 3]) -> Particle {
        let seed = if let Some(source) = self.seed_source.as_mut() {
            source.next_seed()
        } else {
            let seed = self.next_seed;
            self.next_seed = self.next_seed.wrapping_add(1);
            seed
        };
        let mut p = Particle::new_in(
            kind,
            p,
            v,
            &mut self.math_random,
            seed,
            &self.creation_terrain,
        );
        p.cloud_player = self.cloud_player;
        p
    }
    pub fn add(&mut self, p: Particle) {
        let l = &mut self.layers[p.layer()];
        if l.len() >= 4000 {
            l.remove(0);
        }
        l.push(p);
    }
    pub fn spawn(&mut self, name: &str, p: [f64; 3], v: [f64; 3], view: [f64; 3]) {
        self.try_spawn(name, p, v, view).expect("Invalid particle factory input");
    }
    /// RenderGlobal's named factory feeding EffectRenderer. Parsing happens only
    /// after the original distance guard; large explosions bypass that guard.
    pub fn try_spawn(&mut self, name: &str, p: [f64; 3], v: [f64; 3], view: [f64; 3]) -> Result<(), &'static str> {
        let dx = view[0] - p[0];
        let dy = view[1] - p[1];
        let dz = view[2] - p[2];
        let distance = dx * dx + dy * dy + dz * dz;
        if matches!(name, "largeexplode" | "hugeexplosion") {
            let particle = self.create(Kind::named(name).unwrap(), p, v);
            self.add(particle);
            return Ok(());
        }
        if distance > 16.0 * 16.0 { return Ok(()); }
        if let Some(suffix) = name.strip_prefix("iconcrack_") {
            let id = java_particle_integer(suffix)?;
            if !(0..32000).contains(&id) { return Err("ArrayIndexOutOfBoundsException"); }
            if worldgen::entity::player::registry::item(id).is_none() { let _ = self.create(Kind::Snowball, p, [0.0; 3]); return Err("NullPointerException"); }
            let mut particle = self.create(Kind::Snowball, p, [0.0; 3]);
            particle.texture = super::entity::item_table::properties(id as i16, 0).0;
            for axis in 0..3 { particle.motion[axis] = particle.motion[axis] * 0.1f32 as f64 + v[axis]; }
            self.add(particle);
        } else if let Some(suffix) = name.strip_prefix("tilecrack_") {
            let id = java_particle_integer(suffix)?;
            if !(0..256).contains(&id) { return Err("ArrayIndexOutOfBoundsException"); }
            if worldgen::entity::player::registry::item(id).is_none() { let _ = self.create(Kind::Base, p, v); return Err("NullPointerException"); }
            let particle = self.create(Kind::Digging { block: id as u8, metadata: 0 }, p, v);
            self.add(particle);
        } else if let Some(kind) = Kind::named(name) {
            let particle = self.create(kind, p, v);
            self.add(particle);
        }
        Ok(())
    }
    pub fn tick(&mut self, s: &Snapshot, view: [f64; 3]) {
        for layer in 0..4 {
            let mut index = 0;
            while index < self.layers[layer].len() {
                self.layers[layer][index].cloud_player = self.cloud_player;
                let smoke = self.layers[layer][index].tick(s, &mut self.math_random);
                let dead = self.layers[layer][index].dead;
                let emitted = std::mem::take(&mut self.layers[layer][index].emitted);
                for (name, pos, motion) in emitted {
                    self.spawn(name, pos, motion, view);
                }
                if let Some((p, v)) = smoke {
                    self.spawn("smoke", p, v, view);
                }
                if dead {
                    self.layers[layer].remove(index);
                } else {
                    index += 1;
                }
            }
        }
    }
    pub fn vertices(
        &mut self,
        s: &Snapshot,
        cam: [f64; 3],
        yaw: f32,
        pitch: f32,
        partial: f32,
    ) -> [Vec<WeatherVertex>; 3] {
        self.lightmapped_vertices(s, cam, yaw, pitch, partial)
            .map(|layer| layer.into_iter().map(|(vertex, _)| vertex).collect())
    }
    pub fn destroy(&mut self, cell: [i32; 3], block: u8, metadata: i32) {
        if block == 0 {
            return;
        }
        for x in 0..4 {
            for y in 0..4 {
                for z in 0..4 {
                    let p = [
                        cell[0] as f64 + (x as f64 + 0.5) / 4.0,
                        cell[1] as f64 + (y as f64 + 0.5) / 4.0,
                        cell[2] as f64 + (z as f64 + 0.5) / 4.0,
                    ];
                    let v = std::array::from_fn(|i| p[i] - cell[i] as f64 - 0.5);
                    self.rand.next_int(6);
                    let p = self.create(Kind::Digging { block, metadata }, p, v);
                    self.add(p);
                }
            }
        }
    }
    pub fn hit(&mut self, cell: [i32; 3], side: i32, block: u8, metadata: i32, bounds: [f64; 6]) {
        if block == 0 {
            return;
        }
        let margin = 0.1f32;
        let mut p = std::array::from_fn(|i| {
            cell[i] as f64
                + self.rand.next_double() * (bounds[i + 3] - bounds[i] - (margin * 2.0) as f64)
                + margin as f64
                + bounds[i]
        });
        if (0..6).contains(&side) {
            let axis = [1, 1, 2, 2, 0, 0][side as usize];
            p[axis] = if side % 2 == 0 {
                cell[axis] as f64 + bounds[axis] - margin as f64
            } else {
                cell[axis] as f64 + bounds[axis + 3] + margin as f64
            };
        }
        let mut p = self.create(Kind::Digging { block, metadata }, p, [0.0; 3]);
        p.multiply_velocity(0.2);
        p.multiply_scale(0.6);
        self.add(p);
    }
}

fn intersects(a: [f64; 6], b: [f64; 6]) -> bool {
    (0..3).all(|i| a[i + 3] > b[i] && a[i] < b[i + 3])
}

fn move_particle(p: &mut Particle, s: &Snapshot) -> bool {
    if p.no_clip {
        for i in 0..3 {
            p.bounds[i] += p.motion[i];
            p.bounds[i + 3] += p.motion[i];
        }
        p.pos = [
            (p.bounds[0] + p.bounds[3]) / 2.0,
            p.bounds[1] + p.y_offset as f64,
            (p.bounds[2] + p.bounds[5]) / 2.0,
        ];
        return p.on_ground;
    }
    let mut original = p.motion;
    if p.in_web {
        p.in_web = false;
        original[0] *= 0.25;
        original[1] *= 0.05f32 as f64;
        original[2] *= 0.25;
        p.motion = [0.0; 3];
    }
    let mut sweep = p.bounds;
    for i in 0..3 {
        if original[i] < 0.0 {
            sweep[i] += original[i];
        } else {
            sweep[i + 3] += original[i];
        }
    }
    let mut boxes = Vec::new();
    for x in math::floor_double(sweep[0])..math::floor_double(sweep[3] + 1.0) {
        for z in math::floor_double(sweep[2])..math::floor_double(sweep[5] + 1.0) {
            let Some(c) = s.column(x, z) else {
                continue;
            };
            for y in math::floor_double(sweep[1]) - 1..math::floor_double(sweep[4] + 1.0) {
                if !(0..128).contains(&y) {
                    continue;
                }
                for b in collision_boxes(c.blocks[y as usize], c.meta[y as usize], [x, y, z]) {
                    if intersects(b, sweep) {
                        boxes.push(b);
                    }
                }
            }
        }
    }
    let mut delta = original;
    for axis in [1, 0, 2] {
        for b in &boxes {
            if !(0..3)
                .filter(|&i| i != axis)
                .all(|i| p.bounds[i + 3] > b[i] && p.bounds[i] < b[i + 3])
            {
                continue;
            }
            if delta[axis] > 0.0 && p.bounds[axis + 3] <= b[axis] {
                delta[axis] = delta[axis].min(b[axis] - p.bounds[axis + 3]);
            }
            if delta[axis] < 0.0 && p.bounds[axis] >= b[axis + 3] {
                delta[axis] = delta[axis].max(b[axis + 3] - p.bounds[axis]);
            }
        }
        p.bounds[axis] += delta[axis];
        p.bounds[axis + 3] += delta[axis];
        if delta[axis] != original[axis] {
            p.motion[axis] = 0.0;
        }
    }
    p.pos = [
        (p.bounds[0] + p.bounds[3]) / 2.0,
        p.bounds[1] + p.y_offset as f64,
        (p.bounds[2] + p.bounds[5]) / 2.0,
    ];
    // Block.onEntityCollidedWithBlock scans the contracted bounding box.
    for x in math::floor_double(p.bounds[0] + 0.001)..=math::floor_double(p.bounds[3] - 0.001) {
        for z in math::floor_double(p.bounds[2] + 0.001)..=math::floor_double(p.bounds[5] - 0.001) {
            if let Some(c) = s.column(x, z) {
                for y in math::floor_double(p.bounds[1] + 0.001)
                    ..=math::floor_double(p.bounds[4] - 0.001)
                {
                    if !(0..128).contains(&y) {
                        continue;
                    }
                    match c.blocks[y as usize] {
                        30 => p.in_web = true,
                        88 => {
                            p.motion[0] *= 0.4;
                            p.motion[2] *= 0.4;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    delta[1] != original[1] && original[1] < 0.0
}

/// Block collision boxes used by ambient particles. No step-up (EntityFX.stepHeight=0).
fn collision_boxes(id: u8, meta: u8, pos: [i32; 3]) -> Vec<[f64; 6]> {
    if matches!(id, 0 | 6 | 8..=11 | 27 | 28 | 30..=32 | 36 | 37..=40 | 50 | 51 | 55 | 59 | 63 | 66 | 68..=70 | 72 | 75..=77 | 83 | 90)
    {
        return Vec::new();
    }
    let mut b = rprops(id).bounds;
    let mut shapes = Vec::new();
    match id {
        29 | 33 => b = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        34 => {
            let pair = match meta & 7 {
                0 => [
                    [0.0, 0.0, 0.0, 1.0, 0.25, 1.0],
                    [0.375, 0.25, 0.375, 0.625, 1.0, 0.625],
                ],
                1 => [
                    [0.0, 0.75, 0.0, 1.0, 1.0, 1.0],
                    [0.375, 0.0, 0.375, 0.625, 0.75, 0.625],
                ],
                2 => [
                    [0.0, 0.0, 0.0, 1.0, 1.0, 0.25],
                    [0.25, 0.375, 0.25, 0.75, 0.625, 1.0],
                ],
                3 => [
                    [0.0, 0.0, 0.75, 1.0, 1.0, 1.0],
                    [0.25, 0.375, 0.0, 0.75, 0.625, 0.75],
                ],
                4 => [
                    [0.0, 0.0, 0.0, 0.25, 1.0, 1.0],
                    [0.375, 0.25, 0.25, 0.625, 0.75, 1.0],
                ],
                5 => [
                    [0.75, 0.0, 0.0, 1.0, 1.0, 1.0],
                    [0.0, 0.375, 0.25, 0.75, 0.625, 0.75],
                ],
                _ => return shapes,
            };
            for b in pair {
                shapes.push(std::array::from_fn(|i| pos[i % 3] as f64 + b[i]));
            }
            return shapes;
        }
        53 | 67 => {
            let pair = match meta {
                0 => [
                    [0.0, 0.0, 0.0, 0.5, 0.5, 1.0],
                    [0.5, 0.0, 0.0, 1.0, 1.0, 1.0],
                ],
                1 => [
                    [0.0, 0.0, 0.0, 0.5, 1.0, 1.0],
                    [0.5, 0.0, 0.0, 1.0, 0.5, 1.0],
                ],
                2 => [
                    [0.0, 0.0, 0.0, 1.0, 0.5, 0.5],
                    [0.0, 0.0, 0.5, 1.0, 1.0, 1.0],
                ],
                3 => [
                    [0.0, 0.0, 0.0, 1.0, 1.0, 0.5],
                    [0.0, 0.0, 0.5, 1.0, 0.5, 1.0],
                ],
                _ => return shapes,
            };
            for b in pair {
                shapes.push(std::array::from_fn(|i| pos[i % 3] as f64 + b[i]));
            }
            return shapes;
        }
        60 => b = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        78 => {
            if meta & 7 < 3 {
                return shapes;
            }
            b[4] = 0.5;
        }
        85 => b = [0.0, 0.0, 0.0, 1.0, 1.5, 1.0],
        81 => b = [0.0625, 0.0, 0.0625, 0.9375, 0.9375, 0.9375],
        88 => b[4] = 0.875,
        92 => {
            b = [
                (1 + meta as i32 * 2) as f32 / 16.0,
                0.0,
                0.0625,
                0.9375,
                0.4375,
                0.9375,
            ]
        }
        64 | 71 | 65 | 96 => {
            let width = if id == 65 { 0.125 } else { 0.1875 };
            let side = if id == 65 {
                match meta {
                    2 => 2,
                    3 => 0,
                    4 => 1,
                    5 => 3,
                    _ => 0,
                }
            } else if id == 96 {
                if meta & 4 == 0 {
                    b = [0.0, 0.0, 0.0, 1.0, width, 1.0];
                    4
                } else {
                    [2, 0, 1, 3][(meta & 3) as usize]
                }
            } else {
                if meta & 4 == 0 {
                    meta.wrapping_sub(1) & 3
                } else {
                    meta & 3
                }
            };
            b = match side {
                0 => [0.0, 0.0, 0.0, 1.0, 1.0, width],
                1 => [1.0 - width, 0.0, 0.0, 1.0, 1.0, 1.0],
                2 => [0.0, 0.0, 1.0 - width, 1.0, 1.0, 1.0],
                3 => [0.0, 0.0, 0.0, width, 1.0, 1.0],
                _ => b,
            };
        }
        _ => {}
    }
    // Cactus/cake/fence use float addition before widening in vanilla.
    shapes.push(std::array::from_fn(|i| {
        if matches!(id, 81 | 92) || (id == 85 && i == 4) {
            (pos[i % 3] as f32 + b[i]) as f64
        } else {
            pos[i % 3] as f64 + b[i] as f64
        }
    }));
    shapes
}

/// Integer.parseInt uses Character.digit on UTF-16 code units, not ASCII or
/// Unicode scalar values. These decimal ranges are from Mojang Java 8u51.
fn java_particle_integer(s: &str) -> Result<i32, &'static str> {
    const ZERO: &[u16] = &[0x0030, 0x0660, 0x06f0, 0x07c0, 0x0966, 0x09e6, 0x0a66, 0x0ae6, 0x0b66, 0x0be6, 0x0c66, 0x0ce6, 0x0d66, 0x0e50, 0x0ed0, 0x0f20, 0x1040, 0x1090, 0x17e0, 0x1810, 0x1946, 0x19d0, 0x1a80, 0x1a90, 0x1b50, 0x1bb0, 0x1c40, 0x1c50, 0xa620, 0xa8d0, 0xa900, 0xa9d0, 0xaa50, 0xabf0, 0xff10,];
    let mut units = s.encode_utf16().peekable();
    let negative = match units.peek().copied() {
        Some(45) => { units.next(); true },
        Some(43) => { units.next(); false },
        _ => false,
    };
    let limit = if negative { 2147483648i64 } else { 2147483647i64 };
    let mut value = 0i64;
    let mut count = 0;
    for c in units {
        let digit = ZERO.iter().find_map(|&z| if c >= z && c < z + 10 { Some((c-z) as i64) } else { None }).ok_or("NumberFormatException")?;
        value = value * 10 + digit;
        if value > limit { return Err("NumberFormatException"); }
        count += 1;
    }
    if count == 0 { return Err("NumberFormatException"); }
    Ok(if negative { -value as i32 } else { value as i32 })
}
