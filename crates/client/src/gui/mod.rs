//! The game's 2D interface, ported from `Gui`, `GuiScreen`, `GuiButton`, `FontRenderer`,
//! `ScaledResolution` and the screens, drawing through a small fixed-function GL emulation
//! (`gl`) so the original drawing code can be followed line by line.

pub mod font;
pub mod gl;
pub mod ingame;
pub mod keynames;
pub mod lang;
pub mod loading;
pub mod render;
pub mod saves;
pub mod screens;
pub mod settings;
pub mod shot;
pub mod slot;
pub mod system;
pub mod widgets;

use crate::render::gpu::Image;
use font::FontRenderer;
use gl::{Factor, Gl, Shade, TexId};
use std::collections::HashMap;
use widgets::GuiButton;

/// Textures the GUI binds by name (`RenderEngine.getTexture("/gui/gui.png")`).
pub const TEXTURES: &[&str] = &[
    "font/default.png",
    "gui/gui.png",
    "gui/background.png",
    "gui/icons.png",
    "title/mclogo.png",
    "title/mojang.png",
    "pack.png",
    "terrain.png",
    "%blur%/misc/vignette.png",
];

/// A texture name without `RenderEngine`'s `%blur%` / `%clamp%` prefixes, and the flags.
pub fn texture_options(name: &str) -> (&str, bool, bool) {
    let mut n = name;
    let (mut blur, mut clamp) = (false, false);
    loop {
        if let Some(r) = n.strip_prefix("%blur%") {
            blur = true;
            n = r;
        } else if let Some(r) = n.strip_prefix("%clamp%") {
            clamp = true;
            n = r;
        } else {
            break;
        }
    }
    (n.trim_start_matches('/'), blur, clamp)
}

/// `ScaledResolution`
#[derive(Clone, Copy, Debug)]
pub struct ScaledResolution {
    pub width: i32,
    pub height: i32,
    pub width_d: f64,
    pub height_d: f64,
    pub scale_factor: i32,
}

impl ScaledResolution {
    pub fn new(gui_scale: i32, display_w: i32, display_h: i32) -> ScaledResolution {
        let mut scale = 1;
        let limit = if gui_scale == 0 { 1000 } else { gui_scale };
        while scale < limit && display_w / (scale + 1) >= 320 && display_h / (scale + 1) >= 240 {
            scale += 1;
        }
        let wd = display_w as f64 / scale as f64;
        let hd = display_h as f64 / scale as f64;
        ScaledResolution { width: wd.ceil() as i32, height: hd.ceil() as i32, width_d: wd, height_d: hd, scale_factor: scale }
    }
}

/// What a screen asks the game to do (applied after the event that caused it, since
/// screens can't hold the game mutably while it owns them).
pub enum Action {
    /// `mc.displayGuiScreen(screen)`; `None` = back to the game (or the main menu).
    Display(Option<Box<dyn Screen>>),
    /// `mc.displayGuiScreen(new Child(this, ...))`: the new screen keeps the current one as
    /// its parent to return to.
    DisplayChild(Box<dyn FnOnce(Box<dyn Screen>) -> Box<dyn Screen>>),
    /// Open or create a world from the world-selection screens.
    StartWorld { folder: String, name: String, seed: Option<i64> },
    /// `GuiIngameMenu` "Save and quit to title".
    QuitWorld,
    /// `mc.shutdown()`
    Shutdown,
    /// Settings changed (render distance, graphics...): the game should re-read them.
    SettingsChanged,
}

/// The parts of `Minecraft` the GUI code reaches into.
pub struct Ui {
    pub gl: Gl,
    pub font: FontRenderer,
    pub textures: HashMap<String, TexId>,
    pub lang: lang::Lang,
    pub settings: settings::GameSettings,
    pub display_width: i32,
    pub display_height: i32,
    /// `mc.theWorld != null`
    pub world_open: bool,
    /// `mc.hideQuitButton` (false when run stand-alone, as here).
    pub hide_quit_button: bool,
    pub splashes: Vec<String>,
    /// Saves directory for the world list.
    pub saves_dir: std::path::PathBuf,
    pub actions: Vec<Action>,
    /// Milliseconds clock for animations (`System.currentTimeMillis`), settable for tests.
    pub millis: i64,
    /// `Keyboard.isKeyDown(KEY_LSHIFT/RSHIFT)` etc. as needed by screens.
    pub shift_down: bool,
    /// Tests: ignore the date (the title screen's birthday/holiday splashes).
    pub fixed_date: bool,
    /// System clipboard text (`GuiScreen.getClipboardString`), filled in by the app.
    pub clipboard: Option<String>,
    /// `Mouse.isButtonDown(0)`
    pub mouse_down: bool,
    /// Last answer from the world thread to an incremental save (`World.func_650_a`).
    pub world_saved: bool,
    /// Incremental save requests made while drawing (the pause menu saves every frame).
    pub save_requests: Vec<i32>,
    /// `sndManager.playSoundFX(name, volume, pitch)` calls, played by the app.
    pub sounds: Vec<(&'static str, f32, f32)>,
}

impl Ui {
    pub fn texture(&self, name: &str) -> TexId {
        let key = if name.starts_with('%') { name } else { name.trim_start_matches('/') };
        *self.textures.get(key).unwrap_or_else(|| panic!("GUI texture {name} not loaded"))
    }

    pub fn translate(&self, key: &str) -> String {
        self.lang.translate_key(key)
    }

    pub fn scaled(&self) -> ScaledResolution {
        ScaledResolution::new(self.settings.gui_scale, self.display_width, self.display_height)
    }

    pub fn display(&mut self, s: Option<Box<dyn Screen>>) {
        self.actions.push(Action::Display(s));
    }

    pub fn display_child(&mut self, f: impl FnOnce(Box<dyn Screen>) -> Box<dyn Screen> + 'static) {
        self.actions.push(Action::DisplayChild(Box::new(f)));
    }

    /// `World.func_650_a(n)`: ask for a few chunks to be saved; true once nothing is left.
    pub fn quick_save(&mut self, n: i32) -> bool {
        if !self.world_open {
            return true;
        }
        self.save_requests.push(n);
        self.world_saved
    }

    /// `ChatAllowedCharacters.allowedCharacters.indexOf(c) >= 0`
    pub fn font_allows(&self, c: char) -> bool {
        self.font.allows(c)
    }

    /// `EntityRenderer.func_905_b` (setupOverlayRendering).
    pub fn setup_overlay_rendering(&mut self) {
        let sr = self.scaled();
        self.gl.ortho(0.0, sr.width_d, sr.height_d, 0.0, 1000.0, 3000.0);
        self.gl.load_identity();
        self.gl.translatef(0.0, 0.0, -2000.0);
    }

    // ------------------------------------------------------------------ Gui

    /// `Gui.drawRect`
    pub fn draw_rect(&mut self, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, color: i32) {
        if x0 < x1 {
            std::mem::swap(&mut x0, &mut x1);
        }
        if y0 < y1 {
            std::mem::swap(&mut y0, &mut y1);
        }
        let a = (color >> 24 & 255) as f32 / 255.0;
        let r = (color >> 16 & 255) as f32 / 255.0;
        let g = (color >> 8 & 255) as f32 / 255.0;
        let b = (color & 255) as f32 / 255.0;
        let gl = &mut self.gl;
        gl.blend = true;
        gl.texture_2d = false;
        gl.blend_func(Factor::SrcAlpha, Factor::OneMinusSrcAlpha);
        gl.color4f(r, g, b, a);
        gl.start_drawing_quads();
        gl.add_vertex(x0 as f64, y1 as f64, 0.0);
        gl.add_vertex(x1 as f64, y1 as f64, 0.0);
        gl.add_vertex(x1 as f64, y0 as f64, 0.0);
        gl.add_vertex(x0 as f64, y0 as f64, 0.0);
        gl.draw();
        gl.texture_2d = true;
        gl.blend = false;
    }

    /// `Gui.func_27100_a`: horizontal line.
    pub fn draw_horizontal_line(&mut self, mut x0: i32, mut x1: i32, y: i32, color: i32) {
        if x1 < x0 {
            std::mem::swap(&mut x0, &mut x1);
        }
        self.draw_rect(x0, y, x1 + 1, y + 1, color);
    }

    /// `Gui.func_27099_b`: vertical line.
    pub fn draw_vertical_line(&mut self, x: i32, mut y0: i32, mut y1: i32, color: i32) {
        if y1 < y0 {
            std::mem::swap(&mut y0, &mut y1);
        }
        self.draw_rect(x, y0 + 1, x + 1, y1, color);
    }

    /// `Gui.drawGradientRect`
    pub fn draw_gradient_rect(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, c0: i32, c1: i32) {
        let f = |c: i32, s: i32| (c >> s & 255) as f32 / 255.0;
        let gl = &mut self.gl;
        gl.texture_2d = false;
        gl.blend = true;
        gl.alpha_test = false;
        gl.blend_func(Factor::SrcAlpha, Factor::OneMinusSrcAlpha);
        gl.shade = Shade::Smooth;
        gl.start_drawing_quads();
        gl.set_color_rgba_f(f(c0, 16), f(c0, 8), f(c0, 0), f(c0, 24));
        gl.add_vertex(x1 as f64, y0 as f64, 0.0);
        gl.add_vertex(x0 as f64, y0 as f64, 0.0);
        gl.set_color_rgba_f(f(c1, 16), f(c1, 8), f(c1, 0), f(c1, 24));
        gl.add_vertex(x0 as f64, y1 as f64, 0.0);
        gl.add_vertex(x1 as f64, y1 as f64, 0.0);
        gl.draw();
        gl.shade = Shade::Flat;
        gl.blend = false;
        gl.alpha_test = true;
        gl.texture_2d = true;
    }

    pub fn draw_centered_string(&mut self, s: &str, x: i32, y: i32, color: i32) {
        let w = self.font.get_string_width(s);
        self.font.draw_string_with_shadow(&mut self.gl, s, x - w / 2, y, color);
    }

    pub fn draw_string(&mut self, s: &str, x: i32, y: i32, color: i32) {
        self.font.draw_string_with_shadow(&mut self.gl, s, x, y, color);
    }

    pub fn string_width(&self, s: &str) -> i32 {
        self.font.get_string_width(s)
    }

    /// `Gui.drawTexturedModalRect` at `zLevel`.
    pub fn draw_textured_modal_rect_z(&mut self, x: i32, y: i32, u: i32, v: i32, w: i32, h: i32, z: f32) {
        let s = 0.00390625f32;
        let gl = &mut self.gl;
        gl.start_drawing_quads();
        gl.add_vertex_with_uv((x + 0) as f64, (y + h) as f64, z as f64, ((u + 0) as f32 * s) as f64, ((v + h) as f32 * s) as f64);
        gl.add_vertex_with_uv((x + w) as f64, (y + h) as f64, z as f64, ((u + w) as f32 * s) as f64, ((v + h) as f32 * s) as f64);
        gl.add_vertex_with_uv((x + w) as f64, (y + 0) as f64, z as f64, ((u + w) as f32 * s) as f64, ((v + 0) as f32 * s) as f64);
        gl.add_vertex_with_uv((x + 0) as f64, (y + 0) as f64, z as f64, ((u + 0) as f32 * s) as f64, ((v + 0) as f32 * s) as f64);
        gl.draw();
    }

    pub fn draw_textured_modal_rect(&mut self, x: i32, y: i32, u: i32, v: i32, w: i32, h: i32) {
        self.draw_textured_modal_rect_z(x, y, u, v, w, h, 0.0);
    }

    // ------------------------------------------------------------------ GuiScreen

    /// `GuiScreen.drawWorldBackground`
    pub fn draw_world_background(&mut self, width: i32, height: i32, offset: i32) {
        if self.world_open {
            self.draw_gradient_rect(0, 0, width, height, -1072689136, -804253680);
        } else {
            self.draw_background(width, height, offset);
        }
    }

    /// `GuiScreen.drawBackground`: the tinted dirt.
    pub fn draw_background(&mut self, width: i32, height: i32, offset: i32) {
        let t = self.texture("gui/background.png");
        let gl = &mut self.gl;
        gl.bind_texture(t);
        gl.color4f(1.0, 1.0, 1.0, 1.0);
        let s = 32.0f32;
        gl.start_drawing_quads();
        gl.set_color_opaque_i(4210752);
        gl.add_vertex_with_uv(0.0, height as f64, 0.0, 0.0, (height as f32 / s + offset as f32) as f64);
        gl.add_vertex_with_uv(width as f64, height as f64, 0.0, (width as f32 / s) as f64, (height as f32 / s + offset as f32) as f64);
        gl.add_vertex_with_uv(width as f64, 0.0, 0.0, (width as f32 / s) as f64, (0 + offset) as f64);
        gl.add_vertex_with_uv(0.0, 0.0, 0.0, 0.0, (0 + offset) as f64);
        gl.draw();
    }
}

/// Shared `GuiScreen` fields.
#[derive(Default)]
pub struct ScreenBase {
    pub width: i32,
    pub height: i32,
    pub buttons: Vec<GuiButton>,
    selected: Option<usize>,
}

impl ScreenBase {
    pub fn button(&mut self, id: i32) -> &mut GuiButton {
        self.buttons.iter_mut().find(|b| b.id == id).expect("button id")
    }
}

/// A `GuiScreen` subclass. Default methods are the base-class behaviour.
pub trait Screen {
    fn base(&mut self) -> &mut ScreenBase;

    fn init_gui(&mut self, _ui: &mut Ui) {}

    fn draw_screen(&mut self, ui: &mut Ui, mx: i32, my: i32, _partial: f32) {
        self.draw_buttons(ui, mx, my);
    }

    /// `super.drawScreen`
    fn draw_buttons(&mut self, ui: &mut Ui, mx: i32, my: i32) {
        for b in self.base().buttons.iter_mut() {
            b.draw_button(ui, mx, my);
        }
    }

    fn update_screen(&mut self, _ui: &mut Ui) {}

    /// `keyTyped(char, LWJGL key code)`
    fn key_typed(&mut self, ui: &mut Ui, _c: char, key: i32) {
        if key == widgets::KEY_ESCAPE {
            ui.display(None);
            // mc.setIngameFocus() follows from displaying no screen.
        }
    }

    fn mouse_clicked(&mut self, ui: &mut Ui, x: i32, y: i32, button: i32) {
        default_mouse_clicked(self, ui, x, y, button);
    }

    /// Bookkeeping for `default_mouse_clicked`.
    fn set_selected(&mut self, i: Option<usize>) {
        self.base().selected = i;
    }

    fn mouse_moved_or_up(&mut self, _ui: &mut Ui, x: i32, y: i32, button: i32) {
        if button == 0 {
            if let Some(i) = self.base().selected.take() {
                if let Some(b) = self.base().buttons.get_mut(i) {
                    b.mouse_released(x, y);
                }
            }
        }
    }

    /// `GuiYesNo` calls its parent's `deleteWorld`; true if the parent wants to be shown.
    fn delete_world(&mut self, _ui: &mut Ui, _yes: bool, _index: i32) -> bool {
        false
    }

    fn action_performed(&mut self, _ui: &mut Ui, _id: i32) {}

    fn on_gui_closed(&mut self, _ui: &mut Ui) {}

    fn does_gui_pause_game(&self) -> bool {
        true
    }

    /// `instanceof GuiMainMenu` checks in `displayGuiScreen`.
    fn is_main_menu(&self) -> bool {
        false
    }

    /// `instanceof GuiIngameMenu`
    fn is_ingame_menu(&self) -> bool {
        false
    }
}

/// `GuiScreen.mouseClicked` (callable from overrides that call `super`).
pub fn default_mouse_clicked<S: Screen + ?Sized>(s: &mut S, ui: &mut Ui, x: i32, y: i32, button: i32) {
        if button == 0 {
            let mut i = 0;
            while i < s.base().buttons.len() {
                if !s.base().buttons[i].press(ui, x, y) {
                    i += 1;
                    continue;
                }
                s.set_selected(Some(i));
                ui.sounds.push(("random.click", 1.0, 1.0));
                let id = s.base().buttons[i].id;
                s.action_performed(ui, id);
                i += 1;
            }
        }
}

/// `GuiScreen.setWorldAndResolution`
pub fn set_world_and_resolution(s: &mut dyn Screen, ui: &mut Ui, w: i32, h: i32) {
    {
        let b = s.base();
        b.width = w;
        b.height = h;
        b.buttons.clear();
    }
    s.init_gui(ui);
}

/// Load the GUI's textures and text resources from the jar.
pub fn load_resources(jar: &mut crate::assets::Jar) -> Result<(HashMap<String, Image>, String, lang::Lang, Vec<String>), String> {
    let mut imgs = HashMap::new();
    for &t in TEXTURES {
        imgs.insert(t.to_string(), jar.image(texture_options(t).0)?);
    }
    let font_txt = jar.text("font.txt")?;
    let mut lang = lang::Lang::default();
    lang.load(&jar.bytes("lang/en_US.lang")?);
    lang.load(&jar.bytes("lang/stats_US.lang")?);
    let splashes = jar
        .text("title/splashes.txt")?
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();
    Ok((imgs, font_txt, lang, splashes))
}
