//! Entity and complete-world replay of smelting, dispensing and machine removal.
use crate::{
    World,
    container::{ItemStack, tiles::Furnace},
    world::{
        RandSel,
        chunk::{Chunk, idx},
        tile::TileEntity,
    },
};
fn stack(out: &mut Vec<u8>, s: Option<ItemStack>) {
    out.push(s.is_some() as u8);
    if let Some(s) = s {
        for v in [s.id, s.count, s.damage] {
            out.extend(v.to_be_bytes());
        }
    }
}
pub fn tile_bytes(ticks: i32) -> Vec<u8> {
    let mut w = World::new(0, 300);
    w.set_entity_determinism(100, 200, 0);
    w.container_drop_random = std::array::from_fn(|i| crate::JavaRandom::new(700 + i as i64));
    for x in -3..=3 {
        for z in -3..=3 {
            let mut c = Chunk::new(x, z, vec![0; 32768].into_boxed_slice().try_into().unwrap());
            for bx in 0..16 {
                for bz in 0..16 {
                    c.blocks[idx(bx, 63, bz)] = 1;
                }
            }
            c.loaded = true;
            c.populated = true;
            w.chunks.insert((x, z), Box::new(c));
            w.chunk_list.push((x, z));
        }
    }
    let inputs = [15, 14, 56, 12, 4, 319, 349, 81, 17, 16, 1, 15];
    let fuels = [263, 280, 5, 327, 6, 17, 54, 53, 85, 58, 264, 263];
    for i in 0..inputs.len() {
        let x = -12 + i as i32 * 2;
        w.set_block_with_notify(x, 64, 8, 61);
        if let Some(TileEntity::Furnace(f)) = w.get_block_tile_entity(x, 64, 8) {
            f.items[0] = Some(ItemStack::new(inputs[i], 12, i as i32 % 3));
            f.items[1] = Some(ItemStack::new(fuels[i], 4, i as i32 % 2));
            if i == 11 {
                f.items[2] = Some(ItemStack::new(265, 64, 0));
            }
        }
    }
    w.set_block_with_notify(8, 64, -8, 23);
    w.set_block_metadata_with_notify(8, 64, -8, 3);
    w.dispenser_random
        .insert((8, 64, -8), crate::JavaRandom::new(900));
    let items = [262, 344, 332, 35, 263, 264, 327, 5, 351];
    if let Some(TileEntity::Dispenser(d)) = w.get_block_tile_entity(8, 64, -8) {
        for i in 0..9 {
            d[i] = Some(ItemStack::new(
                items[i],
                6,
                if items[i] == 35 { 14 } else { 0 },
            ));
        }
    }
    let mut out = Vec::new();
    for t in 0..=ticks {
        if t > 0 {
            if t < 400 && t % 5 == 0 {
                w.set_block_metadata_with_notify(8, 64, -8, 2 + t / 5 % 4);
                w.dispense_item(8, 64, -8, RandSel::World);
            }
            if t == 450 {
                w.set_block_with_notify(-12, 64, 8, 0);
            }
            if t == 500 {
                w.set_block_with_notify(8, 64, -8, 0);
            }
            if t == 600 {
                if let Some(TileEntity::Furnace(f)) = w.get_block_tile_entity(-6, 64, 8) {
                    f.burn = 1;
                    f.items[1] = None;
                }
            }
            w.update_entities();
        }
        crate::entity::verify::snapshot(&w, &mut out);
        for i in 0..inputs.len() {
            let f: Option<Furnace> = match w.get_block_tile_entity(-12 + i as i32 * 2, 64, 8) {
                Some(TileEntity::Furnace(f)) => Some(f.clone()),
                _ => None,
            };
            out.push(f.is_some() as u8);
            if let Some(f) = f {
                for s in f.items {
                    stack(&mut out, s);
                }
                for v in [f.burn, f.current_fuel, f.cook] {
                    out.extend(v.to_be_bytes());
                }
            }
        }
        let mut bytes = Vec::new();
        crate::entity::verify::append_world(&w, &mut bytes);
        let hash = bytes.iter().fold(0xcbf29ce484222325u64, |n, &b| {
            (n ^ b as u64).wrapping_mul(0x100000001b3)
        });
        out.extend(hash.to_be_bytes());
    }
    crate::entity::verify::append_world(&w, &mut out);
    out
}
