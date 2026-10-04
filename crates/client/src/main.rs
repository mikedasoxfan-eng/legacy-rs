//! Minecraft Beta 1.7.3 in Rust — world viewer (fly camera) on the bit-exact generator.
//!
//! Controls: click to capture the mouse, WASD to fly, Space/Shift up/down, Ctrl faster,
//! T skips 1000 ticks of time, F2 saves a screenshot, Esc releases the mouse.
//!
//! Options: --seed <number or text>  --world-rand <n>  --time <ticks>
//!          --at x,y,z,yaw,pitch  --shot <file.png> (render once everything is loaded, then exit)

mod assets;
mod render;
mod worker;

use glam::DVec3;
use render::blocks::Colorizers;
use render::gpu::{FrameParams, Gpu};
use render::sky;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{CursorGrabMode, Window, WindowId};
use worker::{FromWorker, ToWorker};
use worldgen::BiomeSource;

/// Java's `String.hashCode`, which the world-creation screen uses for non-numeric seeds.
fn java_string_hash(s: &str) -> i32 {
    s.encode_utf16().fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(c as i32))
}

struct Options {
    seed: i64,
    world_rand: i64,
    time: i64,
    at: Option<(f64, f64, f64, f32, f32)>,
    shot: Option<std::path::PathBuf>,
}

fn parse_args() -> Options {
    let args: Vec<String> = std::env::args().collect();
    let get = |name: &str| args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).cloned();
    let seed = match get("--seed") {
        Some(s) => s.parse::<i64>().unwrap_or_else(|_| java_string_hash(&s) as i64),
        None => {
            // GuiCreateWorld with an empty seed: new Random().nextLong()
            let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as i64;
            worldgen::JavaRandom::new(nanos).next_long()
        }
    };
    let at = get("--at").map(|s| {
        let v: Vec<f64> = s.split(',').map(|p| p.trim().parse().expect("--at x,y,z,yaw,pitch")).collect();
        (v[0], v[1], v[2], v[3] as f32, v[4] as f32)
    });
    Options {
        seed,
        world_rand: get("--world-rand").and_then(|s| s.parse().ok()).unwrap_or(seed ^ 0x5DEECE66D),
        time: get("--time").and_then(|s| s.parse().ok()).unwrap_or(0),
        at,
        shot: get("--shot").map(Into::into),
    }
}

struct Camera {
    pos: DVec3,
    yaw: f32,
    pitch: f32,
}

struct App {
    opts: Options,
    gpu: Option<Gpu>,
    tx: Option<std::sync::mpsc::Sender<ToWorker>>,
    rx: Option<std::sync::mpsc::Receiver<FromWorker>>,
    cam: Camera,
    ready: bool,
    idle: bool,
    keys: HashSet<KeyCode>,
    grabbed: bool,
    world_time: i64,
    tick_accum: f64,
    last_frame: Instant,
    last_sent: (i32, i32, i32),
    temps: BiomeSource,
    fps_frames: u32,
    fps_time: Instant,
    fps: u32,
    loaded: usize,
    fx: Vec<render::fx::TextureFx>,
    fx_rng: render::fx::Rng,
    cloud_ticks: i64,
}

impl App {
    fn new(opts: Options) -> App {
        let seed = opts.seed;
        let time = opts.time;
        App {
            opts,
            gpu: None,
            tx: None,
            rx: None,
            cam: Camera { pos: DVec3::new(0.5, 80.0, 0.5), yaw: 0.0, pitch: 0.0 },
            ready: false,
            idle: false,
            keys: HashSet::new(),
            grabbed: false,
            world_time: time,
            tick_accum: 0.0,
            last_frame: Instant::now(),
            last_sent: (i32::MIN, 0, -1),
            temps: BiomeSource::new(seed),
            fps_frames: 0,
            fps_time: Instant::now(),
            fps: 0,
            loaded: 0,
            fx: render::fx::TextureFx::terrain_set(),
            fx_rng: render::fx::Rng::new(0x9E3779B97F4A7C15),
            cloud_ticks: 0,
        }
    }

    fn set_grab(&mut self, on: bool) {
        let Some(gpu) = &self.gpu else { return };
        let w = &gpu.window;
        if on {
            let ok = w.set_cursor_grab(CursorGrabMode::Locked).or_else(|_| w.set_cursor_grab(CursorGrabMode::Confined));
            if ok.is_ok() {
                w.set_cursor_visible(false);
                self.grabbed = true;
            }
        } else {
            let _ = w.set_cursor_grab(CursorGrabMode::None);
            w.set_cursor_visible(true);
            self.grabbed = false;
        }
    }

    fn frame(&mut self, el: &ActiveEventLoop) {
        let now = Instant::now();
        let dt = (now - self.last_frame).as_secs_f64().min(0.25);
        self.last_frame = now;

        // World messages: upload a bounded number of meshes per frame.
        let mut uploads = 0;
        if let (Some(rx), Some(gpu)) = (&self.rx, &mut self.gpu) {
            while uploads < 64 {
                match rx.try_recv() {
                    Ok(FromWorker::Ready { spawn }) => {
                        self.ready = true;
                        if self.opts.at.is_none() {
                            self.cam.pos = DVec3::new(spawn.0, spawn.1, spawn.2);
                        }
                    }
                    Ok(FromWorker::Mesh { cx, cz, opaque, translucent }) => {
                        gpu.upload_chunk(cx, cz, &opaque, &translucent);
                        uploads += 1;
                        self.idle = false;
                    }
                    Ok(FromWorker::Idle { loaded }) => {
                        self.idle = true;
                        self.loaded = loaded;
                    }
                    Err(_) => break,
                }
            }
        }

        // Fly camera (yaw 0 faces +Z, like the original).
        let mut speed = 10.0;
        if self.keys.contains(&KeyCode::ControlLeft) {
            speed = 40.0;
        }
        let (sy, cy) = (self.cam.yaw.to_radians() as f64).sin_cos();
        let fwd = DVec3::new(-sy, 0.0, cy);
        let right = DVec3::new(-cy, 0.0, -sy);
        let mut mv = DVec3::ZERO;
        if self.keys.contains(&KeyCode::KeyW) {
            mv += fwd;
        }
        if self.keys.contains(&KeyCode::KeyS) {
            mv -= fwd;
        }
        if self.keys.contains(&KeyCode::KeyD) {
            mv += right;
        }
        if self.keys.contains(&KeyCode::KeyA) {
            mv -= right;
        }
        if self.keys.contains(&KeyCode::Space) {
            mv.y += 1.0;
        }
        if self.keys.contains(&KeyCode::ShiftLeft) {
            mv.y -= 1.0;
        }
        if mv.length_squared() > 0.0 {
            self.cam.pos += mv.normalize() * speed * dt;
        }

        // 20 ticks per second.
        self.tick_accum += dt * 20.0;
        let whole = self.tick_accum.floor();
        self.world_time += whole as i64;
        self.tick_accum -= whole;
        // Per tick: animated textures and cloud drift (capped so a stall doesn't spin).
        for _ in 0..(whole as i64).min(20) {
            for f in &mut self.fx {
                f.tick(&mut self.fx_rng);
            }
        }
        self.cloud_ticks += whole as i64;
        if whole > 0.0 {
            if let Some(g) = &self.gpu {
                g.update_fx(&self.fx);
            }
        }
        let partial = self.tick_accum as f32;
        let angle = sky::celestial_angle(self.world_time, partial);
        let sky_sub = sky::skylight_subtracted(sky::celestial_angle(self.world_time, 1.0));

        let ccx = (self.cam.pos.x.floor() as i32) >> 4;
        let ccz = (self.cam.pos.z.floor() as i32) >> 4;
        if self.ready && (ccx, ccz, sky_sub) != self.last_sent {
            self.last_sent = (ccx, ccz, sky_sub);
            if let Some(tx) = &self.tx {
                let _ = tx.send(ToWorker::Camera { cx: ccx, cz: ccz, sky_sub });
            }
        }

        let temp = self.temps.raw_temperature(self.cam.pos.x.floor() as i32, self.cam.pos.z.floor() as i32) as f32;
        let params = FrameParams {
            cam_pos: self.cam.pos,
            yaw: self.cam.yaw,
            pitch: self.cam.pitch,
            fov_y_deg: 70.0,
            celestial: angle,
            sky_color: sky::sky_color(angle, temp),
            fog_color: sky::fog_color(angle, temp, 0),
            far: 256.0,
            cloud_offset: self.cloud_ticks as f32 + partial,
        };

        let want_shot = self.opts.shot.is_some() && self.ready && self.idle && uploads == 0;
        let Some(gpu) = &mut self.gpu else { return };
        let shot = gpu.render(&params, want_shot || self.keys.remove(&KeyCode::F2));
        if let Some(img) = shot {
            let path = self.opts.shot.clone().unwrap_or_else(|| {
                let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
                format!("screenshot-{t}.png").into()
            });
            match assets::save_png(&img, &path) {
                Ok(()) => eprintln!("saved {}", path.display()),
                Err(e) => eprintln!("screenshot failed: {e}"),
            }
            if want_shot {
                el.exit();
            }
        }

        self.fps_frames += 1;
        if self.fps_time.elapsed().as_secs_f64() >= 1.0 {
            self.fps = self.fps_frames;
            self.fps_frames = 0;
            if std::env::var_os("BETA_FPS").is_some() {
                eprintln!("fps {} chunks {}", self.fps, gpu.chunk_count());
            }
            self.fps_time = Instant::now();
            let p = self.cam.pos;
            gpu.window.set_title(&format!(
                "Minecraft Beta 1.7.3 (Rust) — seed {} — {} fps — x {:.1} y {:.1} z {:.1} — {} chunks{}",
                self.opts.seed,
                self.fps,
                p.x,
                p.y,
                p.z,
                gpu.chunk_count(),
                if self.ready { "" } else { " — building terrain..." }
            ));
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, el: &ActiveEventLoop) {
        if self.gpu.is_some() {
            return;
        }
        let assets = match assets::load() {
            Ok(a) => a,
            Err(e) => {
                eprintln!("{e}");
                el.exit();
                return;
            }
        };
        let attrs = Window::default_attributes()
            .with_title("Minecraft Beta 1.7.3 (Rust)")
            .with_inner_size(winit::dpi::LogicalSize::new(854.0, 480.0));
        let window = Arc::new(el.create_window(attrs).expect("create window"));
        self.gpu = Some(Gpu::new(window, &assets.terrain, &assets.sun, &assets.moon, &assets.clouds));
        let col = Colorizers { grass: assets.grass_color, foliage: assets.foliage_color };
        let (tx, rx) = worker::spawn(self.opts.seed, self.opts.world_rand, col);
        self.tx = Some(tx);
        self.rx = Some(rx);
        if let Some((x, y, z, yaw, pitch)) = self.opts.at {
            self.cam = Camera { pos: DVec3::new(x, y, z), yaw, pitch };
        }
        eprintln!("seed {}", self.opts.seed);
    }

    fn window_event(&mut self, el: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => el.exit(),
            WindowEvent::Resized(s) => {
                if let Some(g) = &mut self.gpu {
                    g.resize(s.width, s.height);
                }
            }
            WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. } => {
                if !self.grabbed {
                    self.set_grab(true);
                }
            }
            WindowEvent::Focused(false) => self.set_grab(false),
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    match event.state {
                        ElementState::Pressed => {
                            match code {
                                KeyCode::Escape => self.set_grab(false),
                                KeyCode::KeyT if !event.repeat => self.world_time += 1000,
                                _ => {}
                            }
                            self.keys.insert(code);
                        }
                        ElementState::Released => {
                            if code != KeyCode::F2 {
                                self.keys.remove(&code);
                            }
                        }
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                self.frame(el);
                if let Some(g) = &self.gpu {
                    g.window.request_redraw();
                }
            }
            _ => {}
        }
    }

    fn device_event(&mut self, _el: &ActiveEventLoop, _id: winit::event::DeviceId, event: DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta } = event {
            if self.grabbed {
                // Default sensitivity: 0.15 degrees per mouse count, as in EntityPlayerSP.setAngles.
                self.cam.yaw += delta.0 as f32 * 0.15;
                self.cam.pitch = (self.cam.pitch + delta.1 as f32 * 0.15).clamp(-90.0, 90.0);
            }
        }
    }
}

/// `--skycolor seed x z time partial`: print sky/fog/cloud values as float bits,
/// in the same format as the oracle's `skycolor` mode, for comparison.
fn sky_check(a: &[String]) {
    let seed: i64 = a[2].parse().unwrap();
    let (x, z): (f64, f64) = (a[3].parse().unwrap(), a[4].parse().unwrap());
    let time: i64 = a[5].parse().unwrap();
    let partial: f32 = a[6].parse().unwrap();
    let angle = sky::celestial_angle(time, partial);
    let temp = BiomeSource::new(seed).raw_temperature(x.floor() as i32, z.floor() as i32) as f32;
    let s = sky::sky_color(angle, temp);
    let f = sky::fog_color_base(angle);
    let c = sky::cloud_color(angle);
    let b = |v: f32| v.to_bits() as i32;
    println!(
        "angle {} sub {} sky {} {} {} fog {} {} {} cloud {} {} {} stars {}",
        b(angle),
        sky::skylight_subtracted(sky::celestial_angle(time, 1.0)),
        b(s[0]),
        b(s[1]),
        b(s[2]),
        b(f[0]),
        b(f[1]),
        b(f[2]),
        b(c[0]),
        b(c[1]),
        b(c[2]),
        b(sky::star_brightness(angle))
    );
}

/// `--meshcheck seed wrand oracle-mesh.bin`: mesh the same chunks after the vanilla
/// preload and compare every vertex with the original RenderBlocks output.
fn mesh_check(a: &[String]) -> bool {
    let seed: i64 = a[2].parse().unwrap();
    let wrand: i64 = a[3].parse().unwrap();
    let data = std::fs::read(&a[4]).expect("read mesh dump");
    let mut o = 0usize;
    let mut i32_ = || {
        let v = i32::from_le_bytes(data[o..o + 4].try_into().unwrap());
        o += 4;
        v
    };
    let assets = assets::load().expect("assets");
    let col = Colorizers { grass: assets.grass_color, foliage: assets.foliage_color };
    let world = worldgen::World::new_vanilla(seed, wrand);
    let biomes = std::cell::RefCell::new(BiomeSource::new(seed));
    let magic = i32_();
    let _ = i32_();
    assert_eq!(magic, i32::from_le_bytes(*b"B173"), "not a mesh dump");
    let sky_sub = i32_();
    let n = i32_();
    let mut ok = true;
    let mut seen = [0usize; 256];
    for _ in 0..n {
        let (cx, cz) = (i32_(), i32_());
        for &b in world.chunk(cx, cz).blocks.iter() {
            seen[b as usize] += 1;
        }
        let (opaque, translucent) = worker::mesh_chunk(&world, cx, cz, sky_sub, &biomes, &col);
        for (pass, rust) in [(0, &opaque), (1, &translucent)] {
            let nv = i32_() as usize;
            let mut java = Vec::with_capacity(nv);
            for _ in 0..nv {
                let f: Vec<i32> = (0..6).map(|_| i32_()).collect();
                java.push(f);
            }
            let mut bad = 0;
            for (k, j) in java.iter().enumerate() {
                let Some(r) = rust.get(k) else { break };
                let rv = [
                    r.pos[0].to_bits() as i32,
                    r.pos[1].to_bits() as i32,
                    r.pos[2].to_bits() as i32,
                    r.uv[0].to_bits() as i32,
                    r.uv[1].to_bits() as i32,
                    i32::from_le_bytes(r.color),
                ];
                if rv[..] != j[..] {
                    if bad < 4 {
                        let f = |v: i32| f32::from_bits(v as u32);
                        eprintln!(
                            "  chunk ({cx},{cz}) pass {pass} vertex {k}: java pos ({},{},{}) uv ({},{}) rgba {:08x} | rust pos ({},{},{}) uv ({},{}) rgba {:08x}",
                            f(j[0]), f(j[1]), f(j[2]), f(j[3]), f(j[4]), j[5],
                            r.pos[0], r.pos[1], r.pos[2], r.uv[0], r.uv[1], rv[5]
                        );
                    }
                    bad += 1;
                }
            }
            let status = if bad == 0 && java.len() == rust.len() { "exact" } else { ok = false; "DIFFERS" };
            println!(
                "chunk ({cx},{cz}) pass {pass}: java {} vertices, rust {}, {bad} differ: {status}",
                java.len(),
                rust.len()
            );
        }
    }
    let kinds: Vec<String> = (1..256)
        .filter(|&i| seen[i] > 0)
        .map(|i| format!("{}:{}", worldgen::block::props(i as u8).name, seen[i]))
        .collect();
    println!("blocks covered: {}", kinds.join(" "));
    ok
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--skycolor") {
        sky_check(&args);
        return;
    }
    if args.get(1).map(String::as_str) == Some("--meshcheck") {
        let ok = mesh_check(&args);
        println!("{}", if ok { "MESHES EXACT" } else { "MESHES DIFFER" });
        std::process::exit(if ok { 0 } else { 1 });
    }
    let opts = parse_args();
    let el = EventLoop::new().expect("event loop");
    el.set_control_flow(ControlFlow::Poll);
    let mut app = App::new(opts);
    el.run_app(&mut app).expect("run");
}
