//! `--setup <scenario>`: prepare a test situation right after the world opens, so a
//! `--play-script` can play through it in the real client (unattended end-to-end tests).
//! Test-only: without `--setup` nothing here runs. Scenarios only change the world the way
//! a player or the game could (placing blocks, items, entities, positions); they never
//! change how the game behaves.
//!
//! Add a scenario: write `fn name(w: &mut World, player: i32)` in a `playtest/*.rs` file,
//! then list it in `SCENARIOS`. The player stands at its spawn, looking south (+Z, yaw 0).

use std::sync::OnceLock;
use worldgen::World;

/// The `--setup` argument, set once while the options are parsed.
pub static SETUP: OnceLock<String> = OnceLock::new();

type Scenario = fn(&mut World, i32);

/// Every scenario by name.
const SCENARIOS: &[(&str, Scenario)] = &[("end_portal", end_portal)];

/// Run the requested scenario once (the worker calls this after the player joins).
pub fn run(w: &mut World, player: i32) {
    let Some(name) = SETUP.get() else { return };
    match SCENARIOS.iter().find(|(n, _)| n == name) {
        Some((_, f)) => {
            f(w, player);
            eprintln!("playtest: scenario '{name}' ready");
        }
        None => {
            let names: Vec<&str> = SCENARIOS.iter().map(|(n, _)| *n).collect();
            eprintln!("playtest: unknown scenario '{name}' (known: {})", names.join(", "));
        }
    }
}

/// The ground block under the player (the highest non-air block of its column), with the
/// player put standing on it, facing south (+Z). The player may still be falling at setup.
pub fn ground(w: &mut World, player: i32) -> (i32, i32, i32) {
    let fl = worldgen::math::floor_double;
    let (x, z) = {
        let e = &w.entities[&player];
        (fl(e.pos.x), fl(e.pos.z))
    };
    settle(w, x, z, 3);
    let mut y = 127;
    while y > 0 && w.get_block_id(x, y, z) == 0 {
        y -= 1;
    }
    let e = w.entities.get_mut(&player).expect("playtest: player");
    // Entity.setLocationAndAngles, as the game's own teleports: feet on top of the ground block.
    e.set_location_and_angles(x as f64 + 0.5, y as f64 + 1.0, z as f64 + 0.5, 0.0, 0.0);
    e.motion = worldgen::entity::Vec3D { x: 0.0, y: 0.0, z: 0.0 };
    (x, y, z)
}

/// Load the chunks within `r` chunks of (x, z) plus one more ring, so every chunk a scenario
/// edits is already populated (population runs once the neighbours exist and would otherwise
/// drop trees, lakes or ores onto the prepared blocks later).
pub fn settle(w: &mut World, x: i32, z: i32, r: i32) {
    let (cx, cz) = (x >> 4, z >> 4);
    for nx in cx - r - 1..=cx + r + 1 {
        for nz in cz - r - 1..=cz + r + 1 {
            if !w.chunk_exists(nx, nz) {
                w.load_chunk(nx, nz);
            }
        }
    }
    w.drain_lighting();
}

/// A lit End portal sunk level with the ground 3 blocks south of the player: 12 frames
/// facing the middle, all holding eyes (metadata facing | 4), and the 3x3 portal inside.
/// Walking south (W) drops the player into it.
fn end_portal(w: &mut World, player: i32) {
    use worldgen::block::{END_PORTAL, END_PORTAL_FRAME, STONE};
    let (x, top, z) = ground(w, player);
    let (cx, cz) = (x, z + 5);
    // A flat stone pad from under the player to past the portal, cleared above.
    for dx in -3..=3 {
        for dz in -8..=3 {
            for dy in -2..=0 {
                w.set_block_and_metadata(cx + dx, top + dy, cz + dz, STONE, 0);
            }
            for dy in 1..=3 {
                w.set_block_and_metadata(cx + dx, top + dy, cz + dz, 0, 0);
            }
        }
    }
    // Facing: 0 south(+Z), 1 west(-X), 2 north(-Z), 3 east(+X); each frame faces the middle.
    for i in -1..=1 {
        w.set_block_and_metadata(cx + i, top, cz - 2, END_PORTAL_FRAME, 0 | 4);
        w.set_block_and_metadata(cx + i, top, cz + 2, END_PORTAL_FRAME, 2 | 4);
        w.set_block_and_metadata(cx - 2, top, cz + i, END_PORTAL_FRAME, 3 | 4);
        w.set_block_and_metadata(cx + 2, top, cz + i, END_PORTAL_FRAME, 1 | 4);
    }
    for dx in -1..=1 {
        for dz in -1..=1 {
            w.set_block_and_metadata(cx + dx, top, cz + dz, END_PORTAL, 0);
        }
    }
}
