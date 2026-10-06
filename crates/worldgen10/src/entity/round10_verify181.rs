//! Independent round 10 original-bytecode checks.
use super::item::{Item, ItemError, ItemRegistry};
use super::player::ItemStack;
fn i(o: &mut Vec<u8>, v: i32) {
    o.extend(v.to_le_bytes());
}
fn text(o: &mut Vec<u8>, s: &str) {
    i(o, s.len() as i32);
    o.extend(s.as_bytes());
}
fn nullable(o: &mut Vec<u8>, s: Option<&str>) {
    o.push(s.is_some() as u8);
    if let Some(s) = s {
        text(o, s);
    }
}
fn state(o: &mut Vec<u8>, p: &Item) {
    for v in [p.shifted_index, p.max_stack_size(), p.max_damage()] {
        i(o, v);
    }
    for v in [
        p.has_subtypes(),
        p.is_damageable(),
        p.is_full_3d(),
        p.should_rotate(),
    ] {
        o.push(v as u8);
    }
    for v in [
        p.icon_from_damage(i32::MIN),
        p.placed_metadata(i32::MAX),
        p.color_from_damage(-1),
    ] {
        i(o, v);
    }
    nullable(o, p.item_name());
    nullable(o, p.item_name_is(None));
    o.push(p.has_container_item() as u8);
    i(o, p.container_item().map_or(-1, |h| h as i32));
    text(o, &p.stat_name(str::to_owned));
}
pub fn itembase() -> Vec<u8> {
    let mut o = Vec::new();
    let mut registry = ItemRegistry::default();
    for index in [
        i32::MIN,
        -257,
        -256,
        -255,
        30744,
        30744,
        31743,
        31744,
        i32::MAX,
        i32::MAX - 255,
    ] {
        let mut diagnostic = String::new();
        let result = registry.construct(index, |s| {
            diagnostic.push_str(s);
            diagnostic.push('\n');
        });
        text(&mut o, result.as_ref().err().map_or("", |e| e.class()));
        text(&mut o, &diagnostic);
        o.push(result.is_ok() as u8);
        if let Ok(h) = result {
            o.push((registry.lookup(index.wrapping_add(256)) == Ok(Some(h))) as u8);
            state(&mut o, registry.get(h));
        }
    }
    let ph = registry.construct(30745, |_| {}).unwrap();
    let ch = registry.construct(30746, |_| {}).unwrap();
    let p = registry.get_mut(ph);
    let ints = [i32::MIN, -17, -1, 0, 1, 2, 64, 32767, i32::MAX];
    for limit in ints {
        for damage in ints {
            for sub in [false, true] {
                p.set_max_stack_size(limit);
                o.push(1);
                p.set_max_damage(damage);
                o.push(1);
                p.set_has_subtypes(sub);
                o.push(1);
                for c in [Some(ch), None, Some(ph)] {
                    let result = p.set_container_item(c);
                    o.push(result.is_ok() as u8);
                    text(&mut o, result.err().map_or("", ItemError::class));
                    state(&mut o, p);
                }
            }
        }
    }
    for x in ints {
        for y in ints {
            p.set_icon_coord(x, y);
            o.push(1);
            state(&mut o, p);
            p.set_icon_index(y);
            o.push(1);
            state(&mut o, p);
        }
    }
    for n in [
        None,
        Some(""),
        Some("stone"),
        Some("a.b"),
        Some("A\0B"),
        Some("λ"),
    ] {
        p.set_item_name(n);
        o.push(1);
        state(&mut o, p);
    }
    for _ in 0..2 {
        p.set_full_3d();
        o.push(1);
        state(&mut o, p);
    }
    for damage in ints {
        for absent in [false, true] {
            let mut s = (!absent).then(|| {
                let mut s = ItemStack::new(31001, -7, damage);
                s.animations = 91;
                s
            });
            let icon = p.icon_index(s.as_ref());
            i(&mut o, icon.as_ref().copied().unwrap_or(0));
            text(&mut o, icon.err().map_or("", ItemError::class));
            o.push(p.on_item_use(s.as_mut(), None, None, [-1, i32::MAX, 2], -17) as u8);
            o.extend(p.strength(s.as_ref(), None).to_bits().to_le_bytes());
            let identity = s.as_ref().map(|s| s.identity);
            let right = p
                .on_item_right_click(s.as_mut(), None, None)
                .map(|s| s.identity);
            o.push((right == identity) as u8);
            let eaten = p.on_eaten(s.as_mut(), None, None).map(|s| s.identity);
            o.push((eaten == identity) as u8);
            o.push(p.hit_entity(s.as_mut(), None, None) as u8);
            o.push(p.on_block_destroyed(s.as_mut(), -1, [2, 3, 4], None) as u8);
            i(&mut o, p.damage_vs_entity(None));
            o.push(p.can_harvest(None) as u8);
            p.saddle_entity(s.as_mut(), None);
            p.on_update(s.as_mut(), None, None, i32::MIN, true);
            p.on_created(s.as_mut(), None, None);
            p.on_stopped_using(s.as_mut(), None, None, -1);
            i(&mut o, p.use_action(s.as_ref()) as i32);
            i(&mut o, p.use_duration(s.as_ref()));
            if let Some(s) = s {
                for v in [s.id, s.count, s.damage, s.animations] {
                    i(&mut o, v);
                }
            }
        }
    }
    for d in ints {
        for absent in [false, true] {
            for fail in [false, true] {
                let s = (!absent).then(|| ItemStack::new(1, 1, d));
                let (mut calls, mut damage) = (0, 0);
                let result = Item::icon_index_with(s.as_ref(), |d| {
                    calls += 1;
                    damage = d;
                    if fail {
                        Err(ItemError::ContainerStackSize)
                    } else {
                        Ok(d.wrapping_mul(17).wrapping_add(31))
                    }
                });
                i(&mut o, result.as_ref().copied().unwrap_or(0));
                text(&mut o, result.err().map_or("", ItemError::class));
                i(&mut o, calls);
                i(&mut o, damage);
            }
        }
    }
    for n in [
        None,
        Some(""),
        Some("unregistered"),
        Some("item.stone"),
        Some("A\0B"),
    ] {
        let mut calls = 0;
        let result = Item::stat_name_with(
            || {
                calls += 1;
                n.map(str::to_owned)
            },
            str::to_owned,
        );
        text(&mut o, &result);
        i(&mut o, calls);
    }
    let alias = registry.construct(30747, |_| {}).unwrap();
    registry
        .get_mut(ph)
        .set_max_stack_size(1)
        .set_container_item(Some(alias))
        .unwrap();
    let replacement = registry.construct(30747, |_| {}).unwrap();
    o.push((registry.lookup(31003) == Ok(Some(replacement))) as u8);
    o.push((registry.get(ph).container_item() == Some(alias)) as u8);
    registry.get_mut(alias).set_item_name(Some("old"));
    registry.get_mut(replacement).set_item_name(Some("new"));
    state(&mut o, registry.get(alias));
    state(&mut o, registry.get(replacement));
    o
}

use super::item_block::{BlockItem, BlockItemContext, BlockItemProperties};
struct BlockProbe {
    events: Vec<Vec<i64>>,
    fail: i32,
    scene: i32,
    slot: i32,
    existing: i32,
    calls: i32,
    absent: i32,
    virtual_metadata: bool,
}
impl BlockProbe {
    fn new(fail: i32, scene: i32) -> Self {
        Self {
            events: Vec::new(),
            fail,
            scene,
            slot: 1,
            existing: 0,
            calls: 0,
            absent: 0,
            virtual_metadata: false,
        }
    }
    fn event(&mut self, values: &[i64]) -> Result<(), ItemError> {
        self.events.push(values.to_vec());
        if values[0] == self.fail as i64 {
            Err(ItemError::Callback)
        } else {
            Ok(())
        }
    }
    fn events(&self, o: &mut Vec<u8>) {
        i(o, self.events.len() as i32);
        for call in &self.events {
            i(o, call.len() as i32);
            for v in call {
                o.extend(v.to_le_bytes());
            }
        }
    }
    fn serial(&self, id: i32) -> Result<Option<i32>, ItemError> {
        if !(0..256).contains(&id) {
            return Err(ItemError::ArrayIndexOutOfBounds);
        }
        Ok(match id {
            253 => {
                if self.slot == 0 {
                    None
                } else {
                    Some(self.slot)
                }
            }
            254 => Some(2),
            _ => None,
        })
    }
}
impl BlockItemProperties for BlockProbe {
    fn texture(&mut self, id: i32, side: i32) -> Result<i32, ItemError> {
        let serial = self.serial(id)?.ok_or(ItemError::NullPointer)?;
        self.event(&[10, serial as i64, side as i64])?;
        Ok(serial * 1009 + side * 31)
    }
    fn name(&mut self, id: i32) -> Result<Option<String>, ItemError> {
        let serial = self.serial(id)?.ok_or(ItemError::NullPointer)?;
        self.event(&[11, serial as i64])?;
        Ok((serial == 1).then(|| "tile.first".into()))
    }
}
impl BlockItemContext for BlockProbe {
    type Block = Option<i32>;
    fn get_block_id(&mut self, p: [i32; 3]) -> Result<i32, ItemError> {
        if self.absent == 3 {
            return Err(ItemError::NullPointer);
        }
        self.event(&[1, p[0] as i64, p[1] as i64, p[2] as i64])?;
        let value = if self.calls == 0 {
            self.existing
        } else if self.scene == 3 {
            1
        } else {
            253
        };
        self.calls += 1;
        Ok(value)
    }
    fn can_edit(&mut self, p: [i32; 3], s: &mut ItemStack) -> Result<bool, ItemError> {
        if self.absent == 2 {
            return Err(ItemError::NullPointer);
        }
        self.event(&[13, p[0] as i64, p[1] as i64, p[2] as i64])?;
        if self.scene == 9 {
            s.count = i32::MIN;
            s.damage = -17;
        }
        Ok(self.scene != 8)
    }
    fn solid(&mut self, id: i32) -> Result<bool, ItemError> {
        self.serial(id)?.ok_or(ItemError::NullPointer)?;
        Ok(true)
    }
    fn can_place(&mut self, id: i32, p: [i32; 3], side: i32) -> Result<bool, ItemError> {
        self.event(&[
            2,
            id as i64,
            p[0] as i64,
            p[1] as i64,
            p[2] as i64,
            0,
            side as i64,
        ])?;
        Ok(self.scene != 1)
    }
    fn block(&mut self, id: i32) -> Result<Self::Block, ItemError> {
        self.serial(id)
    }
    fn placed_metadata(&mut self, _id: i32, damage: i32) -> Result<i32, ItemError> {
        if !self.virtual_metadata {
            return Ok(0);
        }
        self.event(&[12, damage as i64])?;
        if self.scene == 6 {
            self.slot = 2;
        }
        Ok(damage.wrapping_mul(17).wrapping_add(3))
    }
    fn set_block(&mut self, id: i32, p: [i32; 3], meta: i32) -> Result<bool, ItemError> {
        self.event(&[
            3,
            id as i64,
            p[0] as i64,
            p[1] as i64,
            p[2] as i64,
            meta as i64,
        ])?;
        if self.scene == 7 {
            self.slot = 2;
        }
        Ok(self.scene != 2)
    }
    fn on_placed(&mut self, b: Self::Block, p: [i32; 3], side: i32) -> Result<(), ItemError> {
        let serial = b.ok_or(ItemError::NullPointer)?;
        self.event(&[
            4,
            serial as i64,
            p[0] as i64,
            p[1] as i64,
            p[2] as i64,
            side as i64,
        ])?;
        if self.scene == 4 {
            self.slot = 2;
        }
        if self.scene == 5 {
            self.slot = 0;
        }
        Ok(())
    }
    fn on_placed_by(&mut self, b: Self::Block, p: [i32; 3]) -> Result<(), ItemError> {
        let serial = b.ok_or(ItemError::NullPointer)?;
        self.event(&[5, serial as i64, p[0] as i64, p[1] as i64, p[2] as i64])
    }
    fn sound(&mut self, b: Self::Block, p: [f64; 3]) -> Result<(), ItemError> {
        let serial = b.ok_or(ItemError::NullPointer)?;
        self.event(&[6, serial as i64])?;
        self.event(&[7, serial as i64])?;
        let volume = ((if serial == 1 { 0.25f32 } else { 1.5 }) + 1.0) / 2.0;
        self.event(&[8, serial as i64])?;
        let pitch = (if serial == 1 { 1.25f32 } else { -0.75 }) * 0.8;
        self.event(&[
            9,
            p[0].to_bits() as i64,
            p[1].to_bits() as i64,
            p[2].to_bits() as i64,
            volume.to_bits() as i32 as i64,
            pitch.to_bits() as i32 as i64,
        ])
    }
}
pub fn itemblock() -> Vec<u8> {
    let mut o = Vec::new();
    let mut registry = ItemRegistry::default();
    for index in [i32::MIN, -257, -256, -3, -2, 0, 31743, 31744, i32::MAX] {
        for fail in [0, 10] {
            let mut context = BlockProbe::new(fail, 0);
            let result = BlockItem::construct(&mut registry, index, &mut context, |_| {});
            text(&mut o, result.as_ref().err().map_or("", |e| e.class()));
            o.push(result.is_ok() as u8);
            let registered = registry.lookup(index.wrapping_add(256)).ok().flatten();
            o.push(registered.is_some() as u8);
            if let Some(h) = registered {
                let p = registry.get(h);
                i(&mut o, p.shifted_index);
                i(&mut o, p.icon_from_damage(0));
            }
            if let Ok((item, _)) = result {
                i(&mut o, item.block_id());
                for op in 0..2 {
                    let name = if op == 0 {
                        item.name(&mut context)
                    } else {
                        item.name_is(&mut context, None)
                    };
                    text(&mut o, name.as_ref().err().map_or("", |e| e.class()));
                    nullable(&mut o, name.ok().flatten().as_deref());
                }
            }
            context.events(&mut o);
        }
    }
    let item = BlockItem::registered(253);
    for existing in [0, 78, 106] {
        for y in [64, 127, i32::MIN, i32::MAX] {
            for side in [-1, 0, 1, 2, 3, 4, 5, 6] {
                for scene in 0..10 {
                    for absent in 0..4 {
                        for fail in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 12, 13] {
                            for virtual_metadata in [false, true] {
                                let mut c = BlockProbe::new(fail, scene);
                                c.existing = existing;
                                c.absent = absent;
                                c.virtual_metadata = virtual_metadata;
                                let mut s = ItemStack::new(
                                    253,
                                    if scene == 0 {
                                        0
                                    } else if scene == 1 {
                                        -1
                                    } else {
                                        1
                                    },
                                    i32::MAX,
                                );
                                let result = item.on_item_use(
                                    &mut c,
                                    if absent == 1 { None } else { Some(&mut s) },
                                    [i32::MAX, y, i32::MIN],
                                    side,
                                );
                                o.push(result.as_ref().copied().unwrap_or(false) as u8);
                                text(&mut o, result.err().map_or("", ItemError::class));
                                i(&mut o, s.count);
                                i(&mut o, s.damage);
                                i(&mut o, c.slot);
                                c.events(&mut o);
                            }
                        }
                    }
                }
            }
        }
    }
    o
}

use super::{Entity, EntityKind};
use crate::nbt::{Compound, Tag};
fn listd(values: &[f64]) -> Tag {
    Tag::List(6, values.iter().copied().map(Tag::Double).collect())
}
fn listf(values: &[f32]) -> Tag {
    Tag::List(5, values.iter().copied().map(Tag::Float).collect())
}
const FACTORY_FIELDS: &[&str] = &[
    "FallDistance",
    "Fire",
    "Air",
    "OnGround",
    "Health",
    "HurtTime",
    "DeathTime",
    "AttackTime",
    "ActiveEffects",
    "Age",
    "Value",
    "carried",
    "carryingData",
    "Anger",
    "Angry",
    "Sitting",
    "Owner",
    "Saddle",
    "Sheared",
    "Color",
    "powered",
    "Size",
    "Fuse",
    "Tile",
    "Dir",
    "TileX",
    "TileY",
    "TileZ",
    "Motive",
    "xTile",
    "yTile",
    "zTile",
    "inTile",
    "inData",
    "shake",
    "inGround",
    "player",
    "Item",
    "Type",
    "Items",
    "PushX",
    "PushZ",
    "Fuel",
];
fn factory_nbt(name: Option<&str>, scenario: i32) -> Option<Compound> {
    if scenario == 13 {
        return None;
    }
    let mut n = Compound::new();
    if let Some(name) = name {
        n.put("id", Tag::String(name.into()));
    }
    if scenario != 1 {
        n.put("Pos", listd(&[-3.5, 70.25, 4.5]));
    }
    if scenario != 2 {
        n.put("Motion", listd(&[11.0, -0.25, -10.0]));
    }
    if scenario != 3 {
        n.put("Rotation", listf(&[395.0, -377.0]));
    }
    match scenario {
        4 => n.put("Pos", Tag::Int(1)),
        5 => n.put("Motion", listd(&[1.0, 2.0])),
        6 => n.put("Pos", listf(&[1.0, 2.0, 3.0])),
        7 => n.put("Rotation", listd(&[1.0, 2.0])),
        8 | 9 => {
            let mut list = vec![Tag::Double(1.0), Tag::Double(2.0), Tag::Double(3.0)];
            list.push(Tag::String("extra".into()));
            n.put(
                if scenario == 8 { "Motion" } else { "Pos" },
                Tag::List(8, list),
            );
        }
        10 => n.put(
            "Rotation",
            Tag::List(
                8,
                vec![
                    Tag::Float(1.0),
                    Tag::Float(2.0),
                    Tag::String("extra".into()),
                ],
            ),
        ),
        11 => n.put(
            "Motion",
            listd(&[f64::NAN, f64::INFINITY, f64::NEG_INFINITY]),
        ),
        12 => n.put("id", Tag::Int(1)),
        14 => n.put("id", Tag::String("unregistered".into())),
        15 => {
            n.put("FallDistance", Tag::Float(-3.25));
            for key in [
                "Fire",
                "Air",
                "Health",
                "HurtTime",
                "DeathTime",
                "AttackTime",
                "Age",
                "Value",
                "carried",
                "carryingData",
                "Anger",
                "Fuel",
            ] {
                n.put(key, Tag::Short(-1));
            }
            for key in [
                "OnGround", "Angry", "Sitting", "Saddle", "Sheared", "Color", "powered", "Fuse",
                "Tile", "Dir", "inTile", "inData", "shake", "inGround", "player",
            ] {
                n.put(key, Tag::Byte(-1));
            }
            n.put("Owner", Tag::String("Player".into()));
            n.put("Motive", Tag::String("Pointer".into()));
            for key in ["Size", "TileX", "TileY", "TileZ"] {
                n.put(key, Tag::Int(17));
            }
            n.put("Type", Tag::Int(1));
            let mut item = Compound::new();
            item.put("id", Tag::Short(35));
            item.put("Count", Tag::Byte(-1));
            item.put("Damage", Tag::Short(32767));
            item.put("Slot", Tag::Byte(26));
            n.put("Item", Tag::Compound(item.clone()));
            n.put("Items", Tag::List(10, vec![Tag::Compound(item)]));
            let mut effect = Compound::new();
            effect.put("Id", Tag::Byte(19));
            effect.put("Amplifier", Tag::Byte(-1));
            effect.put("Duration", Tag::Int(137));
            n.put("ActiveEffects", Tag::List(10, vec![Tag::Compound(effect)]));
        }
        16..=58 => {
            let key = FACTORY_FIELDS[(scenario - 16) as usize];
            n.put(key, Tag::String("wrong".into()));
            if key == "Items" {
                n.put("Type", Tag::Int(1));
            }
            if key.starts_with("Push") || key == "Fuel" {
                n.put("Type", Tag::Int(2));
            }
        }
        59..=63 => {
            let mut item = Compound::new();
            item.put(
                "id",
                Tag::Short([-1, 0, 2258, 32767, 35][(scenario - 59) as usize]),
            );
            item.put("Count", Tag::Byte(-128));
            item.put("Damage", Tag::Short(-32768));
            n.put("Item", Tag::Compound(item.clone()));
            item.put("Slot", Tag::Byte(0));
            n.put("Items", Tag::List(10, vec![Tag::Compound(item)]));
            n.put("Type", Tag::Int(1));
        }
        _ => {}
    }
    Some(n)
}
fn factory_extra(e: &Entity, o: &mut Vec<u8>) {
    match &e.kind {
        EntityKind::Item { item, .. } => {
            let present = e.dropped_stack_present;
            o.push(present as u8);
            if present {
                for v in [item.id as i32, item.count as i32, item.damage as i32] {
                    i(o, v);
                }
            }
        }
        EntityKind::Minecart(s) => {
            i(o, s.cart_type);
            for slot in 0..27 {
                let stack = s.items[slot];
                o.push(stack.is_some() as u8);
                if let Some(s) = stack {
                    for v in [s.id, s.count, s.damage] {
                        i(o, v);
                    }
                }
            }
        }
        EntityKind::Painting(s) => {
            o.push(s.art_initialized as u8);
            if s.art_initialized {
                text(o, super::painting::ARTS[s.art].title);
            }
        }
        EntityKind::Sheep { fleece_color, .. } => i(o, *fleece_color as i32),
        EntityKind::Wolf(s) => text(o, &s.owner),
        _ => {}
    }
}
pub fn entitylist() -> Vec<u8> {
    let mut o = Vec::new();
    // The fixture walks EntityList's registrations in addMapping (= ascending id) order.
    let mut entries: Vec<_> = super::registry::ENTRIES.iter().collect();
    entries.sort_by_key(|e| e.id);
    let mut names: Vec<_> = entries.iter().map(|e| Some(e.name)).collect();
    names.extend([None, Some("unknown"), Some("Player"), Some("")]);
    for name in names {
        for _null_world in [false, true] {
            for scenario in -2..64 {
                let mut w = super::verify181::world();
                let result = if scenario == -2 {
                    Ok(super::registry::create_nullable(&mut w, name))
                } else if scenario == -1 {
                    let id = name.and_then(super::registry::by_name).map_or(-1, |e| e.id);
                    Ok(super::registry::create_by_id(&mut w, id))
                } else {
                    super::registry::create_from_nbt_checked(
                        &mut w,
                        factory_nbt(name, scenario).as_ref(),
                    )
                };
                text(&mut o, result.as_ref().err().map_or("", |e| e.class()));
                let entity = result.ok().flatten();
                o.push(entity.is_some() as u8);
                if let Some(e) = entity {
                    text(&mut o, e.kind.name());
                    i(&mut o, super::registry::get_id_nullable(Some(&e)).unwrap());
                    super::verify181::state(&mut w, &e, &mut o);
                    factory_extra(&e, &mut o);
                }
                i(&mut o, w.next_entity_id);
                let super::EntitySeedSource::Deterministic(seed) = w.entity_seed_source else {
                    unreachable!()
                };
                o.extend(seed.to_le_bytes());
                o.extend(w.rand.state().to_le_bytes());
                o.extend(w.math_random.state().to_le_bytes());
            }
        }
    }
    let mut w = super::verify181::world();
    // getClass is an exact-class lookup, including unregistered subclasses.
    let unregistered = [
        None,
        Some(Entity::player(&mut w, "Player")),
        Some(Entity::base(&mut w, EntityKind::Egg(Default::default()))),
        Some(Entity::base(
            &mut w,
            EntityKind::Fireball(Default::default()),
        )),
        Some(Entity::base(
            &mut w,
            EntityKind::Lightning {
                state: 0,
                seed: 0,
                remaining: 0,
            },
        )),
    ];
    for e in unregistered {
        let id = super::registry::get_id_nullable(e.as_ref());
        i(&mut o, id.as_ref().copied().unwrap_or(0));
        text(&mut o, id.err().map_or("", |e| e.class()));
        let name = super::registry::get_string_nullable(e.as_ref());
        text(&mut o, name.as_ref().err().map_or("", |e| e.class()));
        nullable(&mut o, name.ok().flatten());
    }
    o
}
