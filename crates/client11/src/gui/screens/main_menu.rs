//! `GuiMainMenu`

use crate::gui::widgets::GuiButton;
use crate::gui::{Action, Screen, ScreenBase, Ui};
use worldgen::math::{self, PI_F32};

#[path = "main_menu_check10.rs"]
mod check10;
pub use check10::check;
const VERSION_TEXT:&str="Minecraft 1.1";

#[cfg(test)]
mod language_menu11_tests {
 use super::*;
 struct Input{bytes:Vec<u8>,at:usize}
 impl Input {fn byte(&mut self)->bool{let n=self.bytes[self.at];self.at+=1;n!=0}fn int(&mut self)->i32{let end=self.at+4;let n=i32::from_be_bytes(self.bytes[self.at..end].try_into().unwrap());self.at=end;n}fn text(&mut self)->String{let n=self.int();let units:Vec<_>=(0..n).map(|_|{let end=self.at+2;let n=u16::from_be_bytes(self.bytes[self.at..end].try_into().unwrap());self.at=end;n}).collect();String::from_utf16(&units).unwrap()}fn buttons(&mut self,buttons:&[GuiButton]){assert_eq!(self.int(),buttons.len() as i32);for b in buttons{assert_eq!((self.int(),self.int(),self.int(),self.int(),self.int()),(b.id,b.x,b.y,b.width,b.height));assert_eq!(self.byte(),b.enabled);assert_eq!(self.byte(),b.visible);assert_eq!(self.text(),b.text);}}}
 fn saved(dir:&std::path::Path)->String{std::fs::read_to_string(dir.join("options.txt")).ok().and_then(|s|s.lines().find_map(|line|line.strip_prefix("lang:").map(str::to_owned))).unwrap_or("<missing>".into())}
 fn check_child(ui:&mut Ui,parent:Box<dyn Screen>,expected:bool,w:i32,h:i32){
  let pointer=&*parent as *const dyn Screen as *const ();let Some(crate::gui::Action::DisplayChild(make))=ui.actions.pop() else{assert!(!expected);return;};assert!(expected);let mut child=make(parent);crate::gui::set_world_and_resolution(&mut *child,ui,w,h);assert_eq!(child.base().buttons[0].id,6);child.action_performed(ui,6);match ui.actions.pop().unwrap(){crate::gui::Action::Display(Some(returned))=>assert_eq!((&*returned as *const dyn Screen as *const ()),pointer),_=>panic!("language parent transition")}
 }
 #[test]
 #[ignore="requires original SMALL_GUI_MENUDRAW11_FIXTURE and installed assets"]
 fn original_menu_version_decorations(){std::fs::create_dir_all("target/r10gui2").unwrap();assert!(crate::gui::check10_round2::check("menudraw",&std::env::var("SMALL_GUI_MENUDRAW11_FIXTURE").unwrap()));}
 #[test]
 #[ignore="requires original SMALL_GUI_LANGUAGE11_FIXTURE and installed assets"]
 fn original_menu_options_and_language_button(){
  let mut input=Input{bytes:std::fs::read(std::env::var("SMALL_GUI_LANGUAGE11_FIXTURE").unwrap()).unwrap(),at:0};assert_eq!(input.int(),0x53474c31);let mut ui=crate::gui::utility_check::fixture_ui("target/r11/small-gui-menu".into());let mut jar=crate::assets::Jar::open().unwrap();ui.lang.load_languages(&jar.bytes("lang/languages.txt").unwrap());
  let menus=input.int();for _ in 0..menus{let code=input.text();ui.set_language(&code).unwrap();let(w,h)=(input.int(),input.int());ui.hide_quit_button=input.byte();ui.actions.clear();let mut screen=Box::new(GuiMainMenu::with_splash("fixture"));crate::gui::set_world_and_resolution(&mut *screen,&mut ui,w,h);input.buttons(&screen.base.buttons);screen.action_performed(&mut ui,5);assert_eq!(input.int(),ui.actions.len() as i32);let returned=input.byte();check_child(&mut ui,screen,returned,w,h);}
  let options=input.int();for case in 0..options {let code=input.text();ui.set_language(&code).unwrap();let(w,h)=(input.int(),input.int());let enabled=input.byte();let dir=std::path::PathBuf::from(format!("target/r11/small-gui-options-{case}"));std::fs::create_dir_all(&dir).unwrap();let _=std::fs::remove_file(dir.join("options.txt"));ui.settings=crate::gui::settings::GameSettings::load(&dir);ui.settings.language=code;ui.actions.clear();let mut screen=Box::new(super::super::options::GuiOptions::new(Box::new(GuiMainMenu::with_splash("parent"))));crate::gui::set_world_and_resolution(&mut *screen,&mut ui,w,h);input.buttons(&screen.base().buttons);screen.base().button(102).enabled=enabled;screen.action_performed(&mut ui,102);assert_eq!(input.int(),ui.actions.len() as i32);let returned=input.byte();assert_eq!(input.text(),saved(&dir));assert_eq!(input.text(),saved(&dir));check_child(&mut ui,screen,returned,w,h);}
  let ingame=input.int();for _ in 0..ingame{let code=input.text();ui.set_language(&code).unwrap();let(w,h)=(input.int(),input.int());let mut screen=super::super::ingame_menu::GuiIngameMenu::new();crate::gui::set_world_and_resolution(&mut screen,&mut ui,w,h);input.buttons(&screen.base().buttons);}
  ui.textures.insert("gui/gui.png".into(),0);
  let buttons=input.int();for _ in 0..buttons {let(x,y,mx,my)=(input.int(),input.int(),input.int(),input.int());let mut button=GuiButton::language(5,x,y);button.visible=input.byte();button.enabled=input.byte();ui.gl.begin_frame();button.draw_button(&mut ui,x+mx,y+my);let count=input.int();assert_eq!(count as usize,ui.gl.draws.len());for draw in &ui.gl.draws{let(x,y,u,v,w,h)=(input.int(),input.int(),input.int(),input.int(),input.int(),input.int());for (index,px,py,tu,tv) in [(0,x,y+h,u,v+h),(1,x+w,y+h,u+w,v+h),(2,x+w,y,u+w,v),(5,x,y,u,v)]{let vertex=&ui.gl.verts[draw.first as usize+index];assert_eq!(vertex.pos,[px as f32,py as f32,0.0]);assert_eq!(vertex.uv,[tu as f32/256.0,tv as f32/256.0]);assert_eq!(vertex.color,[1.0;4]);}}}
  assert_eq!(input.at,input.bytes.len());println!("SMALL GUI LANGUAGE EXACT: {menus} menus,{options} options save/parent callbacks,{buttons} language-button visibility/enabled/hover boundaries");
 }
}

pub struct GuiMainMenu {
    base: ScreenBase,
    update_counter: f32,
    pub splash_text: String,
    panorama_ticks: i32,
}

impl GuiMainMenu {
    /// `rand` stands in for the class's static unseeded `Random`.
    pub fn new(ui: &Ui, rand: &mut worldgen::JavaRandom) -> GuiMainMenu {
        let splash_text = if ui.splashes.is_empty() {
            "missingno".to_string()
        } else {
            loop {
                let splash = &ui.splashes[rand.next_int(ui.splashes.len() as i32) as usize];
                let hash = splash
                    .encode_utf16()
                    .fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(c as i32));
                if hash != 125780783 {
                    break splash.clone();
                }
            }
        };
        GuiMainMenu {
            base: ScreenBase::default(),
            update_counter: rand.next_float(),
            splash_text,
            panorama_ticks: 0,
        }
    }

    pub fn with_splash(splash: &str) -> GuiMainMenu {
        GuiMainMenu {
            base: ScreenBase::default(),
            update_counter: 0.5,
            splash_text: splash.to_string(),
            panorama_ticks: 0,
        }
    }
}

/// Calendar.getInstance uses the system's local time zone.
fn month_day() -> (u32, u32) {
    use chrono::Datelike;
    let date = chrono::Local::now();
    (date.month(), date.day())
}

impl Screen for GuiMainMenu {
    fn base(&mut self) -> &mut ScreenBase {
        &mut self.base
    }

    fn is_main_menu(&self) -> bool {
        true
    }

    fn does_gui_pause_game(&self) -> bool {
        false
    }

    fn update_screen(&mut self, _ui: &mut Ui) {
        self.panorama_ticks = self.panorama_ticks.wrapping_add(1);
    }

    fn key_typed(&mut self, _ui: &mut Ui, _c: char, _key: i32) {}

    fn init_gui(&mut self, ui: &mut Ui) {
        if !ui.fixed_date {
            match month_day() {
                (11, 9) => self.splash_text = "Happy birthday, ez!".into(),
                (6, 1) => self.splash_text = "Happy birthday, Notch!".into(),
                (12, 24) => self.splash_text = "Merry X-mas!".into(),
                (1, 1) => self.splash_text = "Happy new year!".into(),
                _ => {}
            }
        }
        let (w, h) = (self.base.width, self.base.height);
        let y = h / 4 + 48;
        let b = &mut self.base.buttons;
        b.push(GuiButton::new(
            1,
            w / 2 - 100,
            y,
            ui.translate("menu.singleplayer"),
        ));
        let mut mp = GuiButton::new(2, w / 2 - 100, y + 24, ui.translate("menu.multiplayer"));
        b.push(GuiButton::new(
            3,
            w / 2 - 100,
            y + 48,
            ui.translate("menu.mods"),
        ));
        if ui.hide_quit_button {
            b.push(GuiButton::new(
                0,
                w / 2 - 100,
                y + 72,
                ui.translate("menu.options"),
            ));
        } else {
            b.push(GuiButton::sized(
                0,
                w / 2 - 100,
                y + 72 + 12,
                98,
                20,
                ui.translate("menu.options"),
            ));
            b.push(GuiButton::sized(
                4,
                w / 2 + 2,
                y + 72 + 12,
                98,
                20,
                ui.translate("menu.quit"),
            ));
        }
        // Singleplayer only: there is no session to join servers with (mc.session == null).
        mp.enabled = false;
        b.insert(1, mp);
        b.push(GuiButton::language(5,w/2-124,y+72+12));
    }

    fn action_performed(&mut self, ui: &mut Ui, id: i32) {
        match id {
            0 => ui.display_child(|me| Box::new(super::options::GuiOptions::new(me))),
            1 => ui.display_child(|me| Box::new(super::select_world::GuiSelectWorld::new(me))),
            3 => ui.display_child(|me| Box::new(super::texture_packs::GuiTexturePacks::new(me))),
            4 => ui.actions.push(Action::Shutdown),
            5 => ui.display_child(|me|Box::new(super::language::GuiLanguage::new(me))),
            _ => {}
        }
    }

    fn draw_screen(&mut self, ui: &mut Ui, mx: i32, my: i32, partial: f32) {
        let (w, h) = (self.base.width, self.base.height);
        crate::gui::panorama::draw(ui, w, h, self.panorama_ticks as f32 + partial);
        ui.draw_gradient_rect(0, 0, w, h, -2130706433, 16777215);
        ui.draw_gradient_rect(0, 0, w, h, 0, i32::MIN);
        let t = ui.texture("title/mclogo.png");
        ui.gl.bind_texture(t);
        ui.gl.color4f(1.0, 1.0, 1.0, 1.0);
        for [x, y, u, v, w, h] in self.logo_rectangles(w) { ui.draw_textured_modal_rect(x, y, u, v, w, h); }
        ui.gl.set_color_opaque_i(16777215);
        ui.gl.push_matrix();
        ui.gl.translatef((w / 2 + 90) as f32, 70.0, 0.0);
        ui.gl.rotatef(-20.0, 0.0, 0.0, 1.0);
        let mut s =
            1.8f32 - (math::sin((ui.millis % 1000) as f32 / 1000.0 * PI_F32 * 2.0) * 0.1).abs();
        s = s * 100.0 / (ui.string_width(&self.splash_text) + 32) as f32;
        ui.gl.scalef(s, s, s);
        let splash = self.splash_text.clone();
        ui.draw_centered_string(&splash, 0, -8, 16776960);
        ui.gl.pop_matrix();
        ui.draw_string(VERSION_TEXT, 2, h - 10, 16777215);
        let c = "Copyright Mojang AB. Do not distribute!";
        let cw = ui.string_width(c);
        ui.draw_string(c, w - cw - 2, h - 10, 16777215);
        let _ = partial;
        self.draw_buttons(ui, mx, my);
    }
}

impl GuiMainMenu {
    fn logo_rectangles(&self, width: i32) -> Vec<[i32; 6]> {
        let x = width / 2 - 274 / 2; let y = 30; let mut out = Vec::new();
        if (self.update_counter as f64) < 1.0E-4 {
            out.push([x, y, 0, 0, 99, 44]);
            out.push([x + 99, y, 129, 0, 27, 44]);
            out.push([x + 125, y, 126, 0, 3, 44]);
            out.push([x + 128, y, 99, 0, 26, 44]);
        } else {
            out.push([x, y, 0, 0, 155, 44]);
        }
        out.push([x + 155, y, 0, 45, 155, 44]);
        out
    }
    pub(crate) fn check_decoration10(ui: &mut Ui, out: &mut Vec<u8>) {
        use crate::gui::check10_round2::{i, s};
        for counter in [0.0, 1e-6, 1e-4, 0.5, -1.0, f32::NAN] {
            let mut g = Self::with_splash("fixture");g.update_counter=counter;
            i(out,2);for row in [[0,0,427,240,-2130706433,16777215],[0,0,427,240,0,i32::MIN]] {for n in row {i(out,n);}}
            let rects=g.logo_rectangles(427);i(out,rects.len() as i32);for row in rects {for n in row {i(out,n);}}
            i(out,2);
            for (text,x,y,color) in [(VERSION_TEXT,2,230,16777215),("Copyright Mojang AB. Do not distribute!",427-ui.string_width("Copyright Mojang AB. Do not distribute!")-2,230,16777215)] {s(out,text);for n in [x,y,color] {i(out,n);}}
        }
    }
}
