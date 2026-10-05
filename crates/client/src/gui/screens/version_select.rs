//! beta-rs version picker: shown once at a plain launch, before the title screen. Beta 1.7.3
//! continues in this process; Beta 1.8.1 restarts the exe as `beta --mc b1.8.1`, which keeps its
//! own options, saves and stats in the `b1.8.1` folder.

use crate::gui::widgets::GuiButton;
use crate::gui::{Action, Screen, ScreenBase, Ui};

#[derive(Default)]
pub struct GuiVersionSelect {
    base: ScreenBase,
    error: Option<String>,
}

impl GuiVersionSelect {
    pub fn new() -> GuiVersionSelect {
        GuiVersionSelect::default()
    }
}

impl Screen for GuiVersionSelect {
    fn base(&mut self) -> &mut ScreenBase {
        &mut self.base
    }

    fn init_gui(&mut self, _ui: &mut Ui) {
        self.base.buttons.clear();
        let (w, h) = (self.base.width, self.base.height);
        self.base.buttons.push(GuiButton::new(0, w / 2 - 100, h / 4 + 48, "Minecraft Beta 1.7.3"));
        self.base.buttons.push(GuiButton::new(1, w / 2 - 100, h / 4 + 72, "Minecraft Beta 1.8.1"));
        self.base.buttons.push(GuiButton::new(2, w / 2 - 100, h / 4 + 120 + 12, "Quit"));
    }

    fn action_performed(&mut self, ui: &mut Ui, id: i32) {
        match id {
            // No world is open, so closing this screen shows the b1.7.3 title screen.
            0 => ui.display(None),
            1 => {
                let started = std::env::current_exe()
                    .map_err(|e| e.to_string())
                    .and_then(|exe| std::process::Command::new(exe).args(["--mc", "b1.8.1"]).spawn().map_err(|e| e.to_string()));
                match started {
                    Ok(_) => ui.actions.push(Action::Shutdown),
                    Err(e) => self.error = Some(format!("Could not start Beta 1.8.1: {e}")),
                }
            }
            2 => ui.actions.push(Action::Shutdown),
            _ => {}
        }
    }

    fn draw_screen(&mut self, ui: &mut Ui, mx: i32, my: i32, _partial: f32) {
        let (w, h) = (self.base.width, self.base.height);
        ui.draw_world_background(w, h, 0);
        ui.draw_centered_string("beta-rs", w / 2, h / 4 - 20, 16777215);
        ui.draw_centered_string("Choose a Minecraft version", w / 2, h / 4 + 10, 10526880);
        if let Some(e) = &self.error {
            ui.draw_centered_string(e, w / 2, h / 4 + 100, 16733525);
        }
        self.draw_buttons(ui, mx, my);
    }
}
