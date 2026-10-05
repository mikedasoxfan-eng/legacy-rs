//! GuiInventory's mouse-following RenderPlayer pose, using the shared biped.
use crate::{
    gui::Ui,
    render::entity::{
        geometry::ModelRenderer,
        living::{self, EntityVertex, MobKind, RenderState},
        models::{Model, ModelKind},
    },
};
use glam::{Mat4, Vec3};

#[derive(Clone, Copy, Default)]
pub struct ArmorPassState {
    last: Option<usize>,
    attack: [f32; 2],
    riding: [bool; 2],
}

fn light(x: f32, z: f32) -> Vec3 {
    let len = ((x as f64 * x as f64 + 1.0 + z as f64 * z as f64).sqrt() as f32) as f64;
    Vec3::new(
        (x as f64 / len) as f32,
        (1.0 / len) as f32,
        (z as f64 / len) as f32,
    )
}
/// Legacy GL normal arrays expand a BYTE by replicating its bits into a
/// signed SHORT, then normalize that value. The real-client floating-point
/// framebuffer probe distinguishes this from glNormal3b's scalar conversion.
pub(super) fn byte_normal(packed: u32) -> Vec3 {
    let b = |shift| {
        let byte = (packed >> shift) as u8;
        ((byte as i8 as i32) * 256 + byte as i32) as f32 / 32767.0
    };
    Vec3::new(b(0), b(8), b(16))
}
fn part(
    ui: &mut Ui,
    part: &ModelRenderer,
    parent: Mat4,
    rotation: bool,
    lights: [Vec3; 2],
    overlay: Option<[f32; 4]>,
) {
    if part.hidden || !part.show_model {
        return;
    }
    let m = part.transform(parent, 1.0 / 16.0, rotation);
    let inverse = m.inverse();
    let normal = inverse.transpose();
    let rescale = 1.0 / inverse.row(2).truncate().length();
    let mut vertices = Vec::new();
    for face in &part.faces {
        // ModelRenderer compiles Tessellator.draw into a display list. GL captures
        // array normals as scalar glNormal3b commands, whose conversion differs
        // from a directly drawn BYTE normal array on the reference driver.
        let packed = living::packed_normal(face);
        let b = |shift| (2.0 * ((packed >> shift) as u8 as i8) as f32 + 1.0) / 255.0;
        let n = normal.transform_vector3(Vec3::new(b(0), b(8), b(16))) * rescale;
        let c = (0.4 + 0.6 * n.dot(lights[0]).max(0.0) + 0.6 * n.dot(lights[1]).max(0.0)).min(1.0);
        let v = face.vertices.map(|v| EntityVertex {
            pos: v.pos.map(|n| n as f32 / 16.0),
            uv: v.uv,
            color: overlay.map_or([c, c, c, 1.0], |v| [c * v[0], c * v[1], c * v[2], v[3]]),
        });
        for i in [0, 1, 2, 0, 2, 3] {
            vertices.push(v[i]);
        }
    }
    ui.gl.model_triangles(&vertices, m);
    for child in &part.children {
        self::part(ui, child, m, false, lights, overlay);
    }
}
pub fn draw(
    ui: &mut Ui,
    x: i32,
    y: i32,
    mouse: [f32; 2],
    snapshot: &worldgen::container::ContainerSnapshot,
) {
    let dx = (x + 51) as f32 - mouse[0];
    let dy = (y + 25) as f32 - mouse[1];
    let mut state = RenderState::new(MobKind::Biped);
    state.body_yaw = worldgen::jmath::java_atan((dx / 40.0) as f64) as f32 * 20.0;
    state.yaw = worldgen::jmath::java_atan((dx / 40.0) as f64) as f32 * 40.0;
    state.pitch = -(worldgen::jmath::java_atan((dy / 40.0) as f64) as f32) * 20.0;
    state.age = snapshot.preview.age;
    state.limb_swing = snapshot.preview.limb_swing;
    state.limb_amount = snapshot.preview.limb_amount.min(1.0);
    state.attack = snapshot.preview.attack;
    state.riding = snapshot.preview.riding;
    state.sneak = snapshot.preview.sneak;
    state.partial = 1.0;
    let mut model = Model::new(ModelKind::Biped);
    model.held_right = snapshot.held.is_some();
    model.on_ground = state.attack;
    model.is_riding = state.riding;
    model.is_sneak = state.sneak;
    model.set_rotation_angles([
        state.limb_swing,
        state.limb_amount,
        state.age,
        state.yaw - state.body_yaw,
        state.pitch,
        1.0 / 16.0,
    ]);
    ui.gl.push_matrix();
    ui.gl.translatef((x + 51) as f32, (y + 75) as f32, 50.0);
    ui.gl.scalef(-30.0, 30.0, 30.0);
    ui.gl.rotatef(180.0, 0.0, 0.0, 1.0);
    ui.gl.rotatef(135.0, 0.0, 1.0, 0.0);
    let lights = [light(0.2, -0.7), light(-0.2, 0.7)]
        .map(|l| ui.gl.modelview_matrix().transform_vector3(l).normalize());
    ui.gl.rotatef(-135.0, 0.0, 1.0, 0.0);
    ui.gl.rotatef(state.pitch, 1.0, 0.0, 0.0);
    ui.gl.translatef(0.0, 1.62, 0.0);
    ui.gl.translatef(0.0, -1.62, 0.0);
    ui.gl.rotatef(180.0 - state.body_yaw, 0.0, 1.0, 0.0);
    ui.gl.scalef(-1.0, -1.0, 1.0);
    ui.gl.scalef(15.0 / 16.0, 15.0 / 16.0, 15.0 / 16.0);
    ui.gl.translatef(0.0, -24.0 / 16.0 - 0.0078125, 0.0);
    let parent = ui.gl.modelview_matrix();
    ui.gl.push_matrix();
    ui.gl.load_identity();
    ui.gl.bind_texture(ui.texture("mob/char.png"));
    for (p, r) in model.render_parts() {
        part(ui, p, parent, r, lights, None);
    }
    const PREFIX: [&str; 5] = ["cloth", "chain", "iron", "diamond", "gold"];
    const DATA: &[u8] = include_bytes!("item_render.bin");
    let mut armor_models = Vec::new();
    let mut armor_state = ui.inventory_armor_state;
    // RenderLiving updates only the previous renderPassModel, a vanilla quirk.
    if let Some(last) = armor_state.last {
        armor_state.attack[last] = state.attack;
        armor_state.riding[last] = state.riding;
    }
    for pass in 0..4 {
        if let Some(item) = snapshot.armor[3 - pass] {
            let at = item.id as usize * 6;
            let index = if at + 6 <= DATA.len() {
                i32::from_be_bytes(DATA[at + 2..at + 6].try_into().unwrap())
            } else {
                -1
            };
            if index >= 0 {
                let mut armor = Model::with_options(
                    ModelKind::Biped,
                    if pass == 2 { 0.5 } else { 1.0 },
                    0.0,
                    12,
                );
                armor.held_right = snapshot.held.is_some();
                let armor_index = if pass == 2 { 1 } else { 0 };
                armor.on_ground = armor_state.attack[armor_index];
                armor.is_riding = armor_state.riding[armor_index];
                armor.is_sneak = state.sneak;
                armor_state.last = Some(armor_index);
                armor.set_rotation_angles([
                    state.limb_swing,
                    state.limb_amount,
                    state.age,
                    state.yaw - state.body_yaw,
                    state.pitch,
                    1.0 / 16.0,
                ]);
                armor["bipedHead"].show_model = pass == 0;
                armor["bipedHeadwear"].show_model = pass == 0;
                armor["bipedBody"].show_model = pass == 1 || pass == 2;
                for name in ["bipedRightArm", "bipedLeftArm"] {
                    armor[name].show_model = pass == 1;
                }
                for name in ["bipedRightLeg", "bipedLeftLeg"] {
                    armor[name].show_model = pass == 2 || pass == 3;
                }
                ui.gl.bind_texture(ui.texture(&format!(
                    "armor/{}_{}.png",
                    PREFIX[index as usize],
                    if pass == 2 { 2 } else { 1 }
                )));
                for (p, r) in armor.render_parts() {
                    part(ui, p, parent, r, lights, None);
                }
                armor_models.push(armor);
            }
        }
    }
    ui.inventory_armor_state = armor_state;
    if let Some(item) = snapshot.armor[3].filter(|s| s.id < 256) {
        let mut m = model["bipedHead"].post_render(parent, 1.0 / 16.0);
        if crate::render::entity::inventory::in_3d(item.id as i16) {
            m *= Mat4::from_translation(Vec3::new(0.0, -0.25, 0.0));
            m *= Mat4::from_rotation_y(std::f32::consts::PI);
            m *= Mat4::from_scale(Vec3::new(10.0 / 16.0, -10.0 / 16.0, 10.0 / 16.0));
        }
        block_item(ui, item, m, lights);
    }
    if let Some(item) = snapshot.held {
        if crate::render::entity::inventory::in_3d(item.id as i16) {
            let mut m = model["bipedRightArm"].post_render(parent, 1.0 / 16.0);
            m *= Mat4::from_translation(Vec3::new(-1.0 / 16.0, 7.0 / 16.0, 1.0 / 16.0));
            m *= Mat4::from_translation(Vec3::new(0.0, 3.0 / 16.0, -5.0 / 16.0));
            m *= Mat4::from_rotation_x(20f32.to_radians());
            m *= Mat4::from_rotation_y(45f32.to_radians());
            m *= Mat4::from_scale(Vec3::new(0.375, -0.375, 0.375));
            block_item(ui, item, m, lights);
        } else {
            let at = item.id as usize * 6;
            let (icon, _, _) =
                crate::render::entity::item_table::properties(item.id as i16, item.damage as i16);
            let held = super::inventory_equipped::HeldSprite {
                icon,
                full_3d: DATA.get(at) == Some(&1),
                terrain: item.id < 256,
            };
            let material = [1.0; 4];
            let batch = super::inventory_equipped::render_item(
                &model,
                parent,
                held,
                material,
                false,
                DATA.get(at + 1) == Some(&1),
                lights,
            );
            ui.gl.bind_texture(ui.texture(batch.texture.unwrap()));
            ui.gl.triangles(&batch.vertices);
        }
    }
    if snapshot.preview.hurt_time > 0 {
        ui.gl.texture_2d = false;
        ui.gl.alpha_test = false;
        ui.gl.blend = true;
        ui.gl.blend_func = (
            crate::gui::gl::Factor::SrcAlpha,
            crate::gui::gl::Factor::OneMinusSrcAlpha,
        );
        ui.gl.depth_equal = true;
        let color = Some([1.0, 0.0, 0.0, 0.4]);
        for (p, r) in model.render_parts() {
            part(ui, p, parent, r, lights, color);
        }
        for armor in &armor_models {
            for (p, r) in armor.render_parts() {
                part(ui, p, parent, r, lights, color);
            }
        }
        ui.gl.depth_equal = false;
        ui.gl.blend = false;
        ui.gl.alpha_test = true;
        ui.gl.texture_2d = true;
    }
    ui.gl.pop_matrix();
    ui.gl.pop_matrix();
}
fn block_item(ui: &mut Ui, item: worldgen::container::ItemStack, parent: Mat4, lights: [Vec3; 2]) {
    let mut fixed = crate::render::entity::fixed::Fixed::new(1.0);
    fixed.enable(2, true);
    fixed.texture = 1;
    crate::render::entity::inventory::inventory(&mut fixed, item.id as u8, item.damage, 1.0, true);
    for d in &fixed.draws {
        let m = parent * d.matrix();
        let inv = m.inverse();
        let nm = inv.transpose();
        let rescale = 1.0 / inv.row(2).truncate().length();
        let mut vertices = Vec::new();
        for raw in d.vertices.chunks_exact(4) {
            for i in [0, 1, 2, 0, 2, 3] {
                let n = nm.transform_vector3(if d.flags & 128 != 0 {
                    byte_normal(raw[i].normal)
                } else {
                    Vec3::from_array(d.normal)
                }) * rescale;
                let f = (0.4 + 0.6 * n.dot(lights[0]).max(0.0) + 0.6 * n.dot(lights[1]).max(0.0))
                    .min(1.0);
                vertices.push(EntityVertex {
                    pos: m.transform_point3(Vec3::from_array(raw[i].pos)).to_array(),
                    uv: raw[i].uv,
                    color: [d.material[0] * f, d.material[1] * f, d.material[2] * f, 1.0],
                });
            }
        }
        ui.gl.bind_texture(ui.texture("terrain.png"));
        ui.gl.triangles(&vertices);
    }
}
