//! `LoadingScreenRenderer`: "Generating level" / "Building terrain" with the progress bar.

use super::Ui;

#[derive(Clone, Debug, Default)]
pub struct LoadingScreen {
    /// `printText` (title)
    pub title: String,
    /// `displayLoadingString` (subtitle)
    pub text: String,
    /// `setLoadingProgress`; -1 hides the bar.
    pub progress: i32,
}

impl LoadingScreen {
    /// One `setLoadingProgress` frame (the screen is cleared and fully redrawn).
    pub fn draw(&self, ui: &mut Ui) {
        let sr = ui.scaled();
        let (w, h) = (sr.width, sr.height);
        ui.gl.ortho(0.0, sr.width_d, sr.height_d, 0.0, 100.0, 300.0);
        ui.gl.load_identity();
        ui.gl.translatef(0.0, 0.0, -200.0);
        let bg = ui.texture("gui/background.png");
        let s = 32.0f32;
        let gl = &mut ui.gl;
        gl.bind_texture(bg);
        gl.start_drawing_quads();
        gl.set_color_opaque_i(4210752);
        gl.add_vertex_with_uv(0.0, h as f64, 0.0, 0.0, (h as f32 / s) as f64);
        gl.add_vertex_with_uv(w as f64, h as f64, 0.0, (w as f32 / s) as f64, (h as f32 / s) as f64);
        gl.add_vertex_with_uv(w as f64, 0.0, 0.0, (w as f32 / s) as f64, 0.0);
        gl.add_vertex_with_uv(0.0, 0.0, 0.0, 0.0, 0.0);
        gl.draw();
        if self.progress >= 0 {
            let (bw, bh) = (100, 2);
            let x = w / 2 - bw / 2;
            let y = h / 2 + 16;
            gl.texture_2d = false;
            gl.start_drawing_quads();
            gl.set_color_opaque_i(8421504);
            gl.add_vertex(x as f64, y as f64, 0.0);
            gl.add_vertex(x as f64, (y + bh) as f64, 0.0);
            gl.add_vertex((x + bw) as f64, (y + bh) as f64, 0.0);
            gl.add_vertex((x + bw) as f64, y as f64, 0.0);
            gl.set_color_opaque_i(8454016);
            gl.add_vertex(x as f64, y as f64, 0.0);
            gl.add_vertex(x as f64, (y + bh) as f64, 0.0);
            gl.add_vertex((x + self.progress) as f64, (y + bh) as f64, 0.0);
            gl.add_vertex((x + self.progress) as f64, y as f64, 0.0);
            gl.draw();
            gl.texture_2d = true;
        }
        let tw = ui.string_width(&self.title);
        ui.font.draw_string_with_shadow(&mut ui.gl, &self.title, (w - tw) / 2, h / 2 - 4 - 16, 16777215);
        let sw = ui.string_width(&self.text);
        ui.font.draw_string_with_shadow(&mut ui.gl, &self.text, (w - sw) / 2, h / 2 - 4 + 8, 16777215);
    }
}
