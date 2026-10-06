//! RenderPlayer.renderSpecials and ItemRenderer's extruded sprite geometry.
//! The original flat/full-3D item transforms and the rotating fishing rod.

use super::inventory_model::byte_normal;
use crate::render::entity::living::{Batch, EntityVertex, Pass, pack_normal};
use crate::render::entity::models::Model;
use glam::{Mat4, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct HeldSprite {
    pub icon: i32,
    pub full_3d: bool,
    /// Non-3D blocks use terrain.png; items use gui/items.png.
    pub terrain: bool,
}

pub fn render_item(
    model: &Model,
    m: Mat4,
    item: HeldSprite,
    material: [f32; 4],
    normalize: bool,
    rotate: bool,
    lights: [Vec3; 2],
) -> (Batch, Mat4) {
    let mut gl = crate::gui::gl::Gl::new();
    gl.set_modelview(model["bipedRightArm"].post_render(m, 1.0 / 16.0));
    gl.translatef(-1.0 / 16.0, 7.0 / 16.0, 1.0 / 16.0);
    if item.full_3d {
        if rotate {
            gl.rotatef(180.0, 0.0, 0.0, 1.0);
            gl.translatef(0.0, -2.0 / 16.0, 0.0);
        }
        gl.translatef(0.0, 3.0 / 16.0, 0.0);
        gl.scalef(10.0 / 16.0, -10.0 / 16.0, 10.0 / 16.0);
        gl.rotatef(-100.0, 1.0, 0.0, 0.0);
        gl.rotatef(45.0, 0.0, 1.0, 0.0);
    } else {
        gl.translatef(0.25, 3.0 / 16.0, -3.0 / 16.0);
        gl.scalef(6.0 / 16.0, 6.0 / 16.0, 6.0 / 16.0);
        gl.rotatef(60.0, 0.0, 0.0, 1.0);
        gl.rotatef(-90.0, 1.0, 0.0, 0.0);
        gl.rotatef(20.0, 0.0, 0.0, 1.0);
    }
    // ItemRenderer.renderItem, with its original 1/16 thickness and 16 strips.
    gl.translatef(-0.0, -0.3, 0.0);
    gl.scalef(1.5, 1.5, 1.5);
    gl.rotatef(50.0, 0.0, 1.0, 0.0);
    gl.rotatef(335.0, 0.0, 0.0, 1.0);
    gl.translatef(-15.0 / 16.0, -1.0 / 16.0, 0.0);
    let m = gl.modelview_matrix();
    let u0 = (item.icon % 16 * 16) as f32 / 256.0;
    let u1 = ((item.icon % 16 * 16) as f32 + 15.99) / 256.0;
    let v0 = (item.icon / 16 * 16) as f32 / 256.0;
    let v1 = ((item.icon / 16 * 16) as f32 + 15.99) / 256.0;
    let thickness = 1.0 / 16.0;
    let inv = m.inverse();
    let normal_matrix = inv.transpose();
    let rescale = 1.0 / inv.row(2).truncate().length();
    let mut b = Batch { alpha_cutoff: 0.1, cull: 0,
        texture: Some(if item.terrain {
            "terrain.png"
        } else {
            "gui/items.png"
        }),
        pass: Pass::Opaque,
        vertices: Vec::new(),
    };
    let mut quad = |pts: [[f32; 3]; 4], uv: [[f32; 2]; 4], normal: Vec3| {
        let mut n = normal_matrix.transform_vector3(byte_normal(pack_normal(normal)));
        if normalize {
            n = n.normalize_or_zero();
        } else {
            n *= rescale;
        }
        let f = (0.4 + 0.6 * n.dot(lights[0]).max(0.0) + 0.6 * n.dot(lights[1]).max(0.0)).min(1.0);
        let color = [
            material[0] * f,
            material[1] * f,
            material[2] * f,
            material[3],
        ];
        let verts: [_; 4] = std::array::from_fn(|i| EntityVertex {
            // Tessellator submits object-space vertices. Keep the MVP on the
            // GPU, including the depth ties with the player and armor models.
            pos: pts[i],
            uv: uv[i],
            color,
        });
        for i in [0, 1, 2, 0, 2, 3] {
            b.vertices.push(verts[i]);
        }
    };
    quad(
        [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ],
        [[u1, v1], [u0, v1], [u0, v0], [u1, v0]],
        Vec3::Z,
    );
    quad(
        [
            [0.0, 1.0, -thickness],
            [1.0, 1.0, -thickness],
            [1.0, 0.0, -thickness],
            [0.0, 0.0, -thickness],
        ],
        [[u1, v0], [u0, v0], [u0, v1], [u1, v1]],
        -Vec3::Z,
    );
    for i in 0..16 {
        let f = i as f32 / 16.0;
        let u = u1 + (u0 - u1) * f - 0.001953125;
        let x = f;
        quad(
            [
                [x, 0.0, -thickness],
                [x, 0.0, 0.0],
                [x, 1.0, 0.0],
                [x, 1.0, -thickness],
            ],
            [[u, v1], [u, v1], [u, v0], [u, v0]],
            -Vec3::X,
        );
    }
    for i in 0..16 {
        let f = i as f32 / 16.0;
        let u = u1 + (u0 - u1) * f - 0.001953125;
        let x = f + 1.0 / 16.0;
        quad(
            [
                [x, 1.0, -thickness],
                [x, 1.0, 0.0],
                [x, 0.0, 0.0],
                [x, 0.0, -thickness],
            ],
            [[u, v0], [u, v0], [u, v1], [u, v1]],
            Vec3::X,
        );
    }
    for i in 0..16 {
        let f = i as f32 / 16.0;
        let v = v1 + (v0 - v1) * f - 0.001953125;
        let y = f + 1.0 / 16.0;
        quad(
            [
                [0.0, y, 0.0],
                [1.0, y, 0.0],
                [1.0, y, -thickness],
                [0.0, y, -thickness],
            ],
            [[u1, v], [u0, v], [u0, v], [u1, v]],
            Vec3::Y,
        );
    }
    for i in 0..16 {
        let f = i as f32 / 16.0;
        let v = v1 + (v0 - v1) * f - 0.001953125;
        let y = f;
        quad(
            [
                [1.0, y, 0.0],
                [0.0, y, 0.0],
                [0.0, y, -thickness],
                [1.0, y, -thickness],
            ],
            [[u0, v], [u1, v], [u1, v], [u0, v]],
            -Vec3::Y,
        );
    }
    (b, m)
}
