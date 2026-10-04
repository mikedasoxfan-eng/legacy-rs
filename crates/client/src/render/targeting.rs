//! RenderGlobal.drawSelectionBox and drawBlockBreaking, in player coordinates.
use super::{
    access::Access,
    blocks::{Colorizers, RenderBlocks},
    entity::{
        fixed::{Fixed, RawVertex},
        living::{Batch, EntityVertex, Pass},
    },
    Tessellator,
};
use glam::{Mat4, Vec3, Vec4};
use std::cell::RefCell;
use worldgen::{
    entity::{AxisAlignedBB as BB, Entity, MovingObjectPosition},
    World,
};

pub fn selection_bounds(w: &mut World, x: i32, y: i32, z: i32) -> Option<BB> {
    let id = w.get_block_id(x, y, z);
    if id == 0 {
        return None;
    }
    w.set_block_bounds_based_on_state(id, x, y, z);
    if id == 65 {
        w.block_collision_box(id, x, y, z);
    }
    let b = w.block_bounds[id as usize];
    Some(match id {
        81 => BB::new(
            (x as f32 + 0.0625) as f64,
            y as f64,
            (z as f32 + 0.0625) as f64,
            ((x + 1) as f32 - 0.0625) as f64,
            (y + 1) as f64,
            ((z + 1) as f32 - 0.0625) as f64,
        ),
        92 => {
            let m = w.get_block_metadata(x, y, z);
            BB::new(
                (x as f32 + (1 + m * 2) as f32 / 16.0) as f64,
                y as f64,
                (z as f32 + 0.0625) as f64,
                ((x + 1) as f32 - 0.0625) as f64,
                (y as f32 + 0.5) as f64,
                ((z + 1) as f32 - 0.0625) as f64,
            )
        }
        // Stairs delegate selection to their model block (a complete cube).
        53 | 67 => BB::new(
            x as f64,
            y as f64,
            z as f64,
            (x + 1) as f64,
            (y + 1) as f64,
            (z + 1) as f64,
        ),
        _ => b.translated(x as f64, y as f64, z as f64),
    })
}
pub fn selection(b: BB, camera: [f64; 3]) -> Fixed {
    let mut g = Fixed::new(1.0);
    g.enable(1, false);
    g.enable(8, false);
    g.enable(16, true);
    g.enable(32, false);
    g.src = 770;
    g.dst = 771;
    g.material = [0.0, 0.0, 0.0, 0.4];
    let b = b
        .expand(0.002f32 as f64, 0.002f32 as f64, 0.002f32 as f64)
        .translated(-camera[0], -camera[1], -camera[2]);
    let p = [
        [b.min_x, b.min_y, b.min_z],
        [b.max_x, b.min_y, b.min_z],
        [b.max_x, b.min_y, b.max_z],
        [b.min_x, b.min_y, b.max_z],
        [b.min_x, b.max_y, b.min_z],
        [b.max_x, b.max_y, b.min_z],
        [b.max_x, b.max_y, b.max_z],
        [b.min_x, b.max_y, b.max_z],
    ];
    for (mode, indices) in [
        (3, vec![0, 1, 2, 3, 0]),
        (3, vec![4, 5, 6, 7, 4]),
        (1, vec![0, 4, 1, 5, 2, 6, 3, 7]),
    ] {
        g.mode = mode;
        g.begin();
        for i in indices {
            g.vertex(p[i], [0.0; 2]);
        }
        g.untextured();
        g.end();
    }
    g
}
pub fn damage(
    w: &World,
    x: i32,
    y: i32,
    z: i32,
    progress: f32,
    camera: [f64; 3],
    col: &Colorizers,
    fancy: bool,
) -> Fixed {
    let mut g = Fixed::new(1.0);
    g.enable(1, false);
    g.enable(16, true);
    g.src = 774;
    g.dst = 768;
    g.material = [1.0, 1.0, 1.0, 0.5];
    g.texture = 1;
    if progress <= 0.0 {
        return g;
    }
    let biomes = RefCell::new(worldgen::BiomeSource::new(0));
    let mut access = Access::new(w, x >> 4, z >> 4, w.skylight_subtracted, &biomes);
    access.fancy = fancy;
    let id = access.block(x, y, z);
    let id = if id == 0 { 1 } else { id };
    let mut t = Tessellator::new(camera);
    let mut blocks = RenderBlocks::new(&access, &mut t, col);
    blocks.override_texture = Some(240 + (progress * 10.0) as i32);
    blocks.render_block(id, x, y, z);
    g.begin();
    for v in t.verts {
        g.vertices.push(RawVertex {
            pos: v.pos,
            uv: v.uv,
            color: 0,
            normal: 0,
        });
    }
    g.textured = true;
    g.end();
    g
}
pub fn frame(
    e: &Entity,
    w: &mut World,
    t: f32,
    camera: [f64; 3],
    col: &Colorizers,
    fancy: bool,
) -> (Fixed, Fixed) {
    let hit = e.mouse_over(w, t);
    if let Some(MovingObjectPosition::Block { x, y, z, .. }) = hit {
        let progress = e
            .player_state()
            .map_or(0.0, |p| p.controller.damage_partial(t));
        let cracks = damage(w, x, y, z, progress, camera, col, fancy);
        let outline =
            selection_bounds(w, x, y, z).map_or_else(|| Fixed::new(1.0), |b| selection(b, camera));
        (outline, cracks)
    } else {
        (Fixed::new(1.0), Fixed::new(1.0))
    }
}
/// Expand vanilla's width-2 line strips in screen space. Keep raw GL vertices
/// untouched for the oracle; wgpu doesn't expose glLineWidth.
pub fn line_batches(g: &Fixed, vp: Mat4, width: u32, height: u32) -> Vec<Batch> {
    let mut b = Batch {
        texture: None,
        pass: Pass::Shadow,
        vertices: vec![],
    };
    let inverse = vp.inverse();
    for d in &g.draws {
        let edges: Vec<_> = if d.mode == 3 {
            (0..d.vertices.len() - 1).map(|i| (i, i + 1)).collect()
        } else {
            (0..d.vertices.len())
                .step_by(2)
                .map(|i| (i, i + 1))
                .collect()
        };
        for (i, j) in edges {
            let mut a = vp * Vec3::from_array(d.vertices[i].pos).extend(1.0);
            let mut z = vp * Vec3::from_array(d.vertices[j].pos).extend(1.0);
            // Clip the line to wgpu's near plane before expanding its width.
            // A selected block can straddle the eye; dropping either endpoint
            // would make the visible half of that edge disappear.
            if a.z < 0.0 && z.z < 0.0 {
                continue;
            }
            if a.z < 0.0 {
                a = a.lerp(z, -a.z / (z.z - a.z));
            }
            if z.z < 0.0 {
                z = z.lerp(a, -z.z / (a.z - z.z));
            }
            if a.w <= 0.0 || z.w <= 0.0 {
                continue;
            }
            let av = a / a.w;
            let zv = z / z.w;
            let delta =
                glam::Vec2::new((zv.x - av.x) * width as f32, (zv.y - av.y) * height as f32);
            if delta.length_squared() == 0.0 {
                continue;
            }
            let n = glam::Vec2::new(-delta.y, delta.x).normalize();
            let off = Vec4::new(
                n.x * 2.0 / width as f32,
                n.y * 2.0 / height as f32,
                0.0,
                0.0,
            );
            let q = [av - off, av + off, zv + off, zv - off].map(|p| {
                let p = inverse * p;
                EntityVertex {
                    pos: (p.truncate() / p.w).to_array(),
                    uv: [0.0; 2],
                    color: d.material,
                }
            });
            for k in [0, 1, 2, 0, 2, 3] {
                b.vertices.push(q[k]);
            }
        }
    }
    vec![b]
}
