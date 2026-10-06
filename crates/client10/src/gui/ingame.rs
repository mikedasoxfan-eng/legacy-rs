//! `GuiIngame.renderGameOverlay`: vignette, portal overlay, hotbar, crosshair, hearts,
//! armor, air bubbles, the sleep fade, the F3 debug text, record and chat lines.

use super::gl::{Factor, Shade};
use super::Ui;

/// The player state the overlay reads (`EntityPlayerSP` / `InventoryPlayer`).
#[derive(Clone, Debug)]
pub struct HudPlayer {
    pub food_level: i32,
    pub food_saturation: f32,
    pub xp_points: f32,
    pub xp_level: i32,
    pub seed: i64,
    pub poison: bool,
    pub hunger: bool,
    pub regeneration: bool,
    pub nausea: bool,
    pub health: i32,
    pub prev_health: i32,
    /// `heartsLife` (damage flash countdown)
    pub hearts_life: i32,
    /// `getPlayerArmorValue`
    pub armor: i32,
    /// `inventory.currentItem`
    pub current_item: i32,
    pub air: i32,
    /// `isInsideOfMaterial(Material.water)`
    pub head_in_water: bool,
    /// `getEntityBrightness(partialTicks)`
    pub brightness: f32,
    pub prev_time_in_portal: f32,
    pub time_in_portal: f32,
    /// `func_22060_M`: sleep timer
    pub sleep_timer: i32,
    pub pos: [f64; 3],
    pub yaw: f32,
    /// `playerController.shouldDrawHUD()` (survival)
    pub survival: bool,
    /// `inventory.armorItemInSlot(3)`: the helmet's item id.
    pub helmet: Option<i32>,
    /// `inventory.mainInventory[0..9]` with each stack's `animationsToGo`.
    pub hotbar: [Option<(worldgen::world::tile::ItemStack, i32)>; 9],
}

impl Default for HudPlayer {
    /// A player who has just spawned: full health, empty inventory.
    fn default() -> Self {
        HudPlayer {
            food_level: 20,
            food_saturation: 5.0,
            xp_points: 0.0,
            xp_level: 0,
            seed: 0,
            poison: false,
            hunger: false,
            regeneration: false,
            nausea: false,
            health: 20,
            prev_health: 20,
            hearts_life: 0,
            armor: 0,
            current_item: 0,
            air: 300,
            head_in_water: false,
            brightness: 1.0,
            prev_time_in_portal: 0.0,
            time_in_portal: 0.0,
            sleep_timer: 0,
            pos: [0.0; 3],
            yaw: 0.0,
            survival: true,
            helmet: None,
            hotbar: [None; 9],
        }
    }
}

/// The F3 lines that come from elsewhere in the client.
#[derive(Clone, Debug, Default)]
pub struct DebugInfo {
    /// `mc.debug`: "N fps, M chunk updates"
    pub fps_line: String,
    /// `RenderGlobal.getDebugInfoRenders`
    pub renders: String,
    /// `RenderGlobal.getDebugInfoEntities`
    pub entities: String,
    /// "P: particles. T: All: entities"
    pub particles: String,
    /// `World.func_21119_g` (chunk provider)
    pub chunks: String,
    pub max_memory: i64,
    pub total_memory: i64,
    pub free_memory: i64,
}

#[derive(Clone)]
struct ChatLine {
    message: String,
    update_counter: i32,
}

pub struct GuiIngame {
    chat: Vec<ChatLine>,
    update_counter: i32,
    record_playing: String,
    record_playing_up_for: i32,
    record_rainbow: bool,
    prev_vignette_brightness: f32,
    rand: worldgen::JavaRandom,
}

impl Default for GuiIngame {
    fn default() -> Self {
        Self::new()
    }
}

impl GuiIngame {
    pub fn new() -> GuiIngame {
        GuiIngame {
            chat: Vec::new(),
            update_counter: 0,
            record_playing: String::new(),
            record_playing_up_for: 0,
            record_rainbow: false,
            prev_vignette_brightness: 1.0,
            rand: worldgen::JavaRandom::new(0),
        }
    }

    /// Tests: reset the vignette while keeping chat messages and the update counter.
    pub fn new_with_counter(o: &GuiIngame) -> GuiIngame {
        GuiIngame { chat: o.chat.clone(), update_counter: o.update_counter, ..GuiIngame::new() }
    }

    /// Tests: set `updateCounter` (seeds the low-health heart jitter).
    pub fn set_update_counter(&mut self, n: i32) {
        self.update_counter = n;
    }

    pub(crate) fn next_hud_random_long(&mut self) -> i64 { self.rand.next_long() }

    /// `updateTick`
    pub fn update_tick(&mut self) {
        if self.record_playing_up_for > 0 {
            self.record_playing_up_for -= 1;
        }
        self.update_counter += 1;
        for l in &mut self.chat {
            l.update_counter += 1;
        }
    }

    pub fn clear_chat_messages(&mut self) {
        self.chat.clear();
    }

    /// `addChatMessage`: wrap at 320 wide, newest first, keep 50.
    pub fn add_chat_message(&mut self, ui: &Ui, mut s: String) {
        loop {
            if ui.string_width(&s) <= 320 {
                break;
            }
            let cs: Vec<char> = s.chars().collect();
            let mut n = 1;
            while n < cs.len() && ui.string_width(&cs[..n + 1].iter().collect::<String>()) <= 320 {
                n += 1;
            }
            let head: String = cs[..n].iter().collect();
            self.add_chat_message(ui, head);
            s = cs[n..].iter().collect();
        }
        self.chat.insert(0, ChatLine { message: s, update_counter: 0, },);
        self.chat.truncate(50);
    }

    pub fn set_record_playing(&mut self, name: &str) {
        self.record_playing = format!("Now playing: {name}");
        self.record_playing_up_for = 60;
        self.record_rainbow = true;
    }

    #[allow(clippy::too_many_arguments)]
    pub fn render_game_overlay(
        &mut self,
        ui: &mut Ui,
        partial: f32,
        chat_open: bool,
        p: &HudPlayer,
        debug: &DebugInfo,
    ) {
        let sr = ui.scaled();
        let (w, h) = (sr.width, sr.height);
        ui.setup_overlay_rendering();
        ui.gl.blend = true;
        if ui.settings.fancy_graphics {
            self.render_vignette(ui, p.brightness, w, h);
        } else {
            ui.gl.blend_func(Factor::SrcAlpha, Factor::OneMinusSrcAlpha);
        }
        if ui.settings.third_person_view == 0 && p.helmet == Some(86) {
            render_pumpkin_blur(ui, w, h);
        }
        let portal = p.prev_time_in_portal + (p.time_in_portal - p.prev_time_in_portal) * partial;
        if portal > 0.0 && !p.nausea {
            render_portal_overlay(ui, portal, w, h);
        }

        ui.gl.color4f(1.0, 1.0, 1.0, 1.0);
        let gui_tex = ui.texture("gui/gui.png");
        ui.gl.bind_texture(gui_tex);
        let z = -90.0;
        ui.draw_textured_modal_rect_z(w / 2 - 91, h - 22, 0, 0, 182, 22, z);
        ui.draw_textured_modal_rect_z(w / 2 - 91 - 1 + p.current_item * 20, h - 22 - 1, 0, 22, 24, 22, z,);
        let icons = ui.texture("gui/icons.png");
        ui.gl.bind_texture(icons);
        ui.gl.blend = true;
        ui.gl.blend_func(Factor::OneMinusDstColor, Factor::OneMinusSrcColor);
        ui.draw_textured_modal_rect_z(w / 2 - 7, h / 2 - 7, 0, 0, 16, 16, z);
        ui.gl.blend = false;

        let mut flash = p.hearts_life / 3 % 2 == 1;
        if p.hearts_life < 10 {
            flash = false;
        }
        let health = p.health;
        let prev = p.prev_health;
        self.rand.set_seed(self.update_counter.wrapping_mul(312871) as i64);
        crate::render::entity::render_release::boss_overlay(ui, z);
        let heart_v = if ui.release.hardcore { 45 } else { 0 };
        if p.survival {
            let left = w / 2 - 91;
            let right = w / 2 + 91;
            let cap = 7i32.wrapping_add( p.xp_level.wrapping_mul(7) >> 1);
            if cap > 0 {
                let fill = ( p.xp_points * 183.0) as i32;
                ui.draw_textured_modal_rect_z(left, h - 29, 0, 64, 182, 5, z);
                if fill > 0 { ui.draw_textured_modal_rect_z(left, h - 29, 0, 69, fill, 5, z); }
            }
            let baseline = h - 39;
            let upper = baseline - 10;
            let armor = p.armor;
            let regen_index = if p.regeneration { self.update_counter % 25 } else { -1 };
            for i in 0..10 {
                if armor > 0 {
                    let x = left + i * 8;
                    let u = if i * 2 + 1 < armor { 34 } else if i * 2 + 1 == armor { 25 } else { 16 };
                    ui.draw_textured_modal_rect_z(x, upper, u, 9, 9, 9, z);
                }
                let heart = if p.poison { 52 } else { 16 };
                let f = if flash { 1 } else { 0 };
                let x = left + i * 8;
                let mut y = baseline;
                if health <= 4 { y += self.rand.next_int(2); }
                if i == regen_index { y -= 2; }
                ui.draw_textured_modal_rect_z(x, y, 16 + f * 9, heart_v, 9, 9, z);
                if flash {
                    if i * 2 + 1 < prev { ui.draw_textured_modal_rect_z(x, y, heart + 54, heart_v, 9, 9, z); }
                    if i * 2 + 1 == prev { ui.draw_textured_modal_rect_z(x, y, heart + 63, heart_v, 9, 9, z); }
                }
                if i * 2 + 1 < health { ui.draw_textured_modal_rect_z(x, y, heart + 36, heart_v, 9, 9, z); }
                if i * 2 + 1 == health { ui.draw_textured_modal_rect_z(x, y, heart + 45, heart_v, 9, 9, z); }
            }
            for i in 0..10 {
                let mut y = baseline;
                let food = if p.hunger { 52 } else { 16 };
                let empty = if p.hunger { 13 } else { 0 };
                if p.food_saturation <= 0.0 && self.update_counter % (p.food_level * 3 + 1) == 0 { y += self.rand.next_int(3) - 1; }
                let x = right - i * 8 - 9;
                ui.draw_textured_modal_rect_z(x, y, 16 + empty * 9, 27, 9, 9, z);
                if i * 2 + 1 < p.food_level { ui.draw_textured_modal_rect_z(x, y, food + 36, 27, 9, 9, z); }
                if i * 2 + 1 == p.food_level { ui.draw_textured_modal_rect_z(x, y, food + 45, 27, 9, 9, z); }
            }
            if p.head_in_water {
                let full = ((p.air - 2) as f64 * 10.0 / 300.0).ceil() as i32;
                let popping = (p.air as f64 * 10.0 / 300.0).ceil() as i32 - full;
                for i in 0..full + popping {
                    let u = if i < full { 16 } else { 25 };
                    ui.draw_textured_modal_rect_z(right - i * 8 - 9, upper, u, 18, 9, 9, z);
                }
            }
        }
        ui.gl.blend = false;
        // GL_RESCALE_NORMAL on, the item light rotated 120 degrees about X.
        crate::render::entity::gui_item::set_gui_release_lighting();
        crate::render::entity::gui_item::set_gui_rescale_normal(true);
        for i in 0..9 {
            let x = w / 2 - 90 + i * 20 + 2;
            let y = h - 16 - 3;
            render_inventory_slot(ui, p.hotbar[i as usize].as_ref(), x, y, partial);
        }
        crate::render::entity::gui_item::set_gui_rescale_normal(false);

        if p.sleep_timer > 0 {
            ui.gl.depth_test = false;
            ui.gl.alpha_test = false;
            let t = p.sleep_timer;
            let mut a = t as f32 / 100.0;
            if a > 1.0 {
                a = 1.0 - (t - 100) as f32 / 10.0;
            }
            let c = ((220.0 * a) as i32) << 24 | 1052704;
            ui.draw_rect(0, 0, w, h, c);
            ui.gl.alpha_test = true;
            ui.gl.depth_test = true;
        }

        if p.survival && p.xp_level > 0 {
            let text = p.xp_level.to_string();
            let x = (w - ui.string_width(&text)) / 2;
            let y = h - 35;
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                ui.font.draw_string(&mut ui.gl, &text, x + dx, y + dy, 0);
            }
            ui.font.draw_string(&mut ui.gl, &text, x, y, 8453920);
        }

        if ui.settings.show_debug_info {
            ui.gl.push_matrix();
            ui.draw_string(&format!("Minecraft 1.0.0 ({})", debug.fps_line), 2, 2, 16777215,);
            ui.draw_string(&debug.renders, 2, 12, 16777215);
            ui.draw_string(&debug.entities, 2, 22, 16777215);
            ui.draw_string(&debug.particles, 2, 32, 16777215);
            ui.draw_string(&debug.chunks, 2, 42, 16777215);
            let (max, total, free) = (debug.max_memory.max(1), debug.total_memory, debug.free_memory,);
            let used = total - free;
            let s = format!("Used memory: {}% ({}MB) of {}MB", used * 100 / max, used / 1024 / 1024, max / 1024 / 1024);
            let sw = ui.string_width(&s);
            ui.draw_string(&s, w - sw - 2, 2, 14737632);
            let s = format!("Allocated memory: {}% ({}MB)", total * 100 / max, total / 1024 / 1024);
            let sw = ui.string_width(&s);
            ui.draw_string(&s, w - sw - 2, 12, 14737632);
            ui.draw_string(&format!("x: {}", java_double(p.pos[0])), 2, 64, 14737632);
            ui.draw_string(&format!("y: {}", java_double(p.pos[1])), 2, 72, 14737632);
            ui.draw_string(&format!("z: {}", java_double(p.pos[2])), 2, 80, 14737632);
            let f = worldgen::math::floor_double((p.yaw * 4.0 / 360.0) as f64 + 0.5) & 3;
            ui.draw_string(&format!("f: {f}"), 2, 88, 14737632);
            ui.draw_string(&format!("Seed: {}", p.seed), 2, 104, 14737632);
            ui.gl.pop_matrix();
        }

        if self.record_playing_up_for > 0 {
            let t = self.record_playing_up_for as f32 - partial;
            let a = ((t * 256.0 / 20.0) as i32).min(255);
            if a > 0 {
                ui.gl.push_matrix();
                ui.gl.translatef((w / 2) as f32, (h - 48) as f32, 0.0);
                ui.gl.blend = true;
                ui.gl.blend_func(Factor::SrcAlpha, Factor::OneMinusSrcAlpha);
                let mut c = 16777215;
                if self.record_rainbow {
                    c = (crate::render::sky::hsb_to_rgb(t / 50.0, 0.7, 0.6) & 16777215) as i32;
                }
                let s = self.record_playing.clone();
                let sw = ui.string_width(&s);
                ui.font.draw_string(&mut ui.gl, &s, -sw / 2, -4, c + (a << 24));
                ui.gl.blend = false;
                ui.gl.pop_matrix();
            }
        }

        let max_lines = if chat_open { 20 } else { 10 };
        ui.gl.blend = true;
        ui.gl.blend_func(Factor::SrcAlpha, Factor::OneMinusSrcAlpha);
        ui.gl.alpha_test = false;
        ui.gl.push_matrix();
        ui.gl.translatef(0.0, (h - 48) as f32, 0.0);
        for (i, line) in self.chat.iter().enumerate().take(max_lines) {
            if line.update_counter < 200 || chat_open {
                let mut f = line.update_counter as f64 / 200.0;
                f = 1.0 - f;
                f *= 10.0;
                f = f.clamp(0.0, 1.0);
                f *= f;
                let mut a = (255.0 * f) as i32;
                if chat_open {
                    a = 255;
                }
                if a > 0 {
                    let x = 2;
                    let y = -(i as i32) * 9;
                    let msg = line.message.clone();
                    ui.draw_rect(x, y - 1, x + 320, y + 8, (a / 2) << 24);
                    ui.gl.blend = true;
                    ui.font.draw_string_with_shadow(&mut ui.gl, &msg, x, y, 16777215 + (a << 24));
                }
            }
        }
        ui.gl.pop_matrix();
        ui.gl.alpha_test = true;
        ui.gl.blend = false;
        let _ = Shade::Flat;
    }

    /// `renderVignette`: darken the screen edges by the player's light level (smoothed per
    /// frame, so it depends on the frame rate, as in the original).
    fn render_vignette(&mut self, ui: &mut Ui, brightness: f32, w: i32, h: i32) {
        let b = (1.0 - brightness).clamp(0.0, 1.0);
        self.prev_vignette_brightness =
            (self.prev_vignette_brightness as f64 + (b - self.prev_vignette_brightness) as f64 * 0.01) as f32;
        let t = ui.texture("%blur%/misc/vignette.png");
        let v = self.prev_vignette_brightness;
        let gl = &mut ui.gl;
        gl.depth_test = false;
        gl.depth_mask = false;
        gl.blend_func(Factor::Zero, Factor::OneMinusSrcColor);
        gl.color4f(v, v, v, 1.0);
        gl.bind_texture(t);
        gl.start_drawing_quads();
        gl.add_vertex_with_uv(0.0, h as f64, -90.0, 0.0, 1.0);
        gl.add_vertex_with_uv(w as f64, h as f64, -90.0, 1.0, 1.0);
        gl.add_vertex_with_uv(w as f64, 0.0, -90.0, 1.0, 0.0);
        gl.add_vertex_with_uv(0.0, 0.0, -90.0, 0.0, 0.0);
        gl.draw();
        gl.depth_mask = true;
        gl.depth_test = true;
        gl.color4f(1.0, 1.0, 1.0, 1.0);
        gl.blend_func(Factor::SrcAlpha, Factor::OneMinusSrcAlpha);
    }
}

/// `renderPumpkinBlur`: the carved-pumpkin view while wearing one.
fn render_pumpkin_blur(ui: &mut Ui, w: i32, h: i32) {
    let t = ui.texture("%blur%/misc/pumpkinblur.png");
    let gl = &mut ui.gl;
    gl.depth_test = false;
    gl.depth_mask = false;
    gl.blend_func(Factor::SrcAlpha, Factor::OneMinusSrcAlpha);
    gl.color4f(1.0, 1.0, 1.0, 1.0);
    gl.alpha_test = false;
    gl.bind_texture(t);
    gl.start_drawing_quads();
    gl.add_vertex_with_uv(0.0, h as f64, -90.0, 0.0, 1.0);
    gl.add_vertex_with_uv(w as f64, h as f64, -90.0, 1.0, 1.0);
    gl.add_vertex_with_uv(w as f64, 0.0, -90.0, 1.0, 0.0);
    gl.add_vertex_with_uv(0.0, 0.0, -90.0, 0.0, 0.0);
    gl.draw();
    gl.depth_mask = true;
    gl.depth_test = true;
    gl.alpha_test = true;
    gl.color4f(1.0, 1.0, 1.0, 1.0);
}

/// `renderPortalOverlay`: the portal texture over the screen while teleporting.
fn render_portal_overlay(ui: &mut Ui, mut t: f32, w: i32, h: i32) {
    if t < 1.0 {
        t *= t;
        t *= t;
        t = t * 0.8 + 0.2;
    }
    let terrain = ui.texture("terrain.png");
    let gl = &mut ui.gl;
    gl.alpha_test = false;
    gl.depth_test = false;
    gl.depth_mask = false;
    gl.blend_func(Factor::SrcAlpha, Factor::OneMinusSrcAlpha);
    gl.color4f(1.0, 1.0, 1.0, t);
    gl.bind_texture(terrain);
    // Block.portal.blockIndexInTexture = 14
    let idx = 14;
    let u0 = (idx % 16) as f32 / 16.0;
    let v0 = (idx / 16) as f32 / 16.0;
    let u1 = (idx % 16 + 1) as f32 / 16.0;
    let v1 = (idx / 16 + 1) as f32 / 16.0;
    gl.start_drawing_quads();
    gl.add_vertex_with_uv(0.0, h as f64, -90.0, u0 as f64, v1 as f64);
    gl.add_vertex_with_uv(w as f64, h as f64, -90.0, u1 as f64, v1 as f64);
    gl.add_vertex_with_uv(w as f64, 0.0, -90.0, u1 as f64, v0 as f64);
    gl.add_vertex_with_uv(0.0, 0.0, -90.0, u0 as f64, v0 as f64);
    gl.draw();
    gl.depth_mask = true;
    gl.depth_test = true;
    gl.alpha_test = true;
    gl.color4f(1.0, 1.0, 1.0, 1.0);
}

/// `Double.toString`: shortest round-trip digits, scientific below 1e-3 or from 1e7.
pub fn java_double(d: f64) -> String {
    if d.is_nan() {
        return "NaN".into();
    }
    if d.is_infinite() {
        return if d > 0.0 { "Infinity" } else { "-Infinity" }.into();
    }
    let a = d.abs();
    if a == 0.0 || (1e-3..1e7).contains(&a) {
        format!("{d:?}")
    } else {
        let s = format!("{d:e}");
        let (m, e) = s.split_once('e').unwrap();
        let m = if m.contains('.') { m.to_string() } else { format!("{m}.0") };
        format!("{m}E{e}")
    }
}

/// `GuiIngame.renderInventorySlot`: a stack just picked up bobs for `animationsToGo` ticks.
fn render_inventory_slot(
    ui: &mut Ui,
    slot: Option<&(worldgen::world::tile::ItemStack, i32)>,
    x: i32,
    y: i32,
    partial: f32,
) {
    use crate::render::entity::gui_item;
    let Some((item, animations)) = slot else {
        return;
    };
    let t = *animations as f32 - partial;
    if t > 0.0 {
        ui.gl.push_matrix();
        let s = 1.0 + t / 5.0;
        ui.gl.translatef((x + 8) as f32, (y + 12) as f32, 0.0);
        ui.gl.scalef(1.0 / s, (s + 1.0) / 2.0, 1.0);
        ui.gl.translatef(-(x + 8) as f32, -(y + 12) as f32, 0.0);
    }
    gui_item::render_item_into_gui(ui, Some(item), x, y);
    if t > 0.0 {
        ui.gl.pop_matrix();
    }
    gui_item::render_item_overlay_into_gui(ui, Some(item), x, y);
}
