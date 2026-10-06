//! 1.0 RenderFallingSand's dragon-egg branch: RenderBlocks.renderBlockByRenderType
//! against the world, with the Tessellator translated to the block's centre.
use super::{access::Access, blocks::{Colorizers, RenderBlocks}, Tessellator, Vertex};

/// Block.field_41050_bK (the dragon egg).
pub const DRAGON_EGG: u8 = 122;

/// Tessellator vertices of a falling dragon egg at entity position `pos`, before
/// the GL translation to the camera-relative render position. Brightness and
/// colours come from the world exactly as for a placed block.
pub fn egg_vertices(access: &Access, col: &Colorizers, ao: bool, pos: [f64; 3]) -> Vec<Vertex> {
    let cell = pos.map(worldgen::math::floor_double);
    // setTranslationD((double)((float)(-floor) - 0.5F)): Tessellator::new takes the negated offset.
    let origin = cell.map(|c| -(((-c) as f32 - 0.5) as f64));
    let mut t = Tessellator::new(origin);
    let mut r = RenderBlocks::new(access, &mut t, col);
    r.ao = ao;
    r.render_block(DRAGON_EGG, cell[0], cell[1], cell[2]);
    t.verts
}
