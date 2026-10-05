//! RenderPainting: each 16-pixel tile samples its own world brightness.
use crate::render::entity::fixed::Fixed;
use worldgen::{
    entity::{Entity, EntityKind, painting::ARTS},
    math::floor_double as floor,
};
pub fn light_positions(e: &Entity) -> Vec<[i32; 3]> {
    let EntityKind::Painting(s) = &e.kind else {
        return vec![];
    };
    let art = ARTS[s.art];
    let mut result = Vec::new();
    for tx in 0..art.width / 16 {
        for ty in 0..art.height / 16 {
            let xx = (-(art.width as f32) / 2.0 + (tx * 16 + 8) as f32) / 16.0;
            let yy = (-(art.height as f32) / 2.0 + (ty * 16 + 8) as f32) / 16.0;
            let (mut x, mut z) = (e.pos.x, e.pos.z);
            match s.direction {
                0 => x += xx as f64,
                1 => z -= xx as f64,
                2 => x -= xx as f64,
                3 => z += xx as f64,
                _ => {}
            }
            result.push([floor(x), floor(e.pos.y + yy as f64), floor(z)]);
        }
    }
    result
}
pub fn render(e: &Entity, relative: [f64; 3], lights: &[f32]) -> Fixed {
    let EntityKind::Painting(s) = &e.kind else {
        return Fixed::new(1.0);
    };
    let art = ARTS[s.art];
    let mut g = Fixed::new(1.0);
    g.push();
    let [x, y, z] = relative.map(|v| v as f32);
    g.translate(x, y, z);
    g.rotate(e.yaw, 0.0, 1.0, 0.0);
    g.enable(2, true);
    g.texture = 16;
    g.scale(1.0 / 16.0, 1.0 / 16.0, 1.0 / 16.0);
    for tx in 0..art.width / 16 {
        for ty in 0..art.height / 16 {
            let x1 = (-(art.width as f32) / 2.0) + ((tx + 1) * 16) as f32;
            let x0 = (-(art.width as f32) / 2.0) + (tx * 16) as f32;
            let y1 = (-(art.height as f32) / 2.0) + ((ty + 1) * 16) as f32;
            let y0 = (-(art.height as f32) / 2.0) + (ty * 16) as f32;
            let light = lights[(tx * (art.height / 16) + ty) as usize];
            g.material = [light, light, light, 1.0];
            let u0 = (art.u + art.width - tx * 16) as f32 / 256.0;
            let u1 = (art.u + art.width - (tx + 1) * 16) as f32 / 256.0;
            let v0 = (art.v + art.height - ty * 16) as f32 / 256.0;
            let v1 = (art.v + art.height - (ty + 1) * 16) as f32 / 256.0;
            g.begin();
            g.set_normal(0.0, 0.0, -1.0);
            g.quad(
                [
                    [x1, y0, -0.5],
                    [x0, y0, -0.5],
                    [x0, y1, -0.5],
                    [x1, y1, -0.5],
                ],
                [[u1, v0], [u0, v0], [u0, v1], [u1, v1]],
            );
            g.set_normal(0.0, 0.0, 1.0);
            g.quad(
                [[x1, y1, 0.5], [x0, y1, 0.5], [x0, y0, 0.5], [x1, y0, 0.5]],
                [[0.75, 0.0], [0.8125, 0.0], [0.8125, 0.0625], [0.75, 0.0625]],
            );
            g.set_normal(0.0, -1.0, 0.0);
            g.quad(
                [[x1, y1, -0.5], [x0, y1, -0.5], [x0, y1, 0.5], [x1, y1, 0.5]],
                [
                    [0.75, 0.001953125],
                    [0.8125, 0.001953125],
                    [0.8125, 0.001953125],
                    [0.75, 0.001953125],
                ],
            );
            g.set_normal(0.0, 1.0, 0.0);
            g.quad(
                [[x1, y0, 0.5], [x0, y0, 0.5], [x0, y0, -0.5], [x1, y0, -0.5]],
                [
                    [0.75, 0.001953125],
                    [0.8125, 0.001953125],
                    [0.8125, 0.001953125],
                    [0.75, 0.001953125],
                ],
            );
            g.set_normal(-1.0, 0.0, 0.0);
            g.quad(
                [[x1, y1, 0.5], [x1, y0, 0.5], [x1, y0, -0.5], [x1, y1, -0.5]],
                [
                    [0.751953125, 0.0],
                    [0.751953125, 0.0625],
                    [0.751953125, 0.0625],
                    [0.751953125, 0.0],
                ],
            );
            g.set_normal(1.0, 0.0, 0.0);
            g.quad(
                [[x0, y1, -0.5], [x0, y0, -0.5], [x0, y0, 0.5], [x0, y1, 0.5]],
                [
                    [0.751953125, 0.0],
                    [0.751953125, 0.0625],
                    [0.751953125, 0.0625],
                    [0.751953125, 0.0],
                ],
            );
            g.end();
        }
    }
    g.enable(2, false);
    g.pop();
    g
}
