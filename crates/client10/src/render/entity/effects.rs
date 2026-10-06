//! Render.renderShadow and Render.renderEntityOnFire.
use super::fixed::Fixed;
use worldgen::entity::Entity;
#[derive(Clone, Debug)]
pub struct Surface {
    pub xyz: [i32; 3],
    pub block: u8,
    pub brightness: f32,
}
#[derive(Clone, Debug)]
pub struct Snapshot {
    pub entity: Entity,
    pub brightness: f32,
    /// b1.8.1 `Entity.func_35115_a`: lightmap brightness at the entity (None: not lit by it).
    pub light: Option<u32>,
    pub falling_light: [f32; 7],
    pub vehicle_rails: Option<super::vehicles::RailSnapshot>,
    pub surfaces: Vec<Surface>,
    /// Packed lights a renderer samples itself: RenderPainting's tiles (in
    /// `painting::light_positions` order) or RenderFallingSand's cell.
    pub tile_lights: Vec<i32>,
    /// RenderFallingSand's dragon egg: world-lit RenderBlocks vertices.
    pub block_mesh: Vec<crate::render::Vertex>,
}
pub fn radius(e: &Entity) -> f32 {
    super::manager::renderer_for(e).shadow_size
}

pub fn shadow(e: &Entity, surfaces: &[Surface], partial: f32, camera: [f64; 3]) -> Fixed {
    shadow_with_camera(e, surfaces, partial, camera, camera)
}
pub fn shadow_at(e:&Entity,surfaces:&[Surface],partial:f32,camera:[f64;3],rel:[f64;3])->Fixed {
    shadow_at_with_camera(e, surfaces, partial, camera, rel)
}
/// The render origin and cached viewing entity position differ in third person.
pub fn shadow_with_camera(e: &Entity, surfaces: &[Surface], partial: f32, origin: [f64; 3], camera: [f64; 3]) -> Fixed {
    let p = super::bridge::position(e, partial);
    shadow_at_with_camera(e, surfaces, partial, camera, std::array::from_fn(|i| p[i] - origin[i]))
}
fn shadow_at_with_camera(e: &Entity, surfaces: &[Surface], partial: f32, camera: [f64; 3], rel: [f64; 3]) -> Fixed {
    let mut g = Fixed::new(1.0);
    let size = radius(e);
    if size <= 0.0 {
        return g;
    }
    let current = [e.pos.x, e.pos.y, e.pos.z];
    let distance: f64 = (0..3).map(|i| (current[i] - camera[i]).powi(2)).sum();
    let factor = super::manager::renderer_for(e).shadow_opacity;
    let alpha = ((1.0 - distance / 256.0) * factor as f64) as f32;
    if alpha <= 0.0 {
        return g;
    }
    let p = super::bridge::position(e, partial);
    let offset = e.height / 2.0;
    let y = p[1] + offset as f64;
    let fl = worldgen::math::floor_double;
    let lo = [fl(p[0] - size as f64), fl(y - size as f64), fl(p[2] - size as f64)];
    let hi = [fl(p[0] + size as f64), fl(y), fl(p[2] + size as f64)];
    let delta = [rel[0] - p[0], rel[1] - y + offset as f64, rel[2] - p[2]];
    g.texture = 4;
    g.enable(16, true);
    g.src = 770;
    g.dst = 771;
    g.enable(32, false);
    g.begin();
    for s in surfaces {
        let [x, y, z] = s.xyz;
        if x < lo[0] || x > hi[0] || y < lo[1] || y > hi[1] || z < lo[2] || z > hi[2] {
            continue;
        }
        let b = crate::render::rprops(s.block);
        if !b.normal {
            continue;
        }
        let a = (alpha as f64 - ((rel[1] + offset as f64) - (y as f64 + delta[1])) / 2.0) * 0.5 * s.brightness as f64;
        if a < 0.0 {
            continue;
        }
        g.color([1.0, 1.0, 1.0, a.min(1.0) as f32]);
        let x0 = x as f64 + b.bounds[0] as f64 + delta[0];
        let x1 = x as f64 + b.bounds[3] as f64 + delta[0];
        let yy = y as f64 + b.bounds[1] as f64 + delta[1] + 1.0 / 64.0;
        let z0 = z as f64 + b.bounds[2] as f64 + delta[2];
        let z1 = z as f64 + b.bounds[5] as f64 + delta[2];
        let uv = |a: f64, b: f64| ((a - b) / 2.0 / size as f64 + 0.5) as f32 as f64;
        let u0 = uv(rel[0], x0);
        let u1 = uv(rel[0], x1);
        let v0 = uv(rel[2], z0);
        let v1 = uv(rel[2], z1);
        for (p, uv) in
            [([x0, yy, z0], [u0, v0]), ([x0, yy, z1], [u0, v1]), ([x1, yy, z1], [u1, v1]), ([x1, yy, z0], [u1, v0])]
        {
            g.vertex(p, uv);
        }
    }
    g.end();
    g
}
pub fn fire(e: &Entity, partial: f32, camera: [f64; 3], view_y: f32) -> Fixed {
    let p=super::bridge::position(e,partial);let relative=std::array::from_fn(|i|p[i]-camera[i]);
    fire_at(e,relative,view_y)
}
pub fn fire_at(e:&Entity,relative:[f64;3],view_y:f32)->Fixed {
    let mut g = Fixed::new(1.0);
    if e.fire <= 0 {
        return g;
    }
    g.enable(1, false);
    g.push();
    g.translate(relative[0] as f32,relative[1] as f32,relative[2] as f32);
    let scale = e.width * 1.4;
    g.scale(scale, scale, scale);
    g.texture = 1;
    let mut width = 0.5f32;
    let mut height = e.height / scale;
    let mut y = (e.pos.y - e.bounding_box.min_y) as f32;
    g.rotate(-view_y, 0.0, 1.0, 0.0);
    g.translate(0.0, 0.0, -0.3 + (height as i32) as f32 * 0.02);
    g.material = [1.0; 4];
    let mut z = 0.0f32;
    let mut i = 0;
    g.begin();
    let tex = crate::render::rprops(51).tex_index as i32;
    let tu = (tex & 15) << 4;
    let tv = tex & 240;
    while height > 0.0 {
        let mut u0 = tu as f32 / 256.0;
        let mut u1 = (tu as f32 + 15.99) / 256.0;
        let v = (tv + if i % 2 == 0 { 0 } else { 16 }) as f32;
        let v0 = v / 256.0;
        let v1 = (v + 15.99) / 256.0;
        if i / 2 % 2 == 0 {
            std::mem::swap(&mut u0, &mut u1);
        }
        g.quad(
            [[width, 0.0 - y, z], [-width, 0.0 - y, z], [-width, 1.4 - y, z], [width, 1.4 - y, z]],
            [[u1, v1], [u0, v1], [u0, v0], [u1, v0]],
        );
        height -= 0.45;
        y -= 0.45;
        width *= 0.9;
        z += 0.03;
        i += 1;
    }
    g.end();
    g.pop();
    g.enable(1, true);
    g
}
