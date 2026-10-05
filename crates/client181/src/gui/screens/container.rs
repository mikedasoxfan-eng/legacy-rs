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
    fn origin(&self) -> (i32, i32) {
        let (w, h) = self.size();
        ((self.base.width - w) / 2, (self.base.height - h) / 2)
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
        };
        ui.gl.bind_texture(ui.texture(texture));
        if let ContainerKind::Chest { rows } = self.kind {
            ui.draw_textured_modal_rect(x, y, 0, 0, w, rows as i32 * 18 + 17);
            ui.draw_textured_modal_rect(x, y + rows as i32 * 18 + 17, 0, 126, w, 96);
        } else {
            ui.draw_textured_modal_rect(x, y, 0, 0, w, h);
        }
        if self.kind == ContainerKind::Furnace {
            let [cook, burn, fuel] = s.progress;
            if burn > 0 {
                let n = burn.wrapping_mul(12) / if fuel == 0 { 200 } else { fuel };
                ui.draw_textured_modal_rect(x + 56, y + 48 - n, 176, 12 - n, 14, n + 2);
            }
            let n = cook.wrapping_mul(24) / 200;
            ui.draw_textured_modal_rect(x + 79, y + 34, 176, 14, n + 1, 16);
        }
        if self.kind == ContainerKind::Player {
            super::inventory_model::draw(ui, x, y, self.previous_mouse, s);
        }
    }
    fn foreground(&self, ui: &mut Ui) {
        let (_, h) = self.size();
        let (name, x, y) = match self.kind {
            ContainerKind::Player => ("Crafting", 86, 16),
            ContainerKind::Creative => ("Item selection",8,6),
            ContainerKind::Workbench => ("Crafting", 28, 6),
            ContainerKind::Chest { rows } => {
                (self.inventory_name.unwrap_or(if rows == 3 { "Chest" } else { "Large chest" }), 8, 6)
            }
            ContainerKind::Furnace => ("Furnace", 60, 6),
            ContainerKind::Dispenser => ("Dispenser", 60, 6),
        };
        ui.font.draw_string(&mut ui.gl, name, x, y, 4210752);
        if !matches!(self.kind,ContainerKind::Player|ContainerKind::Creative) {
            ui.font
                .draw_string(&mut ui.gl, "Inventory", 8, h - 94, 4210752);
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
                let name = ui.lang.translate_named_key(item_name(stack));
                let name = name.trim();
                if !name.is_empty() {
                    let x = mx - ox + 12;
                    let y = my - oy - 12;
                    let w = ui.font.get_string_width(name);
                    ui.draw_gradient_rect(
                        x - 3,
                        y - 3,
                        x + w + 3,
                        y + 11,
                        -1073741824,
                        -1073741824,
                    );
                    ui.font.draw_string_with_shadow(&mut ui.gl, name, x, y, -1);
                }
            }
        }
        ui.gl.pop_matrix();
        self.draw_buttons(ui, mx, my);
        ui.gl.depth_test = true;
        let _ = partial;
        if self.kind==ContainerKind::Creative {
            ui.gl.color4f(1.0,1.0,1.0,1.0);ui.gl.bind_texture(ui.texture("gui/allitems.png"));
            ui.draw_textured_modal_rect(ox+154,oy+17+(145.0*self.scroll) as i32,0,208,16,16);
        }
        self.previous_mouse = [mx as f32, my as f32];
    }
    fn mouse_clicked(&mut self, ui: &mut Ui, x: i32, y: i32, button: i32) {
        crate::gui::default_mouse_clicked(self, ui, x, y, button);
        if button != 0 && button != 1 {
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
