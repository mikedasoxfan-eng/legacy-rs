//! GuiMainMenu's 64 cube samples and eight framebuffer-copy blur passes.
use super::{gl::{Factor, Gl, TexId}, render::GuiRenderer, Ui};

pub struct Panorama {
    faces: [TexId; 6],
    snapshot: TexId,
    time: f32,
    projection: glam::Mat4,
    modelview: glam::Mat4,
    width: i32,
    height: i32,
    color_mask: u8,
    depth_test: bool,
}

pub fn draw(ui: &mut Ui, width: i32, height: i32, time: f32) {
    let snapshot = ui.texture("panorama_snapshot");
    ui.gl.panorama = Some(Panorama {
        faces: std::array::from_fn(|i| ui.texture(&format!("title/bg/panorama{i}.png"))),
        snapshot, time,
        projection: ui.gl.proj, modelview: ui.gl.modelview_matrix(), width, height,
        color_mask: ui.gl.color_mask, depth_test: ui.gl.depth_test,
    });
    let scale = 120.0 / width.max(height) as f32;
    let (u, v) = (height as f32 * scale / 256.0, width as f32 * scale / 256.0);
    let g = &mut ui.gl;
    // func_35355_b and every blur restore these states before the final crop.
    g.depth_mask = true;
    g.depth_test = true;
    g.alpha_test = true;
    g.color_mask = 15;
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
    /// Exercise the actual viewport/copy passes with the checker's textures.
    /// Pixel equivalence for arbitrary small-window GL reads is not asserted.
    pub(super) fn check_panorama_resize(&mut self, device: &wgpu::Device, queue: &wgpu::Queue,
        target: &wgpu::TextureView, size: (u32,u32), face: TexId, snapshot: TexId) {
        let mut g = Gl::new();
        g.ortho(0.0,size.0 as f64,size.1 as f64,0.0,1000.0,3000.0);
        g.translatef(0.0,0.0,-2000.0);
        self.render_panorama(device,queue,target,size,&Panorama {
            faces: [face;6], snapshot, time: 0.0,
            projection: g.proj, modelview: g.modelview_matrix(),
            width: size.0 as i32, height: size.1 as i32,
            color_mask: 15, depth_test: true,
        });
    }

    pub(super) fn render_panorama(&mut self, device: &wgpu::Device, queue: &wgpu::Queue,
        target: &wgpu::TextureView, size: (u32, u32), p: &Panorama) {
        let mut g = Gl::new();
        g.color_mask = p.color_mask;
        g.depth_test = p.depth_test;
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
        let (viewport_width, viewport_height) = (size.0.min(256), size.1.min(256));
        // OpenGL permits the viewport to extend outside a small window. WebGPU
        // requires an in-bounds viewport: crop its clip coordinates instead,
        // preserving the original 256-pixel mapping on the visible portion.
        let crop = if viewport_width < 256 || viewport_height < 256 {
            let (x, y) = (256.0/viewport_width as f32, 256.0/viewport_height as f32);
            Some(glam::Mat4::from_translation(glam::Vec3::new(x-1.0,y-1.0,0.0))
                * glam::Mat4::from_scale(glam::Vec3::new(x,y,1.0)))
        } else { None };
        if let Some(crop) = crop {
            for draw in &mut g.draws { draw.proj = crop * draw.proj; }
        }
        let viewport = Some((0.0, if self.gl_orientation { 0.0 } else { size.1.saturating_sub(256) as f32 },
            viewport_width as f32,viewport_height as f32));
        self.render_recorded(device,queue,target,size,&g,None,false,viewport);
        for _ in 0..8 {
            self.copy_framebuffer(device,queue,target,size,p.snapshot);
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
            if let Some(crop) = crop {
                for draw in &mut blur.draws { draw.proj = crop * draw.proj; }
            }
            self.render_recorded(device,queue,target,size,&blur,None,false,viewport);
        }
    }
}
