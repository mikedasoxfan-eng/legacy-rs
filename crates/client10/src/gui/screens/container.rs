//! GuiContainer and its five backgrounds. Contents arrive from the world thread;
//! drawing and hover never mutate simulation state.
use crate::gui::{Screen, ScreenBase, Ui};
use crate::render::entity::gui_item;
use worldgen::container::{ContainerCommand, ContainerKind, ContainerSnapshot, ItemStack};

pub struct GuiContainer {
    base: ScreenBase,
    window: i32,
    kind: ContainerKind,
    inventory_name: Option<&'static str>,
    previous_mouse: [f32; 2],
    scroll:f32, dragging:bool, previous_down:bool,
    left: i32,
    top: i32,
    enchantment: super::enchantment::EnchantmentAnimation,
    galactic: Option<crate::gui::font::FontRenderer>,
}
pub type GuiInventory = GuiContainer;
pub type GuiCrafting = GuiContainer;
pub type GuiChest = GuiContainer;
pub type GuiFurnace = GuiContainer;
pub type GuiDispenser = GuiContainer;
impl GuiContainer {
    pub fn new(snapshot: &ContainerSnapshot) -> Self {
        Self {
            base: Default::default(),
            window: snapshot.window,
            kind: snapshot.kind,
            inventory_name: snapshot.inventory_name,
            previous_mouse: [0.0; 2],
            scroll:0.0,dragging:false,previous_down:false,
            left: 0, top: 0,
            enchantment: super::enchantment::EnchantmentAnimation::new(super::select_world::random_seed()),
            galactic: None,
        }
    }
    fn update_scroll(&self,ui:&mut Ui) {
        if let Some(s)=ui.container_snapshot.as_mut() {let contents=worldgen::container::creative::contents(self.scroll);s.contents[..72].copy_from_slice(&contents);}
        ui.container_commands.push(ContainerCommand::ScrollCreative {window:self.window,position:self.scroll});
    }
    fn size(&self) -> (i32, i32) {
        (
            176,
            match self.kind {
                ContainerKind::Chest { rows } => 114 + rows as i32 * 18,
                ContainerKind::Creative => 208,
                _ => 166,
            },
        )
    }
    pub(crate) fn origin(&self) -> (i32, i32) {
        (self.left, self.top)
    }
    fn hover(&self, s: &worldgen::container::Slot, x: i32, y: i32) -> bool {
        let (ox, oy) = self.origin();
        let (x, y) = (x - ox, y - oy);
        x >= s.x - 1 && x < s.x + 17 && y >= s.y - 1 && y < s.y + 17
    }
    fn background(&mut self, ui: &mut Ui, s: &ContainerSnapshot) {
        let (x, y) = self.origin();
        let (w, h) = self.size();
        ui.gl.color4f(1.0, 1.0, 1.0, 1.0);
        let texture = match self.kind {
            ContainerKind::Player => "gui/inventory.png",
            ContainerKind::Creative => "gui/allitems.png",
            ContainerKind::Workbench => "gui/crafting.png",
            ContainerKind::Chest { .. } => "gui/container.png",
            ContainerKind::Furnace => "gui/furnace.png",
            ContainerKind::Dispenser => "gui/trap.png",
            ContainerKind::BrewingStand => "gui/alchemy.png",
            ContainerKind::Enchantment => "gui/enchant.png",
        };
        ui.gl.bind_texture(ui.texture(texture));
        for [x, y, u, v, w, h] in background_rectangles(s, x, y, self.scroll) {
            ui.draw_textured_modal_rect(x, y, u, v, w, h);
        }
        if self.kind == ContainerKind::Player {
            let effects = ui.release.effects.clone();
            super::potion_effects::draw(ui, x - 124, y, &effects);
            super::inventory_model::draw(ui, x, y, self.previous_mouse, s);
        }
    }
    fn foreground(&self, ui: &mut Ui) {
        for (name, x, y, color) in foreground_labels(self.kind, self.size().1, self.inventory_name) {
            ui.font.draw_string(&mut ui.gl, name, x, y, color);
        }
    }
}
pub(crate) fn item_name(s: ItemStack) -> &'static str {
    use std::{collections::HashMap, sync::OnceLock};
    static NAMES: OnceLock<HashMap<(i32, i32), &'static str>> = OnceLock::new();
    let names = NAMES.get_or_init(|| {
        include_str!("item_names.tsv")
            .lines()
            .filter_map(|l| {
                let mut parts = l.split('\t');
                Some((
                    (parts.next()?.parse().ok()?, parts.next()?.parse().ok()?),
                    parts.next()?,
                ))
            })
            .collect()
    });
    names
        .get(&(s.id, s.damage))
        .or_else(|| names.get(&(s.id, 0)))
        .copied()
        .unwrap_or("")
}
fn draw_item(ui: &mut Ui, s: Option<ItemStack>, x: i32, y: i32) {
    let count = s.map_or(0, |s| s.count);
    let s = s.map(worldgen::world::tile::ItemStack::from);
    if let Some(s) = s.as_ref() {
        let g = gui_item::geometry(s, x, y, true);
        for d in &g.draws {
            let mut batch = d.batch();
            if d.flags & 1 != 0 && d.flags & 128 != 0 {
                let inv = d.matrix().inverse();
                let nm = inv.transpose();
                let rescale = 1.0 / inv.row(2).truncate().length();
                for (quad, raw) in batch
                    .vertices
                    .chunks_exact_mut(6)
                    .zip(d.vertices.chunks_exact(4))
                {
                    for (v, i) in quad.iter_mut().zip([0, 1, 2, 0, 2, 3]) {
                        let mut n = nm
                            .transform_vector3(if s.id==54 {let b=|shift| (2.0*((raw[i].normal>>shift) as u8 as i8) as f32+1.0)/255.0;glam::Vec3::new(b(0),b(8),b(16))}else {super::inventory_model::byte_normal(raw[i].normal)});
                        if d.flags & 4 != 0 {
                            n = n.normalize_or_zero();
                        } else if d.flags & 2 != 0 {
                            n *= rescale;
                        }
                        let c = if d.flags & 64 != 0 {
                            raw[i].color.to_le_bytes().map(|c| c as f32 / 255.0)
                        } else {
                            d.material
                        };
                        v.color = crate::render::entity::living::lit_color_rotated(
                            n,
                            c,
                            d.light_rotation,
                        );
                    }
                }
            }
            ui.gl.texture_2d = batch.texture.is_some();
            if let Some(t) = batch.texture {
                ui.gl.bind_texture(ui.texture(t));
            }
            if matches!(s.id,18|20) {
                let mut front=Vec::new();
                for t in batch.vertices.chunks_exact(3) {
                    let a=t[0].pos;let b=t[1].pos;let c=t[2].pos;
                    if (b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0])<0.0 {front.extend_from_slice(t);}
                }
                ui.gl.triangles(&front);
            }else {ui.gl.triangles(&batch.vertices);}
        }
        ui.gl.color = g.material;
    }
    if count > 1 {
        let text = count.to_string();
        ui.gl.depth_test = false;
        ui.font.draw_string_with_shadow(
            &mut ui.gl,
            &text,
            x + 17 - ui.font.get_string_width(&text),
            y + 9,
            0xffffff,
        );
        ui.gl.depth_test = true;
    }
    let overlay = s.map(|mut s| {
        s.count = 1;
        s
    });
    gui_item::render_item_overlay_into_gui(ui, overlay.as_ref(), x, y);
}
impl Screen for GuiContainer {
    fn init_gui(&mut self, ui: &mut Ui) {
        self.base.buttons.clear();
        self.redirect_inventory(ui);
        if matches!(self.kind, ContainerKind::Player | ContainerKind::Creative)
            && ui.container_snapshot.as_ref().is_some_and(|s| s.creative != (self.kind == ContainerKind::Creative)) {
            return;
        }
        let (w, h) = self.size();
        self.left = (self.base.width - w) / 2;
        self.top = (self.base.height - h) / 2;
        if self.kind == ContainerKind::Player && !ui.release.effects.is_empty() {
            self.left = 160 + (self.base.width - w - 200) / 2;
        }
        if self.kind == ContainerKind::Enchantment && self.galactic.is_none() {
            let mut jar = crate::assets::Jar::open().expect("GuiEnchantment resources");
            self.galactic = Some(crate::gui::font::FontRenderer::new_with_anaglyph(
                &jar.image("font/alternate.png").expect("galactic font"), ui.texture("font/alternate.png"),
                &jar.text("font.txt").expect("font.txt"), ui.settings.anaglyph));
        }
    }
    fn action_performed(&mut self, ui: &mut Ui, id: i32) {
        if self.kind == ContainerKind::Player {
            if id == 0 { ui.display(Some(Box::new(super::achievements::GuiAchievements::new()))); }
            if id == 1 { ui.display_child(|parent| Box::new(super::stats::GuiStats::new(parent))); }
        }
    }
    fn base(&mut self) -> &mut ScreenBase {
        &mut self.base
    }
    fn draw_screen(&mut self, ui: &mut Ui, mx: i32, my: i32, partial: f32) {
        if self.kind==ContainerKind::Creative {
            let (ox,oy)=self.origin();
            if !self.previous_down && ui.mouse_down && mx>=ox+155 && mx<ox+169 && my>=oy+17 && my<oy+179 {self.dragging=true;}
            if !ui.mouse_down {self.dragging=false;}self.previous_down=ui.mouse_down;
            if self.dragging {self.scroll=((my-(oy+25)) as f32/146.0).clamp(0.0,1.0);self.update_scroll(ui);}
        }
        let Some(s) = ui
            .container_snapshot
            .clone()
            .filter(|s| s.window == self.window)
        else {
            return;
        };
        ui.draw_world_background(self.base.width, self.base.height, 0);
        self.background(ui, &s);
        if self.kind == ContainerKind::Enchantment {
            let (x, y) = self.origin();
            for (i, name) in super::enchantment::names(s.enchantment_name_seed).iter().enumerate() {
                let cost = s.progress[i];
                let hovered = mx >= x + 60 && mx < x + 168 && my >= y + 14 + 19 * i as i32 && my < y + 33 + 19 * i as i32;
                let (v, color, level_color) = super::enchantment::offer(cost, ui.release.player_level, hovered);
                ui.gl.bind_texture(ui.texture("gui/enchant.png"));
                ui.gl.color4f(1.0, 1.0, 1.0, 1.0);
                ui.draw_textured_modal_rect(x + 60, y + 14 + 19 * i as i32, 0, v, 108, 19);
                if cost != 0 {
                    if let Some(font) = &self.galactic { font.draw_split_string(&mut ui.gl, name, x + 62, y + 16 + 19 * i as i32, 104, color); }
                    let text = cost.to_string();
                    ui.font.draw_string_with_shadow(&mut ui.gl, &text, x + 166 - ui.font.get_string_width(&text), y + 23 + 19 * i as i32, level_color);
                }
            }
            // ModelBook geometry is owned by the render worker; the exact pose is available here.
            let _book_pose = self.enchantment.pose(partial);
        }
        // GuiContainer sets unit 1's current coordinate after the background
        // (and its player figure), even when texturing on that unit is disabled.
        ui.gl.lightmap_coordinates = 240 | (240 << 16);
        let (ox, oy) = self.origin();
        gui_item::set_gui_light_rotation(120.0);
        gui_item::set_gui_rescale_normal(true);
        ui.gl.push_matrix();
        ui.gl.translatef(ox as f32, oy as f32, 0.0);
        ui.gl.color4f(1.0, 1.0, 1.0, 1.0);
        let mut hover = None;
        for (i, slot) in s.slots.iter().enumerate() {
            draw_item(ui, s.contents[i], slot.x, slot.y);
            if self.hover(slot, mx, my) {
                hover = Some(i);
                ui.gl.depth_test = false;
                ui.draw_gradient_rect(
                    slot.x,
                    slot.y,
                    slot.x + 16,
                    slot.y + 16,
                    -2130706433,
                    -2130706433,
                );
                ui.gl.depth_test = true;
            }
        }
        if s.cursor.is_some() {
            ui.gl.translatef(0.0, 0.0, 32.0);
            draw_item(ui, s.cursor, mx - ox - 8, my - oy - 8);
        }
        gui_item::set_gui_rescale_normal(false);
        gui_item::set_gui_light_rotation(0.0);
        ui.gl.depth_test = false;
        self.foreground(ui);
        if s.cursor.is_none() {
            if let Some(stack) = hover.and_then(|i| s.contents[i]) {
                let lines = super::tooltip::lines(ui, stack);
                super::tooltip::draw(ui, &lines, worldgen::item10::rarity(stack).color(), mx - ox + 12, my - oy - 12);
            }
        }
        ui.gl.pop_matrix();
        self.draw_buttons(ui, mx, my);
        ui.gl.depth_test = true;
        let _ = partial;
        self.previous_mouse = [mx as f32, my as f32];
    }
    fn mouse_clicked(&mut self, ui: &mut Ui, x: i32, y: i32, button: i32) {
        crate::gui::default_mouse_clicked(self, ui, x, y, button);
        if button != 0 && button != 1 {
            self.click_enchantment(ui, x, y);
            return;
        }
        let Some(s) = ui
            .container_snapshot
            .as_ref()
            .filter(|s| s.window == self.window)
        else {
            return;
        };
        let (ox, oy) = self.origin();
        let (w, h) = self.size();
        let outside = x < ox || y < oy || x >= ox + w || y >= oy + h;
        let slot = if outside {
            -999
        } else {
            s.slots
                .iter()
                .position(|s| self.hover(s, x, y))
                .map_or(-1, |i| i as i32)
        };
        if slot != -1 {
            ui.container_commands.push(ContainerCommand::Click {
                window: self.window,
                slot,
                button,
                shift: slot != -999 && ui.shift_down,
            });
        }
            self.click_enchantment(ui, x, y);
    }
    fn mouse_scrolled(&mut self,ui:&mut Ui,amount:i32) {
        if self.kind==ContainerKind::Creative && amount!=0 {
            let rows=worldgen::container::creative::catalog().len()/8-8+1;
            self.scroll=(self.scroll as f64-amount.signum() as f64/rows as f64) as f32;
            self.scroll=self.scroll.clamp(0.0,1.0);self.update_scroll(ui);
        }
    }
    fn key_typed(&mut self, ui: &mut Ui, _c: char, key: i32) {
        if key == 1
            || ui
                .settings
                .key_bindings
                .iter()
                .any(|binding| binding.description == "key.inventory" && key == binding.code())
        {
            ui.display(None);
        }
    }
    fn on_gui_closed(&mut self, ui: &mut Ui) {
        ui.container_commands.push(ContainerCommand::CloseFromGui {
            window: self.window,
        });
    }
    fn update_screen(&mut self, ui: &mut Ui) {
        if self.kind == ContainerKind::Enchantment {
            if let Some(s) = ui.container_snapshot.as_ref().filter(|s| s.window == self.window) {
                self.enchantment.tick(s.contents[0], s.progress);
            }
        }
        if matches!(self.kind, ContainerKind::Player | ContainerKind::Creative) {
            self.redirect_inventory(ui);
            return;
        }
        if ui
            .container_snapshot
            .as_ref()
            .is_some_and(|s| s.window == self.window && !s.alive)
        {
            ui.display(None);
        }
    }
    fn does_gui_pause_game(&self) -> bool {
        false
    }
    fn is_container(&self) -> bool {
        true
    }
}
impl GuiContainer {
    fn redirect_inventory(&self, ui: &mut Ui) {
        if matches!(self.kind, ContainerKind::Player | ContainerKind::Creative)
            && ui.container_snapshot.as_ref().is_some_and(|s| s.creative != (self.kind == ContainerKind::Creative))
        {
            ui.actions.push(crate::gui::Action::RedirectInventory);
        }
    }
}

impl GuiContainer {
    fn click_enchantment(&self, ui: &mut Ui, x: i32, y: i32) {
        if self.kind == ContainerKind::Enchantment {
            let (ox, oy) = self.origin();
            if let Some(s) = ui.container_snapshot.as_ref().filter(|s| s.window == self.window) {
                for offer in 0..3 {
                    let dx = x - (ox + 60);
                    let dy = y - (oy + 14 + 19 * offer as i32);
                    if dx >= 0 && dx < 108 && dy >= 0 && dy < 19 && s.progress[offer] > 0
                        && ui.release.player_level >= s.progress[offer] && s.contents[0].is_some() {
                        ui.container_commands.push(ContainerCommand::Enchant { window: self.window, offer });
                    }
                }
            }
        }
    }
}

fn background_rectangles(s: &ContainerSnapshot, x: i32, y: i32, scroll: f32) -> Vec<[i32; 6]> {
    if s.kind == ContainerKind::BrewingStand { return super::brewing::rectangles(s.progress[0], x, y); }
    let height = match s.kind { ContainerKind::Chest { rows } => 114 + rows as i32 * 18, ContainerKind::Creative => 208, _ => 166 };
    let mut out = if let ContainerKind::Chest { rows } = s.kind {
        vec![[x, y, 0, 0, 176, rows as i32 * 18 + 17], [x, y + rows as i32 * 18 + 17, 0, 126, 176, 96]]
    } else { vec![[x, y, 0, 0, 176, height]] };
    if s.kind == ContainerKind::Furnace {
        let [cook, burn, fuel] = s.progress;
        if burn > 0 { let n = burn.wrapping_mul(12) / if fuel == 0 { 200 } else { fuel }; out.push([x+56, y+48-n, 176, 12-n, 14, n+2]); }
        let n = cook.wrapping_mul(24) / 200; out.push([x+79, y+34, 176, 14, n+1, 16]);
    }
    if s.kind == ContainerKind::Creative { out.push([x+154, y+17+(145.0*scroll) as i32, 0, 208, 16, 16]); }
    out
}
fn foreground_labels(kind: ContainerKind, h: i32, inventory_name: Option<&'static str>) -> Vec<(&'static str, i32, i32, i32)> {

        let (name, x, y) = match kind {
            ContainerKind::Player => ("Crafting", 86, 16),
            ContainerKind::Creative => ("Item selection",8,6),
            ContainerKind::Workbench => ("Crafting", 28, 6),
            ContainerKind::Chest { rows } => {
                (inventory_name.unwrap_or(if rows == 3 { "Chest" } else { "Large chest" }), 8, 6)
            }
            ContainerKind::Furnace => ("Furnace", 60, 6),
            ContainerKind::Dispenser => ("Dispenser", 60, 6),
            ContainerKind::BrewingStand => ("Brewing Stand", 56, 6),
            ContainerKind::Enchantment => ("Enchant", 12, 6),
        };
        let mut labels = vec![(name, x, y, 4210752)];
        if !matches!(kind, ContainerKind::Player | ContainerKind::Creative) { labels.push(("Inventory", 8, h - 94, 4210752)); }
        labels
}
pub(crate) fn check_backgrounds10(out: &mut Vec<u8>) {
    use crate::gui::check10_round2::{i, s};
    use worldgen::container::{Container, InventoryPlayer};
    let mut random = worldgen::JavaRandom::new(10607);
    for c in 0..2000 {
        let rows = 1 + random.next_int(9); let width = random.next_int(1800); let height = random.next_int(1200);
        let cook = random.next_int(400)-50; let burn = random.next_int(500)-50; let fuel = random.next_int(2000);
        let kind = match c%5 { 0=>ContainerKind::Furnace, 1=>ContainerKind::Workbench, 2=>ContainerKind::Dispenser, 3=>ContainerKind::Chest { rows:rows as usize }, _=>ContainerKind::BrewingStand };
        let mut snapshot = Container::new(kind, 0).snapshot(&InventoryPlayer::default());snapshot.progress=[cook,burn,fuel];
        let h = if c%5==3 { 114+rows*18 } else { 166 };
        let rects = background_rectangles(&snapshot, (width-176)/2, (height-h)/2, 0.0);
        i(out, rects.len() as i32); for rect in rects { for n in rect { i(out,n); } }
        let name = if c%5==3 { Some(if rows==3 { "Chest" } else { "Large chest" }) } else { None };
        let labels = foreground_labels(kind, h, name);i(out,labels.len() as i32);
        for (text,x,y,color) in labels { s(out,text);for n in [x,y,color] { i(out,n); } }
    }
}
