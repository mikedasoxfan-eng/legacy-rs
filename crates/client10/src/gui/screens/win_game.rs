//! The release End poem and credits. Text is loaded from the user's 1.0 jar.
use crate::gui::{gl::Factor, Action, Screen, ScreenBase, Ui};
use worldgen::JavaRandom;

pub struct GuiWinGame {
    base: ScreenBase,
    pub ticks: i32,
    pub lines: Option<Vec<String>>,
    pub heights: Vec<i32>,
    pub total_height: i32,
    username: String,
}
impl GuiWinGame {
    pub fn new(username: impl Into<String>) -> Self {
        Self {
            base: Default::default(),
            ticks: 0,
            lines: None,
            heights: Vec::new(),
            total_height: 0,
            username: username.into(),
        }
    }
    fn finish(&self, ui: &mut Ui) {
        ui.display(None);
        ui.actions.push(Action::EndReturn);
    }
    pub fn completion_tick(&self) -> f32 {
        self.total_height
            .wrapping_add(self.base.height)
            .wrapping_add(self.base.height)
            .wrapping_add(24) as f32
            / 0.5
    }
    pub fn background_values(&self, partial: f32) -> [f32; 3] {
        let time = self.ticks as f32 + partial;
        let bottom = 0.0 - time * 0.5 * 0.5;
        let top = self.base.height as f32 - time * 0.5 * 0.5;
        let mut fade = (time - 0.0) * 0.02;
        let end = (self.completion_tick() - 20.0 - time) * 0.005;
        if end < fade {
            fade = end;
        }
        if fade > 1.0 {
            fade = 1.0;
        }
        fade *= fade;
        fade = fade * 96.0 / 255.0;
        [bottom * 0.015625, top * 0.015625, fade]
    }
}
/// Both resources double blank lines; sixteen additional blanks separate them.
pub fn parse_text(poem: &str, credits: &str, username: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut random = JavaRandom::new(8124371);
    for line in poem.lines() {
        let mut line = line.replace("PLAYERNAME", username);
        while let Some(i) = line.find("\u{a7}f\u{a7}k\u{a7}a\u{a7}b") {
            let count = random.next_int(4) + 3;
            line.replace_range(
                i..i + 12,
                &format!("\u{a7}f\u{a7}k{}", "X".repeat(count as usize)),
            );
        }
        if line.is_empty() {
            out.push(String::new());
        }
        out.push(line);
    }
    out.extend((0..16).map(|_| String::new()));
    for line in credits.lines() {
        let line = line.replace("PLAYERNAME", username).replace('\t', "    ");
        if line.is_empty() {
            out.push(String::new());
        }
        out.push(line);
    }
    out
}
impl Screen for GuiWinGame {
    fn base(&mut self) -> &mut ScreenBase {
        &mut self.base
    }
    fn init_gui(&mut self, ui: &mut Ui) {
        if self.lines.is_some() {
            return;
        }
        self.lines = Some(Vec::new());
        let result = (|| {
            let mut jar = crate::assets::Jar::open()?;
            Ok::<_, String>(parse_text(
                &jar.text("title/win.txt")?,
                &jar.text("title/credits.txt")?,
                &self.username,
            ))
        })();
        match result {
            Ok(lines) => {
                ui.font.line_height = 12;
                self.heights = lines
                    .iter()
                    .map(|s| ui.font.split_string_height(s, 274))
                    .collect();
                self.total_height = self
                    .heights
                    .iter()
                    .fold(self.total_height, |h, v| h.wrapping_add(*v));
                ui.font.line_height = 8;
                self.lines = Some(lines);
            }
            Err(e) => eprintln!("GuiWinGame: {e}"),
        }
    }
    fn update_screen(&mut self, ui: &mut Ui) {
        self.ticks = self.ticks.wrapping_add(1);
        if self.ticks as f32 > self.completion_tick() {
            self.finish(ui);
        }
    }
    fn key_typed(&mut self, ui: &mut Ui, _c: char, key: i32) {
        if key == 1 {
            self.finish(ui);
        }
    }
    fn action_performed(&mut self, _ui: &mut Ui, _id: i32) {}
    fn draw_screen(&mut self, ui: &mut Ui, mx: i32, my: i32, partial: f32) {
        let (w, h) = (self.base.width, self.base.height);
        self.render_background(ui, partial);
        let x = w / 2 - 274 / 2;
        let logo_y = h + 50;
        let scroll = -(self.ticks as f32 + partial) * 0.5;
        ui.gl.push_matrix();
        ui.gl.translatef(0.0, scroll, 0.0);
        ui.gl.bind_texture(ui.texture("title/mclogo.png"));
        ui.gl.color4f(1.0, 1.0, 1.0, 1.0);
        ui.draw_textured_modal_rect(x, logo_y, 0, 0, 155, 44);
        ui.draw_textured_modal_rect(x + 155, logo_y, 0, 45, 155, 44);
        ui.gl.set_color_opaque_i(16777215);
        ui.font.line_height = 12;
        for command in self.line_draws(partial, |text|ui.font.get_string_width(text)) {
            match command {
                WinDraw::Translate(y) => ui.gl.translatef(0.0,y,0.0),
                WinDraw::Text { text, kind, x, y, seed } => {
                    if kind==1 {ui.font.draw_string_with_shadow(&mut ui.gl,&text,x,y,16777215);}
                    else {
                        ui.font.random.lock().unwrap().set_seed(seed);
                        ui.font.draw_split_shadow(&mut ui.gl,&text,x,y,274,16777215,kind==2);
                    }
                }
            }
        }
        ui.font.line_height = 8;
        ui.gl.pop_matrix();
        ui.gl.bind_texture(ui.texture("%blur%/misc/vignette.png"));
        ui.gl.blend = true;
        ui.gl.blend_func(Factor::Zero, Factor::OneMinusSrcColor);
        ui.gl.start_drawing_quads();
        ui.gl.set_color_rgba_f(1.0, 1.0, 1.0, 1.0);
        for (x, y, u, v) in [
            (0, h, 0.0, 1.0),
            (w, h, 1.0, 1.0),
            (w, 0, 1.0, 0.0),
            (0, 0, 0.0, 0.0),
        ] {
            ui.gl.add_vertex_with_uv(x as f64, y as f64, 0.0, u, v);
        }
        ui.gl.draw();
        ui.gl.blend = false;
        self.draw_buttons(ui, mx, my);
    }
}

impl GuiWinGame {
    pub(crate) fn render_background(&self, ui: &mut Ui, partial: f32) {
        let (w, h) = (self.base.width, self.base.height);
        let [bottom, top, fade] = self.background_values(partial);
        ui.gl.bind_texture(ui.texture("%blur%/gui/background.png"));
        ui.gl.start_drawing_quads();
        ui.gl.set_color_rgba_f(fade, fade, fade, 1.0);
        for (x, y, u, v) in [
            (0, h, 0.0, bottom),
            (w, h, w as f32 * 0.015625, bottom),
            (w, 0, w as f32 * 0.015625, top),
            (0, 0, 0.0, top),
        ] {
            ui.gl
                .add_vertex_with_uv(x as f64, y as f64, 0.0, u as f64, v as f64);
        }
        ui.gl.draw();
    }
}

#[derive(Debug)]
pub(crate) enum WinDraw {
    Translate(f32),
    Text { text:String,kind:i32,x:i32,y:i32,seed:i64 },
}
impl GuiWinGame {
    pub(crate) fn line_draws(&self,partial:f32,width:impl Fn(&str)->i32)->Vec<WinDraw> {
        let h=self.base.height;let x=self.base.width/2-137;let scroll=-(self.ticks as f32+partial)*0.5;
        let mut y=h.wrapping_add(250);let mut draws=Vec::new();
        if let Some(lines)=&self.lines {for (index,(line,height)) in lines.iter().zip(&self.heights).enumerate() {
            if index==lines.len()-1 {let shift=y as f32+scroll-(h/2-height/2) as f32;if shift<0.0 {draws.push(WinDraw::Translate(-shift));}}
            if y as f32+scroll+*height as f32+8.0>0.0&&y as f32+scroll<h as f32 {
                if let Some(text)=line.strip_prefix("[C]") {draws.push(WinDraw::Text{text:text.into(),kind:1,x:x+(274-width(text))/2,y,seed:0});}
                else {let seed=(index as i64).wrapping_mul(4238972211).wrapping_add((self.ticks/4) as i64);for kind in [2,3] {let offset=if kind==2{1}else{0};draws.push(WinDraw::Text{text:line.clone(),kind,x:x+offset,y:y+offset,seed});}}
            }
            y=y.wrapping_add(*height);
        }}
        draws
    }
}
