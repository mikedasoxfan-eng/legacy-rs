//! `GuiTexturePacks` (+ `GuiTexturePackSlot`). Only the built-in default pack exists here.

use crate::gui::slot::{GuiSlot, SlotList};
use crate::gui::widgets::GuiButton;
use crate::gui::{Screen, ScreenBase, Ui};

pub struct GuiTexturePacks {
    base: ScreenBase,
    parent: Option<Box<dyn Screen>>,
    refresh_timer: i32,
    folder: std::path::PathBuf,
    slot: Option<GuiSlot>,
}

impl GuiTexturePacks {
    pub fn new(parent: Box<dyn Screen>) -> GuiTexturePacks {
        GuiTexturePacks { base: ScreenBase::default(), parent: Some(parent), refresh_timer: -1, folder: Default::default(), slot: None }
    }
}

/// `TexturePackDefault` in the list.
struct Packs {
    background: (i32, i32),
}

impl SlotList for Packs {
    fn size(&self) -> i32 {
        1
    }

    fn element_clicked(&mut self, _ui: &mut Ui, _i: i32, _dbl: bool) {
        // setTexturePack(default) + refreshTextures: nothing changes.
    }

    fn is_selected(&self, _i: i32) -> bool {
        true
    }

    fn content_height(&self) -> Option<i32> {
        Some(36)
    }

    fn draw_background(&mut self, ui: &mut Ui) {
        ui.draw_world_background(self.background.0, self.background.1, 0);
    }

    fn draw_slot(&mut self, ui: &mut Ui, _i: i32, x: i32, y: i32, h: i32) {
        let t = ui.texture("pack.png");
        let gl = &mut ui.gl;
        gl.bind_texture(t);
        gl.color4f(1.0, 1.0, 1.0, 1.0);
        gl.start_drawing_quads();
        gl.set_color_opaque_i(16777215);
        gl.add_vertex_with_uv(x as f64, (y + h) as f64, 0.0, 0.0, 1.0);
        gl.add_vertex_with_uv((x + 32) as f64, (y + h) as f64, 0.0, 1.0, 1.0);
        gl.add_vertex_with_uv((x + 32) as f64, y as f64, 0.0, 1.0, 0.0);
        gl.add_vertex_with_uv(x as f64, y as f64, 0.0, 0.0, 0.0);
        gl.draw();
        ui.draw_string("Default", x + 32 + 2, y + 1, 16777215);
        ui.draw_string("The default look of Minecraft", x + 32 + 2, y + 12, 8421504);
        // secondDescriptionLine is null: nothing drawn.
    }
}

impl Screen for GuiTexturePacks {
    fn base(&mut self) -> &mut ScreenBase {
        &mut self.base
    }

    fn init_gui(&mut self, ui: &mut Ui) {
        let (w, h) = (self.base.width, self.base.height);
        self.base.buttons.push(GuiButton::small(5, w / 2 - 154, h - 48, ui.translate("texturePack.openFolder")));
        self.base.buttons.push(GuiButton::small(6, w / 2 + 4, h - 48, ui.translate("gui.done")));
        self.folder = ui.saves_dir.parent().map(|p| p.join("texturepacks")).unwrap_or_else(|| "texturepacks".into());
        let mut slot = GuiSlot::new(w, h, 32, h - 55 + 4, 36);
        slot.register_scroll_buttons(7, 8);
        self.slot = Some(slot);
    }

    fn action_performed(&mut self, ui: &mut Ui, id: i32) {
        if !self.base.buttons.iter().any(|b| b.id == id && b.enabled) && id != 7 && id != 8 {
            return;
        }
        match id {
            5 => {
                let _ = std::fs::create_dir_all(&self.folder);
                let _ = std::process::Command::new("explorer").arg(&self.folder).spawn();
            }
            6 => ui.display(self.parent.take()),
            _ => {
                if let Some(s) = &mut self.slot {
                    s.action_performed(&Packs { background: (0, 0) }, id, true);
                }
            }
        }
    }

    fn update_screen(&mut self, _ui: &mut Ui) {
        self.refresh_timer -= 1;
    }

    fn draw_screen(&mut self, ui: &mut Ui, mx: i32, my: i32, _partial: f32) {
        let (w, h) = (self.base.width, self.base.height);
        if let Some(s) = &mut self.slot {
            s.draw_screen(ui, &mut Packs { background: (w, h) }, mx, my);
        }
        if self.refresh_timer <= 0 {
            self.refresh_timer += 20;
        }
        let t = ui.translate("texturePack.title");
        ui.draw_centered_string(&t, w / 2, 16, 16777215);
        let f = ui.translate("texturePack.folderInfo");
        ui.draw_centered_string(&f, w / 2 - 77, h - 26, 8421504);
        self.draw_buttons(ui, mx, my);
    }
}
