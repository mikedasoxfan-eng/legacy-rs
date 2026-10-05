//! `GuiStats` with `GuiSlotStatsGeneral`, `GuiSlotStatsBlock`, `GuiSlotStatsItem` and the
//! `SorterStatsBlock` / `SorterStatsItem` orderings.

use crate::gui::slot::{GuiSlot, SlotList};
use crate::gui::stats_table::{self as table, StatType};
use crate::gui::widgets::GuiButton;
use crate::gui::{Screen, ScreenBase, Ui};
use crate::render::entity::gui_item;
use worldgen::world::tile::ItemStack;

/// `StatBase.func_27083_i()`: `NumberFormat.getIntegerInstance(Locale.US)`.
fn integer_format(v: i64) -> String {
    let digits = v.unsigned_abs().to_string();
    let mut out = String::new();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    if v < 0 {
        format!("-{out}")
    } else {
        out
    }
}

/// `StatBase.func_27081_j()`: `new DecimalFormat("########0.00")` (half-even rounding).
fn decimal_format(v: f64) -> String {
    let scaled = v * 100.0;
    let floor = scaled.floor();
    let diff = scaled - floor;
    let mut r = if diff > 0.5 {
        floor + 1.0
    } else if diff < 0.5 {
        floor
    } else if floor % 2.0 == 0.0 {
        floor
    } else {
        floor + 1.0
    };
    if r == -0.0 {
        r = 0.0;
    }
    let r = r as i64;
    format!("{}.{:02}", r / 100, (r % 100).abs())
}

/// Java's `Double.toString` for the small values the time format prints.
fn java_double(v: f64) -> String {
    let a = v.abs();
    if a != 0.0 && (!(1e-3..1e7).contains(&a)) {
        let text = format!("{v:e}");
        let (mantissa, exponent) = text.split_once('e').unwrap();
        let mantissa = if mantissa.contains('.') { mantissa.to_string() } else { format!("{mantissa}.0") };
        return format!("{mantissa}E{exponent}");
    }
    let s = format!("{v}");
    if s.contains('.') || s.contains('e') || s.contains("inf") || s.contains("NaN") {
        s
    } else {
        format!("{s}.0")
    }
}

/// `IStatType.func_27192_a`
pub(crate) fn format_stat(kind: StatType, v: i32) -> String {
    match kind {
        StatType::Simple => integer_format(v as i64),
        StatType::Time => {
            let s = v as f64 / 20.0;
            let m = s / 60.0;
            let h = m / 60.0;
            let d = h / 24.0;
            let y = d / 365.0;
            if y > 0.5 {
                decimal_format(y) + " y"
            } else if d > 0.5 {
                decimal_format(d) + " d"
            } else if h > 0.5 {
                decimal_format(h) + " h"
            } else if m > 0.5 {
                decimal_format(m) + " m"
            } else {
                java_double(s) + " s"
            }
        }
        StatType::Distance => {
            let m = v as f64 / 100.0;
            let km = m / 1000.0;
            if km > 0.5 {
                decimal_format(km) + " km"
            } else if m > 0.5 {
                decimal_format(m) + " m"
            } else {
                format!("{v} cm")
            }
        }
    }
}

/// `GuiStats.func_27136_c`: an 18x18 cell of gui/slot.png.
fn slot_cell(ui: &mut Ui, x: i32, y: i32, u: i32, v: i32) {
    let tex = ui.texture("gui/slot.png");
    let gl = &mut ui.gl;
    gl.color4f(1.0, 1.0, 1.0, 1.0);
    gl.bind_texture(tex);
    let s = 0.0078125f32;
    gl.start_drawing_quads();
    gl.add_vertex_with_uv(x as f64, (y + 18) as f64, 0.0, (u as f32 * s) as f64, ((v + 18) as f32 * s) as f64);
    gl.add_vertex_with_uv((x + 18) as f64, (y + 18) as f64, 0.0, ((u + 18) as f32 * s) as f64, ((v + 18) as f32 * s) as f64);
    gl.add_vertex_with_uv((x + 18) as f64, y as f64, 0.0, ((u + 18) as f32 * s) as f64, (v as f32 * s) as f64);
    gl.add_vertex_with_uv(x as f64, y as f64, 0.0, (u as f32 * s) as f64, (v as f32 * s) as f64);
    gl.draw();
}

/// `GuiStats.func_27138_c`: the slot background and the item icon.
fn item_cell(ui: &mut Ui, x: i32, y: i32, item: i32) {
    slot_cell(ui, x + 1, y + 1, 0, 0);
    gui_item::set_gui_light_rotation(180.0);
    gui_item::set_gui_rescale_normal(true);
    gui_item::render_item_into_gui(ui, Some(&ItemStack::new(item, 1, 0)), x + 2, y + 2);
    gui_item::set_gui_rescale_normal(false);
}

// ------------------------------------------------------------------ general

struct General;

impl SlotList for General {
    fn size(&self) -> i32 {
        table::GENERAL.len() as i32
    }
    fn element_clicked(&mut self, _ui: &mut Ui, _index: i32, _double: bool) {}
    fn is_selected(&self, _index: i32) -> bool {
        false
    }
    fn content_height(&self) -> Option<i32> {
        Some(self.size() * 10)
    }
    fn draw_background(&mut self, ui: &mut Ui) {
        let sr = ui.scaled();
        ui.draw_world_background(sr.width, sr.height, 0);
    }
    fn draw_slot(&mut self, ui: &mut Ui, index: i32, x: i32, y: i32, _h: i32) {
        let (id, kind, name) = table::GENERAL[index as usize];
        let color = if index % 2 == 0 { 16777215 } else { 9474192 };
        ui.draw_string(name, x + 2, y + 1, color);
        let v = format_stat(kind, ui.stats.get(id));
        let w = ui.string_width(&v);
        ui.draw_string(&v, x + 2 + 213 - w, y + 1, color);
    }
}

// ------------------------------------------------------------------ blocks and items

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Block,
    Item,
}

/// `GuiSlotStats` and its two subclasses.
struct StatsList {
    kind: Kind,
    /// (stat id, item id) of the shown rows.
    rows: Vec<(i32, i32)>,
    /// `field_27268_b`: the header column being pressed.
    pressed: i32,
    /// `field_27271_e`, `field_27270_f`: sort column and direction.
    sort_column: i32,
    sort_dir: i32,
}

impl StatsList {
    fn new(ui: &Ui, kind: Kind) -> StatsList {
        let source: &[(i32, i32)] = if kind == Kind::Block { &table::BLOCKS } else { &table::ITEMS };
        let rows = source
            .iter()
            .copied()
            .filter(|&(stat, item)| {
                if ui.stats.get(stat) > 0 {
                    return true;
                }
                let second = if kind == Kind::Block { &table::USE[..] } else { &table::BREAK[..] };
                if table::lookup(second, item).is_some_and(|s| ui.stats.get(s) > 0) {
                    return true;
                }
                table::lookup(&table::CRAFT, item).is_some_and(|s| ui.stats.get(s) > 0)
            })
            .collect();
        StatsList { kind, rows, pressed: -1, sort_column: -1, sort_dir: 0 }
    }

    /// The three columns' stat tables, left to right.
    fn columns(&self) -> [&'static [(i32, i32)]; 3] {
        match self.kind {
            Kind::Block => [&table::CRAFT, &table::USE, &table::MINE],
            Kind::Item => [&table::BREAK, &table::CRAFT, &table::USE],
        }
    }

    /// `func_27263_a`
    fn column_key(&self, c: i32) -> &'static str {
        match (self.kind, c) {
            (Kind::Block, 0) => "stat.crafted",
            (Kind::Block, 1) => "stat.used",
            (Kind::Block, _) => "stat.mined",
            (Kind::Item, 1) => "stat.crafted",
            (Kind::Item, 2) => "stat.used",
            (Kind::Item, _) => "stat.depleted",
        }
    }

    /// `SorterStatsBlock` / `SorterStatsItem`
    fn sort(&mut self, ui: &Ui) {
        let column = self.sort_column;
        let dir = self.sort_dir;
        // The comparators' column indexes differ: block 2 = mined, 0 = crafted, 1 = used;
        // item 0 = broken, 1 = crafted, 2 = used. `columns()` already follows the screen.
        let tables = self.columns();
        self.rows.sort_by(|a, b| {
            let (ia, ib) = (a.1, b.1);
            if (0..3).contains(&column) {
                let t = tables[column as usize];
                let (sa, sb) = (table::lookup(t, ia), table::lookup(t, ib));
                if sa.is_some() || sb.is_some() {
                    let Some(sa) = sa else { return std::cmp::Ordering::Greater };
                    let Some(sb) = sb else { return std::cmp::Ordering::Less };
                    let (va, vb) = (ui.stats.get(sa), ui.stats.get(sb));
                    if va != vb {
                        return ((va - vb) * dir).cmp(&0);
                    }
                }
            }
            (ia - ib).cmp(&0)
        });
    }

    /// `func_27265_a`: a number right-aligned at `x`.
    fn cell(ui: &mut Ui, stat: Option<i32>, x: i32, y: i32, even: bool) {
        let v = match stat {
            Some(s) => format_stat(StatType::Simple, ui.stats.get(s)),
            None => "-".to_string(),
        };
        let w = ui.string_width(&v);
        ui.draw_string(&v, x - w, y + 5, if even { 16777215 } else { 9474192 });
    }
}

impl SlotList for StatsList {
    fn size(&self) -> i32 {
        self.rows.len() as i32
    }
    fn element_clicked(&mut self, _ui: &mut Ui, _index: i32, _double: bool) {}
    fn is_selected(&self, _index: i32) -> bool {
        false
    }
    fn draw_background(&mut self, ui: &mut Ui) {
        let sr = ui.scaled();
        ui.draw_world_background(sr.width, sr.height, 0);
    }
    fn draw_header(&mut self, ui: &mut Ui, x: i32, y: i32) {
        if !ui.mouse_down {
            self.pressed = -1;
        }
        for (c, cx) in [(0, 115), (1, 165), (2, 215)] {
            slot_cell(ui, x + cx - 18, y + 1, 0, if self.pressed == c { 0 } else { 18 });
        }
        if self.sort_column != -1 {
            let ax = [79, 129, 179][self.sort_column as usize];
            let v = if self.sort_dir == 1 { 36 } else { 18 };
            slot_cell(ui, x + ax, y + 1, v, 0);
        }
        let icons = match self.kind {
            Kind::Block => [18, 36, 54],
            Kind::Item => [72, 18, 36],
        };
        for (c, cx) in [(0, 115), (1, 165), (2, 215)] {
            let u = icons[c as usize];
            if self.pressed == c {
                slot_cell(ui, x + cx - 18 + 1, y + 1 + 1, u, 18);
            } else {
                slot_cell(ui, x + cx - 18, y + 1, u, 18);
            }
        }
    }
    fn header_clicked(&mut self, ui: &mut Ui, x: i32, _y: i32) {
        self.pressed = -1;
        if (79..115).contains(&x) {
            self.pressed = 0;
        } else if (129..165).contains(&x) {
            self.pressed = 1;
        } else if (179..215).contains(&x) {
            self.pressed = 2;
        }
        if self.pressed >= 0 {
            // func_27266_c
            let c = self.pressed;
            if c != self.sort_column {
                self.sort_column = c;
                self.sort_dir = -1;
            } else if self.sort_dir == -1 {
                self.sort_dir = 1;
            } else {
                self.sort_column = -1;
                self.sort_dir = 0;
            }
            self.sort(ui);
            ui.sounds.push(("random.click", 1.0, 1.0));
        }
    }
    fn draw_slot(&mut self, ui: &mut Ui, index: i32, x: i32, y: i32, _h: i32) {
        let (stat, item) = self.rows[index as usize];
        item_cell(ui, x + 40, y, item);
        let even = index % 2 == 0;
        let [a, b, _] = self.columns();
        Self::cell(ui, table::lookup(a, item), x + 115, y, even);
        Self::cell(ui, table::lookup(b, item), x + 165, y, even);
        Self::cell(ui, Some(stat), x + 215, y, even);
    }
    fn draw_overlay(&mut self, ui: &mut Ui, slot: &GuiSlot, mx: i32, my: i32) {
        if my < slot.top || my > slot.bottom {
            return;
        }
        let row = slot.slot_at(self, mx, my);
        let left = ui.scaled().width / 2 - 92 - 16;
        let text = if row >= 0 {
            if mx < left + 40 || mx > left + 40 + 20 {
                return;
            }
            let item = self.rows[row as usize].1;
            ui.lang.translate_named_key(super::container::item_name(worldgen::container::ItemStack::new(item, 1, 0)).trim()).trim().to_string()
        } else {
            let key = if mx >= left + 115 - 18 && mx <= left + 115 {
                self.column_key(0)
            } else if mx >= left + 165 - 18 && mx <= left + 165 {
                self.column_key(1)
            } else if mx >= left + 215 - 18 && mx <= left + 215 {
                self.column_key(2)
            } else {
                return;
            };
            ui.translate(key).trim().to_string()
        };
        if !text.is_empty() {
            let (x, y) = (mx + 12, my - 12);
            let w = ui.string_width(&text);
            ui.draw_gradient_rect(x - 3, y - 3, x + w + 3, y + 8 + 3, -1073741824, -1073741824);
            ui.font.draw_string_with_shadow(&mut ui.gl, &text, x, y, -1);
        }
    }
}

// ------------------------------------------------------------------ the screen

pub struct GuiStats {
    base: ScreenBase,
    parent: Option<Box<dyn Screen>>,
    title: String,
    general: General,
    blocks: Option<StatsList>,
    items: Option<StatsList>,
    slots: [GuiSlot; 3],
    /// `field_27155_p`: 0 general, 1 items, 2 blocks (the buttons' ids minus one).
    current: usize,
}

impl GuiStats {
    pub fn new(parent: Box<dyn Screen>) -> GuiStats {
        GuiStats {
            base: ScreenBase::default(),
            parent: Some(parent),
            title: "Select world".into(),
            general: General,
            blocks: None,
            items: None,
            slots: [GuiSlot::new(0, 0, 0, 0, 10), GuiSlot::new(0, 0, 0, 0, 20), GuiSlot::new(0, 0, 0, 0, 20)],
            current: 0,
        }
    }
}

impl Screen for GuiStats {
    fn base(&mut self) -> &mut ScreenBase {
        &mut self.base
    }

    fn init_gui(&mut self, ui: &mut Ui) {
        let (w, h) = (self.base.width, self.base.height);
        self.title = ui.translate("gui.stats");
        let mut general = GuiSlot::new(w, h, 32, h - 64, 10);
        general.set_show_selection(false);
        general.register_scroll_buttons(1, 1);
        let mut items = GuiSlot::new(w, h, 32, h - 64, 20);
        items.set_show_selection(false);
        items.set_header(true, 20);
        items.register_scroll_buttons(1, 1);
        let mut blocks = GuiSlot::new(w, h, 32, h - 64, 20);
        blocks.set_show_selection(false);
        blocks.set_header(true, 20);
        blocks.register_scroll_buttons(1, 1);
        self.slots = [general, items, blocks];
        self.items = Some(StatsList::new(ui, Kind::Item));
        self.blocks = Some(StatsList::new(ui, Kind::Block));
        self.current = 0;
        // func_27130_k
        let b = &mut self.base.buttons;
        b.clear();
        b.push(GuiButton::sized(0, w / 2 + 4, h - 28, 150, 20, ui.translate("gui.done")));
        b.push(GuiButton::sized(1, w / 2 - 154, h - 52, 100, 20, ui.translate("stat.generalButton")));
        let mut blocks_button = GuiButton::sized(2, w / 2 - 46, h - 52, 100, 20, ui.translate("stat.blocksButton"));
        let mut items_button = GuiButton::sized(3, w / 2 + 62, h - 52, 100, 20, ui.translate("stat.itemsButton"));
        if self.blocks.as_ref().is_some_and(|l| l.rows.is_empty()) {
            blocks_button.enabled = false;
        }
        if self.items.as_ref().is_some_and(|l| l.rows.is_empty()) {
            items_button.enabled = false;
        }
        b.push(blocks_button);
        b.push(items_button);
    }

    fn action_performed(&mut self, ui: &mut Ui, id: i32) {
        let enabled = self.base.buttons.iter().find(|b| b.id == id).is_some_and(|b| b.enabled);
        if !enabled {
            return;
        }
        match id {
            0 => {
                if let Some(p) = self.parent.take() {
                    ui.display(Some(p));
                }
            }
            1 => self.current = 0,
            3 => self.current = 1,
            2 => self.current = 2,
            _ => {
                let i = self.current;
                match i {
                    0 => self.slots[0].action_performed(&self.general, id, true),
                    1 => self.slots[1].action_performed(self.items.as_ref().unwrap(), id, true),
                    _ => self.slots[2].action_performed(self.blocks.as_ref().unwrap(), id, true),
                }
            }
        }
    }

    fn draw_screen(&mut self, ui: &mut Ui, mx: i32, my: i32, _partial: f32) {
        match self.current {
            0 => self.slots[0].draw_screen(ui, &mut self.general, mx, my),
            1 => self.slots[1].draw_screen(ui, self.items.as_mut().unwrap(), mx, my),
            _ => self.slots[2].draw_screen(ui, self.blocks.as_mut().unwrap(), mx, my),
        }
        let w = self.base.width;
        let title = self.title.clone();
        ui.draw_centered_string(&title, w / 2, 20, 16777215);
        self.draw_buttons(ui, mx, my);
    }
}
