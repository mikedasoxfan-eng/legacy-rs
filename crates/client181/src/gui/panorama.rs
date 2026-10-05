//! GuiMainMenu's 64 cube samples and eight framebuffer-copy blur passes.
use super::{gl::{Factor, Gl, TexId}, render::GuiRenderer, Ui};

pub struct Panorama {
    faces: [TexId; 6],
    frame: TexId,
    snapshot: TexId,
    time: f32,
    projection: glam::Mat4,
    modelview: glam::Mat4,
    width: i32,
    height: i32,
}

pub fn draw(ui: &mut Ui, width: i32, height: i32, time: f32) {
    let snapshot = ui.texture("panorama_snapshot");
    ui.gl.panorama = Some(Panorama {
        faces: std::array::from_fn(|i| ui.texture(&format!("title/bg/panorama{i}.png"))),
        frame: ui.texture("panorama_frame"), snapshot, time,
        projection: ui.gl.proj, modelview: ui.gl.modelview_matrix(), width, height,
    });
    let scale = 120.0 / width.max(height) as f32;
    let (u, v) = (height as f32 * scale / 256.0, width as f32 * scale / 256.0);
    let g = &mut ui.gl;
    g.bind_texture(snapshot);
    g.blend = true;
    g.blend_func = (Factor::SrcAlpha, Factor::OneMinusSrcAlpha);
    g.color4f(1.0, 1.0, 1.0, 1.0);
    g.start_drawing_quads();
    g.set_color_rgba_f(1.0, 1.0, 1.0, 1.0);
    for (x,y,s,t) in [(0,height,0.5-u,0.5+v),(width,height,0.5-u,0.5-v),(width,0,0.5+u,0.5-v),(0,0,0.5+u,0.5+v)] {
        g.add_vertex_with_uv(x as f64,y as f64,0.0,s as f64,t as f64);
    }
    g.draw();
}

impl GuiRenderer {
    pub(super) fn render_panorama(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, p: &Panorama) {
        let mut g = Gl::new();
        // LWJGL GLU.gluPerspective computes cotangent from a float radian angle.
        let radians = 120.0f32 / 2.0 * std::f32::consts::PI / 180.0;
        let cot = radians.cos() / radians.sin();
        g.proj = glam::Mat4::from_cols_array(&[
            cot,0.0,0.0,0.0, 0.0,cot,0.0,0.0,
            0.0,0.0,-(10.0+0.05)/(10.0-0.05),-1.0,
            0.0,0.0,-2.0*0.05*10.0/(10.0-0.05),0.0,
        ]);
        g.rotatef(180.0, 1.0, 0.0, 0.0);
        g.blend = true;
        g.blend_func = (Factor::SrcAlpha, Factor::OneMinusSrcAlpha);
        g.alpha_test = false;
        g.depth_mask = false;
        for sample in 0..64 {
            g.push_matrix();
            g.translatef(((sample%8) as f32 / 8.0 - 0.5)/64.0,((sample/8) as f32 / 8.0 - 0.5)/64.0,0.0);
            g.rotatef(worldgen::math::sin(p.time/400.0)*25.0+20.0,1.0,0.0,0.0);
            g.rotatef(-p.time*0.1,0.0,1.0,0.0);
            for face in 0..6 {
                g.push_matrix();
                match face {
                    1 => g.rotatef(90.0,0.0,1.0,0.0),
                    2 => g.rotatef(180.0,0.0,1.0,0.0),
                    3 => g.rotatef(-90.0,0.0,1.0,0.0),
                    4 => g.rotatef(90.0,1.0,0.0,0.0),
                    5 => g.rotatef(-90.0,1.0,0.0,0.0),
                    _ => {},
                }
                g.bind_texture(p.faces[face]);
                let matrix = g.modelview_matrix();
                let first = g.verts.len();
                g.start_drawing_quads();
                g.set_color_rgba_i(0xffffff,255/(sample+1));
                for (x,y,u,v) in [(-1.0,-1.0,0.0,0.0),(1.0,-1.0,1.0,0.0),(1.0,1.0,1.0,1.0),(-1.0,1.0,0.0,1.0)] {
                    g.add_vertex_with_uv(x,y,1.0,u,v);
                }
                g.draw();
                // GL keeps the cube in object space through the MVP multiplication.
                let raw = [[-1.0,-1.0,1.0],[1.0,-1.0,1.0],[1.0,1.0,1.0],[-1.0,1.0,1.0]];
                for (vertex, index) in g.verts[first..].iter_mut().zip([0,1,2,0,2,3]) { vertex.pos = raw[index]; }
                g.draws.last_mut().unwrap().modelview = matrix;
                g.pop_matrix();
            }
            g.pop_matrix();
            g.color_mask = 7;
        }
        let view = self.texture_view(p.frame);
        // The copy operation uses OpenGL's bottom-up framebuffer coordinates.
        let orientation = self.gl_orientation;
        self.gl_orientation = true;
        self.render(device,queue,&view,(256,256),&g,Some([0.0;4]));
        for _ in 0..8 {
            self.copy_texture(device,queue,p.frame,p.snapshot);
            let mut blur = Gl::new();
            blur.proj = p.projection;
            blur.set_modelview(p.modelview);
            blur.blend = true;
            blur.blend_func = (Factor::SrcAlpha,Factor::OneMinusSrcAlpha);
            blur.color_mask = 7;
            blur.bind_texture(p.snapshot);
            blur.start_drawing_quads();
            for sample in 0..3 {
                blur.set_color_rgba_f(1.0,1.0,1.0,1.0/(sample+1) as f32);
                let shift = (sample-1) as f32/256.0;
                for (x,y,u,v) in [(p.width,p.height,shift,0.0),(p.width,0,1.0+shift,0.0),(0,0,1.0+shift,1.0),(0,p.height,shift,1.0)] {
                    blur.add_vertex_with_uv(x as f64,y as f64,0.0,u as f64,v as f64);
                }
            }
            blur.draw();
            self.render(device,queue,&view,(256,256),&blur,None);
        }
        self.gl_orientation = orientation;
    }
}
