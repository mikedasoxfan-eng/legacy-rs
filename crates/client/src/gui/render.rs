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
};

@vertex
fn vs(@location(0) pos: vec3<f32>, @location(1) uv: vec2<f32>, @location(2) color: vec4<f32>, @location(3) flags: vec2<f32>) -> VO {
    var o: VO;
    var c = u.proj * vec4<f32>(pos, 1.0);
    // OpenGL depth range [-1, 1] -> [0, 1].
    c.z = (c.z + c.w) * 0.5;
    o.pos = c;
    o.uv = uv;
    o.color = color;
    o.flags = flags;
    return o;
}

@fragment
fn fs(i: VO) -> @location(0) vec4<f32> {
    var c = clamp(i.color, vec4<f32>(0.0), vec4<f32>(1.0));
    let t = textureSampleLevel(tex, samp, i.uv, 0.0);
    if (i.flags.x > 0.5) {
        c = c * t;
    }
    if (i.flags.y >= 0.0 && !(c.a > i.flags.y)) {
        discard;
    }
    return c;
}
"#;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct PipeKey {
    blend: Option<(Factor, Factor)>,
    depth_test: bool,
    depth_write: bool,
}

struct GuiTexture {
    texture: wgpu::Texture,
    bind: wgpu::BindGroup,
}

pub struct GuiRenderer {
    format: wgpu::TextureFormat,
    /// Multisample count (the driver may force MSAA on the original's GL window).
    samples: u32,
    shader: wgpu::ShaderModule,
    layout: wgpu::PipelineLayout,
    pipes: HashMap<PipeKey, wgpu::RenderPipeline>,
    tex_layout: wgpu::BindGroupLayout,
    uniforms: wgpu::Buffer,
    uniform_bind: wgpu::BindGroup,
    uniform_stride: u64,
    uniform_slots: u64,
    sampler: wgpu::Sampler,
    textures: Vec<GuiTexture>,
    white: GuiTexture,
    vbuf: Option<(wgpu::Buffer, u64)>,
    depth: Option<(wgpu::Texture, wgpu::TextureView, u32, u32)>,
    /// Multisampled color target, resolved into the real target after drawing.
    msaa: Option<(wgpu::Texture, wgpu::TextureView, u32, u32)>,
}

fn factor(f: Factor) -> wgpu::BlendFactor {
    match f {
        Factor::Zero => wgpu::BlendFactor::Zero,
        Factor::One => wgpu::BlendFactor::One,
        Factor::SrcAlpha => wgpu::BlendFactor::SrcAlpha,
        Factor::OneMinusSrcAlpha => wgpu::BlendFactor::OneMinusSrcAlpha,
        Factor::DstColor => wgpu::BlendFactor::Dst,
        Factor::OneMinusDstColor => wgpu::BlendFactor::OneMinusDst,
        Factor::SrcColor => wgpu::BlendFactor::Src,
        Factor::OneMinusSrcColor => wgpu::BlendFactor::OneMinusSrc,
    }
}

impl GuiRenderer {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat, samples: u32) -> GuiRenderer {
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
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
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
        let uniform_slots = 256;
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
        let white = make_texture(device, queue, &tex_layout, &sampler, &Image { width: 1, height: 1, rgba: vec![255; 4] });
        GuiRenderer {
            format,
            samples,
            shader,
            layout,
            pipes: HashMap::new(),
            tex_layout,
            uniforms,
            uniform_bind,
            uniform_stride,
            uniform_slots,
            sampler,
            textures: Vec::new(),
            white,
            vbuf: None,
            depth: None,
            msaa: None,
        }
    }

    /// Register a texture (like `RenderEngine.getTexture`); returns its id.
    pub fn add_texture(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, img: &Image) -> TexId {
        let t = make_texture(device, queue, &self.tex_layout, &self.sampler, img);
        self.textures.push(t);
        (self.textures.len() - 1) as TexId
    }

    /// Replace a texture's pixels (same size as when it was added).
    pub fn update_texture(&self, queue: &wgpu::Queue, id: TexId, img: &Image) {
        write(queue, &self.textures[id as usize].texture, img);
    }

    fn pipeline(&mut self, device: &wgpu::Device, key: PipeKey) -> &wgpu::RenderPipeline {
        let (format, shader, layout, samples) = (self.format, &self.shader, &self.layout, self.samples);
        self.pipes.entry(key).or_insert_with(|| {
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
                        attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2, 2 => Float32x4, 3 => Float32x2],
                    })],
                },
                fragment: Some(wgpu::FragmentState {
                    module: shader,
                    entry_point: Some("fs"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState { format, blend, write_mask: wgpu::ColorWrites::ALL })],
                }),
                primitive: wgpu::PrimitiveState { cull_mode: None, ..Default::default() },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: Some(key.depth_write && key.depth_test),
                    depth_compare: Some(if key.depth_test { wgpu::CompareFunction::LessEqual } else { wgpu::CompareFunction::Always }),
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
        if self.depth.as_ref().is_none_or(|d| (d.2, d.3) != size) {
            let t = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("gui depth"),
                size: wgpu::Extent3d { width: size.0, height: size.1, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: self.samples,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth32Float,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            });
            let v = t.create_view(&Default::default());
            self.depth = Some((t, v, size.0, size.1));
            self.msaa = (self.samples > 1).then(|| {
                let t = device.create_texture(&wgpu::TextureDescriptor {
                    label: Some("gui msaa"),
                    size: wgpu::Extent3d { width: size.0, height: size.1, depth_or_array_layers: 1 },
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
        if !bytes.is_empty() && self.vbuf.as_ref().is_none_or(|(_, cap)| *cap < bytes.len() as u64) {
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
        // One projection slot per distinct projection.
        let mut projs: Vec<glam::Mat4> = Vec::new();
        let mut slot_of = Vec::with_capacity(gl.draws.len());
        for d in &gl.draws {
            let i = match projs.iter().position(|p| *p == d.proj) {
                Some(i) => i,
                None => {
                    projs.push(d.proj);
                    projs.len() - 1
                }
            };
            slot_of.push(i.min(self.uniform_slots as usize - 1));
        }
        for (i, p) in projs.iter().enumerate().take(self.uniform_slots as usize) {
            queue.write_buffer(&self.uniforms, i as u64 * self.uniform_stride, bytemuck::cast_slice(&p.to_cols_array()));
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
                            Some(c) => wgpu::LoadOp::Clear(wgpu::Color { r: c[0], g: c[1], b: c[2], a: c[3] }),
                            None => wgpu::LoadOp::Load,
                        },
                        store: wgpu::StoreOp::Store,
                    },
                })],
                // EntityRenderer clears the depth buffer before drawing a screen.
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: depth_view,
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Discard }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            if let Some((vb, _)) = &self.vbuf {
                rp.set_vertex_buffer(0, vb.slice(..));
            }
            for (i, d) in gl.draws.iter().enumerate() {
                rp.set_pipeline(&self.pipes[&keys[i]]);
                rp.set_bind_group(0, &self.uniform_bind, &[(slot_of[i] as u64 * self.uniform_stride) as u32]);
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
    PipeKey { blend: s.blend, depth_test: s.depth_test, depth_write: s.depth_write }
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
        size: wgpu::Extent3d { width: img.width, height: img.height, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    write(queue, &texture, img);
    let view = texture.create_view(&Default::default());
    let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("gui texture"),
        layout,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&view) },
            wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(sampler) },
        ],
    });
    GuiTexture { texture, bind }
}

fn write(queue: &wgpu::Queue, tex: &wgpu::Texture, img: &Image) {
    queue.write_texture(
        wgpu::TexelCopyTextureInfo { texture: tex, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
        &img.rgba,
        wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(img.width * 4), rows_per_image: None },
        wgpu::Extent3d { width: img.width, height: img.height, depth_or_array_layers: 1 },
    );
}
