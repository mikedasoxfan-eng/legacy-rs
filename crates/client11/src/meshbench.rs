//! `--meshbench [seed] [distance]`: times the world thread's renderer-window work without a
//! window: loading (generation, population, lighting) and meshing, nearest chunk first, the
//! same order as `worker::run`. Prints totals so rendering optimizations can be measured.
use crate::render::blocks::Colorizers;
use std::cell::RefCell;
use std::time::{Duration, Instant};
use worldgen::biome::BiomeSource;

pub fn run(args: &[String]) {
    let seed: i64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(42);
    let distance: i32 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(0);
    let assets = crate::assets::load().expect("assets");
    let col = Colorizers { grass: assets.grass_color, foliage: assets.foliage_color };
    let mut video = crate::render::video::VideoSettings::default();
    video.distance = distance;
    let t = Instant::now();
    let mut world = worldgen::World::new_vanilla(seed, 0);
    world.leaves_opaque = !video.fancy;
    let preload = t.elapsed();
    let biomes = RefCell::new(BiomeSource::new(seed));
    let sky_sub = world.skylight_subtracted;
    let spawn = world.spawn;
    let xs = crate::worker::window_axis(spawn.0, video.chunks_wide());
    let zs = crate::worker::window_axis(spawn.2, video.chunks_wide());
    let mut window: Vec<(i32, i32)> = xs.iter().flat_map(|&x| zs.iter().map(move |&z| (x, z))).collect();
    let (ccx, ccz) = (spawn.0 >> 4, spawn.2 >> 4);
    window.sort_by_key(|&(x, z)| (x - ccx) * (x - ccx) + (z - ccz) * (z - ccz));
    let (mut load, mut mesh) = (Duration::ZERO, Duration::ZERO);
    let (mut loaded, mut verts, mut bytes) = (0usize, 0usize, 0usize);
    for &(cx, cz) in &window {
        let t = Instant::now();
        for nx in cx - 1..=cx + 1 {
            for nz in cz - 1..=cz + 1 {
                if !world.chunk_exists(nx, nz) {
                    world.load_chunk(nx, nz);
                    world.drain_lighting();
                    loaded += 1;
                }
            }
        }
        load += t.elapsed();
        let t = Instant::now();
        let (o, tr) = crate::worker::mesh_chunk(&world, cx, cz, sky_sub, &biomes, &col, video);
        mesh += t.elapsed();
        verts += o.len() + tr.len();
        bytes += (o.len() + tr.len()) * std::mem::size_of::<crate::render::Vertex>();
    }
    let n = window.len();
    println!("meshbench seed {seed}, distance {distance}: {n} renderer chunks ({} wide), {loaded} chunks loaded after the {:.2}s spawn preload", video.chunks_wide(), preload.as_secs_f64());
    println!("  loading: {:.2}s ({:.2} ms per chunk loaded)", load.as_secs_f64(), load.as_secs_f64() * 1000.0 / loaded.max(1) as f64);
    println!("  meshing: {:.2}s ({:.2} ms per chunk)", mesh.as_secs_f64(), mesh.as_secs_f64() * 1000.0 / n as f64);
    println!("  vertices: {verts} ({:.1} MB, {:.0} per chunk)", bytes as f64 / 1e6, verts as f64 / n as f64);
}
