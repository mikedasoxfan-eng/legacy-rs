use super::*;
use std::{cell::RefCell, rc::Rc};
fn int(out: &mut Vec<u8>, n: i32) {
    out.extend(n.to_le_bytes());
}
fn string(out: &mut Vec<u8>, s: &str) {
    let units: Vec<_> = s.encode_utf16().collect();
    int(out, units.len() as i32);
    for n in units {
        out.extend(n.to_le_bytes());
    }
}
fn buttons(out: &mut Vec<u8>, base: &ScreenBase) {
    int(out, base.buttons.len() as i32);
    for b in &base.buttons {
        for n in [b.id, b.x, b.y, b.width, b.height] {
            int(out, n);
        }
        out.push(b.enabled as u8);
        out.push(b.visible as u8);
        string(out, &b.text);
    }
}
fn compare(label: &str, path: &str, out: Vec<u8>) -> bool {
    std::fs::write(format!("target/{label}-rust.bin"), &out).unwrap();
    let expected = std::fs::read(path).unwrap();
    if out != expected {
        let at = out
            .iter()
            .zip(&expected)
            .position(|(a, b)| a != b)
            .unwrap_or(out.len().min(expected.len()));
        eprintln!(
            "{label} DIFFERS byte {at}, lengths {}/{}",
            out.len(),
            expected.len()
        );
        false
    } else {
        println!("{label} EXACT ({} bytes)", out.len());
        true
    }
}
#[derive(Default)]
struct Parent {
    base: ScreenBase,
    calls: Rc<RefCell<Vec<(bool, i32)>>>,
    returns: bool,
}
impl Screen for Parent {
    fn base(&mut self) -> &mut ScreenBase {
        &mut self.base
    }
    fn delete_world(&mut self, _: &mut Ui, yes: bool, n: i32) -> bool {
        self.calls.borrow_mut().push((yes, n));
        self.returns
    }
}
pub fn check_yes_no(path: &str) -> bool {
    let mut ui = crate::gui::utility_check::fixture_ui("target/yesno8-saves".into());
    let mut out = Vec::new();
    let mut r = worldgen::JavaRandom::new(8182);
    for c in 0..4096 {
        let w = r.next_int(1800);
        let h = r.next_int(1200);
        let number = r.next_int_unbounded();
        let id = if c % 3 == 0 {
            0
        } else if c % 3 == 1 {
            1
        } else {
            r.next_int_unbounded()
        };
        let calls = Rc::new(RefCell::new(Vec::new()));
        let p = Parent {
            calls: calls.clone(),
            returns: c % 2 == 0,
            ..Default::default()
        };
        let mut g = GuiYesNo::new(
            Box::new(p),
            "question".into(),
            "warning".into(),
            "Yes é".into(),
            "No".into(),
            number,
        );
        g.base.width = w;
        g.base.height = h;
        g.init_gui(&mut ui);
        if c % 4 == 0 {
            g.init_gui(&mut ui);
        }
        buttons(&mut out, &g.base);
        g.action_performed(&mut ui, id);
        let calls = calls.borrow();
        int(&mut out, calls.len() as i32);
        for &(yes, n) in calls.iter() {
            out.push(yes as u8);
            int(&mut out, n);
        }
        int(&mut out, ui.actions.len() as i32);
        for action in ui.actions.drain(..) {
            out.push(matches!(action, Action::Display(Some(_))) as u8);
        }
        g.key_typed(&mut ui, '\0', if c % 2 == 0 { KEY_ESCAPE } else { 28 });
        int(&mut out, ui.actions.len() as i32);
        for action in ui.actions.drain(..) {
            out.push(matches!(action, Action::Display(Some(_))) as u8);
        }
    }
    compare("yesno8_181", path, out)
}
pub fn check_rename(path: &str) -> bool {
    let root = std::path::PathBuf::from("target/rename8-saves");
    let dir = root.join("Fixture");
    std::fs::create_dir_all(&dir).unwrap();
    let mut ui = crate::gui::utility_check::fixture_ui(root);
    let names = [
        "World",
        "",
        "   ",
        "  trim  ",
        "\tWorld\r",
        "\u{a0}",
        "\u{2003}",
        "café",
        "0123456789012345678901234567890123456789",
    ];
    let mut out = Vec::new();
    for c in 0..360 {
        let mut data = worldgen::nbt::Compound::new();
        data.put(
            "LevelName",
            worldgen::nbt::Tag::String(names[c % names.len()].into()),
        );
        let mut root = worldgen::nbt::Compound::new();
        root.put("Data", worldgen::nbt::Tag::Compound(data));
        std::fs::write(
            dir.join("level.dat"),
            worldgen::nbt::write_gzip(&root).unwrap(),
        )
        .unwrap();
        let mut g = GuiRenameWorld::new(Box::new(Parent::default()), "Fixture".into());
        g.base.width = 427;
        g.base.height = 240;
        g.base.buttons.push(GuiButton::new(17, 0, 0, "dirty"));
        g.init_gui(&mut ui);
        buttons(&mut out, &g.base);
        out.push(g.repeats_keys() as u8);
        for _ in 0..c % 30 {
            g.update_screen(&mut ui);
        }
        let (x, y) = if c % 4 == 0 {
            (112, 60)
        } else {
            (113 + c as i32 % 200, 60)
        };
        g.mouse_clicked(&mut ui, x, y, if c % 3 == 0 { 1 } else { 0 });
        for (ch, key) in [('x', 45), ('\0', 14), ('\t', 15), (' ', 57), ('\0', 1)] {
            g.key_typed(&mut ui, ch, key);
        }
        string(&mut out, &g.field.text);
        out.push(g.field.focused as u8);
        out.push(g.base.buttons[0].enabled as u8);
        int(&mut out, g.field.max_length as i32);
        if c % 5 == 0 {
            g.base.buttons[0].enabled = false;
        }
        match c % 4 {
            0 => g.key_typed(&mut ui, '\r', 28),
            1 => g.action_performed(&mut ui, 0),
            2 => g.action_performed(&mut ui, 1),
            _ => g.action_performed(&mut ui, 17),
        }
        string(
            &mut out,
            &saves::world_info(&ui.saves_dir, "Fixture")
                .unwrap()
                .string("LevelName"),
        );
        int(&mut out, ui.actions.len() as i32);
        for action in ui.actions.drain(..) {
            out.push(matches!(action, Action::Display(Some(_))) as u8);
        }
    }
    let mut missing = GuiRenameWorld::new(Box::new(Parent::default()), "Missing".into());
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let failed =
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| missing.init_gui(&mut ui)))
            .is_err();
    std::panic::set_hook(hook);
    out.push(failed as u8);
    buttons(&mut out, &missing.base);
    compare("rename8_181", path, out)
}

pub fn check_select(path: &str) -> bool {
    fn fixture(root: &std::path::Path) {
        for (i, folder) in ["A", "B", "C"].iter().enumerate() {
            let dir = root.join(folder);
            std::fs::create_dir_all(&dir).unwrap();
            let mut data = worldgen::nbt::Compound::new();
            data.put(
                "LevelName",
                worldgen::nbt::Tag::String(format!("Name {folder}")),
            );
            data.put("LastPlayed", worldgen::nbt::Tag::Long((2 - i) as i64));
            data.put("version", worldgen::nbt::Tag::Int(19132));
            let mut nbt = worldgen::nbt::Compound::new();
            nbt.put("Data", worldgen::nbt::Tag::Compound(data));
            std::fs::write(
                dir.join("level.dat"),
                worldgen::nbt::write_gzip(&nbt).unwrap(),
            )
            .unwrap();
        }
    }
    let root = std::path::PathBuf::from("target/select8-saves");
    fixture(&root);
    let mut ui = crate::gui::utility_check::fixture_ui(root.clone());
    let mut out = Vec::new();
    let mut r = worldgen::JavaRandom::new(8183);
    let mut world = worldgen::entity::verify181::world();
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    for c in 0..2048 {
        let game_type = match c % 8 {
            0 => 0,
            1 => 1,
            2 => -1,
            3 => 2,
            4 => i32::MIN,
            5 => i32::MAX,
            _ => r.next_int_unbounded(),
        };
        let index = c % 5 - 1;
        let mut g = GuiSelectWorld::new(Box::new(Parent::default()));
        int(&mut out, g.selected_world);
        g.base.width = 427;
        g.base.height = 240;
        g.init_gui(&mut ui);
        for label in [
            &g.title,
            &g.world_label,
            &g.conversion_label,
            &g.game_mode_labels[0],
            &g.game_mode_labels[1],
        ] {
            string(&mut out, label);
        }
        int(&mut out, g.selected_world);
        buttons(&mut out, &g.base);
        for s in &mut g.saves {
            s.game_mode = game_type;
            if c % 2 == 0 {
                s.display_name.clear();
            }
        }
        let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            g.select_world(&mut ui, index)
        }))
        .is_err();
        out.push(failed as u8);
        out.push(g.selected as u8);
        int(&mut out, ui.actions.len() as i32);
        for action in ui.actions.drain(..) {
            match action {
                Action::Display(None) => out.push(0),
                Action::StartWorld {
                    folder,
                    name,
                    seed,
                    settings,
                    ..
                } => {
                    out.push(2);
                    string(&mut out, &folder);
                    string(&mut out, &name);
                    out.push(worldgen::entity::player::controller::PlayerControllerSP::saved_world_is_creative(game_type) as u8);
                    out.push((seed.is_none() && settings.is_none()) as u8);
                    let mut player =
                        worldgen::entity::player::controller::PlayerControllerSP::create_player(
                            &mut world, "Player",
                        );
                    worldgen::entity::player::controller::PlayerControllerSP::flip_player(&mut player,
                        worldgen::entity::player::controller::PlayerControllerSP::saved_world_is_creative(game_type));
                    out.extend(player.yaw.to_le_bytes());
                }
                _ => panic!("unexpected select action"),
            }
        }
        g.select_world(&mut ui, -77);
        int(&mut out, ui.actions.len() as i32);
        for action in ui.actions.drain(..) {
            out.push(matches!(action, Action::Display(Some(_))) as u8);
        }
    }
    for c in 0..96 {
        fixture(&root);
        let mut g = GuiSelectWorld::new(Box::new(Parent::default()));
        g.base.width = 427;
        g.base.height = 240;
        g.init_gui(&mut ui);
        g.selected_world = 1;
        g.deleting = c % 3 != 0;
        let handled = g.delete_world(&mut ui, c % 2 == 0, 1);
        // GuiYesNo returns this parent exactly when its callback requests redisplay.
        int(&mut out, handled as i32);
        out.push(g.deleting as u8);
        int(&mut out, g.selected_world);
        int(&mut out, g.saves.len() as i32);
        for s in &g.saves {
            string(&mut out, &s.file_name);
        }
    }
    std::panic::set_hook(hook);
    compare("select8_181", path, out)
}
