//! Diff the Rust generator against an oracle dump.
//!
//!   compare rawnether <seed> <oracle.bin>
//!   compare gridnether <seed> <wrand> <x0> <z0> <x1> <z1> <each|end|none> <oracle.bin>
//!   compare raw <seed> <oracle.bin>
//!   compare sin <oracle-sin.bin>
//!   compare grid <seed> <wrand> <x0> <z0> <x1> <z1> <each|end|none> <oracle.bin>
//!   compare preload <seed> <wrand> <oracle.bin> [fancy]
//!   compare ticks <seed> <wrand> <lcg> <sound> <n> <clear|rain|thunder> <dx> <dz> <oracle.bin> [fires]

use std::collections::BTreeMap;
use std::process::ExitCode;
use worldgen::scenario::{self, LightMode};
use worldgen::world::tile::TileEntity;
use worldgen::{math, OverworldGenerator, World};

const CHUNK_RECORD: usize = 8 + 32768 + 16384 + 256 + 256 * 16;

fn main() -> ExitCode {
    let a: Vec<String> = std::env::args().collect();
    let p = |i: usize| a[i].parse::<i64>().expect("number");
    match a.get(1).map(String::as_str) {
        Some("raw") => raw(p(2), &a[3]),
        Some("rawnether") => raw_nether(p(2), &a[3]),
        Some("sin") => sin(&a[2]),
        Some("math") => java_math(&a[2]),
        Some("grid") | Some("gridnether") => {
            let light = match a[8].as_str() {
                "each" => LightMode::Each,
                "end" => LightMode::End,
                _ => LightMode::None,
            };
            let dimension = if a[1] == "gridnether" {
                worldgen::world::Dimension::Nether
            } else {
                worldgen::world::Dimension::Overworld
            };
            let mut w = World::with_dimension(p(2), p(3), dimension);
            let t = std::time::Instant::now();
            scenario::grid(&mut w, p(4) as i32, p(5) as i32, p(6) as i32, p(7) as i32, light);
            eprintln!("rust: {} chunks in {:.2?}", w.chunks.len(), t.elapsed());
            world(&w, &a[9])
        }
        Some("serverwalk") => {
            // seed wrand radius dir ticks java.bin: the server oracle's scripted walk.
            let dir = std::path::PathBuf::from(&a[5]);
            let (mut w, _) = World::open(Some(&dir), p(2), p(3), true).expect("open");
            let mut pm = worldgen::world::server::PlayerManager::new(p(4) as i32);
            let (sx, sz) = (w.spawn.0 as f64 + 0.5, w.spawn.2 as f64 + 0.5);
            pm.add_player(&mut w, sx, sz);
            // The oracle's player is Unsafe-allocated: posY stays 0.
            w.players.push((sx, 0.0, sz));
            w.update_lcg = 12345;
            w.sound_counter = 100;
            for t in 0..p(6) as i32 {
                w.tick();
                w.drain_lighting();
                let (x, z) = walk_path(sx, sz, t);
                w.players[0] = (x, 0.0, z);
                pm.update_player(&mut w, x, z);
            }
            pm.remove_player(&mut w);
            w.save_world(true);
            println!("rust: {} chunks loaded at the end", w.chunks.len());
            world(&w, &a[7])
        }
        Some("makesave") => {
            // Create a world in a save directory the way the viewer does (preload, then save).
            let (w, new) = World::open(Some(std::path::Path::new(&a[4])), p(2), p(3), false).expect("open");
            println!("{} world, {} chunks, spawn {:?}", if new { "new" } else { "existing" }, w.chunks.len(), w.spawn);
            ExitCode::SUCCESS
        }
        Some("preload-worldonly") => {
            let mut w = World::new(p(2), p(3));
            w.find_initial_spawn();
            w.preload_spawn_area();
            WORLD_ONLY.store(true, std::sync::atomic::Ordering::Relaxed);
            world(&w, &a[4])
        }
        Some("savecheck") => {
            // Load every chunk listed in the dump from a save directory, compare (saves keep
            // no scheduled ticks or pending light updates, so those are skipped).
            let dir = std::path::Path::new(&a[2]);
            let storage = worldgen::storage::Storage::open(dir).expect("open save");
            let info = storage.load_level().expect("level.dat");
            let mut w = World::new(info.seed, 0);
            w.spawn = info.spawn;
            w.world_time = info.time;
            w.storage = Some(storage);
            let data = std::fs::read(&a[3]).expect("read dump");
            for (cx, cz) in dump_chunk_list(&data) {
                w.load_chunk(cx, cz);
            }
            WORLD_ONLY.store(true, std::sync::atomic::Ordering::Relaxed);
            world(&w, &a[3])
        }
        Some("ticks") => {
            let mut w = World::new(p(2), p(3));
            w.find_initial_spawn();
            w.preload_spawn_area();
            w.update_lcg = p(4) as i32;
            w.sound_counter = p(5) as i32;
            if a[7] != "clear" {
                w.info.raining = true;
                w.info.rain_time = 1000000;
                w.raining_strength = 1.0;
                w.prev_raining_strength = 1.0;
                if a[7] == "thunder" {
                    w.info.thundering = true;
                    w.info.thunder_time = 1000000;
                    w.thundering_strength = 1.0;
                    w.prev_thundering_strength = 1.0;
                }
            }
            w.skylight_subtracted = w.calculate_skylight_subtracted(1.0);
            let f = |i: usize| a[i].parse::<f64>().expect("number");
            let (sx, sy, sz) = w.spawn;
            w.players.push((sx as f64 + 0.5 + f(8), sy as f64 + 1.62, sz as f64 + 0.5 + f(9)));
            for f in a.get(11).map_or("", String::as_str).split(',').filter(|f| !f.is_empty()) {
                let (fx, fz) = f.split_once(':').expect("dx:dz");
                let (fx, fz) = (sx + fx.parse::<i32>().unwrap(), sz + fz.parse::<i32>().unwrap());
                let fy = w.get_height_value(fx, fz);
                w.set_block_with_notify(fx, fy, fz, worldgen::block::FIRE);
            }
            w.drain_lighting();
            let t = std::time::Instant::now();
            for _ in 0..p(6) {
                w.tick();
                w.drain_lighting();
            }
            eprintln!("rust: {} ticks, {} chunks in {:.2?}", p(6), w.chunks.len(), t.elapsed());
            let counts: Vec<String> = (0..256)
                .filter_map(|i| {
                    let n = worldgen::world::behavior::TICK_COUNTS[i].load(std::sync::atomic::Ordering::Relaxed);
                    (n > 0).then(|| format!("{}={n}", worldgen::block::props(i as u8).name.trim_start_matches("tile.")))
                })
                .collect();
            eprintln!("block ticks: {}", counts.join(" "));
            let state = format!(
                "time {}\nlcg {}\nsound {}\nrain {} {} {}\nthunder {} {} {}\nskylight {}\nflash {} bolts {}\nrand {}\n",
                w.world_time,
                w.update_lcg,
                w.sound_counter,
                w.info.raining,
                w.info.rain_time,
                w.raining_strength.to_bits() as i32,
                w.info.thundering,
                w.info.thunder_time,
                w.thundering_strength.to_bits() as i32,
                w.skylight_subtracted,
                w.lightning_flash,
                w.lightning_strikes.len(),
                w.rand.next_long()
            );
            let java = std::fs::read_to_string(format!("{}.txt", a[10])).expect("read state").replace('\r', "");
            let state_ok = java == state;
            if !state_ok {
                eprintln!("tick state differs:\n--- java\n{java}--- rust\n{state}");
            }
            let r = world(&w, &a[10]);
            if state_ok { r } else { ExitCode::FAILURE }
        }
        Some("preload") => {
            let t = std::time::Instant::now();
            let mut w = World::new(p(2), p(3));
            w.leaves_opaque = a.get(5).map(String::as_str) != Some("fancy");
            w.find_initial_spawn();
            w.preload_spawn_area();
            eprintln!("rust: spawn {:?}, {} chunks in {:.2?}", w.spawn, w.chunks.len(), t.elapsed());
            world(&w, &a[4])
        }
        _ => {
            eprintln!("see header of compare.rs for usage");
            ExitCode::from(2)
        }
    }
}

struct Reader<'a> {
    b: &'a [u8],
    o: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> &'a [u8] {
        let s = &self.b[self.o..self.o + n];
        self.o += n;
        s
    }
    fn u8(&mut self) -> u8 {
        self.take(1)[0]
    }
    fn i16(&mut self) -> i16 {
        i16::from_le_bytes(self.take(2).try_into().unwrap())
    }
    fn i32(&mut self) -> i32 {
        i32::from_le_bytes(self.take(4).try_into().unwrap())
    }
    fn i64(&mut self) -> i64 {
        i64::from_le_bytes(self.take(8).try_into().unwrap())
    }
    fn f64(&mut self) -> f64 {
        f64::from_le_bytes(self.take(8).try_into().unwrap())
    }
}

// ------------------------------------------------------------------ raw terrain

fn raw(seed: i64, path: &str) -> ExitCode {
    let data = std::fs::read(path).expect("read oracle dump");
    assert_eq!(&data[..8], b"B173RAW1", "not a raw oracle dump");
    let body = &data[8..];
    assert_eq!(body.len() % CHUNK_RECORD, 0, "truncated dump");

    let mut gen = OverworldGenerator::new(seed);
    let (mut chunks, mut bad_chunks, mut bad_blocks) = (0, 0, 0usize);
    let (mut bad_biomes, mut bad_climate) = (0usize, 0usize);
    for rec in body.chunks(CHUNK_RECORD) {
        let mut r = Reader { b: rec, o: 0 };
        let (cx, cz) = (r.i32(), r.i32());
        let blocks = r.take(32768);
        let _data = r.take(16384);
        let biomes = r.take(256);
        let chunk = gen.generate(cx, cz);
        chunks += 1;

        // Climate arrays still hold this chunk's 16x16 values right after generate().
        let temps: Vec<f64> = (0..256).map(|_| r.f64()).collect();
        let humid: Vec<f64> = (0..256).map(|_| r.f64()).collect();
        for i in 0..256 {
            let (t, h) = (temps[i], humid[i]);
            if t.to_bits() != gen.biome_source.temperature[i].to_bits()
                || h.to_bits() != gen.biome_source.humidity[i].to_bits()
            {
                if bad_climate == 0 {
                    eprintln!(
                        "climate mismatch chunk ({cx},{cz}) col {i}: java t={t:e} h={h:e}, rust t={:e} h={:e}",
                        gen.biome_source.temperature[i], gen.biome_source.humidity[i]
                    );
                }
                bad_climate += 1;
            }
            if biomes[i] != chunk.biomes[i] as u8 {
                bad_biomes += 1;
            }
        }

        let mut diff = 0;
        for (i, (&j, &r)) in blocks.iter().zip(chunk.blocks.iter()).enumerate() {
            if j != r {
                if diff < 5 {
                    let (x, z, y) = (i >> 11, (i >> 7) & 15, i & 127);
                    eprintln!("chunk ({cx},{cz}) block x={x} y={y} z={z}: java={j} rust={r}");
                }
                diff += 1;
            }
        }
        if diff > 0 {
            bad_chunks += 1;
            bad_blocks += diff;
        }
    }

    println!(
        "seed {seed}: {chunks} chunks, {bad_chunks} differ ({bad_blocks} blocks), \
         {bad_biomes} biome columns differ, {bad_climate} climate values differ"
    );
    verdict(bad_chunks == 0 && bad_biomes == 0 && bad_climate == 0)
}

fn raw_nether(seed: i64, path: &str) -> ExitCode {
    let data = std::fs::read(path).expect("read oracle dump");
    assert_eq!(&data[..8], b"B173RAW1", "not a raw oracle dump");
    let body = &data[8..];
    assert_eq!(body.len() % CHUNK_RECORD, 0, "truncated dump");

    let mut gen = worldgen::nether::NetherGenerator::new(seed);
    let (mut chunks, mut bad_chunks, mut bad_blocks) = (0, 0, 0usize);
    let (mut bad_biomes, mut bad_climate, mut bad_metadata) = (0usize, 0usize, 0usize);
    let mut climate = worldgen::biome::BiomeSource::hell();
    let mut biome_buffer = Vec::new();
    for rec in body.chunks(CHUNK_RECORD) {
        let mut r = Reader { b: rec, o: 0 };
        let (cx, cz) = (r.i32(), r.i32());
        let blocks = r.take(32768);
        let metadata = r.take(16384);
        let metadata_diff = metadata.iter().filter(|&&v| v != 0).count();
        bad_metadata += metadata_diff;
        if metadata_diff > 0 {
            eprintln!("chunk ({cx},{cz}): {metadata_diff} metadata bytes differ");
        }
        let biomes = r.take(256);
        let chunk = gen.generate(cx, cz);
        chunks += 1;

        // The Nether generator does not call its WorldChunkManager; the oracle
        // explicitly queries the manager after each provideChunk.
        climate.biomes(&mut biome_buffer, cx * 16, cz * 16, 16, 16);
        let temps: Vec<f64> = (0..256).map(|_| r.f64()).collect();
        let humid: Vec<f64> = (0..256).map(|_| r.f64()).collect();
        for i in 0..256 {
            let (t, h) = (temps[i], humid[i]);
            if t.to_bits() != climate.temperature[i].to_bits() || h.to_bits() != climate.humidity[i].to_bits() {
                if bad_climate == 0 {
                    eprintln!(
                        "climate mismatch chunk ({cx},{cz}) col {i}: java t={t:e} h={h:e}, rust t={:e} h={:e}",
                        climate.temperature[i], climate.humidity[i]
                    );
                }
                bad_climate += 1;
            }
            if biomes[i] != chunk.biomes[i] as u8 || biomes[i] != biome_buffer[i] as u8 {
                bad_biomes += 1;
            }
        }

        let mut diff = 0;
        for (i, (&j, &r)) in blocks.iter().zip(chunk.blocks.iter()).enumerate() {
            if j != r {
                if diff < 5 {
                    let (x, z, y) = (i >> 11, (i >> 7) & 15, i & 127);
                    eprintln!("chunk ({cx},{cz}) block x={x} y={y} z={z}: java={j} rust={r}");
                }
                diff += 1;
            }
        }
        if diff > 0 {
            bad_chunks += 1;
            bad_blocks += diff;
        }
    }

    println!(
        "seed {seed}: {chunks} chunks, {bad_chunks} differ ({bad_blocks} blocks), \
         {bad_biomes} biome columns differ, {bad_climate} climate values differ"
    );
    println!("{bad_metadata} metadata bytes differ");
    verdict(bad_chunks == 0 && bad_metadata == 0 && bad_biomes == 0 && bad_climate == 0)
}

fn verdict(ok: bool) -> ExitCode {
    if ok {
        println!("EXACT MATCH");
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn sin(path: &str) -> ExitCode {
    let data = std::fs::read(path).expect("read sin dump");
    let mut bad = 0;
    for (i, r) in math::sin_table_bits().enumerate() {
        let j = u32::from_le_bytes(data[i * 4..i * 4 + 4].try_into().unwrap());
        if j != r {
            if bad < 5 {
                eprintln!("sin[{i}]: java={:e} rust={:e}", f32::from_bits(j), f32::from_bits(r));
            }
            bad += 1;
        }
    }
    println!("sin table: {bad} of 65536 entries differ");
    if bad == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

// ------------------------------------------------------------------ whole worlds

#[derive(Default)]
struct Tally {
    count: usize,
    shown: usize,
}

impl Tally {
    fn hit(&mut self, what: &str, msg: impl FnOnce() -> String) {
        self.count += 1;
        if self.shown < 8 {
            self.shown += 1;
            eprintln!("  {what}: {}", msg());
        }
    }
}

/// The server oracle's scripted walk (ServerOracle.path).
fn walk_path(sx: f64, sz: f64, t: i32) -> (f64, f64) {
    if t < 600 {
        return (sx + 3.0 * t as f64, sz + 1.0 * t as f64);
    }
    let (x600, z600) = (sx + 3.0 * 600.0, sz + 1.0 * 600.0);
    (x600 - 3.0 * (t - 600) as f64, z600 + 0.5 * (t - 600) as f64)
}

static WORLD_ONLY: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Chunk coordinates in a world dump, in order.
fn dump_chunk_list(data: &[u8]) -> Vec<(i32, i32)> {
    let mut r = Reader { b: data, o: 20 };
    let n = r.i32() as usize;
    let mut out = Vec::new();
    for _ in 0..n {
        out.push((r.i32(), r.i32()));
        r.take(1 + 32768 + 16384 * 3 + 256);
        let nte = r.i32();
        for _ in 0..nte {
            r.take(12);
            match r.u8() {
                1 => {
                    r.take(27 * 5);
                }
                _ => {
                    let len = r.u8() as usize;
                    r.take(len);
                }
            }
        }
    }
    out
}

const BLOCK_NAMES: fn(u8) -> &'static str = |id| worldgen::block::props(id).name;

fn world(w: &World, path: &str) -> ExitCode {
    let data = std::fs::read(path).expect("read world dump");
    let mut r = Reader { b: &data, o: 0 };
    assert_eq!(r.take(8), b"B173WLD1", "not a world dump");
    let spawn = (r.i32(), r.i32(), r.i32());
    let n = r.i32() as usize;

    let mut cats: BTreeMap<&str, Tally> = BTreeMap::new();
    let mut ok = true;
    if spawn != w.spawn {
        eprintln!("spawn differs: java {spawn:?} rust {:?}", w.spawn);
        ok = false;
    }
    if n != w.chunks.len() {
        eprintln!("chunk count differs: java {n} rust {}", w.chunks.len());
        ok = false;
    }

    let mut populated = 0;
    for _ in 0..n {
        let (cx, cz) = (r.i32(), r.i32());
        let pop = r.u8() != 0;
        let blocks = r.take(32768);
        let meta = r.take(16384);
        let sky = r.take(16384);
        let blk = r.take(16384);
        let height = r.take(256);
        let nte = r.i32();
        let mut tes = Vec::new();
        for _ in 0..nte {
            let (x, y, z) = (r.i32(), r.i32(), r.i32());
            let te = match r.u8() {
                1 => {
                    let mut slots = Box::new([None; 27]);
                    for s in slots.iter_mut() {
                        let (id, count, dmg) = (r.i16(), r.u8(), r.i16());
                        if id >= 0 {
                            *s = Some(worldgen::world::tile::ItemStack { id, count, damage: dmg });
                        }
                    }
                    TileEntity::Chest(slots)
                }
                2 => {
                    let len = r.u8() as usize;
                    let mob = String::from_utf8(r.take(len).to_vec()).unwrap();
                    TileEntity::MobSpawner { mob, delay: 20 }
                }
                t => panic!("bad tile entity tag {t}"),
            };
            tes.push(((x, y, z), te));
        }
        populated += pop as usize;

        let Some(c) = w.chunks.get(&(cx, cz)) else {
            cats.entry("missing chunk").or_default().hit("missing chunk", || format!("({cx},{cz})"));
            continue;
        };
        if c.populated != pop {
            cats.entry("populated flag").or_default().hit("populated", || format!("({cx},{cz}) java={pop}"));
        }
        for i in 0..32768 {
            if blocks[i] != c.blocks[i] {
                let (x, z, y) = (cx * 16 + (i >> 11) as i32, cz * 16 + ((i >> 7) & 15) as i32, (i & 127) as i32);
                cats.entry("block").or_default().hit("block", || {
                    format!(
                        "({x},{y},{z}) java={} ({}) rust={} ({})",
                        blocks[i],
                        BLOCK_NAMES(blocks[i]),
                        c.blocks[i],
                        BLOCK_NAMES(c.blocks[i])
                    )
                });
            }
        }
        for (name, java, rust) in [("metadata", meta, &c.data.0[..]), ("skylight", sky, &c.sky_light.0[..]), ("blocklight", blk, &c.block_light.0[..])] {
            for i in 0..16384 {
                if java[i] != rust[i] {
                    let lo = i * 2;
                    let (x, z, y) = (cx * 16 + (lo >> 11) as i32, cz * 16 + ((lo >> 7) & 15) as i32, (lo & 127) as i32);
                    cats.entry(name).or_default().hit(name, || {
                        format!("({x},{y}..{},{z}) java={:#04x} rust={:#04x}", y + 1, java[i], rust[i])
                    });
                }
            }
        }
        if height != &c.height_map[..] {
            cats.entry("heightmap").or_default().hit("heightmap", || format!("chunk ({cx},{cz})"));
        }
        let mut rust_tes: Vec<_> = c
            .tile_entities
            .iter()
            .map(|(&(x, y, z), te)| ((cx * 16 + x, y, cz * 16 + z), te.clone()))
            .collect();
        rust_tes.sort_by_key(|(p, _)| *p);
        let norm = |t: &TileEntity| match t {
            TileEntity::MobSpawner { mob, .. } => TileEntity::MobSpawner { mob: mob.clone(), delay: 20 },
            o => o.clone(),
        };
        let rust_tes: Vec<_> = rust_tes.into_iter().map(|(p, t)| (p, norm(&t))).collect();
        if rust_tes != tes {
            cats.entry("tile entities").or_default().hit("tile entities", || {
                format!("chunk ({cx},{cz}) java={tes:?} rust={rust_tes:?}")
            });
        }
    }

    let nt = r.i32() as usize;
    let mut java_ticks = Vec::new();
    for _ in 0..nt {
        java_ticks.push((r.i32(), r.i32(), r.i32(), r.i32(), r.i64()));
    }
    // Both in processing order (scheduled time, then NextTickListEntry id).
    let rust_ticks: Vec<_> = w.tick_order.iter().map(|&(t, _, k)| (k.x, k.y, k.z, k.block, t)).collect();
    let world_only = WORLD_ONLY.load(std::sync::atomic::Ordering::Relaxed);
    if java_ticks != rust_ticks && !world_only {
        // Entries present with different times, or only on one side (first few).
        let jm: std::collections::HashMap<_, _> = java_ticks.iter().enumerate().map(|(i, t)| ((t.0, t.1, t.2, t.3), (i, t.4))).collect();
        let rm: std::collections::HashMap<_, _> = rust_ticks.iter().enumerate().map(|(i, t)| ((t.0, t.1, t.2, t.3), (i, t.4))).collect();
        let mut shown = 0;
        for (k, &(ji, jt)) in &jm {
            let r = rm.get(k);
            if r.map(|&(_, rt)| rt) != Some(jt) && shown < 8 {
                shown += 1;
                eprintln!("  tick {k:?}: java #{ji} t={jt}, rust {r:?}");
            }
        }
        for (k, r) in &rm {
            if !jm.contains_key(k) && shown < 12 {
                shown += 1;
                eprintln!("  tick {k:?}: rust only {r:?}");
            }
        }
        let first = java_ticks.iter().zip(&rust_ticks).position(|(a, b)| a != b);
        cats.entry("pending ticks").or_default().hit("pending ticks", || {
            format!(
                "java {} entries, rust {}; first difference at #{first:?}: java {:?} rust {:?}",
                java_ticks.len(),
                rust_ticks.len(),
                first.map(|i| java_ticks[i]),
                first.map(|i| rust_ticks[i])
            )
        });
    }
    let java_lq = r.i32() as usize;
    if java_lq != w.light_queue.len() && !world_only {
        cats.entry("light queue").or_default().hit("light queue", || {
            format!("java {java_lq} rust {}", w.light_queue.len())
        });
    }

    println!(
        "{n} chunks ({populated} populated), {} chests/spawners, {} pending ticks",
        w.chunks.values().map(|c| c.tile_entities.len()).sum::<usize>(),
        nt
    );
    for (name, t) in &cats {
        println!("  {name}: {} mismatches", t.count);
        ok = false;
    }
    verdict(ok)
}

/// Our Java-math emulation vs Java's Math/StrictMath sin/cos over every input
/// WorldGenBigTree can produce, plus Math.pow(x, 2.0) vs x*x.
fn java_math(path: &str) -> ExitCode {
    use worldgen::jmath;
    let data = std::fs::read(path).expect("read math dump");
    let mut r = Reader { b: &data, o: 0 };
    let n = 1usize << 24;
    let arg = |i: usize| (i as f32 / (1u32 << 24) as f32) as f64 * 2.0 * 3.14159;
    let read = |r: &mut Reader| (0..n).map(|_| r.f64()).collect::<Vec<f64>>();
    let (msin, mcos, ssin, scos) = (read(&mut r), read(&mut r), read(&mut r), read(&mut r));
    let mut ok = true;
    let checks: [(&str, &Vec<f64>, fn(f64) -> f64); 4] = [
        ("StrictMath.sin vs fdlibm port", &ssin, jmath::fdlibm_sin),
        ("StrictMath.cos vs fdlibm port", &scos, jmath::fdlibm_cos),
        ("Math.sin vs java_sin", &msin, jmath::java_sin),
        ("Math.cos vs java_cos", &mcos, jmath::java_cos),
    ];
    for (name, java, f) in checks {
        let (mut small, mut large, mut shown) = (0, 0, 0);
        for i in 0..n {
            let a = arg(i);
            let v = f(a);
            if v.to_bits() != java[i].to_bits() {
                if a.abs() < std::f64::consts::FRAC_PI_4 { small += 1 } else { large += 1 }
                if shown < 4 {
                    shown += 1;
                    eprintln!("  {name}: x={a:e} java={:e} rust={v:e}", java[i]);
                }
                if std::env::var_os("JMATH_TABLE").is_some() {
                    println!("    ({:#018x}, {:#018x}), // {name} x={a:e}", a.to_bits(), java[i].to_bits());
                }
            }
        }
        let libm = (0..n).filter(|&i| {
            let a = arg(i);
            let v = if name.contains("sin") { a.sin() } else { a.cos() };
            v.to_bits() != java[i].to_bits()
        }).count();
        println!("{name}: {small} differ below pi/4, {large} above (plain libm would differ in {libm})");
        ok &= small + large == 0;
    }
    let (mut bad_mul, mut pows) = (0, 0);
    while r.o < data.len() {
        let (x, java) = (r.f64(), r.f64());
        pows += 1;
        if (x * x).to_bits() != java.to_bits() {
            bad_mul += 1;
        }
    }
    println!("Math.pow(x, 2.0) vs x*x: {bad_mul} of {pows} differ");
    verdict(ok && bad_mul == 0)
}
