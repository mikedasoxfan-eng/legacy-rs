//! `GuiGameOver`

use crate::gui::widgets::GuiButton;
use crate::gui::{Action, Screen, ScreenBase, Ui};

#[derive(Default)]
pub struct GuiGameOver {
    base: ScreenBase,
}

impl GuiGameOver {
    pub fn new() -> GuiGameOver {
        GuiGameOver::default()
    }
    pub(crate) fn labels(&self, ui: &Ui) -> Vec<(String, i32, i32, i32, bool)> {
        let w = self.base.width;
        let mut labels = vec![(ui.translate(if ui.release.hardcore { "deathScreen.title.hardcore" } else { "deathScreen.title" }), w / 2 / 2, 30, 16777215, true)];
        if ui.release.hardcore { labels.push((ui.translate("deathScreen.hardcoreInfo"), w / 2, 144, 16777215, false)); }
        labels.push((format!("{}: \u{a7}e{}", ui.translate("deathScreen.score"), ui.player_score), w / 2, 100, 16777215, false));
        labels
    }
}

impl Screen for GuiGameOver {
    fn base(&mut self) -> &mut ScreenBase {
        &mut self.base
    }

    fn init_gui(&mut self, ui: &mut Ui) {
        self.base.buttons.clear();
        let (w, h) = (self.base.width, self.base.height);
        if ui.release.hardcore {
            self.base.buttons.push(GuiButton::new(1, w / 2 - 100, h / 4 + 96, ui.translate("deathScreen.deleteWorld")));
        } else {
            self.base.buttons.push(GuiButton::new(1, w / 2 - 100, h / 4 + 72, ui.translate("deathScreen.respawn")));
            self.base.buttons.push(GuiButton::new(2, w / 2 - 100, h / 4 + 96, ui.translate("deathScreen.titleScreen")));
            self.base.buttons[1].enabled = !ui.release.session_missing;
        }
    }

    fn key_typed(&mut self, _ui: &mut Ui, _c: char, _key: i32) {}

    fn action_performed(&mut self, ui: &mut Ui, id: i32) {
        if id == 1 {
            if ui.release.hardcore {
                ui.actions.push(Action::DeleteHardcoreWorld);
                return;
            }
            // thePlayer.respawnPlayer(); displayGuiScreen(null)
            ui.actions.push(Action::Respawn);
            ui.player_dead = false;
            ui.display(None);
        }
        if id == 2 {
            // changeWorld1(null) saves the world; then the title screen.
            ui.actions.push(Action::QuitWorld);
        }
    }

    fn draw_screen(&mut self, ui: &mut Ui, mx: i32, my: i32, _partial: f32) {
        let (w, h) = (self.base.width, self.base.height);
        ui.draw_gradient_rect(0, 0, w, h, 1615855616, -1602211792);
        for (text, x, y, color, doubled) in self.labels(ui) {
            if doubled { ui.gl.push_matrix(); ui.gl.scalef(2.0, 2.0, 2.0); }
            ui.draw_centered_string(&text, x, y, color);
            if doubled { ui.gl.pop_matrix(); }
        }
        self.draw_buttons(ui, mx, my);
    }

    fn does_gui_pause_game(&self) -> bool {
        false
    }
}
