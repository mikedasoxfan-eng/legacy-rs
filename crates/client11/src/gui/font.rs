//! `FontRenderer`: glyph widths measured from `font/default.png`, the 7.99-unit glyph quads
//! (originally display lists that also advance the modelview), shadows and `§` color codes.

use super::gl::{Gl, TexId};
use crate::render::gpu::Image;

pub struct FontRenderer {
    char_width: [i32; 256],
    pub line_height: i32,
    pub random: std::sync::Mutex<worldgen::JavaRandom>,
    pub texture: TexId,
    /// `ChatAllowedCharacters.allowedCharacters` (from `font.txt`).
    allowed: Vec<char>,
    /// The 32 color display lists: 16 normal then 16 shadow colors.
    colors: [[f32; 3]; 32],
    glyph_sizes: Vec<i8>,
    pub unicode: bool,
    pub bidi: bool,
    pub glyph_pages: std::sync::Mutex<[Option<TexId>; 256]>,
}

impl FontRenderer {
    pub fn new(img: &Image, texture: TexId, font_txt: &str) -> FontRenderer {
        Self::new_with_anaglyph(img, texture, font_txt, false)
    }

    pub fn new_with_anaglyph(img: &Image, texture: TexId, font_txt: &str, anaglyph: bool) -> FontRenderer {
        let w = img.width as usize;
        // BufferedImage.getRGB & 255 = blue channel.
        let blue = |x: usize, y: usize| img.rgba[(y * w + x) * 4 + 2] as i32;
        let mut char_width = [0i32; 256];
        for (c, cw) in char_width.iter_mut().enumerate() {
            let (col, row) = (c % 16, c / 16);
            let mut x = 7i32;
            while x >= 0 {
                let px = col * 8 + x as usize;
                let empty = (0..8).all(|y| blue(px, row * 8 + y) <= 0);
                if !empty {
                    break;
                }
                x -= 1;
            }
            if c == 32 {
                x = 2;
            }
            *cw = x + 2;
        }
        let mut colors = [[0.0f32; 3]; 32];
        for (i, c) in colors.iter_mut().enumerate() {
            let base = (i as i32 >> 3 & 1) * 85;
            let mut r = (i as i32 >> 2 & 1) * 170 + base;
            let mut g = (i as i32 >> 1 & 1) * 170 + base;
            let mut b = (i as i32 & 1) * 170 + base;
            if i == 6 {
                r += 85;
            }
            if anaglyph {
                let [ar,ag,ab] = [(r * 30 + g * 59 + b * 11) / 100, (r * 30 + g * 70) / 100, (r * 30 + b * 70) / 100];
                r = ar; g = ag; b = ab;
            }
            if i >= 16 {
                r /= 4;
                g /= 4;
                b /= 4;
            }
            *c = [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0];
        }
        let allowed = font_txt
            .lines()
            .filter(|l| !l.starts_with('#'))
            .flat_map(|l| l.chars())
            .collect();
        let glyph_sizes = crate::assets::Jar::open().and_then(|mut jar| jar.bytes("font/glyph_sizes.bin"))
            .expect("Minecraft 1.1 glyph_sizes.bin").into_iter().map(|value| value as i8).collect::<Vec<_>>();
        assert_eq!(glyph_sizes.len(), 65536);
        FontRenderer { char_width, texture, allowed, colors, glyph_sizes, unicode:false, bidi:false,
            glyph_pages:std::sync::Mutex::new([None;256]), line_height: 8, random: std::sync::Mutex::new(worldgen::JavaRandom::new(0)) }
    }

    pub fn allows(&self, c: char) -> bool {
        self.allowed.contains(&c)
    }

    /// Original immediate font strips in the world renderer's recorded GL stream.
    pub fn draw_fixed(&self, g: &mut crate::render::entity::fixed::Fixed, s: &str, x: i32, y: i32, color: i32) {
        self.draw_fixed_utf16(g, &s.encode_utf16().collect::<Vec<_>>(), x, y, color);
    }

    pub fn draw_fixed_utf16(&self, g: &mut crate::render::entity::fixed::Fixed, s: &[u16], x: i32, y: i32, color: i32) {
        let mut gl=Gl::new();
        self.render_utf16(&mut gl,s,x,y,color,false);
        let mode=g.mode;
        for draw in &gl.draws {
            g.texture=match draw.state.texture.unwrap() { t if t==self.texture=>14,t=>t };
            g.material=gl.verts[draw.first as usize].color;
            g.mode=5;g.begin();
            for i in [0,1,2,5] {
                let v=&gl.verts[draw.first as usize+i];
                g.vertex(v.pos.map(|f|f as f64),v.uv.map(|f|f as f64));
            }
            g.end();
        }
        g.material=gl.color;
        g.mode=mode;
    }

    fn index(&self, c: char) -> Option<usize> {
        self.allowed.iter().position(|&a| a == c)
    }

    /// Immediate GL_TRIANGLE_STRIP, preserving the original f32 additions.
    fn glyph_strip(gl: &mut Gl, x: f32, y: f32, width: f32, u: f32, v: f32, uw: f32, vh: f32, atlas: f32) {
        use crate::render::entity::living::EntityVertex;
        let points = [[x,y,0.0],[x,y+7.99,0.0],[x+width,y,0.0],[x+width,y+7.99,0.0]];
        let uv = [[u/atlas,v/atlas],[u/atlas,(v+vh)/atlas],[(u+uw)/atlas,v/atlas],[(u+uw)/atlas,(v+vh)/atlas]];
        let vertices = [0,1,2,2,1,3].map(|i| EntityVertex { pos:points[i], uv:uv[i], color:gl.color });
        gl.triangles(&vertices);
    }

    fn draw_glyph(&self, gl: &mut Gl, command: i32, x: &mut f32, y: f32) {
        if command == 32 { *x += 4.0; return; }
        if command >= 0 {
            let c = command as usize;
            gl.bind_texture(self.texture);
            let width = self.char_width[c] as f32 - 0.01;
            Self::glyph_strip(gl,*x,y,width,(c%16*8) as f32,(c/16*8) as f32,width,7.99,128.0);
            *x += self.char_width[c] as f32;
        } else {
            let c = (-command-1) as usize;
            let size = self.glyph_sizes[c];
            if size == 0 { return; }
            let page = c/256;
            let texture = self.glyph_pages.lock().unwrap()[page].unwrap_or(0x8000_0000 | page as u32);
            gl.bind_texture(texture);
            // Java byte promotion followed by >>>, deliberately different from width's >>.
            let start = ((size as i32 as u32)>>4) as f32;
            let end = ((size as i32 & 15)+1) as f32;
            let u = (c%16*16) as f32 + start;
            let v = ((c&255)/16*16) as f32;
            let width = end-start-0.02;
            Self::glyph_strip(gl,*x,y,width/2.0,u,v,width,15.98,256.0);
            *x += (end-start)/2.0+1.0;
        }
    }

    pub fn draw_string_with_shadow(&self, gl: &mut Gl, s: &str, x: i32, y: i32, color: i32) {
        super::draw_trace10::record(||super::draw_trace10::Call::Text(s.into(),[1,x,y,0,color]));
        self.draw_utf16_with_shadow(gl, &s.encode_utf16().collect::<Vec<_>>(), x, y, color);
    }

    pub fn draw_utf16_with_shadow(&self, gl:&mut Gl, s:&[u16],x:i32,y:i32,color:i32)->f32 {
        self.render_utf16(gl,s,x.wrapping_add(1),y.wrapping_add(1),color,true);
        self.render_utf16(gl,s,x,y,color,false)
    }

    pub fn draw_string(&self, gl: &mut Gl, s: &str, x: i32, y: i32, color: i32) {
        super::draw_trace10::record(||super::draw_trace10::Call::Text(s.into(),[0,x,y,0,color]));
        self.render_string(gl, s, x, y, color, false);
    }

    pub fn render_string(&self, gl: &mut Gl, s: &str, x: i32, y: i32, color: i32, shadow: bool) {
        self.render_utf16(gl, &s.encode_utf16().collect::<Vec<_>>(), x, y, color, shadow);
    }

    pub fn render_utf16(&self, gl: &mut Gl, s: &[u16], x: i32, y: i32, mut color: i32, shadow: bool) -> f32 {
        if color & -67108864 == 0 { color |= -16777216; }
        if shadow { color = (color & 16579836) >> 2 | color & -16777216; }
        gl.color4f((color>>16&255) as f32/255.0,(color>>8&255) as f32/255.0,
            (color&255) as f32/255.0,(color>>24&255) as f32/255.0);
        let mut pos = x as f32;
        let ordered=if self.bidi {super::bidi11::reorder_utf16(s)}else{s.to_vec()};
        for command in self.utf16_commands(&ordered,shadow) {
            if command >= 256 {
                let [r,g,b] = self.colors[(command-256) as usize];
                gl.color3f(r,g,b);
            } else { self.draw_glyph(gl,command,&mut pos,y as f32); }
        }
        pos
    }

    /// Formatting and glyph dispatch from original 1.1 charAt iteration.
    /// Negative commands encode Unicode units; positive slots include the +32 offset.
    pub fn display_list_commands(&self, s: &str, shadow: bool) -> Vec<i32> {
        self.utf16_commands(&s.encode_utf16().collect::<Vec<_>>(),shadow)
    }

    pub fn utf16_commands(&self, chars: &[u16], shadow: bool) -> Vec<i32> {
        let mut commands=Vec::new(); let mut obfuscated=false; let mut i=0;
        while i<chars.len() {
            let c=chars[i];
            if c==167 && i+1<chars.len() {
                let code=char::from_u32(chars[i+1] as u32).unwrap_or('\u{fffd}').to_lowercase().next().unwrap();
                if code=='k' {obfuscated=true;} else {
                    obfuscated=false;
                    commands.push(256+"0123456789abcdef".find(code).unwrap_or(15) as i32+if shadow {16}else {0});
                }
                i+=2; continue;
            }
            let mut index=self.allowed.iter().position(|&a| a as u32==c as u32).map_or(-1,|n|n as i32);
            if obfuscated && index>0 {
                let mut random=self.random.lock().unwrap(); let original=index;
                loop {
                    index=random.next_int(self.allowed.len() as i32);
                    if self.char_width[(original+32) as usize]==self.char_width[(index+32) as usize] {break;}
                }
            }
            if c==32 {commands.push(32);}
            else if index>0 && !self.unicode {commands.push(index+32);}
            else {commands.push(-(c as i32)-1);}
            i+=1;
        }
        commands
    }

    pub fn get_string_width(&self, s: &str) -> i32 {
        self.get_utf16_width(&s.encode_utf16().collect::<Vec<_>>())
    }

    /// Original charAt iteration includes isolated surrogate units.
    pub fn get_utf16_width(&self, chars: &[u16]) -> i32 {
        let mut w = 0;
        let mut i = 0;
        while i < chars.len() {
            if chars[i] == 167 {
                i += 1;
            } else {
                let allowed = self.allowed.iter().position(|&c| c as u32 == chars[i] as u32);
                if let Some(k) = allowed.filter(|_| !self.unicode) {
                    w += self.char_width[k + 32];
                } else {
                    let size = self.glyph_sizes[chars[i] as usize];
                    if size != 0 {
                        let mut start = (size as i32) >> 4;
                        let mut end = (size as i32) & 15;
                        if end > 7 { end = 15; start = 0; }
                        w += (end + 1 - start) / 2 + 1;
                    }
                }
            }
            i += 1;
        }
        w
    }

    /// `drawSplitString` / `func_40609_a`. A newline recursion deliberately clears shadow.
    pub fn draw_split_string(&self, gl: &mut Gl, s: &str, x: i32, y: i32, width: i32, color: i32) {
        self.draw_split_shadow(gl, s, x, y, width, color, false);
    }
    pub fn draw_split_shadow(&self, gl: &mut Gl, s: &str, x: i32, y: i32, width: i32, color: i32, shadow: bool) {
        for (line, y, shadow) in self.split_draw_lines(s, y, width, shadow) {
            self.render_string(gl, &line, x, y, color, shadow);
        }
    }
    pub fn split_draw_lines(&self, s: &str, mut y: i32, width: i32, shadow: bool) -> Vec<(String, i32, bool)> {
        let mut out = Vec::new();
        let lines = java_split(s, '\n');
        if lines.len() > 1 {
            for line in lines {
                out.extend(self.split_draw_lines(line, y, width, false));
                y += self.split_string_height(line, width);
            }
            return out;
        }
        for line in self.wrapped_draw_lines(s, width) {
            out.push((line, y, shadow));
            y += self.line_height;
        }
        out
    }
    pub fn wrapped_draw_lines(&self, s: &str, width: i32) -> Vec<String> {
        let words = java_split(s, ' ');
        let mut out = Vec::new();
        let mut i = 0;
        let mut format = String::new();
        while i < words.len() {
            let mut line = format!("{format}{} ", words[i]); i += 1;
            while i < words.len() && self.get_string_width(&format!("{line}{}", words[i])) < width {
                line.push_str(words[i]); line.push(' '); i += 1;
            }
            while self.get_string_width(&line) > width {
                let chars: Vec<_> = line.chars().collect();
                let mut n = 0;
                while n < chars.len() && self.get_string_width(&chars[..n+1].iter().collect::<String>()) <= width { n += 1; }
                assert!(n > 0, "FontRenderer wrapping width cannot fit a glyph");
                let head: String = chars[..n].iter().collect();
                if !head.trim_matches(|c: char| c <= ' ').is_empty() {
                    retain_format(&head, &mut format);
                    out.push(head);
                }
                line = format!("{format}{}", chars[n..].iter().collect::<String>());
            }
            if self.get_string_width(line.trim_matches(|c: char| c <= ' ')) > 0 {
                retain_format(&line, &mut format);
                out.push(line);
            }
        }
        out
    }

    /// `func_27277_a`: height of the wrapped text.
    pub fn split_string_height(&self, s: &str, width: i32) -> i32 {
        let lines: Vec<&str> = java_split(s, '\n');
        if lines.len() > 1 {
            return lines.iter().map(|l| self.split_string_height(l, width)).sum();
        }
        let mut h = self.wrap(s, width).len() as i32 * self.line_height;
        if h < self.line_height {
            h += self.line_height;
        }
        h
    }

    fn wrap(&self, s: &str, width: i32) -> Vec<String> {
        let words: Vec<&str> = java_split(s, ' ');
        let mut out = Vec::new();
        let mut i = 0;
        while i < words.len() {
            let mut line = format!("{} ", words[i]);
            i += 1;
            while i < words.len() && self.get_string_width(&format!("{line}{}", words[i])) < width {
                line = format!("{line}{} ", words[i]);
                i += 1;
            }
            while self.get_string_width(&line) > width {
                let cs: Vec<char> = line.chars().collect();
                let mut n = 0;
                while self.get_string_width(&cs[..n + 1].iter().collect::<String>()) <= width {
                    n += 1;
                }
                let head: String = cs[..n].iter().collect();
                if !head.trim_matches(|c: char| c <= ' ').is_empty() {
                    out.push(head);
                }
                line = cs[n..].iter().collect();
            }
            if !line.trim_matches(|c: char| c <= ' ').is_empty() {
                out.push(line);
            }
        }
        out
    }
}

/// `String.split(regex)` for a single literal character: trailing empty strings removed.
pub fn java_split(s: &str, c: char) -> Vec<&str> {
    let mut v: Vec<&str> = s.split(c).collect();
    while v.len() > 1 && v.last() == Some(&"") {
        v.pop();
    }
    if v.len() == 1 && v[0].is_empty() {
        return vec![""];
    }
    v
}

fn retain_format(line: &str, format: &mut String) {
    if let Some(i) = line.rfind('\u{a7}') {
        let code = line[i..].chars().nth(1).expect("truncated FontRenderer format");
        *format = format!("\u{a7}{code}");
    }
}
#[cfg(test)]
mod bidi_draw_tests {
 use super::*;
 struct Input{bytes:Vec<u8>,at:usize}
 impl Input{
  fn byte(&mut self)->u8{let b=self.bytes[self.at];self.at+=1;b}
  fn word(&mut self)->u32{(0..4).fold(0,|n,_|(n<<8)|self.byte() as u32)}
  fn long(&mut self)->i64{((self.word() as u64)<<32|self.word() as u64) as i64}
  fn text(&mut self)->Vec<u16>{let len=self.word();(0..len).map(|_|(self.byte() as u16)<<8|self.byte() as u16).collect()}
 }
 #[test]
 #[ignore="requires original1.1 FONT_BIDI11_FIXTURE and installed assets"]
 fn original_public_draw_methods(){
  let mut input=Input{bytes:std::fs::read(std::env::var("FONT_BIDI11_FIXTURE").unwrap()).unwrap(),at:0};assert_eq!(input.word(),0x42464431);let count=input.word();
  let mut jar=crate::assets::Jar::open().unwrap();let image=jar.image("font/default.png").unwrap();let allowed=jar.text("font.txt").unwrap();let mut font=FontRenderer::new(&image,0,&allowed);
  for case in 0..count{
   font.unicode=input.byte()!=0;font.bidi=input.byte()!=0;let shadow=input.byte()!=0;let text=input.text();let (x,y,color)=(input.word() as i32,input.word() as i32,input.word() as i32);let seed=input.long();*font.random.lock().unwrap()=worldgen::JavaRandom::new(seed);
   let mut gl=Gl::new();let end=if shadow{font.draw_utf16_with_shadow(&mut gl,&text,x,y,color)}else{font.render_utf16(&mut gl,&text,x,y,color,false)};
   let vertices=input.word() as usize;assert_eq!(gl.draws.len()*4,vertices,"case{case} vertex count");
   for draw in &gl.draws{for i in [0,1,2,5]{let expected_texture=input.word();let actual_texture=match draw.state.texture.unwrap(){0=>256,n=>n&255};assert_eq!(actual_texture,expected_texture,"case{case} texture");let v=&gl.verts[draw.first as usize+i];for value in v.pos.into_iter().chain(v.uv).chain(v.color){assert_eq!(value.to_bits(),input.word(),"case{case} geometry/color");}}}
   for value in gl.color{assert_eq!(value.to_bits(),input.word(),"case{case} final color");}assert_eq!(end.to_bits(),input.word(),"case{case} end position");assert_eq!(font.random.lock().unwrap().next_long(),input.long(),"case{case} RNG tail");
   if let Ok(valid)=String::from_utf16(&text){*font.random.lock().unwrap()=worldgen::JavaRandom::new(seed);let mut public=Gl::new();if shadow{font.draw_string_with_shadow(&mut public,&valid,x,y,color);}else{font.draw_string(&mut public,&valid,x,y,color);}assert_eq!(public.draws.len(),gl.draws.len());assert_eq!(public.verts.len(),gl.verts.len());for (a,b) in public.verts.iter().zip(&gl.verts){assert_eq!(a.pos,b.pos);assert_eq!(a.uv,b.uv);assert_eq!(a.color,b.color);}}
  }
  assert_eq!(input.at,input.bytes.len());println!("FONT BIDI DRAW EXACT: {count} original public draw cases");
 }
}
