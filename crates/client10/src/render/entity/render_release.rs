//! Release 1.0 entity renderer callbacks, reading simulation state only.
use super::{
    fixed::Fixed,
    inventory,
    living::{MobKind, RenderState},
    models::{Model, ModelKind},
    special,
};
use worldgen::{
    math::{self, PI_F32},
    JavaRandom,
};

pub fn equipped(g: &mut Fixed, m: &Model, s: &RenderState) {
    if s.kind == MobKind::Mooshroom && !s.is_child {
        g.texture = 1;
        g.culling = true;
        g.push();
        g.scale(1.0, -1.0, 1.0);
        g.translate(0.2, 0.4, 0.5);
        g.rotate(42.0, 0.0, 1.0, 0.0);
        inventory::inventory(g, 40, 0, 1.0, true);
        g.translate(0.1, 0.0, -0.6);
        g.rotate(42.0, 0.0, 1.0, 0.0);
        inventory::inventory(g, 40, 0, 1.0, true);
        g.pop();
        g.push();
        special::part(g, &m["head"], true);
        g.scale(1.0, -1.0, 1.0);
        g.translate(0.0, 0.75, -0.2);
        g.rotate(12.0, 0.0, 1.0, 0.0);
        inventory::inventory(g, 40, 0, 1.0, true);
        g.pop();
        g.culling = false;
    }
    if s.kind == MobKind::SnowMan {
        g.push();
        special::part(g, &m["field_40305_c"], true);
        g.translate(0.0, -0.34375, 0.0);
        g.rotate(180.0, 0.0, 1.0, 0.0);
        g.scale(0.625, -0.625, 0.625);
        crate::render::first_person::item(
            g,
            worldgen::container::ItemStack::new(86, 1, 0),
            1.0,
            false,
        );
        g.pop();
    }
}

pub fn dragon(s: &RenderState, camera: [f64; 3], normalize: bool) -> Fixed {
    submit_boss_health(s.boss_health);
    let mut g = Fixed::new(s.brightness);
    if normalize {
        g.enable(4, true);
    }
    let state = s.dragon.as_ref().expect("dragon render history");
    let history = std::array::from_fn(|i| state.movement_offsets(i as i32, s.partial, s.health));
    let animation =
        state.previous_animation + (state.animation - state.previous_animation) * s.partial;
    let mut m = Model::new(ModelKind::Dragon);
    // The first version-4 main-model replacement retains the constructor pass
    // model. Its setLivingAnimations partial remains zero in the original jar.
    let pass_history = std::array::from_fn(|i| state.movement_offsets(i as i32, 0.0, s.health));
    let mut pass_model = Model::new(ModelKind::Dragon);
    g.push();
    g.translate(
        (s.position[0] - camera[0]) as f32,
        (s.position[1] - camera[1]) as f32,
        (s.position[2] - camera[2]) as f32,
    );
    g.rotate(-(history[7][0] as f32), 0.0, 1.0, 0.0);
    g.rotate(
        (history[5][1] - history[10][1]) as f32 * 10.0,
        1.0,
        0.0,
        0.0,
    );
    g.translate(0.0, 0.0, 1.0);
    if s.death_time > 0 {
        let mut f = (((s.death_time as f32 + s.partial - 1.0) / 20.0 * 1.6) as f64).sqrt() as f32;
        if f > 1.0 {
            f = 1.0;
        }
        g.rotate(f * 90.0, 0.0, 0.0, 1.0);
    }
    g.enable(2, true);
    g.scale(-1.0, -1.0, 1.0);
    g.translate(0.0, -24.0 * 0.0625 - 0.0078125, 0.0);
    if state.death_ticks > 0 {
        g.texture = 72;
        g.alpha_cutoff = state.death_ticks as f32 / 200.0;
        m.dragon_render(&mut g, animation, &history, 0.0625);
        g.alpha_cutoff = 0.1;
        g.depth = 514;
    }
    g.texture = 70;
    m.dragon_render(&mut g, animation, &history, 0.0625);
    if s.hurt_time > 0 {
        g.depth = 514;
        g.enable(8, false);
        g.enable(16, true);
        g.src = 770;
        g.dst = 771;
        g.material = [1.0, 0.0, 0.0, 0.5];
        m.dragon_render(&mut g, animation, &history, 0.0625);
        g.enable(8, true);
        g.enable(16, false);
        g.depth = 515;
    }
    g.texture = 71;
    g.depth = 514;
    g.enable(16, true);
    g.src = 1;
    g.dst = 1;
    g.material = [1.0; 4];
    pass_model.dragon_render(&mut g, state.previous_animation, &pass_history, 0.0625);
    g.enable(16, false);
    g.depth = 515;
    if state.death_ticks > 0 {
        death_rays(&mut g, state.death_ticks, s.partial);
    }
    if s.hurt_time > 0 || s.death_time > 0 {
        g.enable(8, false);
        g.enable(16, true);
        g.src = 770;
        g.dst = 771;
        g.depth = 514;
        g.material = [s.overlay_brightness, 0.0, 0.0, 0.4];
        m.dragon_render(&mut g, animation, &history, 0.0625);
        g.texture = 71;
        g.src = 1;
        g.dst = 1;
        pass_model.dragon_render(&mut g, state.previous_animation, &pass_history, 0.0625);
        g.enable(8, true);
        g.enable(16, false);
        g.depth = 515;
    }
    g.enable(2, false);
    g.pop();
    if let Some((position, rotation)) = s.dragon_crystal {
        dragon_beam(&mut g, s, camera, position, rotation);
    }
    g
}

thread_local! {static BOSS_HEALTH: std::cell::Cell<Option<i32>> = const {std::cell::Cell::new(None)};}
pub struct BossPlan {
    pub rectangles: Vec<[i32; 6]>,
    pub text: [i32; 3],
}
pub fn boss_plan(width: i32, font_width: i32, health: i32) -> BossPlan {
    let x = width / 2 - 182 / 2;
    let fill = (health as f32 / 200.0 * 183.0) as i32;
    let mut rectangles = vec![[x, 12, 0, 74, 182, 5]; 2];
    if fill > 0 {
        rectangles.push([x, 12, 0, 79, fill, 5]);
    }
    BossPlan {
        rectangles,
        text: [width / 2 - font_width / 2, 2, 16711935],
    }
}
/// The HUD consumes the last dragon submitted to RenderDragon once, as in 1.0.
pub(crate) fn submit_boss_health(health: i32) { BOSS_HEALTH.with(|slot| slot.set(Some(health))); }
pub(crate) fn take_boss_plan(width: i32, font_width: i32) -> Option<BossPlan> {
    BOSS_HEALTH.with(|slot| slot.take()).map(|health| boss_plan(width, font_width, health))
}
pub fn boss_overlay(ui: &mut crate::gui::Ui, z: f32) {
    let Some(plan) = take_boss_plan(ui.scaled().width, ui.string_width("Boss health")) else { return; };
    for [x, y, u, v, width, height] in plan.rectangles {
        ui.draw_textured_modal_rect_z(x, y, u, v, width, height, z);
    }
    let [x, y, color] = plan.text;
    ui.draw_string("Boss health", x, y, color);
    ui.gl.color4f(1.0, 1.0, 1.0, 1.0);
    let icons = ui.texture("gui/icons.png");
    ui.gl.bind_texture(icons);
}

pub fn dragon_beam(
    g: &mut Fixed,
    s: &RenderState,
    camera: [f64; 3],
    crystal: [f64; 3],
    rotation: i32,
) {
    let age = rotation as f32 + s.partial;
    let bob = math::sin(age * 0.2) / 2.0 + 0.5;
    let bob = (bob * bob + bob) * 0.2;
    let delta: [f32; 3] = std::array::from_fn(|i| {
        let position = if i == 1 {
            bob as f64 + crystal[i] - 1.0
        } else {
            crystal[i]
        };
        (position
            - s.current_position[i]
            - (s.previous_position[i] - s.current_position[i]) * (1.0 - s.partial) as f64)
            as f32
    });
    let [x, y, z] = delta;
    let horizontal = ((x * x + z * z) as f64).sqrt() as f32;
    let distance = ((x * x + y * y + z * z) as f64).sqrt() as f32;
    g.push();
    g.translate(
        (s.position[0] - camera[0]) as f32,
        (s.position[1] - camera[1]) as f32 + 2.0,
        (s.position[2] - camera[2]) as f32,
    );
    g.rotate(
        (-worldgen::jmath::java_atan2(z as f64, x as f64)) as f32 * 180.0 / PI_F32 - 90.0,
        0.0,
        1.0,
        0.0,
    );
    g.rotate(
        (-worldgen::jmath::java_atan2(horizontal as f64, y as f64)) as f32 * 180.0 / PI_F32 - 90.0,
        1.0,
        0.0,
        0.0,
    );
    g.enable(1, false);
    g.culling = false;
    g.texture = 73;
    let age = s.ticks as f32 + s.partial;
    let v0 = 0.0 - age * 0.01;
    let v1 = distance / 32.0 - age * 0.01;
    g.mode = 5;
    g.begin();
    for i in 0..=8 {
        let angle = (i % 8) as f32 * PI_F32 * 2.0 / 8.0;
        let x = math::sin(angle) * 0.75;
        let y = math::cos(angle) * 0.75;
        let u = (i % 8) as f32 * 1.0 / 8.0;
        g.vertex_color = Some(0xff00_0000);
        g.vertex(
            [(x * 0.2) as f64, (y * 0.2) as f64, 0.0],
            [u as f64, v1 as f64],
        );
        g.vertex_color = Some(0xffff_ffff);
        g.vertex([x as f64, y as f64, distance as f64], [u as f64, v0 as f64]);
    }
    g.end();
    g.mode = 7;
    g.culling = true;
    g.enable(1, true);
    g.pop();
}

pub fn death_rays(g: &mut Fixed, ticks: i32, partial: f32) {
    let progress = (ticks as f32 + partial) / 200.0;
    let fade = if progress > 0.8 {
        (progress - 0.8) / 0.2
    } else {
        0.0
    };
    let mut random = JavaRandom::new(432);
    g.enable(1, false);
    g.enable(8, false);
    g.enable(16, true);
    g.enable(32, false);
    g.src = 770;
    g.dst = 1;
    g.culling = true;
    g.push();
    g.translate(0.0, -1.0, -2.0);
    let mut i = 0;
    while (i as f32) < (progress + progress * progress) / 2.0 * 60.0 {
        for (n, axis) in [
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ]
        .into_iter()
        .enumerate()
        {
            let mut angle = random.next_float() * 360.0;
            if n == 5 {
                angle += progress * 90.0;
            }
            g.rotate(angle, axis[0], axis[1], axis[2]);
        }
        g.mode = 6;
        g.begin();
        let height = random.next_float() * 20.0 + 5.0 + fade * 10.0;
        let radius = random.next_float() * 2.0 + 1.0 + fade * 2.0;
        let alpha = (255.0 * (1.0 - fade)) as i32;
        g.vertex_color = Some(0x00ff_ffff | ((alpha.clamp(0, 255) as u32) << 24));
        g.vertex([0.0; 3], [0.0; 2]);
        g.vertex_color = Some(0x00ff_00ff);
        for xyz in [
            [
                -0.866 * radius as f64,
                height as f64,
                (-0.5 * radius) as f64,
            ],
            [0.866 * radius as f64, height as f64, (-0.5 * radius) as f64],
            [0.0, height as f64, (1.0 * radius) as f64],
            [
                -0.866 * radius as f64,
                height as f64,
                (-0.5 * radius) as f64,
            ],
        ] {
            g.vertex(xyz, [0.0; 2]);
        }
        g.untextured();
        g.end();
        i += 1;
    }
    g.pop();
    g.culling = false;
    g.mode = 7;
    g.enable(32, true);
    g.enable(16, false);
    g.material = [1.0; 4];
    g.enable(8, true);
    g.enable(1, true);
}

/// RenderEnchantmentTable's presentation snapshot; the tile's animation lives in worldgen10.
#[derive(Clone, Copy, Debug)]
pub struct BookState {
    pub ticks: i32,
    pub rotation: [f32; 2],
    pub pages: [f32; 2],
    pub opening: [f32; 2],
}
pub fn book(state: BookState, relative: [f64; 3], partial: f32) -> Fixed {
    let mut g = Fixed::new(1.0);
    g.texture = 75;
    g.push();
    g.translate(
        relative[0] as f32 + 0.5,
        relative[1] as f32 + 0.75,
        relative[2] as f32 + 0.5,
    );
    let age = state.ticks as f32 + partial;
    g.translate(0.0, 0.1 + math::sin(age * 0.1) * 0.01, 0.0);
    let mut turn = state.rotation[1] - state.rotation[0];
    while turn >= PI_F32 {
        turn -= PI_F32 * 2.0;
    }
    while turn < -PI_F32 {
        turn += PI_F32 * 2.0;
    }
    let angle = state.rotation[0] + turn * partial;
    g.rotate(-angle * 180.0 / PI_F32, 0.0, 1.0, 0.0);
    g.rotate(80.0, 0.0, 0.0, 1.0);
    let pages = state.pages[0] + (state.pages[1] - state.pages[0]) * partial;
    let page = |offset: f32| {
        let v = pages + offset;
        ((v - (v as f64).floor() as f32) * 1.6 - 0.3).clamp(0.0, 1.0)
    };
    let opening = state.opening[0] + (state.opening[1] - state.opening[0]) * partial;
    let mut m = Model::new(ModelKind::Book);
    m.set_rotation_angles([age, page(0.25), page(0.75), opening, 0.0, 0.0625]);
    special::model(&mut g, &m);
    g.pop();
    g
}

#[derive(Clone, Copy, Debug)]
pub struct BookTile {
    pub position: [i32; 3],
    pub light: u32,
    pub state: BookState,
}
impl From<&worldgen::container::enchanting::EnchantmentTable> for BookState {
    fn from(tile: &worldgen::container::enchanting::EnchantmentTable) -> Self {
        Self {
            ticks: tile.ticks,
            rotation: [tile.previous_rotation, tile.rotation],
            pages: [tile.previous_page, tile.page],
            opening: [tile.previous_open, tile.open],
        }
    }
}
