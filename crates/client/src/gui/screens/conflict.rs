//! `GuiConflictWarning`: shown when another program took the world's session lock; the
//! world was dropped without saving.

use crate::gui::widgets::GuiButton;
use crate::gui::{Screen, ScreenBase, Ui};

#[derive(Default)]
pub struct GuiConflictWarning {
    base: ScreenBase,
    update_counter: i32,
}

impl GuiConflictWarning {
    pub fn new() -> GuiConflictWarning {
        GuiConflictWarning::default()
    }
}

impl Screen for GuiConflictWarning {
    fn base(&mut self) -> &mut ScreenBase {
        &mut self.base
    }

    fn update_screen(&mut self, _ui: &mut Ui) {
        self.update_counter += 1;
    }

    fn init_gui(&mut self, _ui: &mut Ui) {
        self.base.buttons.clear();
        let (w, h) = (self.base.width, self.base.height);
        self.base.buttons.push(GuiButton::new(0, w / 2 - 100, h / 4 + 120 + 12, "Back to title screen"));
    }

    fn action_performed(&mut self, ui: &mut Ui, id: i32) {
        if id == 0 {
            // displayGuiScreen(new GuiMainMenu()): there is no world any more.
            ui.display(None);
        }
    }

    fn draw_screen(&mut self, ui: &mut Ui, mx: i32, my: i32, _partial: f32) {
        let (w, h) = (self.base.width, self.base.height);
        ui.draw_world_background(w, h, 0);
        ui.draw_centered_string("Level save conflict", w / 2, h / 4 - 60 + 20, 16777215);
        let x = w / 2 - 140;
        let y = h / 4 - 60 + 60;
        ui.draw_string("Minecraft detected a conflict in the level save data.", x, y, 10526880);
        ui.draw_string("This could be caused by two copies of the game", x, y + 18, 10526880);
        ui.draw_string("accessing the same level.", x, y + 27, 10526880);
        ui.draw_string("To prevent level corruption, the current game has quit.", x, y + 45, 10526880);
        self.draw_buttons(ui, mx, my);
    }
}
