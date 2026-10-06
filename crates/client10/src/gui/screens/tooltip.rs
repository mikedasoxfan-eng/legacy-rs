//! GuiContainer's release tooltip, with the world-owned item and enchantment data.
use crate::gui::Ui;
use worldgen::container::ItemStack;
pub fn lines(ui: &Ui, stack: ItemStack) -> Vec<String> {
    let mut potion = worldgen::item10::PotionItem::default();
    let name = ui
        .lang
        .translate_named_key(super::container::item_name(stack));
    let name = if stack.id == 373 {
        potion.display_name(stack.damage, &name, |key| ui.translate(key))
    } else {
        name
    };
    let mut lines = vec![name.trim_matches(|c: char| c <= ' ').to_string()];
    if stack.id == 373 {
        lines.extend(potion.tooltip(stack.damage, |key| ui.translate(key)));
    }
    if let Some(record) = worldgen::item10::record_tooltip(stack.id) {
        lines.push(record);
    }
    for enchantment in stack.enchantments() {
        if let Some(kind) = worldgen::enchantment::get(enchantment.id) {
            lines.push(kind.translated_name(enchantment.level, |key| ui.translate(key)));
        }
    }
    lines
}
pub fn draw(ui: &mut Ui, lines: &[String], rarity: i32, x: i32, mut y: i32) {
    if lines.is_empty() {
        return;
    }
    let width = lines.iter().map(|s| ui.string_width(s)).max().unwrap();
    let height = 8 + if lines.len() > 1 {
        2 + (lines.len() as i32 - 1) * 10
    } else {
        0
    };
    let bg = -267386864;
    let top = 1347420415;
    let bottom = (top & 16711422) >> 1 | top & -16777216;
    for [x0, y0, x1, y1, c0, c1] in [
        [x - 3, y - 4, x + width + 3, y - 3, bg, bg],
        [x - 3, y + height + 3, x + width + 3, y + height + 4, bg, bg],
        [x - 3, y - 3, x + width + 3, y + height + 3, bg, bg],
        [x - 4, y - 3, x - 3, y + height + 3, bg, bg],
        [x + width + 3, y - 3, x + width + 4, y + height + 3, bg, bg],
        [x - 3, y - 2, x - 2, y + height + 2, top, bottom],
        [
            x + width + 2,
            y - 2,
            x + width + 3,
            y + height + 2,
            top,
            bottom,
        ],
        [x - 3, y - 3, x + width + 3, y - 2, top, top],
        [
            x - 3,
            y + height + 2,
            x + width + 3,
            y + height + 3,
            bottom,
            bottom,
        ],
    ] {
        ui.draw_gradient_rect_z(x0, y0, x1, y1, c0, c1, 300.0);
    }
    ui.gl.push_matrix();
    for (i, line) in lines.iter().enumerate() {
        let color = if i == 0 { rarity } else { 7 };
        ui.font
            .draw_string_with_shadow(&mut ui.gl, &format!("\u{a7}{color:x}{line}"), x, y, -1);
        if i == 0 {
            y += 2;
        }
        y += 10;
    }
    ui.gl.pop_matrix();
}
