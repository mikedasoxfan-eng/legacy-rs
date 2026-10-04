//! Additive wgpu implementation of RenderLiving's opaque, alpha, powered and
//! depth-equal overlay passes. Shared by the client and the headless showcase.

use super::living::{self, Batch, EntityVertex, Pass, RenderState};
use crate::render::gpu::Image;
use glam::{DVec3, Mat4};
use std::collections::HashMap;
use std::ops::Range;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    view_proj: [[f32; 4]; 4],
    fog_color: [f32; 4],
    fog: [f32; 4],
}

struct Draw {
    range: Range<u32>,
    texture: String,
    pass: Pass,
}

pub struct EntityPipeline {
    pub states: Vec<RenderState>,
    opaque: wgpu::RenderPipeline,
    alpha: wgpu::RenderPipeline,
    eyes: wgpu::RenderPipeline,
    additive: wgpu::RenderPipeline,
    overlay: wgpu::RenderPipeline,
    uniforms: wgpu::Buffer,
    frame_bind: wgpu::BindGroup,
    textures: HashMap<String, wgpu::BindGroup>,
    vertices: wgpu::Buffer,
    capacity: usize,
    draws: Vec<Draw>,
    normalize: bool,
}

impl EntityPipeline {
    pub fn new<'a>(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        images: impl IntoIterator<Item = (&'a str, &'a Image)>,
    ) -> Self {
        let frame_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("entity frame"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let texture_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("entity texture"),
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
            label: Some("entity layout"),
            bind_group_layouts: &[Some(&frame_layout), Some(&texture_layout)],
            immediate_size: 0,
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("entity shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });
        let make = |label, blend, depth, fragment| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(label),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    compilation_options: Default::default(),
                    buffers: &[Some(wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<EntityVertex>() as u64,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x2,2=>Float32x4],
                    })],
                },
                primitive: wgpu::PrimitiveState { cull_mode: None, ..Default::default() },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: Some(true),
                    depth_compare: Some(depth),
                    stencil: Default::default(),
                    bias: Default::default(),
                }),
                multisample: Default::default(),
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some(fragment),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState { format, blend, write_mask: wgpu::ColorWrites::ALL })],
                }),
                multiview_mask: None,
                cache: None,
            })
        };
        use wgpu::CompareFunction::{Equal, LessEqual};
        let opaque = make("entity opaque", None, LessEqual, "fs_test");
        let alpha = make("entity translucent slime", Some(wgpu::BlendState::ALPHA_BLENDING), LessEqual, "fs_test");
        let eyes = make("entity spider eyes", Some(wgpu::BlendState::ALPHA_BLENDING), LessEqual, "fs_main");
        let additive = make(
            "entity powered creeper",
            Some(wgpu::BlendState {
                color: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::One,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::One,
                    operation: wgpu::BlendOperation::Add,
                },
            }),
            LessEqual,
            "fs_test",
        );
        let overlay = make("entity hurt/flash", Some(wgpu::BlendState::ALPHA_BLENDING), Equal, "fs_main");
        let uniforms = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("entity uniforms"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let frame_bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("entity frame"),
            layout: &frame_layout,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: uniforms.as_entire_binding() }],
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("entity nearest"),
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let white = Image { width: 1, height: 1, rgba: vec![255; 4] };
        let mut textures = HashMap::new();
        let mut upload = |name: &str, img: &Image| {
            let tex = device.create_texture_with_data(
                queue,
                &wgpu::TextureDescriptor {
                    label: Some(name),
                    size: wgpu::Extent3d { width: img.width, height: img.height, depth_or_array_layers: 1 },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    view_formats: &[],
                },
                wgpu::util::TextureDataOrder::LayerMajor,
                &img.rgba,
            );
            let view = tex.create_view(&Default::default());
            textures.insert(
                name.into(),
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some(name),
                    layout: &texture_layout,
                    entries: &[
                        wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&view) },
                        wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&sampler) },
                    ],
                }),
            );
        };
        for (name, img) in images {
            upload(name, img);
        }
        upload("__white", &white);
        let vertices = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("entities"),
            size: 4,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        Self {
            states: Vec::new(),
            opaque,
            alpha,
            eyes,
            additive,
            overlay,
            uniforms,
            frame_bind,
            textures,
            vertices,
            capacity: 0,
            draws: Vec::new(),
            normalize: false,
        }
    }

    pub fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        view_proj: Mat4,
        camera: DVec3,
        fog: [f32; 3],
        far: f32,
    ) {
        let batches = living::scene_with_lighting(&self.states, camera, &mut self.normalize);
        self.upload(device, queue, batches, view_proj, fog, far);
    }

    pub fn upload(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        batches: Vec<Batch>,
        view_proj: Mat4,
        fog: [f32; 3],
        far: f32,
    ) {
        queue.write_buffer(
            &self.uniforms,
            0,
            bytemuck::bytes_of(&Uniforms {
                view_proj: view_proj.to_cols_array_2d(),
                fog_color: [fog[0], fog[1], fog[2], 1.0],
                fog: [far * 0.25, far, 0.0, 0.0],
            }),
        );
        self.draws.clear();
        let mut all = Vec::new();
        for b in batches {
            let start = all.len() as u32;
            all.extend_from_slice(&b.vertices);
            self.draws.push(Draw {
                range: start..all.len() as u32,
                texture: b.texture.unwrap_or("__white").into(),
                pass: b.pass,
            });
        }
        if all.len() > self.capacity {
            self.capacity = all.len().next_power_of_two();
            self.vertices = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("entities"),
                size: (self.capacity * std::mem::size_of::<EntityVertex>()) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }
        if !all.is_empty() {
            queue.write_buffer(&self.vertices, 0, bytemuck::cast_slice(&all));
        }
    }

    pub fn draw(&self, rp: &mut wgpu::RenderPass<'_>) {
        if self.draws.is_empty() {
            return;
        }
        rp.set_bind_group(0, &self.frame_bind, &[]);
        rp.set_vertex_buffer(0, self.vertices.slice(..));
        for d in &self.draws {
            rp.set_pipeline(match d.pass {
                Pass::Opaque => &self.opaque,
                Pass::Alpha => &self.alpha,
                Pass::Eyes => &self.eyes,
                Pass::Additive => &self.additive,
                Pass::Overlay => &self.overlay,
            });
            rp.set_bind_group(1, &self.textures[&d.texture], &[]);
            rp.draw(d.range.clone(), 0..1);
        }
    }
}
