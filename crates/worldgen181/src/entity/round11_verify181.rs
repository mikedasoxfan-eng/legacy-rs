//! Round 11 independent original-bytecode checks.
use super::{player::ItemStack, Entity};
fn i(o: &mut Vec<u8>, v: i32) {
    o.extend(v.to_le_bytes());
}
fn text(o: &mut Vec<u8>, v: &str) {
    i(o, v.len() as i32);
    o.extend(v.as_bytes());
}
fn stack_state(o: &mut Vec<u8>, s: &ItemStack) {
    for v in [s.id, s.count, s.damage, s.animations] {
        i(o, v);
    }
}
pub fn blockfull() -> Vec<u8> {
    // Derive the registered input set independently, rather than reading any
    // expected record. Every registered sub-256 item extends ItemBlock.
    let ids: Vec<_> = (1..256)
        .filter(|&id| id != 36 && super::player::registry::item(id).is_some())
        .collect();
    let mut inputs = Vec::new();
    i(&mut inputs, ids.len() as i32);
    for id in ids {
        i(&mut inputs, id);
    }
    super::round6_verify181::block_items_replay(&inputs)
}
pub fn stackdata() -> Vec<u8> {
    use crate::nbt::{Compound, Tag};
    fn maybe(o: &mut Vec<u8>, s: Option<ItemStack>) {
        o.push(s.is_some() as u8);
        if let Some(s) = s {
            stack_state(o, &s);
        }
    }
    fn error(o: &mut Vec<u8>, r: Result<(), StackError>) {
        text(o, r.err().map_or("", |e| e.class()));
    }
    let mut o = Vec::new();
    let values = [i32::MIN, -1, 0, 1, i32::MAX];
    for kind in 0..8 {
        for count in values {
            for damage in values {
                let item = super::item::Item::defaults(267);
                let result = match kind {
                    0 => Ok(ItemStack::new(32000, count, damage)),
                    1 => ItemStack::from_block181(Some(1), 1, 0),
                    2 => ItemStack::from_block181(Some(1), count, 0),
                    3 => ItemStack::from_block181(Some(1), count, damage),
                    4 => ItemStack::from_item181(Some(&item), 1, 0),
                    5 => ItemStack::from_item181(Some(&item), count, 0),
                    6 => ItemStack::from_item181(Some(&item), count, damage),
                    _ => ItemStack::from_item181(None, count, damage),
                };
                text(&mut o, result.as_ref().err().map_or("", |e| e.class()));
                maybe(&mut o, result.ok());
            }
        }
    }
    for id in [-1, 0, 1, 267, 32000] {
        for count in values {
            for damage in values {
                for amount in values {
                    let mut s = ItemStack::new(id, count, damage);
                    s.animations = 17;
                    let alias = s;
                    let split = s.split(amount);
                    stack_state(&mut o, &s);
                    stack_state(&mut o, &split);
                    o.push(s.same_object(alias) as u8);
                    o.push(split.same_object(s) as u8);
                    let copy = s.copy_stack181();
                    stack_state(&mut o, &copy);
                    o.push(copy.same_object(s) as u8);
                    o.push(ItemStack::stacks_equal181(Some(&s), Some(&copy)) as u8);
                    o.push(s.item_equal181(Some(&copy)).unwrap() as u8);
                    o.push(s.stack_equal181(&copy) as u8);
                    s.damage = amount;
                    i(&mut o, s.damage);
                    i(&mut o, s.damage);
                    o.push(ItemStack::stacks_equal181(Some(&s), Some(&copy)) as u8);
                    maybe(&mut o, ItemStack::copy_nullable181(None));
                    maybe(&mut o, ItemStack::copy_nullable181(Some(&s)));
                    for (a, b) in [(None, None), (Some(&s), None), (None, Some(&s))] {
                        o.push(ItemStack::stacks_equal181(a, b) as u8);
                    }
                    error(&mut o, s.item_equal181(None).map(|_| ()));
                }
            }
        }
    }
    let ids = [
        i16::MIN as i32,
        -1,
        0,
        1,
        119,
        255,
        256,
        267,
        2257,
        30700,
        31999,
        32000,
        i16::MAX as i32,
    ];
    let counts = [i32::MIN, -129, -128, -1, 0, 1, 127, 128, i32::MAX];
    let damages = [i32::MIN, -32769, -32768, -1, 0, 1, 32767, 32768, i32::MAX];
    for id in ids {
        for count in counts {
            for damage in damages {
                for mode in 0..5 {
                    let mut n = ItemStack::new(id, count, damage).to_nbt();
                    match mode {
                        1 => n = Compound::new(),
                        2 => n.put("id", Tag::Int(1)),
                        3 => n.put("Count", Tag::Int(1)),
                        4 => n.put("Damage", Tag::Int(1)),
                        _ => {}
                    }
                    let mut s = ItemStack::new(4, 29, 33);
                    s.animations = 17;
                    error(&mut o, s.read_nbt_checked181(Some(&n)));
                    stack_state(&mut o, &s);
                    let loaded = ItemStack::load_checked181(Some(&n), |id| {
                        if !(0..32000).contains(&id) {
                            Err(StackError::ArrayIndexOutOfBounds)
                        } else {
                            Ok(super::player::registry::item(id).is_some())
                        }
                    });
                    text(&mut o, loaded.as_ref().err().map_or("", |e| e.class()));
                    maybe(&mut o, loaded.ok().flatten());
                    let mut dest = Compound::new();
                    dest.put("keep", Tag::String("value".into()));
                    dest.put("Count", Tag::Int(72));
                    s = ItemStack::new(id, count, damage);
                    s.write_nbt181(Some(&mut dest)).unwrap();
                    o.push(1);
                    for v in [
                        dest.short("id") as i32,
                        dest.byte("Count") as i32,
                        dest.short("Damage") as i32,
                    ] {
                        i(&mut o, v);
                    }
                    text(&mut o, &dest.string("keep"));
                }
            }
        }
    }
    let mut s = ItemStack::new(1, 2, 3);
    error(&mut o, s.read_nbt_checked181(None));
    error(&mut o, s.write_nbt181(None).map(|_| ()));
    error(
        &mut o,
        ItemStack::load_checked181(None, |_| Ok(true)).map(|_| ()),
    );
    for key in ["id", "Count", "Damage"] {
        for ty in 1..=10 {
            let mut n = ItemStack::new(267, -17, 99).to_nbt();
            n.put(
                key,
                match ty {
                    1 => Tag::Byte(0),
                    2 => Tag::Short(0),
                    3 => Tag::Int(0),
                    4 => Tag::Long(0),
                    5 => Tag::Float(0.0),
                    6 => Tag::Double(0.0),
                    7 => Tag::ByteArray(vec![]),
                    8 => Tag::String(String::new()),
                    9 => Tag::List(1, vec![]),
                    _ => Tag::Compound(Compound::new()),
                },
            );
            let mut s = ItemStack::new(4, 29, 33);
            s.animations = 17;
            error(&mut o, s.read_nbt_checked181(Some(&n)));
            stack_state(&mut o, &s);
        }
    }
    for id in [i32::MIN, -1, 0, 1, 267, 31999, 32000, i32::MAX] {
        for damage in [-1, 0, 1, 6] {
            for op in 0..6 {
                let s = ItemStack::new(id, 2, damage);
                let result = match op {
                    0 => s.max_stack_checked181(),
                    1 => s.max_damage_checked181(),
                    2 => s.max_damage_checked181().map(|v| (v > 0) as i32),
                    3 => s
                        .max_damage_checked181()
                        .map(|v| (v > 0 && s.damage > 0) as i32),
                    4 => s.max_stack_checked181().and_then(|limit| {
                        if limit <= 1 {
                            Ok(0)
                        } else {
                            s.max_damage_checked181()
                                .map(|v| (!(v > 0 && s.damage > 0)) as i32)
                        }
                    }),
                    _ => {
                        if !(0..32000).contains(&id) {
                            Err(StackError::ArrayIndexOutOfBounds)
                        } else {
                            super::player::registry::item(id)
                                .map(|v| v.subtypes as i32)
                                .ok_or(StackError::NullPointer)
                        }
                    }
                };
                text(&mut o, result.as_ref().err().map_or("", |e| e.class()));
                i(&mut o, result.unwrap_or(0));
            }
        }
    }
    o
}
pub fn stacknbt() -> Vec<u8> {
    use crate::nbt::Tag;
    let mut o = Vec::new();
    for op in 0..2 {
        for scene in 0..4 {
            for fail in -1..6 {
                for value in [i32::MIN, -1, 0, 1, i32::MAX] {
                    let mut s = ItemStack::new(value, value, value);
                    s.animations = 17;
                    let mut events: Vec<[i64; 6]> = Vec::new();
                    let mut written = [0; 3];
                    let mut present = [false; 3];
                    let mut call =
                        |s: &mut ItemStack, code: i32, value: i32| -> Result<(), StackError> {
                            events.push([
                                code as i64,
                                value as i64,
                                s.id as i64,
                                s.count as i64,
                                s.damage as i64,
                                s.animations as i64,
                            ]);
                            if code == fail {
                                return Err(StackError::Callback);
                            }
                            match scene {
                                1 => {
                                    s.id = -17;
                                    s.count = i32::MIN;
                                    s.damage = i32::MAX;
                                    s.animations = -33;
                                }
                                2 => {
                                    s.id = s.id.wrapping_add(1);
                                    s.count = s.count.wrapping_add(7);
                                    s.damage = s.damage.wrapping_add(3);
                                    s.animations = s.animations.wrapping_add(1);
                                }
                                3 => {
                                    s.count = -91;
                                    return Err(StackError::Callback);
                                }
                                _ => {}
                            }
                            Ok(())
                        };
                    let result = if op == 0 {
                        s.read_nbt_with181(|s, key, _| {
                            let (code, value) = match key {
                                "id" => (0, 267),
                                "Count" => (1, -17),
                                _ => (2, 99),
                            };
                            call(s, code, value)?;
                            Ok(value)
                        })
                    } else {
                        s.write_nbt_with181(|s, key, tag| {
                            let index = match key {
                                "id" => 0,
                                "Count" => 1,
                                _ => 2,
                            };
                            let value = match tag {
                                Tag::Byte(v) => v as i32,
                                Tag::Short(v) => v as i32,
                                _ => unreachable!(),
                            };
                            call(s, index as i32 + 3, value)?;
                            present[index] = true;
                            written[index] = value;
                            Ok(())
                        })
                    };
                    text(&mut o, result.as_ref().err().map_or("", |e| e.class()));
                    o.push(result.is_ok() as u8);
                    stack_state(&mut o, &s);
                    for k in 0..3 {
                        o.push(present[k] as u8);
                        i(&mut o, written[k]);
                    }
                    i(&mut o, events.len() as i32);
                    for row in events {
                        for v in row {
                            o.extend(v.to_le_bytes());
                        }
                    }
                }
            }
        }
    }
    o
}
use super::item_stack::{StackContext, StackError, StackOp, StackStat, StackValue};
struct StackProbe {
    scene: i32,
    fail: i32,
    absent: i32,
    events: Vec<Vec<i64>>,
    other: ItemStack,
}
impl StackProbe {
    fn event(
        &mut self,
        op: i32,
        serial: i32,
        stack: &ItemStack,
        args: &[i64],
    ) -> Result<(), StackError> {
        let mut row = vec![
            op as i64,
            serial as i64,
            stack.id as i64,
            stack.count as i64,
            stack.damage as i64,
            stack.animations as i64,
            self.absent as i64,
        ];
        row.extend(args);
        self.events.push(row);
        if op == self.fail {
            Err(StackError::Callback)
        } else {
            Ok(())
        }
    }
    fn mutate(&self, s: &mut ItemStack) {
        match self.scene {
            1 => s.id = 30701,
            2 => s.id = 30702,
            3 => s.id = -1,
            4 => s.id = 32000,
            5 => {
                s.count = s.count.wrapping_add(7);
                s.damage = s.damage.wrapping_add(3);
                s.animations = s.animations.wrapping_add(11);
            }
            _ => {}
        }
    }
}
impl StackContext for StackProbe {
    type Item = i32;
    fn lookup(&mut self, id: i32) -> Result<Option<i32>, StackError> {
        if !(0..32000).contains(&id) {
            return Err(StackError::ArrayIndexOutOfBounds);
        }
        Ok(match id {
            30700 => Some(1),
            30701 => Some(2),
            _ => None,
        })
    }
    fn entity_is_player(&self) -> bool {
        self.absent != 1 && self.absent != 3
    }
    fn stat(
        &mut self,
        kind: StackStat,
        id: i32,
        amount: i32,
        s: &mut ItemStack,
    ) -> Result<(), StackError> {
        if !(0..32000).contains(&id) {
            return Err(StackError::ArrayIndexOutOfBounds);
        }
        if self.absent == 1 {
            return Err(StackError::NullPointer);
        }
        let stat = if matches!(id, 30700 | 30701) {
            id + match kind {
                StackStat::Use => 700000,
                StackStat::Break => 800000,
                StackStat::Craft => 900000,
            }
        } else {
            -1
        };
        self.event(20, stat, s, &[amount as i64])?;
        if self.scene == 10 {
            s.id = 30701;
        }
        if self.scene == 11 {
            return Err(StackError::Callback);
        }
        Ok(())
    }
    fn invoke(
        &mut self,
        serial: i32,
        op: StackOp,
        s: &mut ItemStack,
    ) -> Result<StackValue, StackError> {
        use StackOp::*;
        let p = (self.absent != 1) as i64;
        let w = (self.absent != 2) as i64;
        let t = (self.absent != 3) as i64;
        let (code, args) = match op {
            Icon => (0, vec![s.damage as i64]),
            Use { position, side } => (
                1,
                vec![
                    position[0] as i64,
                    position[1] as i64,
                    position[2] as i64,
                    side as i64,
                    1,
                    p,
                    w,
                ],
            ),
            Strength => (2, vec![1, t]),
            RightClick => (3, vec![1, w, p]),
            Eaten => (4, vec![1, w, p]),
            MaxStack => (5, vec![]),
            MaxDamage => (6, vec![]),
            HasSubtypes => (7, vec![]),
            Hit => (8, vec![1, t, p]),
            Destroy { block, position } => (
                9,
                vec![
                    block as i64,
                    position[0] as i64,
                    position[1] as i64,
                    position[2] as i64,
                    1,
                    p,
                ],
            ),
            DamageVsEntity => (10, vec![t]),
            Harvest => (11, vec![t]),
            Saddle => (12, vec![1, t]),
            Name => (13, vec![1]),
            DisplayString => (14, vec![]),
            Update { slot, selected } => (15, vec![slot as i64, selected as i64, 1, w, t]),
            Craft => (16, vec![1, w, p]),
            Duration => (17, vec![1]),
            Action => (18, vec![1]),
            Stop { remaining } => (19, vec![remaining as i64, 1, w, p]),
            _ => unreachable!(),
        };
        self.event(code, serial, s, &args)?;
        self.mutate(s);
        Ok(match op {
            Icon => StackValue::Int((args[0] as i32).wrapping_mul(17).wrapping_add(serial)),
            Use { .. } | Hit | Destroy { .. } => StackValue::Bool(self.scene != 6),
            Strength => StackValue::Float(if serial == 1 { -1.25 } else { 3.5 }),
            RightClick | Eaten => StackValue::Stack(match self.scene {
                7 => None,
                8 => Some(ItemStack::new(1, -17, 33)),
                9 => Some(self.other),
                _ => Some(*s),
            }),
            MaxStack => StackValue::Int(if serial == 1 { 64 } else { 1 }),
            MaxDamage => StackValue::Int(if serial == 1 { 5 } else { 0 }),
            HasSubtypes | Harvest => StackValue::Bool(serial == 1),
            DamageVsEntity => StackValue::Int(serial * 19),
            Name | DisplayString => StackValue::Name((self.scene != 7).then(|| {
                format!(
                    "{}.{}",
                    if matches!(op, Name) { "stack" } else { "plain" },
                    serial
                )
            })),
            Duration => StackValue::Int(serial * 72000),
            Action => StackValue::Action(if self.scene == 7 {
                None
            } else {
                Some(if serial == 1 {
                    super::player::registry::ItemUseAction::Bow
                } else {
                    super::player::registry::ItemUseAction::Eat
                })
            }),
            _ => StackValue::Unit,
        })
    }
}
pub fn itemstack() -> Vec<u8> {
    use StackOp::*;
    let mut o = Vec::new();
    for op in 0..26 {
        for scene in 0..12 {
            for absent in 0..4 {
                for fail in [-1, 0, 1, 3, 5, 6, 8, 9, 15, 16, 20] {
                    for config in 0..5 {
                        let mut s = ItemStack::new(
                            if config == 3 { 30702 } else { 30700 },
                            if config == 1 || config == 4 {
                                i32::MIN
                            } else if config == 2 {
                                0
                            } else {
                                2
                            },
                            if config == 1 {
                                i32::MAX
                            } else if config == 2 {
                                -1
                            } else {
                                6
                            },
                        );
                        s.animations = if config == 2 { -1 } else { 3 };
                        let mut c = StackProbe {
                            scene,
                            fail,
                            absent,
                            events: Vec::new(),
                            other: ItemStack::new(4, 9, 2),
                        };
                        let operation = match op {
                            0 => Icon,
                            1 => Use {
                                position: [i32::MAX, -17, i32::MIN],
                                side: 6,
                            },
                            2 => Strength,
                            3 => RightClick,
                            4 => Eaten,
                            5 => MaxStack,
                            6 => Stackable,
                            7 => Damageable,
                            8 => HasSubtypes,
                            9 => Damaged,
                            10 => MaxDamage,
                            11 => Damage {
                                amount: if config == 1 { i32::MAX } else { 1 },
                            },
                            12 => Hit,
                            13 => Destroy {
                                block: -17,
                                position: [i32::MAX, -17, i32::MIN],
                            },
                            14 => DamageVsEntity,
                            15 => Harvest,
                            16 => Saddle,
                            17 => Name,
                            18 => DisplayString,
                            19 => Update {
                                slot: -17,
                                selected: true,
                            },
                            20 => Craft,
                            21 => Duration,
                            22 => Action,
                            23 => Stop {
                                remaining: i32::MIN,
                            },
                            _ => Destroyed,
                        };
                        let result = if op == 25 {
                            s.get_item181(&mut c)
                                .map(|v| StackValue::Int(v.unwrap_or(-1)))
                        } else {
                            s.dispatch181(&mut c, operation)
                        };
                        text(&mut o, result.as_ref().err().map_or("", |e| e.class()));
                        match result.unwrap_or(StackValue::Unit) {
                            StackValue::Unit
                            | StackValue::Name(None)
                            | StackValue::Stack(None)
                            | StackValue::Action(None) => o.push(0),
                            StackValue::Bool(v) => {
                                o.push(1);
                                o.push(v as u8);
                            }
                            StackValue::Int(v) => {
                                o.push(2);
                                i(&mut o, v);
                            }
                            StackValue::Float(v) => {
                                o.push(3);
                                i(&mut o, v.to_bits() as i32);
                            }
                            StackValue::Name(Some(v)) => {
                                o.push(4);
                                o.push(1);
                                text(&mut o, &v);
                            }
                            StackValue::Stack(Some(v)) => {
                                o.push(5);
                                o.push(1);
                                i(
                                    &mut o,
                                    if v.same_object(s) {
                                        1
                                    } else if v.same_object(c.other) {
                                        2
                                    } else {
                                        3
                                    },
                                );
                                stack_state(&mut o, &v);
                            }
                            StackValue::Action(Some(v)) => {
                                o.push(6);
                                i(&mut o, v as i32);
                            }
                        }
                        stack_state(&mut o, &s);
                        i(&mut o, c.events.len() as i32);
                        for row in c.events {
                            i(&mut o, row.len() as i32);
                            for v in row {
                                o.extend(v.to_le_bytes());
                            }
                        }
                    }
                }
            }
        }
    }
    o
}
pub fn itempiston() -> Vec<u8> {
    use crate::{
        nbt::Tag,
        world::{chunk::idx, pistons::Piston, tile::TileEntity},
    };
    let mut o = Vec::new();
    for x in [0, 15] {
        for y in [-1, 0, 64, 127, 128, 256, 32768] {
            for side in [-1, 0, 1, 5, 6] {
                for tile in 0..4 {
                    for stored in [0, 1, 6, 36, 51, 85, 255] {
                        for facing in [-1, 0, 1, 2, 3, 4, 5, 6, 7, i32::MAX] {
                            let mut w = super::verify181::world();
                            let mut player = Entity::player(&mut w, "Player");
                            player.set_position(8.5, 64.0, 8.5);
                            if (0..128).contains(&y) {
                                w.chunk_mut(x >> 4, 0).blocks[idx(x & 15, y, 0)] = 36;
                            }
                            let (tx, ty, tz) = (
                                x + if side == 5 { 1 } else { 0 },
                                y + if side == 0 {
                                    -1
                                } else if side == 1 {
                                    1
                                } else {
                                    0
                                },
                                0,
                            );
                            let mut te_id = None;
                            if tile > 0 && (0..128).contains(&ty) {
                                w.chunk_mut(tx >> 4, 0).blocks[idx(tx & 15, ty, 0)] = 36;
                                let te = if tile == 1 {
                                    TileEntity::Sign(Default::default())
                                } else {
                                    let mut n =
                                        Piston::new(stored, 0, facing, stored & 1 != 0, false)
                                            .to_nbt(tx, ty, tz);
                                    n.put("progress", Tag::Float(0.5));
                                    TileEntity::Piston(Piston::from_nbt(&n))
                                };
                                w.set_block_tile_entity(tx, ty, tz, te);
                                let id = w.chunk(tx >> 4, 0).tile_entity_ids[&(tx & 15, ty, 0)];
                                te_id = Some(id);
                                if tile == 3 {
                                    w.invalidate_tile181(id);
                                }
                            }
                            let mut stack = ItemStack::new(36, 1, 0);
                            let result = player
                                .place_block_item_checked181(&mut w, &mut stack, x, y, 0, side);
                            o.push(result.as_ref().is_ok_and(|v| *v) as u8);
                            text(&mut o, result.err().map_or("", |e| e.class()));
                            i(&mut o, stack.count);
                            o.push(te_id.is_some_and(|id| w.tile_objects[&id].invalid) as u8);
                            o.push(
                                w.chunk(tx >> 4, 0)
                                    .tile_entities
                                    .contains_key(&(tx & 15, ty, 0))
                                    as u8,
                            );
                            o.extend(w.rand.state().to_le_bytes());
                            crate::world::events::write_events(&w.drain_events(), &mut o);
                        }
                    }
                }
            }
        }
    }
    o
}
