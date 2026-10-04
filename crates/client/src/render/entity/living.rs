//! `RenderManager.renderEntity`, `RenderLiving.doRenderLiving` and mob callbacks.
//! State contains render inputs only; it requires no world, spawning or AI.

use super::geometry::{ModelRenderer, TexturedQuad};
use super::models::{Model, ModelKind, WolfAnimation};
use glam::{DVec3, Mat4, Vec3};
use worldgen::math::{self, PI_F32};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MobKind {
    Biped,
    Zombie,
    Skeleton,
    Pig,
    Cow,
    Sheep,
    Chicken,
    Spider,
    Creeper,
    Slime,
    Squid,
    Wolf,
    Ghast,
    PigZombie,
    Giant,
}

/// Interpolated `EntityLiving` inputs. The simulation can fill this at partialTicks:
/// position = lastTickPos + (pos-lastTickPos)*partial (doubles), yaw/bodyYaw/pitch
/// = prev + (current-prev)*partial (floats), limbAmount = Q+(R-Q)*partial,
/// limbSwing = S-R*(1-partial), age = ticksExisted+partial, attack = getSwingProgress.
#[derive(Clone, Debug)]
pub struct RenderState {
    pub ghast_attack: f32,
    pub ghast_attacking: bool,
    pub held_item: Option<super::equipped::HeldSprite>,
    pub kind: MobKind,
    pub position: [f64; 3],
    pub yaw: f32,
    pub body_yaw: f32,
    pub pitch: f32,
    pub limb_swing: f32,
    pub limb_amount: f32,
    pub age: f32,
    pub attack: f32,
    pub partial: f32,
    pub hurt_time: i32,
    pub death_time: i32,
    pub brightness: f32,
    pub riding: bool,
    pub sneak: bool,
    pub held_left: bool,
    pub held_right: bool,
    pub saddled: bool,
    pub sheared: bool,
    pub fleece_color: usize,
    /// EntityCreeper.setCreeperFlashTime(partial).
    pub creeper_flash: f32,
    pub powered: bool,
    pub slime_size: i32,
    /// field_767_b + (field_768_a-field_767_b)*partial, before size division.
    pub slime_squish: f32,
    /// Interpolated chicken flap phase and amplitude (field_752_b, destPos).
    pub chicken_flap: f32,
    pub chicken_flap_amount: f32,
    /// Squid's interpolated field_21089_a/field_21087_c and field_21083_i.
    pub squid_pitch: f32,
    pub squid_yaw: f32,
    pub tentacle_angle: f32,
    pub wolf: WolfAnimation,
    pub wolf_tamed: bool,
    pub wolf_health: i32,
    pub wolf_shaking: bool,
    /// Spider eyes read brightness at partialTicks=1 rather than the current partial.
    pub spider_eye_brightness: f32,
}

impl RenderState {
    pub fn new(kind: MobKind) -> Self {
        Self {
            ghast_attack: 0.0,
            ghast_attacking: false,
            held_item: if kind == MobKind::Skeleton {
                Some(super::equipped::HeldSprite::BOW)
            } else if kind == MobKind::PigZombie {
                Some(super::equipped::HeldSprite { icon: 68, full_3d: true, terrain: false })
            } else {
                None
            },
            kind,
            position: [0.0; 3],
            yaw: 0.0,
            body_yaw: 0.0,
            pitch: 0.0,
            limb_swing: 0.0,
            limb_amount: 0.0,
            age: 0.0,
            attack: 0.0,
            partial: 0.0,
            hurt_time: 0,
            death_time: 0,
            brightness: 1.0,
            riding: false,
            sneak: false,
            held_left: false,
            held_right: false,
            saddled: false,
            sheared: false,
            fleece_color: 0,
            creeper_flash: 0.0,
            powered: false,
            slime_size: 1,
            slime_squish: 0.0,
            chicken_flap: 0.0,
            chicken_flap_amount: 0.0,
            squid_pitch: 0.0,
            squid_yaw: 0.0,
            tentacle_angle: 0.0,
            wolf: WolfAnimation::default(),
            wolf_tamed: false,
            wolf_health: 8,
            wolf_shaking: false,
            spider_eye_brightness: 1.0,
        }
    }
}

/// EntitySheep.fleeceColorTable, in metadata order.
pub const FLEECE_COLORS: [[f32; 3]; 16] = [
    [1.0, 1.0, 1.0],
    [0.95, 0.7, 0.2],
    [0.9, 0.5, 0.85],
    [0.6, 0.7, 0.95],
    [0.9, 0.9, 0.2],
    [0.5, 0.8, 0.1],
    [0.95, 0.7, 0.8],
    [0.3, 0.3, 0.3],
    [0.6, 0.6, 0.6],
    [0.3, 0.6, 0.7],
    [0.7, 0.4, 0.9],
    [0.2, 0.4, 0.8],
    [0.5, 0.4, 0.3],
    [0.4, 0.5, 0.2],
    [0.8, 0.3, 0.3],
    [0.1, 0.1, 0.1],
];

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct EntityVertex {
    pub pos: [f32; 3],
    pub uv: [f32; 2],
    /// GL primary color after fixed-function lighting and clamping.
    pub color: [f32; 4],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pass {
    Opaque,
    Alpha,
    Eyes,
    Additive,
    Overlay,
    TntFlash,
    Shadow,
}

pub struct Batch {
    /// `None` is GL_TEXTURE_2D disabled (hurt/flash), represented by a white texel.
    pub texture: Option<&'static str>,
    pub pass: Pass,
    pub vertices: Vec<EntityVertex>,
}

/// All jar resources used by the models, including the powered creeper pass.
pub const TEXTURES: &[&str] = &[
    "gui/items.png",
    "terrain.png",
    "item/arrows.png",
    "misc/shadow.png",
    "mob/ghast.png",
    "mob/ghast_fire.png",
    "mob/pigzombie.png",
    "mob/char.png",
    "mob/zombie.png",
    "mob/skeleton.png",
    "mob/pig.png",
    "mob/saddle.png",
    "mob/cow.png",
    "mob/sheep.png",
    "mob/sheep_fur.png",
    "mob/chicken.png",
    "mob/spider.png",
    "mob/spider_eyes.png",
    "mob/creeper.png",
    "mob/slime.png",
    "mob/squid.png",
    "mob/wolf.png",
    "mob/wolf_angry.png",
    "mob/wolf_tame.png",
    "armor/power.png",
];

pub fn model_kind(kind: MobKind) -> ModelKind {
    match kind {
        MobKind::Biped => ModelKind::Biped,
        MobKind::Zombie => ModelKind::Zombie,
        MobKind::Skeleton => ModelKind::Skeleton,
        MobKind::Pig => ModelKind::Pig,
        MobKind::Cow => ModelKind::Cow,
        MobKind::Sheep => ModelKind::Sheep,
        MobKind::Chicken => ModelKind::Chicken,
        MobKind::Spider => ModelKind::Spider,
        MobKind::Creeper => ModelKind::Creeper,
        MobKind::Slime => ModelKind::SlimeInner,
        MobKind::Squid => ModelKind::Squid,
        MobKind::Wolf => ModelKind::Wolf,
        MobKind::Ghast => ModelKind::Ghast,
        MobKind::PigZombie => ModelKind::Zombie,
        MobKind::Giant => ModelKind::Zombie,
    }
}

fn texture(s: &RenderState) -> &'static str {
    match s.kind {
        MobKind::Biped => "mob/char.png",
        MobKind::Zombie | MobKind::Giant => "mob/zombie.png",
        MobKind::PigZombie => "mob/pigzombie.png",
        MobKind::Ghast => {
            if s.ghast_attacking {
                "mob/ghast_fire.png"
            } else {
                "mob/ghast.png"
            }
        }
        MobKind::Skeleton => "mob/skeleton.png",
        MobKind::Pig => "mob/pig.png",
        MobKind::Cow => "mob/cow.png",
        MobKind::Sheep => "mob/sheep.png",
        MobKind::Chicken => "mob/chicken.png",
        MobKind::Spider => "mob/spider.png",
        MobKind::Creeper => "mob/creeper.png",
        MobKind::Slime => "mob/slime.png",
        MobKind::Squid => "mob/squid.png",
        MobKind::Wolf => {
            if s.wolf_tamed {
                "mob/wolf_tame.png"
            } else if s.wolf.angry {
                "mob/wolf_angry.png"
            } else {
                "mob/wolf.png"
            }
        }
    }
}

/// `RenderLiving.func_170_d` and RenderChicken/RenderSquid/RenderWolf overrides.
pub fn animation_age(s: &RenderState) -> f32 {
    match s.kind {
        MobKind::Chicken => (math::sin(s.chicken_flap) + 1.0) * s.chicken_flap_amount,
        MobKind::Squid => s.tentacle_angle,
        MobKind::Wolf => {
            if s.wolf.angry {
                PI_F32 * 0.49
            } else if s.wolf_tamed {
                (0.55 - (20 - s.wolf_health) as f32 * 0.02) * PI_F32
            } else {
                PI_F32 * 0.2
            }
        }
        _ => s.age,
    }
}

/// RenderLiving.rotateCorpse/preRenderCallback and the mob-specific overrides.
pub fn model_transform(s: &RenderState, camera: DVec3) -> Mat4 {
    let rel = (DVec3::from_array(s.position) - camera).as_vec3();
    let mut m = Mat4::from_translation(rel);
    if s.kind == MobKind::Squid {
        m *= Mat4::from_translation(Vec3::new(0.0, 0.5, 0.0));
        m *= Mat4::from_rotation_y((180.0 - s.body_yaw).to_radians());
        m *= Mat4::from_rotation_x(s.squid_pitch.to_radians());
        m *= Mat4::from_rotation_y(s.squid_yaw.to_radians());
        m *= Mat4::from_translation(Vec3::new(0.0, -1.2, 0.0));
    } else {
        m *= Mat4::from_rotation_y((180.0 - s.body_yaw).to_radians());
        if s.death_time > 0 {
            let mut f = (s.death_time as f32 + s.partial - 1.0) / 20.0 * 1.6;
            f = (f as f64).sqrt() as f32;
            if f > 1.0 {
                f = 1.0;
            }
            let max = if s.kind == MobKind::Spider { 180.0 } else { 90.0 };
            m *= Mat4::from_rotation_z((f * max).to_radians());
        }
    }
    m *= Mat4::from_scale(Vec3::new(-1.0, -1.0, 1.0));
    if s.kind == MobKind::Ghast {
        let mut f = (s.ghast_attack / 20.0).max(0.0);
        f = 1.0 / (f * f * f * f * f * 2.0 + 1.0);
        let y = (8.0 + f) / 2.0;
        let xz = (8.0 + 1.0 / f) / 2.0;
        m *= Mat4::from_scale(Vec3::new(xz, y, xz));
    } else if s.kind == MobKind::Giant {
        m *= Mat4::from_scale(Vec3::splat(6.0));
    } else if s.kind == MobKind::Creeper {
        let mut f = s.creeper_flash;
        let wobble = 1.0 + math::sin(f * 100.0) * f * 0.01;
        if f < 0.0 {
            f = 0.0;
        }
        if f > 1.0 {
            f = 1.0;
        }
        f *= f;
        f *= f;
        let xz = (1.0 + f * 0.4) * wobble;
        let y = (1.0 + f * 0.1) / wobble;
        m *= Mat4::from_scale(Vec3::new(xz, y, xz));
    } else if s.kind == MobKind::Slime {
        let squish = s.slime_squish / (s.slime_size as f32 * 0.5 + 1.0);
        let inv = 1.0 / (squish + 1.0);
        let size = s.slime_size as f32;
        m *= Mat4::from_scale(Vec3::new(inv * size, 1.0 / inv * size, inv * size));
    }
    m * Mat4::from_translation(Vec3::new(0.0, -24.0 * (1.0 / 16.0) - 0.0078125, 0.0))
}

/// RenderHelper light positions: Vec3D.normalize narrows sqrt to float first.
fn light(x: f32, z: f32) -> Vec3 {
    let x = x as f64;
    let z = z as f64;
    let len = ((x * x + 1.0 + z * z).sqrt() as f32) as f64;
    Vec3::new((x / len) as f32, (1.0 / len) as f32, (z / len) as f32).normalize()
}

/// GL_BYTE normal arrays as written by Tessellator.setNormal. Preserve both its
/// 128 X multiplier and signed-byte OR/sign-extension, rather than repairing them.
pub fn packed_normal(q: &TexturedQuad) -> u32 {
    pack_normal(q.normal())
}

pub fn pack_normal(n: Vec3) -> u32 {
    let x = (n.x * 128.0) as i32 as i8 as i32;
    let y = (n.y * 127.0) as i32 as i8 as i32;
    let z = (n.z * 127.0) as i32 as i8 as i32;
    (x | (y << 8) | (z << 16)) as u32
}

pub fn tessellator_normal(q: &TexturedQuad) -> Vec3 {
    normal_from_packed(packed_normal(q))
}

pub fn normal_from_packed(packed: u32) -> Vec3 {
    let packed = packed as i32;
    let byte = |shift| (((packed >> shift) as i8 as f32) / 127.0).max(-1.0);
    Vec3::new(byte(0), byte(8), byte(16))
}

/// GL_COLOR_MATERIAL is FRONT_AND_BACK/AMBIENT_AND_DIFFUSE, with ambient 0.4,
/// two diffuse 0.6 directional lights and zero light ambient/specular. Clamp
/// the final primary color, after multiplying material (entity brightness/tint).
pub fn lit_color(n: Vec3, material: [f32; 4]) -> [f32; 4] {
    let diffuse = 0.6 * n.dot(light(0.2, -0.7)).max(0.0) + 0.6 * n.dot(light(-0.2, 0.7)).max(0.0);
    let f = 0.4 + diffuse;
    [
        (material[0] * f).clamp(0.0, 1.0),
        (material[1] * f).clamp(0.0, 1.0),
        (material[2] * f).clamp(0.0, 1.0),
        material[3],
    ]
}

fn part_triangles(
    part: &ModelRenderer,
    parent: Mat4,
    with_rotation: bool,
    color: [f32; 4],
    lit: bool,
    normalize: bool,
    uv_offset: f32,
    out: &mut Vec<EntityVertex>,
) {
    if part.hidden || !part.show_model {
        return;
    }
    let scale = 1.0 / 16.0;
    let m = if with_rotation { part.transform(parent, scale, true) } else { part.post_render(parent, scale) };
    let inverse = m.inverse();
    let normal_matrix = inverse.transpose();
    // GL_RESCALE_NORMAL: reciprocal length of the inverse modelview's third row.
    let rescale = 1.0 / inverse.row(2).truncate().length();
    for face in &part.faces {
        let mut n = normal_matrix.transform_vector3(tessellator_normal(face));
        if normalize {
            n = n.normalize_or_zero();
        } else {
            n *= rescale;
        }
        let c = if lit { lit_color(n, color) } else { color };
        let v = face.vertices.map(|v| {
            let p = Vec3::from_array(v.pos.map(|p| p as f32 * scale));
            EntityVertex { pos: m.transform_point3(p).to_array(), uv: v.uv.map(|u| u + uv_offset), color: c }
        });
        for i in [0, 1, 2, 0, 2, 3] {
            out.push(v[i]);
        }
    }
    for child in &part.children {
        part_triangles(child, m, false, color, lit, normalize, uv_offset, out);
    }
}

fn batch(
    m: &Model,
    transform: Mat4,
    texture: Option<&'static str>,
    pass: Pass,
    color: [f32; 4],
    lit: bool,
    normalize: bool,
    uv_offset: f32,
) -> Batch {
    let mut b = Batch { texture, pass, vertices: Vec::new() };
    for (part, r) in m.render_parts() {
        part_triangles(part, transform, r, color, lit, normalize, uv_offset, &mut b.vertices);
    }
    b
}

/// Produces the original ordered textured/lit triangle layers. Culling is disabled
/// throughout RenderLiving; overlays use GL_EQUAL and no alpha test or texture.
pub fn render_living(s: &RenderState, camera: DVec3) -> Vec<Batch> {
    render_with_lighting(s, camera, false)
}

fn render_with_lighting(s: &RenderState, camera: DVec3, normalize: bool) -> Vec<Batch> {
    if matches!(s.kind, MobKind::Ghast | MobKind::PigZombie | MobKind::Giant) {
        return super::special::render(s, camera.to_array(), normalize).draws.iter().map(|d| d.batch()).collect();
    }
    let transform = model_transform(s, camera);
    let mut main = Model::new(model_kind(s.kind));
    main.on_ground = s.attack;
    main.is_riding = s.riding;
    main.is_sneak = s.sneak;
    main.held_left = s.held_left;
    main.held_right = s.held_right;
    let mut amount = s.limb_amount;
    if amount > 1.0 {
        amount = 1.0;
    }
    let pose = [s.limb_swing, amount, animation_age(s), s.yaw - s.body_yaw, s.pitch, 1.0 / 16.0];
    main.set_living_animations(pose[0], pose[1], s.partial, &s.wolf);
    main.set_rotation_angles(pose);
    let mut brightness = s.brightness;
    if s.kind == MobKind::Wolf && s.wolf_shaking {
        let shake = s.wolf.shake[0] + (s.wolf.shake[1] - s.wolf.shake[0]) * s.partial;
        brightness *= 12.0 / 16.0 + shake / 2.0 * 0.25;
    }
    let color = if s.kind == MobKind::Ghast { [1.0; 4] } else { [brightness, brightness, brightness, 1.0] };
    let mut result = vec![batch(&main, transform, Some(texture(s)), Pass::Opaque, color, true, normalize, 0.0)];
    let mut pass_model = None;
    let mut pass_color = color;
    let mut pass_texture = texture(s);
    let mut pass = Pass::Opaque;
    match s.kind {
        MobKind::Pig if s.saddled => {
            pass_model = Some(Model::with_options(ModelKind::Pig, 0.5, 0.0, 6));
            pass_texture = "mob/saddle.png";
        }
        MobKind::Sheep if !s.sheared => {
            pass_model = Some(Model::new(ModelKind::SheepFur));
            pass_texture = "mob/sheep_fur.png";
            let fleece = FLEECE_COLORS[s.fleece_color];
            pass_color = [s.brightness * fleece[0], s.brightness * fleece[1], s.brightness * fleece[2], 1.0];
        }
        MobKind::Spider => {
            pass_model = Some(Model::new(ModelKind::Spider));
            pass_texture = "mob/spider_eyes.png";
            pass_color = [1.0, 1.0, 1.0, (1.0 - s.spider_eye_brightness) * 0.5];
            pass = Pass::Eyes;
        }
        MobKind::Slime => {
            pass_model = Some(Model::new(ModelKind::SlimeOuter));
            pass = Pass::Alpha;
        }
        MobKind::Creeper if s.powered => {
            pass_model = Some(Model::with_options(ModelKind::Creeper, 2.0, 0.0, 12));
            pass_texture = "armor/power.png";
            pass_color = [0.5, 0.5, 0.5, 1.0];
            pass = Pass::Additive;
        }
        _ => {}
    }
    if let Some(m) = &mut pass_model {
        m.on_ground = s.attack;
        m.is_riding = s.riding;
        m.set_rotation_angles(pose);
        let offset = if pass == Pass::Additive { s.age * 0.01 } else { 0.0 };
        result.push(batch(
            m,
            transform,
            Some(pass_texture),
            pass,
            pass_color,
            pass != Pass::Additive,
            normalize || s.kind == MobKind::Slime,
            offset,
        ));
    }
    if matches!(s.kind, MobKind::Biped | MobKind::Zombie | MobKind::Skeleton | MobKind::PigZombie) {
        if let Some(item) = s.held_item {
            result.push(super::equipped::render_item(&main, transform, item, color, normalize));
        }
    }
    let mut overlays = Vec::new();
    if s.hurt_time > 0 || s.death_time > 0 {
        overlays.push([s.brightness, 0.0, 0.0, 0.4]);
    }
    if s.kind == MobKind::Creeper && (s.creeper_flash * 10.0) as i32 % 2 != 0 {
        let a = ((s.creeper_flash * 0.2 * 255.0) as i32).clamp(0, 255) as f32 / 255.0;
        if a > 0.0 {
            overlays.push([1.0, 1.0, 1.0, a]);
        }
    }
    for c in overlays {
        result.push(batch(&main, transform, None, Pass::Overlay, c, true, normalize || s.kind == MobKind::Slime, 0.0));
        // RenderCreeper.func_27005_b explicitly suppresses overlay passes.
        if s.kind != MobKind::Creeper {
            if let Some(m) = &pass_model {
                result.push(batch(
                    m,
                    transform,
                    None,
                    Pass::Overlay,
                    c,
                    true,
                    normalize || s.kind == MobKind::Slime,
                    0.0,
                ));
            }
        }
    }
    result
}

pub fn scene(states: &[RenderState], camera: DVec3) -> Vec<Batch> {
    scene_with_lighting(states, camera, &mut false)
}

/// RenderSlime leaves GL_NORMALIZE enabled. Track the original GL state across
/// entities and frames, without global mutable Rust state.
pub fn scene_with_lighting(states: &[RenderState], camera: DVec3, normalize: &mut bool) -> Vec<Batch> {
    let mut batches = Vec::new();
    for s in states {
        batches.extend(render_with_lighting(s, camera, *normalize));
        if s.kind == MobKind::Slime {
            *normalize = true;
        }
    }
    batches
}
