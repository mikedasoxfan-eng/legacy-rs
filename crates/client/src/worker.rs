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
    /// `isGamePaused`: a screen that pauses the game is open (no ticks, no catch-up).
    Pause(bool),
    /// `World.func_650_a(n)`: the pause menu saves a few chunks every frame.
    QuickSave(i32),
    Quit,
}

/// `LoadingScreenRenderer` calls made by the world thread.
pub enum Loading {
    Title(String),
    Text(String),
    Progress(i32),
}

pub enum FromWorker {
    Ready {
        spawn: (f64, f64, f64),
        seed: i64,
        name: String,
    },
    Mesh {
        cx: i32,
        cz: i32,
        opaque: Vec<Vertex>,
        translucent: Vec<Vertex>,
    },
    Drop {
        cx: i32,
        cz: i32,
    },
    Time {
        time: i64,
        /// `EntityPlayer.getEntityBrightness` at the camera (vignette), loaded chunk count.
        brightness: f32,
        chunks: usize,
        weather: crate::render::weather::Weather,
        terrain: crate::render::weather::Snapshot,
    },
    /// Everything in the renderer window is loaded and meshed.
    Idle,
    Loading(Loading),
    /// Answer to `QuickSave`: true once nothing was left to save.
    QuickSaved(bool),
    /// Entities near the camera after a tick, with `getEntityBrightness`.
    Entities(Vec<crate::render::entity::effects::Snapshot>),
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
    pub weather: Option<crate::render::weather::Weather>,
    pub time: Option<i64>,
    /// Name for a newly created world (`GuiCreateWorld`'s text box).
    pub name: Option<String>,
}

/// `RenderGlobal`: 26 renderers across on Far (render distance capped at 400 blocks).
pub const RENDER_CHUNKS_WIDE: i32 = 26;

pub fn spawn(
    opts: WorldOptions,
    col: Colorizers,
) -> (Sender<ToWorker>, Receiver<FromWorker>, std::thread::JoinHandle<()>) {
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
    let mut last_progress = -2;
    let mut progress = |step: worldgen::scenario::LoadStep| {
        use worldgen::scenario::LoadStep;
        let msg = match step {
            LoadStep::Title(t) => Loading::Title(t.to_string()),
            LoadStep::Text(t) => {
                last_progress = -2;
                Loading::Text(t.to_string())
            }
            LoadStep::Progress(p) => {
                if p == last_progress {
                    return;
                }
                last_progress = p;
                Loading::Progress(p)
            }
        };
        let _ = tx.send(FromWorker::Loading(msg));
    };
    let opened =
        World::open_with(opts.dir.as_deref(), opts.seed, opts.world_rand, opts.server, opts.name.as_deref(), &mut progress);
    let (mut world, is_new) = match opened {
        Ok(w) => w,
        Err(e) => {
            let _ = tx.send(FromWorker::Error(format!("could not open world: {e}")));
            return;
        }
    };
    if let Some(w) = opts.weather {
        world.info.raining = w.rain > 0.0;
        world.info.thundering = w.thunder > 0.0;
        world.info.rain_time = 1000000;
        world.info.thunder_time = 1000000;
        world.raining_strength = w.rain;
        world.prev_raining_strength = w.prev_rain;
        world.thundering_strength = w.thunder;
        world.prev_thundering_strength = w.prev_thunder;
        world.lightning_flash = w.flash;
    }
    if let Some(time) = opts.time {
        world.world_time = time;
    }
    world.skylight_subtracted = world.calculate_skylight_subtracted(1.0);
    let (sx, _, sz) = world.spawn;
    // The saved player, if the real game left one; otherwise stand on the spawn column.
    let mut start = None;
    if let Some(p) = &world.info.player {
        if let [worldgen::nbt::Tag::Double(x), worldgen::nbt::Tag::Double(y), worldgen::nbt::Tag::Double(z)] =
            p.list("Pos")
        {
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

    let biomes = RefCell::new(BiomeSource::new(opts.seed));
    let mut cam = start;
    let mut sort_pos = (f64::MAX, f64::MAX, f64::MAX);
    let mut window: Vec<(i32, i32)> = Vec::new();
    let mut window_set: HashSet<(i32, i32)>;
    let mut meshed: HashMap<(i32, i32), u64> = HashMap::new();
    let mut sky_sub = world.skylight_subtracted;
    let mut join_player_counter = 0;
    let mut weather_tail = 0;
    let mut next_tick = Instant::now();
    let mut idle_reported = false;

    let mut paused = false;

    loop {
        // Messages.
        loop {
            match rx.try_recv() {
                Ok(ToWorker::Camera { x, y, z }) => cam = (x, y, z),
                Ok(ToWorker::AddTime(t)) => world.world_time += t,
                Ok(ToWorker::Pause(p)) => {
                    if paused && !p {
                        // The Timer kept running while paused; those ticks are not caught up.
                        next_tick = Instant::now();
                    }
                    paused = p;
                }
                Ok(ToWorker::QuickSave(n)) => {
                    if n == 0 {
                        world.save_level();
                    }
                    let done = world.save_chunks(false);
                    let _ = tx.send(FromWorker::QuickSaved(done));
                }
                Ok(ToWorker::Quit) | Err(mpsc::TryRecvError::Disconnected) => {
                    // Minecraft quitting to the title screen / server shutdown: save everything.
                    // changeWorld(null, ""): printText(""), then saveWorldIndirectly.
                    let _ = tx.send(FromWorker::Loading(Loading::Title(String::new())));
                    let _ = tx.send(FromWorker::Loading(Loading::Text("Saving chunks".into())));
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
        if paused {
            // No ticks while paused, but keep the clock moving so meshing isn't starved.
            next_tick = Instant::now() + Duration::from_millis(50);
        }
        while !paused && Instant::now() >= next_tick && ticks < 20 {
            world.players.clear();
            world.players.push(cam);
            if players.is_none() {
                // Minecraft.runTick: joinEntityInSurroundings every 30 ticks, updateEntities,
                // World.tick (with mob spawning), randomDisplayUpdates.
                join_player_counter += 1;
                world.client_tick(0, join_player_counter);
            } else {
                // MinecraftServer.doTick: world tick, lighting, then entities.
                world.tick();
                world.drain_lighting();
                world.update_entities();
            }
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
            let weather = crate::render::weather::Weather {
                rain: world.raining_strength,
                prev_rain: world.prev_raining_strength,
                thunder: world.thundering_strength,
                prev_thunder: world.prev_thundering_strength,
                flash: world.lightning_flash,
            };
            if weather.rain > 0.0 || weather.prev_rain > 0.0 {
                weather_tail = 40;
            }
            let terrain = if weather_tail > 0 {
                weather_tail -= 1;
                weather_snapshot(&mut world, cam)
            } else {
                crate::render::weather::Snapshot::default()
            };
            let brightness = entity_brightness(&mut world, cam);
            // Entities near the camera, with their brightness, for the renderer.
            let near: Vec<i32> = world
                .loaded_entity_list
                .iter()
                .copied()
                .filter(|id| {
                    world.entities.get(id).is_some_and(|e| {
                        let (dx, dy, dz) = (e.pos.x - cam.0, e.pos.y - cam.1, e.pos.z - cam.2);
                        dx * dx + dy * dy + dz * dz < 128.0 * 128.0
                    })
                })
                .collect();
            let mut snapshot = Vec::with_capacity(near.len());
            for id in near {
                let e = world.entities[&id].clone();
                let b = living_brightness(&mut world, &e);
                let biomes=RefCell::new(BiomeSource::new(opts.seed));
                let access=Access::new(&world, e.chunk_x, e.chunk_z, world.skylight_subtracted, &biomes);
                let fl=worldgen::math::floor_double;
                let (x,y,z)=(fl(e.pos.x),fl(e.pos.y),fl(e.pos.z));
                let ds=[[0,0,0],[0,-1,0],[0,1,0],[0,0,-1],[0,0,1],[-1,0,0],[1,0,0]];
                let falling_light=ds.map(|[dx,dy,dz]|access.light_brightness(x+dx,y+dy,z+dz));
                let size=crate::render::entity::effects::radius(&e) as f64;
                let mut surfaces=Vec::new();
                // Cover both endpoints of the partial-tick interval, queried from loaded chunks only.
                for xx in fl(e.pos.x.min(e.last_tick_pos.x)-size)..=fl(e.pos.x.max(e.last_tick_pos.x)+size) {
                    for yy in fl(e.pos.y.min(e.last_tick_pos.y)+e.height as f64/2.0-size)..=fl(e.pos.y.max(e.last_tick_pos.y)+e.height as f64/2.0) {
                        for zz in fl(e.pos.z.min(e.last_tick_pos.z)-size)..=fl(e.pos.z.max(e.last_tick_pos.z)+size) {
                            let block=access.block(xx,yy-1,zz);
                            if block>0 && access.light_value(xx,yy,zz)>3 {
                                surfaces.push(crate::render::entity::effects::Surface{xyz:[xx,yy,zz],block,brightness:access.light_brightness(xx,yy,zz)});
                            }
                        }
                    }
                }
                snapshot.push(crate::render::entity::effects::Snapshot{entity:e,brightness:b,falling_light,surfaces});
            }
            let _ = tx.send(FromWorker::Entities(snapshot));
            let chunks = world.chunks.len();
            let _ = tx.send(FromWorker::Time { time: world.world_time, brightness, chunks, weather, terrain });
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

/// `Entity.getEntityBrightness` for a simulated entity.
fn living_brightness(world: &mut World, e: &worldgen::entity::Entity) -> f32 {
    let fl = worldgen::math::floor_double;
    let b = &e.bounding_box;
    if !world.check_chunks_exist(fl(b.min_x), fl(b.min_y), fl(b.min_z), fl(b.max_x), fl(b.max_y), fl(b.max_z)) {
        return 0.0;
    }
    let h = (b.max_y - b.min_y) * 0.66;
    let (x, y, z) = (fl(e.pos.x), fl(e.pos.y - e.y_offset as f64 + h), fl(e.pos.z));
    let l = world.get_block_light_value(x, y, z);
    world.light_brightness_table[l.clamp(0, 15) as usize]
}

/// `Entity.getEntityBrightness` for a player whose eyes are at `cam` (1.8 tall, eyes 1.62 up).
fn entity_brightness(world: &mut World, cam: (f64, f64, f64)) -> f32 {
    let feet = cam.1 - 1.62;
    let (x0, x1) = (cam.0 - 0.3, cam.0 + 0.3);
    let (z0, z1) = (cam.2 - 0.3, cam.2 + 0.3);
    let fl = worldgen::math::floor_double;
    if !world.check_chunks_exist(fl(x0), fl(feet), fl(z0), fl(x1), fl(feet + 1.8), fl(z1)) {
        return 0.0;
    }
    let h = (feet + 1.8 - feet) * 0.66;
    let (x, y, z) = (fl(cam.0), fl(cam.1 - 1.62 + h), fl(cam.2));
    let l = world.get_block_light_value(x, y, z);
    world.light_brightness_table[l.clamp(0, 15) as usize]
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

/// World.getLightBrightness uses World (y>=128 clamps to 127), not ChunkCache.
pub fn weather_snapshot(world: &mut World, cam: (f64, f64, f64)) -> crate::render::weather::Snapshot {
    use crate::render::weather::{Column, Snapshot};
    let (cx, cz) = (worldgen::math::floor_double(cam.0), worldgen::math::floor_double(cam.2));
    let mut bs = BiomeSource::new(world.seed);
    let mut biomes = Vec::new();
    bs.biomes(&mut biomes, cx - 10, cz - 10, 21, 21);
    let mut columns = Vec::with_capacity(441);
    for x in cx - 10..=cx + 10 {
        for z in cz - 10..=cz + 10 {
            let top = world.find_top_solid_block(x, z);
            let mut c = Column {
                x,
                z,
                top,
                biome: biomes[columns.len()],
                particle_rain: crate::render::weather::rains(bs.biome_at(x, z)),
                blocks: [0; 128],
                meta: [0; 128],
                brightness: [0.0; 129],
            };
            for y in 0..128 {
                c.blocks[y] = world.get_block_id(x, y as i32, z);
                c.meta[y] = world.get_block_metadata(x, y as i32, z) as u8;
                let l = world.get_block_light_value(x, y as i32, z);
                c.brightness[y] = world.light_brightness_table[l as usize];
            }
            let light = world.get_block_light_value(x, 128, z);
            c.brightness[128] = world.light_brightness_table[light as usize];
            columns.push(c);
        }
    }
    Snapshot { columns }
}
