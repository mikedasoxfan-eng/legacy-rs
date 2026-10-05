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
    },
    QuitWorld,
    Shutdown,
    SettingsChanged,
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
    /// `GuiMainMenu`'s static `Random`.
    pub splash_rand: worldgen::JavaRandom,
    /// `Minecraft.guiAchievement`
    pub toast: super::achievement_toast::AchievementToast,
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
        let font_tex = textures["font/default.png"];
        let font = FontRenderer::new(&res.images["font/default.png"], font_tex, &res.font_txt);
        let ui = Ui {
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
        };
        GuiSystem {
            ui,
            renderer,
            screen: None,
            splash_rand: worldgen::JavaRandom::new(splash_seed),
            toast: Default::default(),
        }
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
                    Action::StartWorld { folder, name, seed } => {
                        out.push(GameAction::StartWorld { folder, name, seed })
                    }
                    Action::QuitWorld => out.push(GameAction::QuitWorld),
                    Action::Shutdown => out.push(GameAction::Shutdown),
                    Action::SettingsChanged => out.push(GameAction::SettingsChanged),
                    Action::Respawn => out.push(GameAction::Respawn),
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
