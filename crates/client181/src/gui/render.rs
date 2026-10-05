//! Executes `gl::Draw` lists with wgpu: fixed-function texturing (GL_MODULATE), the alpha
//! test, blending, depth testing (GL_LEQUAL) and nearest/repeat sampling like `RenderEngine`.

use super::gl::{DrawState, Factor, Gl, GuiVertex, TexId};
use crate::render::gpu::Image;
use std::collections::HashMap;

const SHADER: &str = r#"
struct U { proj: mat4x4<f32> };
@group(0) @binding(0) var<uniform> u: U;
@group(1) @binding(0) var tex: texture_2d<f32>;
@group(1) @binding(1) var samp: sampler;

struct VO {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) flags: vec2<f32>,
    @location(3) lightmap: vec3<f32>,
};

@vertex
fn vs(@location(0) pos: vec3<f32>, @location(1) uv: vec2<f32>, @location(2) color: vec4<f32>, @location(3) flags: vec2<f32>, @location(4) lightmap: vec3<f32>) -> VO {
    var o: VO;
    var c = u.proj * vec4<f32>(pos, 1.0);
    // OpenGL depth range [-1, 1] -> [0, 1].
    c.z = c.z * 0.5 + c.w * 0.5;
    o.pos = c;
    o.uv = uv;
    o.color = color;
    o.flags = flags;
    o.lightmap = lightmap;
    return o;
}

@fragment
fn fs(i: VO) -> @location(0) vec4<f32> {
    var c = clamp(i.color, vec4<f32>(0.0), vec4<f32>(1.0));
    let t = textureSampleLevel(tex, samp, i.uv, 0.0);
    if (i.flags.x > 0.5) {
        c = c * t;
    }
    c = vec4<f32>(c.rgb * i.lightmap, c.a);
    if (i.flags.y >= 0.0 && !(c.a > i.flags.y)) {
        discard;
    }
    return c;
}
"#;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct PipeKey {
    color_mask: u8,
    blend: Option<(Factor, Factor)>,
    depth_test: bool,
    depth_gequal: bool,
    depth_equal: bool,
    depth_write: bool,
}

struct GuiTexture {
    texture: wgpu::Texture,
    bind: wgpu::BindGroup,
}

pub struct GuiRenderer {
    pub(super) format: wgpu::TextureFormat,
    pub engine: super::render_engine::Engine,
    /// Multisample count (the driver may force MSAA on the original's GL window).
    samples: u32,
    /// Rasterize bottom-up like an OpenGL framebuffer (the image comes out upside down).
    /// Interpolation rounds the same way as the original's then, down to the last bit.
    pub gl_orientation: bool,
    shader: wgpu::ShaderModule,
    layout: wgpu::PipelineLayout,
    /// Pipelines per state and colour-target format (the window, or an offscreen texture).
    pipes: HashMap<(PipeKey, wgpu::TextureFormat), wgpu::RenderPipeline>,
    tex_layout: wgpu::BindGroupLayout,
    uniforms: wgpu::Buffer,
    uniform_bind: wgpu::BindGroup,
    uniform_stride: u64,
    uniform_slots: u64,
    sampler: wgpu::Sampler,
    /// `%blur%` (GL_LINEAR) and `%clamp%` variants: [nearest/linear][repeat/clamp].
    samplers: [[wgpu::Sampler; 2]; 2],
    textures: Vec<GuiTexture>,
    white: GuiTexture,
    panorama_faces: Option<([TexId;6],std::sync::Arc<[Image;6]>)>,
    vbuf: Option<(wgpu::Buffer, u64)>,
    pub(super) depth: Option<(wgpu::Texture, wgpu::TextureView, u32, u32)>,
    /// Multisampled color target, resolved into the real target after drawing.
    msaa: Option<(wgpu::Texture, wgpu::TextureView, u32, u32)>,
}

fn factor(f: Factor) -> wgpu::BlendFactor {
    match f {
        Factor::Zero => wgpu::BlendFactor::Zero,
        Factor::One => wgpu::BlendFactor::One,
        Factor::SrcAlpha => wgpu::BlendFactor::SrcAlpha,
        Factor::DstAlpha => wgpu::BlendFactor::DstAlpha,
        Factor::OneMinusSrcAlpha => wgpu::BlendFactor::OneMinusSrcAlpha,
        Factor::DstColor => wgpu::BlendFactor::Dst,
        Factor::OneMinusDstColor => wgpu::BlendFactor::OneMinusDst,
        Factor::SrcColor => wgpu::BlendFactor::Src,
        Factor::OneMinusSrcColor => wgpu::BlendFactor::OneMinusSrc,
    }
}

impl GuiRenderer {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        samples: u32,
    ) -> GuiRenderer {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("gui"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        let ubl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("gui uniforms"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: wgpu::BufferSize::new(64),
                },
                count: None,
            }],
        });
        let tex_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("gui texture"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("gui"),
            bind_group_layouts: &[Some(&ubl), Some(&tex_layout)],
            immediate_size: 0,
        });
        let align = device.limits().min_uniform_buffer_offset_alignment as u64;
        let uniform_stride = 64u64.div_ceil(align) * align;
        let uniform_slots = 512;
        let uniforms = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("gui uniforms"),
            size: uniform_stride * uniform_slots,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let uniform_bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("gui uniforms"),
            layout: &ubl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &uniforms,
                    offset: 0,
                    size: wgpu::BufferSize::new(64),
                }),
            }],
        });
        // RenderEngine: GL_NEAREST, GL_REPEAT.
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("gui nearest repeat"),
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let mk = |linear: bool, clamp: bool| {
            let f = if linear {
                wgpu::FilterMode::Linear
            } else {
                wgpu::FilterMode::Nearest
            };
            let a = if clamp {
                wgpu::AddressMode::ClampToEdge
            } else {
                wgpu::AddressMode::Repeat
            };
            device.create_sampler(&wgpu::SamplerDescriptor {
                label: Some("gui sampler"),
                address_mode_u: a,
                address_mode_v: a,
                mag_filter: f,
                min_filter: f,
                ..Default::default()
            })
        };
        let samplers = [
            [mk(false, false), mk(false, true)],
            [mk(true, false), mk(true, true)],
        ];
        let white = make_texture(
            device,
            queue,
            &tex_layout,
            &sampler,
            &Image {
                width: 1,
                height: 1,
                rgba: vec![255; 4],
            },
        );
        GuiRenderer {
            engine: Default::default(),
            format,
            samples,
            gl_orientation: false,
            shader,
            layout,
            pipes: HashMap::new(),
            tex_layout,
            uniforms,
            uniform_bind,
            uniform_stride,
            uniform_slots,
            sampler,
            samplers,
            textures: Vec::new(),
            white,
            panorama_faces: None,
            vbuf: None,
            depth: None,
            msaa: None,
        }
    }

    /// Register a texture (like `RenderEngine.getTexture`); returns its id.
    pub fn add_texture(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &Image,
    ) -> TexId {
        self.add_texture_with(device, queue, img, false, false)
    }

    /// With `RenderEngine`'s name prefixes: `%blur%` = linear filtering, `%clamp%` = clamped.
    pub fn add_texture_with(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &Image,
        blur: bool,
        clamp: bool,
    ) -> TexId {
        let previous = (self.engine.blur, self.engine.clamp);
        self.engine.blur = blur;
        self.engine.clamp = clamp;
        let image = std::sync::Arc::new(std::sync::Mutex::new(Image { width: img.width, height: img.height, rgba: img.rgba.clone() }));
        let result = self.with_engine(device, queue, |engine, backend| engine.allocate(backend, image));
        (self.engine.blur, self.engine.clamp) = previous;
        result.unwrap_or_else(|e| panic!("{e}")) as TexId
    }

    fn with_engine<T>(&mut self, device: &wgpu::Device, queue: &wgpu::Queue,
        call: impl FnOnce(&mut super::render_engine::Engine, &mut UploadAdapter<'_>) -> T) -> T {
        let mut engine = std::mem::take(&mut self.engine);
        let mut backend = UploadAdapter { renderer: self, device, queue };
        let result = call(&mut engine, &mut backend);
        self.engine = engine;
        result
    }

    pub fn named_texture(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, name: &str,
        missing: &Image, read: impl FnMut(&str, bool, bool) -> Result<Option<Image>, super::render_engine::Error>)
        -> Result<TexId, super::render_engine::Error> {
        self.with_engine(device, queue, |engine, backend| engine.get_texture(backend, name, missing, read)).map(|id| id as TexId)
    }

    pub fn refresh_engine(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, missing: &Image,
        read: impl FnMut(&str, bool, bool) -> Result<Option<Image>, super::render_engine::Error>) -> Result<(), super::render_engine::Error> {
        self.with_engine(device, queue, |engine, backend| engine.refresh(backend, missing, read))
    }

    pub fn delete_texture(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, id: TexId) {
        self.with_engine(device, queue, |engine, backend| engine.delete(backend, id as i32));
    }

    /// glTexImage2D may change the dimensions while retaining the texture name.
    pub fn replace_texture(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, id: TexId, img: &Image, blur: bool, clamp: bool) {
        if self.panorama_faces.as_ref().is_some_and(|(ids,_)|ids.contains(&id)){self.panorama_faces=None;}
        self.textures[id as usize] = make_texture(device, queue, &self.tex_layout, &self.samplers[blur as usize][clamp as usize], img);
    }

    /// Replace a texture's pixels (same size as when it was added).
    pub fn update_texture(&self, queue: &wgpu::Queue, id: TexId, img: &Image) {
        write(queue, &self.textures[id as usize].texture, img);
    }

    pub fn update_atlas_tile(&self, queue: &wgpu::Queue, id: TexId, icon: u32, image: &[u8; 1024]) {
        write_tile(queue, &self.textures[id as usize].texture, icon, image);
    }

    pub(super) fn texture_view(&self, id: TexId) -> wgpu::TextureView {
        self.textures[id as usize].texture.create_view(&Default::default())
    }

    /// glCopyTexSubImage2D reads the framebuffer, including its retained alpha.
    /// The intermediate keeps the source format (window surfaces can be BGRA).
    pub(super) fn copy_framebuffer(&mut self, device: &wgpu::Device, queue: &wgpu::Queue,
        source: &wgpu::TextureView, size: (u32, u32), destination: TexId) {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("GUI framebuffer copy"),
            size: wgpu::Extent3d { width: 256, height: 256, depth_or_array_layers: 1 },
            mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2,
            format: self.format,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let mut enc = device.create_command_encoder(&Default::default());
        let (width, height) = (size.0.min(256), size.1.min(256));
        let origin = wgpu::Origin3d { x: 0, y: if self.gl_orientation { 0 } else { size.1.saturating_sub(256) }, z: 0 };
        // GL leaves pixels outside a small window undefined. Copy every defined
        // pixel and leave the inaccessible texels at WebGPU's initialized zero.
        let destination_origin = wgpu::Origin3d { x: 0, y: if self.gl_orientation { 0 } else { 256-height }, z: 0 };
        enc.copy_texture_to_texture(wgpu::TexelCopyTextureInfo { origin, ..source.texture().as_image_copy() },
            wgpu::TexelCopyTextureInfo { origin: destination_origin, ..texture.as_image_copy() },
            wgpu::Extent3d { width, height, depth_or_array_layers: 1 });
        queue.submit([enc.finish()]);
        let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("GUI framebuffer copy"), layout: &self.tex_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&texture.create_view(&Default::default())) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&self.sampler) },
            ],
        });
        let id = self.textures.len() as TexId;
        self.textures.push(GuiTexture { texture, bind });
        let mut copy = Gl::new();
        copy.depth_test = false;
        copy.alpha_test = false;
        copy.bind_texture(id);
        copy.start_drawing_quads();
        for (x,y,u,v) in [(-1.0,-1.0,0.0,0.0),(1.0,-1.0,1.0,0.0),(1.0,1.0,1.0,1.0),(-1.0,1.0,0.0,1.0)] {
            copy.add_vertex_with_uv(x,y,0.0,u,if self.gl_orientation { v } else { 1.0-v });
        }
        copy.draw();
        let depth = self.depth.take();
        let msaa = self.msaa.take();
        let format = std::mem::replace(&mut self.format, wgpu::TextureFormat::Rgba8Unorm);
        let orientation = std::mem::replace(&mut self.gl_orientation, true);
        // GuiMainMenu's blur callback sets GL_LINEAR on the snapshot every
        // pass, including after RenderEngine refresh restored GL_NEAREST.
        let destination_view = self.textures[destination as usize].texture.create_view(&Default::default());
        self.textures[destination as usize].bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("panorama snapshot linear"), layout: &self.tex_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&destination_view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&self.samplers[1][0]) },
            ],
        });
        let view = self.texture_view(destination);
        self.render_recorded(device, queue, &view, (256,256), &copy, None, true, None);
        self.gl_orientation = orientation;
        self.format = format;
        self.depth = depth;
        self.msaa = msaa;
        self.textures.pop();
    }

    pub(super) fn blit_panorama_pixels(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,target:&wgpu::TextureView,size:(u32,u32),pixels:Vec<u8>,viewport:Option<(f32,f32,f32,f32)>) {
        // A frame-local native transfer is not a RenderEngine dynamic image.
        let id = self.textures.len() as TexId;
        self.textures.push(make_texture(device, queue, &self.tex_layout, &self.sampler,
            &Image { width: 256, height: 256, rgba: pixels }));
        let mut gl=Gl::new();gl.depth_test=false;gl.depth_mask=false;gl.alpha_test=false;gl.bind_texture(id);
        gl.start_drawing_quads();
        for (x,y,u,v) in [(-1.0,-1.0,0.0,0.0),(1.0,-1.0,1.0,0.0),(1.0,1.0,1.0,1.0),(-1.0,1.0,0.0,1.0)] {
            gl.add_vertex_with_uv(x,y,0.0,u,if self.gl_orientation {v}else{1.0-v});
        }
        gl.draw();self.render_recorded(device,queue,target,size,&gl,None,false,viewport);self.textures.pop();
    }
    pub(super) fn panorama_face_pixels(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,ids:[TexId;6])->std::sync::Arc<[Image;6]> {
        if self.panorama_faces.as_ref().is_none_or(|(old,_)|*old!=ids){
            let images=std::array::from_fn(|i|self.texture_pixels(device,queue,ids[i]));
            self.panorama_faces=Some((ids,std::sync::Arc::new(images)));
        }
        self.panorama_faces.as_ref().unwrap().1.clone()
    }
    pub(super) fn samples_for_panorama(&self)->u32{self.samples}
    pub(super) fn texture_pixels(&self, device:&wgpu::Device,queue:&wgpu::Queue,id:TexId)->Image {
        let mut image=super::shot::read_back(device,queue,&self.textures[id as usize].texture);
        // read_back presents a GL framebuffer top down; asset texels have top-down storage.
        let row=image.width as usize*4;let mut pixels=Vec::with_capacity(image.rgba.len());
        for y in (0..image.height as usize).rev(){pixels.extend_from_slice(&image.rgba[y*row..(y+1)*row]);}
        image.rgba=pixels;image
    }
    fn pipeline(&mut self, device: &wgpu::Device, key: PipeKey) -> &wgpu::RenderPipeline {
        let (format, shader, layout, samples) =
            (self.format, &self.shader, &self.layout, self.samples);
        self.pipes.entry((key, format)).or_insert_with(|| {
            let blend = key.blend.map(|(s, d)| {
                let c = wgpu::BlendComponent { src_factor: factor(s), dst_factor: factor(d), operation: wgpu::BlendOperation::Add };
                wgpu::BlendState { color: c, alpha: c }
            });
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("gui"),
                layout: Some(layout),
                vertex: wgpu::VertexState {
                    module: shader,
                    entry_point: Some("vs"),
                    compilation_options: Default::default(),
                    buffers: &[Some(wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<GuiVertex>() as u64,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2, 2 => Float32x4, 3 => Float32x2, 4 => Float32x3],
                    })],
                },
                fragment: Some(wgpu::FragmentState {
                    module: shader,
                    entry_point: Some("fs"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState { format, blend, write_mask: wgpu::ColorWrites::from_bits_truncate(key.color_mask as u32) })],
                }),
                primitive: wgpu::PrimitiveState { cull_mode: None, ..Default::default() },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth24PlusStencil8,
                    depth_write_enabled: Some(key.depth_write && key.depth_test),
                    depth_compare: Some(match (key.depth_test, key.depth_equal,key.depth_gequal) {
                        (false, _,_) => wgpu::CompareFunction::Always,
                        (true, true,_)=>wgpu::CompareFunction::Equal,
                        (true, false,false) => wgpu::CompareFunction::LessEqual,
                        (true, false,true) => wgpu::CompareFunction::GreaterEqual,
                    }),
                    stencil: Default::default(),
                    bias: Default::default(),
                }),
                multisample: wgpu::MultisampleState { count: samples, ..Default::default() },
                multiview_mask: None,
                cache: None,
            })
        })
    }

    /// Draw everything recorded in `gl` onto `target`. `clear` = `glClear(COLOR)` color first.
    pub fn render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        target: &wgpu::TextureView,
        size: (u32, u32),
        gl: &Gl,
        clear: Option<[f64; 4]>,
    ) {
        if gl.panorama.is_none() && gl.clear_color_mask == 15 {
            self.render_recorded(device, queue, target, size, gl, clear, true, None);
            return;
        }
        if let Some(c) = clear {
            // glClear obeys GL_COLOR_WRITEMASK. A render-pass clear cannot do that.
            let mut reset = Gl::new();
            reset.color_mask = gl.clear_color_mask;
            reset.depth_test = false;
            reset.texture_2d = false;
            reset.alpha_test = false;
            reset.color4f(c[0] as f32, c[1] as f32, c[2] as f32, c[3] as f32);
            reset.start_drawing_quads();
            for (x, y) in [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)] {
                reset.add_vertex(x, y, 0.0);
            }
            reset.draw();
            self.render_recorded(device, queue, target, size, &reset, None, true, None);
        }
        if let Some(panorama) = &gl.panorama {
            self.render_panorama(device, queue, target, size, panorama);
        }
        self.render_recorded(device, queue, target, size, gl, None, gl.panorama.is_none(), None);
    }

    pub(super) fn render_recorded(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        target: &wgpu::TextureView,
        size: (u32, u32),
        gl: &Gl,
        clear: Option<[f64; 4]>,
        clear_depth: bool,
        viewport: Option<(f32, f32, f32, f32)>,
    ) {
        if self.depth.as_ref().is_none_or(|d| (d.2, d.3) != size) {
            let t = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("gui depth"),
                size: wgpu::Extent3d {
                    width: size.0,
                    height: size.1,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: self.samples,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth24PlusStencil8,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            });
            let v = t.create_view(&Default::default());
            self.depth = Some((t, v, size.0, size.1));
            self.msaa = (self.samples > 1).then(|| {
                let t = device.create_texture(&wgpu::TextureDescriptor {
                    label: Some("gui msaa"),
                    size: wgpu::Extent3d {
                        width: size.0,
                        height: size.1,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: self.samples,
                    dimension: wgpu::TextureDimension::D2,
                    format: self.format,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                });
                let v = t.create_view(&Default::default());
                (t, v, size.0, size.1)
            });
        }
        let bytes: &[u8] = bytemuck::cast_slice(&gl.verts);
        if !bytes.is_empty()
            && self
                .vbuf
                .as_ref()
                .is_none_or(|(_, cap)| *cap < bytes.len() as u64)
        {
            let cap = (bytes.len() as u64).next_power_of_two().max(1 << 16);
            let b = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("gui vertices"),
                size: cap,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            self.vbuf = Some((b, cap));
        }
        if !bytes.is_empty() {
            queue.write_buffer(&self.vbuf.as_ref().unwrap().0, 0, bytes);
        }
        // Keep display-list models in object space through the combined MVP,
        // as GL does. This preserves 24-bit depth ties between armor faces.
        let mut projs: Vec<(glam::Mat4, glam::Mat4)> = Vec::new();
        let mut slot_of = Vec::with_capacity(gl.draws.len());
        for d in &gl.draws {
            let i = match projs.iter().position(|p| *p == (d.proj, d.modelview)) {
                Some(i) => i,
                None => {
                    projs.push((d.proj, d.modelview));
                    projs.len() - 1
                }
            };
            slot_of.push(i.min(self.uniform_slots as usize - 1));
        }
        for (i, (p, m)) in projs.iter().enumerate().take(self.uniform_slots as usize) {
            let p = if self.gl_orientation {
                glam::Mat4::from_scale(glam::Vec3::new(1.0, -1.0, 1.0)) * *p
            } else {
                *p
            };
            queue.write_buffer(
                &self.uniforms,
                i as u64 * self.uniform_stride,
                bytemuck::cast_slice(&(p * *m).to_cols_array()),
            );
        }
        let keys: Vec<PipeKey> = gl.draws.iter().map(|d| key_of(&d.state)).collect();
        for k in &keys {
            self.pipeline(device, *k);
        }
        let mut enc = device.create_command_encoder(&Default::default());
        {
            let depth_view = &self.depth.as_ref().unwrap().1;
            let mut rp = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("gui"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: self.msaa.as_ref().map_or(target, |m| &m.1),
                    depth_slice: None,
                    resolve_target: self.msaa.as_ref().map(|_| target),
                    ops: wgpu::Operations {
                        load: match clear {
                            Some(c) => wgpu::LoadOp::Clear(wgpu::Color {
                                r: c[0],
                                g: c[1],
                                b: c[2],
                                a: c[3],
                            }),
                            None => wgpu::LoadOp::Load,
                        },
                        store: wgpu::StoreOp::Store,
                    },
                })],
                // EntityRenderer clears the depth buffer before drawing a screen.
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: if clear_depth { wgpu::LoadOp::Clear(1.0) } else { wgpu::LoadOp::Load },
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            if let Some((vb, _)) = &self.vbuf {
                rp.set_vertex_buffer(0, vb.slice(..));
            }
            if let Some((x, y, w, h)) = viewport {
                rp.set_viewport(x, y, w, h, 0.0, 1.0);
            }
            for (i, d) in gl.draws.iter().enumerate() {
                rp.set_pipeline(&self.pipes[&(keys[i], self.format)]);
                rp.set_bind_group(
                    0,
                    &self.uniform_bind,
                    &[(slot_of[i] as u64 * self.uniform_stride) as u32],
                );
                let tb = match d.state.texture {
                    Some(t) => &self.textures[t as usize].bind,
                    None => &self.white.bind,
                };
                rp.set_bind_group(1, tb, &[]);
                rp.draw(d.first..d.first + d.count, 0..1);
            }
        }
        queue.submit([enc.finish()]);
    }
}

fn key_of(s: &DrawState) -> PipeKey {
    PipeKey {
        color_mask: s.color_mask,
        blend: s.blend,
        depth_test: s.depth_test,
        depth_write: s.depth_write,
        depth_gequal: s.depth_gequal,
        depth_equal: s.depth_equal,
    }
}

fn make_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    layout: &wgpu::BindGroupLayout,
    sampler: &wgpu::Sampler,
    img: &Image,
) -> GuiTexture {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("gui texture"),
        size: wgpu::Extent3d {
            width: img.width,
            height: img.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    write(queue, &texture, img);
    let view = texture.create_view(&Default::default());
    let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("gui texture"),
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(sampler),
            },
        ],
    });
    GuiTexture { texture, bind }
}

fn write(queue: &wgpu::Queue, tex: &wgpu::Texture, img: &Image) {
    assert!(img.rgba.len() <= 16_777_216, "BufferOverflowException");
    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: tex,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &img.rgba,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(img.width * 4),
            rows_per_image: None,
        },
        wgpu::Extent3d {
            width: img.width,
            height: img.height,
            depth_or_array_layers: 1,
        },
    );
}

pub(crate) fn write_tile(queue: &wgpu::Queue, texture: &wgpu::Texture, icon: u32, image: &[u8;1024]) {
    if texture.width() < icon%16*16+16 || texture.height() < icon/16*16+16 { return; }
    queue.write_texture(wgpu::TexelCopyTextureInfo { texture, mip_level: 0, origin: wgpu::Origin3d { x: icon%16*16, y: icon/16*16, z: 0 }, aspect: wgpu::TextureAspect::All },
        image, wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(64), rows_per_image: Some(16) }, wgpu::Extent3d { width: 16, height: 16, depth_or_array_layers: 1 });
}

/// GL storage adapter; the same Engine transactions drive gameplay and the oracle.
struct UploadAdapter<'a> { renderer: &'a mut GuiRenderer, device: &'a wgpu::Device, queue: &'a wgpu::Queue }
impl super::render_engine::Backend for UploadAdapter<'_> {
    fn allocate(&mut self) -> i32 {
        let image = Image { width: 1, height: 1, rgba: vec![0,0,0,255] };
        let t = make_texture(self.device, self.queue, &self.renderer.tex_layout, &self.renderer.sampler, &image);
        self.renderer.textures.push(t);
        (self.renderer.textures.len()-1) as i32
    }
    fn setup(&mut self, image: &Image, id: i32, blur: bool, clamp: bool) -> Result<(), super::render_engine::Error> {
        self.renderer.replace_texture(self.device, self.queue, id as TexId, image, blur, clamp);
        Ok(())
    }
    fn delete(&mut self, id: i32) {
        if id >= 0 && (id as usize) < self.renderer.textures.len() {
            self.renderer.replace_texture(self.device, self.queue, id as TexId,
                &Image { width: 1, height: 1, rgba: vec![0,0,0,255] }, false, false);
        }
    }
}
