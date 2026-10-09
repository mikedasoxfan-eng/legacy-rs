//! `GuiButton`, `GuiSmallButton`, `GuiSlider`, `GuiTextField` and LWJGL key codes.

use super::settings::EnumOptions;
use super::Ui;

// LWJGL `Keyboard` codes the screens use.
pub const KEY_ESCAPE: i32 = 1;
pub const KEY_BACK: i32 = 14;
pub const KEY_TAB: i32 = 15;
pub const KEY_RETURN: i32 = 28;
pub const KEY_F11: i32 = 87;

#[derive(Clone, Debug)]
pub enum ButtonKind {
    Plain,
    Language,
    /// `GuiSmallButton` (an options toggle when the option is set).
    Small(Option<EnumOptions>),
    /// `GuiSlider`
    Slider { option: EnumOptions, value: f32, dragging: bool },
}

#[derive(Clone, Debug)]
pub struct GuiButton {
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
    pub text: String,
    pub id: i32,
    pub enabled: bool,
    /// `enabled2`: visible
    pub visible: bool,
    pub kind: ButtonKind,
}

impl GuiButton {
    pub fn language(id:i32,x:i32,y:i32)->GuiButton {GuiButton{kind:ButtonKind::Language,..GuiButton::sized(id,x,y,20,20,"")}}
    pub fn new(id: i32, x: i32, y: i32, text: impl Into<String>) -> GuiButton {
        GuiButton::sized(id, x, y, 200, 20, text)
    }

    pub fn sized(id: i32, x: i32, y: i32, w: i32, h: i32, text: impl Into<String>) -> GuiButton {
        GuiButton { width: w, height: h, x, y, text: text.into(), id, enabled: true, visible: true, kind: ButtonKind::Plain }
    }

    /// `GuiSmallButton(id, x, y, text)`
    pub fn small(id: i32, x: i32, y: i32, text: impl Into<String>) -> GuiButton {
        GuiButton { kind: ButtonKind::Small(None), ..GuiButton::sized(id, x, y, 150, 20, text) }
    }

    /// `GuiSmallButton(id, x, y, option, text)`
    pub fn option(id: i32, x: i32, y: i32, o: EnumOptions, text: impl Into<String>) -> GuiButton {
        GuiButton { kind: ButtonKind::Small(Some(o)), ..GuiButton::sized(id, x, y, 150, 20, text) }
    }

    /// `GuiSlider`
    pub fn slider(id: i32, x: i32, y: i32, o: EnumOptions, text: impl Into<String>, value: f32) -> GuiButton {
        GuiButton {
            kind: ButtonKind::Slider { option: o, value, dragging: false },
            ..GuiButton::sized(id, x, y, 150, 20, text)
        }
    }

    pub fn options(&self) -> Option<EnumOptions> {
        match self.kind {
            ButtonKind::Small(o) => o,
            ButtonKind::Slider { option, .. } => Some(option),
            ButtonKind::Plain | ButtonKind::Language => None,
        }
    }

    pub(crate) fn hover_state(&self, hovered: bool) -> i32 {
        if let ButtonKind::Slider { .. } = self.kind {
            return 0;
        }
        if !self.enabled {
            0
        } else if hovered {
            2
        } else {
            1
        }
    }

    pub fn draw_button(&mut self, ui: &mut Ui, mx: i32, my: i32) {
        if !self.visible {
            return;
        }
        let t = ui.texture("gui/gui.png");
        ui.gl.bind_texture(t);
        ui.gl.color4f(1.0, 1.0, 1.0, 1.0);
        let hovered = mx >= self.x && my >= self.y && mx < self.x + self.width && my < self.y + self.height;
        if matches!(self.kind,ButtonKind::Language) {
            ui.draw_textured_modal_rect(self.x,self.y,0,106+if hovered{self.height}else{0},self.width,self.height);
            return;
        }
        let h = self.hover_state(hovered);
        ui.draw_textured_modal_rect(self.x, self.y, 0, 46 + h * 20, self.width / 2, self.height);
        ui.draw_textured_modal_rect(self.x + self.width / 2, self.y, 200 - self.width / 2, 46 + h * 20, self.width / 2, self.height);
        self.mouse_dragged(ui, mx, my);
        let (cx, cy) = (self.x + self.width / 2, self.y + (self.height - 8) / 2);
        let color = if !self.enabled {
            -6250336
        } else if hovered {
            16777120
        } else {
            14737632
        };
        let text = self.text.clone();
        ui.draw_centered_string(&text, cx, cy, color);
    }

    fn mouse_dragged(&mut self, ui: &mut Ui, mx: i32, _my: i32) {
        let (x, y, w, visible) = (self.x, self.y, self.width, self.visible);
        if let ButtonKind::Slider { option, value, dragging } = &mut self.kind {
            if !visible {
                return;
            }
            if *dragging {
                *value = ((mx - (x + 4)) as f32 / (w - 8) as f32).clamp(0.0, 1.0);
                ui.settings.set_option_float_value(*option, *value);
                self.text = ui.settings.get_key_binding(&ui.lang, *option);
            }
            let v = *value;
            ui.gl.color4f(1.0, 1.0, 1.0, 1.0);
            ui.draw_textured_modal_rect(x + (v * (w - 8) as f32) as i32, y, 0, 66, 4, 20);
            ui.draw_textured_modal_rect(x + (v * (w - 8) as f32) as i32 + 4, y, 196, 66, 4, 20);
        }
    }

    pub(crate) fn inside(&self, x: i32, y: i32) -> bool {
        self.enabled && self.visible && x >= self.x && y >= self.y && x < self.x + self.width && y < self.y + self.height
    }

    /// `mousePressed` (base version; sliders update through `slider_pressed`).
    pub fn mouse_pressed(&self, x: i32, y: i32) -> bool {
        self.inside(x, y)
    }

    /// `GuiSlider.mousePressed`'s side effects.
    pub fn press(&mut self, ui: &mut Ui, x: i32, y: i32) -> bool {
        if !self.inside(x, y) {
            return false;
        }
        let (bx, w) = (self.x, self.width);
        if let ButtonKind::Slider { option, value, dragging } = &mut self.kind {
            *value = ((x - (bx + 4)) as f32 / (w - 8) as f32).clamp(0.0, 1.0);
            ui.settings.set_option_float_value(*option, *value);
            self.text = ui.settings.get_key_binding(&ui.lang, *option);
            *dragging = true;
        }
        true
    }

    pub fn mouse_released(&mut self, _x: i32, _y: i32) {
        if let ButtonKind::Slider { dragging, .. } = &mut self.kind {
            *dragging = false;
        }
    }
}

/// `GuiTextField`
#[derive(Clone, Debug)]
pub struct GuiTextField {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub text: String,
    // Retain Java UTF-16 input for display and subsequent edits. Public String
    // consumers receive the usual replacement for an incomplete surrogate pair.
    text_units: Vec<u16>,
    text_snapshot: String,
    pub max_length: usize,
    cursor_counter: i32,
    pub focused: bool,
    pub enabled: bool,
}

impl GuiTextField {
    pub fn new(x: i32, y: i32, w: i32, h: i32, text: impl Into<String>) -> GuiTextField {
        let text = text.into();
        let text_units = text.encode_utf16().collect();
        let text_snapshot = text.clone();
        GuiTextField { x, y, width: w, height: h, text, text_units, text_snapshot, max_length: 0, cursor_counter: 0, focused: false, enabled: true }
    }

    pub fn update_cursor_counter(&mut self) {
        self.cursor_counter += 1;
    }

    /// `textboxKeyTyped`; returns true if Tab asked the screen to select the next field.
    pub fn key_typed(&mut self, ui: &Ui, c: char, key: i32, clipboard: impl FnOnce() -> Option<String>) -> bool {
        let clip = if self.enabled && self.focused && c == '\u{16}' { clipboard() } else { None };
        let mut next = false;
        for &unit in c.encode_utf16(&mut [0; 2]).iter() { next |= self.key_unit(ui, unit, key, clip.as_deref()); }
        next
    }

    pub(crate) fn key_unit(&mut self, ui: &Ui, unit: u16, key: i32, clipboard: Option<&str>) -> bool {
        if self.text != self.text_snapshot { self.text_units = self.text.encode_utf16().collect(); }
        let mut next = false;
        if self.enabled && self.focused {
            if unit == 9 {
                next = true;
            }
            if unit == 22 {
                let clip: Vec<u16> = clipboard.unwrap_or_default().encode_utf16().collect();
                let len = self.text_units.len() as i32;
                let mut n = 32 - len;
                let clip_len = clip.len() as i32;
                if n > clip_len {
                    n = clip_len;
                }
                if n > 0 {
                    self.text_units.extend_from_slice(&clip[..n as usize]);
                }
            }
            if key == KEY_BACK && !self.text_units.is_empty() {
                self.text_units.pop();
            }
            let len = self.text_units.len();
            let allowed = unit > 32 || char::from_u32(unit as u32).is_some_and(|c| ui.font_allows(c));
            if allowed && (len < self.max_length || self.max_length == 0) {
                self.text_units.push(unit);
            }
        }
        self.text = String::from_utf16_lossy(&self.text_units);
        self.text_snapshot = self.text.clone();
        next
    }

    pub(crate) fn display_units(&self) -> Vec<u16> {
        if self.text == self.text_snapshot { self.text_units.clone() } else { self.text.encode_utf16().collect() }
    }

    pub fn mouse_clicked(&mut self, x: i32, y: i32, _button: i32) {
        let hit = self.enabled && x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height;
        self.set_focused(hit);
    }

    pub fn set_focused(&mut self, f: bool) {
        if f && !self.focused {
            self.cursor_counter = 0;
        }
        self.focused = f;
    }

    pub fn draw_text_box(&self, ui: &mut Ui) {
        ui.draw_rect(self.x - 1, self.y - 1, self.x + self.width + 1, self.y + self.height + 1, -6250336);
        ui.draw_rect(self.x, self.y, self.x + self.width, self.y + self.height, -16777216);
        let ty = self.y + (self.height - 8) / 2;
        if self.enabled {
            let cursor = self.focused && self.cursor_counter / 6 % 2 == 0;
            let mut s = self.display_units();
            if cursor { s.push(b'_' as u16); }
            ui.font.draw_utf16_with_shadow(&mut ui.gl, &s, self.x + 4, ty, 14737632);
        } else {
            let s = self.display_units();
            ui.font.draw_utf16_with_shadow(&mut ui.gl, &s, self.x + 4, ty, 7368816);
        }
    }
}
