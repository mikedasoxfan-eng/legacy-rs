//! GuiChat. EntityPlayerSP.sendChatMessage is empty in singleplayer.
use crate::gui::{Screen, ScreenBase, Ui};

#[derive(Default)]
pub struct GuiChat {
    base: ScreenBase,
    // Java accepts UTF-16 units, including a lone surrogate at the 100-unit limit.
    message: Vec<u16>,
    update_counter: i32,
}

impl GuiChat {
    fn key_unit(&mut self, ui: &mut Ui, unit: u16, key: i32) {
        if key == 1 || key == 28 {
            ui.display(None);
        } else {
            if key == 14 { self.message.pop(); }
            let allowed = unit > 32 || char::from_u32(unit as u32).is_some_and(|c| ui.font_allows(c));
            if allowed && self.message.len() < 100 { self.message.push(unit); }
        }
    }
}

impl Screen for GuiChat {
    fn repeats_keys(&self) -> bool { true }
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
        for &unit in c.encode_utf16(&mut [0; 2]).iter() { self.key_unit(ui, unit, key); }
    }
    fn draw_screen(&mut self, ui: &mut Ui, mx: i32, my: i32, _partial: f32) {
        let (w, h) = (self.base.width, self.base.height);
        ui.draw_rect(2, h - 14, w - 2, h - 2, i32::MIN);
        let mut text = vec![b'>' as u16, b' ' as u16];
        text.extend_from_slice(&self.message);
        if self.update_counter / 6 % 2 == 0 { text.push(b'_' as u16); }
        ui.font.draw_utf16_with_shadow(&mut ui.gl, &text, 4, h - 12, 14737632);
        self.draw_buttons(ui, mx, my);
    }
}

#[cfg(test)]
mod remaining_gui11_tests {
    use super::*;
    use crate::gui::widgets::GuiTextField;
    struct Input { bytes: Vec<u8>, at: usize }
    impl Input {
        fn int(&mut self) -> i32 { let n=i32::from_be_bytes(self.bytes[self.at..self.at+4].try_into().unwrap());self.at+=4;n }
        fn byte(&mut self) -> bool {let n=self.bytes[self.at];self.at+=1;n!=0}
        fn units(&mut self) -> Option<Vec<u16>> {let n=self.int();if n<0{return None;}Some((0..n).map(|_|{let v=u16::from_be_bytes(self.bytes[self.at..self.at+2].try_into().unwrap());self.at+=2;v}).collect())}
    }
    #[test]
    #[ignore="requires original UNICODE_INPUT11_FIXTURE and installed assets"]
    fn original_unicode_input_units_and_boundaries() {
        let mut input=Input{bytes:std::fs::read(std::env::var("UNICODE_INPUT11_FIXTURE").unwrap()).unwrap(),at:0};
        assert_eq!(input.int(),0x55494e31);
        let mut ui=crate::gui::utility_check::fixture_ui("target/r11/unicode-input".into());
        let units=input.int();
        for unit in 0..units {
            let expected=input.units().unwrap();let mut chat=GuiChat::default();chat.key_unit(&mut ui,unit as u16,0);assert_eq!(chat.message,expected,"chat {unit}");
            let expected=input.units();if let Some(expected)=expected {let mut field=GuiTextField::new(0,0,200,20,"");field.focused=true;field.max_length=32;field.key_unit(&ui,unit as u16,0,None);assert_eq!(field.display_units(),expected,"field {unit}");}
        }
        let boundaries=input.int();
        for _ in 0..boundaries {
            let field=input.byte();let initial=input.int();let unit=input.int();let key=input.int();let expected=input.units().unwrap();
            if field {let mut field=GuiTextField::new(0,0,200,20,"A".repeat(initial as usize));field.focused=true;field.max_length=100;field.key_unit(&ui,unit as u16,key,None);assert_eq!(field.display_units(),expected);}else{let mut chat=GuiChat{message:vec![65;initial as usize],..Default::default()};chat.key_unit(&mut ui,unit as u16,key);assert_eq!(chat.message,expected);}
        }
        let events=input.int();let mut chat=GuiChat::default();let mut field=GuiTextField::new(0,0,200,20,"");field.focused=true;field.max_length=100;
        for _ in 0..events {let is_field=input.byte();let unit=input.int();let key=input.int();let expected=input.units().unwrap();if is_field{field.key_unit(&ui,unit as u16,key,None);assert_eq!(field.display_units(),expected);}else{chat.key_unit(&mut ui,unit as u16,key);assert_eq!(chat.message,expected);}}
        assert_eq!(input.at,input.bytes.len());
        // Exercise the actual scalar-event adapters, including truncation after
        // the high surrogate and subsequent backspace of one Java char.
        for initial in [0,99,100] {let mut chat=GuiChat{message:vec![65;initial],..Default::default()};chat.key_typed(&mut ui,'\u{1f600}',0);let mut expected=vec![65;initial];for unit in [0xd83d,0xde00] {if expected.len()<100{expected.push(unit);}}assert_eq!(chat.message,expected);let mut field=GuiTextField::new(0,0,200,20,"A".repeat(initial));field.focused=true;field.max_length=100;field.key_typed(&ui,'\u{1f600}',0,||None);assert_eq!(field.display_units(),expected);chat.key_typed(&mut ui,'\0',14);field.key_typed(&ui,'\0',14,||None);expected.pop();assert_eq!(chat.message,expected);assert_eq!(field.display_units(),expected);}
        field.text="external".into();field.key_typed(&ui,'\u{100}',0,||None);assert_eq!(field.text,"external\u{100}");
        println!("UNICODE INPUT EXACT: {units} UTF16 units, {boundaries} length/backspace boundaries, {events} sequential surrogate edits; scalar production adapters");
    }
}
