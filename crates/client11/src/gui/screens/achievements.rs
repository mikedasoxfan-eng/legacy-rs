//! `GuiAchievements`: the scrollable achievement map.

use crate::gui::gl::{Factor, Shade};
use crate::gui::stats::{display_bounds, ACHIEVEMENTS};
use crate::gui::widgets::{ButtonKind, GuiButton};
use crate::gui::{Screen, ScreenBase, Ui};
use crate::render::entity::gui_item;
use worldgen::world::tile::ItemStack;
use worldgen::JavaRandom;

const PANEL_W: i32 = 256;
const PANEL_H: i32 = 202;

// Block.blockIndexInTexture values used by the background.
const SAND: i32 = 18;
const DIAMOND: i32 = 50;
const REDSTONE: i32 = 51;
const IRON: i32 = 33;
const COAL: i32 = 34;
const STONE: i32 = 1;
const DIRT: i32 = 2;
const BEDROCK: i32 = 17;

pub struct GuiAchievements {
    base: ScreenBase,
    prev_x: f64,
    prev_y: f64,
    x: f64,
    y: f64,
    target_x: f64,
    target_y: f64,
    drag_state: i32,
    last_mx: i32,
    last_my: i32,
}

/// The limits the map may scroll to.
fn limits() -> (i32, i32, i32, i32) {
    let (min_c, min_r, max_c, max_r) = display_bounds();
    (min_c * 24 - 112, min_r * 24 - 112, max_c * 24 - 77, max_r * 24 - 77)
}

impl Default for GuiAchievements {
    fn default() -> Self {
        Self::new()
    }
}

impl GuiAchievements {
    pub fn new() -> GuiAchievements {
        let (w, h) = (141, 141);
        let open = &ACHIEVEMENTS[0];
        let x = (open.column * 24 - w / 2 - 12) as f64;
        let y = (open.row * 24 - h / 2) as f64;
        GuiAchievements {
            base: ScreenBase::default(),
            prev_x: x,
            prev_y: y,
            x,
            y,
            target_x: x,
            target_y: y,
            drag_state: 0,
            last_mx: 0,
            last_my: 0,
        }
    }

    fn draw_map(&mut self, ui: &mut Ui, mx: i32, my: i32, partial: f32) {
        let (lx0, ly0, lx1, ly1) = limits();
        let mut sx = worldgen::math::floor_double(self.prev_x + (self.x - self.prev_x) * partial as f64);
        let mut sy = worldgen::math::floor_double(self.prev_y + (self.y - self.prev_y) * partial as f64);
        sx = sx.max(lx0);
        sy = sy.max(ly0);
        if sx >= lx1 {
            sx = lx1 - 1;
        }
        if sy >= ly1 {
            sy = ly1 - 1;
        }
        let terrain = ui.texture("terrain.png");
        let bg = ui.texture("achievement/bg.png");
        let (w, h) = (self.base.width, self.base.height);
        let px = (w - PANEL_W) / 2;
        let py = (h - PANEL_H) / 2;
        let ox = px + 16;
        let oy = py + 17;
        ui.gl.depth_gequal = true;
        ui.gl.push_matrix();
        ui.gl.translatef(0.0, 0.0, -200.0);
        ui.gl.texture_2d = true;
        ui.gl.bind_texture(terrain);
        let cx = (sx + 288) >> 4;
        let cy = (sy + 288) >> 4;
        let fx = (sx + 288) % 16;
        let fy = (sy + 288) % 16;
        let mut rand = JavaRandom::new(0);
        let mut row = 0;
        while row * 16 - fy < 155 {
            let shade = 0.6 - (cy + row) as f32 / 25.0 * 0.3;
            ui.gl.color4f(shade, shade, shade, 1.0);
            let mut col = 0;
            while col * 16 - fx < 224 {
                rand.set_seed((1234 + cx + col) as i64);
                rand.next_int_unbounded();
                let depth = rand.next_int(1 + cy + row) + (cy + row) / 2;
                let mut tex = SAND;
                if depth <= 37 && cy + row != 35 {
                    if depth == 22 {
                        tex = if rand.next_int(2) == 0 { DIAMOND } else { REDSTONE };
                    } else if depth == 10 {
                        tex = IRON;
                    } else if depth == 8 {
                        tex = COAL;
                    } else if depth > 4 {
                        tex = STONE;
                    } else if depth > 0 {
                        tex = DIRT;
                    }
                } else {
                    tex = BEDROCK;
                }
                ui.draw_textured_modal_rect(ox + col * 16 - fx, oy + row * 16 - fy, tex % 16 << 4, tex >> 4 << 4, 16, 16);
                col += 1;
            }
            row += 1;
        }
        ui.gl.depth_test = true;
        ui.gl.depth_gequal = false;
        ui.gl.texture_2d = false;

        let blink = |ui: &Ui| ((ui.millis % 600) as f64 / 600.0 * std::f64::consts::PI * 2.0).sin();
        for (i, a) in ACHIEVEMENTS.iter().enumerate() {
            let Some(p) = a.parent else { continue };
            let par = &ACHIEVEMENTS[p];
            let x0 = a.column * 24 - sx + 11 + ox;
            let y0 = a.row * 24 - sy + 11 + oy;
            let x1 = par.column * 24 - sx + 11 + ox;
            let y1 = par.row * 24 - sy + 11 + oy;
            let pulse = if blink(ui) > 0.6 { 255 } else { 130 };
            let c = if ui.stats.unlocked(i) {
                -9408400
            } else if ui.stats.can_unlock(i) {
                0xff00 + (pulse << 24)
            } else {
                -16777216
            };
            ui.draw_horizontal_line(x0, x1, y0, c);
            ui.draw_vertical_line(x1, y0, y1, c);
        }

        // RenderHelper.func_41089_c: -30 degrees Y, then 165 degrees X.
        // glEnable(GL_RESCALE_NORMAL)
        gui_item::set_gui_release_lighting();
        gui_item::set_gui_rescale_normal(true);
        let mut hovered = None;
        for (i, a) in ACHIEVEMENTS.iter().enumerate() {
            let rx = a.column * 24 - sx;
            let ry = a.row * 24 - sy;
            if !(rx >= -24 && ry >= -24 && rx <= 224 && ry <= 155) {
                continue;
            }
            let shade = if ui.stats.unlocked(i) {
                1.0
            } else if ui.stats.can_unlock(i) {
                if blink(ui) < 0.6 {
                    0.6
                } else {
                    0.8
                }
            } else {
                0.3
            };
            ui.gl.color4f(shade, shade, shade, 1.0);
            ui.gl.texture_2d = true;
            ui.gl.bind_texture(bg);
            let (bx, by) = (ox + rx, oy + ry);
            let u = if a.special { 26 } else { 0 };
            ui.draw_textured_modal_rect(bx - 2, by - 2, u, 202, 26, 26);
            let stack = ItemStack::new(a.icon.0 as i32, 1, a.icon.1 as i32);
            if !ui.stats.can_unlock(i) {
                ui.gl.color4f(0.1, 0.1, 0.1, 1.0);
                gui_item::render_item_into_gui_untinted(ui, Some(&stack), bx + 3, by + 3);
            } else {
                gui_item::render_item_into_gui(ui, Some(&stack), bx + 3, by + 3);
            }
            ui.gl.texture_2d = true;
            ui.gl.color4f(1.0, 1.0, 1.0, 1.0);
            if mx >= ox && my >= oy && mx < ox + 224 && my < oy + 155 && mx >= bx && mx <= bx + 22 && my >= by && my <= by + 22 {
                hovered = Some(i);
            }
        }

        gui_item::set_gui_light_rotation(0.0);
        gui_item::set_gui_rescale_normal(false);
        ui.gl.depth_test = false;
        ui.gl.blend = true;
        ui.gl.color4f(1.0, 1.0, 1.0, 1.0);
        ui.gl.bind_texture(bg);
        ui.draw_textured_modal_rect(px, py, 0, 0, PANEL_W, PANEL_H);
        ui.gl.pop_matrix();
        ui.gl.depth_gequal = false;
        ui.gl.depth_test = false;
        ui.gl.texture_2d = true;
        self.draw_buttons(ui, mx, my);

        if let Some(i) = hovered {
            let a = &ACHIEVEMENTS[i];
            let name = ui.translate(&format!("achievement.{}", a.key));
            let tx = mx + 12;
            let ty = my - 4;
            if ui.stats.can_unlock(i) {
                let desc = description(ui, i);
                let width = ui.string_width(&name).max(120);
                let mut height = ui.font.split_string_height(&desc, width);
                if ui.stats.unlocked(i) {
                    height += 12;
                }
                ui.draw_gradient_rect(tx - 3, ty - 3, tx + width + 3, ty + height + 3 + 12, -1073741824, -1073741824);
                ui.font.draw_split_string(&mut ui.gl, &desc, tx, ty + 12, width, -6250336);
                if ui.stats.unlocked(i) {
                    let taken = ui.translate("achievement.taken");
                    ui.font.draw_string_with_shadow(&mut ui.gl, &taken, tx, ty + height + 4, -7302913);
                }
            } else {
                let width = ui.string_width(&name).max(120);
                let parent = ui.translate(&format!("achievement.{}", ACHIEVEMENTS[a.parent.unwrap()].key));
                let req = ui.lang.translate_key_format("achievement.requires", &[parent]);
                let height = ui.font.split_string_height(&req, width);
                ui.draw_gradient_rect(tx - 3, ty - 3, tx + width + 3, ty + height + 12 + 3, -1073741824, -1073741824);
                ui.font.draw_split_string(&mut ui.gl, &req, tx, ty + 12, width, -9416624);
            }
            let color = match (ui.stats.can_unlock(i), a.special) {
                (true, true) => -128,
                (true, false) => -1,
                (false, true) => -8355776,
                (false, false) => -8355712,
            };
            ui.font.draw_string_with_shadow(&mut ui.gl, &name, tx, ty, color);
        }
        ui.gl.depth_test = true;
        let _ = (Shade::Flat, Factor::One);
    }
}

impl Screen for GuiAchievements {
    fn base(&mut self) -> &mut ScreenBase {
        &mut self.base
    }

    fn init_gui(&mut self, ui: &mut Ui) {
        self.base.buttons.clear();
        let (w, h) = (self.base.width, self.base.height);
        self.base.buttons.push(GuiButton {
            kind: ButtonKind::Small(None),
            ..GuiButton::sized(1, w / 2 + 24, h / 2 + 74, 80, 20, ui.translate("gui.done"))
        });
    }

    fn action_performed(&mut self, ui: &mut Ui, id: i32) {
        if id == 1 {
            ui.display(None);
        }
    }

    fn key_typed(&mut self, ui: &mut Ui, _c: char, key: i32) {
        if ui.settings.key_bindings.iter().any(|k| k.description == "key.inventory" && key == k.code()) || key == crate::gui::widgets::KEY_ESCAPE {
            ui.display(None);
        }
    }

    fn update_screen(&mut self, _ui: &mut Ui) {
        self.prev_x = self.x;
        self.prev_y = self.y;
        let dx = self.target_x - self.x;
        let dy = self.target_y - self.y;
        if dx * dx + dy * dy < 4.0 {
            self.x += dx;
            self.y += dy;
        } else {
            self.x += dx * 0.85;
            self.y += dy * 0.85;
        }
    }

    fn draw_screen(&mut self, ui: &mut Ui, mx: i32, my: i32, partial: f32) {
        let (lx0, ly0, lx1, ly1) = limits();
        if ui.mouse_down {
            let px = (self.base.width - PANEL_W) / 2;
            let py = (self.base.height - PANEL_H) / 2;
            let (ax, ay) = (px + 8, py + 17);
            if (self.drag_state == 0 || self.drag_state == 1) && mx >= ax && mx < ax + 224 && my >= ay && my < ay + 155 {
                if self.drag_state == 0 {
                    self.drag_state = 1;
                } else {
                    self.x -= (mx - self.last_mx) as f64;
                    self.y -= (my - self.last_my) as f64;
                    self.target_x = self.x;
                    self.prev_x = self.x;
                    self.target_y = self.y;
                    self.prev_y = self.y;
                }
                self.last_mx = mx;
                self.last_my = my;
            }
            if self.target_x < lx0 as f64 {
                self.target_x = lx0 as f64;
            }
            if self.target_y < ly0 as f64 {
                self.target_y = ly0 as f64;
            }
            if self.target_x >= lx1 as f64 {
                self.target_x = (lx1 - 1) as f64;
            }
            if self.target_y >= ly1 as f64 {
                self.target_y = (ly1 - 1) as f64;
            }
        } else {
            self.drag_state = 0;
        }
        let (w, h) = (self.base.width, self.base.height);
        ui.draw_world_background(w, h, 0);
        self.draw_map(ui, mx, my, partial);
        ui.gl.depth_test = false;
        let px = (w - PANEL_W) / 2;
        let py = (h - PANEL_H) / 2;
        ui.font.draw_string(&mut ui.gl, "Achievements", px + 15, py + 5, 4210752);
        ui.gl.depth_test = true;
    }
}

/// `Achievement.getDescription`: "Taking Inventory" names the inventory key
/// (`StatStringFormatKeyInv`).
pub fn description(ui: &Ui, i: usize) -> String {
    let desc = ui.translate(&format!("achievement.{}.desc", ACHIEVEMENTS[i].key));
    if i == 0 {
        let code = ui.settings.key_bindings.iter().find(|binding| binding.description == "key.inventory").unwrap().code();
        return match super::options::key_display_string(ui,code).and_then(|key|ui.lang.try_translate_key_format(&desc,&[key])) {
            Ok(formatted)=>formatted,Err(error)=>format!("Error: {}",error.message)
        };
    }
    desc
}
