//! `Minecraft.displayDebugInfo`: the frame-time graph at the bottom left while F3 is on.
//! Each of the last 512 frames is a column: its height is the frame time (green, red above
//! 1/60 s) with the time spent ticking on top in grey, over two bands marking 1/60 s and 1/30 s.

use super::Ui;
use std::time::Instant;

const TARGET: i64 = 16_666_666;

pub struct FrameGraph {
    frame_times: [i64; 512],
    tick_times: [i64; 512],
    recorded: usize,
    prev: Option<Instant>,
}

impl Default for FrameGraph {
    fn default() -> Self {
        FrameGraph { frame_times: [0; 512], tick_times: [0; 512], recorded: 0, prev: None }
    }
}

impl FrameGraph {
    /// F3 off: `prevFrameTime = System.nanoTime()`.
    pub fn skip(&mut self) {
        self.prev = Some(Instant::now());
    }

    pub fn draw(&mut self, ui: &mut Ui, tick_nanos: i64) {
        let now = Instant::now();
        let prev = *self.prev.get_or_insert(now);
        let len = self.frame_times.len();
        self.tick_times[self.recorded & (len - 1)] = tick_nanos;
        self.frame_times[self.recorded & (len - 1)] = (now - prev).as_nanos() as i64;
        self.recorded += 1;
        self.prev = Some(now);

        let (w, h) = (ui.display_width as f64, ui.display_height as f64);
        let gl = &mut ui.gl;
        gl.ortho(0.0, w, h, 0.0, 1000.0, 3000.0);
        gl.load_identity();
        gl.translatef(0.0, 0.0, -2000.0);
        let texture = gl.texture_2d;
        gl.texture_2d = false;
        let band = (TARGET / 200_000) as f64;
        let n = len as f64;
        gl.start_drawing_quads();
        gl.set_color_opaque_i(536870912);
        quad(gl, 0.0, h - band, n, h);
        gl.set_color_opaque_i(538968064);
        quad(gl, 0.0, h - band * 2.0, n, h - band);
        gl.draw();
        let average = (self.frame_times.iter().sum::<i64>() / 200_000 / len as i64) as f64;
        gl.start_drawing_quads();
        gl.set_color_opaque_i(541065216);
        quad(gl, 0.0, h - average, n, h);
        gl.draw();
        // GL_LINES one pixel wide: each column as a one-pixel-wide quad.
        gl.start_drawing_quads();
        for i in 0..len {
            let fade = ((i as i64 - self.recorded as i64) & (len as i64 - 1)) * 255 / len as i64;
            let mut c = fade * fade / 255;
            c = c * c / 255;
            let color = if self.frame_times[i] > TARGET { -16777216 + c * 65536 } else { -16777216 + c * 256 };
            let frame = (self.frame_times[i] / 200_000) as f64;
            let tick = (self.tick_times[i] / 200_000) as f64;
            let x = i as f64;
            gl.set_color_opaque_i(color as i32);
            quad(gl, x, h - frame, x + 1.0, h);
            gl.set_color_opaque_i((-16777216 + c * 65536 + c * 256 + c) as i32);
            quad(gl, x, h - frame, x + 1.0, h - (frame - tick));
        }
        gl.draw();
        gl.texture_2d = texture;
    }
}

fn quad(gl: &mut super::gl::Gl, x0: f64, y0: f64, x1: f64, y1: f64) {
    gl.add_vertex(x0, y1, 0.0);
    gl.add_vertex(x1, y1, 0.0);
    gl.add_vertex(x1, y0, 0.0);
    gl.add_vertex(x0, y0, 0.0);
}
