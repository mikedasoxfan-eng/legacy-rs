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
}

impl Screen for GuiGameOver {
    fn base(&mut self) -> &mut ScreenBase {
        &mut self.base
    }

    fn init_gui(&mut self, _ui: &mut Ui) {
        self.base.buttons.clear();
        let (w, h) = (self.base.width, self.base.height);
        self.base.buttons.push(GuiButton::new(1, w / 2 - 100, h / 4 + 72, "Respawn"));
        self.base.buttons.push(GuiButton::new(2, w / 2 - 100, h / 4 + 96, "Title menu"));
        // mc.session is never null here, so "Title menu" stays enabled.
    }

    fn key_typed(&mut self, _ui: &mut Ui, _c: char, _key: i32) {}

    fn action_performed(&mut self, ui: &mut Ui, id: i32) {
        if id == 1 {
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
        ui.gl.push_matrix();
        ui.gl.scalef(2.0, 2.0, 2.0);
        ui.draw_centered_string("Game over!", w / 2 / 2, 30, 16777215);
        ui.gl.pop_matrix();
        let score = format!("Score: &e{}", ui.player_score);
        ui.draw_centered_string(&score, w / 2, 100, 16777215);
        self.draw_buttons(ui, mx, my);
    }

    fn does_gui_pause_game(&self) -> bool {
        false
    }
}
