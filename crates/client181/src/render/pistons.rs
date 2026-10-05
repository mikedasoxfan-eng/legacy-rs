//! TileEntityRendererPiston. The worker renders the two arm lengths once; the
//! window applies the original float translation before rounding vertex doubles.
use super::{
    access::Access,
    blocks::{Colorizers, RenderBlocks},
    Tessellator, Vertex,
};
use worldgen::world::pistons::Piston;

#[derive(Clone)]
pub struct Geometry {
    positions: Vec<[f64; 3]>,
    vertices: Vec<Vertex>,
}
#[derive(Clone)]
pub struct Snapshot {
    pub position: [i32; 3],
    pub piston: Piston,
    early: Geometry,
    late: Geometry,
    base: Option<Geometry>,
}
fn geometry(
    w: &Access,
    col: &Colorizers,
    ao: bool,
    position: [i32; 3],
    p: &Piston,
    early: bool,
    base: bool,
) -> Geometry {
    let [x, y, z] = position;
    let mut t = Tessellator::new([0.0; 3]);
    t.raw_positions = Some(Vec::new());
    let mut r = RenderBlocks::new(w, &mut t, col);
    r.ao = ao;
    r.render_all_faces = true;
    if base {
        r.render_piston_base(p.block_id.try_into().expect("piston block ID"), x, y, z, true);
    } else if p.block_id == 34 && early {
        r.render_piston_head(34, x, y, z, false);
    } else if p.render_head && !p.extending {
        r.piston_head_texture = Some(if p.block_id == 29 { 106 } else { 107 });
        r.render_piston_head(34, x, y, z, early);
    } else {
        r.render_block(p.block_id.try_into().expect("piston block ID"), x, y, z);
    }
    Geometry {
        positions: t.raw_positions.unwrap(),
        vertices: t.verts,
    }
}
impl Snapshot {
    pub fn new(w: &Access, col: &Colorizers, ao: bool, position: [i32; 3], piston: Piston) -> Self {
        let early = geometry(w, col, ao, position, &piston, true, false);
        let late = geometry(w, col, ao, position, &piston, false, false);
        let base = (piston.render_head && !piston.extending)
            .then(|| geometry(w, col, ao, position, &piston, false, true));
        Self {
            position,
            piston,
            early,
            late,
            base,
        }
    }
    /// The original Tessellator output at the camera-relative render position.
    pub fn vertices(&self, partial: f32, render_position: [f64; 3]) -> Vec<Vertex> {
        let p = &self.piston;
        if p.block_id == 0 || p.progress(partial) >= 1.0 {
            return Vec::new();
        }
        let shift = p.offset(partial);
        let translation = std::array::from_fn::<_, 3, _>(|i| {
            ((render_position[i] as f32 - self.position[i] as f32) + shift[i]) as f64
        });
        let mut verts = Vec::new();
        let mesh = if p.progress(partial) < 0.5 {
            &self.early
        } else {
            &self.late
        };
        append(&mut verts, mesh, translation);
        if let Some(base) = &self.base {
            append(
                &mut verts,
                base,
                std::array::from_fn(|i| {
                    (render_position[i] as f32 - self.position[i] as f32) as f64
                }),
            );
        }
        verts
    }
    pub fn visible(&self, camera: [f64; 3]) -> bool {
        let d: [f64; 3] = std::array::from_fn(|i| self.position[i] as f64 + 0.5 - camera[i]);
        d[0] * d[0] + d[1] * d[1] + d[2] * d[2] < 4096.0
    }
    /// The GPU batch (quads split for drawing) and each vertex's lightmap brightness.
    pub fn batch(&self, partial: f32, camera: [f64; 3]) -> (super::entity::living::Batch, Vec<u32>) {
        use super::entity::living::{Batch, EntityVertex, Pass};
        let render_position = std::array::from_fn(|i| self.position[i] as f64 - camera[i]);
        let quads = self.vertices(partial, render_position);
        let mut vertices = Vec::with_capacity(quads.len() / 4 * 6);
        let mut lights = Vec::with_capacity(quads.len() / 4 * 6);
        for quad in quads.chunks_exact(4) {
            for i in [0, 1, 2, 0, 2, 3] {
                let v = quad[i];
                vertices.push(EntityVertex {
                    pos: v.pos,
                    uv: v.uv,
                    color: v.color.map(|c| c as f32 / 255.0),
                });
                lights.push(v.light);
            }
        }
        let batch = Batch {
            texture: Some("terrain.png"),
            pass: Pass::Alpha,
            vertices,
        };
        (batch, lights)
    }
}
fn append(out: &mut Vec<Vertex>, mesh: &Geometry, translation: [f64; 3]) {
    for (pos, vertex) in mesh.positions.iter().zip(&mesh.vertices) {
        let mut vertex = *vertex;
        vertex.pos = std::array::from_fn(|i| (pos[i] + translation[i]) as f32);
        out.push(vertex);
    }
}
