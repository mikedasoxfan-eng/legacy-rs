//! The world thread: owns the `World`, runs it at 20 ticks per second, loads chunks the
//! way singleplayer or the multiplayer server does, and builds meshes for the renderer
//! window (`RenderGlobal`).

use crate::render::access::Access;
use crate::render::blocks::{Colorizers, RenderBlocks};
use crate::render::{rprops, Tessellator, Vertex};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::{Duration, Instant};
use worldgen::world::server::PlayerManager;
use worldgen::{BiomeSource, World};

pub enum ToWorker {
    Camera { x: f64, y: f64, z: f64 },
    AddTime(i64),
    Quit,
}

pub enum FromWorker {
    Ready { spawn: (f64, f64, f64), seed: i64, name: String },
    Mesh { cx: i32, cz: i32, opaque: Vec<Vertex>, translucent: Vec<Vertex> },
    Drop { cx: i32, cz: i32 },
    Time(i64),
    /// Everything in the renderer window is loaded and meshed.
    Idle,
    Saved,
    Error(String),
}

#[derive(Clone)]
pub struct WorldOptions {
    pub dir: Option<PathBuf>,
    pub seed: i64,
    pub world_rand: i64,
    /// Multiplayer-server chunk loading/unloading instead of singleplayer's.
    pub server: bool,
    pub view_distance: i32,
}

/// `RenderGlobal`: 26 renderers across on Far (render distance capped at 400 blocks).
pub const RENDER_CHUNKS_WIDE: i32 = 26;

pub fn spawn(opts: WorldOptions, col: Colorizers) -> (Sender<ToWorker>, Receiver<FromWorker>, std::thread::JoinHandle<()>) {
    let (to_tx, to_rx) = mpsc::channel();
    let (from_tx, from_rx) = mpsc::channel();
    let h = std::thread::Builder::new()
        .name("world".into())
        .spawn(move || run(opts, col, to_rx, from_tx))
        .expect("spawn world thread");
    (to_tx, from_rx, h)
}

/// `RenderGlobal.markRenderersForNewPosition` along one axis: the chunk coordinates the
/// renderer window covers around block position `p`.
fn window_axis(p: i32) -> Vec<i32> {
    let p = p - 8;
    let w = RENDER_CHUNKS_WIDE * 16;
    let half = w / 2;
    (0..RENDER_CHUNKS_WIDE)
        .map(|i| {
            let mut x = i * 16;
            let mut k = x + half - p;
            if k < 0 {
                k -= w - 1;
            }
            k /= w;
            x -= k * w;
            x >> 4
        })
        .collect()
}

fn run(opts: WorldOptions, col: Colorizers, rx: Receiver<ToWorker>, tx: Sender<FromWorker>) {
    let t = Instant::now();
    let (mut world, is_new) = match World::open(opts.dir.as_deref(), opts.seed, opts.world_rand, opts.server) {
        Ok(w) => w,
        Err(e) => {
            let _ = tx.send(FromWorker::Error(format!("could not open world: {e}")));
            return;
        }
    };
    let (sx, _, sz) = world.spawn;
    // The saved player, if the real game left one; otherwise stand on the spawn column.
    let mut start = None;
    if let Some(p) = &world.info.player {
        if let [worldgen::nbt::Tag::Double(x), worldgen::nbt::Tag::Double(y), worldgen::nbt::Tag::Double(z)] = p.list("Pos") {
            start = Some((*x, *y + 1.62, *z));
        }
    }
    let start = start.unwrap_or_else(|| {
        let mut top = 127;
        while top > 0 && world.get_block_id(sx, top, sz) == 0 {
            top -= 1;
        }
        (sx as f64 + 0.5, top as f64 + 1.0 + 1.62, sz as f64 + 0.5)
    });
    eprintln!(
        "world: {} '{}', seed {}, spawn {:?}, {} chunks ready in {:.2?}{}",
        if is_new { "created" } else { "loaded" },
        world.info.name,
        world.seed,
        world.spawn,
        world.chunks.len(),
        t.elapsed(),
        if opts.server { " (server chunk handling)" } else { "" }
    );
    let _ = tx.send(FromWorker::Ready { spawn: start, seed: world.seed, name: world.info.name.clone() });

    let mut players = if opts.server {
        let mut pm = PlayerManager::new(opts.view_distance.clamp(3, 15));
        pm.add_player(&mut world, start.0, start.2);
        Some(pm)
    } else {
        None
    };

    let biomes = RefCell::new(BiomeSource::new(world.seed));
    let mut cam = start;
    let mut sort_pos = (f64::MAX, f64::MAX, f64::MAX);
    let mut window: Vec<(i32, i32)> = Vec::new();
    let mut window_set: HashSet<(i32, i32)>;
    let mut meshed: HashMap<(i32, i32), u64> = HashMap::new();
    let mut sky_sub = world.skylight_subtracted;
    let mut join_player_counter = 0;
    let mut next_tick = Instant::now();
    let mut idle_reported = false;

    loop {
        // Messages.
        loop {
            match rx.try_recv() {
                Ok(ToWorker::Camera { x, y, z }) => cam = (x, y, z),
                Ok(ToWorker::AddTime(t)) => world.world_time += t,
                Ok(ToWorker::Quit) | Err(mpsc::TryRecvError::Disconnected) => {
                    // Minecraft quitting to the title screen / server shutdown: save everything.
                    if let Some(pm) = &mut players {
                        pm.remove_player(&mut world);
                    }
                    world.save_world(true);
                    let _ = tx.send(FromWorker::Saved);
                    return;
                }
                Err(mpsc::TryRecvError::Empty) => break,
            }
        }

        // Ticks (at most a second's worth of catch-up).
        let mut ticks = 0;
        while Instant::now() >= next_tick && ticks < 20 {
            world.players.clear();
            world.players.push(cam);
            if players.is_none() {
                // Minecraft.runTick: pull in the chunks around the player every 30 ticks.
                join_player_counter += 1;
                if join_player_counter == 30 {
                    join_player_counter = 0;
                    world.join_player_surroundings(cam.0, cam.2);
                }
            }
            world.tick();
            // The client relights a little every frame and the server once per tick; here
            // the queue is simply emptied.
            world.drain_lighting();
            if let Some(pm) = &mut players {
                // MinecraftServer.doTick: world tick, lighting, then player movement packets.
                pm.update_player(&mut world, cam.0, cam.2);
            }
            if world.renderers_dirty {
                // skylightSubtracted changed (dusk, dawn, rain): updateAllRenderers.
                world.renderers_dirty = false;
                sky_sub = world.skylight_subtracted;
                meshed.clear();
                idle_reported = false;
            }
            let _ = tx.send(FromWorker::Time(world.world_time));
            next_tick += Duration::from_millis(50);
            ticks += 1;
        }
        if ticks == 20 {
            next_tick = Instant::now();
        }

        // Unloaded chunks lose their meshes.
        for (cx, cz) in std::mem::take(&mut world.unloaded) {
            if meshed.remove(&(cx, cz)).is_some() {
                let _ = tx.send(FromWorker::Drop { cx, cz });
            }
            idle_reported = false;
        }

        // Renderer window: re-centred after moving more than 4 blocks.
        let (dx, dy, dz) = (cam.0 - sort_pos.0, cam.1 - sort_pos.1, cam.2 - sort_pos.2);
        if dx * dx + dy * dy + dz * dz > 16.0 {
            sort_pos = cam;
            let xs = window_axis(cam.0.floor() as i32);
            let zs = window_axis(cam.2.floor() as i32);
            window.clear();
            for &x in &xs {
                for &z in &zs {
                    window.push((x, z));
                }
            }
            window_set = window.iter().copied().collect();
            let gone: Vec<(i32, i32)> = meshed.keys().filter(|k| !window_set.contains(k)).copied().collect();
            for (cx, cz) in gone {
                meshed.remove(&(cx, cz));
                let _ = tx.send(FromWorker::Drop { cx, cz });
            }
            idle_reported = false;
        }
        let (ccx, ccz) = ((cam.0.floor() as i32) >> 4, (cam.2.floor() as i32) >> 4);
        window.sort_by_key(|&(x, z)| (x - ccx) * (x - ccx) + (z - ccz) * (z - ccz));

        // Load (singleplayer) and mesh, nearest first, until the next tick is due.
        let mut did_work = false;
        for i in 0..window.len() {
            if Instant::now() >= next_tick {
                did_work = true;
                break;
            }
            let (cx, cz) = window[i];
            let mut complete = true;
            for nx in cx - 1..=cx + 1 {
                for nz in cz - 1..=cz + 1 {
                    if !world.chunk_exists(nx, nz) {
                        if players.is_none() {
                            // Singleplayer renderers pull chunks in through ChunkCache.
                            world.load_chunk(nx, nz);
                            world.drain_lighting();
                            did_work = true;
                        } else {
                            complete = false;
                        }
                    }
                }
            }
            if !complete {
                continue;
            }
            let mut version = 0;
            for nx in cx - 1..=cx + 1 {
                for nz in cz - 1..=cz + 1 {
                    version = version.max(world.chunk(nx, nz).version);
                }
            }
            if meshed.get(&(cx, cz)).is_some_and(|&v| v >= version) {
                continue;
            }
            let (opaque, translucent) = mesh_chunk(&world, cx, cz, sky_sub, &biomes, &col);
            meshed.insert((cx, cz), version);
            did_work = true;
            if tx.send(FromWorker::Mesh { cx, cz, opaque, translucent }).is_err() {
                return;
            }
        }
        if !did_work {
            if !idle_reported {
                idle_reported = true;
                let _ = tx.send(FromWorker::Idle);
            }
            let wait = next_tick.saturating_duration_since(Instant::now()).min(Duration::from_millis(50));
            std::thread::sleep(wait);
        }
    }
}

/// `WorldRenderer.updateRenderer` for a whole 16x128x16 chunk: both render passes.
pub fn mesh_chunk(
    world: &World,
    cx: i32,
    cz: i32,
    sky_sub: i32,
    biomes: &RefCell<BiomeSource>,
    col: &Colorizers,
) -> (Vec<Vertex>, Vec<Vertex>) {
    let access = Access::new(world, cx, cz, sky_sub, biomes);
    let origin = [cx as f64 * 16.0, 0.0, cz as f64 * 16.0];
    let mut t0 = Tessellator::new(origin);
    let mut t1 = Tessellator::new(origin);
    let chunk = world.chunk(cx, cz);
    for y in 0..128 {
        for z in 0..16 {
            for x in 0..16 {
                let id = chunk.block(x, y, z);
                if id == 0 || !rprops(id).exists {
                    continue;
                }
                let t = if rprops(id).pass == 0 { &mut t0 } else { &mut t1 };
                let mut rb = RenderBlocks::new(&access, t, col);
                rb.render_block(id, cx * 16 + x, y, cz * 16 + z);
            }
        }
    }
    (t0.verts, t1.verts)
}
