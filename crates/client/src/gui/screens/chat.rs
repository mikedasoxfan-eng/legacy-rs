//! GuiChat. EntityPlayerSP.sendChatMessage is empty in singleplayer.
use crate::gui::{Screen, ScreenBase, Ui};

#[derive(Default)]
pub struct GuiChat {
    base: ScreenBase,
    message: String,
    update_counter: i32,
}

impl Screen for GuiChat {
    fn is_chat(&self) -> bool {
        true
    }
    fn base(&mut self) -> &mut ScreenBase {
        &mut self.base
    }
    fn update_screen(&mut self, _ui: &mut Ui) {
        self.update_counter = self.update_counter.wrapping_add(1);
    }
    fn key_typed(&mut self, ui: &mut Ui, c: char, key: i32) {
        if key == 1 || key == 28 {
            ui.display(None);
        } else {
            if key == 14 {
                self.message.pop();
            }
            if ui.font_allows(c) && self.message.encode_utf16().count() < 100 {
                self.message.push(c);
            }
        }
    }
    fn draw_screen(&mut self, ui: &mut Ui, mx: i32, my: i32, _partial: f32) {
        let (w, h) = (self.base.width, self.base.height);
        ui.draw_rect(2, h - 14, w - 2, h - 2, i32::MIN);
        let cursor = if self.update_counter / 6 % 2 == 0 { "_" } else { "" };
        ui.draw_string(&format!("> {}{cursor}", self.message), 4, h - 12, 14737632);
        self.draw_buttons(ui, mx, my);
    }
}
