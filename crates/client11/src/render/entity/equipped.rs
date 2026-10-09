//! `RenderBiped.renderEquippedItems` and ItemRenderer's extruded item-sprite path.
//! This covers the skeleton's bow and flat/full-3D held items. Inventory block
//! geometry can attach using ModelRenderer.post_render when that renderer is ported.

use super::living::{lit_color, normal_from_packed, pack_normal, Batch, EntityVertex, Pass};
use super::models::Model;
use glam::{Mat4, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct HeldSprite {
    pub icon: i32,
    pub full_3d: bool,
    /// Non-3D blocks use terrain.png; items use gui/items.png.
    pub terrain: bool,
}

impl HeldSprite {
    pub const BOW: Self = Self { icon: 21, full_3d: false, terrain: false };
}

pub fn render_item(model: &Model, mut m: Mat4, item: HeldSprite, material: [f32; 4], normalize: bool) -> Batch {
    m = model["bipedRightArm"].post_render(m, 1.0 / 16.0);
    m *= Mat4::from_translation(Vec3::new(-1.0 / 16.0, 7.0 / 16.0, 1.0 / 16.0));
    if item.full_3d {
        m *= Mat4::from_translation(Vec3::new(0.0, 3.0 / 16.0, 0.0));
        m *= Mat4::from_scale(Vec3::new(10.0 / 16.0, -10.0 / 16.0, 10.0 / 16.0));
        m *= Mat4::from_rotation_x((-100f32).to_radians());
        m *= Mat4::from_rotation_y(45f32.to_radians());
    } else {
        m *= Mat4::from_translation(Vec3::new(0.25, 3.0 / 16.0, -3.0 / 16.0));
        m *= Mat4::from_scale(Vec3::splat(6.0 / 16.0));
        m *= Mat4::from_rotation_z(60f32.to_radians());
        m *= Mat4::from_rotation_x((-90f32).to_radians());
        m *= Mat4::from_rotation_z(20f32.to_radians());
    }
    // ItemRenderer.renderItem, with its original 1/16 thickness and 16 strips.
    m *= Mat4::from_translation(Vec3::new(-0.0, -0.3, 0.0));
    m *= Mat4::from_scale(Vec3::splat(1.5));
    m *= Mat4::from_rotation_y(50f32.to_radians());
    m *= Mat4::from_rotation_z(335f32.to_radians());
    m *= Mat4::from_translation(Vec3::new(-15.0 / 16.0, -1.0 / 16.0, 0.0));
    let u0 = (item.icon % 16 * 16) as f32 / 256.0;
    let u1 = ((item.icon % 16 * 16) as f32 + 15.99) / 256.0;
    let v0 = (item.icon / 16 * 16) as f32 / 256.0;
    let v1 = ((item.icon / 16 * 16) as f32 + 15.99) / 256.0;
    let thickness = 1.0 / 16.0;
    let inv = m.inverse();
    let normal_matrix = inv.transpose();
    let rescale = 1.0 / inv.row(2).truncate().length();
    let mut b = Batch { alpha_cutoff: 0.1, cull: 0,
        texture: Some(if item.terrain { "terrain.png" } else { "gui/items.png" }),
        pass: Pass::Opaque,
        vertices: Vec::new(),
    };
    let mut quad = |pts: [[f32; 3]; 4], uv: [[f32; 2]; 4], normal: Vec3| {
        let mut n = normal_matrix.transform_vector3(normal_from_packed(pack_normal(normal)));
        if normalize {
            n = n.normalize_or_zero();
        } else {
            n *= rescale;
        }
        let color = lit_color(n, material);
        let verts: [_; 4] = std::array::from_fn(|i| EntityVertex {
            pos: m.transform_point3(Vec3::from_array(pts[i])).to_array(),
            uv: uv[i],
            color,
        });
        for i in [0, 1, 2, 0, 2, 3] {
            b.vertices.push(verts[i]);
        }
    };
    quad(
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]],
        [[u1, v1], [u0, v1], [u0, v0], [u1, v0]],
        Vec3::Z,
    );
    quad(
        [[0.0, 1.0, -thickness], [1.0, 1.0, -thickness], [1.0, 0.0, -thickness], [0.0, 0.0, -thickness]],
        [[u1, v0], [u0, v0], [u0, v1], [u1, v1]],
        -Vec3::Z,
    );
    for i in 0..16 {
        let f = i as f32 / 16.0;
        let u = u1 + (u0 - u1) * f - 0.001953125;
        let x = f;
        quad(
            [[x, 0.0, -thickness], [x, 0.0, 0.0], [x, 1.0, 0.0], [x, 1.0, -thickness]],
            [[u, v1], [u, v1], [u, v0], [u, v0]],
            -Vec3::X,
        );
    }
    for i in 0..16 {
        let f = i as f32 / 16.0;
        let u = u1 + (u0 - u1) * f - 0.001953125;
        let x = f + 1.0 / 16.0;
        quad(
            [[x, 1.0, -thickness], [x, 1.0, 0.0], [x, 0.0, 0.0], [x, 0.0, -thickness]],
            [[u, v0], [u, v0], [u, v1], [u, v1]],
            Vec3::X,
        );
    }
    for i in 0..16 {
        let f = i as f32 / 16.0;
        let v = v1 + (v0 - v1) * f - 0.001953125;
        let y = f + 1.0 / 16.0;
        quad(
            [[0.0, y, 0.0], [1.0, y, 0.0], [1.0, y, -thickness], [0.0, y, -thickness]],
            [[u1, v], [u0, v], [u0, v], [u1, v]],
            Vec3::Y,
        );
    }
    for i in 0..16 {
        let f = i as f32 / 16.0;
        let v = v1 + (v0 - v1) * f - 0.001953125;
        let y = f;
        quad(
            [[1.0, y, 0.0], [0.0, y, 0.0], [0.0, y, -thickness], [1.0, y, -thickness]],
            [[u0, v], [u1, v], [u1, v], [u0, v]],
            -Vec3::Y,
        );
    }
    b
}
