//! `GuiSelectWorld` (+ `GuiWorldSlot`), `GuiCreateWorld`, `GuiRenameWorld`, `GuiYesNo`

use crate::gui::saves::{self, SaveEntry};
use crate::gui::slot::{GuiSlot, SlotList};
use crate::gui::widgets::{GuiButton, GuiTextField, KEY_ESCAPE};
use crate::gui::{Action, Screen, ScreenBase, Ui};

pub struct GuiSelectWorld {
    base: ScreenBase,
    parent: Option<Box<dyn Screen>>,
    title: String,
    selected: bool,
    selected_world: i32,
    saves: Vec<SaveEntry>,
    slot: Option<GuiSlot>,
    world_label: String,
    conversion_label: String,
    deleting: bool,
}

impl GuiSelectWorld {
    pub fn new(parent: Box<dyn Screen>) -> GuiSelectWorld {
        GuiSelectWorld {
            base: ScreenBase::default(),
            parent: Some(parent),
            title: "Select world".into(),
            selected: false,
            selected_world: -1,
            saves: Vec::new(),
            slot: None,
            world_label: String::new(),
            conversion_label: String::new(),
            deleting: false,
        }
    }

    fn load_saves(&mut self, ui: &Ui) {
        self.saves = saves::list(&ui.saves_dir);
        self.selected_world = -1;
    }

    fn save_file_name(&self, i: i32) -> Option<String> {
        self.saves.get(i as usize).map(|s| s.file_name.clone())
    }

    fn save_name(&self, ui: &Ui, i: i32) -> String {
        match self.saves.get(i as usize) {
            Some(s) if !s.display_name.is_empty() => s.display_name.clone(),
            _ => format!("{} {}", ui.translate("selectWorld.world"), i + 1),
        }
    }

    fn init_buttons(&mut self, ui: &Ui) {
        let (w, h) = (self.base.width, self.base.height);
        let b = &mut self.base.buttons;
        b.push(GuiButton::sized(1, w / 2 - 154, h - 52, 150, 20, ui.translate("selectWorld.select")));
        b.push(GuiButton::sized(6, w / 2 - 154, h - 28, 70, 20, ui.translate("selectWorld.rename")));
        b.push(GuiButton::sized(2, w / 2 - 74, h - 28, 70, 20, ui.translate("selectWorld.delete")));
        b.push(GuiButton::sized(3, w / 2 + 4, h - 52, 150, 20, ui.translate("selectWorld.create")));
        b.push(GuiButton::sized(0, w / 2 + 4, h - 28, 150, 20, ui.translate("gui.cancel")));
        for id in [1, 6, 2] {
            self.base.button(id).enabled = false;
        }
    }

    /// `selectWorld(index)`
    fn select_world(&mut self, ui: &mut Ui, i: i32) {
        ui.display(None);
        if !self.selected {
            self.selected = true;
            let folder = self.save_file_name(i).unwrap_or_else(|| format!("World{i}"));
            let name = self.save_name(ui, i);
            ui.actions.push(Action::StartWorld { folder, name, seed: None, settings: None });
            ui.display(None);
        }
    }
}

/// `GuiWorldSlot`'s view of the screen.
struct WorldSlot<'a> {
    saves: &'a [SaveEntry],
    selected_world: &'a mut i32,
    buttons: &'a mut Vec<GuiButton>,
    world_label: &'a str,
    conversion_label: &'a str,
    background: (i32, i32),
    /// Double-click asks the screen to open the world after drawing.
    open: Option<i32>,
}

impl SlotList for WorldSlot<'_> {
    fn size(&self) -> i32 {
        self.saves.len() as i32
    }

    fn element_clicked(&mut self, _ui: &mut Ui, i: i32, dbl: bool) {
        *self.selected_world = i;
        let ok = *self.selected_world >= 0 && *self.selected_world < self.size();
        for b in self.buttons.iter_mut().filter(|b| matches!(b.id, 1 | 6 | 2)) {
            b.enabled = ok;
        }
        if dbl && ok {
            self.open = Some(i);
        }
    }

    fn is_selected(&self, i: i32) -> bool {
        i == *self.selected_world
    }

    fn content_height(&self) -> Option<i32> {
        Some(self.saves.len() as i32 * 36)
    }

    fn draw_background(&mut self, ui: &mut Ui) {
        ui.draw_world_background(self.background.0, self.background.1, 0);
    }

    fn draw_slot(&mut self, ui: &mut Ui, i: i32, x: i32, y: i32, _h: i32) {
        let s = &self.saves[i as usize];
        let name = if s.display_name.is_empty() { format!("{} {}", self.world_label, i + 1) } else { s.display_name.clone() };
        // (float)(size / 1024L * 100L / 1024L) / 100.0F, printed by Java's Float.toString.
        let mb = (s.size_on_disk / 1024 * 100 / 1024) as f32 / 100.0;
        let info = format!("{} ({}, {} MB)", s.file_name, saves::format_date(s.last_played), java_float_string(mb));
        let mut conv = String::new();
        if s.needs_conversion {
            conv = format!("{} {}", self.conversion_label, conv);
        }
        ui.draw_string(&name, x + 2, y + 1, 16777215);
        ui.draw_string(&info, x + 2, y + 12, 8421504);
        ui.draw_string(&conv, x + 2, y + 12 + 10, 8421504);
    }
}

/// `Float.toString(f)`: shortest digits that round-trip, at least one decimal, with
/// scientific notation below 1e-3 or from 1e7.
pub fn java_float_string(f: f32) -> String {
    if f.is_nan() {
        return "NaN".into();
    }
    if f.is_infinite() {
        return if f > 0.0 { "Infinity" } else { "-Infinity" }.into();
    }
    let a = f.abs();
    if a == 0.0 || (1e-3..1e7).contains(&a) {
        format!("{f:?}")
    } else {
        // Rust's {:e} gives "1.5e7"; Java gives "1.5E7".
        let s = format!("{f:e}");
        let (m, e) = s.split_once('e').unwrap();
        let m = if m.contains('.') { m.to_string() } else { format!("{m}.0") };
        format!("{m}E{e}")
    }
}

impl Screen for GuiSelectWorld {
    fn base(&mut self) -> &mut ScreenBase {
        &mut self.base
    }

    fn init_gui(&mut self, ui: &mut Ui) {
        self.title = ui.translate("selectWorld.title");
        self.world_label = ui.translate("selectWorld.world");
        self.conversion_label = ui.translate("selectWorld.conversion");
        self.load_saves(ui);
        let (w, h) = (self.base.width, self.base.height);
        let mut slot = GuiSlot::new(w, h, 32, h - 64, 36);
        slot.register_scroll_buttons(4, 5);
        self.slot = Some(slot);
        self.init_buttons(ui);
    }

    fn action_performed(&mut self, ui: &mut Ui, id: i32) {
        let enabled = self.base.buttons.iter().any(|b| b.id == id && b.enabled);
        if !enabled {
            return;
        }
        match id {
            2 => {
                let name = self.save_name(ui, self.selected_world);
                self.deleting = true;
                let q = ui.translate("selectWorld.deleteQuestion");
                let warn = format!("'{}' {}", name, ui.translate("selectWorld.deleteWarning"));
                let yes = ui.translate("selectWorld.deleteButton");
                let no = ui.translate("gui.cancel");
                let n = self.selected_world;
                ui.display_child(move |me| Box::new(GuiYesNo::new(me, q, warn, yes, no, n)));
            }
            1 => self.select_world(ui, self.selected_world),
            3 => ui.display_child(|me| Box::new(GuiCreateWorld::new(me))),
            6 => {
                if let Some(folder) = self.save_file_name(self.selected_world) {
                    ui.display_child(move |me| Box::new(GuiRenameWorld::new(me, folder)));
                }
            }
            0 => ui.display(self.parent.take()),
            _ => {
                if let Some(slot) = &mut self.slot {
                    let l = WorldSlot {
                        saves: &self.saves,
                        selected_world: &mut self.selected_world,
                        buttons: &mut self.base.buttons,
                        world_label: &self.world_label,
                        conversion_label: &self.conversion_label,
                        background: (self.base.width, self.base.height),
                        open: None,
                    };
                    slot.action_performed(&l, id, true);
                }
            }
        }
    }

    fn delete_world(&mut self, ui: &mut Ui, yes: bool, i: i32) -> bool {
        if !self.deleting {
            return false;
        }
        self.deleting = false;
        if yes {
            if let Some(f) = self.save_file_name(i) {
                saves::delete(&ui.saves_dir, &f);
            }
            self.load_saves(ui);
        }
        true
    }

    fn draw_screen(&mut self, ui: &mut Ui, mx: i32, my: i32, _partial: f32) {
        let mut open = None;
        if let Some(slot) = &mut self.slot {
            let mut l = WorldSlot {
                saves: &self.saves,
                selected_world: &mut self.selected_world,
                buttons: &mut self.base.buttons,
                world_label: &self.world_label,
                conversion_label: &self.conversion_label,
                background: (self.base.width, self.base.height),
                open: None,
            };
            slot.draw_screen(ui, &mut l, mx, my);
            open = l.open;
        }
        if let Some(i) = open {
            self.select_world(ui, i);
        }
        let t = self.title.clone();
        ui.draw_centered_string(&t, self.base.width / 2, 20, 16777215);
        self.draw_buttons(ui, mx, my);
    }
}

pub struct GuiCreateWorld {
    base: ScreenBase,
    parent: Option<Box<dyn Screen>>,
    name: GuiTextField,
    seed: GuiTextField,
    folder_name: String,
    create_clicked: bool,
    creative: bool,
    map_features: bool,
    more_options: bool,
}

impl GuiCreateWorld {
    pub fn new(parent: Box<dyn Screen>) -> GuiCreateWorld {
        GuiCreateWorld {
            base: ScreenBase::default(),
            parent: Some(parent),
            name: GuiTextField::new(0, 0, 0, 0, ""),
            seed: GuiTextField::new(0, 0, 0, 0, ""),
            folder_name: String::new(),
            create_clicked: false,
            creative: false,
            map_features: true,
            more_options: false,
        }
    }

    /// `func_22129_j`: the folder name the world will get.
    fn update_folder_name(&mut self, ui: &Ui) {
        let mut f = self.name.text.trim_matches(|c: char| c <= ' ').to_string();
        for c in ['/', '\n', '\r', '\t', '\0', '\x0c', '`', '?', '*', '\\', '<', '>', '|', '"', ':'] {
            f = f.replace(c, "_");
        }
        if f.is_empty() {
            f = "World".into();
        }
        self.folder_name = saves::unused_folder_name(&ui.saves_dir, f);
    }

    fn update_labels(&mut self, ui: &Ui) {
        let mode = if self.creative { "creative" } else { "survival" };
        self.base.button(2).text = format!("{} {}", ui.translate("selectWorld.gameMode"), ui.translate(&format!("selectWorld.gameMode.{mode}")));
        self.base.button(4).text = format!("{} {}", ui.translate("selectWorld.mapFeatures"), ui.translate(if self.map_features { "options.on" } else { "options.off" }));
        self.base.button(5).text = format!("{} {}", ui.translate("selectWorld.mapType"), ui.translate("selectWorld.mapType.normal"));
    }

    fn create(&mut self, ui: &mut Ui) {
        if !self.base.buttons.first().is_some_and(|b| b.enabled) {
            return;
        }
        ui.display(None);
        if self.create_clicked {
            return;
        }
        self.create_clicked = true;
        let text = self.seed.text.clone();
        // new Random().nextLong() when empty, zero or unparsable as a number... except that a
        // non-numeric seed uses its String.hashCode.
        let mut seed = None;
        if !text.is_empty() {
            match text.parse::<i64>() {
                Ok(v) if v != 0 => seed = Some(v),
                Ok(_) => {}
                Err(_) => seed = Some(text.encode_utf16().fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(c as i32)) as i64),
            }
        }
        let seed = seed.unwrap_or_else(random_seed);
        ui.actions.push(Action::StartWorld { folder: self.folder_name.clone(), name: self.name.text.clone(), seed: Some(seed), settings: Some(worldgen::entity::player::food::WorldSettings::new(seed, self.creative as i32, self.map_features)) });
        ui.display(None);
    }
}

/// `new Random().nextLong()`
pub fn random_seed() -> i64 {
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_nanos()) as i64;
    worldgen::JavaRandom::new(nanos ^ 8682522807148012).next_long()
}

impl Screen for GuiCreateWorld {
    fn repeats_keys(&self) -> bool { true }
    fn base(&mut self) -> &mut ScreenBase {
        &mut self.base
    }

    fn update_screen(&mut self, _ui: &mut Ui) {
        self.name.update_cursor_counter();
        self.seed.update_cursor_counter();
    }

    fn init_gui(&mut self, ui: &mut Ui) {
        let (w, h) = (self.base.width, self.base.height);
        self.base.buttons.clear();
        self.base.buttons.push(GuiButton::sized(0, w / 2 - 155, h - 28, 150, 20, ui.translate("selectWorld.create")));
        self.base.buttons.push(GuiButton::sized(1, w / 2 + 5, h - 28, 150, 20, ui.translate("gui.cancel")));
        self.base.buttons.push(GuiButton::small(2, w / 2 - 75, 100, ""));
        self.base.buttons.push(GuiButton::small(3, w / 2 - 75, 172, ui.translate("selectWorld.moreWorldOptions")));
        self.base.buttons.push(GuiButton::small(4, w / 2 - 155, 100, ""));
        self.base.buttons.push(GuiButton::small(5, w / 2 + 5, 100, ""));
        self.base.button(4).visible = false;
        self.base.button(5).visible = false;
        self.base.button(5).enabled = false;
        self.name = GuiTextField::new(w / 2 - 100, 60, 200, 20, ui.translate("selectWorld.newWorld"));
        self.name.focused = true;
        self.name.max_length = 32;
        self.seed = GuiTextField::new(w / 2 - 100, 60, 200, 20, "");
        self.update_labels(ui);
        self.update_folder_name(ui);
    }

    fn action_performed(&mut self, ui: &mut Ui, id: i32) {
        if !self.base.buttons.iter().any(|b| b.id == id && b.enabled) { return; }
        match id {
            1 => ui.display(self.parent.take()),
            0 => self.create(ui),
            2 => { self.creative = !self.creative; self.update_labels(ui); }
            3 => {
                self.more_options = !self.more_options;
                self.base.button(2).visible = !self.more_options;
                self.base.button(4).visible = self.more_options;
                self.base.button(5).visible = self.more_options;
                self.base.button(3).text = ui.translate(if self.more_options { "gui.done" } else { "selectWorld.moreWorldOptions" });
            }
            4 => { self.map_features = !self.map_features; self.update_labels(ui); }
            _ => {}
        }
    }

    fn key_typed(&mut self, ui: &mut Ui, c: char, key: i32) {
        let clip = ui.clipboard.clone();
        let next = if self.name.focused && !self.more_options {
            self.name.key_typed(ui, c, key, || clip)
        } else if self.seed.focused && self.more_options {
            self.seed.key_typed(ui, c, key, || clip)
        } else { false };
        if next {
            // selectNextField
            if self.name.focused {
                self.name.set_focused(false);
                self.seed.set_focused(true);
            } else {
                self.name.set_focused(true);
                self.seed.set_focused(false);
            }
        }
        if c == '\r' {
            self.create(ui);
        }
        self.base.buttons[0].enabled = !self.name.text.is_empty();
        self.update_folder_name(ui);
    }

    fn mouse_clicked(&mut self, ui: &mut Ui, x: i32, y: i32, button: i32) {
        crate::gui::default_mouse_clicked(self, ui, x, y, button);
        if !self.more_options { self.name.mouse_clicked(x, y, button); }
        else { self.seed.mouse_clicked(x, y, button); }
    }

    fn draw_screen(&mut self, ui: &mut Ui, mx: i32, my: i32, _partial: f32) {
        let (w, h) = (self.base.width, self.base.height);
        ui.draw_world_background(w, h, 0);
        let t = ui.translate("selectWorld.create");
        ui.draw_centered_string(&t, w / 2, 20, 16777215);
        if !self.more_options {
            let label = ui.translate("selectWorld.enterName");
            ui.draw_string(&label, w / 2 - 100, 47, 10526880);
            let label = format!("{} {}", ui.translate("selectWorld.resultFolder"), self.folder_name);
            ui.draw_string(&label, w / 2 - 100, 85, 10526880);
            self.name.draw_text_box(ui);
            let mode = if self.creative { "creative" } else { "survival" };
            let label = ui.translate(&format!("selectWorld.gameMode.{mode}.line1"));
            ui.draw_string(&label, w / 2 - 100, 122, 10526880);
            let label = ui.translate(&format!("selectWorld.gameMode.{mode}.line2"));
            ui.draw_string(&label, w / 2 - 100, 134, 10526880);
        } else {
            let label = ui.translate("selectWorld.enterSeed");
            ui.draw_string(&label, w / 2 - 100, 47, 10526880);
            let label = ui.translate("selectWorld.seedInfo");
            ui.draw_string(&label, w / 2 - 100, 85, 10526880);
            let label = ui.translate("selectWorld.mapFeatures.info");
            ui.draw_string(&label, w / 2 - 150, 122, 10526880);
            self.seed.draw_text_box(ui);
        }
        self.draw_buttons(ui, mx, my);
    }
}

pub struct GuiRenameWorld {
    base: ScreenBase,
    parent: Option<Box<dyn Screen>>,
    field: GuiTextField,
    folder: String,
}

impl GuiRenameWorld {
    pub fn new(parent: Box<dyn Screen>, folder: String) -> GuiRenameWorld {
        GuiRenameWorld { base: ScreenBase::default(), parent: Some(parent), field: GuiTextField::new(0, 0, 0, 0, ""), folder }
    }
}

impl Screen for GuiRenameWorld {
    fn repeats_keys(&self) -> bool { true }
    fn base(&mut self) -> &mut ScreenBase {
        &mut self.base
    }

    fn update_screen(&mut self, _ui: &mut Ui) {
        self.field.update_cursor_counter();
    }

    fn init_gui(&mut self, ui: &mut Ui) {
        let (w, h) = (self.base.width, self.base.height);
        self.base.buttons.clear();
        self.base.buttons.push(GuiButton::new(0, w / 2 - 100, h / 4 + 96 + 12, ui.translate("selectWorld.renameButton")));
        self.base.buttons.push(GuiButton::new(1, w / 2 - 100, h / 4 + 120 + 12, ui.translate("gui.cancel")));
        let name = saves::world_info(&ui.saves_dir, &self.folder).map(|c| c.string("LevelName")).unwrap_or_default();
        self.field = GuiTextField::new(w / 2 - 100, 60, 200, 20, name);
        self.field.focused = true;
        self.field.max_length = 32;
    }

    fn action_performed(&mut self, ui: &mut Ui, id: i32) {
        if !self.base.buttons.iter().any(|b| b.id == id && b.enabled) {
            return;
        }
        if id == 1 {
            ui.display(self.parent.take());
        } else if id == 0 {
            saves::rename(&ui.saves_dir, &self.folder, self.field.text.trim_matches(|c: char| c <= ' '));
            ui.display(self.parent.take());
        }
    }

    fn key_typed(&mut self, ui: &mut Ui, c: char, key: i32) {
        let clip = ui.clipboard.clone();
        self.field.key_typed(ui, c, key, || clip);
        self.base.buttons[0].enabled = !self.field.text.trim_matches(|c: char| c <= ' ').is_empty();
        if c == '\r' {
            self.action_performed(ui, 0);
        }
    }

    fn mouse_clicked(&mut self, ui: &mut Ui, x: i32, y: i32, button: i32) {
        crate::gui::default_mouse_clicked(self, ui, x, y, button);
        self.field.mouse_clicked(x, y, button);
    }

    fn draw_screen(&mut self, ui: &mut Ui, mx: i32, my: i32, _partial: f32) {
        let (w, h) = (self.base.width, self.base.height);
        ui.draw_world_background(w, h, 0);
        let t = ui.translate("selectWorld.renameTitle");
        ui.draw_centered_string(&t, w / 2, h / 4 - 60 + 20, 16777215);
        let s = ui.translate("selectWorld.enterName");
        ui.draw_string(&s, w / 2 - 100, 47, 10526880);
        self.field.draw_text_box(ui);
        self.draw_buttons(ui, mx, my);
    }
}

pub struct GuiYesNo {
    base: ScreenBase,
    parent: Option<Box<dyn Screen>>,
    message1: String,
    message2: String,
    yes: String,
    no: String,
    world_number: i32,
}

impl GuiYesNo {
    pub fn new(parent: Box<dyn Screen>, m1: String, m2: String, yes: String, no: String, n: i32) -> GuiYesNo {
        GuiYesNo { base: ScreenBase::default(), parent: Some(parent), message1: m1, message2: m2, yes, no, world_number: n }
    }
}

impl Screen for GuiYesNo {
    fn base(&mut self) -> &mut ScreenBase {
        &mut self.base
    }

    fn init_gui(&mut self, _ui: &mut Ui) {
        let (w, h) = (self.base.width, self.base.height);
        self.base.buttons.push(GuiButton::small(0, w / 2 - 155, h / 6 + 96, self.yes.clone()));
        self.base.buttons.push(GuiButton::small(1, w / 2 - 155 + 160, h / 6 + 96, self.no.clone()));
    }

    fn action_performed(&mut self, ui: &mut Ui, id: i32) {
        if let Some(p) = &mut self.parent {
            if p.delete_world(ui, id == 0, self.world_number) {
                ui.display(self.parent.take());
            }
        }
    }

    fn key_typed(&mut self, ui: &mut Ui, _c: char, key: i32) {
        if key == KEY_ESCAPE {
            ui.display(None);
        }
    }

    fn draw_screen(&mut self, ui: &mut Ui, mx: i32, my: i32, _partial: f32) {
        let (w, h) = (self.base.width, self.base.height);
        ui.draw_world_background(w, h, 0);
        let (m1, m2) = (self.message1.clone(), self.message2.clone());
        ui.draw_centered_string(&m1, w / 2, 70, 16777215);
        ui.draw_centered_string(&m2, w / 2, 90, 16777215);
        self.draw_buttons(ui, mx, my);
    }
}
