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
    fog_view: [[f32; 4]; 4],
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
    shadow: wgpu::RenderPipeline,
    tnt_flash: wgpu::RenderPipeline,
    damage: wgpu::RenderPipeline,
    untested: wgpu::RenderPipeline,
    untested_alpha: wgpu::RenderPipeline,
    pub extra: Vec<Batch>,
    pub fog_override: Option<[f32; 3]>,
    pub fog_view: Mat4,
    pub snapshots: Vec<super::effects::Snapshot>,
    pub partial: f32,
    pub fancy: bool,
    pub view: [f32; 2],
    uniforms: wgpu::Buffer,
    frame_bind: wgpu::BindGroup,
    textures: HashMap<String, wgpu::BindGroup>,
    terrain_tex: Option<wgpu::Texture>,
    map_tex: Option<wgpu::Texture>,
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
        let make = |label, blend, depth, fragment, write| {
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
                    depth_write_enabled: Some(write),
                    depth_compare: Some(depth),
                    stencil: Default::default(),
                    bias: if label == "block damage" { wgpu::DepthBiasState { constant: -3, slope_scale: -3.0, clamp: 0.0 } } else { Default::default() },
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
        let opaque = make("entity opaque", None, LessEqual, "fs_test", true);
        let alpha =
            make("entity translucent slime", Some(wgpu::BlendState::ALPHA_BLENDING), LessEqual, "fs_test", true);
        let eyes = make("entity spider eyes", Some(wgpu::BlendState::ALPHA_BLENDING), LessEqual, "fs_main", true);
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
            true,
        );
        let overlay = make("entity hurt/flash", Some(wgpu::BlendState::ALPHA_BLENDING), Equal, "fs_main", true);
        let shadow = make("entity shadow", Some(wgpu::BlendState::ALPHA_BLENDING), LessEqual, "fs_test", false);
        let tnt_flash = make(
            "TNT flash",
            Some(wgpu::BlendState {
                color: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::DstAlpha,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::DstAlpha,
                    operation: wgpu::BlendOperation::Add,
                },
            }),
            LessEqual,
            "fs_test",
            true,
        );
        let damage = make(
            "block damage",
            Some(wgpu::BlendState {
                color: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::Dst,
                    dst_factor: wgpu::BlendFactor::Src,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::Dst,
                    dst_factor: wgpu::BlendFactor::Src,
                    operation: wgpu::BlendOperation::Add,
                },
            }),
            LessEqual,
            "fs_test",
            true,
        );
        let untested = make("overlay opaque", None, LessEqual, "fs_main", true);
        let untested_alpha = make(
            "overlay alpha",
            Some(wgpu::BlendState::ALPHA_BLENDING),
            LessEqual,
            "fs_main",
            true,
        );
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
        let shadow_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("shadow clamp"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let white = Image { width: 1, height: 1, rgba: vec![255; 4] };
        let mut textures = HashMap::new();
        let mut terrain_tex = None;
        let mut map_tex = None;
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
            if name == "terrain.png" {
                terrain_tex = Some(tex.clone());
            }
            if name == "__map" {
                map_tex = Some(tex.clone());
            }
            let view = tex.create_view(&Default::default());
            textures.insert(
                name.into(),
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some(name),
                    layout: &texture_layout,
                    entries: &[
                        wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&view) },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(if name == "misc/shadow.png" {
                                &shadow_sampler
                            } else {
                                &sampler
                            }),
                        },
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
            damage,
            untested,
            untested_alpha,
            extra: Vec::new(),
            fog_override: None,
            fog_view: Mat4::IDENTITY,
            snapshots: Vec::new(),
            partial: 0.0,
            fancy: true,
            view: [0.0; 2],
            shadow,
            tnt_flash,
            opaque,
            alpha,
            eyes,
            additive,
            overlay,
            uniforms,
            frame_bind,
            textures,
            terrain_tex,
            map_tex,
            vertices,
            capacity: 0,
            draws: Vec::new(),
            normalize: false,
        }
    }

    /// RenderEngine.updateDynamicTextures also updates the terrain atlas bound by entities.
    pub fn update_map(&self, queue: &wgpu::Queue, image: &Image) {
        if let Some(texture) = &self.map_tex {
            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &image.rgba,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(512),
                    rows_per_image: Some(128),
                },
                wgpu::Extent3d {
                    width: 128,
                    height: 128,
                    depth_or_array_layers: 1,
                },
            );
        }
    }
    pub fn update_fx(&self, queue: &wgpu::Queue, fx: &[crate::render::fx::TextureFx]) {
        let Some(texture) = &self.terrain_tex else {
            return;
        };
        for f in fx {
            for i in 0..f.tile_size {
                for j in 0..f.tile_size {
                    queue.write_texture(
                        wgpu::TexelCopyTextureInfo {
                            texture,
                            mip_level: 0,
                            origin: wgpu::Origin3d { x: f.icon % 16 * 16 + i * 16, y: f.icon / 16 * 16 + j * 16, z: 0 },
                            aspect: wgpu::TextureAspect::All,
                        },
                        &f.image,
                        wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(64), rows_per_image: Some(16) },
                        wgpu::Extent3d { width: 16, height: 16, depth_or_array_layers: 1 },
                    );
                }
            }
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
        let mut batches = living::scene_with_lighting(&self.states, camera, &mut self.normalize);
        for snapshot in &self.snapshots {
            let e = &snapshot.entity;
            if let Some(s) = super::bridge::render_state(e, self.partial, snapshot.brightness) {
                batches.extend(living::scene_with_lighting(&[s], camera, &mut self.normalize));
            } else if let Some(s) =
                super::bridge::object_state(e, self.partial, snapshot.brightness, snapshot.falling_light)
            {
                let mut g = crate::world_effects::Effects::object_draws(&s, camera.to_array(), self.view, self.fancy);
                for d in &mut g.draws {
                    if self.normalize {
                        d.flags |= 4;
                    }
                }
                batches.extend(g.draws.iter().map(|d| d.batch()));
            }
            if !matches!(e.kind, worldgen::entity::EntityKind::Lightning { .. }) {
                if self.fancy {
                    let g = super::effects::shadow(e, &snapshot.surfaces, self.partial, camera.to_array());
                    batches.extend(g.draws.iter().map(|d| d.batch()));
                }
                batches.extend(
                    super::effects::fire(e, self.partial, camera.to_array(), self.view[0])
                        .draws
                        .iter()
                        .map(|d| d.batch()),
                );
            }
        }
        batches.append(&mut self.extra);
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
        let f = self.fog_override.unwrap_or(if far > 0.0 {
            [far * 0.25, far, 1.0]
        } else {
            [0.0, 1.0, 0.0]
        });
        queue.write_buffer(
            &self.uniforms,
            0,
            bytemuck::bytes_of(&Uniforms {
                view_proj: view_proj.to_cols_array_2d(),
                fog_view: self.fog_view.to_cols_array_2d(),
                fog_color: [fog[0], fog[1], fog[2], 1.0],
                fog: [f[0], f[1], f[2], 0.0],
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
                Pass::Shadow => &self.shadow,
                Pass::TntFlash => &self.tnt_flash,
                Pass::Damage => &self.damage,
                Pass::Untested => &self.untested,
                Pass::UntestedAlpha => &self.untested_alpha,
            });
            rp.set_bind_group(1, &self.textures[&d.texture], &[]);
            rp.draw(d.range.clone(), 0..1);
        }
    }
}
