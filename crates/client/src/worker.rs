//! The world thread: owns the `World`, generates chunks around the camera
//! (nearest first, after the vanilla spawn preload) and builds their meshes.

use crate::render::access::Access;
use crate::render::blocks::{Colorizers, RenderBlocks};
use crate::render::{rprops, Tessellator, Vertex};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::time::{Duration, Instant};
use worldgen::{BiomeSource, World};

pub enum ToWorker {
    Camera { cx: i32, cz: i32, sky_sub: i32 },
}

pub enum FromWorker {
    Ready { spawn: (f64, f64, f64) },
    Mesh { cx: i32, cz: i32, opaque: Vec<Vertex>, translucent: Vec<Vertex> },
    /// Everything within the render radius is generated and meshed.
    Idle { loaded: usize },
}

/// Chunk radius of the mesh area: "Far" render distance is 256 blocks.
pub const RENDER_RADIUS: i32 = 16;

pub fn spawn(seed: i64, world_rand: i64, col: Colorizers) -> (Sender<ToWorker>, Receiver<FromWorker>) {
    let (to_tx, to_rx) = mpsc::channel();
    let (from_tx, from_rx) = mpsc::channel();
    std::thread::Builder::new()
        .name("world".into())
        .spawn(move || run(seed, world_rand, col, to_rx, from_tx))
        .expect("spawn world thread");
    (to_tx, from_rx)
}

fn run(seed: i64, world_rand: i64, col: Colorizers, rx: Receiver<ToWorker>, tx: Sender<FromWorker>) {
    let t = Instant::now();
    let mut world = World::new_vanilla(seed, world_rand);
    let (sx, _, sz) = world.spawn;
    let mut top = 127;
    while top > 0 && world.get_block_id(sx, top, sz) == 0 {
        top -= 1;
    }
    eprintln!("world: spawn {:?}, {} chunks preloaded in {:.2?}", world.spawn, world.chunks.len(), t.elapsed());
    let _ = tx.send(FromWorker::Ready { spawn: (sx as f64 + 0.5, top as f64 + 1.0 + 1.62, sz as f64 + 0.5) });

    let biomes = RefCell::new(BiomeSource::new(seed));
    let mut offsets: Vec<(i32, i32)> = Vec::new();
    for dx in -RENDER_RADIUS..=RENDER_RADIUS {
        for dz in -RENDER_RADIUS..=RENDER_RADIUS {
            offsets.push((dx, dz));
        }
    }
    offsets.sort_by_key(|&(dx, dz)| dx * dx + dz * dz);

    let mut cam = (sx >> 4, sz >> 4);
    let mut sky_sub = 0;
    let mut meshed: HashMap<(i32, i32), u64> = HashMap::new();
    let mut idle_reported = false;
    loop {
        // Take the newest camera state.
        loop {
            match rx.try_recv() {
                Ok(ToWorker::Camera { cx, cz, sky_sub: s }) => {
                    if (cx, cz) != cam {
                        idle_reported = false;
                    }
                    cam = (cx, cz);
                    if s != sky_sub {
                        sky_sub = s;
                        meshed.clear(); // vanilla re-renders everything when the sky darkens
                        idle_reported = false;
                    }
                }
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => return,
            }
        }

        let budget = Instant::now();
        let mut did_work = false;
        for &(dx, dz) in &offsets {
            if budget.elapsed() > Duration::from_millis(25) {
                did_work = true;
                break;
            }
            let (cx, cz) = (cam.0 + dx, cam.1 + dz);
            // Meshing needs the 3x3 neighbourhood, which also guarantees population is done.
            let mut loaded_any = false;
            for nx in cx - 1..=cx + 1 {
                for nz in cz - 1..=cz + 1 {
                    if !world.chunk_exists(nx, nz) {
                        world.load_chunk(nx, nz);
                        world.drain_lighting();
                        loaded_any = true;
                    }
                }
            }
            if loaded_any {
                did_work = true;
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
                let _ = tx.send(FromWorker::Idle { loaded: world.chunks.len() });
            }
            match rx.recv_timeout(Duration::from_millis(50)) {
                Ok(ToWorker::Camera { cx, cz, sky_sub: s }) => {
                    if (cx, cz) != cam || s != sky_sub {
                        idle_reported = false;
                    }
                    cam = (cx, cz);
                    if s != sky_sub {
                        sky_sub = s;
                        meshed.clear();
                    }
                }
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => return,
            }
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
