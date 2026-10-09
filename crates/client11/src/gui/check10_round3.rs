//! Round-three original-bytecode comparisons. Values come from production simulation/UI code.
use super::check10_round2::{i, s};
use worldgen::{
    container::{Container, ContainerEvent, ContainerKind, InventoryPlayer, ItemStack},
    JavaRandom,
};
fn stack(o: &mut Vec<u8>, s: Option<ItemStack>) {
    o.push(s.is_some() as u8);
    if let Some(s) = s {
        for n in [s.id, s.count, s.damage] {
            i(o, n);
        }
    }
}
fn layout(o: &mut Vec<u8>, c: &Container) {
    i(o, c.slots.len() as i32);
    for (number, s) in c.slots.iter().enumerate() {
        for n in [number as i32, s.index as i32, s.x, s.y, s.stack_limit()] {
            i(o, n);
        }
    }
}
fn achievements(o: &mut Vec<u8>, events: &[ContainerEvent]) {
    let ids: Vec<_> = events
        .iter()
        .filter_map(|e| {
            if let ContainerEvent::Achievement(key) = e {
                super::stats::ACHIEVEMENTS
                    .iter()
                    .find(|a| a.key == *key)
                    .map(|a| a.id)
            } else {
                None
            }
        })
        .collect();
    i(o, ids.len() as i32);
    for id in ids {
        i(o, id);
    }
}
fn state(o: &mut Vec<u8>, c: &mut Container, p: &InventoryPlayer) {
    for n in 0..c.slots.len() {
        stack(o, c.stack(p, n));
    }
    stack(o, p.cursor);
    achievements(o, &c.events);
    let drops: Vec<_> = c
        .events
        .iter()
        .filter_map(|e| {
            if let ContainerEvent::Drop(s) = e {
                Some(*s)
            } else {
                None
            }
        })
        .collect();
    i(o, drops.len() as i32);
    for s in drops {
        stack(o, Some(s));
    }
    c.events.clear();
}
fn shelves(r: &mut JavaRandom) -> [u8; 50] {
    std::array::from_fn(|_| match r.next_int(4) {
        0 => 1,
        1 => 47,
        _ => 0,
    })
}
fn shelf_count(blocks: &[u8; 50]) -> i32 {
    worldgen::container::enchanting::bookshelf_count(
        |x, y, z| blocks[((z + 2) * 10 + y * 5 + x + 2) as usize],
        (0, 0, 0),
    )
}
fn enchstate(o: &mut Vec<u8>, c: &worldgen::container::enchanting::Enchanting, level: i32) {
    o.extend(c.name_seed.to_le_bytes());
    for n in c.costs {
        i(o, n);
    }
    i(o, level);
    stack(o, c.item);
    let es = c.item.map_or_else(Vec::new, |s| s.enchantments());
    i(o, es.len() as i32);
    for e in es {
        i(o, e.id);
        i(o, e.level);
    }
}
fn fixture_ui() -> super::Ui {
    let mut ui = super::utility_check::fixture_ui("target".into());
    for &name in super::TEXTURES {
        ui.textures.insert(name.to_string(), 0);
    }
    ui
}
pub fn check(mode: &str, path: &str) -> bool {
    let mut out = Vec::new();
    match mode {
        "enchantclick" => {
            use super::{screens::container::GuiContainer, Screen};
            use worldgen::container::ContainerCommand;
            let mut ui = fixture_ui();
            let mut r = JavaRandom::new(30043);
            for n in 0..1000 {
                let mut c = Container::new(ContainerKind::Enchantment, n);
                let mut p = InventoryPlayer::default();
                let w = 300 + r.next_int(1000);
                let h = 220 + r.next_int(600);
                if r.next_bool() {
                    c.put_stack(&mut p, 0, Some(ItemStack::new(276, 1, 0)));
                }
                for k in 0..3 {
                    c.progress[k] = r.next_int(40) - 5;
                }
                ui.release.player_level = r.next_int(45) - 5;
                let row = r.next_int(3);
                let mx = (w - 176) / 2 + 60 + r.next_int(112) - 2;
                let my = (h - 166) / 2 + 14 + 19 * row + r.next_int(23) - 2;
                let button = r.next_int(3);
                let snap = c.snapshot(&p);
                let mut g = GuiContainer::new(&snap);
                g.base().width = w;
                g.base().height = h;
                ui.container_snapshot = Some(snap);
                g.init_gui(&mut ui);
                ui.container_commands.clear();
                g.mouse_clicked(&mut ui, mx, my, button);
                let calls: Vec<_> = ui
                    .container_commands
                    .iter()
                    .filter_map(|c| {
                        if let ContainerCommand::Enchant { window, offer } = c {
                            Some([*window, *offer as i32])
                        } else {
                            None
                        }
                    })
                    .collect();
                i(&mut out, calls.len() as i32);
                for call in calls {
                    for v in call {
                        i(&mut out, v);
                    }
                }
            }
        }
        "progress" => {
            let mut r = JavaRandom::new(30042);
            for kind in [ContainerKind::Enchantment, ContainerKind::BrewingStand] {
                let mut c = Container::new(kind, 0);
                for n in 0..1000 {
                    for id in -1..4 {
                        let value = if n % 4 == 0 { 0 } else { r.next_int(500) - 50 };
                        c.update_progress_bar(id, value);
                    }
                    let listeners = (n % 5) as usize;
                    let updates = c.progress_updates(listeners);
                    i(&mut out, updates.len() as i32);
                    for u in updates {
                        for v in u {
                            i(&mut out, v);
                        }
                    }
                    for v in c.progress {
                        i(&mut out, v);
                    }
                    let again = c.progress_updates(listeners);
                    i(&mut out, again.len() as i32);
                    for u in again {
                        for v in u {
                            i(&mut out, v);
                        }
                    }
                }
            }
        }
        "endcopy" => {
            use worldgen::entity::Entity;
            fn player(out: &mut Vec<u8>, e: &Entity) {
                let p = e.player_state().unwrap();
                for v in [
                    e.living.health,
                    p.dimension,
                    p.experience.level,
                    p.experience.total,
                    p.score,
                    p.inventory.current_item,
                ] {
                    i(out, v);
                }
                out.extend(p.experience.points.to_le_bytes());
                let food = p.food.snapshot();
                for v in [food.level, food.tick_timer, food.prev_level] {
                    i(out, v);
                }
                for v in [food.saturation, food.exhaustion] {
                    out.extend(v.to_le_bytes());
                }
                for s in p.inventory.main.iter().chain(p.inventory.armor.iter()) {
                    stack(out, *s);
                }
                stack(out, p.inventory.cursor);
            }
            let mut w = worldgen::World::new(42, 42);
            let mut r = JavaRandom::new(30041);
            for _ in 0..200 {
                let mut p = Entity::player(&mut w, "Player");
                let mut q = Entity::player(&mut w, "Player");
                p.living.health = r.next_int(21);
                let s = p.player_state_mut().unwrap();
                s.dimension = 1;
                s.experience.level = r.next_int(70);
                s.experience.total = r.next_int(3000);
                s.experience.points = r.next_float();
                s.score = r.next_int_unbounded();
                s.inventory.current_item = r.next_int(9);
                s.inventory.cursor = Some(ItemStack::new(264, 5, 0));
                {
                    let mut food = s.food.write();
                    food.level = r.next_int(21);
                    food.saturation = r.next_float() * 5.0;
                    food.exhaustion = r.next_float() * 4.0;
                    food.tick_timer = r.next_int(80);
                }
                for k in 0..40 {
                    if r.next_bool() {
                        let ids = [1, 264, 276, 306];
                        let id = ids[r.next_int(4) as usize];
                        let item = Some(ItemStack::new(id, 1 + r.next_int(5), r.next_int(20)));
                        if k < 36 {
                            s.inventory.main[k] = item;
                        } else {
                            s.inventory.armor[k - 36] = item;
                        }
                    }
                }
                q.copy_player_from(&p);
                q.player_state_mut().unwrap().dimension = 0;
                player(&mut out, &q);
                q.pos.y = 0.0;
                q.prepare_player_to_spawn(&mut w);
                player(&mut out, &q);
            }
        }
        "creativegui" => {
            use super::{
                draw_trace10::{self, Call},
                screens::container::GuiContainer,
                Screen,
            };
            let mut ui = fixture_ui();
            ui.world_open = true;
            let mut r = JavaRandom::new(30039);
            for _ in 0..500 {
                let p = InventoryPlayer::default();
                let c = Container::new(ContainerKind::Creative, 0);
                let mut snap = c.snapshot(&p);
                let mut g = GuiContainer::new(&snap);
                g.base().width = 300 + r.next_int(1000);
                g.base().height = 240 + r.next_int(600);
                ui.container_snapshot = Some(snap.clone());
                g.init_gui(&mut ui);
                let (x, y) = g.origin();
                i(&mut out, x);
                i(&mut out, y);
                i(&mut out, g.base().buttons.len() as i32);
                let scroll = r.next_float();
                snap.contents[..72]
                    .copy_from_slice(&worldgen::container::creative::contents(scroll));
                ui.container_snapshot = Some(snap);
                g.check_set_scroll(scroll);
                g.base().width += 17;
                g.base().height += 23;
                ui.gl.begin_frame();
                draw_trace10::begin();
                g.draw_screen(&mut ui, -100, -100, 0.5);
                let rects: Vec<_> = draw_trace10::take()
                    .into_iter()
                    .filter_map(|c| {
                        if let Call::Rect(v, _) = c {
                            Some(v)
                        } else {
                            None
                        }
                    })
                    .collect();
                i(&mut out, rects.len() as i32);
                for rect in rects {
                    for v in rect {
                        i(&mut out, v);
                    }
                }
            }
        }
        "lifecycle" => {
            let mut r = JavaRandom::new(30040);
            for n in 0..200 {
                let blocks = shelves(&mut r);
                let mut c = Container::new(ContainerKind::Enchantment, 0);
                c.multiplayer = n % 3 == 0;
                c.bookshelves = shelf_count(&blocks);
                c.enchanting.as_mut().unwrap().rng = JavaRandom::new(n);
                let mut p = InventoryPlayer::default();
                for k in 0..12 {
                    if k % 3 == 0 {
                        c.slot_click(&mut p, 0, 0, false);
                        p.cursor = None;
                    } else {
                        let ids = [276, 278, 340, 1, 373, 359];
                        let id = ids[r.next_int(6) as usize];
                        c.put_stack(&mut p, 0, Some(ItemStack::new(id, 1 + r.next_int(5), 0)));
                    }
                    let e = c.enchanting.as_mut().unwrap();
                    enchstate(&mut out, e, 0);
                    out.extend(e.rng.next_long().to_le_bytes());
                }
            }
        }
        "lighting" => {
            use super::{
                achievement_toast::AchievementToast,
                screens::{achievements::GuiAchievements, stats::item_cell},
                Screen,
            };
            use crate::render::entity::gui_item;
            let mut ui = fixture_ui();
            ui.display_width = 854;
            ui.display_height = 480;
            let mut g = GuiAchievements::new();
            g.base().width = 427;
            g.base().height = 240;
            let mut toast = AchievementToast::default();
            for n in 0..100 {
                for k in 0..3 {
                    ui.gl.begin_frame();
                    gui_item::capture_lights();
                    match k {
                        0 => item_cell(&mut ui, 17, 23, 1),
                        1 => g.draw_screen(&mut ui, -1, -1, n as f32 / 100.0),
                        _ => {
                            ui.millis = 10000;
                            toast.queue_taken(&ui, 0);
                            ui.millis += 1500;
                            toast.draw(&mut ui);
                        }
                    }
                    let calls = gui_item::take_light_calls();
                    i(&mut out, calls.len() as i32);
                    for call in calls {
                        for op in call {
                            for v in op {
                                out.extend(v.to_le_bytes());
                            }
                        }
                    }
                }
            }
        }
        "hud" => {
            use super::{
                draw_trace10,
                ingame::{DebugInfo, GuiIngame, HudPlayer},
            };
            use crate::render::entity::render_release::{submit_boss_health, take_boss_plan};
            let mut ui = fixture_ui();
            ui.settings.gui_scale = 1;
            ui.settings.fancy_graphics = false;
            let mut r = JavaRandom::new(30038);
            let mut g = GuiIngame::new();
            for n in 0..1000 {
                ui.display_width = 320 + r.next_int(1000);
                ui.display_height = 240 + r.next_int(600);
                g.set_update_counter(r.next_int_unbounded());
                let mut p = HudPlayer::default();
                p.health = r.next_int(25) - 2;
                p.prev_health = r.next_int(25) - 2;
                p.hearts_life = r.next_int(40);
                p.armor = r.next_int(23);
                p.food_level = r.next_int(23);
                p.food_saturation = if r.next_bool() { 0.0 } else { 5.0 };
                p.xp_level = r.next_int(60) - 5;
                p.xp_points = r.next_float() * 1.5 - 0.2;
                p.current_item = r.next_int(9);
                p.air = r.next_int(340) - 20;
                p.head_in_water = r.next_bool();
                p.poison = r.next_bool();
                p.hunger = r.next_bool();
                p.regeneration = r.next_bool();
                p.survival = n % 4 != 0;
                ui.release.hardcore = n % 2 == 0;
                let health = r.next_int(251) - 25;
                if n % 3 == 0 {
                    submit_boss_health(health);
                }
                ui.gl.begin_frame();
                draw_trace10::begin();
                g.render_game_overlay(&mut ui, 0.5, false, &p, &DebugInfo::default());
                draw_trace10::write(&mut out, draw_trace10::take());
                out.extend(g.next_hud_random_long().to_le_bytes());
                out.push(
                    take_boss_plan(ui.scaled().width, ui.string_width("Boss health")).is_none()
                        as u8,
                );
            }
        }
        "containerdraw" => {
            use super::{draw_trace10, screens::container::GuiContainer, Screen};
            let mut ui = fixture_ui();
            ui.world_open = true;
            let mut r = JavaRandom::new(30037);
            for n in 0..400 {
                let mut p = InventoryPlayer::default();
                let mut c = Container::new(ContainerKind::Dispenser, 0);
                let w = 300 + r.next_int(1000);
                let h = 220 + r.next_int(600);
                let ids = [1, 264, 276, 373, 2256];
                for k in 0..45 {
                    if r.next_int(4) == 0 {
                        let id = ids[r.next_int(ids.len() as i32) as usize];
                        let stack = ItemStack::new(
                            id,
                            1 + r.next_int(5),
                            if id == 373 { 16 } else { r.next_int(5) },
                        );
                        if id == 276 {
                            stack.add_enchantment(16, 3);
                        }
                        c.put_stack(&mut p, k, Some(stack));
                    }
                }
                if n % 3 == 0 {
                    p.cursor = Some(ItemStack::new(264, 2, 0));
                }
                let slot = r.next_int(45) as usize;
                let hover = c.slots[slot];
                let mx = (w - 176) / 2 + hover.x + r.next_int(19) - 1;
                let my = (h - 166) / 2 + hover.y + r.next_int(19) - 1;
                let snap = c.snapshot(&p);
                let mut g = GuiContainer::new(&snap);
                g.base().width = w;
                g.base().height = h;
                ui.container_snapshot = Some(snap);
                g.init_gui(&mut ui);
                ui.gl.begin_frame();
                draw_trace10::begin();
                g.draw_screen(&mut ui, mx, my, 0.5);
                draw_trace10::write(&mut out, draw_trace10::take());
            }
        }
        "dynamic" => {
            use super::render_engine::{Engine, TextureEffect};
            use std::{
                cell::{Cell, RefCell},
                rc::Rc,
            };
            struct Fx {
                id: i32,
                icon: i32,
                tile: i32,
                frame: i32,
                image: Vec<u8>,
                calls: Rc<RefCell<Vec<[i32; 3]>>>,
                bound: Rc<Cell<i32>>,
            }
            impl TextureEffect for Fx {
                fn icon(&self) -> i32 {
                    self.icon
                }
                fn tile_size(&self) -> i32 {
                    self.tile
                }
                fn image(&self) -> &[u8] {
                    &self.image
                }
                fn tick(&mut self, anaglyph: bool) {
                    self.calls.borrow_mut().push([0, self.id, anaglyph as i32]);
                    self.frame += 1;
                    for (n, b) in self.image.iter_mut().enumerate() {
                        *b = (n as i32 * 37
                            + self.id * 13
                            + self.frame * 7
                            + if anaglyph { 91 } else { 0 }) as u8;
                    }
                }
                fn bind(&mut self) {
                    self.calls.borrow_mut().push([1, self.id, self.icon]);
                    self.bound.set(17 + self.id);
                }
            }
            let mut r = JavaRandom::new(30036);
            let calls = Rc::new(RefCell::new(Vec::new()));
            let bound = Rc::new(Cell::new(0));
            let mut engine = Engine::default();
            for n in 0..200 {
                let mut effects: Vec<_> = (0..8)
                    .map(|k| Fx {
                        id: k,
                        icon: if k < 3 {
                            19
                        } else if k == 3 {
                            -1
                        } else {
                            r.next_int(256)
                        },
                        tile: r.next_int(4),
                        frame: 0,
                        image: vec![0; 1024],
                        calls: calls.clone(),
                        bound: bound.clone(),
                    })
                    .collect();
                engine.anaglyph = n % 2 == 0;
                for _ in 0..3 {
                    calls.borrow_mut().clear();
                    let mut uploads = Vec::new();
                    engine.update_texture_fx(&mut effects, |x, y, image| {
                        uploads.push(([bound.get(), 0, x, y, 16, 16], image.to_vec()))
                    });
                    i(&mut out, calls.borrow().len() as i32);
                    for call in calls.borrow().iter() {
                        for v in call {
                            i(&mut out, *v);
                        }
                    }
                    i(&mut out, uploads.len() as i32);
                    for (args, bytes) in uploads {
                        for v in args {
                            i(&mut out, v);
                        }
                        i(&mut out, bytes.len() as i32);
                        out.extend(bytes);
                    }
                    i(&mut out, engine.buffer_position as i32);
                    i(&mut out, engine.buffer_limit as i32);
                }
            }
        }
        "winflow" => {
            use super::{
                screens::win_game::{GuiWinGame, WinDraw},
                Action, Screen,
            };
            let mut ui = fixture_ui();
            let mut r = JavaRandom::new(30035);
            let mut g = GuiWinGame::new("Player");
            g.lines = Some(vec![
                "[C]Minecraft 1.1".into(),
                "one two three four five".into(),
                "".into(),
                "\u{a7}a\u{a7}kXXXXXXXXXXX".into(),
                "[C]".into(),
                "last line".into(),
            ]);
            g.heights = vec![12, 24, 12, 12, 12, 12];
            g.total_height = 84;
            for n in 0..1000 {
                let w = 300 + r.next_int(800);
                let h = 200 + r.next_int(400);
                g.base().width = w;
                g.base().height = h;
                g.ticks = r.next_int(4000) - 1000;
                let partial = r.next_float();
                i(&mut out, 2);
                for rect in [
                    [w / 2 - 137, h + 50, 0, 0, 155, 44],
                    [w / 2 - 137 + 155, h + 50, 0, 45, 155, 44],
                ] {
                    for v in rect {
                        i(&mut out, v);
                    }
                }
                let plan = g.line_draws(partial, |text| ui.string_width(text));
                let mut ops = vec![[0.0, 0.0, -(g.ticks as f32 + partial) * 0.5, 0.0, 0.0]];
                let mut texts = Vec::new();
                for draw in plan {
                    match draw {
                        WinDraw::Translate(y) => ops.push([0.0, 0.0, y, 0.0, 0.0]),
                        WinDraw::Text {
                            text,
                            kind,
                            x,
                            y,
                            seed,
                        } => texts.push((text, kind, x, y, seed)),
                    }
                }
                i(&mut out, ops.len() as i32);
                for op in ops {
                    for v in op {
                        out.extend(v.to_le_bytes());
                    }
                }
                i(&mut out, texts.len() as i32);
                for (text, kind, x, y, seed) in texts {
                    s(&mut out, &text);
                    for v in [kind, x, y, if kind == 1 { 0 } else { 274 }, 16777215] {
                        i(&mut out, v);
                    }
                    out.extend(
                        (if kind == 1 {
                            0
                        } else {
                            JavaRandom::new(seed).next_long()
                        })
                        .to_le_bytes(),
                    );
                }
                i(&mut out, 8);
                ui.actions.clear();
                g.update_screen(&mut ui);
                g.key_typed(&mut ui, 'x', if n % 3 == 0 { 1 } else { 2 });
                i(&mut out, g.ticks);
                let count = ui
                    .actions
                    .iter()
                    .filter(|a| matches!(a, Action::EndReturn))
                    .count();
                i(&mut out, (count * 2) as i32);
                for _ in 0..count {
                    for v in [0, 0, 0, 0, 1, 0, 0, 1] {
                        i(&mut out, v);
                    }
                }
                out.push(g.does_gui_pause_game() as u8);
            }
        }
        "enchantmentdraw" => {
            use super::screens::enchantment::{book_inputs, offer_draws, EnchantmentAnimation};
            let mut ui = fixture_ui();
            let mut r = JavaRandom::new(30034);
            for n in 0..1000 {
                ui.display_width = 320 + r.next_int(1600);
                ui.display_height = 240 + r.next_int(1000);
                ui.settings.gui_scale = n % 4;
                let scaled = ui.scaled();
                let mut anim = EnchantmentAnimation::new(0);
                anim.previous_page = r.next_float() * 40.0 - 20.0;
                anim.page = r.next_float() * 40.0 - 20.0;
                anim.previous_opening = r.next_float() * 3.0 - 1.0;
                anim.opening = r.next_float() * 3.0 - 1.0;
                let partial = r.next_float();
                let seed = r.next_long();
                let costs = std::array::from_fn(|_| r.next_int(55) - 5);
                let level = r.next_int(55) - 5;
                let mx = r.next_int(scaled.width);
                let my = r.next_int(scaled.height);
                let (vp, ops, pose) = book_inputs(&anim, partial, scaled);
                for v in vp {
                    i(&mut out, v);
                }
                i(&mut out, ops.len() as i32);
                for op in ops {
                    for v in op {
                        out.extend(v.to_le_bytes());
                    }
                }
                for v in pose {
                    out.extend(v.to_le_bytes());
                }
                let x = (scaled.width - 176) / 2;
                let y = (scaled.height - 166) / 2;
                let (rects, mut texts) = offer_draws(seed, costs, level, x, y, mx, my, |text| {
                    ui.font.get_string_width(text)
                });
                i(&mut out, 1 + rects.len() as i32);
                for rect in std::iter::once([x, y, 0, 0, 176, 166]).chain(rects) {
                    for v in rect {
                        i(&mut out, v);
                    }
                }
                texts.push(("Enchant".into(), [0, 12, 6, 0, 4210752]));
                texts.push(("Inventory".into(), [0, 8, 72, 0, 4210752]));
                i(&mut out, texts.len() as i32);
                for (text, args) in texts {
                    s(&mut out, &text);
                    for v in args {
                        i(&mut out, v);
                    }
                }
            }
        }
        "enchanting" => {
            use worldgen::container::enchanting::Enchanting;
            let mut input = JavaRandom::new(30031);
            for n in 0..600 {
                let blocks = shelves(&mut input);
                let remote = n % 3 == 0;
                let mut level = input.next_int(100) - 10;
                let mut c = Enchanting::new(input.next_long());
                layout(&mut out, &Container::new(ContainerKind::Enchantment, 0));
                let ids = [276, 278, 306, 283, 340, 1, 261, 359];
                let id = ids[input.next_int(ids.len() as i32) as usize];
                c.item = Some(ItemStack::new(id, 1, 0));
                let count = shelf_count(&blocks);
                c.changed(remote, count);
                enchstate(&mut out, &c, level);
                for offer in 0..3 {
                    out.push(c.enchant(offer, &mut level, remote, count) as u8);
                    enchstate(&mut out, &c, level);
                }
                for k in -1..4 {
                    c.progress(k, k * 13 - 5);
                }
                enchstate(&mut out, &c, level);
                c.item = None;
                c.changed(remote, count);
                enchstate(&mut out, &c, level);
                out.extend(c.rng.next_long().to_le_bytes());
            }
        }
        "containers" => {
            let mut input = JavaRandom::new(30032);
            for kind in [ContainerKind::Enchantment, ContainerKind::BrewingStand] {
                for n in 0..160 {
                    let blocks = shelves(&mut input);
                    let mut p = InventoryPlayer::default();
                    let mut c = Container::new(kind, 0);
                    c.multiplayer = n % 3 == 0;
                    c.bookshelves = shelf_count(&blocks);
                    if let Some(e) = &mut c.enchanting {
                        e.rng = JavaRandom::new(n);
                    }
                    layout(&mut out, &c);
                    let ids = [1, 5, 264, 373, 374, 353, 276];
                    for k in 0..c.slots.len() {
                        if input.next_int(4) != 0 {
                            let id = ids[input.next_int(ids.len() as i32) as usize];
                            c.put_stack(
                                &mut p,
                                k,
                                Some(ItemStack::new(
                                    id,
                                    1 + input.next_int(70),
                                    input.next_int(3),
                                )),
                            );
                        }
                    }
                    state(&mut out, &mut c, &p);
                    for k in 0..c.slots.len() {
                        let result = c.transfer_stack(&mut p, k);
                        stack(&mut out, result);
                        state(&mut out, &mut c, &p);
                    }
                    p.cursor = Some(ItemStack::new(264, 3, 0));
                    let remote = c.multiplayer;
                    c.close(&mut p, remote);
                    state(&mut out, &mut c, &p);
                    c.close(&mut p, remote);
                    state(&mut out, &mut c, &p);
                }
            }
        }
        "creative" => {
            use worldgen::container::creative::catalog;
            let mut c = Container::new(ContainerKind::Creative, 0);
            let mut p = InventoryPlayer::default();
            i(&mut out, catalog().len() as i32);
            for s in catalog() {
                stack(&mut out, Some(*s));
            }
            layout(&mut out, &c);
            let mut input = JavaRandom::new(30033);
            for n in 0..500 {
                let v = match n {
                    0 => 0.0,
                    1 => 1.0,
                    _ => input.next_float() * 3.0 - 1.0,
                };
                c.scroll_creative(v);
                for k in 0..72 {
                    stack(&mut out, c.stack(&p, k));
                }
            }
            out.push(c.enchant_item(&mut 0, 0) as u8);
            stack(&mut out, c.transfer_stack(&mut p, 0));
        }
        "recipes" => {
            let manager = worldgen::container::recipes::CraftingManager::instance();
            i(&mut out, manager.recipes.len() as i32);
            for r in &manager.recipes {
                out.push(r.shaped as u8);
                i(&mut out, r.width as i32);
                i(&mut out, r.height as i32);
                stack(&mut out, Some(r.output));
                i(&mut out, r.ingredients.len() as i32);
                for s in &r.ingredients {
                    stack(&mut out, *s);
                }
                for width in 2..=3 {
                    for variant in 0..4 {
                        let mut grid = vec![None; width * width];
                        for (k, s) in r.ingredients.iter().enumerate() {
                            if let Some(mut s) = *s {
                                if s.damage == -1 {
                                    s.damage = variant as i32;
                                }
                                let index = if r.shaped {
                                    (if variant % 2 == 0 {
                                        k % r.width
                                    } else {
                                        r.width - k % r.width - 1
                                    }) + (k / r.width) * width
                                } else {
                                    k
                                };
                                if index < grid.len() {
                                    grid[index] = Some(s);
                                }
                            }
                        }
                        stack(&mut out, manager.find_matching_recipe(&grid, width));
                    }
                }
            }
            for id in -2..2260 {
                stack(&mut out, manager.smelting_result(id));
            }
            for id in [256, 257, 261, 276, 359, 1] {
                for a in [0, 1, 20, 100, 1500, i32::MAX, i32::MIN] {
                    for b in [0, 1, 20, 100, 1500, i32::MAX, i32::MIN] {
                        let mut grid = vec![None; 9];
                        grid[0] = Some(ItemStack::new(id, 1, a));
                        grid[8] = Some(ItemStack::new(id, 1, b));
                        stack(&mut out, manager.find_matching_recipe(&grid, 3));
                    }
                }
            }
        }
        "slots" => {
            use worldgen::entity::player::registry;
            let b = Container::new(ContainerKind::BrewingStand, 0);
            let e = Container::new(ContainerKind::Enchantment, 0);
            for s in [b.slots[0], b.slots[3], e.slots[0]] {
                i(&mut out, s.stack_limit());
                out.push((s.kind == worldgen::container::SlotKind::Enchantment) as u8);
                for id in 0..2258 {
                    if registry::item(id).is_some() {
                        out.push(s.is_item_valid(ItemStack::new(id, 1, 0)) as u8);
                    }
                }
            }
            for damage in [-1, 0, 1, 16, 16384, 32767] {
                let mut p = InventoryPlayer::default();
                let mut c = Container::new(ContainerKind::BrewingStand, 0);
                c.put_stack(&mut p, 0, Some(ItemStack::new(373, 1, damage)));
                c.slot_click(&mut p, 0, 0, false);
                achievements(&mut out, &c.events);
            }
            for id in [58, 270, 61, 290, 297, 354, 274, 268, 116, 47, 264] {
                let mut p = InventoryPlayer::default();
                let mut c = Container::new(ContainerKind::Workbench, 0);
                c.result = Some(ItemStack::new(id, 1, 0));
                c.slot_click(&mut p, 0, 0, false);
                achievements(&mut out, &c.events);
            }
        }
        _ => panic!("unknown GUI round-three mode {mode}"),
    }
    std::fs::create_dir_all("target/r10gui3").unwrap();
    std::fs::write(format!("target/r10gui3/{mode}-rust.bin"), &out).unwrap();
    let expected = std::fs::read(path).expect("original bytecode fixture");
    if out == expected {
        println!("R10GUI3 {mode} EXACT ({} bytes)", out.len());
        true
    } else {
        let at = out
            .iter()
            .zip(&expected)
            .position(|(a, b)| a != b)
            .unwrap_or(out.len().min(expected.len()));
        eprintln!(
            "R10GUI3 {mode} DIFFERS at {at}, lengths {}/{}",
            out.len(),
            expected.len()
        );
        false
    }
}
