//! GuiTexturePacks / GuiTexturePackSlot.

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
    entries: Vec<(String, Option<String>, Option<String>, crate::gui::gl::TexId)>,
    selected: usize,
}

impl Packs {
    fn new(ui: &Ui, background: (i32, i32)) -> Self {
        Self { background, selected: ui.packs.selected, entries: ui.packs.available.iter().enumerate().map(|(i, p)| (
            p.name.clone(), p.first.clone(), p.second.clone(), ui.pack_icons.get(i).copied().unwrap_or_else(|| ui.texture("gui/unknown_pack.png"))
        )).collect() }
    }
}

impl SlotList for Packs {
    fn size(&self) -> i32 {
        self.entries.len() as i32
    }

    fn element_clicked(&mut self, ui: &mut Ui, i: i32, _dbl: bool) {
        if ui.packs.select(i as usize) {
            ui.settings.skin.clone_from(&ui.packs.available[i as usize].name);
            ui.settings.save();
        }
        self.selected = i as usize;
        ui.actions.push(crate::gui::Action::RefreshTextures);
    }

    fn is_selected(&self, i: i32) -> bool { self.selected == i as usize }

    fn content_height(&self) -> Option<i32> {
        Some(self.size() * 36)
    }

    fn draw_background(&mut self, ui: &mut Ui) {
        ui.draw_world_background(self.background.0, self.background.1, 0);
    }

    fn draw_slot(&mut self, ui: &mut Ui, i: i32, x: i32, y: i32, h: i32) {
        let (name, first, second, t) = &self.entries[i as usize];
        let t = *t;
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
        ui.draw_string(name, x + 32 + 2, y + 1, 16777215);
        if let Some(first) = first { ui.draw_string(first, x + 34, y + 12, 8421504); }
        if let Some(second) = second { ui.draw_string(second, x + 34, y + 22, 8421504); }
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
        ui.packs.update();
        self.folder = ui.packs.folder.clone();
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
            6 => {
                ui.actions.push(crate::gui::Action::RefreshTextures);
                ui.display(self.parent.take());
            },
            _ => {
                if let Some(s) = &mut self.slot {
                    s.action_performed(&Packs::new(ui, (0, 0)), id, true);
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
            s.draw_screen(ui, &mut Packs::new(ui, (w, h)), mx, my);
        }
        if self.refresh_timer <= 0 {
            ui.packs.update();
            self.refresh_timer += 20;
        }
        let t = ui.translate("texturePack.title");
        ui.draw_centered_string(&t, w / 2, 16, 16777215);
        let f = ui.translate("texturePack.folderInfo");
        ui.draw_centered_string(&f, w / 2 - 77, h - 26, 8421504);
        self.draw_buttons(ui, mx, my);
    }
}
