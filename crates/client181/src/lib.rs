//! Minecraft Beta 1.8.1 in Rust: the original menus and the singleplayer game on the
//! bit-exact world simulation.
//!
//! Controls are the original's (rebindable in Options): WASD, Space, Shift, mouse, 1-9,
//! Q, F1/F2/F3/F5/F8, F for render distance, Esc for the game menu. With `--server` there is
//! no player: WASD flies the camera, Space/Shift up/down, Ctrl faster.
//!
//! Worlds are saved in the b1.8.1 format under `b1.8.1/saves/<name>` (copy a folder into
//! `.minecraft/saves` to open it in the real game, or point --world-dir at a real save).
//!
//! Without options the title screen opens. To go straight into a world:
//!          --world <name>   --world-dir <path>   --no-save   --seed <number or text>
//!          --server [--view-distance 3..15]   (multiplayer-server chunk loading/unloading)
//!          --world-rand <n>  --at x,y,z,yaw,pitch  --shot <file.png>

mod app;
mod screenshot;
mod assets;
mod texture_packs;
mod gui;
mod playscript;
mod render;
mod sound;
mod worker;
mod world_effects;

use render::blocks::Colorizers;
use render::sky;
use winit::event_loop::{ControlFlow, EventLoop};
use worker::WorldOptions;
use worldgen::BiomeSource;

const HELP: &str = "\
beta-rs: Minecraft Beta 1.8.1 in Rust

Usage: beta [options]

Without options the title screen opens, exactly like the original. Textures, sounds and fonts
are read from your own installation (%APPDATA%\\.minecraft\\versions\\b1.8.1\\b1.8.1.jar).

Options:
  --seed <number or text>        create and enter a world with this seed
  --world <name>                 enter (or create) saves\\<name>
  --world-dir <path>             enter any save folder, including one from the real game
  --no-save                      play without writing anything to disk
  --time <ticks>                 start at this time of day
  --weather <rain,thunder,flash> start with rain/thunder strength (0 to 1 each)
  --options-dir <path>           read options.txt (and keep stats, screenshots) there
  --server [--view-distance n]   use the dedicated server's chunk loading
  --shot <file.png> [--shot-ticks n]
                                 take a screenshot after n ticks and exit
  --play-script <file.txt>       drive the player from a script (unattended testing)
  --version                      print the version
  --help                         print this help
";

/// Java's `String.hashCode`, which the world-creation screen uses for non-numeric seeds.
fn java_string_hash(s: &str) -> i32 {
    s.encode_utf16().fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(c as i32))
}

/// The b1.8.1 data folder (options.txt, saves, stats, screenshots, texture packs).
pub const DATA_DIR: &str = "b1.8.1";

pub struct Options {
    pub world: WorldOptions,
    pub at: Option<(f64, f64, f64, f32, f32)>,
    pub shot: Option<std::path::PathBuf>,
    pub bolt: Option<render::weather::Lightning>,
    /// A world was named on the command line: open it directly instead of the title screen.
    pub direct: bool,
    /// `--flowtest <dir>`: scripted click-through of the menus with screenshots.
    pub flowtest: Option<std::path::PathBuf>,
    /// Read options.txt from this directory (default: working directory).
    pub options_dir: Option<std::path::PathBuf>,
    pub video_flowtest: Option<std::path::PathBuf>,
    /// `--shot-ticks N`: with --shot, let the world run N ticks first.
    pub shot_ticks: i64,
    /// `--play-script <file>`: scripted keys and mouse for the player (see playscript.rs).
    pub play_script: Option<std::path::PathBuf>,
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
        Some(std::path::Path::new(DATA_DIR).join("saves").join(name))
    };
    let at = get("--at").map(|s| {
        let v: Vec<f64> = s.split(',').map(|p| p.trim().parse().expect("--at x,y,z,yaw,pitch")).collect();
        (v[0], v[1], v[2], v[3] as f32, v[4] as f32)
    });
    Options {
        world: WorldOptions {
            settings: None,
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
        // Beta 1.8.1 keeps its options, saves, stats and screenshots apart from b1.7.3's.
        options_dir: Some(get("--options-dir").map(Into::into).unwrap_or_else(|| DATA_DIR.into())),
        video_flowtest: get("--video-flowtest").map(Into::into),
        shot_ticks: get("--shot-ticks").and_then(|s| s.parse().ok()).unwrap_or(0),
        play_script: get("--play-script").map(Into::into),
        direct: ["--world", "--world-dir", "--seed", "--no-save", "--shot", "--at", "--server"]
            .iter()
        .any(|f| {
            args.iter()
                .any(|a| a == f || a.starts_with(&format!("{f}=")))
        }),
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

/// `--skycolor seed x z time partial [rain thunder flash [prev-rain prev-thunder [fog-prev fog-current [distance]]]]`: float bits,
/// in the same format as the oracle's `skycolor` mode, for comparison.
fn sky_check(a: &[String]) {
    let seed: i64 = a[2].parse().unwrap();
    let (x, z): (f64, f64) = (a[3].parse().unwrap(), a[4].parse().unwrap());
    let time: i64 = a[5].parse().unwrap();
    let partial: f32 = a[6].parse().unwrap();
    // WorldProviderHell: celestial angle 0.5, one hell biome at temperature 1.
    let nether = a[1] == "--skycolor-nether";
    let celestial = |time: i64, partial: f32| if nether { 0.5 } else { sky::celestial_angle(time, partial) };
    let angle = celestial(time, partial);
    let biomes = if nether { BiomeSource::hell() } else { BiomeSource::new(seed) };
    let temp = biomes.raw_temperature(x.floor() as i32, z.floor() as i32) as f32;
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
    let f = if nether { [0.2f32, 0.03, 0.03] } else { sky::fog_color_base(angle) };
    let c = sky::cloud_color(angle, rain, thunder);
    let fog_prev = a.get(12).map_or(1.0f32, |v| v.parse().unwrap());
    let fog_current = a.get(13).map_or(1.0f32, |v| v.parse().unwrap());
    let distance = a.get(14).map_or(0i32, |v| v.parse().unwrap());
    let far = (256 >> distance) as f32;
    let fog = if nether {
        sky::nether_fog_color(s, distance, rain, thunder)
    } else {
        // The oracle camera is a headless EntityPlayerSP with zero rotations.
        sky::fog_color(angle, temp, distance, rain, thunder, weather.flash, partial, sky::look(0.0, 0.0, 0.0, 0.0, partial))
    }
    .map(|c| c * (fog_prev + (fog_current - fog_prev) * partial));
    let fog_start = if nether { 0.0 } else { far * 0.25f32 };
    let b = |v: f32| v.to_bits() as i32;
    println!(
        "angle {} sub {} sky {} {} {} fog {} {} {} cloud {} {} {} stars {} renderfog {} {} {} renderstars {} sunalpha {} fogrange {} {}",
        b(angle),
        sky::skylight_subtracted(celestial(time, 1.0), weather.rain_at(1.0), weather.thunder_at(1.0)),
        b(s[0]),
        b(s[1]),
        b(s[2]),
        b(f[0]),
        b(f[1]),
        b(f[2]),
        b(c[0]),
        b(c[1]),
        b(c[2]),
        b(sky::star_brightness(angle)), b(fog[0]), b(fog[1]), b(fog[2]), b(sky::star_brightness(angle) * (1.0f32 - rain)), b(1.0f32 - rain), b(fog_start), b(far)
    );
}

/// The block showcase (tools/oracle/make_showcase.py): flat stone at y 63, the listed
/// blocks placed raw, a fixed light pattern; built exactly like the oracle's.
fn showcase_world(path: &str) -> worldgen::World {
    use worldgen::world::chunk::{idx, Chunk};
    let text = std::fs::read_to_string(path).expect("read showcase layout");
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let head: Vec<i32> = lines.next().unwrap().split(' ').skip(1).map(|v| v.parse().unwrap()).collect();
    let (x0, z0, x1, z1) = (head[0], head[1], head[2], head[3]);
    let mut w = worldgen::World::new(0, 0);
    for cx in x0 - 1..=x1 + 1 {
        for cz in z0 - 1..=z1 + 1 {
            let mut c = Chunk::new(cx, cz, vec![0; 32768].into_boxed_slice().try_into().unwrap());
            for x in 0..16 {
                for z in 0..16 {
                    c.blocks[idx(x, 63, z)] = 1;
                }
            }
            c.loaded = true;
            c.populated = true;
            w.chunks.insert((cx, cz), Box::new(c));
            w.chunk_list.push((cx, cz));
        }
    }
    for l in lines {
        let v: Vec<i32> = l.split(' ').map(|v| v.parse().unwrap()).collect();
        let c = w.chunk_mut(v[0] >> 4, v[2] >> 4);
        c.blocks[idx(v[0] & 15, v[1], v[2] & 15)] = v[3] as u8;
        c.data.set(v[0] & 15, v[1], v[2] & 15, v[4]);
    }
    for cx in x0 - 1..=x1 + 1 {
        for cz in z0 - 1..=z1 + 1 {
            let c = w.chunk_mut(cx, cz);
            for x in 0..16 {
                for y in 0..128 {
                    for z in 0..16 {
                        let (wx, wz) = (cx * 16 + x, cz * 16 + z);
                        c.sky_light.set(x, y, z, (wx * 7 + y * 3 + wz * 5) & 15);
                        c.block_light.set(x, y, z, (wx * 3 + y * 11 + wz) & 15);
                    }
                }
            }
        }
    }
    w
}

/// `--meshcheck seed wrand oracle-mesh.bin [fancy|fast] [on|off] [nether|showcase=FILE]`: mesh the same chunks after the vanilla
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
    video.anaglyph = a.iter().any(|s| s == "anaglyph");
    // `nether` after the graphics and AO words: the same preload in the Nether.
    let nether = a.get(7).map(String::as_str) == Some("nether");
    let showcase = a.get(7).and_then(|s| s.strip_prefix("showcase="));
    let mut world = if let Some(path) = showcase {
        showcase_world(path)
    } else if nether {
        let mut w = worldgen::World::with_dimension(seed, wrand, worldgen::world::Dimension::Nether);
        w.find_initial_spawn();
        w.preload_spawn_area();
        w
    } else {
        worldgen::World::new_vanilla(seed, wrand)
    };
    world.leaves_opaque = !video.fancy;
    let biomes = std::cell::RefCell::new(if nether { BiomeSource::hell() } else { BiomeSource::new(seed) });
    let magic = i32_();
    let _ = i32_();
    assert_eq!(magic, i32::from_le_bytes(*b"B181"), "not a b1.8.1 mesh dump");
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
                let f: Vec<i32> = (0..7).map(|_| i32_()).collect();
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
                    r.light as i32,
                ];
                if rv[..] != j[..] {
                    if bad < 4 {
                        let f = |v: i32| f32::from_bits(v as u32);
                        eprintln!(
                            "  chunk ({cx},{cz}) pass {pass} vertex {k}: java pos ({},{},{}) uv ({},{}) rgba {:08x} light {:x} | rust pos ({},{},{}) uv ({},{}) rgba {:08x} light {:x}",
                            f(j[0]), f(j[1]), f(j[2]), f(j[3]), f(j[4]), j[5], j[6],
                            r.pos[0], r.pos[1], r.pos[2], r.uv[0], r.uv[1], rv[5], rv[6]
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

/// The whole b1.8.1 program (`beta --mc b1.8.1`), with the process arguments.
pub fn run() {
    // beta.exe passes `--mc b1.8.1` through; drop it so positional modes see their usual layout.
    let mut args: Vec<String> = std::env::args().collect();
    if let Some(i) = args.iter().position(|a| a == "--mc") {
        args.drain(i..(i + 2).min(args.len()));
    }
    let _ = std::fs::create_dir_all(DATA_DIR);
    match args.get(1).map(String::as_str) {
        Some("--version" | "-V") => {
            println!("beta-rs {} (Minecraft Beta 1.8.1)", env!("CARGO_PKG_VERSION"));
            return;
        }
        Some("--help" | "-h") => {
            print!("{}", HELP);
            return;
        }
        _ => {}
    }
    if args.get(1).map(String::as_str) == Some("--skydrawcheck") {
        render::video_check::sky_draw(&args[2]);
        return;
    }
    if args.get(1).map(String::as_str) == Some("--cloudcheck") {
        render::video_check::clouds(&args);
        return;
    }
    if args.get(1).map(String::as_str) == Some("--videowindow") {
        let video = render::video::VideoSettings { distance: args[2].parse().unwrap(), ..Default::default() };
        let xs = worker::window_axis(args[3].parse().unwrap(), video.chunks_wide());
        println!("window {} {}", video.chunks_wide(), xs.iter().map(i32::to_string).collect::<Vec<_>>().join(" "));
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
    if args.get(1).map(String::as_str) == Some("--texturecheck") {
        match render::texture_check::check(args.get(2).expect("--texturecheck oracle.bin")) {
            Ok(()) => return,
            Err(e) => { eprintln!("{e}"); std::process::exit(1); }
        }
    }
    if args.get(1).map(String::as_str) == Some("--firstpersoncheck") {
        let ok =
            render::first_person_check::check(args.get(2).expect("--firstpersoncheck oracle.bin"));
        std::process::exit(if ok { 0 } else { 1 });
    }
    if args.get(1).map(String::as_str) == Some("--pistoncheck") {
        let ok=render::piston_check::check(args.get(2).expect("--pistoncheck oracle dump"));
        println!("{}",if ok {"PISTON RENDERERS EXACT"} else {"PISTON RENDERERS DIFFER"});
        std::process::exit(if ok { 0 } else { 1 });
    }
    if args.get(1).map(String::as_str) == Some("--vehiclerenderercheck") {
        let ok=render::entity::object_check::vehicle_check(args.get(2).expect("oracle vehicle renderer dump"));
        println!("{}",if ok {"VEHICLE RENDERERS EXACT"} else {"VEHICLE RENDERERS DIFFER"});
        std::process::exit(if ok {0} else {1});
    }
    if args.get(1).map(String::as_str) == Some("--renderercheck") {
        let ok = render::entity::object_check::check(args.get(2).expect("--renderercheck oracle-renderers.bin"));
        println!("{}", if ok { "RENDERERS EXACT" } else { "RENDERERS DIFFER" });
        std::process::exit(if ok { 0 } else { 1 });
    }
    if args.get(1).map(String::as_str)==Some("--renderlistcheck") {std::process::exit(if render::render_list::check(&args[2]) {0}else{1});}
    if args.get(1).map(String::as_str)==Some("--fishrenderercheck") {std::process::exit(if render::entity::object_check::fish_check(&args[2]) {0}else{1});}
    if args.get(1).map(String::as_str)==Some("--screenshotcheck") {
        let ok=screenshot::check(std::path::Path::new(&args[2]),&args[3]);std::process::exit(if ok {0}else{1});
    }
    if args.get(1).map(String::as_str)==Some("--signrenderercheck") {
        let ok=render::entity::object_check::sign_check(&args[2]);std::process::exit(if ok {0}else{1});
    }
    if args.get(1).map(String::as_str)==Some("--paintingrenderercheck") {
        let ok=render::entity::object_check::painting_check(&args[2]);std::process::exit(if ok {0}else{1});
    }
    if args.get(1).map(String::as_str) == Some("--modelcheck") {
        let ok = render::entity::check::model_check(args.get(2).expect("--modelcheck oracle-models.bin"));
        println!("{}", if ok { "MODELS EXACT" } else { "MODELS DIFFER" });
        std::process::exit(if ok { 0 } else { 1 });
    }
    // `--statsfile write <dir> id=n,id=n...` writes <dir>/stats/stats_player_unsent.dat;
    // `--statsfile read <file>` prints the stats a file holds, sorted ("null" if rejected).
    if args.get(1).map(String::as_str) == Some("--statsfile") {
        let map = assets::Jar::open().and_then(|mut j| j.text("achievement/map.txt")).unwrap_or_default();
        match args.get(2).map(String::as_str) {
            Some("write") => {
                let _ = std::fs::create_dir_all(&args[3]);
                let mut stats = gui::stats::Stats::open(std::path::Path::new(&args[3]), "Player", &map);
                for kv in args.get(4).map_or("", String::as_str).split(',').filter(|s| !s.is_empty()) {
                    let (k, v) = kv.split_once('=').expect("id=n");
                    stats.add(k.parse().expect("stat id"), v.parse().expect("stat value"));
                }
                stats.sync();
            }
            _ => match gui::stats_file::Syncher::read_file(std::path::Path::new(&args[3]), &map) {
                None => println!("null"),
                Some(m) => {
                    let mut rows: Vec<_> = m.into_iter().collect();
                    rows.sort_unstable();
                    for (k, v) in rows {
                        println!("{k} {v}");
                    }
                }
            },
        }
        return;
    }
    if args.get(1).map(String::as_str)==Some("--worldseedcheck"){std::process::exit(if gui::screens::select_world::check_seeds(&args[2]) {0}else{1});}
    if args.get(1).map(String::as_str)==Some("--worldcallbackcheck"){std::process::exit(if gui::screens::select_world::check_callbacks(&args[2]) {0}else{1});}
    if args.get(1).map(String::as_str)==Some("--guiclosecheck"){std::process::exit(if gui::container_check::check(&args[2]) {0}else{1});}
    if args.get(1).map(String::as_str)==Some("--inventoryguicheck"){std::process::exit(if gui::inventory_check::check(&args[2]) {0}else{1});}
    if args.get(1).map(String::as_str)==Some("--guiframebuffercheck"){std::process::exit(if gui::framebuffer_check9::check(&args[2]) {0}else{1});}
    if args.get(1).map(String::as_str)==Some("--guislot8check"){std::process::exit(if gui::slot::check(&args[2]) {0}else{1});}
    if args.get(1).map(String::as_str)==Some("--yesno8check"){std::process::exit(if gui::screens::select_world::check_yes_no(&args[2]) {0}else{1});}
    if args.get(1).map(String::as_str)==Some("--rename8check"){std::process::exit(if gui::screens::select_world::check_rename(&args[2]) {0}else{1});}
    if args.get(1).map(String::as_str)==Some("--select8check"){std::process::exit(if gui::screens::select_world::check_select(&args[2]) {0}else{1});}
    if args.get(1).map(String::as_str)==Some("--stats8check"){std::process::exit(if gui::screens::stats::check(&args[2]) {0}else{1});}
    if args.get(1).map(String::as_str)==Some("--statsordercheck"){std::process::exit(if gui::screens::stats::check_order(&args[2]) {0}else{1});}
    if args.get(1).map(String::as_str)==Some("--generalstatscheck"){std::process::exit(if gui::screens::stats::check_general(&args[2]) {0}else{1});}
    if args.get(1).map(String::as_str)==Some("--keyregistrycheck"){std::process::exit(if gui::settings::check_key_registry(&args[2]) {0}else{1});}
    if args.get(1).map(String::as_str)==Some("--guiutilitycheck"){std::process::exit(if gui::utility_check::check(&args[2]) {0}else{1});}
    if args.get(1).map(String::as_str)==Some("--statsynccheck"){std::process::exit(if gui::stats_check::check(std::path::Path::new(&args[2])) {0}else{1});}
    if args.get(1).map(String::as_str)==Some("--playerguicheck"){std::process::exit(if gui::player_check::check(&args[2]) {0}else{1});}
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
        println!("{}", if ok { "GUI MATCHES" } else { "GUI DIFFERS" });
        std::process::exit(if ok { 0 } else { 1 });
    }
    if args.get(1).map(String::as_str)==Some("--particlefxcheck") {let ok=render::particle_check::effects_check(args.get(2).expect("--particlefxcheck oracle.bin"));println!("{}",if ok {"EFFECT RENDERER EXACT"}else{"EFFECT RENDERER DIFFER"});std::process::exit(if ok {0}else{1});}
    if args.get(1).map(String::as_str) == Some("--modelregistrycheck") {
        std::process::exit(if render::entity::check::registry_check(&args[2]) {0} else {1});
    }
    if args.get(1).map(String::as_str) == Some("--arrowcheck") {std::process::exit(if render::entity::object_check::arrow_check(&args[2]) {0}else{1});}
    if args.get(1).map(String::as_str) == Some("--bedmodelcheck") {std::process::exit(if render::entity::object_check::bed_check(&args[2]) {0}else{1});}
    if args.get(1).map(String::as_str) == Some("--particlefactorycheck") { std::process::exit(if render::particle_check::factory_check(&args[2]) {0}else{1}); }
    if args.get(1).map(String::as_str) == Some("--effectlifecyclecheck") { std::process::exit(if render::particle_check::lifecycle_check(&args[2]) {0}else{1}); }
    if args.get(1).map(String::as_str) == Some("--effectdispatch8check") { std::process::exit(if render::effect_check8::check(&args[2]) {0}else{1}); }
    if args.get(1).map(String::as_str) == Some("--playerlivingcheck") { std::process::exit(if render::entity::object_check::player_living_check(&args[2]) {0}else{1}); }
    if args.get(1).map(String::as_str) == Some("--newlivingcheck") { std::process::exit(if render::entity::object_check::new_living_check(&args[2]) {0}else{1}); }
    if args.get(1).map(String::as_str) == Some("--chestcheck") {
        std::process::exit(if render::entity::object_check::chest_check(&args[2]) {0} else {1});
    }
    if args.get(1).map(String::as_str) == Some("--particleemittercheck") {
        std::process::exit(if render::particle_check::emitter_check(&args[2]) {0} else {1});
    }
    if args.get(1).map(String::as_str) == Some("--particlecheck") {
        let ok=render::particle_check::check(args.get(2).expect("--particlecheck oracle.bin"));
        println!("{}",if ok {"PARTICLES EXACT"}else{"PARTICLES DIFFER"});std::process::exit(if ok {0}else{1});
    }
    if args.get(1).map(String::as_str) == Some("--weathercheck") {
        render::weather_check::check(&args);
        return;
    }
    if args.get(1).map(String::as_str) == Some("--soundcheck") {
        std::process::exit(if sound::check() { 0 } else { 1 });
    }
    if args.get(1).map(String::as_str) == Some("--muscheck") {
        std::process::exit(if sound::check_mus(args.get(2).expect("--muscheck oracle.bin")) { 0 } else { 1 });
    }
    if matches!(args.get(1).map(String::as_str), Some("--skycolor") | Some("--skycolor-nether")) {
        sky_check(&args);
        return;
    }
    if args.get(1).map(String::as_str) == Some("--lightmapcheck") {
        let ok = render::lightmap::check(&args[2]);
        std::process::exit(if ok { 0 } else { 1 });
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
