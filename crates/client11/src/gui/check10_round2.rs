//! Round-two comparisons with the unmodified Release 1.0 jar.
use super::{screens, settings::*, utility_check::fixture_ui, Screen};
const TEXT: &[&str] = &[
    "",
    " ",
    "   ",
    "Minecraft 1.1.0",
    "one two three four five",
    "one\n\ntwo\n",
    "\n",
    "\n\n",
    "longunbrokentextwithoutspaces",
    "\u{a7}aGreen \u{a7}rReset",
    "\u{a7}kObfuscated \u{a7}bblue \u{a7}kagain",
    "\u{a7}KUPPER \u{a7}zbad",
    "1234567890 !?",
    "éßñå",
    "  leading trailing  ",
    "\u{a7}a\u{a7}kXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
];
pub(crate) fn i(o: &mut Vec<u8>, n: i32) {
    o.extend(n.to_le_bytes());
}
pub(crate) fn s(o: &mut Vec<u8>, text: &str) {
    i(o, text.encode_utf16().count() as i32);
    for n in text.encode_utf16() {
        o.extend(n.to_le_bytes());
    }
}
pub(crate) fn buttons(o: &mut Vec<u8>, g: &mut dyn Screen) {
    let buttons = &g.base().buttons;
    i(o, buttons.len() as i32);
    for b in buttons {
        for n in [b.id, b.x, b.y, b.width, b.height] {
            i(o, n);
        }
        o.push(b.enabled as u8);
        o.push(b.visible as u8);
        s(o, &b.text);
    }
}
pub fn check(mode: &str, path: &str) -> bool {
    if mode == "menustate" {
        return screens::main_menu::check(path);
    }
    let mut out = Vec::new();
    let mut ui = fixture_ui("target".into());
    match mode {
        "font" => {
            for height in [8, 12, 1, 19] {
                for text in TEXT {
                    for width in [10, 11, 20, 40, 80, 104, 274] {
                        for shadow in [false, true] {
                            ui.font.line_height = height;
                            let lines = ui.font.split_draw_lines(text, 13, width, shadow);
                            i(&mut out, ui.font.get_string_width(text));
                            i(&mut out, ui.font.split_string_height(text, width));
                            i(&mut out, lines.len() as i32);
                            for (text, y, shadow) in lines {
                                s(&mut out, &text);
                                i(&mut out, y);
                                out.push(shadow as u8);
                            }
                        }
                    }
                }
            }
            let mut random = worldgen::JavaRandom::new(10102);
            for n in 0..2000 {
                let text = TEXT[n % TEXT.len()];
                let seed = random.next_long();
                let shadow = random.next_bool();
                ui.font.random.lock().unwrap().set_seed(seed);
                let commands = ui.font.display_list_commands(text, shadow);
                i(&mut out, commands.len() as i32);
                for command in commands {
                    i(&mut out, command);
                }
                out.extend(ui.font.random.lock().unwrap().next_long().to_le_bytes());
            }
        }
        "enchantment" => {
            use worldgen::container::ItemStack;
            let mut animation = screens::enchantment::EnchantmentAnimation::new(10203);
            let mut inputs = worldgen::JavaRandom::new(20103);
            let mut stack: Option<ItemStack> = None;
            for _ in 0..10000 {
                let op = inputs.next_int(5);
                match op {
                    0 => stack = None,
                    1 => stack = Some(ItemStack::new(276, 1, inputs.next_int(20))),
                    2 => {
                        if let Some(old) = stack {
                            stack = Some(old.copy_stack181());
                        }
                    }
                    3 => {
                        if let Some(old) = stack.as_mut() {
                            old.damage = inputs.next_int(20);
                        }
                    }
                    4 => {
                        if let Some(old) = stack {
                            old.add_enchantment(16, inputs.next_int(5) + 1);
                        }
                    }
                    _ => {}
                }
                let costs = std::array::from_fn(|_| {
                    if inputs.next_int(4) == 0 {
                        0
                    } else {
                        inputs.next_int(50) - 10
                    }
                });
                animation.tick(stack, costs);
                i(&mut out, animation.ticks);
                for value in [
                    animation.page,
                    animation.previous_page,
                    animation.target,
                    animation.velocity,
                    animation.opening,
                    animation.previous_opening,
                ] {
                    out.extend(value.to_le_bytes());
                }
            }
            out.extend(animation.random.next_long().to_le_bytes());
            for _ in 0..1000 {
                for name in screens::enchantment::names(inputs.next_long()) {
                    s(&mut out, &name);
                }
            }
        }
        "brewing" => {
            let mut random = worldgen::JavaRandom::new(10304);
            for time in -10..=1200 {
                let width = random.next_int(1800);
                let height = random.next_int(1200);
                let rects =
                    screens::brewing::rectangles(time, (width - 176) / 2, (height - 166) / 2);
                i(&mut out, rects.len() as i32);
                for rect in rects {
                    for n in rect {
                        i(&mut out, n);
                    }
                }
            }
        }
        "registry" => {
            for x in super::stats::ACHIEVEMENTS {
                for n in [
                    x.id,
                    x.column,
                    x.row,
                    x.icon.0 as i32,
                    x.parent.map_or(-1, |p| super::stats::ACHIEVEMENTS[p].id),
                ] {
                    i(&mut out, n);
                }
                out.push(x.special as u8);
                s(&mut out, &ui.translate(&format!("achievement.{}", x.key)));
                s(
                    &mut out,
                    &ui.translate(&format!("achievement.{}.desc", x.key)),
                );
            }
            let (a, b, c, d) = super::stats::display_bounds();
            for n in [a, b, c, d] {
                i(&mut out, n);
            }
            for e in EnumOptions::ALL {
                i(&mut out, e.ordinal());
                s(&mut out, e.key());
                out.push(e.is_float() as u8);
                out.push(e.is_bool() as u8);
                s(&mut out, &ui.settings.get_key_binding(&ui.lang, e));
            }
            for distance in -2..6 {
                for clouds in [false, true] {
                    ui.settings.render_distance = distance;
                    ui.settings.clouds = clouds;
                    out.push(ui.settings.should_render_clouds() as u8);
                }
            }
        }
        "inventory" => {
            use worldgen::container::*;
            let mut random = worldgen::JavaRandom::new(10405);
            let snapshot =
                Container::new(ContainerKind::Player, 0).snapshot(&InventoryPlayer::default());
            ui.container_snapshot = Some(snapshot.clone());
            for c in 0..1000 {
                let width = 320 + random.next_int(1000);
                let height = 240 + random.next_int(1000);
                ui.release.effects = (0..c % 20)
                    .map(|n| worldgen::entity::potion::PotionEffect {
                        id: n + 1,
                        duration: random.next_int(144001) - 1200,
                        amplifier: random.next_int(6) - 1,
                    })
                    .collect();
                let mut screen = screens::container::GuiContainer::new(&snapshot);
                screen.base().width = width;
                screen.base().height = height;
                screen.init_gui(&mut ui);
                let (x, y) = screen.origin();
                i(&mut out, x);
                i(&mut out, y);
                let panels = screens::potion_effects::panels(&ui, x - 124, y, &ui.release.effects);
                i(
                    &mut out,
                    panels.iter().map(|p| p.rectangles.len()).sum::<usize>() as i32,
                );
                for panel in &panels {
                    for rect in &panel.rectangles {
                        for n in rect {
                            i(&mut out, *n);
                        }
                    }
                }
                i(&mut out, panels.len() as i32 * 2);
                for panel in panels {
                    for (text, x, y, color) in panel.text {
                        s(&mut out, &text);
                        for n in [x, y, color] {
                            i(&mut out, n);
                        }
                    }
                }
            }
        }
        "gradient" => {
            let mut random = worldgen::JavaRandom::new(10506);
            for _ in 0..1000 {
                let x = random.next_int(2000) - 1000;
                let y = random.next_int(2000) - 1000;
                let w = random.next_int(1000) - 500;
                let h = random.next_int(1000) - 500;
                let top = random.next_int_unbounded();
                let bottom = random.next_int_unbounded();
                let z = (random.next_int(2000) - 1000) as f32 / 7.0;
                ui.gl.begin_frame();
                ui.draw_gradient_rect_z(x, y, x + w, y + h, top, bottom, z);
                for n in [0, 1, 2, 5] {
                    let v = ui.gl.verts[n];
                    for f in v.pos.into_iter().chain(v.color) {
                        out.extend(f.to_le_bytes());
                    }
                }
            }
        }
        "backgrounds" => screens::container::check_backgrounds10(&mut out),
        "settings" => {
            i(&mut out, ui.settings.third_person_view);
            let mut random = worldgen::JavaRandom::new(10708);
            for c in 0..1000 {
                ui.settings.particles = random.next_int_unbounded();
                ui.settings.clouds = random.next_bool();
                let delta = random.next_int_unbounded();
                ui.settings.set_option_value(EnumOptions::Particles, delta);
                ui.settings
                    .set_option_value(EnumOptions::RenderClouds, delta);
                i(&mut out, ui.settings.particles);
                out.push(ui.settings.clouds as u8);
                for e in EnumOptions::ALL {
                    out.push(ui.settings.get_option_ordinal_value(e) as u8);
                }
                ui.settings.particles = c % 3;
                s(
                    &mut out,
                    &ui.settings
                        .get_key_binding(&ui.lang, EnumOptions::Particles),
                );
                s(
                    &mut out,
                    &ui.settings
                        .get_key_binding(&ui.lang, EnumOptions::RenderClouds),
                );
            }
            let dir = std::path::Path::new("target/r10gui2/settings-rust");
            std::fs::create_dir_all(dir).unwrap();
            for c in 0..32 {
                std::fs::write(
                    dir.join("options.txt"),
                    format!("particles:{}\nclouds:{}\n", c - 4, c % 2 == 0),
                )
                .unwrap();
                let loaded = GameSettings::load(dir);
                i(&mut out, loaded.particles);
                out.push(loaded.clouds as u8);
                loaded.save();
                s(
                    &mut out,
                    &std::fs::read_to_string(dir.join("options.txt"))
                        .unwrap()
                        .replace("\r\n", "\n"),
                );
            }
        }
        "statformats" => {
            use super::stats_table::StatType;
            let mut random = worldgen::JavaRandom::new(10809);
            for c in 0..5000 {
                let value = match c {
                    0 => i32::MIN,
                    1 => i32::MAX,
                    2 => 0,
                    _ => random.next_int_unbounded(),
                };
                for kind in [StatType::Simple, StatType::Time, StatType::Distance] {
                    s(&mut out, &screens::stats::format_stat(kind, value));
                }
            }
        }
        "hardcore" => screens::select_world::check_hardcore10(&mut ui, &mut out),
        "slot" => super::slot::GuiSlot::check_wheel10(&mut ui, &mut out),
        "disconnected" => {
            let mut random = worldgen::JavaRandom::new(11112);
            for c in 0..1000 {
                let message = if c % 2 == 0 {
                    "disconnect.genericReason"
                } else {
                    "missing.key"
                };
                let detail = if c % 3 == 0 {
                    "disconnect.genericReason"
                } else {
                    "disconnect.timeout"
                };
                let arguments = if c % 2 == 0 {
                    Some(vec!["test reason".to_string()])
                } else {
                    None
                };
                let mut g = screens::disconnected::GuiDisconnected::new(
                    &ui,
                    message,
                    detail,
                    arguments.as_deref(),
                );
                g.base().width = random.next_int(1800);
                g.base().height = random.next_int(1200);
                g.init_gui(&mut ui);
                buttons(&mut out, &mut g);
                let labels = g.labels();
                i(&mut out, labels.len() as i32);
                for (text, x, y, color) in labels {
                    s(&mut out, &text);
                    for n in [x, y, color] {
                        i(&mut out, n);
                    }
                }
                ui.actions.clear();
                g.key_typed(&mut ui, 'x', 1);
                i(&mut out, ui.actions.len() as i32);
                for id in [-1, 0, 1] {
                    ui.actions.clear();
                    g.action_performed(&mut ui, id);
                    i(&mut out, ui.actions.len() as i32);
                }
            }
        }
        "win" => {
            let mut g = screens::win_game::GuiWinGame::new("Player_10");
            g.base().width = 427;
            g.base().height = 240;
            g.init_gui(&mut ui);
            let lines = g.lines.as_ref().unwrap();
            i(&mut out, lines.len() as i32);
            for (line, height) in lines.iter().zip(&g.heights) {
                s(&mut out, line);
                i(&mut out, *height);
            }
            i(&mut out, g.total_height);
            i(&mut out, ui.font.line_height);
            out.push(g.does_gui_pause_game() as u8);
            g.base().height = 360;
            g.init_gui(&mut ui);
            i(&mut out, g.total_height);
            ui.textures.insert("%blur%/gui/background.png".into(), 0);
            let mut random = worldgen::JavaRandom::new(11213);
            for _ in 0..1000 {
                g.ticks = random.next_int(100000) - 1000;
                let partial = random.next_float();
                ui.gl.begin_frame();
                g.render_background(&mut ui, partial);
                for n in [0, 1, 2, 5] {
                    let v = ui.gl.verts[n];
                    for f in v.pos.into_iter().chain(v.uv).chain(v.color) {
                        out.extend(f.to_le_bytes());
                    }
                }
            }
        }
        "menudraw" => screens::main_menu::GuiMainMenu::check_decoration10(&mut ui, &mut out),
        "video" => {
            let mut random = worldgen::JavaRandom::new(11314);
            for c in 0..300 {
                ui.settings = GameSettings::default();
                ui.settings.gui_scale = c % 4;
                ui.settings.render_distance = c % 3;
                let mut g = screens::options::GuiVideoSettings::new(Box::new(
                    screens::main_menu::GuiMainMenu::with_splash("fixture"),
                ));
                g.base().width = 320 + random.next_int(1000);
                g.base().height = 240 + random.next_int(1000);
                g.runtime_64_bit = c % 2 == 0;
                g.init_gui(&mut ui);
                for id in [15, 16, 14, 200] {
                    g.action_performed(&mut ui, id);
                    for n in [
                        g.base().width,
                        g.base().height,
                        ui.settings.gui_scale,
                        ui.settings.particles,
                    ] {
                        i(&mut out, n);
                    }
                    out.push(ui.settings.clouds as u8);
                    buttons(&mut out, &mut g);
                    let labels = g.labels(&ui);
                    i(&mut out, labels.len() as i32);
                    for (text, x, y, color) in labels {
                        s(&mut out, &text);
                        for n in [x, y, color] {
                            i(&mut out, n);
                        }
                    }
                    ui.actions.clear();
                }
            }
        }
        "itemnames" => {
            let lines: Vec<_> = include_str!("screens/item_names.tsv").lines().collect();
            i(&mut out, lines.len() as i32);
            for line in lines {
                let mut parts = line.split('\t');
                i(&mut out, parts.next().unwrap().parse().unwrap());
                i(&mut out, parts.next().unwrap().parse().unwrap());
                s(&mut out, parts.next().unwrap());
            }
        }
        "boss" => {
            let mut world = worldgen::World::new(1, 1);
            let mut dragon = worldgen::entity::Entity::living(&mut world, "EnderDragon").unwrap();
            use crate::render::entity::render_release::{submit_boss_health, take_boss_plan};
            let mut random = worldgen::JavaRandom::new(11415);
            for _ in 0..1000 {
                let width = 320 + random.next_int(1000);
                dragon
                    .data_watcher
                    .update(
                        16,
                        worldgen::entity::data_watcher::Value::Int(random.next_int(251) - 25),
                    )
                    .unwrap();
                submit_boss_health(dragon.data_watcher.int(16).unwrap());
                ui.settings.gui_scale=1;ui.display_width=width;ui.display_height=480;
                ui.textures.insert("gui/icons.png".into(),0);
                ui.gl.begin_frame();super::draw_trace10::begin();
                crate::render::entity::render_release::boss_overlay(&mut ui,-90.0);
                let calls=super::draw_trace10::take();
                let rects:Vec<_>=calls.iter().filter_map(|c|if let super::draw_trace10::Call::Rect(v,_)=c{Some(*v)}else{None}).collect();
                i(&mut out,rects.len() as i32);for rect in rects{for v in rect{i(&mut out,v);}}
                out.push(take_boss_plan(width,ui.string_width("Boss health")).is_none() as u8);
                let texts:Vec<_>=calls.iter().filter_map(|c|if let super::draw_trace10::Call::Text(text,[_,x,y,_,color])=c{Some((text,*x,*y,*color))}else{None}).collect();
                i(&mut out,texts.len() as i32);for (text,x,y,color) in texts{s(&mut out,text);for v in [x,y,color]{i(&mut out,v);}}
            }
        }
        "gameover" => {
            let mut random = worldgen::JavaRandom::new(11516);
            for c in 0..1000 {
                ui.release.hardcore = c % 2 == 0;
                ui.release.session_missing = c % 3 == 0;
                ui.player_score = random.next_int_unbounded();
                let mut g = screens::game_over::GuiGameOver::new();
                g.base().width = 320 + random.next_int(1000);
                g.base().height = 240 + random.next_int(1000);
                g.init_gui(&mut ui);
                buttons(&mut out, &mut g);
                let labels = g.labels(&ui);
                i(&mut out, labels.len() as i32);
                for (text, x, y, color, doubled) in labels {
                    s(&mut out, &text);
                    for n in [x, y, color] {
                        i(&mut out, n);
                    }
                    out.push(doubled as u8);
                }
                out.push(g.does_gui_pause_game() as u8);
                ui.actions.clear();
                g.key_typed(&mut ui, 'x', 1);
                i(&mut out, ui.actions.len() as i32);
                for id in [-1, 0, 1, 2] {
                    ui.actions.clear();
                    g.action_performed(&mut ui, id);
                    let action = ui
                        .actions
                        .iter()
                        .find_map(|a| match a {
                            super::Action::Respawn => Some(1),
                            super::Action::QuitWorld => Some(2),
                            super::Action::DeleteHardcoreWorld => Some(3),
                            _ => None,
                        })
                        .unwrap_or(0);
                    i(&mut out, action);
                }
            }
        }
        "worldslot" => {
            let mut random = worldgen::JavaRandom::new(11617);
            for c in 0..1000 {
                let time = random.next_int(2000000000) as i64;
                let size = random.next_long();
                let x = random.next_int(1800) - 200;
                let y = random.next_int(1200) - 100;
                let n = c % 4;
                let save = super::saves::SaveEntry {
                    file_name_utf16: None,
                    display_name_utf16: None,
                    file_name: format!("World_{n}"),
                    display_name: if n % 2 == 0 {
                        String::new()
                    } else {
                        format!("Display {n}")
                    },
                    last_played: time,
                    size_on_disk: size,
                    game_mode: n % 2,
                    needs_conversion: c % 3 == 0,
                    hardcore: c % 2 == 0,
                };
                let labels = screens::select_world::world_slot_labels(
                    &ui,
                    &save,
                    n,
                    x,
                    y,
                    &ui.translate("selectWorld.world"),
                    &ui.translate("selectWorld.conversion"),
                    &[
                        ui.translate("gameMode.survival"),
                        ui.translate("gameMode.creative"),
                    ],
                );
                i(&mut out, labels.len() as i32);
                for (text, x, y, color) in labels {
                    s(&mut out, &text);
                    for n in [x, y, color] {
                        i(&mut out, n);
                    }
                }
            }
        }
        _ => panic!("unknown GUI round-two mode {mode}"),
    }
    std::fs::write(format!("target/r10gui2/{mode}-rust.bin"), &out).unwrap();
    let expected = std::fs::read(path).expect("original bytecode output");
    if out == expected {
        println!("R10GUI2 {mode} EXACT ({} bytes)", out.len());
        true
    } else {
        let at = out
            .iter()
            .zip(&expected)
            .position(|(a, b)| a != b)
            .unwrap_or(out.len().min(expected.len()));
        eprintln!(
            "R10GUI2 {mode} DIFFERS at {at}, lengths {}/{}",
            out.len(),
            expected.len()
        );
        false
    }
}
