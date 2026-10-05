//! `FontRenderer`: glyph widths measured from `font/default.png`, the 7.99-unit glyph quads
//! (originally display lists that also advance the modelview), shadows and `§` color codes.

use super::gl::{Gl, TexId};
use crate::render::gpu::Image;

pub struct FontRenderer {
    char_width: [i32; 256],
    pub texture: TexId,
    /// `ChatAllowedCharacters.allowedCharacters` (from `font.txt`).
    allowed: Vec<char>,
    /// The 32 color display lists: 16 normal then 16 shadow colors.
    colors: [[f32; 3]; 32],
}

impl FontRenderer {
    pub fn new(img: &Image, texture: TexId, font_txt: &str) -> FontRenderer {
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
        FontRenderer { char_width, texture, allowed, colors }
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
        let chars:Vec<_>=s.chars().collect();let mut i=0;
        while i<chars.len() {
            while i+1<chars.len() && chars[i]=='§' {
                let code=chars[i+1].to_ascii_lowercase();let k="0123456789abcdef".find(code).unwrap_or(15);
                let [r,b,c]=self.colors[k];g.material=[r,b,c,1.0];i+=2;
            }
            if i<chars.len() {
                if let Some(k)=self.index(chars[i]).filter(|k| k+32<256) {
                    let c=k+32;let u=(c%16*8) as f32;let v=(c/16*8) as f32;let s=7.99;
                    g.begin();g.quad([[0.0,s,0.0],[s,s,0.0],[s,0.0,0.0],[0.0,0.0,0.0]],[[u/128.0,(v+s)/128.0],[(u+s)/128.0,(v+s)/128.0],[(u+s)/128.0,v/128.0],[u/128.0,v/128.0]]);g.end();
                    g.translate(self.char_width[c] as f32,0.0,0.0);
                }
            }
            i+=1;
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
        self.render_string(gl, s, x + 1, y + 1, color, true);
        self.draw_string(gl, s, x, y, color);
    }

    pub fn draw_string(&self, gl: &mut Gl, s: &str, x: i32, y: i32, color: i32) {
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
        let chars: Vec<char> = s.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            while chars.len() > i + 1 && chars[i] == '§' {
                let code = chars[i + 1].to_lowercase().next().unwrap_or(chars[i + 1]);
                let mut k = "0123456789abcdef".find(code).map_or(-1, |p| p as i32);
                if !(0..=15).contains(&k) {
                    k = 15;
                }
                let c = self.colors[k as usize + if shadow { 16 } else { 0 }];
                gl.color3f(c[0], c[1], c[2]);
                i += 2;
            }
            if i < chars.len() {
                if let Some(k) = self.index(chars[i]) {
                    if k + 32 < 256 {
                        self.glyph(gl, k + 32);
                    }
                }
            }
            i += 1;
        }
        gl.pop_matrix();
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

    /// `func_27278_a`: draw word-wrapped to `width`.
    pub fn draw_split_string(&self, gl: &mut Gl, s: &str, x: i32, mut y: i32, width: i32, color: i32) {
        let lines: Vec<&str> = java_split(s, '\n');
        if lines.len() > 1 {
            for l in lines {
                self.draw_split_string(gl, l, x, y, width, color);
                y += self.split_string_height(l, width);
            }
            return;
        }
        for line in self.wrap(s, width) {
            self.draw_string(gl, &line, x, y, color);
            y += 8;
        }
    }

    /// `func_27277_a`: height of the wrapped text.
    pub fn split_string_height(&self, s: &str, width: i32) -> i32 {
        let lines: Vec<&str> = java_split(s, '\n');
        if lines.len() > 1 {
            return lines.iter().map(|l| self.split_string_height(l, width)).sum();
        }
        let mut h = self.wrap(s, width).len() as i32 * 8;
        if h < 8 {
            h += 8;
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
                if !head.trim().is_empty() {
                    out.push(head);
                }
                line = cs[n..].iter().collect();
            }
            if !line.trim().is_empty() {
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
