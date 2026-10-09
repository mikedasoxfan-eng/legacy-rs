//! `GuiOptions`, `GuiVideoSettings`, `GuiControls`

use crate::gui::keynames::KEY_NAMES;
use crate::gui::settings::EnumOptions;
use crate::gui::widgets::{ButtonKind, GuiButton};
use crate::gui::{set_world_and_resolution, Action, Screen, ScreenBase, Ui};

/// Small buttons and sliders for a list of options, two per row (`initGui` loop).
fn option_buttons(ui: &Ui, base: &mut ScreenBase, opts: &[EnumOptions]) {
    let (w, h) = (base.width, base.height);
    for (i, &o) in opts.iter().enumerate() {
        let i = i as i32;
        let (x, y) = (w / 2 - 155 + i % 2 * 160, h / 6 + 24 * (i >> 1));
        let label = ui.settings.get_key_binding(&ui.lang, o);
        base.buttons.push(if o.is_float() {
            GuiButton::slider(o.ordinal(), x, y, o, label, ui.settings.get_option_float_value(o))
        } else {
            GuiButton::option(o.ordinal(), x, y, o, label)
        });
    }
}

/// `if (id < 100 && button instanceof GuiSmallButton)`: cycle the option, relabel.
fn toggle_option(ui: &mut Ui, base: &mut ScreenBase, id: i32) {
    let Some(b) = base.buttons.iter_mut().find(|b| b.id == id) else { return };
    if id < 100 {
        if let ButtonKind::Small(o) = b.kind {
            if let Some(o) = o {
                ui.settings.set_option_value(o, 1);
                ui.actions.push(Action::SettingsChanged);
            }
            if let Some(o) = EnumOptions::from_ordinal(id) {
                b.text = ui.settings.get_key_binding(&ui.lang, o);
            }
        }
    }
}

pub struct GuiOptions {
    base: ScreenBase,
    parent: Option<Box<dyn Screen>>,
    title: String,
}

impl GuiOptions {
    pub fn new(parent: Box<dyn Screen>) -> GuiOptions {
        GuiOptions { base: ScreenBase::default(), parent: Some(parent), title: "Options".into() }
    }
}

impl Screen for GuiOptions {
    fn base(&mut self) -> &mut ScreenBase {
        &mut self.base
    }

    fn init_gui(&mut self, ui: &mut Ui) {
        self.title = ui.translate("options.title");
        option_buttons(
            ui,
            &mut self.base,
            &[EnumOptions::Music, EnumOptions::Sound, EnumOptions::InvertMouse, EnumOptions::Sensitivity, EnumOptions::Fov, EnumOptions::Difficulty],
        );
        if ui.world_open && ui.release.hardcore {
            let text = format!("{}: {}", ui.translate("options.difficulty"), ui.translate("options.difficulty.hardcore"));
            let b = self.base.button(EnumOptions::Difficulty.ordinal());
            b.enabled = false;
            b.text = text;
        }
        let (w, h) = (self.base.width, self.base.height);
        self.base.buttons.push(GuiButton::new(101, w / 2 - 100, h / 6 + 96 - 6, ui.translate("options.video")));
        self.base.buttons.push(GuiButton::new(100, w / 2 - 100, h / 6 + 120 - 6, ui.translate("options.controls")));
        self.base.buttons.push(GuiButton::new(102, w / 2 - 100, h / 6 + 144 - 6, ui.translate("options.language")));
        self.base.buttons.push(GuiButton::new(200, w / 2 - 100, h / 6 + 168, ui.translate("gui.done")));
    }

    fn action_performed(&mut self, ui: &mut Ui, id: i32) {
        if !self.base.buttons.iter().any(|b| b.id == id && b.enabled) {
            return;
        }
        toggle_option(ui, &mut self.base, id);
        match id {
            101 => {
                ui.settings.save();
                ui.display_child(|me| Box::new(GuiVideoSettings::new(me)));
            }
            100 => {
                ui.settings.save();
                ui.display_child(|me| Box::new(GuiControls::new(me)));
            }
            102 => {
                ui.settings.save();
                ui.display_child(|me|Box::new(super::language::GuiLanguage::new(me)));
            }
            200 => {
                ui.settings.save();
                ui.display(self.parent.take());
            }
            _ => {}
        }
    }

    fn draw_screen(&mut self, ui: &mut Ui, mx: i32, my: i32, _partial: f32) {
        let (w, h) = (self.base.width, self.base.height);
        ui.draw_world_background(w, h, 0);
        let t = self.title.clone();
        ui.draw_centered_string(&t, w / 2, 20, 16777215);
        self.draw_buttons(ui, mx, my);
    }
}

pub struct GuiVideoSettings {
    base: ScreenBase,
    parent: Option<Box<dyn Screen>>,
    title: String,
    pub runtime_64_bit: bool,
}

impl GuiVideoSettings {
    pub fn new(parent: Box<dyn Screen>) -> GuiVideoSettings {
        GuiVideoSettings { base: ScreenBase::default(), parent: Some(parent), title: "Video Settings".into(), runtime_64_bit: usize::BITS == 64 }
    }
    pub(crate) fn labels(&self, ui: &Ui) -> Vec<(String, i32, i32, i32)> {
        let (w, h) = (self.base.width, self.base.height);
        let mut labels = vec![(self.title.clone(), w / 2, 20, 16777215)];
        if !self.runtime_64_bit && ui.settings.render_distance == 0 {
            for (key, y) in [("options.farWarning1", h / 6 + 144), ("options.farWarning2", h / 6 + 156)] {
                labels.push((ui.translate(key), w / 2, y, 11468800));
            }
        }
        labels
    }
}

impl Screen for GuiVideoSettings {
    fn base(&mut self) -> &mut ScreenBase {
        &mut self.base
    }

    fn init_gui(&mut self, ui: &mut Ui) {
        self.title = ui.translate("options.videoTitle");
        option_buttons(
            ui,
            &mut self.base,
            &[
                EnumOptions::Graphics,
                EnumOptions::RenderDistance,
                EnumOptions::AmbientOcclusion,
                EnumOptions::FramerateLimit,
                EnumOptions::Anaglyph,
                EnumOptions::ViewBobbing,
                EnumOptions::GuiScale,
                EnumOptions::AdvancedOpengl,
                EnumOptions::Gamma,
                EnumOptions::RenderClouds,
                EnumOptions::Particles,
            ],
        );
        let (w, h) = (self.base.width, self.base.height);
        self.base.buttons.push(GuiButton::new(200, w / 2 - 100, h / 6 + 168, ui.translate("gui.done")));
    }

    fn action_performed(&mut self, ui: &mut Ui, id: i32) {
        if !self.base.buttons.iter().any(|b| b.id == id && b.enabled) {
            return;
        }
        let old_scale = ui.settings.gui_scale;
        toggle_option(ui, &mut self.base, id);
        if id == 200 {
            ui.settings.save();
            ui.display(self.parent.take());
        }
        if ui.settings.gui_scale != old_scale {
            let sr = ui.scaled();
            set_world_and_resolution(self, ui, sr.width, sr.height);
        }
    }

    fn draw_screen(&mut self, ui: &mut Ui, mx: i32, my: i32, _partial: f32) {
        let (w, h) = (self.base.width, self.base.height);
        ui.draw_world_background(w, h, 0);
        for (text, x, y, color) in self.labels(ui) { ui.draw_centered_string(&text, x, y, color); }
        self.draw_buttons(ui, mx, my);
    }
}

pub struct GuiControls {
    base: ScreenBase,
    parent: Option<Box<dyn Screen>>,
    title: String,
    button_id: i32,
}

impl GuiControls {
    pub fn new(parent: Box<dyn Screen>) -> GuiControls {
        GuiControls { base: ScreenBase::default(), parent: Some(parent), title: "Controls".into(), button_id: -1 }
    }

    fn left(&self) -> i32 {
        self.base.width / 2 - 155
    }
}

/// `GameSettings.getOptionDisplayString`: LWJGL's key name.
fn key_name(ui: &Ui, i: usize) -> String {
    let code = ui.settings.key_bindings[i].code();
    if code<0 {return ui.translate("key.mouseButton").replace("%d",&(code+101).to_string()).replace("%1$s",&(code+101).to_string());}
    KEY_NAMES.get(code as usize).copied().flatten().unwrap_or("").to_string()
}

/// GameSettings.getKeyDisplayString for the installed 1.1 language resources.
pub(crate) fn key_display_string(ui:&Ui,code:i32)->Result<String,crate::gui::lang::FormatError> {
    if code<0 {ui.lang.try_translate_key_format("key.mouseButton",&[code.wrapping_add(101).to_string()])}
    else if code>=256 {Err(crate::gui::lang::FormatError{class:"java.lang.ArrayIndexOutOfBoundsException",message:code.to_string()})}
    else {Ok(KEY_NAMES.get(code as usize).copied().flatten().unwrap_or("null").into())}
}

impl Screen for GuiControls {
    fn base(&mut self) -> &mut ScreenBase {
        &mut self.base
    }

    fn init_gui(&mut self, ui: &mut Ui) {
        let x = self.left();
        let h = self.base.height;
        for i in 0..ui.settings.key_bindings.len() {
            let n = i as i32;
            let name = key_name(ui, i);
            self.base.buttons.push(GuiButton {
                kind: ButtonKind::Small(None),
                ..GuiButton::sized(n, x + n % 2 * 160, h / 6 + 24 * (n >> 1), 70, 20, name)
            });
        }
        let w = self.base.width;
        self.base.buttons.push(GuiButton::new(200, w / 2 - 100, h / 6 + 168, ui.translate("gui.done")));
        self.title = ui.translate("controls.title");
    }

    fn action_performed(&mut self, ui: &mut Ui, id: i32) {
        for i in 0..ui.settings.key_bindings.len() {
            self.base.buttons[i].text = key_name(ui, i);
        }
        if id == 200 {
            ui.display(self.parent.take());
        } else {
            self.button_id = id;
            let n = key_name(ui, id as usize);
            self.base.button(id).text = format!("> {n} <");
        }
    }

    fn mouse_clicked(&mut self,ui:&mut Ui,x:i32,y:i32,button:i32){
        if self.button_id>=0 {let i=self.button_id as usize;ui.settings.set_key_binding(i,-100+button);super::super::settings::KeyBinding::rebuild_registry();self.base.buttons[i].text=key_name(ui,i);self.button_id=-1;}
        else {crate::gui::default_mouse_clicked(self,ui,x,y,button);}
    }
    fn key_typed(&mut self, ui: &mut Ui, _c: char, key: i32) {
        if self.button_id >= 0 {
            let i = self.button_id as usize;
            ui.settings.set_key_binding(i, key);
            super::super::settings::KeyBinding::rebuild_registry();
            self.base.buttons[i].text = key_name(ui, i);
            self.button_id = -1;
        } else if key == crate::gui::widgets::KEY_ESCAPE {
            ui.display(None);
        }
    }

    fn draw_screen(&mut self, ui: &mut Ui, mx: i32, my: i32, _partial: f32) {
        let (w, h) = (self.base.width, self.base.height);
        ui.draw_world_background(w, h, 0);
        let t = self.title.clone();
        ui.draw_centered_string(&t, w / 2, 20, 16777215);
        let x = self.left();
        for i in 0..ui.settings.key_bindings.len() {
            let n = i as i32;
            let code=ui.settings.key_bindings[i].code();
            let duplicate=ui.settings.key_bindings.iter().enumerate().any(|(j,k)|j!=i && k.code()==code);
            self.base.buttons[i].text=if self.button_id==i as i32 {"\u{a7}f> \u{a7}e??? \u{a7}f<".into()}else if duplicate {format!("\u{a7}c{}",key_name(ui,i))}else {key_name(ui,i)};
            let d = ui.settings.key_binding_description(&ui.lang, i);
            ui.draw_string(&d, x + n % 2 * 160 + 70 + 6, h / 6 + 24 * (n >> 1) + 7, -1);
        }
        self.draw_buttons(ui, mx, my);
    }
}
