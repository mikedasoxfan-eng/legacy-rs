//! Potion inventory labels and icon layout, in the entity's HashMap iteration order.
use crate::gui::Ui;
use worldgen::entity::potion::PotionEffect;
pub fn icon(id: i32) -> Option<i32> {
    match id {
        1..=5 => Some(id - 1),
        8 => Some(10),
        9 => Some(11),
        10 => Some(7),
        11 => Some(14),
        12 => Some(15),
        13 => Some(16),
        14 => Some(8),
        15 => Some(13),
        16 => Some(12),
        17 => Some(9),
        18 => Some(5),
        19 => Some(6),
        _ => None,
    }
}
pub fn duration(ticks: i32) -> String {
    let seconds = ticks / 20;
    let minutes = seconds / 60;
    let seconds = seconds % 60;
    if seconds < 10 {
        format!("{minutes}:0{seconds}")
    } else {
        format!("{minutes}:{seconds}")
    }
}
pub struct Panel {
    pub rectangles: Vec<[i32; 6]>,
    pub text: [(String, i32, i32, i32); 2],
}
pub fn panels(ui: &Ui, x: i32, mut y: i32, effects: &[PotionEffect]) -> Vec<Panel> {
    let mut out = Vec::new();
    if effects.is_empty() {
        return out;
    }
    let step = if effects.len() > 5 {
        132 / (effects.len() as i32 - 1)
    } else {
        33
    };
    for effect in effects {
        let mut rectangles = vec![[x, y, 0, 166, 140, 32]];
        if let Some(icon) = icon(effect.id) {
            rectangles.push([x + 6, y + 7, icon % 8 * 18, 198 + icon / 8 * 18, 18, 18]);
        }
        let mut label = ui.translate(worldgen::item10::potion_name(effect.id));
        label.push_str(match effect.amplifier {1=>" II",2=>" III",3=>" IV",_=>""});
        out.push(Panel {
            rectangles,
            text: [
                (label, x + 28, y + 6, 16777215),
                (duration(effect.duration), x + 28, y + 16, 8355711),
            ],
        });
        y += step;
    }
    out
}
pub fn draw(ui: &mut Ui, x: i32, y: i32, effects: &[PotionEffect]) {
    for panel in panels(ui, x, y, effects) {
        ui.gl.color4f(1.0, 1.0, 1.0, 1.0);
        ui.gl.bind_texture(ui.texture("gui/inventory.png"));
        for [x, y, u, v, w, h] in panel.rectangles {
            ui.draw_textured_modal_rect(x, y, u, v, w, h);
        }
        for (text, x, y, color) in panel.text {
            ui.font
                .draw_string_with_shadow(&mut ui.gl, &text, x, y, color);
        }
    }
}
