use super::*;
use crate::block::*;
pub fn drop_stack(w: &mut World, x: i32, y: i32, z: i32, stack: ItemStack) {
    let s = 0.7f32;
    let ox = (w.rand.next_float() * s) as f64 + (1.0 - s) as f64 * 0.5;
    let oy = (w.rand.next_float() * s) as f64 + (1.0 - s) as f64 * 0.5;
    let oz = (w.rand.next_float() * s) as f64 + (1.0 - s) as f64 * 0.5;
    let mut e = Entity::item(w, x as f64 + ox, y as f64 + oy, z as f64 + oz, stack.into());
    if let EntityKind::Item { pickup_delay, .. } = &mut e.kind {
        *pickup_delay = 10;
    }
    w.entity_joined_world(e);
}
/// Empty inherited Block hooks, explicitly enumerated for player-placeable blocks.
fn plain(id: u8) -> bool {
    matches!(
        id,
        20 | 21
            | 22
            | 25
            | 26
            | 30
            | 35
            | 41
            | 42
            | 47
            | 53
            | 58
            | 64
            | 65
            | 67
            | 69
            | 71
            | 77
            | 84
            | 85
            | 89
            | 91
            | 92
            | 96
    )
}
pub fn added(w: &mut World, id: u8, x: i32, y: i32, z: i32) -> bool {
    if matches!(id, 25 | 84) {
        ensure_container(w, id, x, y, z);
        return true;
    }
    if matches!(id, 93 | 94) {
        for (dx, dy, dz) in [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 0, 1),
            (0, 0, -1),
            (0, -1, 0),
            (0, 1, 0),
        ] {
            w.notify_blocks_of_neighbor_change(x + dx, y + dy, z + dz, id);
        }
        return true;
    }
    if matches!(id, 6 | 59) {
        return true;
    }
    if matches!(id, 43 | 44) {
        if id == 44
            && w.get_block_id(x, y - 1, z) == 44
            && w.get_block_metadata(x, y, z) == w.get_block_metadata(x, y - 1, z)
        {
            let m = w.get_block_metadata(x, y, z);
            w.set_block_with_notify(x, y, z, 0);
            w.set_block_and_metadata_with_notify(x, y - 1, z, 43, m);
        }
        return true;
    }
    if id == 50 {
        if w.get_block_metadata(x, y, z) != 0 {
            neighbor(w, id, x, y, z, 0);
            return true;
        }
        let mut meta = None;
        for (dx, dy, dz, m) in [
            (-1, 0, 0, 1),
            (1, 0, 0, 2),
            (0, 0, -1, 3),
            (0, 0, 1, 4),
            (0, -1, 0, 5),
        ] {
            if w.is_block_normal_cube(x + dx, y + dy, z + dz)
                || (m == 5 && w.get_block_id(x, y - 1, z) == 85)
            {
                meta = Some(m);
                break;
            }
        }
        if let Some(m) = meta {
            w.set_block_metadata_with_notify(x, y, z, m);
        }
        neighbor(w, id, x, y, z, 0);
        return true;
    }
    if matches!(id, 23 | 61 | 62) {
        ensure_container(w, id, x, y, z);
        let north = opaque_cube_lookup(w.get_block_id(x, y, z - 1));
        let south = opaque_cube_lookup(w.get_block_id(x, y, z + 1));
        let west = opaque_cube_lookup(w.get_block_id(x - 1, y, z));
        let east = opaque_cube_lookup(w.get_block_id(x + 1, y, z));
        let mut m = 3;
        if south && !north {
            m = 2;
        }
        if west && !east {
            m = 5;
        }
        if east && !west {
            m = 4;
        }
        w.set_block_metadata_with_notify(x, y, z, m);
        return true;
    }
    plain(id)
}
pub fn removed(w: &mut World, id: u8, x: i32, y: i32, z: i32) -> bool {
    if matches!(id, 25 | 84) {
        if id == 84 {
            w.eject_record(x, y, z);
        }
        w.remove_block_tile_entity(x, y, z);
        w.note_power.remove(&(x, y, z));
        return true;
    }
    if matches!(id, 69 | 77) {
        let m = w.get_block_metadata(x, y, z);
        if m & 8 > 0 {
            super::activation::notify_power(w, id, x, y, z, m & 7);
        }
        return true;
    }
    if matches!(id, 23 | 61 | 62) {
        w.chunk_mut(x >> 4, z >> 4)
            .raw_tile_entities
            .retain(|n| n.int("x") != x || n.int("y") != y || n.int("z") != z);
        return true;
    }
    plain(id) || matches!(id, 6 | 43 | 44 | 50 | 59 | 93 | 94)
}
pub fn neighbor(w: &mut World, id: u8, x: i32, y: i32, z: i32, changed: u8) -> bool {
    if id == 25 {
        if matches!(changed, 55 | 69 | 70 | 72 | 75 | 76 | 77 | 93 | 94) {
            let powered = w.is_block_getting_powered(x, y, z);
            let prev = w.note_power.get(&(x, y, z)).copied().unwrap_or(false);
            if prev != powered {
                if powered {
                    w.trigger_note(x, y, z, false);
                }
                w.note_power.insert((x, y, z), powered);
            }
        }
        return true;
    }
    if id == 26 {
        let meta = w.get_block_metadata(x, y, z);
        let (dx, dz) = [(0, 1), (-1, 0), (0, -1), (1, 0)][(meta & 3) as usize];
        let (dx, dz) = if meta & 8 != 0 { (-dx, -dz) } else { (dx, dz) };
        if w.get_block_id(x + dx, y, z + dz) != 26 {
            w.set_block_with_notify(x, y, z, 0);
            if meta & 8 == 0 {
                crate::world::behavior::drop_block_as_item(w, id, x, y, z, meta);
            }
        }
        return true;
    }
    if matches!(id, 93 | 94) {
        let m = w.get_block_metadata(x, y, z);
        if !w.is_block_normal_cube(x, y - 1, z) {
            crate::world::behavior::drop_block_as_item(w, id, x, y, z, m);
            w.set_block_with_notify(x, y, z, 0);
        } else {
            let powered = repeater_input(w, x, y, z, m);
            if (id == 94 && !powered) || (id == 93 && powered) {
                w.schedule_block_update(x, y, z, id, ((m & 12) >> 2) * 2 + 2);
            }
        }
        return true;
    }
    if id == 23 {
        if changed > 0
            && crate::entity::block_hooks::provides_power(changed)
            && (w.is_block_indirectly_getting_powered(x, y, z)
                || w.is_block_indirectly_getting_powered(x, y + 1, z))
        {
            w.schedule_block_update(x, y, z, id, 4);
        }
        return true;
    }
    if id == 96 {
        let m = w.get_block_metadata(x, y, z);
        let (dx, dz) = [(0, 1), (0, -1), (1, 0), (-1, 0)][(m & 3) as usize];
        if !w.is_block_normal_cube(x + dx, y, z + dz) {
            w.set_block_with_notify(x, y, z, 0);
            crate::world::behavior::drop_block_as_item(w, id, x, y, z, m);
        }
        if changed > 0 && crate::entity::block_hooks::provides_power(changed) {
            let powered = w.is_block_indirectly_getting_powered(x, y, z);
            if (w.get_block_metadata(x, y, z) & 4 != 0) != powered {
                let m = w.get_block_metadata(x, y, z);
                w.set_block_metadata_with_notify(x, y, z, m ^ 4);
            }
        }
        return true;
    }
    if id == 59 {
        crop_check(w, x, y, z);
        return true;
    }
    if matches!(id, 64 | 71) {
        let m = w.get_block_metadata(x, y, z);
        if m & 8 != 0 {
            if w.get_block_id(x, y - 1, z) != id {
                w.set_block_with_notify(x, y, z, 0);
            }
            if changed > 0 && crate::entity::block_hooks::provides_power(changed) {
                neighbor(w, id, x, y - 1, z, changed);
            }
        } else {
            let mut removed = false;
            if w.get_block_id(x, y + 1, z) != id {
                w.set_block_with_notify(x, y, z, 0);
                removed = true;
            }
            if !w.is_block_normal_cube(x, y - 1, z) {
                w.set_block_with_notify(x, y, z, 0);
                removed = true;
                if w.get_block_id(x, y + 1, z) == id {
                    w.set_block_with_notify(x, y + 1, z, 0);
                }
            }
            if removed {
                crate::world::behavior::drop_block_as_item(w, id, x, y, z, m);
            } else if changed > 0 && crate::entity::block_hooks::provides_power(changed) {
                let powered = w.is_block_indirectly_getting_powered(x, y, z)
                    || w.is_block_indirectly_getting_powered(x, y + 1, z);
                if (m & 4 != 0) != powered {
                    if w.get_block_id(x, y + 1, z) == id {
                        w.set_block_metadata_with_notify(x, y + 1, z, (m ^ 4) + 8);
                    }
                    w.set_block_metadata_with_notify(x, y, z, m ^ 4);
                }
            }
        }
        return true;
    }
    if matches!(id, 50 | 65 | 69 | 77) {
        let m = w.get_block_metadata(x, y, z);
        let orientation = m & 7;
        let (dx, dy, dz) = if id == 65 {
            match orientation {
                2 => (0, 0, 1),
                3 => (0, 0, -1),
                4 => (1, 0, 0),
                5 => (-1, 0, 0),
                _ => (0, 0, 0),
            }
        } else {
            match orientation {
                1 => (-1, 0, 0),
                2 => (1, 0, 0),
                3 => (0, 0, -1),
                4 => (0, 0, 1),
                _ => (0, -1, 0),
            }
        };
        let valid = w.is_block_normal_cube(x + dx, y + dy, z + dz)
            || (id == 50 && orientation == 5 && w.get_block_id(x, y - 1, z) == 85)
            || orientation == 0;
        if !valid {
            crate::world::behavior::drop_block_as_item(w, id, x, y, z, m);
            w.set_block_with_notify(x, y, z, 0);
        }
        return true;
    }
    if id == 92 {
        if !mat(w.get_block_material(x, y - 1, z)).is_solid {
            w.set_block_with_notify(x, y, z, 0);
        }
        return true;
    }
    plain(id) || matches!(id, 23 | 43 | 44 | 61 | 62)
}
pub fn tick(w: &mut World, id: u8, x: i32, y: i32, z: i32, rs: crate::world::RandSel) -> bool {
    if matches!(id, 93 | 94) {
        let m = w.get_block_metadata(x, y, z);
        let input = repeater_input(w, x, y, z, m);
        if id == 94 && !input {
            w.set_block_and_metadata_with_notify(x, y, z, 93, m);
        } else if id == 93 {
            w.set_block_and_metadata_with_notify(x, y, z, 94, m);
            if !input {
                w.schedule_block_update(x, y, z, 94, ((m & 12) >> 2) * 2 + 2);
            }
        }
        return true;
    }
    if id == 23 {
        return true;
    }
    if id == 59 {
        crop_check(w, x, y, z);
        if w.get_block_light_value(x, y + 1, z) >= 9 {
            let m = w.get_block_metadata(x, y, z);
            if m < 7 {
                let mut rate = 1.0f32;
                let along_x =
                    w.get_block_id(x - 1, y, z) == 59 || w.get_block_id(x + 1, y, z) == 59;
                let along_z =
                    w.get_block_id(x, y, z - 1) == 59 || w.get_block_id(x, y, z + 1) == 59;
                let diagonal = [(-1, -1), (1, -1), (1, 1), (-1, 1)]
                    .into_iter()
                    .any(|(dx, dz)| w.get_block_id(x + dx, y, z + dz) == 59);
                for bx in x - 1..=x + 1 {
                    for bz in z - 1..=z + 1 {
                        let mut r = 0.0;
                        if w.get_block_id(bx, y - 1, bz) == 60 {
                            r = if w.get_block_metadata(bx, y - 1, bz) > 0 {
                                3.0
                            } else {
                                1.0
                            };
                        }
                        if bx != x || bz != z {
                            r /= 4.0;
                        }
                        rate += r;
                    }
                }
                if diagonal || along_x && along_z {
                    rate /= 2.0;
                }
                if w.rng(rs).next_int((100.0 / rate) as i32) == 0 {
                    w.set_block_metadata_with_notify(x, y, z, m + 1);
                }
            }
        }
        return true;
    }
    if id == 77 {
        let m = w.get_block_metadata(x, y, z);
        if m & 8 != 0 {
            w.set_block_metadata_with_notify(x, y, z, m & 7);
            super::activation::notify_power(w, id, x, y, z, m & 7);
        }
        return true;
    }
    if id == 50 {
        if w.get_block_metadata(x, y, z) == 0 {
            added(w, id, x, y, z);
        }
        return true;
    }
    false
}
fn crop_check(w: &mut World, x: i32, y: i32, z: i32) {
    if !crate::world::behavior::flower_can_stay(w, 59, x, y, z) {
        let m = w.get_block_metadata(x, y, z);
        crate::world::behavior::drop_block_as_item(w, 59, x, y, z, m);
        w.set_block_with_notify(x, y, z, 0);
    }
}
pub fn repeater_input(w: &mut World, x: i32, y: i32, z: i32, m: i32) -> bool {
    let (dx, dz, side) = [(0, 1, 3), (-1, 0, 4), (0, -1, 2), (1, 0, 5)][(m & 3) as usize];
    w.indirectly_provides_power(x + dx, y, z + dz, side)
        || (w.get_block_id(x + dx, y, z + dz) == 55 && w.get_block_metadata(x + dx, y, z + dz) > 0)
}
pub fn ensure_container(w: &mut World, id: u8, x: i32, y: i32, z: i32) {
    if w.chunk(x >> 4, z >> 4)
        .raw_tile_entities
        .iter()
        .any(|n| n.int("x") == x && n.int("y") == y && n.int("z") == z)
    {
        return;
    }
    let mut n = crate::nbt::Compound::new();
    n.put(
        "id",
        crate::nbt::Tag::String(
            match id {
                23 => "Trap",
                25 => "Music",
                84 => "RecordPlayer",
                _ => "Furnace",
            }
            .into(),
        ),
    );
    for (key, v) in [("x", x), ("y", y), ("z", z)] {
        n.put(key, crate::nbt::Tag::Int(v));
    }
    if id == 25 {
        n.put("note", crate::nbt::Tag::Byte(0));
    } else if id == 84 {
        n.put("Record", crate::nbt::Tag::Int(0));
    } else {
        n.put("Items", crate::nbt::Tag::List(1, Vec::new()));
    }
    if matches!(id, 61 | 62) {
        n.put("BurnTime", crate::nbt::Tag::Short(0));
        n.put("CookTime", crate::nbt::Tag::Short(0));
    }
    w.chunk_mut(x >> 4, z >> 4).raw_tile_entities.push(n);
}
