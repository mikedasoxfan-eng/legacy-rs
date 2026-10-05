//! Direct, original-bytecode checks for the five round-ten classes.
use super::*;
use crate::world::{tile::TileEntity, tile_nbt181::TileError181};

fn error(o: &mut Vec<u8>, result: Result<(), TileError181>) {
    o.push(match result {
        Ok(()) => 0,
        Err(TileError181::ClassCast) => 1,
        Err(TileError181::ArrayIndexOutOfBounds) => 2,
        Err(TileError181::NullPointer) => 3,
        Err(TileError181::MissingMapping) => 4,
    });
}
fn nbt(o: &mut Vec<u8>, n: &Compound) {
    let mut bytes = Vec::new();
    crate::nbt::write_root(&mut bytes, n).unwrap();
    i(o, bytes.len() as i32);
    o.extend(bytes);
}
fn tag(ty: i32) -> Tag {
    match ty {
        1 => Tag::Byte(-1),
        2 => Tag::Short(-1),
        3 => Tag::Int(-1),
        4 => Tag::Long(-1),
        5 => Tag::Float(-1.0),
        6 => Tag::Double(-1.0),
        7 => Tag::ByteArray(vec![255]),
        8 => Tag::String("bad".into()),
        9 => Tag::List(1, vec![Tag::Byte(-1)]),
        10 => Tag::Compound(Compound::new()),
        _ => unreachable!(),
    }
}
fn input(id: i32, slot: i32) -> Compound {
    let mut n = Compound::new();
    for (key, v) in [("x", i32::MIN), ("y", i32::MAX), ("z", -12345)] {
        n.put(key, Tag::Int(v));
    }
    n.put("BurnTime", Tag::Short(-32768));
    n.put("CookTime", Tag::Short(32767));
    let mut s = Compound::new();
    s.put("Slot", Tag::Byte(slot as i8));
    s.put("id", Tag::Short(id as i16));
    s.put("Count", Tag::Byte(id as i8));
    s.put("Damage", Tag::Short((id.wrapping_mul(17)) as i16));
    n.put("Items", Tag::List(10, vec![Tag::Compound(s)]));
    n
}
fn machine_case(o: &mut Vec<u8>, block: u8, n: &Compound) {
    let mut t = TileEntity::for_block(block);
    if let TileEntity::Furnace(f) = &mut t {
        f.items
            .fill(Some(crate::container::ItemStack::new(263, 12, 7)));
        f.burn = 99;
        f.cook = -99;
        f.current_fuel = 123;
    }
    if let TileEntity::Dispenser(items) = &mut t {
        items.fill(Some(crate::container::ItemStack::new(263, 12, 7)));
    }
    let mut pos = (7, 8, 9);
    error(o, t.read_machine_nbt181(&mut pos, n));
    t.open_machine181();
    t.close_machine181();
    let out = match &t {
        TileEntity::Furnace(f) => {
            for v in [f.burn, f.current_fuel, f.cook] {
                i(o, v);
            }
            f.to_nbt(pos.0, pos.1, pos.2)
        }
        TileEntity::Dispenser(items) => {
            crate::container::tiles::inventory_nbt("Trap", pos.0, pos.1, pos.2, &**items)
        }
        _ => unreachable!(),
    };
    // readFromNBT and the empty inventory hooks preserve base caches/validity.
    i(o, 11);
    i(o, 1);
    o.push(1);
    nbt(o, &out);
}
pub(super) fn furnace(o: &mut Vec<u8>) {
    machine(o, 61);
}
pub(super) fn dispenser(o: &mut Vec<u8>) {
    machine(o, 23);
    // Detached and attached tiles, every occupancy mask, zero/negative counts.
    for mask in 0..512 {
        let mut w = fixture(true);
        raw(&mut w, 8, 64, 8, 23, 5);
        w.get_block_tile_entity(8, 64, 8);
        let id = w.chunk(0, 0).tile_entity_ids[&(8, 64, 8)];
        for slot in 0..9 {
            if mask & (1 << slot) != 0 {
                w.set_machine_stack181(
                    id,
                    slot,
                    Some(crate::container::ItemStack::new(
                        263,
                        slot as i32 - 3,
                        slot as i32,
                    )),
                );
            }
        }
        if mask & 1 != 0 {
            w.set_block_tile_entity(8, 64, 8, TileEntity::for_block(23));
        }
        w.set_dispenser_seed181(id, 42);
        for _ in 0..12 {
            w.chunk_mut(0, 0).modified = false;
            let s = w.random_dispenser_stack181(id);
            o.push(s.is_some() as u8);
            if let Some(s) = s {
                for v in [s.id, s.count, s.damage] {
                    i(o, v);
                }
            }
            for slot in 0..9 {
                let s = w.machine_stack181(id, slot);
                o.push(s.is_some() as u8);
                if let Some(s) = s {
                    for v in [s.id, s.count, s.damage] {
                        i(o, v);
                    }
                }
            }
            l(
                o,
                w.tile_objects[&id]
                    .dispenser_random181
                    .as_ref()
                    .unwrap()
                    .state(),
            );
            i(o, w.tile_objects[&id].cached_metadata);
            o.push(w.chunk(0, 0).modified as u8);
        }
    }
}
fn machine(o: &mut Vec<u8>, block: u8) {
    for id in -32768..=32767 {
        machine_case(o, block, &input(id, 0));
    }
    for slot in -128..=127 {
        for id in [-32768, -1, 0, 15, 263, 500, 31999, 32000, 32767] {
            machine_case(o, block, &input(id, slot));
        }
    }
    for key in ["x", "y", "z", "Items", "BurnTime", "CookTime"] {
        for ty in 1..=10 {
            let mut n = input(263, 1);
            n.put(key, tag(ty));
            machine_case(o, block, &n);
        }
    }
    for key in ["Slot", "id", "Count", "Damage"] {
        for ty in 1..=10 {
            let mut n = input(-1, 1);
            let Some(Tag::List(_, items)) =
                n.0.iter_mut().find(|e| e.0 == "Items").map(|e| &mut e.1)
            else {
                unreachable!()
            };
            let Tag::Compound(s) = &mut items[0] else {
                unreachable!()
            };
            s.put(key, tag(ty));
            machine_case(o, block, &n);
        }
    }
    for ty in 1..=10 {
        let mut n = input(15, 0);
        let Some(Tag::List(_, items)) = n.0.iter_mut().find(|e| e.0 == "Items").map(|e| &mut e.1)
        else {
            unreachable!()
        };
        items.push(tag(ty));
        machine_case(o, block, &n);
    }
    for id in [263, 500, 0, -1, 32000] {
        let mut n = input(15, 0);
        let other = input(id, 0);
        let Some(Tag::List(_, items)) = n.0.iter_mut().find(|e| e.0 == "Items").map(|e| &mut e.1)
        else {
            unreachable!()
        };
        items.extend_from_slice(other.list("Items"));
        machine_case(o, block, &n);
    }
    machine_case(o, block, &Compound::new());
}
pub(super) fn jukebox(o: &mut Vec<u8>) {
    for side in [i32::MIN, -1, 0, 1, 2, 5, 6, i32::MAX] {
        i(
            o,
            crate::block::visual181::texture_from_side(84, side as usize, true),
        );
    }
    let t = TileEntity::for_block(84);
    o.push(matches!(t, TileEntity::RecordPlayer { record: 0 }) as u8);
    o.push(matches!(
        TileEntity::for_block(84),
        TileEntity::RecordPlayer { record: 0 }
    ) as u8);
    for remote in [false, true] {
        for meta in 0..16 {
            for record in [
                i32::MIN,
                -65537,
                -1,
                0,
                1,
                2256,
                2257,
                32768,
                65536,
                i32::MAX,
            ] {
                for config in 0..5 {
                    for hook in 0..5 {
                        let mut w = fixture(true);
                        raw(&mut w, 8, 64, 8, 84, meta);
                        w.set_block_tile_entity(8, 64, 8, TileEntity::RecordPlayer { record });
                        let old = w.chunk(0, 0).tile_entity_ids[&(8, 64, 8)];
                        if config == 1 {
                            w.tile_objects.get_mut(&old).unwrap().invalid = true;
                        }
                        if config == 2 {
                            w.remove_block_tile_entity(8, 64, 8);
                            raw(&mut w, 8, 64, 8, 1, meta);
                        }
                        if config == 3 {
                            w.set_block_tile_entity(8, 64, 8, TileEntity::for_block(61));
                        }
                        if config == 4 {
                            w.remove_block_tile_entity(8, 64, 8);
                            w.scanning_tiles = true;
                        }
                        w.multiplayer_world = remote;
                        round6::seed(&mut w);
                        w.events.clear();
                        w.chunk_mut(0, 0).modified = false;
                        let result = match hook {
                            0 => w.try_set_record181(8, 64, 8, record),
                            1 => w.try_eject_record181(8, 64, 8),
                            2 => {
                                let active = w.get_block_metadata(8, 64, 8) != 0;
                                o.push(active as u8);
                                if active {
                                    w.try_eject_record181(8, 64, 8)
                                } else {
                                    Ok(())
                                }
                            }
                            3 => {
                                let result = w.try_eject_record181(8, 64, 8);
                                if result.is_ok() {
                                    w.remove_block_tile_entity(8, 64, 8);
                                }
                                result
                            }
                            4 => {
                                behavior::drop_block_with_chance(&mut w, 84, 8, 64, 8, meta, 0.75);
                                Ok(())
                            }
                            _ => unreachable!(),
                        };
                        error(o, result);
                        let TileEntity::RecordPlayer { record: retained } =
                            w.tile_object(old).unwrap()
                        else {
                            unreachable!()
                        };
                        i(o, *retained);
                        i(o, w.tile_objects[&old].cached_metadata);
                        o.push(w.tile_objects[&old].invalid as u8);
                        o.push(w.chunk(0, 0).modified as u8);
                        round6::result(o, &mut w);
                    }
                }
            }
        }
    }
}
fn base_snapshot(o: &mut Vec<u8>, base: &crate::world::tile_nbt181::TileBase181, w: &World) {
    for v in [
        base.position.0,
        base.position.1,
        base.position.2,
        base.cached_metadata,
        base.cached_block.unwrap_or(0) as i32,
    ] {
        i(o, v);
    }
    o.push(base.invalid as u8);
    o.push(w.chunk(0, 0).modified as u8);
}
pub(super) fn tileentity(o: &mut Vec<u8>) {
    use crate::world::tile_nbt181::TileBase181;
    // Stateful superclass calls, including detached null worlds and null block caches.
    let mut w = fixture(true);
    let mut base = TileBase181 {
        position: (8, 64, 8),
        ..Default::default()
    };
    let mut rng = crate::JavaRandom::new(1729);
    for step in 0..20000 {
        let op = rng.next_int(10);
        let block = [0, 1, 23, 61, 84, 110, 255][rng.next_int(7) as usize];
        let meta = rng.next_int(16);
        raw(&mut w, 8, 64, 8, block, meta);
        w.chunk_mut(0, 0).modified = false;
        let attached = step % 3 != 0;
        match op {
            0 => {
                let result = base.metadata(attached.then_some(&mut w));
                error(o, result.map(|_| ()));
                if let Ok(v) = result {
                    i(o, v);
                }
            }
            1 => {
                let result = base.block(attached.then_some(&mut w));
                error(o, result.map(|_| ()));
                if let Ok(v) = result {
                    i(o, v.unwrap_or(0) as i32);
                }
            }
            2 => base.inventory_changed(attached.then_some(&mut w)),
            3 => base.invalidate(),
            4 => base.validate(),
            5 => base.reset_cache(),
            6 => base.client_event(i32::MIN, i32::MAX),
            7 => base.update_entity(),
            8 => {
                let mut n = Compound::new();
                n.put("keep", Tag::Byte(7));
                error(
                    o,
                    base.write_to_nbt(&mut n, if attached { Some("Music") } else { None }),
                );
                nbt(o, &n);
            }
            9 => {
                let mut n = Compound::new();
                n.put("x", Tag::Int(8));
                n.put("y", Tag::Int(64));
                n.put("z", Tag::Int(8));
                error(o, base.read_from_nbt(&n));
            }
            _ => unreachable!(),
        }
        base_snapshot(o, &base, &w);
    }
    for key in ["x", "y", "z"] {
        for ty in 1..=10 {
            let mut base = TileBase181 {
                position: (7, 8, 9),
                cached_metadata: 11,
                cached_block: Some(1),
                invalid: true,
            };
            let mut n = input(15, 0);
            n.put(key, tag(ty));
            error(o, base.read_from_nbt(&n));
            base_snapshot(o, &base, &w);
        }
    }
    for pos in [(0, 0, 0), (i32::MIN, i32::MAX, -12345), (-1, 64, -1)] {
        base.position = pos;
        for value in [
            f64::NEG_INFINITY,
            -f64::MAX,
            -123.5,
            -0.0,
            0.0,
            0.5,
            123.5,
            f64::MAX,
            f64::INFINITY,
            f64::NAN,
        ] {
            let d = base.distance(value, value, value);
            o.extend(
                if d.is_nan() {
                    f64::NAN.to_bits()
                } else {
                    d.to_bits()
                }
                .to_le_bytes(),
            );
        }
    }
    for name in [
        "Furnace",
        "Chest",
        "RecordPlayer",
        "Trap",
        "Sign",
        "MobSpawner",
        "Music",
        "Piston",
        "",
        "Unknown",
        "furnace",
        "Furnace\0",
    ] {
        for malformed in 0..5 {
            let mut n = Compound::new();
            n.put("id", Tag::String(name.into()));
            n.put("x", Tag::Int(-123));
            n.put("y", Tag::Int(64));
            n.put("z", Tag::Int(456));
            if malformed == 1 {
                n.put("x", Tag::Byte(1));
            }
            if malformed == 2 {
                n.put("id", Tag::Int(1));
            }
            if malformed == 3 {
                n.put("Items", Tag::Int(1));
            }
            if malformed == 4 {
                n.put("BurnTime", Tag::Byte(1));
            }
            match TileEntity::create_and_load181(&n) {
                Err(e) => {
                    error(o, Err(e));
                    o.push(0);
                }
                Ok(tile) => {
                    error(o, Ok(()));
                    o.push(tile.is_some() as u8);
                    if let Some((base, tile)) = tile {
                        base_snapshot(o, &base, &w);
                        nbt(o, &tile.write_nbt181(base.position));
                    }
                }
            }
        }
    }
}
fn f(o: &mut Vec<u8>, v: f32) {
    i(
        o,
        if v.is_nan() {
            f32::NAN.to_bits()
        } else {
            v.to_bits()
        } as i32,
    );
}
fn piston_state(
    o: &mut Vec<u8>,
    p: &crate::world::pistons::Piston,
    pos: (i32, i32, i32),
    invalid: bool,
) {
    for v in [pos.0, pos.1, pos.2, p.block_id, p.block_data, p.facing] {
        i(o, v);
    }
    o.extend([p.extending as u8, p.render_head as u8]);
    f(o, p.progress);
    f(o, p.last_progress);
    o.push(invalid as u8);
    nbt(o, &p.to_nbt(pos.0, pos.1, pos.2));
    for partial in [
        f32::NEG_INFINITY,
        -1.0,
        -0.0,
        0.0,
        0.5,
        1.0,
        2.0,
        f32::INFINITY,
        f32::NAN,
    ] {
        f(o, p.progress(partial));
        let result = p.offset181(partial);
        error(o, result.map(|_| ()));
        if let Ok(offset) = result {
            for v in offset {
                f(o, v);
            }
        }
    }
}
pub(super) fn piston(o: &mut Vec<u8>) {
    use crate::world::pistons::Piston;
    let ids = [
        i32::MIN,
        -257,
        -1,
        0,
        1,
        8,
        36,
        110,
        255,
        256,
        257,
        65537,
        i32::MAX,
    ];
    let faces = [i32::MIN, -1, 0, 1, 2, 3, 4, 5, 6, i32::MAX];
    let bits = [
        0, 0x80000000, 0x3f000000, 0x3f800000, 0x3fc00000, 0xbf800000, 0x7f800000, 0xff800000,
        0x7fc01234,
    ];
    for id in ids {
        for face in faces {
            for extend in [false, true] {
                for head in [false, true] {
                    for bit in bits {
                        let mut p = Piston::new(id, -12345, face, extend, head);
                        p.progress = f32::from_bits(bit);
                        p.last_progress = 0.25;
                        piston_state(o, &p, (-123, 64, 456), true);
                        let n = p.to_nbt(-123, 64, 456);
                        let mut tile = TileEntity::Piston(Piston::new(99, 98, 5, !extend, head));
                        let mut pos = (7, 8, 9);
                        error(o, tile.read_piston_nbt181(&mut pos, &n));
                        let TileEntity::Piston(p) = tile else {
                            unreachable!()
                        };
                        piston_state(o, &p, pos, false);
                    }
                }
            }
        }
    }
    for key in [
        "x",
        "y",
        "z",
        "blockId",
        "blockData",
        "facing",
        "progress",
        "extending",
    ] {
        for ty in 1..=10 {
            let mut n = Piston::new(-257, i32::MIN, i32::MAX, true, false).to_nbt(-123, 64, 456);
            n.put(key, tag(ty));
            let mut tile = TileEntity::Piston(Piston::new(99, 98, 5, false, true));
            let mut pos = (7, 8, 9);
            error(o, tile.read_piston_nbt181(&mut pos, &n));
            let TileEntity::Piston(p) = tile else {
                unreachable!()
            };
            piston_state(o, &p, pos, true);
        }
    }
    for remote in [false, true] {
        for id in ids {
            for face in [-1, 0, 5, 6] {
                for extend in [false, true] {
                    for bit in [0, 0x3f000000, 0x3f800000, 0x7fc01234] {
                        for config in 0..3 {
                            for hook in 0..2 {
                                let mut w = fixture(true);
                                raw(&mut w, 8, 64, 8, 36, face);
                                let mut p = Piston::new(id, -12345, face, extend, true);
                                p.progress = f32::from_bits(bit);
                                p.last_progress = p.progress;
                                w.set_block_tile_entity(8, 64, 8, TileEntity::Piston(p.clone()));
                                let old = w.chunk(0, 0).tile_entity_ids[&(8, 64, 8)];
                                if config == 1 {
                                    raw(&mut w, 8, 64, 8, 1, 7);
                                }
                                if config == 2 {
                                    w.set_block_tile_entity(
                                        8,
                                        64,
                                        8,
                                        TileEntity::Piston(Piston::new(1, 0, 0, false, false)),
                                    );
                                    w.scanning_tiles = true;
                                }
                                w.multiplayer_world = remote;
                                round6::seed(&mut w);
                                w.events.clear();
                                w.chunk_mut(0, 0).modified = false;
                                error(
                                    o,
                                    if hook == 0 {
                                        w.tick_piston181(old, &mut p)
                                    } else {
                                        w.clear_piston_tile181(old)
                                    },
                                );
                                let TileEntity::Piston(p) = w.tile_object(old).unwrap() else {
                                    unreachable!()
                                };
                                piston_state(o, p, (8, 64, 8), w.tile_objects[&old].invalid);
                                o.push(w.loaded_tile_entities.contains(&old) as u8);
                                o.push(w.chunk(0, 0).modified as u8);
                                round6::result(o, &mut w);
                            }
                        }
                    }
                }
            }
        }
    }
}
