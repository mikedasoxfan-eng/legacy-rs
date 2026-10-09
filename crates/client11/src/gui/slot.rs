//! `GuiSlot`: the scrolling list used by the world and texture-pack screens.

use super::gl::{Factor, Shade};
use super::Ui;

#[path = "slot_check8.rs"]
mod check8;
pub use check8::check;

/// The subclass callbacks (`getSize`, `elementClicked`, ...).
pub trait SlotList {
    fn size(&self) -> i32;
    fn element_clicked(&mut self, ui: &mut Ui, index: i32, double_click: bool);
    fn is_selected(&self, index: i32) -> bool;
    /// `getContentHeight` override, if any.
    fn content_height(&self) -> Option<i32> {
        None
    }
    fn draw_background(&mut self, ui: &mut Ui);
    fn draw_slot(&mut self, ui: &mut Ui, index: i32, x: i32, y: i32, h: i32);
    /// `func_27260_a`: draw the header row (when the slot has one).
    fn draw_header(&mut self, _ui: &mut Ui, _x: i32, _y: i32) {}
    /// `func_27255_a`: a click in the header, relative to the list's left edge and top.
    fn header_clicked(&mut self, _ui: &mut Ui, _x: i32, _y: i32) {}
    /// `func_27257_b`: drawn last, over everything (tooltips).
    fn draw_overlay(&mut self, _ui: &mut Ui, _slot: &GuiSlot, _mx: i32, _my: i32) {}
}

pub struct GuiSlot {
    width: i32,
    height: i32,
    pub top: i32,
    pub bottom: i32,
    right: i32,
    left: i32,
    pub slot_height: i32,
    /// GuiSlot's protected mouse coordinates, assigned before subclass callbacks.
    pub mouse_x: i32,
    pub mouse_y: i32,
    scroll_up_id: i32,
    scroll_down_id: i32,
    initial_click_y: f32,
    scroll_multiplier: f32,
    amount_scrolled: f32,
    selected_element: i32,
    last_clicked: i64,
    show_selection: bool,
    has_header: bool,
    header_height: i32,
}

impl GuiSlot {
    pub(crate) fn check_wheel10(ui: &mut Ui, out: &mut Vec<u8>) {
        use crate::gui::check10_round2::i;
        struct List(i32);
        impl SlotList for List {
            fn size(&self) -> i32 { self.0 }
            fn element_clicked(&mut self, _: &mut Ui, _: i32, _: bool) {}
            fn is_selected(&self, _: i32) -> bool { false }
            fn draw_background(&mut self, _: &mut Ui) {}
            fn draw_slot(&mut self, _: &mut Ui, _: i32, _: i32, _: i32, _: i32) {}
        }
        ui.textures.insert("gui/background.png".into(), 0);
        let mut random = worldgen::JavaRandom::new(11011);
        for _ in 0..1000 {
            let width=320+random.next_int(800);let height=200+random.next_int(500);let top=20+random.next_int(40);let bottom=height-30;let sh=8+random.next_int(60);
            let mut list=List(random.next_int(40));let mut slot=Self::new(width,height,top,bottom,sh);ui.release.wheel_events.clear();
            for _ in 0..8 {
                let wheel=random.next_int(1001)-500;ui.mouse_down=random.next_bool();ui.release.wheel_events.push(wheel);
                slot.initial_click_y=-2.0;ui.gl.begin_frame();slot.draw_screen(ui,&mut list,0,0);
                out.extend(slot.amount_scrolled.to_le_bytes());out.extend(slot.initial_click_y.to_le_bytes());i(out,slot.slot_at(&list,width/2,top+sh/2));
            }
        }
    }
    pub fn new(width: i32, height: i32, top: i32, bottom: i32, slot_height: i32) -> GuiSlot {
        GuiSlot {
            width,
            height,
            top,
            bottom,
            right: width,
            left: 0,
            slot_height,
            mouse_x: 0,
            mouse_y: 0,
            scroll_up_id: 0,
            scroll_down_id: 0,
            initial_click_y: -2.0,
            scroll_multiplier: 0.0,
            amount_scrolled: 0.0,
            selected_element: -1,
            last_clicked: 0,
            show_selection: true,
            has_header: false,
            header_height: 0,
        }
    }

    /// `func_27258_a`: draw the selection box around selected entries.
    pub fn set_show_selection(&mut self, on: bool) {
        self.show_selection = on;
    }

    /// `func_27259_a`: a header row of `height` above the entries.
    pub fn set_header(&mut self, on: bool, height: i32) {
        self.has_header = on;
        self.header_height = if on { height } else { 0 };
    }

    /// `func_27256_c`: the entry under the mouse, or -1.
    pub fn slot_at(&self, l: &dyn SlotList, mx: i32, my: i32) -> i32 {
        let x0 = self.width / 2 - 110;
        let x1 = self.width / 2 + 110;
        let rel = my - self.top - self.header_height + self.amount_scrolled as i32 - 4;
        let idx = rel / self.slot_height;
        if mx >= x0 && mx <= x1 && idx >= 0 && rel >= 0 && idx < l.size() {
            idx
        } else {
            -1
        }
    }

    pub fn register_scroll_buttons(&mut self, up: i32, down: i32) {
        self.scroll_up_id = up;
        self.scroll_down_id = down;
    }

    fn content_height(&self, l: &dyn SlotList) -> i32 {
        l.content_height().unwrap_or(l.size() * self.slot_height + self.header_height)
    }

    fn bind_amount_scrolled(&mut self, l: &dyn SlotList) {
        let mut max = self.content_height(l) - (self.bottom - self.top - 4);
        if max < 0 {
            max /= 2;
        }
        if self.amount_scrolled < 0.0 {
            self.amount_scrolled = 0.0;
        }
        if self.amount_scrolled > max as f32 {
            self.amount_scrolled = max as f32;
        }
    }

    pub fn action_performed(&mut self, l: &dyn SlotList, id: i32, enabled: bool) {
        if !enabled {
            return;
        }
        if id == self.scroll_up_id {
            self.amount_scrolled -= (self.slot_height * 2 / 3) as f32;
            self.initial_click_y = -2.0;
            self.bind_amount_scrolled(l);
        } else if id == self.scroll_down_id {
            self.amount_scrolled += (self.slot_height * 2 / 3) as f32;
            self.initial_click_y = -2.0;
            self.bind_amount_scrolled(l);
        }
    }

    pub fn draw_screen(&mut self, ui: &mut Ui, l: &mut dyn SlotList, mx: i32, my: i32) {
        self.mouse_x = mx;
        self.mouse_y = my;
        l.draw_background(ui);
        let size = l.size();
        let bar0 = self.width / 2 + 124;
        let bar1 = bar0 + 6;
        if ui.mouse_down {
            if self.initial_click_y == -1.0 {
                let mut drag = true;
                if my >= self.top && my <= self.bottom {
                    let x0 = self.width / 2 - 110;
                    let x1 = self.width / 2 + 110;
                    let rel = my - self.top - self.header_height + self.amount_scrolled as i32 - 4;
                    let idx = rel / self.slot_height;
                    if mx >= x0 && mx <= x1 && idx >= 0 && rel >= 0 && idx < size {
                        let dbl = idx == self.selected_element && ui.millis - self.last_clicked < 250;
                        l.element_clicked(ui, idx, dbl);
                        self.selected_element = idx;
                        self.last_clicked = ui.millis;
                    } else if mx >= x0 && mx <= x1 && rel < 0 {
                        l.header_clicked(ui, mx - x0, my - self.top + self.amount_scrolled as i32 - 4);
                        drag = false;
                    }
                    if mx >= bar0 && mx <= bar1 {
                        self.scroll_multiplier = -1.0;
                        let mut max = self.content_height(l) - (self.bottom - self.top - 4);
                        if max < 1 {
                            max = 1;
                        }
                        let mut thumb = (((self.bottom - self.top) * (self.bottom - self.top)) as f32 / self.content_height(l) as f32) as i32;
                        thumb = thumb.max(32).min(self.bottom - self.top - 8);
                        self.scroll_multiplier /= (self.bottom - self.top - thumb) as f32 / max as f32;
                    } else {
                        self.scroll_multiplier = 1.0;
                    }
                    self.initial_click_y = if drag { my as f32 } else { -2.0 };
                } else {
                    self.initial_click_y = -2.0;
                }
            } else if self.initial_click_y >= 0.0 {
                self.amount_scrolled -= (my as f32 - self.initial_click_y) * self.scroll_multiplier;
                self.initial_click_y = my as f32;
            }
        } else {
            for wheel in ui.release.wheel_events.drain(..) {
                self.amount_scrolled += (-wheel.signum() * self.slot_height / 2) as f32;
            }
            self.initial_click_y = -1.0;
        }
        self.bind_amount_scrolled(l);

        let bg = ui.texture("gui/background.png");
        let s = 32.0f32;
        let scrolled = self.amount_scrolled as i32;
        {
            let gl = &mut ui.gl;
            gl.bind_texture(bg);
            gl.color4f(1.0, 1.0, 1.0, 1.0);
            gl.start_drawing_quads();
            gl.set_color_opaque_i(2105376);
            let (l0, r0, t0, b0) = (self.left, self.right, self.top, self.bottom);
            gl.add_vertex_with_uv(l0 as f64, b0 as f64, 0.0, (l0 as f32 / s) as f64, ((b0 + scrolled) as f32 / s) as f64);
            gl.add_vertex_with_uv(r0 as f64, b0 as f64, 0.0, (r0 as f32 / s) as f64, ((b0 + scrolled) as f32 / s) as f64);
            gl.add_vertex_with_uv(r0 as f64, t0 as f64, 0.0, (r0 as f32 / s) as f64, ((t0 + scrolled) as f32 / s) as f64);
            gl.add_vertex_with_uv(l0 as f64, t0 as f64, 0.0, (l0 as f32 / s) as f64, ((t0 + scrolled) as f32 / s) as f64);
            gl.draw();
        }
        let x = self.width / 2 - 92 - 16;
        let y0 = self.top + 4 - scrolled;
        if self.has_header {
            l.draw_header(ui, x, y0);
        }
        for i in 0..size {
            let y = y0 + i * self.slot_height + self.header_height;
            let h = self.slot_height - 4;
            if y <= self.bottom && y + h >= self.top {
                if self.show_selection && l.is_selected(i) {
                    let (x0, x1) = (self.width / 2 - 110, self.width / 2 + 110);
                    let gl = &mut ui.gl;
                    gl.color4f(1.0, 1.0, 1.0, 1.0);
                    gl.texture_2d = false;
                    gl.start_drawing_quads();
                    gl.set_color_opaque_i(8421504);
                    gl.add_vertex_with_uv(x0 as f64, (y + h + 2) as f64, 0.0, 0.0, 1.0);
                    gl.add_vertex_with_uv(x1 as f64, (y + h + 2) as f64, 0.0, 1.0, 1.0);
                    gl.add_vertex_with_uv(x1 as f64, (y - 2) as f64, 0.0, 1.0, 0.0);
                    gl.add_vertex_with_uv(x0 as f64, (y - 2) as f64, 0.0, 0.0, 0.0);
                    gl.set_color_opaque_i(0);
                    gl.add_vertex_with_uv((x0 + 1) as f64, (y + h + 1) as f64, 0.0, 0.0, 1.0);
                    gl.add_vertex_with_uv((x1 - 1) as f64, (y + h + 1) as f64, 0.0, 1.0, 1.0);
                    gl.add_vertex_with_uv((x1 - 1) as f64, (y - 1) as f64, 0.0, 1.0, 0.0);
                    gl.add_vertex_with_uv((x0 + 1) as f64, (y - 1) as f64, 0.0, 0.0, 0.0);
                    gl.draw();
                    gl.texture_2d = true;
                }
                l.draw_slot(ui, i, x, y, h);
            }
        }
        ui.gl.depth_test = false;
        let edge = 4;
        self.overlay_background(ui, 0, self.top, 255, 255);
        self.overlay_background(ui, self.bottom, self.height, 255, 255);
        let content = self.content_height(l);
        let (left, right, top, bottom) = (self.left, self.right, self.top, self.bottom);
        let gl = &mut ui.gl;
        gl.blend = true;
        gl.blend_func(Factor::SrcAlpha, Factor::OneMinusSrcAlpha);
        gl.alpha_test = false;
        gl.shade = Shade::Smooth;
        gl.texture_2d = false;
        gl.start_drawing_quads();
        gl.set_color_rgba_i(0, 0);
        gl.add_vertex_with_uv(left as f64, (top + edge) as f64, 0.0, 0.0, 1.0);
        gl.add_vertex_with_uv(right as f64, (top + edge) as f64, 0.0, 1.0, 1.0);
        gl.set_color_rgba_i(0, 255);
        gl.add_vertex_with_uv(right as f64, top as f64, 0.0, 1.0, 0.0);
        gl.add_vertex_with_uv(left as f64, top as f64, 0.0, 0.0, 0.0);
        gl.draw();
        gl.start_drawing_quads();
        gl.set_color_rgba_i(0, 255);
        gl.add_vertex_with_uv(left as f64, bottom as f64, 0.0, 0.0, 1.0);
        gl.add_vertex_with_uv(right as f64, bottom as f64, 0.0, 1.0, 1.0);
        gl.set_color_rgba_i(0, 0);
        gl.add_vertex_with_uv(right as f64, (bottom - edge) as f64, 0.0, 1.0, 0.0);
        gl.add_vertex_with_uv(left as f64, (bottom - edge) as f64, 0.0, 0.0, 0.0);
        gl.draw();
        let max = content - (bottom - top - 4);
        if max > 0 {
            let mut thumb = (bottom - top) * (bottom - top) / content;
            thumb = thumb.max(32).min(bottom - top - 8);
            let mut ty = scrolled * (bottom - top - thumb) / max + top;
            if ty < top {
                ty = top;
            }
            gl.start_drawing_quads();
            gl.set_color_rgba_i(0, 255);
            gl.add_vertex_with_uv(bar0 as f64, bottom as f64, 0.0, 0.0, 1.0);
            gl.add_vertex_with_uv(bar1 as f64, bottom as f64, 0.0, 1.0, 1.0);
            gl.add_vertex_with_uv(bar1 as f64, top as f64, 0.0, 1.0, 0.0);
            gl.add_vertex_with_uv(bar0 as f64, top as f64, 0.0, 0.0, 0.0);
            gl.draw();
            gl.start_drawing_quads();
            gl.set_color_rgba_i(8421504, 255);
            gl.add_vertex_with_uv(bar0 as f64, (ty + thumb) as f64, 0.0, 0.0, 1.0);
            gl.add_vertex_with_uv(bar1 as f64, (ty + thumb) as f64, 0.0, 1.0, 1.0);
            gl.add_vertex_with_uv(bar1 as f64, ty as f64, 0.0, 1.0, 0.0);
            gl.add_vertex_with_uv(bar0 as f64, ty as f64, 0.0, 0.0, 0.0);
            gl.draw();
            gl.start_drawing_quads();
            gl.set_color_rgba_i(12632256, 255);
            gl.add_vertex_with_uv(bar0 as f64, (ty + thumb - 1) as f64, 0.0, 0.0, 1.0);
            gl.add_vertex_with_uv((bar1 - 1) as f64, (ty + thumb - 1) as f64, 0.0, 1.0, 1.0);
            gl.add_vertex_with_uv((bar1 - 1) as f64, ty as f64, 0.0, 1.0, 0.0);
            gl.add_vertex_with_uv(bar0 as f64, ty as f64, 0.0, 0.0, 0.0);
            gl.draw();
        }
        l.draw_overlay(ui, self, mx, my);
        let gl = &mut ui.gl;
        gl.texture_2d = true;
        gl.shade = Shade::Flat;
        gl.alpha_test = true;
        gl.blend = false;
    }

    fn overlay_background(&self, ui: &mut Ui, y0: i32, y1: i32, a0: i32, a1: i32) {
        let t = ui.texture("gui/background.png");
        let gl = &mut ui.gl;
        gl.bind_texture(t);
        gl.color4f(1.0, 1.0, 1.0, 1.0);
        let s = 32.0f32;
        gl.start_drawing_quads();
        gl.set_color_rgba_i(4210752, a1);
        gl.add_vertex_with_uv(0.0, y1 as f64, 0.0, 0.0, (y1 as f32 / s) as f64);
        gl.add_vertex_with_uv(self.width as f64, y1 as f64, 0.0, (self.width as f32 / s) as f64, (y1 as f32 / s) as f64);
        gl.set_color_rgba_i(4210752, a0);
        gl.add_vertex_with_uv(self.width as f64, y0 as f64, 0.0, (self.width as f32 / s) as f64, (y0 as f32 / s) as f64);
        gl.add_vertex_with_uv(0.0, y0 as f64, 0.0, 0.0, (y0 as f32 / s) as f64);
        gl.draw();
    }
}
