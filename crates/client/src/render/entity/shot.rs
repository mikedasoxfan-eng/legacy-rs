//! `beta --modelshot file.png`: deterministic offscreen showcase, no world/window.

use super::living::{self, Batch, EntityVertex, MobKind, Pass, RenderState};
use super::pipeline::EntityPipeline;
use crate::assets;
use crate::render::gpu::Image;
use glam::{DVec3, Mat4, Vec3};

pub const WIDTH: u32 = 2560;
pub const HEIGHT: u32 = 720;
pub const CAMERA: [f64; 3] = [0.0, 4.6, 13.5];
pub const PITCH: f32 = 14.0;
pub const FOV: f32 = 40.0;
pub const FAR: f32 = 128.0;
pub const FOG: [f32; 3] = [0.69, 0.79, 0.91];

pub fn states(variants: bool) -> Vec<RenderState> {
    use MobKind::*;
    let kinds = [Biped, Zombie, Skeleton, Pig, Cow, Sheep, Chicken, Spider, Creeper, Slime, Squid, Wolf];
    kinds
        .into_iter()
        .enumerate()
        .map(|(i, kind)| {
            let mut s = RenderState::new(kind);
            s.position[0] = (i as f64 - 5.5) * 2.45;
            s.yaw = -25.0;
            s.body_yaw = -25.0;
            s.age = 4.5;
            s.slime_size = 2;
            s.tentacle_angle = 0.25;
            if variants {
                s.partial = 0.5;
                match kind {
                    Biped => {
                        s.sneak = true;
                        s.attack = 0.35;
                        s.held_right = true;
                    }
                    Zombie => s.hurt_time = 5,
                    Skeleton => {
                        s.death_time = 8;
                    }
                    Pig => s.saddled = true,
                    Sheep => s.fleece_color = 14,
                    Chicken => {
                        s.chicken_flap = 1.25;
                        s.chicken_flap_amount = 0.6;
                    }
                    Spider => {
                        s.brightness = 0.1;
                        s.spider_eye_brightness = 0.1;
                    }
                    Creeper => {
                        s.powered = true;
                        s.creeper_flash = 0.75;
                    }
                    Slime => s.slime_squish = 0.4,
                    Squid => {
                        s.squid_pitch = 20.0;
                        s.squid_yaw = 15.0;
                        s.tentacle_angle = 0.8;
                    }
                    Wolf => {
                        s.wolf.sitting = true;
                        s.wolf_tamed = true;
                        s.wolf.interested = [0.5, 1.0];
                    }
                    _ => {}
                }
            }
            // A test floor has no entity collision boxes. Place the mesh's lowest
            // vertex just above it (including the squid's different origin).
            let min = living::render_living(&s, DVec3::ZERO)
                .iter()
                .flat_map(|b| &b.vertices)
                .map(|v| v.pos[1])
                .fold(f32::INFINITY, f32::min);
            s.position[1] = -min as f64 + 0.0078125;
            s
        })
        .collect()
}

pub fn batches(states: &[RenderState], camera: DVec3) -> Vec<Batch> {
    // Stone tiles on a flat lit floor, directly in the entity pipeline's format.
    let mut floor = Batch { texture: Some("terrain.png"), pass: Pass::Opaque, vertices: Vec::new() };
    for x in -20..20 {
        for z in -5..5 {
            let p = [
                [x as f32, 0.0, z as f32],
                [x as f32, 0.0, z as f32 + 1.0],
                [x as f32 + 1.0, 0.0, z as f32 + 1.0],
                [x as f32 + 1.0, 0.0, z as f32],
            ];
            let uv = [[1.0 / 16.0, 0.0], [1.0 / 16.0, 1.0 / 16.0], [2.0 / 16.0, 1.0 / 16.0], [2.0 / 16.0, 0.0]];
            let color = living::lit_color(Vec3::Y, [0.75, 0.75, 0.75, 1.0]);
            let v: [_; 4] = std::array::from_fn(|i| EntityVertex {
                pos: (Vec3::from_array(p[i]) - camera.as_vec3()).to_array(),
                uv: uv[i],
                color,
            });
            for i in [0, 1, 2, 0, 2, 3] {
                floor.vertices.push(v[i]);
            }
        }
    }
    let mut result = vec![floor];
    result.extend(living::scene(states, camera));
    result
}

pub fn view_proj() -> Mat4 {
    crate::render::gpu::perspective(FOV.to_radians(), WIDTH as f32 / HEIGHT as f32, 0.05, FAR * 2.0)
        * Mat4::from_rotation_x(PITCH.to_radians())
        * Mat4::from_rotation_y(360f32.to_radians())
}

pub fn model_shot(path: &std::path::Path, variants: bool, scene_path: Option<&std::path::Path>) -> Result<(), String> {
    let states = states(variants);
    if let Some(path) = scene_path {
        let mut lines = format!("{}\n", variants as u8);
        for s in &states {
            lines.push_str(&format!("{} {} {}\n", s.position[0], s.position[1], s.position[2]));
        }
        std::fs::write(path, lines).map_err(|e| e.to_string())?;
    }
    save_batches(path, batches(&states, DVec3::from_array(CAMERA)), view_proj())
}

pub fn renderer_shot(path: &std::path::Path) -> Result<(), String> {
    use super::objects::{ObjectKind, ObjectState};
    let camera = DVec3::new(0.0, 13.0, 34.0);
    let mut all = batches(&[], camera);
    let kinds = [
        ObjectKind::Item { item: worldgen::world::tile::ItemStack::new(1, 21, 0), age: 77, hover: 0.4 },
        ObjectKind::Item { item: worldgen::world::tile::ItemStack::new(18, 6, 1), age: 22, hover: 1.2 },
        ObjectKind::Item { item: worldgen::world::tile::ItemStack::new(264, 21, 0), age: 11, hover: 2.0 },
        ObjectKind::Falling { block: 12, light: [0.8; 7] },
        ObjectKind::Tnt { fuse: 9 },
        ObjectKind::Arrow { yaw: 35.0, pitch: -25.0, shake: 3, visible: true },
        ObjectKind::Sprite { icon: 14, scale: 2.0 },
    ];
    for (i, kind) in kinds.into_iter().enumerate() {
        let s = ObjectState { kind, position: [-18.0 + i as f64 * 2.5, 0.8, 0.0], partial: 0.5, brightness: 1.0 };
        all.extend(super::objects::render(&s, camera.to_array(), [0.0, PITCH]).draws.iter().map(|d| d.batch()));
    }
    let mut world = worldgen::World::new(0, 1);
    for (i, kind) in [MobKind::Ghast, MobKind::PigZombie, MobKind::Giant].into_iter().enumerate() {
        let mut s = RenderState::new(kind);
        s.position = [3.0 + i as f64 * 6.0, 0.0, 0.0];
        s.age = 4.5;
        s.partial = 0.5;
        s.body_yaw = -25.0;
        s.yaw = -25.0;
        s.ghast_attack = 15.0;
        s.ghast_attacking = true;
        let min = living::render_living(&s, DVec3::ZERO)
            .iter()
            .flat_map(|b| &b.vertices)
            .map(|v| v.pos[1])
            .fold(f32::INFINITY, f32::min);
        s.position[1] = -min as f64 + 0.015625;
        all.extend(living::render_living(&s, camera));
        let mut e = worldgen::entity::Entity::item(
            &mut world,
            s.position[0],
            s.position[1],
            0.0,
            worldgen::world::tile::ItemStack::new(1, 1, 0),
        );
        e.last_tick_pos = e.pos;
        e.fire = 10;
        e.width = if kind == MobKind::Giant { 3.6 } else { 0.6 };
        e.height = if kind == MobKind::Giant { 10.8 } else { 1.8 };
        e.bounding_box.min_y = e.pos.y;
        if kind == MobKind::PigZombie {
            all.extend(super::effects::fire(&e, 0.5, camera.to_array(), 0.0).draws.iter().map(|d| d.batch()));
        }
    }
    let view = crate::render::gpu::perspective(FOV.to_radians(), WIDTH as f32 / HEIGHT as f32, 0.05, FAR * 2.0)
        * Mat4::from_rotation_x(PITCH.to_radians());
    save_batches(path, all, view)
}

fn save_batches(path: &std::path::Path, batches: Vec<Batch>, view_proj: Mat4) -> Result<(), String> {
    let a = assets::load()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
        apply_limit_buckets: false,
    }))
    .map_err(|e| format!("showcase GPU: {e}"))?;
    let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default()))
        .map_err(|e| format!("showcase device: {e}"))?;
    let mut renderer = EntityPipeline::new(
        &device,
        &queue,
        wgpu::TextureFormat::Rgba8Unorm,
        a.mobs.iter().map(|(n, i)| (n.as_str(), i)).chain(std::iter::once(("terrain.png", &a.terrain))),
    );
    let mut fx = crate::render::fx::TextureFx::terrain_set();
    let mut rng = crate::render::fx::Rng::new(187);
    for _ in 0..64 {
        for f in &mut fx {
            f.tick(&mut rng);
        }
    }
    renderer.update_fx(&queue, &fx);
    renderer.upload(&device, &queue, batches, view_proj, FOG, FAR);
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("modelshot"),
        size: wgpu::Extent3d { width: WIDTH, height: HEIGHT, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let depth = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("modelshot depth"),
        size: wgpu::Extent3d { width: WIDTH, height: HEIGHT, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let view = texture.create_view(&Default::default());
    let depth_view = depth.create_view(&Default::default());
    let mut encoder = device.create_command_encoder(&Default::default());
    {
        let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("model showcase"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: FOG[0] as f64,
                        g: FOG[1] as f64,
                        b: FOG[2] as f64,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth_view,
                depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        renderer.draw(&mut rp);
    }
    let bpr = (WIDTH * 4).div_ceil(256) * 256;
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("modelshot pixels"),
        size: (bpr * HEIGHT) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: Default::default(),
            aspect: Default::default(),
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &buffer,
            layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(bpr), rows_per_image: Some(HEIGHT) },
        },
        wgpu::Extent3d { width: WIDTH, height: HEIGHT, depth_or_array_layers: 1 },
    );
    queue.submit([encoder.finish()]);
    let slice = buffer.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |r| {
        let _ = tx.send(r);
    });
    device.poll(wgpu::PollType::wait_indefinitely()).map_err(|e| e.to_string())?;
    rx.recv().map_err(|e| e.to_string())?.map_err(|e| e.to_string())?;
    let data = slice.get_mapped_range().map_err(|e| e.to_string())?;
    let mut rgba = Vec::with_capacity((WIDTH * HEIGHT * 4) as usize);
    for y in 0..HEIGHT {
        rgba.extend_from_slice(&data[(y * bpr) as usize..(y * bpr + WIDTH * 4) as usize]);
    }
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    assets::save_png(&Image { width: WIDTH, height: HEIGHT, rgba }, path).map_err(|e| e.to_string())?;
    eprintln!("saved {}", path.display());
    Ok(())
}
