//! GuiDisconnected (the renamed GuiConnectFailed screen).
use crate::gui::{widgets::GuiButton, Screen, ScreenBase, Ui};
pub struct GuiDisconnected {
    base: ScreenBase,
    pub error_message: String,
    pub error_detail: String,
}
impl GuiDisconnected {
    pub fn new(ui: &Ui, message: &str, detail: &str, arguments: Option<&[String]>) -> Self {
        Self {
            base: Default::default(),
            error_message: ui.translate(message),
            error_detail: arguments.map_or_else(
                || ui.translate(detail),
                |a| ui.lang.translate_key_format(detail, a),
            ),
        }
    }
    pub(crate) fn labels(&self) -> [(String, i32, i32, i32); 2] {
        let (w, h) = (self.base.width, self.base.height);
        [
            (self.error_message.clone(), w / 2, h / 2 - 50, 16777215),
            (self.error_detail.clone(), w / 2, h / 2 - 10, 16777215),
        ]
    }
}
impl Screen for GuiDisconnected {
    fn base(&mut self) -> &mut ScreenBase {
        &mut self.base
    }
    fn key_typed(&mut self, _ui: &mut Ui, _c: char, _key: i32) {}
    fn init_gui(&mut self, ui: &mut Ui) {
        self.base.buttons.clear();
        self.base.buttons.push(GuiButton::new(
            0,
            self.base.width / 2 - 100,
            self.base.height / 4 + 132,
            ui.translate("gui.toMenu"),
        ));
    }
    fn action_performed(&mut self, ui: &mut Ui, id: i32) {
        if id == 0 {
            ui.actions.push(crate::gui::Action::MainMenu);
        }
    }
    fn draw_screen(&mut self, ui: &mut Ui, mx: i32, my: i32, _partial: f32) {
        let (w, h) = (self.base.width, self.base.height);
        ui.draw_world_background(w, h, 0);
        for (text, x, y, color) in self.labels() {
            ui.draw_centered_string(&text, x, y, color);
        }
        self.draw_buttons(ui, mx, my);
    }
}
