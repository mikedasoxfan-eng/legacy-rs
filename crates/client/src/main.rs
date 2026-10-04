//! Minecraft Beta 1.7.3 in Rust: the original menus, then a world viewer (fly camera) on
//! the bit-exact world simulation.
//!
//! Controls in a world: WASD to fly, Space/Shift up/down, Ctrl faster, T skips 1000 ticks,
//! F2 saves a screenshot, Esc opens the game menu.
//!
//! Worlds are saved in the b1.7.3 format under `saves/<name>` (copy a folder into
//! `.minecraft/saves` to open it in the real game, or point --world-dir at a real save).
//!
//! Without options the title screen opens. To go straight into a world:
//!          --world <name>   --world-dir <path>   --no-save   --seed <number or text>
//!          --server [--view-distance 3..15]   (multiplayer-server chunk loading/unloading)
//!          --world-rand <n>  --at x,y,z,yaw,pitch  --shot <file.png>

mod app;
mod assets;
mod gui;
mod render;
mod sound;
mod worker;

use render::blocks::Colorizers;
use render::sky;
use winit::event_loop::{ControlFlow, EventLoop};
use worker::WorldOptions;
use worldgen::BiomeSource;

/// Java's `String.hashCode`, which the world-creation screen uses for non-numeric seeds.
fn java_string_hash(s: &str) -> i32 {
    s.encode_utf16().fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(c as i32))
}

pub struct Options {
    pub world: WorldOptions,
    pub at: Option<(f64, f64, f64, f32, f32)>,
    pub shot: Option<std::path::PathBuf>,
    pub bolt: Option<render::weather::Lightning>,
    /// A world was named on the command line: open it directly instead of the title screen.
    pub direct: bool,
    /// `--flowtest <dir>`: scripted click-through of the menus with screenshots.
    pub flowtest: Option<std::path::PathBuf>,
    /// `--shot-ticks N`: with --shot, let the world run N ticks first.
    pub shot_ticks: i64,
}

fn parse_args() -> Options {
    let args: Vec<String> = std::env::args().collect();
    let get = |name: &str| {
        args.iter().enumerate().find_map(|(i, a)| {
            if a == name {
                args.get(i + 1).cloned()
            } else {
                a.strip_prefix(&format!("{name}=")).map(str::to_owned)
            }
        })
    };
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
            weather: get("--weather").map(|v| {
                let v: Vec<f32> = v.split(',').map(|s| s.parse().expect("--weather rain,thunder,flash")).collect();
                render::weather::Weather {
                    rain: v[0],
                    prev_rain: v[0],
                    thunder: v[1],
                    prev_thunder: v[1],
                    flash: v.get(2).copied().unwrap_or(0.0) as i32,
                }
            }),
            time: get("--time").map(|s| s.parse().expect("--time ticks")),
            name: None,
        },
        flowtest: get("--flowtest").map(Into::into),
        shot_ticks: get("--shot-ticks").and_then(|s| s.parse().ok()).unwrap_or(0),
        direct: ["--world", "--world-dir", "--seed", "--no-save", "--shot", "--at", "--server"]
            .iter()
            .any(|f| args.iter().any(|a| a == f || a.starts_with(&format!("{f}=")))),
        at,
        shot: get("--shot").map(Into::into),
        bolt: get("--bolt").map(|v| {
            let v: Vec<&str> = v.split(',').collect();
            render::weather::Lightning {
                pos: [v[0].parse().unwrap(), v[1].parse().unwrap(), v[2].parse().unwrap()],
                bolt_vertex: v[3].parse().unwrap(),
            }
        }),
    }
}

/// `--skycolor seed x z time partial [rain thunder flash [prev-rain prev-thunder [fog-prev fog-current]]]`: float bits,
/// in the same format as the oracle's `skycolor` mode, for comparison.
fn sky_check(a: &[String]) {
    let seed: i64 = a[2].parse().unwrap();
    let (x, z): (f64, f64) = (a[3].parse().unwrap(), a[4].parse().unwrap());
    let time: i64 = a[5].parse().unwrap();
    let partial: f32 = a[6].parse().unwrap();
    let angle = sky::celestial_angle(time, partial);
    let temp = BiomeSource::new(seed).raw_temperature(x.floor() as i32, z.floor() as i32) as f32;
    let value = |i: usize| a.get(i).map_or(0.0, |v| v.parse::<f32>().unwrap());
    let weather = render::weather::Weather {
        rain: value(7),
        thunder: value(8),
        flash: value(9) as i32,
        prev_rain: a.get(10).map_or(value(7), |v| v.parse().unwrap()),
        prev_thunder: a.get(11).map_or(value(8), |v| v.parse().unwrap()),
    };
    let rain = weather.rain_at(partial);
    let thunder = weather.thunder_at(partial);
    let s = sky::sky_color(angle, temp, rain, thunder, weather.flash, partial);
    let f = sky::fog_color_base(angle);
    let c = sky::cloud_color(angle, rain, thunder);
    let fog_prev = a.get(12).map_or(1.0f32, |v| v.parse().unwrap());
    let fog_current = a.get(13).map_or(1.0f32, |v| v.parse().unwrap());
    let distance = a.get(14).map_or(0i32, |v| v.parse().unwrap());
    let far = (256 >> distance) as f32;
    let fog = sky::fog_color(angle, temp, distance, rain, thunder, weather.flash, partial)
        .map(|c| c * (fog_prev + (fog_current - fog_prev) * partial));
    let b = |v: f32| v.to_bits() as i32;
    println!(
        "angle {} sub {} sky {} {} {} fog {} {} {} cloud {} {} {} stars {} renderfog {} {} {} renderstars {} sunalpha {} fogrange {} {}",
        b(angle),
        sky::skylight_subtracted(sky::celestial_angle(time, 1.0), weather.rain_at(1.0), weather.thunder_at(1.0)),
        b(s[0]),
        b(s[1]),
        b(s[2]),
        b(f[0]),
        b(f[1]),
        b(f[2]),
        b(c[0]),
        b(c[1]),
        b(c[2]),
        b(sky::star_brightness(angle)), b(fog[0]), b(fog[1]), b(fog[2]), b(sky::star_brightness(angle) * (1.0f32 - rain)), b(1.0f32 - rain), b(far * 0.25f32), b(far)
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
    let mut video = render::video::VideoSettings::default();
    video.fancy = a.get(5).map(String::as_str) != Some("fast");
    video.ao = a.get(6).map(String::as_str) != Some("off");
    let mut world = worldgen::World::new_vanilla(seed, wrand);
    world.leaves_opaque = !video.fancy;
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
        let (opaque, translucent) = worker::mesh_chunk(&world, cx, cz, sky_sub, &biomes, &col, video);
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
            let status = if bad == 0 && java.len() == rust.len() {
                "exact"
            } else {
                ok = false;
                "DIFFERS"
            };
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
    if args.get(1).map(String::as_str) == Some("--cloudcheck") {
        render::video_check::clouds(&args);
        return;
    }
    if args.get(1).map(String::as_str) == Some("--renderershot") {
        if let Err(e) =
            render::entity::shot::renderer_shot(std::path::Path::new(args.get(2).expect("--renderershot file.png")))
        {
            eprintln!("{e}");
            std::process::exit(1);
        }
        return;
    }
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
    if args.get(1).map(String::as_str) == Some("--livingrenderercheck") {
        let ok =
            render::entity::object_check::living_check(args.get(2).expect("--livingrenderercheck oracle-living.bin"));
        println!("{}", if ok { "LIVING RENDERERS EXACT" } else { "LIVING RENDERERS DIFFER" });
        std::process::exit(if ok { 0 } else { 1 });
    }
    if args.get(1).map(String::as_str) == Some("--effectcheck") {
        let ok = render::entity::object_check::effects_check(args.get(2).expect("--effectcheck oracle-effects.bin"));
        println!("{}", if ok { "EFFECTS EXACT" } else { "EFFECTS DIFFER" });
        std::process::exit(if ok { 0 } else { 1 });
    }
    if args.get(1).map(String::as_str) == Some("--renderercheck") {
        let ok = render::entity::object_check::check(args.get(2).expect("--renderercheck oracle-renderers.bin"));
        println!("{}", if ok { "RENDERERS EXACT" } else { "RENDERERS DIFFER" });
        std::process::exit(if ok { 0 } else { 1 });
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
    if args.get(1).map(String::as_str) == Some("--weathercheck") {
        render::weather_check::check(&args);
        return;
    }
    if args.get(1).map(String::as_str) == Some("--soundcheck") {
        std::process::exit(if sound::check() { 0 } else { 1 });
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
    let mut app = app::App::new(opts);
    el.run_app(&mut app).expect("run");
}
