//! `GuiAchievement`: the "Achievement get!" box that slides in at the top right.

use super::stats::ACHIEVEMENTS;
use super::Ui;
use crate::render::entity::gui_item;
use worldgen::world::tile::ItemStack;

#[derive(Default)]
pub struct AchievementToast {
    /// `field_25085_d`, `field_25084_e`
    title: String,
    text: String,
    achievement: Option<usize>,
    /// `field_25083_f`: when it was queued (ms), 0 when idle.
    start: i64,
    /// `field_27103_i`: an information box (the inventory hint) instead of "Achievement get!".
    information: bool,
}

impl AchievementToast {
    #[cfg(test)]
    pub(crate) fn queued_text(&self)->(&str,&str){(&self.title,&self.text)}
    /// `queueTakenAchievement`
    pub fn queue_taken(&mut self, ui: &Ui, index: usize) {
        self.title = ui.translate("achievement.get");
        self.text = ui.translate(&format!("achievement.{}", ACHIEVEMENTS[index].key));
        self.start = ui.millis;
        self.achievement = Some(index);
        self.information = false;
    }

    /// `queueAchievementInformation`: queued every tick until the achievement is taken, so
    /// the box stays fully shown.
    pub fn queue_information(&mut self, ui: &Ui, index: usize) {
        self.title = ui.translate(&format!("achievement.{}", ACHIEVEMENTS[index].key));
        self.text = super::screens::achievements::description(ui, index);
        self.start = ui.millis - 2500;
        self.achievement = Some(index);
        self.information = true;
    }

    /// `updateAchievementWindow`, drawn after everything else each frame.
    pub fn draw(&mut self, ui: &mut Ui) {
        let Some(index) = self.achievement else { return };
        if self.start == 0 {
            return;
        }
        let t = (ui.millis - self.start) as f64 / 3000.0;
        if !self.information && !(0.0..=1.0).contains(&t) {
            self.start = 0;
            return;
        }
        // updateAchievementWindowScale
        let sr = ui.scaled();
        ui.setup_overlay_rendering();
        ui.gl.depth_test = false;
        ui.gl.depth_mask = false;
        let mut slide = t * 2.0;
        if slide > 1.0 {
            slide = 2.0 - slide;
        }
        slide *= 4.0;
        slide = 1.0 - slide;
        if slide < 0.0 {
            slide = 0.0;
        }
        slide *= slide;
        slide *= slide;
        let x = sr.width - 160;
        let y = -((slide * 36.0) as i32);
        ui.gl.color4f(1.0, 1.0, 1.0, 1.0);
        ui.gl.texture_2d = true;
        let bg = ui.texture("achievement/bg.png");
        ui.gl.bind_texture(bg);
        ui.draw_textured_modal_rect(x, y, 96, 202, 160, 32);
        let (title, text) = (self.title.clone(), self.text.clone());
        if self.information {
            ui.font.draw_split_string(&mut ui.gl, &text, x + 30, y + 7, 120, -1);
        } else {
            ui.font.draw_string(&mut ui.gl, &title, x + 30, y + 7, -256);
            ui.font.draw_string(&mut ui.gl, &text, x + 30, y + 18, -1);
        }
        // Release GUI lighting; GL_RESCALE_NORMAL
        gui_item::set_gui_release_lighting();
        gui_item::set_gui_rescale_normal(true);
        let icon = ACHIEVEMENTS[index].icon;
        let stack = ItemStack::new(icon.0 as i32, 1, icon.1 as i32);
        gui_item::render_item_into_gui(ui, Some(&stack), x + 8, y + 8);
        ui.gl.depth_mask = true;
        ui.gl.depth_test = true;
    }
}
