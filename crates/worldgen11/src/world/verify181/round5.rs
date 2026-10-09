//! Round-five original-bytecode probes through production APIs.
use super::*;
use std::{fs, path::Path};

pub(super) fn legacy(o: &mut Vec<u8>) {
    use crate::{legacy_chunk181::LegacyChunkLoader181 as Loader, storage::Storage};
    let root = verification_dir("wb181-legacy5-rust");
    for create in [false, true] {
        let dir = root.join(if create { "create" } else { "readonly" });
        fs::create_dir_all(&dir).unwrap();
        let loader = Loader::new(&dir, create);
        for x in [0, -1, 63, 64, -64, i32::MIN, i32::MAX] {
            for z in [0, -1, 64] {
                let path = loader.chunk_file(x, z);
                o.push(path.is_some() as u8);
                if let Some(path) = path {
                    let name = path
                        .strip_prefix(&dir)
                        .unwrap()
                        .to_string_lossy()
                        .replace('\\', "/");
                    i(o, name.len() as i32);
                    o.extend(name.as_bytes());
                }
                let digits = b"0123456789abcdefghijklmnopqrstuvwxyz";
                let mask_name = |v: i32| {
                    let v = (v & 63) as usize;
                    if v < 36 {
                        (digits[v] as char).to_string()
                    } else {
                        format!("{}{}", digits[v / 36] as char, digits[v % 36] as char)
                    }
                };
                let first = dir.join(mask_name(x));
                let second = first.join(mask_name(z));
                o.push(first.exists() as u8);
                o.push(second.exists() as u8);
                o.push(loader.read_root(x, z).is_some() as u8);
            }
        }
    }
    let oracle=Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tools/oracle10/out/wb10-legacy5-java/input");
    let mut reader = Storage::open_legacy_with_players(
        &oracle,
        false,
    )
    .unwrap();
    for k in 0..9 {
        let c = reader.load_chunk(k, 0);
        o.push(c.is_some() as u8);
        if let Some(c) = c {
            array_snapshot(o, &c);
        }
    }
    let save = root.join("save");
    let mut writer = Storage::open_legacy_with_players(&save, false).unwrap();
    for dimension in [
        super::super::Dimension::Overworld,
        super::super::Dimension::Nether,
        super::super::Dimension::Sky,
    ] {
        let _loader = writer.for_dimension(dimension).unwrap();
        o.push(1);
        o.push(save.join("DIM-1").exists() as u8);
    }
    let mut chunk = Chunk::new(-67, 131, vec![0; 32768].into_boxed_slice());
    chunk.populated = true;
    let loader = Loader::new(&save, true);
    for k in 0..4 {
        let path = loader.chunk_file(chunk.x, chunk.z).unwrap();
        let old = fs::metadata(&path).map(|m| m.len() as i64).unwrap_or(0);
        if k == 2 {
            fs::create_dir(save.join("tmp_chunk.dat")).unwrap();
        }
        let delta = writer.save_chunk(&chunk, 77 + k).unwrap();
        let new = fs::metadata(&path).map(|m| m.len() as i64).unwrap_or(0);
        o.push((delta == if k == 2 { -old } else { new - old }) as u8);
        o.push(path.exists() as u8);
        o.push(save.join("tmp_chunk.dat").exists() as u8);
        if k == 2 {
            fs::remove_dir(save.join("tmp_chunk.dat")).unwrap();
        }
        let tag = crate::storage181::read_gzip_file(&path).unwrap();
        let mut bytes = Vec::new();
        crate::nbt::write_root(&mut bytes, &tag).unwrap();
        i(o, bytes.len() as i32);
        o.extend(bytes);
    }
    loader.tick();
    loader.save_extra_data();
    loader.save_extra_chunk_data(&chunk);
    fs::write(save.join("session.lock"), [0; 8]).unwrap();
    o.push(writer.save_chunk(&chunk, 80).is_err() as u8);
}

fn f(o: &mut Vec<u8>, v: f32) {
    i(o, if v.is_nan() { 0x7fc00000 } else { v.to_bits() } as i32);
}
fn piston_snapshot(o: &mut Vec<u8>, p: &super::super::pistons::Piston, invalid: bool) {
    i(o, p.block_id as i32);
    i(o, p.block_data);
    i(o, p.facing);
    o.extend([p.extending as u8, p.render_head as u8]);
    f(o, p.progress);
    f(o, p.progress(0.0));
    o.push(invalid as u8);
}
fn piston_cells(o: &mut Vec<u8>, w: &mut World, x: i32, y: i32, z: i32, face: usize) {
    let (dx, dy, dz) = super::super::pistons::DIRECTIONS[face];
    for j in -1..=15 {
        let (px, py, pz) = (x + dx * j, y + dy * j, z + dz * j);
        o.push(w.get_block_id(px, py, pz));
        o.push(w.get_block_metadata(px, py, pz) as u8);
        let tile = if (0..128).contains(&py) {
            w.get_block_tile_entity(px, py, pz).cloned()
        } else {
            None
        };
        if let Some(super::super::tile::TileEntity::Piston(p)) = tile {
            o.push(1);
            piston_snapshot(o, &p, false);
        } else {
            o.push(0);
        }
    }
    l(o, w.rand.state());
    super::super::events::write_events(&w.drain_events(), o);
}
pub(super) fn pistons(o: &mut Vec<u8>) {
    use super::super::{
        pistons::{Piston, DIRECTIONS},
        tile::TileEntity,
    };
    for side in super::super::pistons::OPPOSITE_SIDES {
        i(o, side);
    }
    for axis in 0..3 {
        for (x, y, z) in DIRECTIONS {
            i(o, [x, y, z][axis]);
        }
    }
    for id in [29, 33] {
        for face in 0..6 {
            for config in 0..24 {
                i(o, id as i32);
                i(o, face);
                i(o, config);
                let mut w = fixture(true);
                w.rand.set_seed(42);
                w.entity_seed_source = crate::entity::EntitySeedSource::Deterministic(100);
                w.math_random.set_seed(200);
                let (x, y, z) = (
                    8,
                    if config == 22 {
                        1
                    } else if config == 23 {
                        126
                    } else {
                        72
                    },
                    8,
                );
                raw(&mut w, x, y, z, id, face);
                let (lx, lz) = if face == 4 { (x, z - 1) } else { (x - 1, z) };
                raw(&mut w, lx, y - 1, lz, 1, 0);
                raw(&mut w, lx, y, lz, 69, 13);
                let count = if config < 14 { config } else { 1 };
                let carried = if config < 14 {
                    1
                } else {
                    [49, 7, 54, 23, 98, 103, 107, 106][(config - 14).min(7) as usize]
                };
                let (dx, dy, dz) = DIRECTIONS[face as usize];
                for j in 1..=count {
                    raw(&mut w, x + dx * j, y + dy * j, z + dz * j, carried, 0);
                }
                behavior::on_neighbor_block_change(&mut w, id, x, y, z, 69);
                piston_cells(o, &mut w, x, y, z, face as usize);
                for _ in 0..4 {
                    w.update_entities();
                    piston_cells(o, &mut w, x, y, z, face as usize);
                }
                raw(&mut w, lx, y, lz, 0, 0);
                behavior::on_neighbor_block_change(&mut w, id, x, y, z, 69);
                piston_cells(o, &mut w, x, y, z, face as usize);
                for _ in 0..4 {
                    w.update_entities();
                    piston_cells(o, &mut w, x, y, z, face as usize);
                }
            }
        }
    }
    for face in 0..6 {
        for extend in [false, true] {
            for bits in [
                0, 0x80000000, 0x3f000000, 0x3f800000, 0x3fc00000, 0xbf800000, 0x7f800000,
                0xff800000, 0x7fc01234,
            ] {
                let mut w = fixture(true);
                raw(&mut w, 8, 72, 8, 36, face);
                let mut n = Compound::new();
                n.put("blockId", Tag::Int(1));
                n.put("blockData", Tag::Int(-123));
                n.put("facing", Tag::Int(face));
                n.put("progress", Tag::Float(f32::from_bits(bits)));
                n.put("extending", Tag::Byte(extend as i8));
                w.set_block_tile_entity(8, 72, 8, TileEntity::Piston(Piston::from_nbt(&n)));
                let id = w.chunk(0, 0).tile_entity_ids[&(8, 72, 8)];
                for _ in 0..4 {
                    let Some(TileEntity::Piston(p)) = w.tile_object(id).cloned() else {
                        panic!()
                    };
                    piston_snapshot(o, &p, w.tile_objects[&id].invalid);
                    for partial in [-1.0, 0.0, 0.5, 1.0, 2.0, f32::NAN] {
                        f(o, p.progress(partial));
                        for v in p.offset(partial) {
                            f(o, v);
                        }
                    }
                    let saved = p.to_nbt(8, 72, 8);
                    f(
                        o,
                        match saved.get("progress") {
                            Some(Tag::Float(v)) => *v,
                            _ => panic!(),
                        },
                    );
                    let bb = w.block_collision_box(36, 8, 72, 8);
                    o.push(bb.is_some() as u8);
                    if let Some(bb) = bb {
                        for v in [bb.min_x, bb.min_y, bb.min_z, bb.max_x, bb.max_y, bb.max_z] {
                            o.extend(
                                (if v.is_nan() {
                                    0x7ff8000000000000
                                } else {
                                    v.to_bits()
                                })
                                .to_le_bytes(),
                            );
                        }
                    }
                    if !f32::from_bits(bits).is_infinite() {
                        w.tick_piston(id, p);
                    }
                }
            }
        }
    }
}

fn stack(o: &mut Vec<u8>, s: Option<super::super::tile::ItemStack>) {
    o.push(s.is_some() as u8);
    if let Some(s) = s {
        i(o, s.id as i32);
        i(o, s.count);
        i(o, s.damage as i32);
    }
}
pub(super) fn removal(o: &mut Vec<u8>) {
    use super::super::tile::{ItemStack, TileEntity};
    for block in [54, 23, 61, 62] {
        for config in 0..8 {
            let mut w = fixture(true);
            w.set_block(8, 64, 8, block);
            w.get_block_tile_entity(8, 64, 8);
            let tile_id = w.chunk(0, 0).tile_entity_ids[&(8, 64, 8)];
            let size = if block == 54 {
                36
            } else if block == 23 {
                9
            } else {
                3
            };
            let mut items = vec![None; size];
            for slot in 0..if block == 54 { 27 } else { size } {
                if slot % 3 == 0 {
                    items[slot] = Some(ItemStack::new(
                        if slot % 2 == 0 { 4 } else { 264 },
                        [0, 1, 9, 10, 21, 64, -1, 127][config],
                        slot as i32 + 2,
                    ));
                }
            }
            if block == 54 {
                items[35] = Some(ItemStack::new(1, 64, 3));
            }
            let tile = match block {
                54 => TileEntity::Chest(items.into_boxed_slice()),
                23 => TileEntity::Dispenser(Box::new(std::array::from_fn(|n| {
                    items[n].map(Into::into)
                }))),
                _ => TileEntity::Furnace(crate::container::tiles::Furnace {
                    items: std::array::from_fn(|n| items[n].map(Into::into)),
                    ..Default::default()
                }),
            };
            w.update_tile_object(tile_id, tile);
            if config == 7 {
                w.tile_objects.get_mut(&tile_id).unwrap().invalid = true;
            }
            let rng_index = match block {
                61 => 0,
                62 => 1,
                _ => 2,
            };
            if block == 54 {
                w.chest_random.set_seed(500);
            } else {
                w.container_drop_random[rng_index].set_seed(500);
            }
            w.rand.set_seed(42);
            w.math_random.set_seed(200);
            w.entity_seed_source = crate::entity::EntitySeedSource::Deterministic(100);
            w.set_block_with_notify(8, 64, 8, 0);
            o.push(w.tile_objects[&tile_id].invalid as u8);
            let items: Vec<Option<ItemStack>> = match w.tile_object(tile_id).unwrap() {
                TileEntity::Chest(s) => s.to_vec(),
                TileEntity::Dispenser(s) => s.iter().map(|s| s.map(Into::into)).collect(),
                TileEntity::Furnace(s) => s.items.iter().map(|s| s.map(Into::into)).collect(),
                _ => panic!(),
            };
            for s in items {
                stack(o, s);
            }
            l(
                o,
                if block == 54 {
                    w.chest_random.state()
                } else {
                    w.container_drop_random[rng_index].state()
                },
            );
            l(o, w.rand.state());
            i(o, w.loaded_entity_list.len() as i32);
            for &id in &w.loaded_entity_list {
                let e = &w.entities[&id];
                let crate::entity::EntityKind::Item { item: s, .. } = e.kind else {
                    panic!()
                };
                stack(o, Some(s));
                for v in [
                    e.pos.x, e.pos.y, e.pos.z, e.motion.x, e.motion.y, e.motion.z,
                ] {
                    o.extend(v.to_bits().to_le_bytes());
                }
            }
            super::super::events::write_events(&w.drain_events(), o);
        }
    }
}

pub(super) fn mchash(o: &mut Vec<u8>) {
    use super::super::mchash181::{compute_hash, MCHash181};
    fn value(o: &mut Vec<u8>, v: Option<i32>) {
        o.push(v.is_some() as u8);
        if let Some(v) = v {
            i(o, v);
        }
    }
    let mut map = MCHash181::default();
    let mut rng = crate::JavaRandom::new(42);
    for t in 0..1024 {
        let key = if t < 32 {
            t % 8
        } else if t < 64 {
            (t - 32) * 16
        } else if t < 96 {
            [i32::MIN, i32::MAX, -1, 0][t as usize % 4]
        } else {
            rng.next_int(257) - 128
        };
        i(o, compute_hash(key));
        let action = if t < 128 { 0 } else { rng.next_int(8) };
        match action {
            0..=3 => map.add_key(key, if t % 7 == 0 { None } else { Some(t) }),
            4 => value(o, map.remove_object(key)),
            5 => {
                let e = map.remove_entry(key);
                value(o, e.and_then(|id| map.entries[id].value));
            }
            6 => {
                map.keys.insert(key, key);
            }
            _ => {
                map.keys.remove(key, key);
            }
        }
        if t == 250 || t == 750 {
            map.clear_map();
        }
        if t == 500 {
            map.version_stamp = i32::MAX;
        }
        i(o, map.slots.len() as i32);
        i(o, map.count as i32);
        i(o, map.threshold as i32);
        i(o, map.version_stamp);
        for &first in &map.slots {
            let mut e = first;
            let mut n = 0;
            while let Some(id) = e {
                n += 1;
                e = map.entries[id].next;
            }
            i(o, n);
            let mut e = first;
            while let Some(id) = e {
                let entry = &map.entries[id];
                i(o, entry.key);
                i(o, entry.hash);
                value(o, entry.value);
                e = entry.next;
            }
        }
        i(o, map.keys.len() as i32);
        for key in map.keys.iter() {
            i(o, key);
        }
        for key in [0, 1, -1, i32::MIN, i32::MAX, 16, 64, 128] {
            o.push(map.contains(key) as u8);
            value(o, map.lookup(key).copied());
        }
    }
}

pub(super) fn chest_open(o: &mut Vec<u8>) {
    use super::super::tile::ItemStack;
    for mask in 0..16 {
        for invalid in 0..32 {
            for roof in 0..6 {
                let mut w = fixture(true);
                let positions = [(8, 8), (7, 8), (9, 8), (8, 7), (8, 9)];
                for n in 0..5 {
                    if n == 0 || mask & (1 << (n - 1)) != 0 {
                        let (x, z) = positions[n];
                        raw(&mut w, x, 64, z, 54, 0);
                        w.get_block_tile_entity(x, 64, z);
                        let id = w.chunk(0, 0).tile_entity_ids[&(x, 64, z)];
                        w.set_chest_stack181(
                            id,
                            0,
                            Some(ItemStack::new(4 + n as i32, n as i32 + 1, n as i32)),
                        );
                        if invalid & (1 << n) != 0 {
                            w.invalidate_tile181(id);
                        }
                    }
                }
                if roof > 0 {
                    let (x, z) = positions[roof - 1];
                    raw(&mut w, x, 65, z, 1, 0);
                }
                o.push(1);
                let c = w.make_block_container(54, 8, 64, 8, 1);
                o.push(c.is_some() as u8);
                if let Some(c) = c {
                    i(o, c.tile.len() as i32);
                    for s in c.tile {
                        stack(o, s.map(Into::into));
                    }
                }
            }
        }
    }
}

pub(super) fn flow(o: &mut Vec<u8>) {
    let above = [0, 1, 79, 30, 106, 20];
    for block in [8, 9, 10, 11] {
        let mut w = fixture(true);
        for meta in 0..16 {
            for config in 0..256 {
                raw(&mut w, 8, 64, 8, block, meta);
                for (k, (dx, dz)) in [(-1, 0), (0, -1), (1, 0), (0, 1)].into_iter().enumerate() {
                    let k = k as i32;
                    raw(
                        &mut w,
                        8 + dx,
                        64,
                        8 + dz,
                        ((config + 37 * k) % 110) as u8,
                        (config + 3 * k) & 15,
                    );
                    raw(
                        &mut w,
                        8 + dx,
                        63,
                        8 + dz,
                        (8 + (config + 3 * k) % 4) as u8,
                        (config + 5 * k) & 15,
                    );
                    raw(
                        &mut w,
                        8 + dx,
                        65,
                        8 + dz,
                        above[((config / 16 + k) % 6) as usize],
                        0,
                    );
                }
                let v = w.fluid_flow(8, 64, 8, crate::block::props(block).material);
                for value in [v.x, v.y, v.z] {
                    o.extend(value.to_bits().to_le_bytes());
                }
            }
        }
    }
}
