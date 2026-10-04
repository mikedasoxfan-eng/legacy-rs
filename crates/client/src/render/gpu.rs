//! wgpu device, pipelines and per-frame drawing.

use super::{sky, Vertex};
use glam::{DMat4, DVec3, Mat4, Vec4};
use std::collections::HashMap;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::window::Window;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    view_proj: [[f32; 4]; 4],
    model: [[f32; 4]; 4],
    color: [f32; 4],
    fog_color: [f32; 4],
    fog: [f32; 4],
}

/// One textured image uploaded to the GPU (terrain atlas, sun, moon, or plain white).
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

struct ChunkGpu {
    opaque: Option<(wgpu::Buffer, u32)>,
    translucent: Option<(wgpu::Buffer, u32)>,
}

/// Everything the frame needs from the game state.
pub struct FrameParams {
    pub cam_pos: DVec3,
    pub yaw: f32,
    pub pitch: f32,
    pub fov_y_deg: f32,
    pub celestial: f32,
    pub sky_color: [f32; 3],
    pub fog_color: [f32; 3],
    pub far: f32,
    /// cloudOffsetX + partialTicks
    pub cloud_offset: f32,
}

struct DrawSlot {
    buf: wgpu::Buffer,
    bind: wgpu::BindGroup,
}

pub struct Gpu {
    pub window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    depth: wgpu::TextureView,
    origins_bind: wgpu::BindGroup,
    origins_buf: wgpu::Buffer,
    origins_cap: usize,
    p_opaque: wgpu::RenderPipeline,
    p_water_depth: wgpu::RenderPipeline,
    p_water_color: wgpu::RenderPipeline,
    p_sky: wgpu::RenderPipeline,
    p_sky_alpha: wgpu::RenderPipeline,
    p_sky_add: wgpu::RenderPipeline,
    p_cloud_depth: wgpu::RenderPipeline,
    p_cloud_color: wgpu::RenderPipeline,
    cloud_slot: DrawSlot,
    cloud_buf: wgpu::Buffer,
    terrain_tex: wgpu::Texture,
    quad_index: wgpu::Buffer,
    quad_index_quads: u32,
    chunks: HashMap<(i32, i32), ChunkGpu>,
    sky_plane: (wgpu::Buffer, u32),
    void_plane: (wgpu::Buffer, u32),
    stars: (wgpu::Buffer, u32),
    sun_quad: (wgpu::Buffer, u32),
    moon_quad: (wgpu::Buffer, u32),
    fan: wgpu::Buffer,
    slots: Vec<DrawSlot>,
    terrain_slot: DrawSlot,
    water_slot: DrawSlot,
    sun_slot: DrawSlot,
    moon_slot: DrawSlot,
}

const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

fn tri_list(verts: &[[f32; 3]], color: [u8; 4]) -> Vec<Vertex> {
    verts.iter().map(|p| Vertex { pos: *p, uv: [0.0, 0.0], color }).collect()
}

impl Gpu {
    pub fn new(window: Arc<Window>, terrain: &Image, sun: &Image, moon: &Image, clouds: &Image) -> Gpu {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
        let surface = instance.create_surface(window.clone()).expect("create surface");
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
            apply_limit_buckets: false,
        }))
        .expect("no GPU adapter");
        let (device, queue) =
            pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).expect("request device");
        let mut config = surface
            .get_default_config(&adapter, size.width.max(1), size.height.max(1))
            .expect("surface not supported");
        // The original wrote colors straight to the framebuffer: no sRGB encoding.
        config.format = config.format.remove_srgb_suffix();
        config.usage |= wgpu::TextureUsages::COPY_SRC;
        config.present_mode = wgpu::PresentMode::AutoVsync;
        surface.configure(&device, &config);
        let depth = Self::make_depth(&device, config.width, config.height);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });
        let layout0 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("layout0"),
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
        let layout1 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("layout1"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let terrain_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("terrain layout"),
            bind_group_layouts: &[Some(&layout0), Some(&layout1)],
            immediate_size: 0,
        });
        let sky_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("sky layout"),
            bind_group_layouts: &[Some(&layout0)],
            immediate_size: 0,
        });

        let vertex_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2, 2 => Unorm8x4],
        };
        let alpha_blend = wgpu::BlendState::ALPHA_BLENDING;
        let add_blend = wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent::OVER,
        };
        let make = |label: &str,
                    layout: &wgpu::PipelineLayout,
                    vs: &str,
                    blend: Option<wgpu::BlendState>,
                    write_mask: wgpu::ColorWrites,
                    depth_write: bool,
                    depth_cmp: wgpu::CompareFunction,
                    cull: Option<wgpu::Face>| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(label),
                layout: Some(layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some(vs),
                    compilation_options: Default::default(),
                    buffers: &[Some(vertex_layout.clone())],
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: cull,
                    ..Default::default()
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: DEPTH_FORMAT,
                    depth_write_enabled: Some(depth_write),
                    depth_compare: Some(depth_cmp),
                    stencil: Default::default(),
                    bias: Default::default(),
                }),
                multisample: Default::default(),
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState { format: config.format, blend, write_mask })],
                }),
                multiview_mask: None,
                cache: None,
            })
        };
        use wgpu::CompareFunction::{Always, LessEqual};
        let all = wgpu::ColorWrites::ALL;
        let p_opaque = make("opaque", &terrain_layout, "vs_terrain", None, all, true, LessEqual, Some(wgpu::Face::Back));
        let p_water_depth = make(
            "water depth",
            &terrain_layout,
            "vs_terrain",
            Some(alpha_blend),
            wgpu::ColorWrites::empty(),
            true,
            LessEqual,
            None,
        );
        let p_water_color = make("water color", &terrain_layout, "vs_terrain", Some(alpha_blend), all, true, LessEqual, None);
        let p_sky = make("sky", &sky_layout, "vs_sky", None, all, false, Always, None);
        let p_sky_alpha = make("sky alpha", &sky_layout, "vs_sky", Some(alpha_blend), all, false, Always, None);
        let p_sky_add = make("sky add", &sky_layout, "vs_sky", Some(add_blend), all, false, Always, None);
        let p_cloud_depth =
            make("cloud depth", &sky_layout, "vs_sky", Some(alpha_blend), wgpu::ColorWrites::empty(), true, LessEqual, None);
        let p_cloud_color = make("cloud color", &sky_layout, "vs_sky", Some(alpha_blend), all, true, LessEqual, None);

        let upload = |img: &Image, label: &str| -> (wgpu::Texture, wgpu::TextureView) {
            let tex = device.create_texture_with_data(
                &queue,
                &wgpu::TextureDescriptor {
                    label: Some(label),
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
            (tex, view)
        };
        let (terrain_tex, terrain_view) = upload(terrain, "terrain.png");
        let (_, sun_view) = upload(sun, "sun.png");
        let (_, moon_view) = upload(moon, "moon.png");
        let (_, clouds_view) = upload(clouds, "clouds.png");
        let (_, white_view) = upload(&Image { width: 1, height: 1, rgba: vec![255; 4] }, "white");
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("nearest"),
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let origins_cap = 4096;
        let origins_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("origins"),
            size: (origins_cap * 16) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let origins_bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("origins"),
            layout: &layout1,
            entries: &[wgpu::BindGroupEntry { binding: 0, resource: origins_buf.as_entire_binding() }],
        });

        let vbuf = |data: &[Vertex], label: &str| -> (wgpu::Buffer, u32) {
            let b = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(label),
                contents: bytemuck::cast_slice(data),
                usage: wgpu::BufferUsages::VERTEX,
            });
            (b, data.len() as u32)
        };
        let white = [255u8; 4];
        let sky_plane = vbuf(&tri_list(&sky::sky_plane(16.0, true), white), "sky plane");
        let void_plane = vbuf(&tri_list(&sky::sky_plane(-16.0, false), white), "void plane");
        let stars = vbuf(&tri_list(&sky::stars(), white), "stars");
        let quad = |y: f32, s: f32, uvs: [[f32; 2]; 4], pts: [[f32; 3]; 4]| -> Vec<Vertex> {
            let v: Vec<Vertex> = (0..4).map(|i| Vertex { pos: pts[i], uv: uvs[i], color: white }).collect();
            let _ = (y, s);
            vec![v[0], v[1], v[2], v[0], v[2], v[3]]
        };
        let s = 30.0;
        let sun_quad = vbuf(
            &quad(100.0, s, [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]], [
                [-s, 100.0, -s],
                [s, 100.0, -s],
                [s, 100.0, s],
                [-s, 100.0, s],
            ]),
            "sun",
        );
        let s = 20.0;
        let moon_quad = vbuf(
            &quad(-100.0, s, [[1.0, 1.0], [0.0, 1.0], [0.0, 0.0], [1.0, 0.0]], [
                [-s, -100.0, s],
                [s, -100.0, s],
                [s, -100.0, -s],
                [-s, -100.0, -s],
            ]),
            "moon",
        );
        let fan = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("sunrise fan"),
            size: (16 * 3 * std::mem::size_of::<Vertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let slot = |view: &wgpu::TextureView| make_slot(&device, &layout0, &sampler, view);
        let terrain_slot = slot(&terrain_view);
        let water_slot = slot(&terrain_view);
        let sun_slot = slot(&sun_view);
        let moon_slot = slot(&moon_view);
        let cloud_slot = slot(&clouds_view);
        let cloud_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("clouds"),
            size: (8192 * std::mem::size_of::<Vertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let slots = (0..4).map(|_| slot(&white_view)).collect();
        let quad_index = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("quad index"),
            size: 4,
            usage: wgpu::BufferUsages::INDEX,
            mapped_at_creation: false,
        });

        let mut gpu = Gpu {
            window,
            surface,
            depth,
            origins_bind,
            origins_buf,
            origins_cap,
            p_opaque,
            p_water_depth,
            p_water_color,
            p_sky,
            p_sky_alpha,
            p_sky_add,
            p_cloud_depth,
            p_cloud_color,
            cloud_slot,
            cloud_buf,
            terrain_tex,
            quad_index,
            quad_index_quads: 0,
            chunks: HashMap::new(),
            sky_plane,
            void_plane,
            stars,
            sun_quad,
            moon_quad,
            fan,
            slots,
            terrain_slot,
            water_slot,
            sun_slot,
            moon_slot,
            device,
            queue,
            config,
        };
        gpu.ensure_quad_indices(1 << 16);
        gpu
    }

    fn make_depth(device: &wgpu::Device, w: u32, h: u32) -> wgpu::TextureView {
        device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("depth"),
                size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: DEPTH_FORMAT,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            })
            .create_view(&Default::default())
    }

    fn ensure_quad_indices(&mut self, quads: u32) {
        if quads <= self.quad_index_quads {
            return;
        }
        let quads = quads.next_power_of_two();
        let mut idx = Vec::with_capacity(quads as usize * 6);
        for q in 0..quads {
            let b = q * 4;
            idx.extend_from_slice(&[b, b + 1, b + 2, b, b + 2, b + 3]);
        }
        self.quad_index = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("quad index"),
            contents: bytemuck::cast_slice(&idx),
            usage: wgpu::BufferUsages::INDEX,
        });
        self.quad_index_quads = quads;
    }

    /// RenderEngine.updateDynamicTextures: copy animated tiles into terrain.png.
    pub fn update_fx(&self, fx: &[super::fx::TextureFx]) {
        for f in fx {
            for i in 0..f.tile_size {
                for j in 0..f.tile_size {
                    self.queue.write_texture(
                        wgpu::TexelCopyTextureInfo {
                            texture: &self.terrain_tex,
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

    pub fn resize(&mut self, w: u32, h: u32) {
        if w == 0 || h == 0 {
            return;
        }
        self.config.width = w;
        self.config.height = h;
        self.surface.configure(&self.device, &self.config);
        self.depth = Self::make_depth(&self.device, w, h);
    }

    pub fn upload_chunk(&mut self, cx: i32, cz: i32, opaque: &[Vertex], translucent: &[Vertex]) {
        let quads = (opaque.len().max(translucent.len()) / 4) as u32;
        self.ensure_quad_indices(quads);
        let mk = |d: &[Vertex]| {
            if d.is_empty() {
                None
            } else {
                let b = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("chunk"),
                    contents: bytemuck::cast_slice(d),
                    usage: wgpu::BufferUsages::VERTEX,
                });
                Some((b, (d.len() / 4 * 6) as u32))
            }
        };
        let c = ChunkGpu { opaque: mk(opaque), translucent: mk(translucent) };
        self.chunks.insert((cx, cz), c);
    }

    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }

    fn write_uniforms(&self, slot: &DrawSlot, u: &Uniforms) {
        self.queue.write_buffer(&slot.buf, 0, bytemuck::bytes_of(u));
    }

    /// Render one frame to the window (or, with `capture`, also return the pixels).
    pub fn render(&mut self, p: &FrameParams, capture: bool) -> Option<Image> {
        let frame = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(f) | wgpu::CurrentSurfaceTexture::Suboptimal(f) => f,
            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                self.surface.configure(&self.device, &self.config);
                return None;
            }
            _ => return None,
        };
        let view = frame.texture.create_view(&Default::default());
        let (w, h) = (self.config.width, self.config.height);

        // Camera: glRotatef(pitch, 1,0,0); glRotatef(yaw + 180, 0,1,0); gluPerspective(fov, aspect, 0.05, far*2).
        let rot = Mat4::from_rotation_x(p.pitch.to_radians()) * Mat4::from_rotation_y((p.yaw + 180.0).to_radians());
        let proj = perspective(p.fov_y_deg.to_radians(), w as f32 / h as f32, 0.05, p.far * 2.0);
        let view_proj = proj * rot;
        let ident = Mat4::IDENTITY.to_cols_array_2d();
        let fog4 = [p.fog_color[0], p.fog_color[1], p.fog_color[2], 1.0];

        // ---- sky (RenderGlobal.renderSky)
        let sky_fog = [0.0, p.far * 0.8, 1.0, -1.0];
        let sky_u = Uniforms {
            view_proj: view_proj.to_cols_array_2d(),
            model: ident,
            color: [p.sky_color[0], p.sky_color[1], p.sky_color[2], 1.0],
            fog_color: fog4,
            fog: sky_fog,
        };
        self.write_uniforms(&self.slots[0], &sky_u);
        let sunrise = sky::sunrise_colors(p.celestial);
        if let Some(c) = sunrise {
            // Triangle fan: center at (0,100,0), rim of 17 points; rotated 90 about X then 0/180 about Z.
            let center = Vertex { pos: [0.0, 100.0, 0.0], uv: [0.0; 2], color: rgba(c[0], c[1], c[2], c[3]) };
            let mut rim = Vec::new();
            for i in 0..=16 {
                let a = i as f32 * std::f64::consts::PI as f32 * 2.0 / 16.0;
                let (s, co) = (worldgen::math::sin(a), worldgen::math::cos(a));
                rim.push(Vertex { pos: [s * 120.0, co * 120.0, -co * 40.0 * c[3]], uv: [0.0; 2], color: rgba(c[0], c[1], c[2], 0.0) });
            }
            let mut fan = Vec::new();
            for i in 0..16 {
                fan.extend_from_slice(&[center, rim[i], rim[i + 1]]);
            }
            self.queue.write_buffer(&self.fan, 0, bytemuck::cast_slice(&fan));
            let z = if p.celestial > 0.5 { 180.0f32 } else { 0.0 };
            let model = Mat4::from_rotation_x(90f32.to_radians()) * Mat4::from_rotation_z(z.to_radians());
            let u = Uniforms { view_proj: view_proj.to_cols_array_2d(), model: model.to_cols_array_2d(), color: [1.0; 4], fog_color: fog4, fog: [0.0, 1.0, 0.0, -1.0] };
            self.write_uniforms(&self.slots[1], &u);
        }
        let celestial_rot = Mat4::from_rotation_x((p.celestial * 360.0).to_radians());
        let sunmoon_u = Uniforms {
            view_proj: view_proj.to_cols_array_2d(),
            model: celestial_rot.to_cols_array_2d(),
            color: [1.0, 1.0, 1.0, 1.0],
            fog_color: fog4,
            fog: [0.0, 1.0, 0.0, -1.0],
        };
        let star_b = sky::star_brightness(p.celestial);
        let stars_u = Uniforms { color: [star_b; 4], ..sunmoon_u };
        let void_u = Uniforms {
            view_proj: view_proj.to_cols_array_2d(),
            model: ident,
            color: [p.sky_color[0] * 0.2 + 0.04, p.sky_color[1] * 0.2 + 0.04, p.sky_color[2] * 0.6 + 0.1, 1.0],
            fog_color: fog4,
            fog: sky_fog,
        };
        self.write_uniforms(&self.slots[2], &stars_u);
        self.write_uniforms(&self.slots[3], &void_u);
        self.write_uniforms(&self.sun_slot, &sunmoon_u);
        self.write_uniforms(&self.moon_slot, &sunmoon_u);

        // ---- terrain: chunk origins relative to the camera, frustum-culled
        let cam = p.cam_pos;
        let mut visible: Vec<(i32, i32)> = Vec::new();
        let vp64 = DMat4::from_cols_array(&view_proj.to_cols_array().map(|v| v as f64));
        for &(cx, cz) in self.chunks.keys() {
            let min = DVec3::new(cx as f64 * 16.0, 0.0, cz as f64 * 16.0) - cam;
            if aabb_visible(&vp64, min, min + DVec3::new(16.0, 128.0, 16.0)) {
                visible.push((cx, cz));
            }
        }
        visible.sort_by_key(|&(cx, cz)| {
            let dx = cx as f64 * 16.0 + 8.0 - cam.x;
            let dz = cz as f64 * 16.0 + 8.0 - cam.z;
            (dx * dx + dz * dz) as i64
        });
        if visible.len() > self.origins_cap {
            self.origins_cap = visible.len().next_power_of_two();
            self.origins_buf = self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("origins"),
                size: (self.origins_cap * 16) as u64,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            let layout1 = self.p_opaque.get_bind_group_layout(1);
            self.origins_bind = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("origins"),
                layout: &layout1,
                entries: &[wgpu::BindGroupEntry { binding: 0, resource: self.origins_buf.as_entire_binding() }],
            });
        }
        let origins: Vec<[f32; 4]> = visible
            .iter()
            .map(|&(cx, cz)| {
                let o = DVec3::new(cx as f64 * 16.0, 0.0, cz as f64 * 16.0) - cam;
                [o.x as f32, o.y as f32, o.z as f32, 0.0]
            })
            .collect();
        if !origins.is_empty() {
            self.queue.write_buffer(&self.origins_buf, 0, bytemuck::cast_slice(&origins));
        }
        let terrain_fog = [p.far * 0.25, p.far, 1.0, 0.1];
        let tu = Uniforms { view_proj: view_proj.to_cols_array_2d(), model: ident, color: [1.0; 4], fog_color: fog4, fog: terrain_fog };
        self.write_uniforms(&self.terrain_slot, &tu);
        self.write_uniforms(&self.water_slot, &tu);

        // Clouds (Fancy): rebuilt every frame around the camera, drawn scaled 12x horizontally.
        let cloud_verts = sky::fancy_clouds(cam.x, cam.y, cam.z, p.cloud_offset, sky::cloud_color(p.celestial));
        let cloud_n = cloud_verts.len().min(8192);
        self.queue.write_buffer(&self.cloud_buf, 0, bytemuck::cast_slice(&cloud_verts[..cloud_n]));
        let cu = Uniforms {
            view_proj: view_proj.to_cols_array_2d(),
            model: Mat4::from_scale(glam::Vec3::new(12.0, 1.0, 12.0)).to_cols_array_2d(),
            color: [1.0; 4],
            fog_color: fog4,
            fog: [p.far * 0.25, p.far, 1.0, 0.1],
        };
        self.write_uniforms(&self.cloud_slot, &cu);

        // Offscreen copy target for screenshots.
        let capture_tex = capture.then(|| {
            self.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("capture"),
                size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: self.config.format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
                view_formats: &[],
            })
        });
        let capture_view = capture_tex.as_ref().map(|t| t.create_view(&Default::default()));
        let target = capture_view.as_ref().unwrap_or(&view);

        let mut enc = self.device.create_command_encoder(&Default::default());
        {
            let mut rp = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("main"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: p.fog_color[0] as f64,
                            g: p.fog_color[1] as f64,
                            b: p.fog_color[2] as f64,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth,
                    depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            // Sky plane, sunrise glow, sun, moon, stars, void.
            rp.set_pipeline(&self.p_sky);
            rp.set_bind_group(0, &self.slots[0].bind, &[]);
            rp.set_vertex_buffer(0, self.sky_plane.0.slice(..));
            rp.draw(0..self.sky_plane.1, 0..1);
            if sunrise.is_some() {
                rp.set_pipeline(&self.p_sky_alpha);
                rp.set_bind_group(0, &self.slots[1].bind, &[]);
                rp.set_vertex_buffer(0, self.fan.slice(..));
                rp.draw(0..48, 0..1);
            }
            rp.set_pipeline(&self.p_sky_add);
            rp.set_bind_group(0, &self.sun_slot.bind, &[]);
            rp.set_vertex_buffer(0, self.sun_quad.0.slice(..));
            rp.draw(0..self.sun_quad.1, 0..1);
            rp.set_bind_group(0, &self.moon_slot.bind, &[]);
            rp.set_vertex_buffer(0, self.moon_quad.0.slice(..));
            rp.draw(0..self.moon_quad.1, 0..1);
            if star_b > 0.0 {
                rp.set_bind_group(0, &self.slots[2].bind, &[]);
                rp.set_vertex_buffer(0, self.stars.0.slice(..));
                rp.draw(0..self.stars.1, 0..1);
            }
            rp.set_pipeline(&self.p_sky);
            rp.set_bind_group(0, &self.slots[3].bind, &[]);
            rp.set_vertex_buffer(0, self.void_plane.0.slice(..));
            rp.draw(0..self.void_plane.1, 0..1);

            // Terrain pass 0.
            rp.set_index_buffer(self.quad_index.slice(..), wgpu::IndexFormat::Uint32);
            rp.set_pipeline(&self.p_opaque);
            rp.set_bind_group(0, &self.terrain_slot.bind, &[]);
            rp.set_bind_group(1, &self.origins_bind, &[]);
            for (i, k) in visible.iter().enumerate() {
                if let Some((buf, n)) = &self.chunks[k].opaque {
                    rp.set_vertex_buffer(0, buf.slice(..));
                    rp.draw_indexed(0..*n, 0, i as u32..i as u32 + 1);
                }
            }
            // Pass 1 (water, ice), Fancy: depth-only first, then color with LEQUAL.
            for pipe in [&self.p_water_depth, &self.p_water_color] {
                rp.set_pipeline(pipe);
                rp.set_bind_group(0, &self.water_slot.bind, &[]);
                for (i, k) in visible.iter().enumerate().rev() {
                    if let Some((buf, n)) = &self.chunks[k].translucent {
                        rp.set_vertex_buffer(0, buf.slice(..));
                        rp.draw_indexed(0..*n, 0, i as u32..i as u32 + 1);
                    }
                }
            }
            for pipe in [&self.p_cloud_depth, &self.p_cloud_color] {
                rp.set_pipeline(pipe);
                rp.set_bind_group(0, &self.cloud_slot.bind, &[]);
                rp.set_vertex_buffer(0, self.cloud_buf.slice(..));
                rp.draw_indexed(0..(cloud_n / 4 * 6) as u32, 0, 0..1);
            }
        }

        let mut out = None;
        if let Some(tex) = &capture_tex {
            let bpr = (w * 4).div_ceil(256) * 256;
            let buf = self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("readback"),
                size: (bpr * h) as u64,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            });
            enc.copy_texture_to_buffer(
                wgpu::TexelCopyTextureInfo { texture: tex, mip_level: 0, origin: Default::default(), aspect: Default::default() },
                wgpu::TexelCopyBufferInfo {
                    buffer: &buf,
                    layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(bpr), rows_per_image: Some(h) },
                },
                wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
            );
            self.queue.submit([enc.finish()]);
            let slice = buf.slice(..);
            slice.map_async(wgpu::MapMode::Read, |_| {});
            let _ = self.device.poll(wgpu::PollType::wait_indefinitely());
            let data = slice.get_mapped_range().expect("map readback");
            let bgra = matches!(self.config.format, wgpu::TextureFormat::Bgra8Unorm | wgpu::TextureFormat::Bgra8UnormSrgb);
            let mut rgba = Vec::with_capacity((w * h * 4) as usize);
            for y in 0..h {
                let row = &data[(y * bpr) as usize..(y * bpr + w * 4) as usize];
                for px in row.chunks(4) {
                    if bgra {
                        rgba.extend_from_slice(&[px[2], px[1], px[0], 255]);
                    } else {
                        rgba.extend_from_slice(&[px[0], px[1], px[2], 255]);
                    }
                }
            }
            out = Some(Image { width: w, height: h, rgba });
        } else {
            self.queue.submit([enc.finish()]);
        }
        self.queue.present(frame);
        out
    }
}

fn make_slot(device: &wgpu::Device, layout0: &wgpu::BindGroupLayout, sampler: &wgpu::Sampler, view: &wgpu::TextureView) -> DrawSlot {
    let buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("uniforms"),
        size: std::mem::size_of::<Uniforms>() as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("slot"),
        layout: layout0,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(view) },
            wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(sampler) },
        ],
    });
    DrawSlot { buf, bind }
}

fn rgba(r: f32, g: f32, b: f32, a: f32) -> [u8; 4] {
    let c = |v: f32| ((v * 255.0f32) as i32).clamp(0, 255) as u8;
    [c(r), c(g), c(b), c(a)]
}

/// Conservative frustum test of a camera-relative box.
fn aabb_visible(vp: &DMat4, min: DVec3, max: DVec3) -> bool {
    let rows = [vp.row(0), vp.row(1), vp.row(2), vp.row(3)];
    let planes = [rows[3] + rows[0], rows[3] - rows[0], rows[3] + rows[1], rows[3] - rows[1], rows[3] - rows[2]];
    for pl in planes {
        let px = if pl.x >= 0.0 { max.x } else { min.x };
        let py = if pl.y >= 0.0 { max.y } else { min.y };
        let pz = if pl.z >= 0.0 { max.z } else { min.z };
        if pl.x * px + pl.y * py + pl.z * pz + pl.w < 0.0 {
            return false;
        }
    }
    true
}

/// Right-handed perspective with a 0..1 depth range (gluPerspective's frustum).
fn perspective(fov_y: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    let f = 1.0 / (fov_y / 2.0).tan();
    Mat4::from_cols(
        Vec4::new(f / aspect, 0.0, 0.0, 0.0),
        Vec4::new(0.0, f, 0.0, 0.0),
        Vec4::new(0.0, 0.0, far / (near - far), -1.0),
        Vec4::new(0.0, 0.0, near * far / (near - far), 0.0),
    )
}
