//! RenderItem GUI entry points, ready for hotbar/inventory callers.
use super::{fixed::Fixed, inventory, item_table};
use crate::gui::Ui;
use worldgen::world::tile::ItemStack;

pub fn geometry(item: &ItemStack, x: i32, y: i32, tint: bool) -> Fixed {
    geometry_with(item, x, y, tint, None)
}

/// With `RenderItem.field_27004_a = false` the item's own color isn't applied and the
/// caller's `glColor` stays the material (GuiAchievements darkens locked icons this way).
pub fn geometry_with(item: &ItemStack, x: i32, y: i32, tint: bool, base: Option<[f32; 4]>) -> Fixed {
    draw_item_into_gui(item, x, y, tint, base, 0.0)
}

/// `RenderItem.drawItemIntoGui` (1.0); `z_level` is `field_40268_b`.
pub fn draw_item_into_gui(item: &ItemStack, x: i32, y: i32, tint: bool, base: Option<[f32; 4]>, z_level: f32) -> Fixed {
    let mut g = Fixed::new(1.0);
    g.light_rotation = gui_light_rotation();
    g.light_transform = LIGHT_TRANSFORM.with(|c|c.get());
    // The calling screen's GL_RESCALE_NORMAL (GuiIngame/GuiContainer/GuiAchievements enable it).
    g.enable(2, gui_rescale_normal());
    if let Some(m) = base {
        g.material = m;
    }
    let (icon, color, _) = item_table::properties(item.id.try_into().expect("item ID"), item.damage);
    let rgb = |c: u32| [(c >> 16 & 255) as f32 / 255.0, (c >> 8 & 255) as f32 / 255.0, (c & 255) as f32 / 255.0, 1.0];
    if inventory::in_3d(item.id.try_into().expect("item ID")) {
        g.texture = 1;
        g.push();
        g.translate((x - 2) as f32, (y + 3) as f32, -3.0 + z_level);
        g.scale(10.0, 10.0, 10.0);
        g.translate(1.0, 0.5, 1.0);
        g.scale(1.0, 1.0, -1.0);
        g.rotate(210.0, 1.0, 0.0, 0.0);
        g.rotate(45.0, 0.0, 1.0, 0.0);
        if tint {
            g.material = rgb(color);
        }
        g.rotate(-90.0, 0.0, 1.0, 0.0);
        inventory::inventory(&mut g, item.id as u8, item.damage as i32, 1.0, tint);
        g.pop();
    } else if item.id == 373 {
        // 1.0: the tinted liquid (icon 141), then the bottle with the color reset.
        g.enable(1, false);
        g.texture = 2;
        if tint {
            g.material = rgb(color);
        }
        textured_quad(&mut g, x, y, 141, z_level);
        if tint {
            g.material = [1.0; 4];
        }
        textured_quad(&mut g, x, y, icon, z_level);
        g.enable(1, true);
    } else if icon >= 0 {
        g.enable(1, false);
        g.texture = if item.id < 256 { 1 } else { 2 };
        if tint {
            g.material = rgb(color);
        }
        textured_quad(&mut g, x, y, icon, z_level);
        g.enable(1, true);
    }
    g
}

/// `RenderItem.renderTexturedQuad` for a 16x16 icon at `field_40268_b`.
fn textured_quad(g: &mut Fixed, x: i32, y: i32, icon: i32, z: f32) {
    let u = icon % 16 * 16;
    let v = icon / 16 * 16;
    g.begin();
    g.quad(
        [
            [x as f32, (y + 16) as f32, z],
            [(x + 16) as f32, (y + 16) as f32, z],
            [(x + 16) as f32, y as f32, z],
            [x as f32, y as f32, z],
        ],
        [
            [u as f32 * 0.00390625, (v + 16) as f32 * 0.00390625],
            [(u + 16) as f32 * 0.00390625, (v + 16) as f32 * 0.00390625],
            [(u + 16) as f32 * 0.00390625, v as f32 * 0.00390625],
            [u as f32 * 0.00390625, v as f32 * 0.00390625],
        ],
    );
    g.end();
}

/// `System.currentTimeMillis() % (3000 + pass * 1873) / (3000 + pass * 1873) * 256` per glint pass.
pub fn gui_glint_offsets(millis: u64) -> [f32; 2] {
    std::array::from_fn(|pass| {
        let period = 3000 + pass as i64 * 1873;
        (millis as i64 % period) as f32 / (3000.0f32 + (pass as i32 * 1873) as f32) * 256.0
    })
}

/// `RenderItem.renderItemIntoGUI` (1.0): the icon, then the enchantment glint for items whose
/// `hasEffect` is set, drawn 50 units behind `field_40268_b` with GL_GREATER.
pub fn render_item_geometry(item: &ItemStack, x: i32, y: i32, tint: bool, base: Option<[f32; 4]>, z_level: f32, offsets: [f32; 2]) -> Fixed {
    let mut g = draw_item_into_gui(item, x, y, tint, base, z_level);
    // drawItemIntoGui ends with glEnable(GL_CULL_FACE).
    g.culling = true;
    if worldgen::item10::glint(worldgen::entity::player::ItemStack::from(*item)) {
        g.depth = 516;
        g.enable(1, false);
        g.enable(32, false);
        g.texture = 76;
        let z = z_level - 50.0;
        g.enable(16, true);
        g.src = 774;
        g.dst = 774;
        g.material = [0.5, 0.25, 0.8, 1.0];
        let (x0, y0, w, h) = (x - 2, y - 2, 20, 20);
        for (pass, offset) in offsets.into_iter().enumerate() {
            g.src = 768;
            g.dst = 1;
            let scale = if pass == 1 { -1.0f32 } else { 4.0 };
            let (k, v) = (0.00390625f32, 0.0f32);
            g.begin();
            g.vertex([x0 as f64, (y0 + h) as f64, z as f64], [((offset + h as f32 * scale) * k) as f64, ((v + h as f32) * k) as f64]);
            g.vertex([(x0 + w) as f64, (y0 + h) as f64, z as f64], [((offset + w as f32 + h as f32 * scale) * k) as f64, ((v + h as f32) * k) as f64]);
            g.vertex([(x0 + w) as f64, y0 as f64, z as f64], [((offset + w as f32) * k) as f64, ((v + 0.0) * k) as f64]);
            g.vertex([x0 as f64, y0 as f64, z as f64], [((offset + 0.0) * k) as f64, ((v + 0.0) * k) as f64]);
            g.end();
        }
        g.enable(16, false);
        g.enable(32, true);
        g.enable(1, true);
        g.depth = 515;
    }
    g
}
thread_local! {
    static LIGHT_ROTATION: std::cell::Cell<f32> = const { std::cell::Cell::new(0.0) };
    static LIGHT_TRANSFORM: std::cell::Cell<Option<glam::Mat4>> = const { std::cell::Cell::new(None) };
    static LIGHT_CAPTURE: std::cell::RefCell<Option<Vec<[[f32;4];2]>>> = const { std::cell::RefCell::new(None) };
    static RESCALE_NORMAL: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// The GL state the calling screen set up for item icons: the rotation applied before
/// `enableStandardItemLighting` and whether `GL_RESCALE_NORMAL` is on.
pub fn set_gui_light_rotation(degrees: f32) {
    LIGHT_ROTATION.with(|c| c.set(degrees));
    LIGHT_TRANSFORM.with(|c| c.set(None));
}

pub fn set_gui_rescale_normal(on: bool) {
    RESCALE_NORMAL.with(|c| c.set(on));
}

fn gui_rescale_normal() -> bool {
    RESCALE_NORMAL.with(|c| c.get())
}

fn gui_light_rotation() -> f32 {
    LIGHT_ROTATION.with(|c| c.get())
}

/// Vanilla's three durability rectangles. Counts use the GUI's existing FontRenderer.
pub fn durability(item: &ItemStack, x: i32, y: i32) -> Fixed {
    let mut g = Fixed::new(1.0);
    let (_, _, max) = item_table::properties(item.id.try_into().expect("item ID"), item.damage);
    if item.damage > 0 && max > 0 {
        let damage = item.damage as f64;
        let width = (13.0 - damage * 13.0 / max as f64 + 0.5).floor() as i32;
        let green = (255.0 - damage * 255.0 / max as f64 + 0.5).floor() as i32;
        g.enable(1, false);
        g.enable(8, false);
        g.enable(256, false);
        for (w, h, c) in
            [(13, 2, 0), (12, 1, ((255 - green) / 4 << 16) | 16128), (width, 1, ((255 - green) << 16) | (green << 8))]
        {
            g.begin();
            g.color([(c >> 16 & 255) as f32 / 255.0, (c >> 8 & 255) as f32 / 255.0, (c & 255) as f32 / 255.0, 1.0]);
            g.quad(
                [
                    [(x + 2) as f32, (y + 13) as f32, 0.0],
                    [(x + 2) as f32, (y + 13 + h) as f32, 0.0],
                    [(x + 2 + w) as f32, (y + 13 + h) as f32, 0.0],
                    [(x + 2 + w) as f32, (y + 13) as f32, 0.0],
                ],
                [[0.0; 2]; 4],
            );
            g.untextured();
            g.end();
        }
    }
    g
}
fn submit(ui: &mut Ui, g: Fixed) {
    for d in &g.draws {
        let batch = d.batch();
        ui.gl.texture_2d = batch.texture.is_some();
        if let Some(tex) = batch.texture {
            ui.gl.bind_texture(ui.texture(tex));
        }
        glint_state(ui, d, |ui| ui.gl.triangles(&batch.vertices));
    }
    ui.gl.color = g.material;
}

/// The enchantment glint's blend, depth-mask and GL_GREATER state around one draw. The glint
/// quad lies 50 units behind the icon, so GEQUAL passes exactly where GREATER does.
pub(crate) fn glint_state(ui: &mut Ui, d: &super::fixed::Draw, draw: impl FnOnce(&mut Ui)) {
    if d.texture != 76 {
        draw(ui);
        return;
    }
    use crate::gui::gl::Factor;
    let saved = (ui.gl.blend, ui.gl.blend_func, ui.gl.depth_mask, ui.gl.depth_gequal);
    ui.gl.blend = true;
    ui.gl.blend_func = (Factor::SrcColor, Factor::One);
    ui.gl.depth_mask = false;
    ui.gl.depth_gequal = true;
    draw(ui);
    (ui.gl.blend, ui.gl.blend_func, ui.gl.depth_mask, ui.gl.depth_gequal) = saved;
}
#[allow(dead_code)]
pub fn render_item_into_gui(ui: &mut Ui, item: Option<&ItemStack>, x: i32, y: i32) {
    if let Some(item) = item {
        submit(ui, render_item_geometry(item, x, y, true, None, 0.0, gui_glint_offsets(crate::render::first_person::millis())));
    }
}
/// `renderItemIntoGUI` with item tinting switched off: the current color is the material.
pub fn render_item_into_gui_untinted(ui: &mut Ui, item: Option<&ItemStack>, x: i32, y: i32) {
    if let Some(item) = item {
        let base = ui.gl.color;
        submit(ui, render_item_geometry(item, x, y, false, Some(base), 0.0, gui_glint_offsets(crate::render::first_person::millis())));
    }
}
#[allow(dead_code)]
pub fn render_item_overlay_into_gui(ui: &mut Ui, item: Option<&ItemStack>, x: i32, y: i32) {
    if let Some(item) = item {
        if item.count > 1 {
            let text = item.count.to_string();
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
        let g = durability(item, x, y);
        if !g.draws.is_empty() {
            ui.gl.depth_test = false;
            submit(ui, g);
            ui.gl.texture_2d = true;
            ui.gl.depth_test = true;
            ui.gl.color = [1.0; 4];
        }
    }
}

/// RenderHelper.func_41089_c (Release 1.0), shared by all GUI item callers.
pub fn set_gui_release_lighting() {
    let ops=[[-30.0f32,0.0,1.0,0.0],[165.0,1.0,0.0,0.0]];
    LIGHT_CAPTURE.with(|c|{if let Some(c)=c.borrow_mut().as_mut(){c.push(ops);}});
    let mut matrix=glam::Mat4::IDENTITY;
    for [angle,x,y,z] in ops {matrix*=glam::Mat4::from_axis_angle(glam::Vec3::new(x,y,z),angle.to_radians());}
    LIGHT_ROTATION.with(|c|c.set(0.0));
    LIGHT_TRANSFORM.with(|c|c.set(Some(matrix)));
}
pub(crate) fn capture_lights(){LIGHT_CAPTURE.with(|c|*c.borrow_mut()=Some(Vec::new()));}
pub(crate) fn take_light_calls()->Vec<[[f32;4];2]>{LIGHT_CAPTURE.with(|c|c.borrow_mut().take().unwrap_or_default())}
