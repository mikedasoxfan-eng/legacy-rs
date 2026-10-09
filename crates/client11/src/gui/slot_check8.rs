use super::*;
struct List {
    size: i32,
    content: Option<i32>,
    calls: Vec<i32>,
}
impl SlotList for List {
    fn size(&self) -> i32 {
        self.size
    }
    fn content_height(&self) -> Option<i32> {
        self.content
    }
    fn is_selected(&self, i: i32) -> bool {
        i % 3 == 0
    }
    fn element_clicked(&mut self, _: &mut Ui, i: i32, d: bool) {
        self.calls.extend([2, i, d as i32]);
    }
    fn draw_background(&mut self, _: &mut Ui) {
        self.calls.push(1);
    }
    fn draw_slot(&mut self, _: &mut Ui, i: i32, x: i32, y: i32, h: i32) {
        self.calls.extend([5, i, x, y, h]);
    }
    fn draw_header(&mut self, _: &mut Ui, x: i32, y: i32) {
        self.calls.extend([4, x, y]);
    }
    fn header_clicked(&mut self, _: &mut Ui, x: i32, y: i32) {
        self.calls.extend([3, x, y]);
    }
    fn draw_overlay(&mut self, _: &mut Ui, _: &GuiSlot, x: i32, y: i32) {
        self.calls.extend([6, x, y]);
    }
}
pub fn check(path: &str) -> bool {
    let mut ui = crate::gui::utility_check::fixture_ui("target/slot8-saves".into());
    ui.textures.insert("gui/background.png".into(), 0);
    ui.millis = 1000;
    let mut out = Vec::new();
    let mut random = worldgen::JavaRandom::new(8181);
    for c in 0..2000 {
        let width = 320 + random.next_int(800);
        let height = 200 + random.next_int(500);
        let top = 20 + random.next_int(40);
        let bottom = height - 30;
        let sh = 8 + random.next_int(60);
        let mut l = List {
            size: random.next_int(40),
            content: None,
            calls: Vec::new(),
        };
        if c % 3 == 0 {
            l.content = Some(random.next_int(3000));
        }
        let mut s = GuiSlot::new(width, height, top, bottom, sh);
        s.set_show_selection(c % 2 == 0);
        s.set_header(c % 3 != 0, random.next_int(50));
        s.register_scroll_buttons(7, 8);
        for t in 0..5 {
            let mx = match t {
                0 => width / 2,
                1 => width / 2 + 125,
                _ => random.next_int(width + 40) - 20,
            };
            let my = match t {
                0 => top + 1,
                1 => top + 20,
                _ => random.next_int(height + 40) - 20,
            };
            ui.mouse_down = t % 3 != 2;
            s.initial_click_y = match (c + t) % 4 {
                0 => -1.0,
                1 => -2.0,
                _ => (top + 15) as f32,
            };
            s.amount_scrolled = (random.next_int(4001) - 1000) as f32 / 7.0;
            s.scroll_multiplier = (random.next_int(2001) - 1000) as f32 / 317.0;
            s.selected_element = if c % 2 == 0 { 0 } else { -1 };
            s.last_clicked = if c % 2 == 0 { i64::MAX } else { 0 };
            let last = s.last_clicked;
            s.action_performed(
                &l,
                match t {
                    0 => 7,
                    1 => 8,
                    _ => 99,
                },
                c % 4 != 0,
            );
            s.draw_screen(&mut ui, &mut l, mx, my);
            for n in [
                s.mouse_x,
                s.mouse_y,
                s.selected_element,
                s.content_height(&l),
                s.slot_at(&l, mx, my),
                s.slot_at(&l, width / 2, top),
                s.slot_at(&l, width / 2, bottom + 100),
            ] {
                out.extend(n.to_le_bytes());
            }
            for n in [s.initial_click_y, s.scroll_multiplier, s.amount_scrolled] {
                out.extend((if n.is_nan() { 0x7fc00000 } else { n.to_bits() }).to_le_bytes());
            }
            out.push((s.last_clicked != last) as u8);
            out.extend((l.calls.len() as i32).to_le_bytes());
            for n in l.calls.drain(..) {
                out.extend(n.to_le_bytes());
            }
            ui.gl.verts.clear();
            ui.gl.draws.clear();
        }
    }
    std::fs::write("target/guislot8_181-rust.bin", &out).unwrap();
    let expected = std::fs::read(path).unwrap();
    if out != expected {
        let at = out
            .iter()
            .zip(&expected)
            .position(|(a, b)| a != b)
            .unwrap_or(out.len().min(expected.len()));
        eprintln!(
            "GUI SLOT ROUND 8 DIFFERS byte {at}, lengths {}/{}",
            out.len(),
            expected.len()
        );
        return false;
    }
    println!("GUI SLOT ROUND 8 EXACT: 10000 original scroll/drag/header/row callbacks and retained mouse coordinates ({} bytes)",out.len());
    true
}
