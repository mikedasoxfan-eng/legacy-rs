//! TileEntitySignRenderer. The GUI and world share the same board and text transforms.
use crate::render::entity::{fixed::Fixed, geometry::ModelRenderer};
use worldgen::sign::Sign;

#[derive(Clone, Debug)]
pub struct Snapshot {
    pub position: [i32; 3],
    pub block: u8,
    pub metadata: i32,
    pub sign: Sign,
    pub brightness: f32,
}
pub fn angle(block: u8, metadata: i32) -> f32 {
    if block == 63 {
        (metadata * 360) as f32 / 16.0
    } else {
        match metadata {
            2 => 180.0,
            4 => 90.0,
            5 => -90.0,
            _ => 0.0,
        }
    }
}
pub fn models() -> [ModelRenderer; 2] {
    let m = crate::render::entity::models::Model::new(crate::render::entity::models::ModelKind::Sign);
    [m.parts["signBoard"].clone(), m.parts["signStick"].clone()]
}
pub fn render(sign: &Sign, block: u8, metadata: i32, position: [f64; 3], brightness: f32) -> Fixed {
    let mut g = Fixed::new(brightness);
    let [x, y, z] = position.map(|v| v as f32);
    let scale = 2.0f32 / 3.0;
    g.push();
    g.translate(x + 0.5, y + 12.0 / 16.0 * scale, z + 0.5);
    g.rotate(-angle(block, metadata), 0.0, 1.0, 0.0);
    if block != 63 {
        g.translate(0.0, -5.0 / 16.0, -7.0 / 16.0);
    }
    g.texture = 15;
    g.push();
    g.scale(scale, -scale, -scale);
    for (i, model) in models().iter().enumerate() {
        if i == 1 && block != 63 {
            continue;
        }
        for face in &model.faces {
            g.begin();
            let n = face.normal();
            g.set_normal(n.x, n.y, n.z);
            for v in face.vertices {
                g.vertex(
                    v.pos.map(|v| (v as f32 / 16.0) as f64),
                    v.uv.map(|v| v as f64),
                );
            }
            g.end();
        }
    }
    g.pop();
    let text_scale = (1.0f64 / 60.0) as f32 * scale;
    g.translate(0.0, 0.5 * scale, 0.07 * scale);
    g.scale(text_scale, -text_scale, text_scale);
    g.normal = [0.0, 0.0, -text_scale];
    g.enable(32, false);
    let font = font();
    for (i, text) in sign.text.iter().enumerate() {
        let text = if i as i32 == sign.line_being_edited {
            format!("> {text} <")
        } else {
            text.clone()
        };
        font.draw_fixed(
            &mut g,
            &text,
            -font.get_string_width(&text) / 2,
            i as i32 * 10 - 20,
            0,
        );
    }
    g.enable(32, true);
    g.material = [1.0; 4];
    g.pop();
    g
}
pub fn font() -> &'static crate::gui::font::FontRenderer {
    static FONT: std::sync::OnceLock<crate::gui::font::FontRenderer> = std::sync::OnceLock::new();
    FONT.get_or_init(|| {
        let mut jar = crate::assets::Jar::open().expect("sign font jar");
        crate::gui::font::FontRenderer::new(
            &jar.image("font/default.png").expect("sign font"),
            14,
            &jar.text("font.txt").expect("font.txt"),
        )
    })
}
