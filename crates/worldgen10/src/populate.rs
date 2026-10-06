//! `ChunkProviderGenerate.populate` and `BiomeDecorator` (Beta 1.8.1).
use crate::{
    biome::Biome,
    block,
    features::{self, TreeKind},
    world::World,
};

pub fn populate(w: &mut World, cx: i32, cz: i32) {
    w.sand_fall_instantly = true;
    let x0 = cx * 16;
    let z0 = cz * 16;
    let biome = w.terrain.biome_source.biome_at(x0 + 16, z0 + 16);
    let seed = w.seed;
    let r = w.prand();
    r.set_seed(seed);
    let mx = r.next_long() / 2 * 2 + 1;
    let mz = r.next_long() / 2 * 2 + 1;
    r.set_seed(
        (cx as i64)
            .wrapping_mul(mx)
            .wrapping_add((cz as i64).wrapping_mul(mz))
            ^ seed,
    );
    let mut village = false;
    if w.info.map_features {
        let mut structures = std::mem::take(&mut w.mineshafts);
        let mut r = w.prand().clone();
        structures.populate(w, &mut r, cx, cz);
        *w.prand() = r;
        // A structure callback can load more chunks. Keep starts registered by those
        // nested loads while the outer generator was temporarily borrowed out.
        for key in w.mineshafts.keys() {
            let start = w.mineshafts.starts.remove(&key).unwrap();
            structures.merge_start(key, start);
        }
        w.mineshafts = structures;
        let mut structures = std::mem::take(&mut w.villages);
        let mut r = w.prand().clone();
        village = structures.populate(w, &mut r, cx, cz);
        *w.prand() = r;
        for key in w.villages.keys() {
            let start = w.villages.starts.remove(&key).unwrap();
            structures.merge_start(key, start);
        }
        w.villages = structures;
        let mut strongholds = std::mem::take(&mut w.strongholds);
        let mut r = w.prand().clone();
        strongholds.populate(w, &mut r, cx, cz);
        *w.prand() = r;
        for key in w.strongholds.keys() {
            let start = w.strongholds.starts.remove(&key).unwrap();
            strongholds.merge_start(key, start);
        }
        w.strongholds = strongholds;
    }
    if !village && w.prand().next_int(4) == 0 {
        let (x, y, z) = surface_pos(w, x0, z0);
        features::lakes(w, block::WATER_STILL, x, y, z);
    }
    if !village && w.prand().next_int(8) == 0 {
        let x = x0 + w.prand().next_int(16) + 8;
        let bound = w.prand().next_int(120) + 8;
        let y = w.prand().next_int(bound);
        let z = z0 + w.prand().next_int(16) + 8;
        if y < 63 || w.prand().next_int(10) == 0 {
            features::lakes(w, block::LAVA_STILL, x, y, z);
        }
    }
    for _ in 0..8 {
        let (x, y, z) = surface_pos(w, x0, z0);
        features::dungeon(w, x, y, z);
    }
    decorate(w, biome, x0, z0);
    spawn_population(w, biome, x0 + 8, z0 + 8, 16, 16);
    for x in 0..16 {
        for z in 0..16 {
            let wx = x0.wrapping_add(8).wrapping_add(x);
            let wz = z0.wrapping_add(8).wrapping_add(z);
            let y = w.precipitation_height181(wx, wz);
            if w.can_block_freeze(wx, y - 1, wz, false) { w.set_block_with_notify(wx, y - 1, wz, block::ICE); }
            if w.can_snow_at(wx, y, wz) { w.set_block_with_notify(wx, y, wz, block::SNOW); }
        }
    }
    w.sand_fall_instantly = false;
}

pub(crate) fn generate_ores(w: &mut World, x0: i32, z0: i32) {
    let ores = [
        (block::DIRT, 32, 20, 128),
        (block::GRAVEL, 32, 10, 128),
        (block::ORE_COAL, 16, 20, 128),
        (block::ORE_IRON, 8, 20, 64),
        (block::ORE_GOLD, 8, 2, 32),
        (block::ORE_REDSTONE, 7, 8, 16),
        (block::ORE_DIAMOND, 7, 1, 16),
    ];
    for (ore, size, count, high) in ores {
        for _ in 0..count {
            let x = x0 + w.prand().next_int(16);
            let y = w.prand().next_int(high);
            let z = z0 + w.prand().next_int(16);
            features::minable(w, ore, size, x, y, z);
        }
    }
    let x = x0 + w.prand().next_int(16);
    let y = w.prand().next_int(16) + w.prand().next_int(16);
    let z = z0 + w.prand().next_int(16);
    features::minable(w, block::ORE_LAPIS, 6, x, y, z);
}

pub fn decorate(w: &mut World, biome: Biome, x0: i32, z0: i32) {
    // BiomeGenEnd's decorator is a BiomeEndDecorator (ores, spikes, the dragon).
    if biome == Biome::Sky {
        return crate::end::decorate(w, x0, z0);
    }
    decorate_with_settings(w, biome, x0, z0, crate::decorator10::DecoratorSettings::for_biome(biome));
}
pub fn decorate_with_settings(w: &mut World, biome: Biome, x0: i32, z0: i32, d: crate::decorator10::DecoratorSettings) {
    crate::decorator10::begin(biome);
    generate_ores(w, x0, z0);
    for _ in 0..d.sand2 {
        let (x, y, z) = disc_pos(w, x0, z0);
        features::sand(w, 7, block::SAND, x, y, z);
    }
    for _ in 0..d.clay {
        let (x, y, z) = disc_pos(w, x0, z0);
        features::clay(w, 4, x, y, z);
    }
    for _ in 0..d.sand {
        let (x, y, z) = disc_pos(w, x0, z0);
        features::sand(w, 7, block::SAND, x, y, z);
    }
    let mut trees = d.trees;
    if w.prand().next_int(10) == 0 {
        trees += 1;
    }
    for _ in 0..trees {
        let x = x0 + w.prand().next_int(16) + 8;
        let z = z0 + w.prand().next_int(16) + 8;
        let kind = tree_for_biome(w, biome);
        let y = w.get_height_value(x, z);
        if matches!(kind, TreeKind::BigOak) {
            // BiomeGenBase retains its big-tree generator, including its first height limit.
            crate::biome::big_tree(biome, w, x, y, z);
        } else {
            features::tree(w, kind, x, y, z);
        }
    }
    for _ in 0..d.big_mushrooms {
        let x = x0 + w.prand().next_int(16) + 8;
        let z = z0 + w.prand().next_int(16) + 8;
        let y = w.get_height_value(x, z);
        crate::features181::big_mushroom(w, None, x, y, z);
    }
    for _ in 0..d.flowers {
        let (x, y, z) = surface_pos(w, x0, z0);
        features::flowers(w, block::PLANT_YELLOW, x, y, z);
        if w.prand().next_int(4) == 0 {
            let (x, y, z) = surface_pos(w, x0, z0);
            features::flowers(w, block::PLANT_RED, x, y, z);
        }
    }
    for _ in 0..d.grass {
        let (x, y, z) = surface_pos(w, x0, z0);
        features::tall_grass(w, block::TALL_GRASS, 1, x, y, z);
    }
    for _ in 0..d.dead_bush {
        let (x, y, z) = surface_pos(w, x0, z0);
        features::dead_bush(w, block::DEAD_BUSH, x, y, z);
    }
    for _ in 0..d.waterlilies {
        let x = x0 + w.prand().next_int(16) + 8;
        let z = z0 + w.prand().next_int(16) + 8;
        let mut y = w.prand().next_int(128);
        while y > 0 && w.get_block_id(x, y - 1, z) == 0 { y -= 1; }
        crate::features181::waterlily(w, x, y, z);
    }
    for _ in 0..d.mushrooms {
        if w.prand().next_int(4) == 0 {
            let x = x0 + w.prand().next_int(16) + 8;
            let z = z0 + w.prand().next_int(16) + 8;
            let y = w.get_height_value(x, z);
            features::flowers(w, block::MUSHROOM_BROWN, x, y, z);
        }
        if w.prand().next_int(8) == 0 {
            let x = x0 + w.prand().next_int(16) + 8;
            let z = z0 + w.prand().next_int(16) + 8;
            let y = w.prand().next_int(128);
            features::flowers(w, block::MUSHROOM_RED, x, y, z);
        }
    }
    if w.prand().next_int(4) == 0 {
        let (x, y, z) = surface_pos(w, x0, z0);
        features::flowers(w, block::MUSHROOM_BROWN, x, y, z);
    }
    if w.prand().next_int(8) == 0 {
        let (x, y, z) = surface_pos(w, x0, z0);
        features::flowers(w, block::MUSHROOM_RED, x, y, z);
    }
    for _ in 0..d.reeds {
        let x = x0 + w.prand().next_int(16) + 8;
        let z = z0 + w.prand().next_int(16) + 8;
        let y = w.prand().next_int(128);
        features::reed(w, x, y, z);
    }
    for _ in 0..10 {
        let (x, y, z) = surface_pos(w, x0, z0);
        features::reed(w, x, y, z);
    }
    if w.prand().next_int(32) == 0 {
        let (x, y, z) = surface_pos(w, x0, z0);
        features::pumpkin(w, x, y, z);
    }
    for _ in 0..d.cacti {
        let (x, y, z) = surface_pos(w, x0, z0);
        features::cactus(w, x, y, z);
    }
    if d.generate_fluid {
    for _ in 0..50 {
        let x = x0 + w.prand().next_int(16) + 8;
        let bound = w.prand().next_int(120) + 8;
        let y = w.prand().next_int(bound);
        let z = z0 + w.prand().next_int(16) + 8;
        features::liquid_spring(w, block::WATER_MOVING, x, y, z);
    }
    for _ in 0..20 {
        let x = x0 + w.prand().next_int(16) + 8;
        let bound = w.prand().next_int(112) + 8;
        let bound = w.prand().next_int(bound) + 8;
        let y = w.prand().next_int(bound);
        let z = z0 + w.prand().next_int(16) + 8;
        features::liquid_spring(w, block::LAVA_MOVING, x, y, z);
    }
    }
    crate::decorator10::finish(biome);
}
fn surface_pos(w: &mut World, x0: i32, z0: i32) -> (i32, i32, i32) {
    let x = x0 + w.prand().next_int(16) + 8;
    let y = w.prand().next_int(128);
    let z = z0 + w.prand().next_int(16) + 8;
    (x, y, z)
}
fn disc_pos(w: &mut World, x0: i32, z0: i32) -> (i32, i32, i32) {
    let x = x0 + w.prand().next_int(16) + 8;
    let z = z0 + w.prand().next_int(16) + 8;
    // World.getTopSolidOrLiquidBlock skips leaves and liquids despite its name.
    let mut y = 127;
    while y > 0 {
        let m = w.get_block_material(x, y, z);
        if block::mat(m).blocks_movement && m != block::MAT_LEAVES {
            return (x, y + 1, z);
        }
        y -= 1;
    }
    (x, -1, z)
}
pub(crate) fn tree_for_biome(w: &mut World, biome: Biome) -> TreeKind {
    let r = w.prand();
    match biome {
        Biome::Forest => {
            if r.next_int(5) == 0 {
                TreeKind::Birch
            } else if r.next_int(10) == 0 {
                TreeKind::BigOak
            } else {
                TreeKind::Oak
            }
        }
        Biome::Taiga => {
            if r.next_int(3) == 0 {
                TreeKind::Spruce1
            } else {
                TreeKind::Spruce2
            }
        }
        Biome::Swampland => TreeKind::Swamp,
        _ => {
            if r.next_int(10) == 0 {
                TreeKind::BigOak
            } else {
                TreeKind::Oak
            }
        }
    }
}

/// `SpawnerAnimals.func_35957_a`: population uses the provider RNG, selection uses World.rand.
pub fn spawn_population(w: &mut World, biome: Biome, x0: i32, z0: i32, width: i32, height: i32) {
    use crate::entity::{Entity, EntityKind};
    let choices = crate::weighted::creatures(biome);
    if choices.is_empty() {
        return;
    }
    while w.prand().next_float() < 0.1f32 {
        let choice = *crate::weighted::choose(&mut w.rand, &choices).unwrap();
        let count = choice.min + w.prand().next_int(1 + choice.max - choice.min);
        let mut x = x0 + w.prand().next_int(width);
        let mut z = z0 + w.prand().next_int(height);
        let anchor = (x, z);
        for _ in 0..count {
            for _ in 0..4 {
                let (_, y, _) = top_solid(w, x, z);
                let good = w.is_block_normal_cube(x, y - 1, z)
                    && !w.is_block_normal_cube(x, y, z)
                    && !block::mat(w.get_block_material(x, y, z)).is_liquid
                    && !w.is_block_normal_cube(x, y + 1, z);
                if good {
                    let Some(mut e) = Entity::living(w, choice.entity) else { return; };
                    let id = e.entity_id;
                    let yaw = w.prand().next_float() * 360.0f32;
                    e.set_location_and_angles(
                        (x as f32 + 0.5f32) as f64,
                        y as f32 as f64,
                        (z as f32 + 0.5f32) as f64,
                        yaw,
                        0.0,
                    );
                    w.entity_joined_world(e);
                    if choice.entity == "Sheep" {
                        let n = w.rand.next_int(100);
                        let color = if n < 5 {
                            15
                        } else if n < 10 {
                            7
                        } else if n < 15 {
                            8
                        } else if n < 18 {
                            12
                        } else if w.rand.next_int(500) == 0 {
                            6
                        } else {
                            0
                        };
                        if let Some(e) = w.entities.get_mut(&id) {
                            if let EntityKind::Sheep { fleece_color, .. } = &mut e.kind {
                                *fleece_color = color;
                            }
                        }
                    }
                }
                x += w.prand().next_int(5) - w.prand().next_int(5);
                z += w.prand().next_int(5) - w.prand().next_int(5);
                // Vanilla checks width for both axes.
                while x < x0 || x >= x0 + width || z < z0 || z >= z0 + width {
                    x = anchor.0 + w.prand().next_int(5) - w.prand().next_int(5);
                    z = anchor.1 + w.prand().next_int(5) - w.prand().next_int(5);
                }
                if good {
                    break;
                }
            }
        }
    }
}
fn top_solid(w: &mut World, x: i32, z: i32) -> (i32, i32, i32) {
    let mut y = 127;
    while y > 0 {
        let m = w.get_block_material(x, y, z);
        if block::mat(m).blocks_movement && m != block::MAT_LEAVES {
            return (x, y + 1, z);
        }
        y -= 1;
    }
    (x, -1, z)
}
