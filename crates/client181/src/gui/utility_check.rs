use super::{font::FontRenderer, widgets::GuiButton, ScaledResolution};
pub fn check(path: &str) -> bool {
    let mut out = Vec::new();
    let mut r = worldgen::JavaRandom::new(181);
    for _ in 0..10000 {
        let scale = r.next_int(7) - 1;
        let w = r.next_int(8193);
        let h = r.next_int(4097);
        let s = ScaledResolution::new(scale, w, h);
        for v in [s.width, s.height, s.scale_factor] {
            out.extend(v.to_le_bytes());
        }
        for v in [s.width_d, s.height_d] {
            out.extend(v.to_le_bytes());
        }
    }
    for c in 0..10000 {
        let x = r.next_int(1000) - 500;
        let y = r.next_int(1000) - 500;
        let w = r.next_int(250);
        let h = r.next_int(50);
        let mx = x + r.next_int(260) - 5;
        let my = y + r.next_int(60) - 5;
        let mut b = GuiButton::sized(c, x, y, w, h, "label");
        b.enabled = r.next_bool();
        b.visible = r.next_bool();
        out.push(b.inside(mx, my) as u8);
        out.extend(
            b.hover_state(mx >= x && my >= y && mx < x + w && my < y + h)
                .to_le_bytes(),
        );
    }
    let mut jar = crate::assets::Jar::open().unwrap();
    let image = jar.image("font/default.png").unwrap();
    let allowed = jar.text("font.txt").unwrap();
    let font = FontRenderer::new(&image, 0, &allowed);
    for text in [
        "",
        " ",
        "   ",
        "Minecraft Beta 1.8.1",
        "one two three four five",
        "one\n\ntwo\n",
        "\n",
        "\n\n",
        "longunbrokentextwithoutspaces",
        "§aGreen §rReset",
        "§",
        "§zbad",
        "1234567890 !?",
        "éßñå",
        "  leading trailing  ",
    ] {
        out.extend(font.get_string_width(text).to_le_bytes());
        for w in [10, 11, 20, 40, 80, 120, 200] {
            out.extend(font.split_string_height(text, w).to_le_bytes());
        }
    }
    for c in 0..256 {
        out.extend(
            font.get_string_width(&char::from_u32(c).unwrap().to_string())
                .to_le_bytes(),
        );
    }
    let expected = std::fs::read(path).unwrap();
    std::fs::write("target/guiutility181-rust.bin", &out).unwrap();
    if out == expected {
        println!("GUI UTILITIES EXACT: 10000 scales, 10000 buttons, 376 font cases");
        true
    } else {
        eprintln!(
            "GUI UTILITIES DIFFER byte {}",
            out.iter()
                .zip(expected)
                .position(|(a, b)| *a != b)
                .unwrap_or(out.len())
        );
        false
    }
}
