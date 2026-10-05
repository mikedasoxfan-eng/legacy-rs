use super::*;
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
pub fn check(path: &str) -> bool {
    let mut ui = crate::gui::utility_check::fixture_ui("target".into());
    let mut jar = crate::assets::Jar::open().unwrap();
    ui.splashes = jar
        .text("title/splashes.txt")
        .unwrap()
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    let mut random = worldgen::JavaRandom::new(18110);
    let mut out = Vec::new();
    for _ in 0..1000 {
        string(&mut out, &GuiMainMenu::new(&ui, &mut random).splash_text);
    }
    out.extend(random.next_long().to_be_bytes());
    let mut r = worldgen::JavaRandom::new(18110);
    for c in 0..10000 {
        let mut screen = GuiMainMenu::with_splash("Ported to Rust!");
        screen.base.width = r.next_int(1800);
        screen.base.height = r.next_int(1200);
        ui.hide_quit_button = r.next_bool();
        screen.init_gui(&mut ui);
        int(&mut out, screen.base.buttons.len() as i32);
        for b in &screen.base.buttons {
            for n in [b.id, b.x, b.y, b.width, b.height] {
                int(&mut out, n);
            }
            out.push(b.enabled as u8);
            out.push(b.visible as u8);
            string(&mut out, &b.text);
        }
        screen.update_counter = match c % 4 {
            0 => 16777216.0,
            1 => -16777216.0,
            _ => r.next_int_unbounded() as f32,
        };
        screen.panorama_ticks = r.next_int_unbounded();
        for _ in 0..4 {
            screen.update_screen(&mut ui);
            out.extend(screen.update_counter.to_le_bytes());
            int(&mut out, screen.panorama_ticks);
        }
        let ch = r.next_int(65536) as u32;
        let key = r.next_int_unbounded();
        screen.key_typed(&mut ui, char::from_u32(ch).unwrap_or('\0'), key);
        int(&mut out, ui.actions.len() as i32);
        int(&mut out, 0);
        out.push(screen.does_gui_pause_game() as u8);
        ui.actions.clear();
        for id in [-1, 0, 1, 3, 4, 5] {
            screen.action_performed(&mut ui, id);
            let (mut displays, mut quits) = (0, 0);
            for a in std::mem::take(&mut ui.actions) {
                match a {
                    Action::DisplayChild(_) => displays += 1,
                    Action::Shutdown => quits += 1,
                    _ => panic!("unexpected title callback"),
                }
            }
            int(&mut out, displays);
            int(&mut out, quits);
        }
    }
    std::fs::write("target/mainmenu10_181-rust.bin", &out).unwrap();
    let expected = std::fs::read(path).unwrap();
    if out != expected {
        let at = out
            .iter()
            .zip(&expected)
            .position(|(a, b)| a != b)
            .unwrap_or(out.len().min(expected.len()));
        eprintln!(
            "MAINMENU10 DIFFERS byte {at} lengths {}/{}",
            out.len(),
            expected.len()
        );
        false
    } else {
        println!(
            "MAINMENU10 EXACT: 1000 constructor splashes and RNG, 10000 layouts/keys, 40000 update boundaries, 60000 actions ({} bytes)",
            out.len()
        );
        true
    }
}
