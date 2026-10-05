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
    line: crate::render::color_mask::Pipelines,
    opaque: crate::render::color_mask::Pipelines,
    alpha: crate::render::color_mask::Pipelines,
    eyes: crate::render::color_mask::Pipelines,
    eyes_overlay: crate::render::color_mask::Pipelines,
    additive: crate::render::color_mask::Pipelines,
    overlay: crate::render::color_mask::Pipelines,
    shadow: crate::render::color_mask::Pipelines,
    tnt_flash: crate::render::color_mask::Pipelines,
    damage: crate::render::color_mask::Pipelines,
    untested: crate::render::color_mask::Pipelines,
    untested_alpha: crate::render::color_mask::Pipelines,
    pub extra: Vec<Batch>,
    pub fog_override: Option<[f32; 3]>,
    pub fog_view: Mat4,
    pub snapshots: Vec<super::effects::Snapshot>,
    pub pistons: Vec<crate::render::pistons::Snapshot>,
    pub angler: Option<worldgen::entity::Entity>,
    pub third_person: bool,
    pub signs: Vec<crate::render::sign::Snapshot>,
    /// Batches whose vertices carry their own lightmap brightness (batch index, per vertex).
    vertex_lights: Vec<(usize, Vec<u32>)>,
    pub chests: Vec<super::chest::Tile>,
    pub spawners: Vec<crate::render::spawner::Snapshot>,
    pub partial: f32,
    pub fancy: bool,
    pub view: [f32; 2],
    uniforms: wgpu::Buffer,
    frame_bind: wgpu::BindGroup,
    texture_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    shadow_sampler: wgpu::Sampler,
    textures: HashMap<String, wgpu::BindGroup>,
    items_tex: Option<wgpu::Texture>,
    terrain_tex: Option<wgpu::Texture>,
    map_tex: Option<wgpu::Texture>,
    vertices: wgpu::Buffer,
    /// b1.8.1: per-vertex lightmap brightness (`Entity.func_35115_a`), `NO_LIGHTMAP` when the
    /// lightmap unit is off for that draw.
    lights: wgpu::Buffer,
    lightmap: wgpu::Texture,
    capacity: usize,
    draws: Vec<Draw>,
    normalize: bool,
    enderman_random: crate::render::weather::ClientRandom,
}

/// Light value of a vertex drawn without the lightmap (texture unit 1 disabled).
pub const NO_LIGHTMAP: u32 = u32::MAX;

impl EntityPipeline {
    pub fn new<'a>(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        images: impl IntoIterator<Item = (&'a str, &'a Image)>,
    ) -> Self {
        let frame_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("entity frame"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
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
            crate::render::color_mask::Pipelines::new(|mask| device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(label),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    compilation_options: Default::default(),
                    buffers: &[
                        Some(wgpu::VertexBufferLayout {
                            array_stride: std::mem::size_of::<EntityVertex>() as u64,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x2,2=>Float32x4],
                        }),
                        Some(wgpu::VertexBufferLayout {
                            array_stride: 4,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &wgpu::vertex_attr_array![3=>Uint32],
                        }),
                    ],
                },
                primitive: wgpu::PrimitiveState { topology: if label=="fishing line" {wgpu::PrimitiveTopology::LineStrip}else{wgpu::PrimitiveTopology::TriangleList}, cull_mode: None, ..Default::default() },
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
                    targets: &[Some(wgpu::ColorTargetState { format, blend, write_mask: mask })],
                }),
                multiview_mask: None,
                cache: None,
            }))
        };
        use wgpu::CompareFunction::{Equal, LessEqual};
        let line=make("fishing line",None,LessEqual,"fs_main",true);
        let opaque = make("entity opaque", None, LessEqual, "fs_test", true);
        let alpha =
            make("entity translucent slime", Some(wgpu::BlendState::ALPHA_BLENDING), LessEqual, "fs_test", true);
        let eye_blend = wgpu::BlendState { color: wgpu::BlendComponent { src_factor: wgpu::BlendFactor::One, dst_factor: wgpu::BlendFactor::One, operation: wgpu::BlendOperation::Add }, alpha: wgpu::BlendComponent { src_factor: wgpu::BlendFactor::One, dst_factor: wgpu::BlendFactor::One, operation: wgpu::BlendOperation::Add } };
        let eyes = make("entity spider eyes", Some(eye_blend), LessEqual, "fs_main", true);
        let eyes_overlay = make("entity eye hurt overlay", Some(eye_blend), Equal, "fs_main", true);
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
        // This pipeline's copy of the b1.8.1 lightmap (white until the first set_lightmap).
        let lightmap = device.create_texture_with_data(
            queue,
            &wgpu::TextureDescriptor {
                label: Some("entity lightmap"),
                size: wgpu::Extent3d { width: 16, height: 16, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            },
            wgpu::util::TextureDataOrder::LayerMajor,
            &[255u8; 1024],
        );
        let lightmap_view = lightmap.create_view(&Default::default());
        let lightmap_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("entity lightmap"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let frame_bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("entity frame"),
            layout: &frame_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: uniforms.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&lightmap_view) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&lightmap_sampler) },
            ],
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
        let mut items_tex = None;
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
            if name == "gui/items.png" { items_tex = Some(tex.clone()); }
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
        let lights = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("entity lights"),
            size: 4,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        Self {
            lights,
            lightmap,
            states: Vec::new(),
            damage,
            untested,
            untested_alpha,
            extra: Vec::new(),
            fog_override: None,
            fog_view: Mat4::IDENTITY,
            snapshots: Vec::new(),
            pistons: Vec::new(),
            angler: None,
            third_person: false,
            signs: Vec::new(),
            vertex_lights: Vec::new(),
            chests: Vec::new(),
            spawners: Vec::new(),
            partial: 0.0,
            fancy: true,
            view: [0.0; 2],
            shadow,
            tnt_flash,
            line,
            opaque,
            alpha,
            eyes,
            eyes_overlay,
            additive,
            overlay,
            uniforms,
            frame_bind,
            texture_layout,
            sampler,
            shadow_sampler,
            textures,
            items_tex,
            terrain_tex,
            map_tex,
            vertices,
            capacity: 0,
            draws: Vec::new(),
            normalize: false,
            enderman_random: crate::render::weather::ClientRandom::new(worldgen::entity::EntitySeedSource::Time.next_seed()),
        }
    }

    pub fn replace_texture(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, name: &str, img: &Image) {
        let tex = device.create_texture_with_data(queue, &wgpu::TextureDescriptor {
            label: Some(name), size: wgpu::Extent3d { width: img.width, height: img.height, depth_or_array_layers: 1 },
            mip_level_count: 1, sample_count: 1, dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm, usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST, view_formats: &[],
        }, wgpu::util::TextureDataOrder::LayerMajor, &img.rgba);
        if name == "gui/items.png" { self.items_tex = Some(tex.clone()); }
        if name == "terrain.png" { self.terrain_tex = Some(tex.clone()); }
        if name == "__map" { self.map_tex = Some(tex.clone()); }
        let view = tex.create_view(&Default::default());
        self.textures.insert(name.into(), device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(name), layout: &self.texture_layout, entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(&view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(if name == "misc/shadow.png" { &self.shadow_sampler } else { &self.sampler }) },
            ],
        }));
    }

    /// RenderEngine.updateDynamicTextures also updates the terrain atlas bound by entities.
    pub fn update_items(&self, queue: &wgpu::Queue, fx: &crate::render::item_fx::ItemFx) {
        if let Some(texture) = &self.items_tex {
            for (i, icon) in crate::render::item_fx::ICONS.into_iter().enumerate() { crate::gui::render::write_tile(queue, texture, icon, &fx.images[i]); }
        }
    }
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
        let mut lights = vec![NO_LIGHTMAP; batches.len()];
        for snapshot in &self.snapshots {
            let e = &snapshot.entity;
            let first = batches.len();
            let mut explicit_lights = Vec::new();
            // b1.8.1 RenderManager.renderEntity: glColor 1 and the entity's lightmap coordinates;
            // the colour no longer carries the brightness.
            let overlay_brightness = snapshot.brightness;
            let brightness = if snapshot.light.is_some() { 1.0 } else { snapshot.brightness };
            let snapshot = &super::effects::Snapshot { brightness, ..snapshot.clone() };
            if let worldgen::entity::EntityKind::Fish(s)=&e.kind {
                let owner=self.angler.as_ref().filter(|p|Some(p.entity_id)==s.arrow.owner).or_else(||self.snapshots.iter().find(|v|Some(v.entity.entity_id)==s.arrow.owner).map(|v|&v.entity));
                let p=super::bridge::position(e,self.partial);let relative=std::array::from_fn(|i|p[i]-camera[i]);let g=crate::render::fishing::render(e,owner,relative,self.partial,snapshot.brightness,self.view,self.third_person);batches.extend(g.draws.iter().map(|d|d.batch()));
            }
            if matches!(e.kind,worldgen::entity::EntityKind::Painting(_)) {
                let lights:Vec<_>=crate::render::painting::light_positions(e).iter().map(|p|snapshot.surfaces.iter().find(|s|s.xyz==*p&&s.block==0).map_or(snapshot.brightness,|s|s.brightness)).collect();
                let p=super::bridge::position(e,self.partial);let relative=std::array::from_fn(|i|p[i]-camera[i] as f64);
                let g=crate::render::painting::render(e,relative,&lights);batches.extend(g.draws.iter().map(|d|d.batch()));
            }
            if let Some(mut s) = super::bridge::render_state(e, self.partial, snapshot.brightness) {
                s.overlay_brightness = overlay_brightness;
                if s.kind == living::MobKind::Enderman && s.screaming { s.scream_jitter = [self.enderman_random.gaussian()*0.02,self.enderman_random.gaussian()*0.02]; }
                if matches!(s.kind,living::MobKind::Enderman|living::MobKind::Silverfish|living::MobKind::Spider|living::MobKind::CaveSpider|living::MobKind::Zombie|living::MobKind::Skeleton|living::MobKind::Pig|living::MobKind::Cow|living::MobKind::Sheep|living::MobKind::Chicken|living::MobKind::Squid|living::MobKind::Wolf|living::MobKind::Creeper) {
                    let g=super::raw_living181::render(&s,camera.to_array(),self.normalize);
                    explicit_lights=super::raw_living181::packed_brightnesses(&g.draws,snapshot.light.unwrap_or(NO_LIGHTMAP) as i32).into_iter().map(|v|v as u32).collect();
                    batches.extend(g.draws.iter().map(|d| {
                        let mut batch=d.batch();
                        if d.texture==58 && d.flags & 8 != 0 {for vertex in &mut batch.vertices {vertex.uv[0]+=s.age*0.01;vertex.uv[1]+=s.age*0.01;}}
                        batch
                    }));
                } else { batches.extend(living::scene_with_lighting(&[s], camera, &mut self.normalize)); }
            } else if let Some(mut s) =
                super::bridge::object_state(e, self.partial, snapshot.brightness, snapshot.falling_light)
            {
                if let (super::objects::ObjectKind::Vehicle(v),Some(rails))=(&mut s.kind,&snapshot.vehicle_rails) {
                    v.track=rails.sample(v.position);
                }
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
            // RenderLiving turns the lightmap off for its hurt/flash overlay passes.
            let light = snapshot.light.unwrap_or(NO_LIGHTMAP);
            let trailing_light=explicit_lights.last().copied().unwrap_or(light);
            lights.extend(batches[first..].iter().enumerate().map(|(i,b)| {
                if matches!(b.pass, Pass::Overlay | Pass::EyesOverlay | Pass::Damage | Pass::TntFlash) { NO_LIGHTMAP } else { explicit_lights.get(i).copied().unwrap_or(trailing_light) }
            }));
        }
        lights.resize(batches.len(), NO_LIGHTMAP);
        batches.append(&mut self.extra);
        lights.resize(batches.len(), NO_LIGHTMAP);
        // Moving pistons: RenderBlocks geometry with per-vertex lightmap brightness.
        for p in self.pistons.iter().filter(|p|p.visible(camera.to_array())) {
            let (b, l) = p.batch(self.partial, camera.to_array());
            self.vertex_lights.push((batches.len(), l));
            batches.push(b);
            lights.push(NO_LIGHTMAP);
        }
        for s in &self.signs {
            let relative=std::array::from_fn(|i|s.position[i] as f64-camera[i] as f64);
            let g=crate::render::sign::render(&s.sign,s.block,s.metadata,relative,s.brightness);
            batches.extend(g.draws.iter().map(|d|d.batch()));
            lights.resize(batches.len(), s.light);
        }
        for c in &self.chests {
            let g=super::chest::render(&super::chest::ChestState{
                position:std::array::from_fn(|i|c.position[i] as f64-camera[i] as f64),
                metadata:c.metadata,adjacent:c.adjacent,lid:c.lid,prev_lid:c.prev_lid,partial:self.partial});
            batches.extend(g.draws.iter().map(|d|d.batch()));
            lights.resize(batches.len(), c.light);
        }
        for s in &self.spawners {
            if let Some(st) = crate::render::spawner::state(s, camera.to_array(), self.partial) {
                batches.extend(living::scene_with_lighting(&[st], DVec3::ZERO, &mut self.normalize));
                lights.resize(batches.len(), s.light);
            }
        }
        self.upload_lit(device, queue, batches, &lights, view_proj, fog, far);
    }

    /// Upload this frame's lightmap texels (`EntityRenderer.func_35808_e`, ARGB).
    pub fn set_lightmap(&self, queue: &wgpu::Queue, texels: &[u32; 256]) {
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.lightmap,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &crate::render::lightmap::rgba(texels),
            wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(64), rows_per_image: Some(16) },
            wgpu::Extent3d { width: 16, height: 16, depth_or_array_layers: 1 },
        );
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
        let lights = vec![NO_LIGHTMAP; batches.len()];
        self.upload_lit(device, queue, batches, &lights, view_proj, fog, far);
    }

    /// `upload` with one lightmap brightness per batch (`NO_LIGHTMAP`: lightmap off).
    #[allow(clippy::too_many_arguments)]
    pub fn upload_lit(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        batches: Vec<Batch>,
        batch_lights: &[u32],
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
        let mut light_of = Vec::new();
        let mut per_vertex: std::collections::HashMap<usize, Vec<u32>> = std::mem::take(&mut self.vertex_lights).into_iter().collect();
        for (i, b) in batches.into_iter().enumerate() {
            let start = all.len() as u32;
            all.extend_from_slice(&b.vertices);
            match per_vertex.remove(&i) {
                Some(l) if l.len() == b.vertices.len() => light_of.extend(l),
                _ => light_of.resize(all.len(), batch_lights.get(i).copied().unwrap_or(NO_LIGHTMAP)),
            }
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
            self.lights = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("entity lights"),
                size: (self.capacity * 4) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }
        if !all.is_empty() {
            queue.write_buffer(&self.vertices, 0, bytemuck::cast_slice(&all));
            queue.write_buffer(&self.lights, 0, bytemuck::cast_slice(&light_of));
        }
    }

    pub fn draw(&self, rp: &mut wgpu::RenderPass<'_>) { self.draw_masked(rp, 0); }
    pub fn draw_masked(&self, rp: &mut wgpu::RenderPass<'_>, mask: usize) {
        if self.draws.is_empty() {
            return;
        }
        rp.set_bind_group(0, &self.frame_bind, &[]);
        rp.set_vertex_buffer(0, self.vertices.slice(..));
        rp.set_vertex_buffer(1, self.lights.slice(..));
        for d in &self.draws {
            rp.set_pipeline((match d.pass {
                Pass::Line => &self.line,
                Pass::Opaque => &self.opaque,
                Pass::Alpha => &self.alpha,
                Pass::Eyes => &self.eyes,
                Pass::EyesOverlay => &self.eyes_overlay,
                Pass::Additive => &self.additive,
                Pass::Overlay => &self.overlay,
                Pass::Shadow => &self.shadow,
                Pass::TntFlash => &self.tnt_flash,
                Pass::Damage => &self.damage,
                Pass::Untested => &self.untested,
                Pass::UntestedAlpha => &self.untested_alpha,
            }).get(mask));
            rp.set_bind_group(1, &self.textures[&d.texture], &[]);
            rp.draw(d.range.clone(), 0..1);
        }
    }
}
