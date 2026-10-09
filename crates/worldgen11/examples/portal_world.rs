//! Test fixture: a new world with a lit Nether portal two blocks in front of the player.
//! `cargo run --release -p worldgen --example portal_world -- <save dir> <seed>`

use worldgen::World;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dir = std::path::PathBuf::from(&args[1]);
    let seed: i64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(404);
    let (mut w, _) =
        World::open_with(Some(&dir), seed, seed ^ 0x5DEECE66D, false, Some("Portal test"), &mut |_| {}).expect("open");
    let id = w.spawn_player("Player");
    let e = &w.entities[&id];
    let (x, z) = (e.pos.x.floor() as i32, e.pos.z.floor() as i32);
    // Where the player will stand: on the first solid block below.
    let mut feet = (e.pos.y - 1.62).floor() as i32;
    while feet > 1 && w.get_block_id(x, feet - 1, z) == 0 {
        feet -= 1;
    }
    // The new player faces -Z (yaw -180): the frame stands across x at z - 3.
    let pz = z - 3;
    for dx in -1..=2 {
        for dy in -1..=3 {
            let frame = dx == -1 || dx == 2 || dy == -1 || dy == 3;
            w.set_block(x + dx, feet + dy, pz, if frame { worldgen::block::OBSIDIAN } else { worldgen::block::PORTAL });
        }
        // Clear the way to it.
        for dz in pz + 1..z {
            for dy in 0..3 {
                w.set_block(x + dx, feet + dy, dz, 0);
            }
            w.set_block(x + dx, feet - 1, dz, 1);
        }
    }
    w.save_world(true);
    eprintln!("portal at x {x}..{} y {feet}..{} z {pz}; player at {x} {feet} {z}", x + 1, feet + 2);
}
