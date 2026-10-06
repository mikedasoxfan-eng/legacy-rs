//! Original-bytecode comparisons for the release blocks area, round 3.
use worldgen::container::brewing::BrewingStand;
use worldgen::entity::{player::ItemStack, potion::PotionEffect};
use worldgen::{JavaRandom, enchantment as en, potion_helper as ph};
struct Dump(Vec<u8>);
impl Dump {
    fn i(&mut self, n: i32) {
        self.0.extend(n.to_be_bytes());
    }
    fn l(&mut self, n: i64) {
        self.0.extend(n.to_be_bytes());
    }
    fn b(&mut self, b: bool) {
        self.0.push(b as u8);
    }
    fn text(&mut self, s: &str) {
        self.i(s.len() as i32);
        self.0.extend(s.as_bytes());
    }
    fn effects(&mut self, e: Option<Vec<PotionEffect>>) {
        self.i(e.as_ref().map_or(-1, |e| e.len() as i32));
        if let Some(e) = e {
            for p in e {
                self.i(p.id);
                self.i(p.duration);
                self.i(p.amplifier);
            }
        }
    }
    fn data(&mut self, e: Option<Vec<en::EnchantmentData>>) {
        self.i(e.as_ref().map_or(-1, |e| e.len() as i32));
        if let Some(e) = e {
            for p in e {
                self.i(p.id);
                self.i(p.level);
                self.i(en::get(p.id).unwrap().weight);
            }
        }
    }
    fn stack(&mut self, s: Option<ItemStack>) {
        self.b(s.is_some());
        if let Some(s) = s {
            self.i(s.id);
            self.i(s.count);
            self.i(s.damage);
        }
    }
    fn brewing(&mut self, b: &BrewingStand) {
        self.i(b.brew_time);
        self.i(b.metadata());
        for s in b.items {
            self.stack(s);
        }
    }
    fn bb(&mut self, b: Option<worldgen::entity::AxisAlignedBB>) {
        self.b(b.is_some());
        if let Some(b) = b {
            for v in [b.min_x, b.min_y, b.min_z, b.max_x, b.max_y, b.max_z] {
                self.l(v.to_bits() as i64);
            }
        }
    }
    fn events(&mut self, w: &mut worldgen::World) {
        let events = w.drain_events();
        self.0.extend((events.len() as i32).to_le_bytes());
        for event in events {
            match event {
                worldgen::world::events::WorldEvent::Particle {
                    name,
                    position,
                    velocity,
                } => {
                    self.0.push(1);
                    self.0.extend((name.len() as i32).to_le_bytes());
                    self.0.extend(name.as_bytes());
                    for v in [
                        position.x, position.y, position.z, velocity.x, velocity.y, velocity.z,
                    ] {
                        self.0.extend(v.to_bits().to_le_bytes());
                    }
                }
                _ => panic!("unexpected event"),
            }
        }
    }
}
const INGREDIENTS: [i32; 11] = [353, 370, 375, 376, 382, 377, 378, 331, 348, 289, 372];
fn potion(o: &mut Dump) {
    let mut helper = ph::PotionHelper::default();
    for data in -128..65536 {
        o.i(ph::prefix_index(data) as i32);
        o.text(&ph::prefix(data));
        for flag in [false, true] {
            o.effects(ph::effects(data, flag));
            o.i(helper.color(data, flag));
            o.i(helper.color(data, flag));
        }
        for id in INGREDIENTS {
            o.i(ph::brew(data, ph::ingredient(id).unwrap()));
        }
    }
    for data in [i32::MIN, i32::MAX, -1, 0, 1, 31, 32, 32767, 65535] {
        for s in [
            "",
            "0",
            "!0",
            "-0",
            "0*12",
            "0|1",
            "0 | 1",
            "0 & 1",
            "!0 & 1 & !2 & !3 & 1+6",
            "=0",
            ">2",
            "<3",
            "31+32+63+64",
            "2147483648",
            "99999999999999999",
            "0-1-2+3",
            "0!!1",
            "0--1",
            "0&&1",
            "0||1",
            "ÃƒÂ°Ã…Â¸Ã‹Å“Ã¢â€šÂ¬+5",
            "+14&13-13",
            "-0+1-2-3&4-4+13",
        ] {
            o.i(ph::evaluate(s, data));
            o.i(ph::brew(data, s));
            for bit in -65..=65 {
                o.b(ph::bit(data, bit));
            }
            o.i(ph::pack_bits(data, [-1, 32, 33, 64, 65]));
        }
    }
    o.effects(None);
    o.i(ph::color(None));
    for amp in -2..=4 {
        for id in 1..20 {
            o.i(ph::color(Some(&[
                PotionEffect::new(id, 100, amp),
                PotionEffect::new(1, 1, 2),
            ])));
        }
    }
}
fn enchantments(o: &mut Dump) {
    let sources = [
        en::ProtectionSource::default(),
        en::ProtectionSource {
            fire: true,
            ..Default::default()
        },
        en::ProtectionSource {
            fire: true,
            ..Default::default()
        },
        en::ProtectionSource {
            fire: true,
            ..Default::default()
        },
        en::ProtectionSource {
            fall: true,
            ..Default::default()
        },
        en::ProtectionSource {
            explosion: true,
            ..Default::default()
        },
        en::ProtectionSource {
            creative_harm: true,
            ..Default::default()
        },
        en::ProtectionSource::default(),
        en::ProtectionSource::default(),
        en::ProtectionSource {
            projectile: true,
            ..Default::default()
        },
    ];
    for id in 0..256 {
        o.b(en::get(id).is_some());
        let Some(e) = en::get(id) else {
            continue;
        };
        o.i(id);
        o.i(e.weight);
        o.i(e.kind as i32);
        o.text(&e.translation_key());
        o.i(e.min_level());
        o.i(e.max_level());
        for level in [i32::MIN, -100, -1, 0, 1, 2, 3, 4, 5, 100, 46341, i32::MAX] {
            o.i(e.min_enchantability(level));
            o.i(e.max_enchantability(level));
            for s in sources {
                o.i(e.modifier_damage(level, s));
            }
            for a in [
                en::CreatureAttribute::Undefined,
                en::CreatureAttribute::Undead,
                en::CreatureAttribute::Arthropod,
            ] {
                o.i(e.modifier_living(level, a));
            }
        }
        for other in en::ENCHANTMENTS {
            o.b(e.compatible(other));
        }
    }
    for kind in [
        en::EnchantmentType::All,
        en::EnchantmentType::Armor,
        en::EnchantmentType::Feet,
        en::EnchantmentType::Legs,
        en::EnchantmentType::Torso,
        en::EnchantmentType::Head,
        en::EnchantmentType::Weapon,
        en::EnchantmentType::Digger,
    ] {
        for id in 0..32768 {
            if worldgen::entity::player::registry::item(id).is_some() {
                o.i(id);
                o.b(kind.can_enchant(id));
            }
        }
    }
    for (i, m) in [
        en::ArmorMaterial::Cloth,
        en::ArmorMaterial::Chain,
        en::ArmorMaterial::Iron,
        en::ArmorMaterial::Gold,
        en::ArmorMaterial::Diamond,
    ]
    .into_iter()
    .enumerate()
    {
        o.i(i as i32);
        o.i(m.enchantability());
        for slot in 0..4 {
            o.i(m.durability(slot));
            o.i(m.reduction(slot));
        }
    }
    for (i, r) in [
        worldgen::item10::Rarity::Common,
        worldgen::item10::Rarity::Uncommon,
        worldgen::item10::Rarity::Rare,
        worldgen::item10::Rarity::Epic,
    ]
    .into_iter()
    .enumerate()
    {
        o.i(i as i32);
        o.i(r.color());
        o.text(r.name());
    }
}
fn helper(o: &mut Dump) {
    for id in 256..=382 {
        if worldgen::entity::player::registry::item(id).is_some() {
            for power in 0..=80 {
                o.data(en::map_data(power, ItemStack::new(id, 1, 0)));
            }
        }
    }
    for seed in 0..128 {
        for id in [
            1, 256, 257, 258, 267, 268, 272, 276, 283, 298, 301, 302, 305, 306, 309, 310, 313, 314,
            317, 261, 373,
        ] {
            let stack = ItemStack::new(id, 1, 0);
            for power in [0, 1, 5, 15, 30, 50, 100] {
                let mut rng = JavaRandom::new(seed);
                o.data(en::build_list(&mut rng, stack, power));
                o.l(rng.state());
            }
            for shelves in [0, 1, 15, 30, 31, 100] {
                for slot in -1..4 {
                    let mut rng = JavaRandom::new(seed);
                    o.i(en::calc_enchantability(&mut rng, slot, shelves, stack));
                    o.l(rng.state());
                }
            }
        }
    }
}
fn brewing(o: &mut Dump) {
    for id in INGREDIENTS {
        for damage in 0..32768 {
            let mut b = BrewingStand::default();
            b.items = [
                Some(ItemStack::new(373, 1, damage)),
                Some(ItemStack::new(373, 2, damage | 16384)),
                Some(ItemStack::new(1, 0, 7)),
                Some(ItemStack::new(id, 2, 0)),
            ];
            o.b(b.can_brew());
            b.brew();
            o.brewing(&b);
        }
    }
    let mut meta = 0;
    for scenario in 0..8 {
        let mut b = BrewingStand::default();
        b.items[0] = Some(ItemStack::new(373, 1, if scenario == 7 { 8194 } else { 0 }));
        b.items[3] = Some(ItemStack::new(if scenario == 7 { 331 } else { 372 }, 2, 0));
        for tick in 0..605 {
            if tick == 50 {
                match scenario {
                    1 => b.items[3] = None,
                    2 => b.items[3] = Some(ItemStack::new(353, 1, 0)),
                    3 => b.items[3].as_mut().unwrap().count = 0,
                    4 => b.items[0] = None,
                    5 => {
                        let n = b.to_nbt((0, 65, 0));
                        b = BrewingStand::default();
                        b.read_nbt(&n).unwrap();
                    }
                    6 => b.brew_time = -1,
                    _ => {}
                }
            }
            let update = b.tick();
            if let Some(m) = update.metadata {
                meta = m;
            }
            o.brewing(&b);
            o.i(meta);
        }
    }
    for slot in -2..6 {
        for count in [-1, 0, 1, 100] {
            let mut b = BrewingStand::default();
            b.set(slot, Some(ItemStack::new(353, 99, 3)));
            o.stack(b.get(slot));
            o.stack(b.decrement(slot, count));
            o.stack(b.get(slot));
        }
    }
}
fn modifiers(o: &mut Dump) {
    let sources = [
        en::ProtectionSource::default(),
        en::ProtectionSource {
            fire: true,
            ..Default::default()
        },
        en::ProtectionSource {
            fall: true,
            ..Default::default()
        },
        en::ProtectionSource {
            explosion: true,
            ..Default::default()
        },
        en::ProtectionSource {
            creative_harm: true,
            ..Default::default()
        },
        en::ProtectionSource {
            projectile: true,
            ..Default::default()
        },
    ];
    for level in [0, 1, 2, 4, 10, 127, 255, 256, 32767] {
        let held = ItemStack::new(276, 1, 0);
        for e in en::ENCHANTMENTS {
            held.add_enchantment(e.id, level);
            held.add_enchantment(e.id, 1);
        }
        let armor: Vec<_> = (0..4)
            .map(|slot| {
                let stack = ItemStack::new(310 + slot, 1, 0);
                for e in en::ENCHANTMENTS {
                    stack.add_enchantment(e.id, level);
                }
                Some(stack)
            })
            .collect();
        for id in 0..256 {
            o.i(en::level(id, Some(held)));
            o.i(en::max_level(id, &armor));
        }
        for seed in 0..16 {
            for s in sources {
                let mut rng = JavaRandom::new(seed);
                o.i(0);
                let value = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    en::modifier_damage(&mut rng, &armor, s)
                }));
                o.i(value.unwrap_or(i32::MIN));
                o.l(rng.state());
            }
        }
        for seed in 0..16 {
            for a in [
                en::CreatureAttribute::Undefined,
                en::CreatureAttribute::Undead,
                en::CreatureAttribute::Arthropod,
            ] {
                let mut rng = JavaRandom::new(seed);
                o.i(en::modifier_living(&mut rng, Some(held), a));
                o.l(rng.state());
            }
        }
        o.i(en::level(19, Some(held)));
        o.i(en::level(20, Some(held)));
        o.i(en::max_level(5, &armor));
        o.i(en::level(32, Some(held)));
        o.i(en::level(34, Some(held)));
        o.b(en::level(33, Some(held)) > 0);
        o.i(en::level(35, Some(held)));
        o.i(en::level(21, Some(held)));
        o.b(en::max_level(6, &armor) > 0);
    }
}
fn names(o: &mut Dump) {
    for seed in [0, 1, 42, -1, i64::MIN, i64::MAX] {
        let mut names = en::EnchantmentNameParts::new(seed);
        for _ in 0..1000 {
            o.text(&names.next_name());
        }
        names.set_seed(seed);
        o.text(&names.next_name());
    }
}
fn animation(o: &mut Dump) {
    for x in [0, -20, 16777216] {
        for seed in 0..16 {
            let mut rng = JavaRandom::new(seed);
            let mut t = worldgen::container::enchanting::EnchantmentTable::default();
            for tick in 0..400 {
                let offset = if tick < 100 {
                    0.5
                } else if tick < 200 {
                    -1.0
                } else if tick < 300 {
                    3.0
                } else {
                    2.0
                };
                let px = (x as f32 + 0.5) as f64 + offset;
                let pz = -2.5 + (tick % 3 - 1) as f64;
                let dx = px - (x as f32 + 0.5) as f64;
                let dz = pz - (-3f32 + 0.5) as f64;
                let player = if dx * dx + dz * dz < 9.0 {
                    Some((px, pz))
                } else {
                    None
                };
                t.tick((x, 65, -3), player, &mut rng);
                o.i(t.ticks);
                for f in [
                    t.page,
                    t.previous_page,
                    t.target_page,
                    t.page_velocity,
                    t.open,
                    t.previous_open,
                    t.rotation,
                    t.previous_rotation,
                    t.target_rotation,
                ] {
                    o.i(f.to_bits() as i32);
                }
                o.l(rng.state());
            }
        }
    }
}
fn potionitem(o: &mut Dump) {
    let mut item = worldgen::item10::PotionItem::default();
    let mut color = ph::PotionHelper::default();
    let translate = |s: &str| match s {
        "item.emptyPotion.name" => " \tEMPTY\n".into(),
        "potion.prefix.grenade" => " \tSPLASH\n".into(),
        s => s.into(),
    };
    for damage in -32..32768 {
        let effects = item.effects(damage);
        o.effects(effects.as_ref().map(|e| e.lock().unwrap().clone()));
        let a = item.effects(damage);
        let b = item.effects(damage);
        o.b(match (a, b) {
            (None, None) => true,
            (Some(a), Some(b)) => std::sync::Arc::ptr_eq(&a, &b),
            _ => false,
        });
        o.i(worldgen::item10::PotionItem::icon(damage));
        o.i(color.color(damage, false));
        o.b(item.has_instant(damage));
        o.b(effects.is_some_and(|e| !e.lock().unwrap().is_empty()));
        o.text(&item.display_name(damage, "BASE", translate));
        let lines = item.tooltip(damage, translate);
        o.i(lines.len() as i32);
        for line in lines {
            o.text(&line);
        }
    }
    for remote in [false, true] {
        for count in [-1, 0, 1, 2, 64] {
            for damage in [0, 16, 8194, 8197, 16421] {
                let s = ItemStack::new(373, count, damage);
                let mut added = Vec::new();
                let mut effects = Vec::new();
                let result = item.drink(
                    s,
                    remote,
                    |effect| effects.push(effect),
                    |bottle| added.push(bottle),
                );
                let mut eaten = s;
                eaten.count = eaten.count.wrapping_sub(1);
                o.stack(Some(eaten));
                o.stack(Some(result));
                o.b(eaten.identity == result.identity);
                for slot in 0..36 {
                    o.stack(if slot == 0 {
                        added.first().copied()
                    } else {
                        None
                    });
                }
                for id in 1..20 {
                    let effect = effects.iter().find(|p| p.id == id);
                    o.b(effect.is_some());
                    if let Some(effect) = effect {
                        o.i(effect.duration);
                        o.i(effect.amplifier);
                    }
                }
            }
        }
    }
}
fn raw(w: &mut worldgen::World, x: i32, y: i32, z: i32, id: u8, meta: i32) {
    if !(0..128).contains(&y) {
        return;
    }
    let c = w.chunks.get_mut(&(x >> 4, z >> 4)).unwrap();
    let idx = ((x & 15) << 11 | (z & 15) << 7 | y) as usize;
    c.blocks[idx] = id;
    c.data.set(x & 15, y, z & 15, meta);
}
fn geometry(o: &mut Dump) {
    let mut w = worldgen::entity::verify181::world();
    for id in [
        85, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122,
    ] {
        for meta in 0..16 {
            for n in [0, 1, 20, 54, 85, 86, 107, 113, 117, 118] {
                raw(&mut w, 8, 64, 8, id, meta);
                for (dx, dz) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    raw(&mut w, 8 + dx, 64, 8 + dz, n, 0);
                }
                w.set_block_bounds_based_on_state(id, 8, 64, 8);
                o.bb(Some(w.block_bounds[id as usize].translated(8., 64., 8.)));
                o.bb(w.block_collision_box(id, 8, 64, 8));
                let boxes = w.get_colliding_bounding_boxes(
                    None,
                    worldgen::entity::AxisAlignedBB::new(8., 64., 8., 9., 66., 9.),
                );
                o.i(boxes.len() as i32);
                for b in boxes {
                    o.bb(Some(b));
                }
                for side in 0..6 {
                    o.i(worldgen::block::visual181::texture(
                        id,
                        side,
                        meta as usize,
                        true,
                    ));
                }
            }
        }
    }
    for height in [64, 65, 128] {
        for y in [-1, 0, 63, 64, 65, 127, 128] {
            for below in [0, 1, 8, 9, 88] {
                for meta in 0..2 {
                    w.height10 = height;
                    raw(&mut w, 8, y - 1, 8, below, meta);
                    o.b(worldgen::world::blocks10::can_stay(&mut w, 111, 8, y, 8));
                    o.b(worldgen::world::blocks10::can_stay(&mut w, 115, 8, y, 8));
                }
            }
        }
    }
}
fn particles(o: &mut Dump) {
    let mut w = worldgen::entity::verify181::world();
    for id in [110, 116, 117, 119] {
        for seed in 0..128 {
            for x in 6i32..=10 {
                for z in 6i32..=10 {
                    for y in 64..=65 {
                        raw(
                            &mut w,
                            x,
                            y,
                            z,
                            if (x - 8).abs() == 2 || (z - 8).abs() == 2 {
                                47
                            } else {
                                0
                            },
                            0,
                        );
                    }
                }
            }
            let mut rng = JavaRandom::new(seed);
            worldgen::world::blocks10::display(&mut w, id, 8, 64, 8, &mut rng);
            o.l(rng.state());
            o.events(&mut w);
        }
    }
}
fn recipes(o: &mut Dump) {
    let manager = worldgen::container::recipes::CraftingManager::instance();
    o.i(manager.recipes.len() as i32);
    for recipe in &manager.recipes {
        o.b(recipe.shaped);
        o.i(recipe.width as i32);
        o.i(recipe.height as i32);
        o.stack(Some(recipe.output));
        o.i(recipe.ingredients.len() as i32);
        for s in &recipe.ingredients {
            o.stack(*s);
        }
    }
    for id in 0..2258 {
        o.stack(manager.smelting_result(id as i32));
        o.i(manager.container_items[id]);
        o.i(manager.armor_types[id]);
        o.b(manager.wood[id]);
    }
}
fn fortune(o: &mut Dump) {
    std::panic::set_hook(Box::new(|_| {}));
    let mut w = worldgen::entity::verify181::world();
    for id in 1..=122 {
        for meta in 0..16 {
            for fortune in -1..=4 {
                for seed in 0..8 {
                    for method in 0..3 {
                        w.rand.set_seed(seed);
                        let value =
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(
                                || match method {
                                    0 => worldgen::world::behavior::quantity_dropped(&mut w, id),
                                    1 => worldgen::world::behavior::quantity_dropped_fortune(
                                        &mut w, id, fortune,
                                    ),
                                    _ => worldgen::world::behavior::id_dropped_fortune(
                                        &mut w, id, meta, fortune,
                                    ),
                                },
                            ));
                        o.i(value.unwrap_or(i32::MIN));
                        o.l(w.rand.state());
                    }
                }
            }
        }
    }
}
fn eyeright(o: &mut Dump) {
    for remote in [false, true] {
        for creative in [false, true] {
            for frame in [false, true] {
                for target in 0..3 {
                    for x in [-1.75, 0.5, 9.9] {
                        let mut w = worldgen::entity::verify181::world();
                        let mut p = worldgen::entity::Entity::player(&mut w, "Player");
                        p.set_position(x, 65., -3.25);
                        p.set_creative(creative);
                        w.multiplayer_world = remote;
                        if frame {
                            raw(&mut w, x.floor() as i32, 65, 0, 120, 0);
                        }
                        w.item_random.set_seed(42);
                        let mut query = None;
                        let stack = ItemStack::new(381, 3, 0);
                        let result = p.ender_eye_right_click10(
                            &mut w,
                            stack,
                            |_, name, pos| {
                                assert_eq!(name, "Stronghold");
                                query = Some(pos);
                                match target {
                                    1 => Some([2, 66, 1]),
                                    2 => Some([100, 45, -200]),
                                    _ => None,
                                }
                            },
                            |w, pos, target| {
                                let mut eye = worldgen::entity::Entity::ender_eye(w, pos);
                                eye.set_eye_target(target[0] as f64, target[1], target[2] as f64);
                                w.entity_joined_world(eye);
                            },
                        );
                        o.stack(Some(result));
                        o.b(stack.identity == result.identity);
                        o.i(query.is_some() as i32);
                        if let Some(pos) = query {
                            for v in pos {
                                o.i(v);
                            }
                        }
                        o.l(w.item_random.state());
                        o.i(w.loaded_entity_list.len() as i32);
                        for id in &w.loaded_entity_list {
                            let e = &w.entities[id];
                            for v in [e.pos.x, e.pos.y, e.pos.z] {
                                o.l(v.to_bits() as i64);
                            }
                            if let worldgen::entity::EntityKind::EnderEye(s) = &e.kind {
                                for v in [s.target.x, s.target.y, s.target.z] {
                                    o.l(v.to_bits() as i64);
                                }
                            } else {
                                panic!("eye");
                            }
                            o.l(e.rand.state());
                        }
                        let events = w.drain_events();
                        o.0.extend((events.len() as i32).to_le_bytes());
                        for e in events {
                            match e {
                                worldgen::world::events::WorldEvent::Sound {
                                    name,
                                    position,
                                    volume,
                                    pitch,
                                } => {
                                    o.0.push(0);
                                    let name = name.unwrap();
                                    o.0.extend((name.len() as i32).to_le_bytes());
                                    o.0.extend(name.as_bytes());
                                    for v in [position.x, position.y, position.z] {
                                        o.0.extend(v.to_bits().to_le_bytes());
                                    }
                                    o.0.extend(volume.to_bits().to_le_bytes());
                                    o.0.extend(pitch.to_bits().to_le_bytes());
                                }
                                worldgen::world::events::WorldEvent::Auxiliary {
                                    code,
                                    position,
                                    data,
                                    ..
                                } => {
                                    o.0.push(3);
                                    for v in [code, position.0, position.1, position.2, data] {
                                        o.0.extend(v.to_le_bytes());
                                    }
                                }
                                _ => panic!("eye event"),
                            }
                        }
                    }
                }
            }
        }
    }
}
fn dragon(o: &mut Dump) {
    let mut w = worldgen::entity::verify181::world();
    for remote in [false, true] {
        for seed in 0..64 {
            w.multiplayer_world = remote;
            w.rand.set_seed(seed);
            raw(&mut w, 8, 64, 8, 122, 11);
            worldgen::world::blocks10::dragon_egg_teleport(&mut w, 8, 64, 8);
            o.b(true);
            o.l(w.rand.state());
            o.events(&mut w);
            for x in -7..=23 {
                for y in 57..=71 {
                    for z in -7..=23 {
                        if w.get_block_id(x, y, z) == 122 {
                            o.i(x);
                            o.i(y);
                            o.i(z);
                            o.i(w.get_block_metadata(x, y, z));
                            raw(&mut w, x, y, z, 0, 0);
                        }
                    }
                }
            }
            o.i(i32::MIN);
        }
    }
    w.multiplayer_world = false;
    for x in -7..=23 {
        for y in 57..=71 {
            for z in -7..=23 {
                raw(&mut w, x, y, z, 1, 0);
            }
        }
    }
    raw(&mut w, 8, 64, 8, 122, 0);
    w.rand.set_seed(42);
    worldgen::world::blocks10::dragon_egg_teleport(&mut w, 8, 64, 8);
    o.l(w.rand.state());
    o.events(&mut w);
    o.i(w.get_block_id(8, 64, 8) as i32);
}
fn cauldron(o: &mut Dump) {
    let mut w = worldgen::entity::verify181::world();
    let mut p = worldgen::entity::Entity::player(&mut w, "Player");
    for remote in [false, true] {
        for creative in [false, true] {
            for full in [false, true] {
                for meta in 0..16 {
                    for id in [0, 1, 326, 374] {
                        for count in [-1, 0, 1, 3] {
                            w.multiplayer_world = remote;
                            p.set_creative(creative);
                            for slot in 0..36 {
                                p.player_state_mut().unwrap().inventory.main[slot] = if full {
                                    Some(ItemStack::new(1, 64, 0))
                                } else {
                                    None
                                };
                            }
                            p.player_state_mut().unwrap().inventory.main[0] = if id == 0 {
                                None
                            } else {
                                Some(ItemStack::new(id, count, 0))
                            };
                            w.loaded_entity_list.clear();
                            w.set_entity_determinism(100, 200, 0);
                            w.rand.set_seed(300);
                            raw(&mut w, 8, 64, 8, 118, meta);
                            o.b(worldgen::world::blocks10::cauldron_activate(
                                &mut p, &mut w, 8, 64, 8,
                            ));
                            o.i(w.get_block_metadata(8, 64, 8));
                            for s in p.player_state().unwrap().inventory.main {
                                o.stack(s);
                            }
                            o.i(w.loaded_entity_list.len() as i32);
                            for id in &w.loaded_entity_list {
                                let e = &w.entities[id];
                                if let worldgen::entity::EntityKind::Item { item, .. } = &e.kind {
                                    o.stack(Some((*item).into()));
                                } else {
                                    panic!("item");
                                }
                                for v in [
                                    e.pos.x, e.pos.y, e.pos.z, e.motion.x, e.motion.y, e.motion.z,
                                ] {
                                    o.l(v.to_bits() as i64);
                                }
                                o.l(e.rand.state());
                            }
                            o.l(w.rand.state());
                        }
                    }
                }
            }
        }
    }
}
fn eye(o: &mut Dump) {
    let mut w = worldgen::entity::verify181::world();
    let mut p = worldgen::entity::Entity::player(&mut w, "Player");
    let dx = [0, -1, 0, 1];
    let dz = [1, 0, -1, 0];
    for remote in [false, true] {
        for creative in [false, true] {
            for dir in 0..4 {
                for missing in -1..12 {
                    w.multiplayer_world = remote;
                    p.set_creative(creative);
                    for x in 2..=14 {
                        for z in 2..=14 {
                            raw(&mut w, x, 64, z, 0, 0);
                        }
                    }
                    let (sx, sz) = (dx[(dir + 1) % 4], dz[(dir + 1) % 4]);
                    let (dx, dz) = (dx[dir], dz[dir]);
                    let mut at = 0;
                    for forward in [0, 4] {
                        for side in -1..=1 {
                            raw(
                                &mut w,
                                8 + sx * side + dx * forward,
                                64,
                                8 + sz * side + dz * forward,
                                if at == missing { 0 } else { 120 },
                                dir as i32 + 4,
                            );
                            at += 1;
                        }
                    }
                    for side in [-2, 2] {
                        for forward in 1..=3 {
                            raw(
                                &mut w,
                                8 + sx * side + dx * forward,
                                64,
                                8 + sz * side + dz * forward,
                                if at == missing { 0 } else { 120 },
                                dir as i32 + 4,
                            );
                            at += 1;
                        }
                    }
                    raw(&mut w, 8, 64, 8, 120, dir as i32);
                    w.item_random.set_seed(42);
                    let mut stack = ItemStack::new(381, 3, 0);
                    o.b(p.insert_ender_eye10(&mut w, &mut stack, 8, 64, 8));
                    o.stack(Some(stack));
                    o.l(w.item_random.state());
                    o.events(&mut w);
                    for x in 2..=14 {
                        for z in 2..=14 {
                            o.i(w.get_block_id(x, 64, z) as i32);
                            o.i(w.get_block_metadata(x, 64, z));
                        }
                    }
                }
            }
        }
    }
}
fn enchant_state(o: &mut Dump, c: &worldgen::container::Container, level: i32) {
    let e = c.enchanting.as_ref().unwrap();
    o.l(e.name_seed);
    for cost in e.costs {
        o.i(cost);
    }
    o.l(e.rng.state());
    o.i(level);
    o.stack(e.item);
    for id in 0..256 {
        o.i(en::max_level(id, &[e.item]));
    }
}
fn enchantcontainer(o: &mut Dump) {
    use worldgen::container::{Container, ContainerKind};
    for remote in [false, true] {
        for mask in 0..4 {
            for seed in 0..16 {
                for id in [1, 256, 261, 267, 276, 283, 298, 310, 373] {
                    for count in [1, 2] {
                        let mut w = worldgen::entity::verify181::world();
                        for x in 6i32..=10 {
                            for z in 6i32..=10 {
                                for y in 64..=65 {
                                    raw(
                                        &mut w,
                                        x,
                                        y,
                                        z,
                                        if ((x - 8).abs() == 2 || (z - 8).abs() == 2) && mask > 0 {
                                            47
                                        } else if mask == 2 && x == 9 && z == 9 {
                                            1
                                        } else {
                                            0
                                        },
                                        0,
                                    );
                                }
                            }
                        }
                        let mut c = Container::new(ContainerKind::Enchantment, 1);
                        c.multiplayer = remote;
                        c.bookshelves =
                            worldgen::container::enchanting::world_bookshelves(&mut w, (8, 64, 8));
                        c.enchanting.as_mut().unwrap().rng.set_seed(seed);
                        let mut p = worldgen::entity::player::InventoryPlayer::default();
                        let mut level = 100;
                        c.put_stack(
                            &mut p,
                            0,
                            Some(ItemStack::new(id, count, if mask == 3 { 5 } else { 0 })),
                        );
                        enchant_state(o, &c, level);
                        for button in 0..3 {
                            o.b(c.enchanting.as_mut().unwrap().enchant(
                                button,
                                &mut level,
                                remote,
                                c.bookshelves,
                            ));
                            enchant_state(o, &c, level);
                        }
                        for progress in -1..=3 {
                            c.enchanting
                                .as_mut()
                                .unwrap()
                                .progress(progress, progress * 3);
                            enchant_state(o, &c, level);
                        }
                    }
                }
            }
        }
    }
}
fn sounds(o: &mut Dump) {
    for id in 1..=122 {
        let (step, volume, pitch) = worldgen::world::events::step_sound(id);
        let (broken, _, _) = worldgen::world::events::break_sound(id);
        o.i(id as i32);
        o.text(step);
        o.text(broken);
        o.i(volume.to_bits() as i32);
        o.i(pitch.to_bits() as i32);
    }
}
fn brewcontainer(o: &mut Dump) {
    use worldgen::container::{Container, ContainerEvent, ContainerKind};
    use worldgen::entity::player::{InventoryPlayer, registry};
    let mut c = Container::new(ContainerKind::BrewingStand, 1);
    o.i(c.slots.len() as i32);
    for slot in &c.slots {
        o.i(slot.index as i32);
        o.i(slot.x);
        o.i(slot.y);
        o.i(slot.stack_limit());
    }
    for id in 0..32000 {
        if registry::item(id).is_none() {
            continue;
        }
        for damage in [0, 1, 8194, 16384] {
            for slot in 0..4 {
                let stack = ItemStack::new(id, 1, damage);
                o.b(c.slots[slot].is_item_valid(stack));
                o.i(if slot < 3 && id == 373 && damage > 0 {
                    1
                } else {
                    0
                });
            }
        }
    }
    for source in 0..40 {
        for id in [1, 353, 372, 373, 374] {
            for damage in [0, 8194] {
                for count in [1, 3] {
                    for action in 0..3 {
                        for full in [false, true] {
                            let mut p = InventoryPlayer::default();
                            c = Container::new(ContainerKind::BrewingStand, 1);
                            if full {
                                for slot in 4..40 {
                                    c.put_stack(&mut p, slot, Some(ItemStack::new(1, 63, 0)));
                                }
                            }
                            c.put_stack(&mut p, source, Some(ItemStack::new(id, count, damage)));
                            o.stack(c.slot_click(
                                &mut p,
                                source as i32,
                                if action == 1 { 1 } else { 0 },
                                action == 2,
                            ));
                            for slot in 0..40 {
                                o.stack(c.stack(&p, slot));
                            }
                            o.stack(p.cursor);
                            o.i(c
                                .events
                                .iter()
                                .filter(|e| matches!(e, ContainerEvent::Achievement("potion")))
                                .count() as i32);
                        }
                    }
                }
            }
        }
    }
}
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let mut o = Dump(Vec::new());
    match a[1].as_str() {
        "potion" => potion(&mut o),
        "enchantments" => enchantments(&mut o),
        "helper" => helper(&mut o),
        "modifiers" => modifiers(&mut o),
        "names" => names(&mut o),
        "animation" => animation(&mut o),
        "potionitem" => potionitem(&mut o),
        "geometry" => geometry(&mut o),
        "particles" => particles(&mut o),
        "brewing" => brewing(&mut o),
        "dragon" => dragon(&mut o),
        "cauldron" => cauldron(&mut o),
        "eye" => eye(&mut o),
        "eyeright" => eyeright(&mut o),
        "recipes" => recipes(&mut o),
        "fortune" => fortune(&mut o),
        "enchantcontainer" => enchantcontainer(&mut o),
        "sounds" => sounds(&mut o),
        "brewcontainer" => brewcontainer(&mut o),
        s => panic!("unknown check {s}"),
    }
    let expected = std::fs::read(&a[2]).unwrap();
    if expected != o.0 {
        std::fs::create_dir_all("target/r10").unwrap();
        std::fs::write(format!("target/r10/blocks3-{}.rust.bin", a[1]), &o.0).unwrap();
        let at = expected
            .iter()
            .zip(&o.0)
            .position(|(a, b)| a != b)
            .unwrap_or(expected.len().min(o.0.len()));
        eprintln!(
            "DIFFERS: {} byte {at}, Java {} bytes, Rust {} bytes; Java {:?}, Rust {:?}",
            a[1],
            expected.len(),
            o.0.len(),
            &expected[at.saturating_sub(8)..(at + 12).min(expected.len())],
            &o.0[at.saturating_sub(8)..(at + 12).min(o.0.len())]
        );
        std::process::exit(1);
    }
    println!("EXACT: blocks3 {} ({} bytes)", a[1], expected.len());
}
