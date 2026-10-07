//! Play-test scenarios: items (see ../playtest.rs for how scenarios work).
#[allow(unused_imports)]
use super::{give, ground, settle};
#[allow(unused_imports)]
use worldgen::World;

/// This file's scenarios by name (looked up by `--setup <name>`).
pub const SCENARIOS: &[(&str, super::Scenario)] = &[
    ("items_eye_of_ender", items_eye_of_ender),
    ("items_ender_pearl", items_ender_pearl),
    ("items_splash_potion", items_splash_potion),
    ("items_drink_potion", items_drink_potion),
    ("items_enchanting", items_enchanting),
    ("items_brewing", items_brewing),
    ("items_cauldron", items_cauldron),
    ("items_nether_wart", items_nether_wart),
    ("items_lily_pad", items_lily_pad),
    ("items_dragon_egg", items_dragon_egg),
    ("items_bow", items_bow),
    ("items_creative_inventory", items_creative_inventory),
    ("items_end_frame_eye", items_end_frame_eye),
    ("items_snow_golem", items_snow_golem),
    ("items_shears_mooshroom", items_shears_mooshroom),
    ("items_milk_bucket", items_milk_bucket),
];

/// A flat stone pad (7 wide, 12 deep south of the player's column) at ground level `top`,
/// cleared three blocks above, so the scenarios build on level, empty ground.
fn pad(w: &mut World, x: i32, top: i32, z: i32) {
    for dx in -3..=3 {
        for dz in -3..=10 {
            for dy in -2..=0 {
                w.set_block_and_metadata(x + dx, top + dy, z + dz, 1, 0);
            }
            for dy in 1..=4 {
                w.set_block_and_metadata(x + dx, top + dy, z + dz, 0, 0);
            }
        }
    }
}

/// Four eyes of ender in hand, open flat ground.
fn items_eye_of_ender(w: &mut World, player: i32) {
    let (x, y, z) = ground(w, player);
    pad(w, x, y, z);
    give(w, player, 0, 381, 4, 0);
}

/// An ender pearl in hand, a long flat pad ahead.
fn items_ender_pearl(w: &mut World, player: i32) {
    let (x, y, z) = ground(w, player);
    pad(w, x, y, z);
    give(w, player, 0, 368, 4, 0);
}

/// A splash poison potion in hand, flat ground.
fn items_splash_potion(w: &mut World, player: i32) {
    let (x, y, z) = ground(w, player);
    pad(w, x, y, z);
    give(w, player, 0, 373, 3, 16388);
}

/// A potion of swiftness in hand, flat ground.
fn items_drink_potion(w: &mut World, player: i32) {
    let (x, y, z) = ground(w, player);
    pad(w, x, y, z);
    give(w, player, 0, 373, 1, 8194);
}

/// An enchantment table (116) two blocks south at foot level inside the 1.0 bookshelf ring
/// (TileEntityEnchantmentTable / ContainerEnchantment: shelves at distance 2, two high, air
/// between), the player at XP level 30 holding a diamond sword in slot 0.
fn items_enchanting(w: &mut World, player: i32) {
    let (x, y, z) = ground(w, player);
    pad(w, x, y, z);
    let (tx, ty, tz) = (x, y + 1, z + 2);
    w.set_block_and_metadata(tx, ty, tz, 116, 0);
    for dx in -2..=2i32 {
        for dz in -2..=2i32 {
            if dx.abs().max(dz.abs()) != 2 || (dx == 0 && dz == -2) {
                continue;
            }
            // The ring's four corners are not counted by the table.
            if dx.abs() == 2 && dz.abs() == 2 {
                continue;
            }
            for k in 0..2 {
                w.set_block_and_metadata(tx + dx, ty + k, tz + dz, 47, 0);
            }
        }
    }
    set_level(w, player, 30);
    give(w, player, 0, 276, 1, 0);
}

/// Player XP level (`EntityPlayer.experienceLevel`).
fn set_level(w: &mut World, player: i32, level: i32) {
    w.entities.get_mut(&player).unwrap().player_state_mut().unwrap().experience.level = level;
}

/// A brewing stand (117) two blocks south at foot level; three water bottles (slots 0-2) and a
/// nether wart (slot 3) in the hotbar.
fn items_brewing(w: &mut World, player: i32) {
    let (x, y, z) = ground(w, player);
    pad(w, x, y, z);
    w.set_block_and_metadata(x, y + 1, z + 2, 117, 0);
    for slot in 0..3 {
        give(w, player, slot, 373, 1, 0);
    }
    give(w, player, 3, 372, 1, 0);
}

/// An empty cauldron (118) two blocks south at foot level; a water bucket (slot 0) and glass
/// bottles (slot 1) in the hotbar.
fn items_cauldron(w: &mut World, player: i32) {
    let (x, y, z) = ground(w, player);
    pad(w, x, y, z);
    w.set_block_and_metadata(x, y + 1, z + 2, 118, 0);
    give(w, player, 1, 374, 3, 0);
    give(w, player, 0, 326, 1, 0);
}

/// A 3x3 patch of soul sand (88) in the ground two blocks south, nether wart seeds (372) in hand.
fn items_nether_wart(w: &mut World, player: i32) {
    let (x, y, z) = ground(w, player);
    pad(w, x, y, z);
    for dx in -1..=1 {
        for dz in 2..=4 {
            w.set_block_and_metadata(x + dx, y, z + dz, 88, 0);
        }
    }
    give(w, player, 0, 372, 4, 0);
}

/// A 3x3 pool of still water (9), one deep, sunk into the ground two blocks south, lily pads
/// (111) in hand.
fn items_lily_pad(w: &mut World, player: i32) {
    let (x, y, z) = ground(w, player);
    pad(w, x, y, z);
    for dx in -1..=1 {
        for dz in 2..=4 {
            w.set_block_and_metadata(x + dx, y, z + dz, 9, 0);
        }
    }
    give(w, player, 0, 111, 4, 0);
}

/// A dragon egg (122) on a one-block stone pillar two blocks south, at eye height.
fn items_dragon_egg(w: &mut World, player: i32) {
    let (x, y, z) = ground(w, player);
    pad(w, x, y, z);
    w.set_block_and_metadata(x, y + 1, z + 2, 1, 0);
    w.set_block_and_metadata(x, y + 2, z + 2, 122, 0);
}

/// A bow in slot 0, 16 arrows in slot 1, open flat ground.
fn items_bow(w: &mut World, player: i32) {
    let (x, y, z) = ground(w, player);
    pad(w, x, y, z);
    give(w, player, 1, 262, 16, 0);
    give(w, player, 0, 261, 1, 0);
}

/// A creative-mode player (Minecraft.startWorld -> PlayerController.func_6476_a / the saved
/// world's game type 1): flat ground, the nine default creative hotbar blocks.
fn items_creative_inventory(w: &mut World, player: i32) {
    let (x, y, z) = ground(w, player);
    pad(w, x, y, z);
    w.info.game_type = 1;
    w.entities.get_mut(&player).unwrap().initialize_player_mode(true);
}

/// An End portal frame ring (12 frames flat in the ground, five blocks south) with eleven eyes
/// already set and the frame nearest the player empty; an eye of ender in hand
/// (ItemEnderEye.onItemUse completes the ring and creates the portal).
fn items_end_frame_eye(w: &mut World, player: i32) {
    let (x, y, z) = ground(w, player);
    pad(w, x, y, z);
    let (cx, cz) = (x, z + 5);
    // Facing: 0 south(+Z), 1 west(-X), 2 north(-Z), 3 east(+X); each frame faces the middle.
    for i in -1..=1 {
        w.set_block_and_metadata(cx + i, y, cz - 2, 120, if i == 0 { 0 } else { 4 });
        w.set_block_and_metadata(cx + i, y, cz + 2, 120, 2 | 4);
        w.set_block_and_metadata(cx - 2, y, cz + i, 120, 3 | 4);
        w.set_block_and_metadata(cx + 2, y, cz + i, 120, 1 | 4);
    }
    give(w, player, 0, 381, 1, 0);
}

/// Two snow blocks (80) stacked, their top at foot level, two blocks south and a pumpkin (86) in hand: placing the pumpkin
/// on top builds a snow golem (BlockPumpkin.onBlockAdded).
fn items_snow_golem(w: &mut World, player: i32) {
    let (x, y, z) = ground(w, player);
    pad(w, x, y, z);
    // Sunk into the ground so the top face lies below the eyes and can be clicked.
    w.set_block_and_metadata(x, y, z + 2, 80, 0);
    w.set_block_and_metadata(x, y + 1, z + 2, 80, 0);
    give(w, player, 0, 86, 1, 0);
}

/// A mooshroom in a fenced cell three blocks south and shears (359) in hand (EntityMooshroom.interact).
fn items_shears_mooshroom(w: &mut World, player: i32) {
    let (x, y, z) = ground(w, player);
    pad(w, x, y, z);
    // A one-block fenced cell (the fences are 1.5 high for walking mobs, 1.0 for the aim ray) keeps it
    // from wandering off while the test runs.
    for dx in -1..=1 {
        for dz in 2..=4 {
            if (dx, dz) != (0, 3) {
                w.set_block_and_metadata(x + dx, y + 1, z + dz, 85, 0);
            }
        }
    }
    super::spawn(w, "MushroomCow", x as f64 + 0.5, (y + 1) as f64, z as f64 + 3.5);
    give(w, player, 0, 359, 1, 0);
}

/// A milk bucket (335) in hand, flat ground (ItemBucketMilk: drinking leaves an empty bucket).
fn items_milk_bucket(w: &mut World, player: i32) {
    let (x, y, z) = ground(w, player);
    pad(w, x, y, z);
    give(w, player, 0, 335, 1, 0);
}
