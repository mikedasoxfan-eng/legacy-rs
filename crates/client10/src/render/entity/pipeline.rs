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
    cull: u8,
    range: Range<u32>,
    texture: String,
    pass: Pass,
}

struct RasterPipelines {variants: [crate::render::color_mask::Pipelines; 3]}
impl RasterPipelines {
    fn new(mut create: impl FnMut(Option<wgpu::Face>) -> crate::render::color_mask::Pipelines) -> Self {
        Self { variants: [create(None), create(Some(wgpu::Face::Front)), create(Some(wgpu::Face::Back))] }
    }
    fn get(&self, mask: usize, cull: u8) -> &wgpu::RenderPipeline {self.variants[cull as usize].get(mask)}
}

pub struct EntityPipeline {
    pub states: Vec<RenderState>,
    line: RasterPipelines,
    opaque: RasterPipelines,
    opaque_equal: RasterPipelines,
    alpha: RasterPipelines,
    eyes: RasterPipelines,
    eyes_overlay: RasterPipelines,
    additive: RasterPipelines,
    rays: RasterPipelines,
    overlay: RasterPipelines,
    shadow: RasterPipelines,
    tnt_flash: RasterPipelines,
    damage: RasterPipelines,
    untested: RasterPipelines,
    untested_alpha: RasterPipelines,
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
    pub books: Vec<super::render_release::BookTile>,
    pub portals: Vec<[i32;3]>,
    pub active_info: crate::render::active_info::Info,
    projective_uvs: Vec<(usize,Vec<[f32;3]>)>,
    uvq: wgpu::Buffer,
    pub dragon_crystals: std::collections::HashMap<i32, ([f64; 3], i32)>,
    pub chests: Vec<super::chest::Tile>,
    pub spawners: Vec<crate::render::spawner::Snapshot>,
    pub partial: f32,
    pub fancy: bool,
    pub view: [f32; 2],
    pub manager: super::manager::RenderManager,
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
            RasterPipelines::new(|cull| crate::render::color_mask::Pipelines::new(|mask| device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                            array_stride: 8,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &wgpu::vertex_attr_array![3=>Uint32,4=>Float32],
                        }),
                        Some(wgpu::VertexBufferLayout {array_stride:12,step_mode:wgpu::VertexStepMode::Vertex,attributes:&wgpu::vertex_attr_array![5=>Float32x3]}),
                    ],
                },
                primitive: wgpu::PrimitiveState { topology: if label=="fishing line" {wgpu::PrimitiveTopology::LineStrip}else{wgpu::PrimitiveTopology::TriangleList}, cull_mode: cull, ..Default::default() },
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
            })))
        };
        use wgpu::CompareFunction::{Equal, LessEqual};
        let line=make("fishing line",None,LessEqual,"fs_main",true);
        let opaque = make("entity opaque", None, LessEqual, "fs_test", true);
        let opaque_equal = make("entity opaque equal", None, Equal, "fs_test", true);
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
        let rays = make("dragon death rays", Some(wgpu::BlendState {
            color: wgpu::BlendComponent {src_factor: wgpu::BlendFactor::SrcAlpha, dst_factor: wgpu::BlendFactor::One, operation: wgpu::BlendOperation::Add},
            alpha: wgpu::BlendComponent {src_factor: wgpu::BlendFactor::SrcAlpha, dst_factor: wgpu::BlendFactor::One, operation: wgpu::BlendOperation::Add},
        }), LessEqual, "fs_main", false);
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
            size: 8,
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
            books: Vec::new(),
            portals: Vec::new(),
            active_info: Default::default(),
            projective_uvs: Vec::new(),
            uvq: device.create_buffer(&wgpu::BufferDescriptor {label:Some("projective UV"),size:12,usage:wgpu::BufferUsages::VERTEX|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false}),
            dragon_crystals: std::collections::HashMap::new(),
            chests: Vec::new(),
            spawners: Vec::new(),
            partial: 0.0,
            fancy: true,
            view: [0.0; 2],
            manager: Default::default(),
            shadow,
            tnt_flash,
            line,
            opaque,
            opaque_equal,
            alpha,
            eyes,
            eyes_overlay,
            additive,
            rays,
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
            self.manager.render_origin = camera.to_array();
            let distance_camera = self.manager.camera;
            self.manager.render_entity(
                Some(super::manager::class(e)), Some(e.into()), self.partial,
                |_| Ok(snapshot.light.unwrap_or(0) as i32),
                |event| {
                    match event {
                        super::manager::Event::Body(_, _) => {
                            if let worldgen::entity::EntityKind::Fish(s)=&e.kind {
                                let owner=self.angler.as_ref().filter(|p|Some(p.entity_id)==s.arrow.owner).or_else(||self.snapshots.iter().find(|v|Some(v.entity.entity_id)==s.arrow.owner).map(|v|&v.entity));
                                let p=super::bridge::position(e,self.partial);let relative=std::array::from_fn(|i|p[i]-camera[i]);let g=crate::render::fishing::render(e,owner,relative,self.partial,snapshot.brightness,self.view,self.third_person);batches.extend(g.draws.iter().map(|d|d.batch()));
                            }
                            if matches!(e.kind,worldgen::entity::EntityKind::Painting(_)) {
                                // 1.0 RenderPainting: glColor 1, each tile through the lightmap at its own light.
                                let fallback=snapshot.light.unwrap_or(0) as i32;
                                let lights:Vec<_>=(0..crate::render::painting::light_positions(e).len()).map(|i|snapshot.tile_lights.get(i).copied().unwrap_or(fallback)).collect();
                                let p=super::bridge::position(e,self.partial);let relative=std::array::from_fn(|i|p[i]-camera[i] as f64);
                                let g=crate::render::painting::render(e,relative,&lights);batches.extend(g.draws.iter().map(|d|d.batch()));
                                explicit_lights.extend(g.draw_lightmaps.iter().map(|l|(l[0] as u32)+(l[1] as u32)*65536));
                            }
                            if let Some(mut s) = super::bridge::render_state(e, self.partial, snapshot.brightness) {
                                if let Some(crystal_id) = s.dragon.as_ref().and_then(|d| d.healing_crystal) {
                                    s.dragon_crystal = self.dragon_crystals.get(&crystal_id).copied();
                                }
                                s.overlay_brightness = overlay_brightness;
                                if s.kind == living::MobKind::Enderman && s.screaming { s.scream_jitter = [self.enderman_random.gaussian()*0.02,self.enderman_random.gaussian()*0.02]; }
                                {
                                    let g=super::raw_living181::render(&s,camera.to_array(),self.normalize);
                                    explicit_lights=super::raw_living181::packed_brightnesses(&g.draws,snapshot.light.unwrap_or(NO_LIGHTMAP) as i32).into_iter().map(|v|v as u32).collect();
                                    batches.extend(g.draws.iter().map(|d| {
                                        let mut batch=d.batch();
                                        if d.texture==58 && d.flags & 8 != 0 {for vertex in &mut batch.vertices {vertex.uv[0]+=s.age*0.01;vertex.uv[1]+=s.age*0.01;}}
                                        batch
                                    }));
                                    if s.kind == living::MobKind::Slime { self.normalize = true; }
                                }
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
                                // 1.0 RenderFallingSand: Tessellator brightness (per vertex) is the
                                // block's packed light at its cell; the GL lightmap state is untouched.
                                let falling_light = if matches!(e.kind, worldgen::entity::EntityKind::FallingSand { .. }) { snapshot.tile_lights.first().copied() } else { None };
                                for d in &g.draws {
                                    let batch = d.batch();
                                    if let Some(l) = falling_light { self.vertex_lights.push((batches.len(), vec![l as u32; batch.vertices.len()])); }
                                    batches.push(batch);
                                }
                                if !snapshot.block_mesh.is_empty() {
                                    // The dragon egg: world-lit RenderBlocks quads under glTranslatef(relative).
                                    let p = super::bridge::position(e, self.partial);
                                    let rel: [f32; 3] = std::array::from_fn(|i| (p[i] - camera.to_array()[i]) as f32);
                                    let mut vertices = Vec::with_capacity(snapshot.block_mesh.len() / 4 * 6);
                                    let mut vertex_lights = Vec::with_capacity(vertices.capacity());
                                    for quad in snapshot.block_mesh.chunks_exact(4) {
                                        for i in [0, 1, 2, 0, 2, 3] {
                                            let v = quad[i];
                                            vertices.push(living::EntityVertex { pos: std::array::from_fn(|k| v.pos[k] + rel[k]), uv: v.uv, color: v.color.map(|c| c as f32 / 255.0) });
                                            vertex_lights.push(v.light);
                                        }
                                    }
                                    self.vertex_lights.push((batches.len(), vertex_lights));
                                    batches.push(Batch { alpha_cutoff: 0.1, cull: 0, texture: Some("terrain.png"), pass: Pass::Alpha, vertices });
                                }
                            }
                        }
                        super::manager::Event::ShadowAndFire(_, _) => {
                            if !matches!(e.kind, worldgen::entity::EntityKind::Lightning { .. }) {
                                if self.fancy {
                                    let g = super::effects::shadow_with_camera(e, &snapshot.surfaces, self.partial, camera.to_array(), distance_camera);
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
                        _ => {}
                    }
                    Ok(())
                },
            ).expect("registered simulation entity renderer");
            // RenderLiving turns the lightmap off for its hurt/flash overlay passes.
            let light = snapshot.light.unwrap_or(NO_LIGHTMAP);
            let trailing_light=explicit_lights.last().copied().unwrap_or(light);
            lights.extend(batches[first..].iter().enumerate().map(|(i,b)| {
                if matches!(b.pass, Pass::Overlay | Pass::EyesOverlay | Pass::Damage | Pass::TntFlash) { NO_LIGHTMAP } else { explicit_lights.get(i).copied().unwrap_or(trailing_light) }
            }));
        }
        lights.resize(batches.len(), NO_LIGHTMAP);
        let millis=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64;
        for position in &self.portals {
            let relative=std::array::from_fn(|i|position[i] as f64-camera[i]);
            let portal=crate::render::end_portal::render(relative,camera.to_array(),self.active_info.offset,[crate::render::end_portal::time_fraction(millis);16]);
            for (layer,draw) in portal.gl.draws.iter().enumerate() {
                self.projective_uvs.push((batches.len(),portal.projective_uv(layer,self.fog_view)));
                batches.push(draw.batch());lights.push(NO_LIGHTMAP);
            }
        }
        for tile in &self.books {
            let relative = std::array::from_fn(|i| tile.position[i] as f64 - camera.to_array()[i]);
            let g = super::render_release::book(tile.state, relative, self.partial);
            let start = batches.len();batches.extend(g.draws.iter().map(|d| d.batch()));
            lights.resize(batches.len(), tile.light);
            debug_assert!(batches.len() >= start);
        }
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
        let mut light_of: Vec<[u32; 2]> = Vec::new();
        let mut uvq=Vec::new();
        let mut projected:HashMap<usize,Vec<[f32;3]>>=std::mem::take(&mut self.projective_uvs).into_iter().collect();
        let mut per_vertex: std::collections::HashMap<usize, Vec<u32>> = std::mem::take(&mut self.vertex_lights).into_iter().collect();
        for (i, b) in batches.into_iter().enumerate() {
            let start = all.len() as u32;
            uvq.extend(projected.remove(&i).unwrap_or_else(||b.vertices.iter().map(|v|[v.uv[0],v.uv[1],1.0]).collect()));
            all.extend_from_slice(&b.vertices);
            match per_vertex.remove(&i) {
                Some(l) if l.len() == b.vertices.len() => light_of.extend(l.into_iter().map(|l| [l, b.alpha_cutoff.to_bits()])),
                _ => light_of.resize(all.len(), [batch_lights.get(i).copied().unwrap_or(NO_LIGHTMAP), b.alpha_cutoff.to_bits()]),
            }
            self.draws.push(Draw {
                cull: b.cull,
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
            self.uvq=device.create_buffer(&wgpu::BufferDescriptor {label:Some("projective UV"),size:(self.capacity*12) as u64,usage:wgpu::BufferUsages::VERTEX|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});
            self.lights = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("entity lights"),
                size: (self.capacity * 8) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }
        if !all.is_empty() {
            queue.write_buffer(&self.uvq,0,bytemuck::cast_slice(&uvq));
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
        rp.set_vertex_buffer(2,self.uvq.slice(..));
        for d in &self.draws {
            rp.set_pipeline((match d.pass {
                Pass::Line => &self.line,
                Pass::Opaque => &self.opaque,
                Pass::OpaqueEqual => &self.opaque_equal,
                Pass::Alpha => &self.alpha,
                Pass::Eyes => &self.eyes,
                Pass::EyesOverlay => &self.eyes_overlay,
                Pass::Additive => &self.additive,
                Pass::Rays => &self.rays,
                Pass::Overlay => &self.overlay,
                Pass::Shadow => &self.shadow,
                Pass::TntFlash => &self.tnt_flash,
                Pass::Damage => &self.damage,
                Pass::Untested => &self.untested,
                Pass::UntestedAlpha => &self.untested_alpha,
            }).get(mask, d.cull));
            rp.set_bind_group(1, &self.textures[&d.texture], &[]);
            rp.draw(d.range.clone(), 0..1);
        }
    }
}
