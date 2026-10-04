//! `GuiIngameMenu`

use crate::gui::widgets::GuiButton;
use crate::gui::{Action, Screen, ScreenBase, Ui};
use worldgen::math::{self, PI_F32};

pub struct GuiIngameMenu {
    base: ScreenBase,
    /// `updateCounter2`: frames drawn, passed to `func_650_a` (incremental save).
    pub save_counter: i32,
    update_counter: i32,
}

impl Default for GuiIngameMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl GuiIngameMenu {
    pub fn new() -> GuiIngameMenu {
        GuiIngameMenu { base: ScreenBase::default(), save_counter: 0, update_counter: 0 }
    }
}

impl Screen for GuiIngameMenu {
    fn base(&mut self) -> &mut ScreenBase {
        &mut self.base
    }

    fn is_ingame_menu(&self) -> bool {
        true
    }

    fn init_gui(&mut self, ui: &mut Ui) {
        self.save_counter = 0;
        self.base.buttons.clear();
        let (w, h) = (self.base.width, self.base.height);
        let o = -16;
        let b = &mut self.base.buttons;
        b.push(GuiButton::new(1, w / 2 - 100, h / 4 + 120 + o, "Save and quit to title"));
        b.push(GuiButton::new(4, w / 2 - 100, h / 4 + 24 + o, "Back to game"));
        b.push(GuiButton::new(0, w / 2 - 100, h / 4 + 96 + o, "Options..."));
        b.push(GuiButton::sized(5, w / 2 - 100, h / 4 + 48 + o, 98, 20, ui.translate("gui.achievements")));
        b.push(GuiButton::sized(6, w / 2 + 2, h / 4 + 48 + o, 98, 20, ui.translate("gui.stats")));
    }

    fn action_performed(&mut self, ui: &mut Ui, id: i32) {
        match id {
            0 => ui.display_child(|me| Box::new(super::options::GuiOptions::new(me))),
            1 => ui.actions.push(Action::QuitWorld),
            4 => ui.display(None),
            5 => ui.display(Some(Box::new(super::achievements::GuiAchievements::new()))),
            6 => ui.display_child(|me| Box::new(super::stats::GuiStats::new(me))),
            _ => {}
        }
    }

    fn update_screen(&mut self, _ui: &mut Ui) {
        self.update_counter += 1;
    }

    fn draw_screen(&mut self, ui: &mut Ui, mx: i32, my: i32, partial: f32) {
        let (w, h) = (self.base.width, self.base.height);
        ui.draw_world_background(w, h, 0);
        // !theWorld.func_650_a(updateCounter2++): the world saves a few chunks per frame.
        let still_saving = !ui.quick_save(self.save_counter);
        self.save_counter += 1;
        if still_saving || self.update_counter < 20 {
            let mut f = ((self.update_counter % 10) as f32 + partial) / 10.0;
            f = math::sin(f * PI_F32 * 2.0) * 0.2 + 0.8;
            let c = (255.0 * f) as i32;
            ui.draw_string("Saving level..", 8, h - 16, c << 16 | c << 8 | c);
        }
        ui.draw_centered_string("Game menu", w / 2, 40, 16777215);
        self.draw_buttons(ui, mx, my);
    }
}
