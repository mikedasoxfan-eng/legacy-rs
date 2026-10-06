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
        FontRenderer { char_width, texture, allowed, colors, line_height: 8, random: std::sync::Mutex::new(worldgen::JavaRandom::new(0)) }
    }

    pub fn allows(&self, c: char) -> bool {
        self.allowed.contains(&c)
    }

    /// Font display lists on the world renderer's recorded GL stream.
    pub fn draw_fixed(&self, g: &mut crate::render::entity::fixed::Fixed, s: &str, x: i32, y: i32, color: i32) {
        g.texture=14;
        let alpha=if color>>24&255==0 {1.0} else {(color>>24&255) as f32/255.0};
        g.material=[(color>>16&255) as f32/255.0,(color>>8&255) as f32/255.0,(color&255) as f32/255.0,alpha];
        g.push();g.translate(x as f32,y as f32,0.0);
        for command in self.display_list_commands(s, false) {
            if command >= 256 {
                let [r, green, b] = self.colors[(command - 256) as usize];
                g.material = [r, green, b, 1.0];
            } else {
                let c = command as usize; let u = (c % 16 * 8) as f32; let v = (c / 16 * 8) as f32; let size = 7.99;
                g.begin();g.quad([[0.0,size,0.0],[size,size,0.0],[size,0.0,0.0],[0.0,0.0,0.0]],[[u/128.0,(v+size)/128.0],[(u+size)/128.0,(v+size)/128.0],[(u+size)/128.0,v/128.0],[u/128.0,v/128.0]]);g.end();
                g.translate(self.char_width[c] as f32, 0.0, 0.0);
            }
        }
        g.pop();
    }

    fn index(&self, c: char) -> Option<usize> {
        self.allowed.iter().position(|&a| a == c)
    }

    /// The glyph display list for character slot `c`: one quad, then translate by its width.
    fn glyph(&self, gl: &mut Gl, c: usize) {
        let col = (c % 16 * 8) as f32;
        let row = (c / 16 * 8) as f32;
        let s = 7.99f32;
        gl.start_drawing_quads();
        gl.add_vertex_with_uv(0.0, (0.0f32 + s) as f64, 0.0, (col / 128.0 + 0.0) as f64, ((row + s) / 128.0 + 0.0) as f64);
        gl.add_vertex_with_uv((0.0f32 + s) as f64, (0.0f32 + s) as f64, 0.0, ((col + s) / 128.0 + 0.0) as f64, ((row + s) / 128.0 + 0.0) as f64);
        gl.add_vertex_with_uv((0.0f32 + s) as f64, 0.0, 0.0, ((col + s) / 128.0 + 0.0) as f64, (row / 128.0 + 0.0) as f64);
        gl.add_vertex_with_uv(0.0, 0.0, 0.0, (col / 128.0 + 0.0) as f64, (row / 128.0 + 0.0) as f64);
        gl.draw();
        gl.translatef(self.char_width[c] as f32, 0.0, 0.0);
    }

    pub fn draw_string_with_shadow(&self, gl: &mut Gl, s: &str, x: i32, y: i32, color: i32) {
        super::draw_trace10::record(||super::draw_trace10::Call::Text(s.into(),[1,x,y,0,color]));
        self.render_string(gl, s, x + 1, y + 1, color, true);
        self.render_string(gl, s, x, y, color, false);
    }

    pub fn draw_string(&self, gl: &mut Gl, s: &str, x: i32, y: i32, color: i32) {
        super::draw_trace10::record(||super::draw_trace10::Call::Text(s.into(),[0,x,y,0,color]));
        self.render_string(gl, s, x, y, color, false);
    }

    pub fn render_string(&self, gl: &mut Gl, s: &str, x: i32, y: i32, mut color: i32, shadow: bool) {
        if shadow {
            let a = color & -16777216;
            color = (color & 16579836) >> 2;
            color += a;
        }
        gl.bind_texture(self.texture);
        let r = (color >> 16 & 255) as f32 / 255.0;
        let g = (color >> 8 & 255) as f32 / 255.0;
        let b = (color & 255) as f32 / 255.0;
        let mut a = (color >> 24 & 255) as f32 / 255.0;
        if a == 0.0 {
            a = 1.0;
        }
        gl.color4f(r, g, b, a);
        gl.push_matrix();
        gl.translatef(x as f32, y as f32, 0.0);
        for command in self.display_list_commands(s, shadow) {
            if command >= 256 {
                let [r, g, b] = self.colors[(command - 256) as usize];
                gl.color3f(r, g, b);
            } else {
                self.glyph(gl, command as usize);
            }
        }
        gl.pop_matrix();
    }

    /// Original display-list offsets; also used by the fixed renderer and bytecode fixture.
    pub fn display_list_commands(&self, s: &str, shadow: bool) -> Vec<i32> {
        let chars: Vec<_> = s.chars().collect();
        let mut commands = Vec::new();
        let mut obfuscated = false;
        let mut i = 0;
        while i < chars.len() {
            while i + 1 < chars.len() && chars[i] == '\u{a7}' {
                let code = chars[i + 1].to_lowercase().next().unwrap();
                if code == 'k' { obfuscated = true; }
                else {
                    obfuscated = false;
                    commands.push(256 + "0123456789abcdef".find(code).unwrap_or(15) as i32 + if shadow { 16 } else { 0 });
                }
                i += 2;
            }
            if i < chars.len() {
                if let Some(k) = self.index(chars[i]) {
                    let mut glyph = k;
                    if obfuscated {
                        let mut random = self.random.lock().unwrap();
                        loop {
                            glyph = random.next_int(self.allowed.len() as i32) as usize;
                            if self.char_width[k + 32] == self.char_width[glyph + 32] { break; }
                        }
                        commands.push(256 + random.next_int(2) + 8 + if shadow { 16 } else { 0 });
                    }
                    commands.push(glyph as i32 + 32);
                }
            }
            i += 1;
        }
        commands
    }

    pub fn get_string_width(&self, s: &str) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut w = 0;
        let mut i = 0;
        while i < chars.len() {
            if chars[i] == '§' {
                i += 1;
            } else if let Some(k) = self.index(chars[i]) {
                if k + 32 < 256 {
                    w += self.char_width[k + 32];
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
