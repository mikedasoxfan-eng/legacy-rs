//! The GUI half of `Minecraft`: the current screen, `displayGuiScreen`, input routing and
//! drawing a frame (`EntityRenderer.updateCameraAndRender`'s screen part).

use super::font::FontRenderer;
use super::gl::Gl;
use super::render::GuiRenderer;
use super::screens::main_menu::GuiMainMenu;
use super::{Action, Screen, Ui, set_world_and_resolution};
use crate::render::gpu::Image;
use std::collections::HashMap;

/// What the game itself has to act on.
pub enum GameAction {
    StartWorld {
        folder: String,
        name: String,
        seed: Option<i64>,
        settings: Option<worldgen::entity::player::food::WorldSettings>,
    },
    QuitWorld,
    Shutdown,
    SettingsChanged,
    RefreshTextures,
    /// `GuiGameOver` "Respawn".
    Respawn,
    /// No screen any more: give the mouse back to the game.
    IngameFocus,
    /// A screen opened over the game: release the mouse.
    IngameUnfocus,
}

pub struct GuiSystem {
    pub ui: Ui,
    pub renderer: GuiRenderer,
    pub screen: Option<Box<dyn Screen>>,
    icon_keys: Vec<(String, bool)>,
    icon_slots: Vec<super::gl::TexId>,
    pub texture_anaglyph: bool,
    /// `GuiMainMenu`'s static `Random`.
    pub splash_rand: worldgen::JavaRandom,
    /// `Minecraft.guiAchievement`
    pub toast: super::achievement_toast::AchievementToast,
    /// `displayDebugInfo`'s frame-time graph (F3).
    pub frame_graph: super::frame_graph::FrameGraph,
    /// Nanoseconds spent ticking this frame (Minecraft.run's tick loop), for the graph.
    pub tick_nanos: i64,
}

pub struct Resources {
    pub images: HashMap<String, Image>,
    pub font_txt: String,
    pub lang: super::lang::Lang,
    pub splashes: Vec<String>,
}

impl GuiSystem {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        res: Resources,
        settings: super::settings::GameSettings,
        display: (i32, i32),
        saves_dir: std::path::PathBuf,
        splash_seed: i64,
        samples: u32,
    ) -> GuiSystem {
        let mut renderer = GuiRenderer::new(device, queue, format, samples);
        let mut textures = HashMap::new();
        for (name, img) in &res.images {
            let (_, blur, clamp) = super::texture_options(name);
            let id = renderer.add_texture_with(device, queue, img, blur, clamp);
            textures.insert(name.clone(), id);
        }
        let empty = Image { width: 256, height: 256, rgba: vec![0; 256 * 256 * 4] };
        textures.insert("panorama_snapshot".into(), renderer.add_texture_with(device, queue, &empty, true, false));
        let font_tex = textures["font/default.png"];
        let font = FontRenderer::new_with_anaglyph(&res.images["font/default.png"], font_tex, &res.font_txt, settings.anaglyph);
        let packs = crate::texture_packs::TexturePackList::new(saves_dir.parent().unwrap_or(std::path::Path::new(".")), &settings.skin);
        let ui = Ui {
            packs,
            pack_icons: Vec::new(),
            gl: Gl::new(),
            font,
            textures,
            lang: res.lang,
            settings,
            display_width: display.0,
            display_height: display.1,
            world_open: false,
            hide_quit_button: false,
            splashes: res.splashes,
            saves_dir,
            actions: Vec::new(),
            millis: 0,
            shift_down: false,
            fixed_date: false,
            clipboard: None,
            mouse_down: false,
            world_saved: true,
            player_dead: false,
            player_score: 0,
            save_requests: Vec::new(),
            sounds: Vec::new(),
            stats: Default::default(),
            container_snapshot: None,
            container_commands: Vec::new(),
            sign_commands: Vec::new(),
            inventory_armor_state: Default::default(),
            inventory_redirect_pending: false,
        };
        GuiSystem {
            ui,
            renderer,
            screen: None,
            icon_keys: Vec::new(), icon_slots: Vec::new(),
            texture_anaglyph: false,
            splash_rand: worldgen::JavaRandom::new(splash_seed),
            toast: Default::default(),
            frame_graph: Default::default(),
            tick_nanos: 0,
        }
    }

    pub fn sync_pack_icons(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let keys = self.ui.packs.available.iter().map(|p| (p.key.clone(), self.ui.settings.anaglyph)).collect::<Vec<_>>();
        if self.icon_keys == keys { return; }
        self.icon_keys = keys;
        self.ui.pack_icons.clear();
        for (i, p) in self.ui.packs.available.iter().enumerate() {
            let id = if let Some(img) = &p.thumbnail {
                let mut img = Image { width: img.width, height: img.height, rgba: img.rgba.clone() };
                if self.ui.settings.anaglyph { crate::texture_packs::anaglyph(&mut img.rgba); }
                if let Some(&id) = self.icon_slots.get(i) {
                    self.renderer.replace_texture(device, queue, id, &img, false, false);
                    id
                } else {
                    let id = self.renderer.add_texture(device, queue, &img);
                    // Keep one reusable slot per list position (including gaps for missing icons).
                    while self.icon_slots.len() < i { self.icon_slots.push(self.renderer.add_texture(device, queue, &Image { width: 1, height: 1, rgba: vec![255;4] })); }
                    self.icon_slots.push(id);
                    id
                }
            } else { self.ui.texture("gui/unknown_pack.png") };
            self.ui.pack_icons.push(id);
        }
    }

    pub fn refresh_textures(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<(), String> {
        self.texture_anaglyph = self.ui.settings.anaglyph;
        let mut jar = crate::assets::Jar::with_pack(self.ui.packs.path())?;
        for (name, id) in &self.ui.textures {
            if name.starts_with("panorama_") { continue; }
            // FontRenderer allocates its image directly from the jar, so pack fonts are ignored.
            let (path, blur, clamp) = super::texture_options(name);
            let image = if path == "font/default.png" { crate::assets::Jar::open()?.image(path) } else { jar.image(path) };
            let mut img = match image { Ok(img) => img, Err(e) => { eprintln!("refreshTextures {path}: {e}"); continue; } };
            if self.ui.settings.anaglyph { crate::texture_packs::anaglyph(&mut img.rgba); }
            self.renderer.replace_texture(device, queue, *id, &img, blur, clamp);
        }
        self.sync_pack_icons(device, queue);
        Ok(())
    }

    pub fn main_menu(&mut self) -> Box<dyn Screen> {
        Box::new(GuiMainMenu::new(&self.ui, &mut self.splash_rand))
    }

    /// `Minecraft.displayGuiScreen`
    pub fn display_gui_screen(&mut self, s: Option<Box<dyn Screen>>, out: &mut Vec<GameAction>) {
        if let Some(cur) = &mut self.screen {
            cur.on_gui_closed(&mut self.ui);
        }
        self.ui.stats.sync();
        let mut s = s;
        if s.is_none() && !self.ui.world_open {
            s = Some(self.main_menu());
        } else if s.is_none() && self.ui.player_dead {
            s = Some(Box::new(crate::gui::screens::game_over::GuiGameOver::new()));
        }
        if s.as_ref().is_some_and(|s| s.is_main_menu()) {
            self.ui.settings.show_debug_info = false;
        }
        match s {
            Some(mut s) => {
                out.push(GameAction::IngameUnfocus);
                let sr = self.ui.scaled();
                set_world_and_resolution(s.as_mut(), &mut self.ui, sr.width, sr.height);
                self.screen = Some(s);
            }
            None => {
                self.screen = None;
                out.push(GameAction::IngameFocus);
            }
        }
    }

    /// Apply what screens asked for; return what the game must do.
    pub fn process_actions(&mut self) -> Vec<GameAction> {
        let mut out = Vec::new();
        while !self.ui.actions.is_empty() {
            let acts: Vec<Action> = self.ui.actions.drain(..).collect();
            for a in acts {
                match a {
                    Action::Display(s) => self.display_gui_screen(s, &mut out),
                    Action::DisplayChild(f) => {
                        if let Some(cur) = self.screen.take() {
                            let mut cur = cur;
                            cur.on_gui_closed(&mut self.ui);
                            let child = f(cur);
                            // on_gui_closed already ran for the parent.
                            self.screen = None;
                            self.display_gui_screen(Some(child), &mut out);
                        }
                    }
                    Action::StartWorld { folder, name, seed, settings } => {
                        out.push(GameAction::StartWorld { folder, name, seed, settings })
                    }
                    Action::QuitWorld => out.push(GameAction::QuitWorld),
                    Action::Shutdown => out.push(GameAction::Shutdown),
                    Action::SettingsChanged => out.push(GameAction::SettingsChanged),
                    Action::RefreshTextures => out.push(GameAction::RefreshTextures),
                    Action::Respawn => out.push(GameAction::Respawn),
                    Action::RedirectInventory => {
                        if let Some(old) = self.ui.container_snapshot.clone() {
                            use worldgen::container::{Container, ContainerKind, ContainerCommand};
                            let kind = if old.creative { ContainerKind::Creative } else { ContainerKind::Player };
                            let mut snapshot = Container::new(kind, 0).snapshot(&old.player_inventory);
                            // The old GuiContainer close clears the carried stack before
                            // the replacement screen can draw; drops stay world-owned.
                            snapshot.cursor = None;
                            snapshot.player_inventory.cursor = None;
                            snapshot.preview = old.preview;
                            snapshot.alive = old.alive;
                            self.ui.container_snapshot = Some(snapshot.clone());
                            self.add_stat(worldgen::entity::player::stat::OPEN_INVENTORY, 1);
                            self.display_gui_screen(Some(Box::new(super::screens::container::GuiContainer::new(&snapshot))), &mut out);
                            self.ui.inventory_redirect_pending = true;
                            self.ui.container_commands.push(ContainerCommand::OpenInventory);
                        }
                    }
                }
            }
        }
        out
    }

    /// Mouse position in GUI coordinates (`Mouse.getX() * width / displayWidth`, flipped).
    pub fn gui_mouse(&self, px: f64, py: f64) -> (i32, i32) {
        let sr = self.ui.scaled();
        // LWJGL's mouse y counts up from the bottom of the window.
        let lx = px.floor() as i32;
        let ly = self.ui.display_height - 1 - py.floor() as i32;
        let x = lx * sr.width / self.ui.display_width;
        let y = sr.height - ly * sr.height / self.ui.display_height - 1;
        (x, y)
    }

    pub fn mouse_event(&mut self, x: i32, y: i32, button: i32, pressed: bool) {
        if let Some(s) = &mut self.screen {
            if pressed {
                s.mouse_clicked(&mut self.ui, x, y, button);
            } else {
                s.mouse_moved_or_up(&mut self.ui, x, y, button);
            }
        }
    }

    pub fn key_event(&mut self, c: char, key: i32) {
        if let Some(s) = &mut self.screen {
            s.key_typed(&mut self.ui, c, key);
        }
    }

    pub fn update_screen(&mut self) {
        if let Some(s) = &mut self.screen {
            s.update_screen(&mut self.ui);
        }
    }

    /// Record the current screen's drawing (no world: the menu path of updateCameraAndRender).
    pub fn draw_screen(&mut self, mx: i32, my: i32, partial: f32) {
        self.ui.setup_overlay_rendering();
        if let Some(s) = &mut self.screen {
            s.draw_screen(&mut self.ui, mx, my, partial);
        }
    }

    pub fn resize(&mut self, w: i32, h: i32) {
        self.ui.display_width = w.max(1);
        self.ui.display_height = h.max(1);
        if let Some(mut s) = self.screen.take() {
            let sr = self.ui.scaled();
            set_world_and_resolution(s.as_mut(), &mut self.ui, sr.width, sr.height);
            self.screen = Some(s);
        }
    }

    /// `Minecraft.loadScreen`: the Mojang logo shown while the game starts.
    pub fn draw_mojang_logo(&mut self) {
        let sr = self.ui.scaled();
        let (dw, dh) = (self.ui.display_width, self.ui.display_height);
        let t = self.ui.texture("title/mojang.png");
        let gl = &mut self.ui.gl;
        gl.ortho(0.0, sr.width_d, sr.height_d, 0.0, 1000.0, 3000.0);
        gl.load_identity();
        gl.translatef(0.0, 0.0, -2000.0);
        gl.texture_2d = true;
        gl.bind_texture(t);
        gl.start_drawing_quads();
        gl.set_color_opaque_i(16777215);
        gl.add_vertex_with_uv(0.0, dh as f64, 0.0, 0.0, 0.0);
        gl.add_vertex_with_uv(dw as f64, dh as f64, 0.0, 0.0, 0.0);
        gl.add_vertex_with_uv(dw as f64, 0.0, 0.0, 0.0, 0.0);
        gl.add_vertex_with_uv(0.0, 0.0, 0.0, 0.0, 0.0);
        gl.draw();
        gl.color4f(1.0, 1.0, 1.0, 1.0);
        gl.set_color_opaque_i(16777215);
        let (w, h) = (256, 256);
        let (x, y) = ((sr.width - w) / 2, (sr.height - h) / 2);
        self.ui.draw_textured_modal_rect(x, y, 0, 0, w, h);
        let gl = &mut self.ui.gl;
        gl.alpha_test = true;
        gl.alpha_ref = 0.1;
    }
}

impl GuiSystem {
    /// Minecraft.run after the frame: the F3 frame graph, then the achievement window.
    pub fn draw_after_frame(&mut self) {
        if self.ui.settings.show_debug_info {
            let ticks = self.tick_nanos;
            self.frame_graph.draw(&mut self.ui, ticks);
        } else {
            self.frame_graph.skip();
        }
        self.toast.draw(&mut self.ui);
    }
}
