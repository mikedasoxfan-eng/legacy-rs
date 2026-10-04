//! Minecraft Beta 1.7.3 in Rust — world viewer (fly camera) on the bit-exact generator.
//!
//! Controls: click to capture the mouse, WASD to fly, Space/Shift up/down, Ctrl faster,
//! T skips 1000 ticks of time, F2 saves a screenshot, Esc releases the mouse.
//!
//! Worlds are saved in the b1.7.3 format under `saves/<name>` (copy a folder into
//! `.minecraft/saves` to open it in the real game, or point --world-dir at a real save).
//!
//! Options: --world <name>   --world-dir <path>   --no-save   --seed <number or text>
//!          --server [--view-distance 3..15]   (multiplayer-server chunk loading/unloading)
//!          --world-rand <n>  --at x,y,z,yaw,pitch  --shot <file.png>

mod assets;
mod gui;
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
use worker::{FromWorker, ToWorker, WorldOptions};
use worldgen::BiomeSource;

/// Java's `String.hashCode`, which the world-creation screen uses for non-numeric seeds.
fn java_string_hash(s: &str) -> i32 {
    s.encode_utf16().fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(c as i32))
}

struct Options {
    world: WorldOptions,
    at: Option<(f64, f64, f64, f32, f32)>,
    shot: Option<std::path::PathBuf>,
}

fn parse_args() -> Options {
    let args: Vec<String> = std::env::args().collect();
    let get = |name: &str| args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).cloned();
    let has = |name: &str| args.iter().any(|a| a == name);
    let seed_arg = get("--seed");
    let seed = match &seed_arg {
        Some(s) => s.parse::<i64>().unwrap_or_else(|_| java_string_hash(s) as i64),
        None => {
            // GuiCreateWorld with an empty seed: new Random().nextLong()
            let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as i64;
            worldgen::JavaRandom::new(nanos).next_long()
        }
    };
    let dir = if has("--no-save") {
        None
    } else if let Some(d) = get("--world-dir") {
        Some(std::path::PathBuf::from(d))
    } else {
        let name = get("--world").unwrap_or_else(|| match &seed_arg {
            Some(s) => format!("Seed {s}"),
            None => "World1".to_string(),
        });
        Some(std::path::Path::new("saves").join(name))
    };
    let at = get("--at").map(|s| {
        let v: Vec<f64> = s.split(',').map(|p| p.trim().parse().expect("--at x,y,z,yaw,pitch")).collect();
        (v[0], v[1], v[2], v[3] as f32, v[4] as f32)
    });
    Options {
        world: WorldOptions {
            dir,
            seed,
            world_rand: get("--world-rand").and_then(|s| s.parse().ok()).unwrap_or(seed ^ 0x5DEECE66D),
            server: has("--server"),
            view_distance: get("--view-distance").and_then(|s| s.parse().ok()).unwrap_or(10),
        },
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
    worker: Option<std::thread::JoinHandle<()>>,
    cam: Camera,
    ready: bool,
    idle: bool,
    keys: HashSet<KeyCode>,
    grabbed: bool,
    world_time: i64,
    /// When the last tick arrived from the world thread (for partial ticks).
    last_tick: Instant,
    last_frame: Instant,
    last_cam_sent: DVec3,
    temps: BiomeSource,
    seed: i64,
    world_name: String,
    fps_frames: u32,
    fps_time: Instant,
    fps: u32,
    fx: Vec<render::fx::TextureFx>,
    fx_rng: render::fx::Rng,
    cloud_ticks: i64,
}

impl App {
    fn new(opts: Options) -> App {
        let seed = opts.world.seed;
        App {
            opts,
            gpu: None,
            tx: None,
            rx: None,
            worker: None,
            cam: Camera { pos: DVec3::new(0.5, 80.0, 0.5), yaw: 0.0, pitch: 0.0 },
            ready: false,
            idle: false,
            keys: HashSet::new(),
            grabbed: false,
            world_time: 0,
            last_tick: Instant::now(),
            last_frame: Instant::now(),
            last_cam_sent: DVec3::splat(f64::MAX),
            temps: BiomeSource::new(seed),
            seed,
            world_name: String::new(),
            fps_frames: 0,
            fps_time: Instant::now(),
            fps: 0,
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

    /// Ask the world thread to save and stop, and wait for it (the game saves on quit too).
    fn shutdown(&mut self, el: &ActiveEventLoop) {
        if let Some(g) = &self.gpu {
            g.window.set_title("Minecraft Beta 1.7.3 (Rust) — saving world...");
        }
        if let Some(tx) = self.tx.take() {
            let _ = tx.send(ToWorker::Quit);
        }
        if let Some(h) = self.worker.take() {
            let _ = h.join();
        }
        el.exit();
    }

    fn frame(&mut self, el: &ActiveEventLoop) {
        let now = Instant::now();
        let dt = (now - self.last_frame).as_secs_f64().min(0.25);
        self.last_frame = now;

        // World messages: upload a bounded number of meshes per frame.
        let mut uploads = 0;
        let mut new_ticks = 0;
        if let (Some(rx), Some(gpu)) = (&self.rx, &mut self.gpu) {
            while uploads < 64 {
                match rx.try_recv() {
                    Ok(FromWorker::Ready { spawn, seed, name }) => {
                        self.ready = true;
                        self.seed = seed;
                        self.temps = BiomeSource::new(seed);
                        self.world_name = name;
                        if self.opts.at.is_none() {
                            self.cam.pos = DVec3::new(spawn.0, spawn.1, spawn.2);
                        }
                    }
                    Ok(FromWorker::Mesh { cx, cz, opaque, translucent }) => {
                        gpu.upload_chunk(cx, cz, &opaque, &translucent);
                        uploads += 1;
                        self.idle = false;
                    }
                    Ok(FromWorker::Drop { cx, cz }) => gpu.remove_chunk(cx, cz),
                    Ok(FromWorker::Time(t)) => {
                        if t > self.world_time {
                            new_ticks += (t - self.world_time).min(20);
                        }
                        self.world_time = t;
                        self.last_tick = now;
                    }
                    Ok(FromWorker::Idle) => self.idle = true,
                    Ok(FromWorker::Saved) => {}
                    Ok(FromWorker::Error(e)) => {
                        eprintln!("{e}");
                        el.exit();
                        return;
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
        if self.ready && self.cam.pos.distance_squared(self.last_cam_sent) > 1e-4 {
            self.last_cam_sent = self.cam.pos;
            if let Some(tx) = &self.tx {
                let p = self.cam.pos;
                let _ = tx.send(ToWorker::Camera { x: p.x, y: p.y, z: p.z });
            }
        }

        // Per tick: animated textures and cloud drift.
        for _ in 0..new_ticks {
            for f in &mut self.fx {
                f.tick(&mut self.fx_rng);
            }
        }
        self.cloud_ticks += new_ticks;
        if new_ticks > 0 {
            if let Some(g) = &self.gpu {
                g.update_fx(&self.fx);
            }
        }
        let partial = ((now - self.last_tick).as_secs_f32() * 20.0).min(1.0);
        let angle = sky::celestial_angle(self.world_time, partial);

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
                self.shutdown(el);
                return;
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
                "Minecraft Beta 1.7.3 (Rust) — {} (seed {}) — {} fps — x {:.1} y {:.1} z {:.1} — {} chunks{}{}",
                self.world_name,
                self.seed,
                self.fps,
                p.x,
                p.y,
                p.z,
                gpu.chunk_count(),
                if self.opts.world.server { " — server mode" } else { "" },
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
        self.gpu = Some(Gpu::new(window, &assets.terrain, &assets.sun, &assets.moon, &assets.clouds, &assets.mobs));
        let col = Colorizers { grass: assets.grass_color, foliage: assets.foliage_color };
        let (tx, rx, h) = worker::spawn(self.opts.world.clone(), col);
        self.tx = Some(tx);
        self.rx = Some(rx);
        self.worker = Some(h);
        if let Some((x, y, z, yaw, pitch)) = self.opts.at {
            self.cam = Camera { pos: DVec3::new(x, y, z), yaw, pitch };
        }
    }

    fn window_event(&mut self, el: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => self.shutdown(el),
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
                                KeyCode::KeyT if !event.repeat => {
                                    if let Some(tx) = &self.tx {
                                        let _ = tx.send(ToWorker::AddTime(1000));
                                    }
                                }
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
    if args.get(1).map(String::as_str) == Some("--modelshot") {
        let path = std::path::Path::new(args.get(2).expect("--modelshot file.png [--variants]"));
        let scene = args
            .iter()
            .position(|a| a == "--dump-scene")
            .map(|i| std::path::Path::new(args.get(i + 1).expect("--dump-scene file.txt")));
        if let Err(e) = render::entity::shot::model_shot(path, args.iter().any(|a| a == "--variants"), scene) {
            eprintln!("{e}");
            std::process::exit(1);
        }
        return;
    }
    if args.get(1).map(String::as_str) == Some("--modelcheck") {
        let ok = render::entity::check::model_check(args.get(2).expect("--modelcheck oracle-models.bin"));
        println!("{}", if ok { "MODELS EXACT" } else { "MODELS DIFFER" });
        std::process::exit(if ok { 0 } else { 1 });
    }
    if args.get(1).map(String::as_str) == Some("--guishot") {
        let script = args.get(2).expect("--guishot script.txt outdir");
        let out = std::path::Path::new(args.get(3).map(String::as_str).unwrap_or("shots/gui"));
        if let Err(e) = gui::shot::run(script, out) {
            eprintln!("{e}");
            std::process::exit(1);
        }
        return;
    }
    if args.get(1).map(String::as_str) == Some("--guidiff") {
        let ok = gui::shot::diff(
            std::path::Path::new(&args[2]),
            std::path::Path::new(&args[3]),
            args.get(4).map(std::path::Path::new),
        );
        println!("{}", if ok { "GUI EXACT" } else { "GUI DIFFERS" });
        std::process::exit(if ok { 0 } else { 1 });
    }
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
