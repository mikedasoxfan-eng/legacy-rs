//! `GuiMainMenu`

use crate::gui::widgets::GuiButton;
use crate::gui::{Action, Screen, ScreenBase, Ui};
use worldgen::math::{self, PI_F32};

pub struct GuiMainMenu {
    base: ScreenBase,
    update_counter: f32,
    pub splash_text: String,
}

impl GuiMainMenu {
    /// `rand` stands in for the class's static unseeded `Random`.
    pub fn new(ui: &Ui, rand: &mut worldgen::JavaRandom) -> GuiMainMenu {
        let splash_text = if ui.splashes.is_empty() {
            "missingno".to_string()
        } else {
            ui.splashes[rand.next_int(ui.splashes.len() as i32) as usize].clone()
        };
        GuiMainMenu { base: ScreenBase::default(), update_counter: 0.0, splash_text }
    }

    pub fn with_splash(splash: &str) -> GuiMainMenu {
        GuiMainMenu { base: ScreenBase::default(), update_counter: 0.0, splash_text: splash.to_string() }
    }
}

/// Today's (month, day) from the system clock (UTC).
fn month_day() -> (u32, u32) {
    let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs()) as i64;
    let days = secs.div_euclid(86400);
    // Civil-from-days (Howard Hinnant).
    let z = days + 719468;
    let era = z.div_euclid(146097);
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (m, d)
}

impl Screen for GuiMainMenu {
    fn base(&mut self) -> &mut ScreenBase {
        &mut self.base
    }

    fn is_main_menu(&self) -> bool {
        true
    }

    fn update_screen(&mut self, _ui: &mut Ui) {
        self.update_counter += 1.0;
    }

    fn key_typed(&mut self, _ui: &mut Ui, _c: char, _key: i32) {}

    fn init_gui(&mut self, ui: &mut Ui) {
        if !ui.fixed_date {
            match month_day() {
                (11, 9) => self.splash_text = "Happy birthday, ez!".into(),
                (6, 1) => self.splash_text = "Happy birthday, Notch!".into(),
                (12, 24) => self.splash_text = "Merry X-mas!".into(),
                (1, 1) => self.splash_text = "Happy new year!".into(),
                _ => {}
            }
        }
        let (w, h) = (self.base.width, self.base.height);
        let y = h / 4 + 48;
        let b = &mut self.base.buttons;
        b.push(GuiButton::new(1, w / 2 - 100, y, ui.translate("menu.singleplayer")));
        let mut mp = GuiButton::new(2, w / 2 - 100, y + 24, ui.translate("menu.multiplayer"));
        b.push(GuiButton::new(3, w / 2 - 100, y + 48, ui.translate("menu.mods")));
        if ui.hide_quit_button {
            b.push(GuiButton::new(0, w / 2 - 100, y + 72, ui.translate("menu.options")));
        } else {
            b.push(GuiButton::sized(0, w / 2 - 100, y + 72 + 12, 98, 20, ui.translate("menu.options")));
            b.push(GuiButton::sized(4, w / 2 + 2, y + 72 + 12, 98, 20, ui.translate("menu.quit")));
        }
        // Singleplayer only: there is no session to join servers with (mc.session == null).
        mp.enabled = false;
        b.insert(1, mp);
    }

    fn action_performed(&mut self, ui: &mut Ui, id: i32) {
        match id {
            0 => ui.display_child(|me| Box::new(super::options::GuiOptions::new(me))),
            1 => ui.display_child(|me| Box::new(super::select_world::GuiSelectWorld::new(me))),
            3 => ui.display_child(|me| Box::new(super::texture_packs::GuiTexturePacks::new(me))),
            4 => ui.actions.push(Action::Shutdown),
            _ => {}
        }
    }

    fn draw_screen(&mut self, ui: &mut Ui, mx: i32, my: i32, partial: f32) {
        let (w, h) = (self.base.width, self.base.height);
        ui.draw_world_background(w, h, 0);
        let logo_w = 274;
        let x = w / 2 - logo_w / 2;
        let y = 30;
        let t = ui.texture("title/mclogo.png");
        ui.gl.bind_texture(t);
        ui.gl.color4f(1.0, 1.0, 1.0, 1.0);
        ui.draw_textured_modal_rect(x, y, 0, 0, 155, 44);
        ui.draw_textured_modal_rect(x + 155, y, 0, 45, 155, 44);
        ui.gl.set_color_opaque_i(16777215);
        ui.gl.push_matrix();
        ui.gl.translatef((w / 2 + 90) as f32, 70.0, 0.0);
        ui.gl.rotatef(-20.0, 0.0, 0.0, 1.0);
        let mut s = 1.8f32 - (math::sin((ui.millis % 1000) as f32 / 1000.0 * PI_F32 * 2.0) * 0.1).abs();
        s = s * 100.0 / (ui.string_width(&self.splash_text) + 32) as f32;
        ui.gl.scalef(s, s, s);
        let splash = self.splash_text.clone();
        ui.draw_centered_string(&splash, 0, -8, 16776960);
        ui.gl.pop_matrix();
        ui.draw_string("Minecraft Beta 1.7.3", 2, 2, 5263440);
        let c = "Copyright Mojang AB. Do not distribute.";
        let cw = ui.string_width(c);
        ui.draw_string(c, w - cw - 2, h - 10, 16777215);
        let _ = partial;
        self.draw_buttons(ui, mx, my);
    }
}
