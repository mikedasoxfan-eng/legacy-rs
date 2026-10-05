//! TileEntityChestRenderer and ChestItemRenderHelper, Beta 1.8.1.
use super::{fixed::Fixed, models::{Model, ModelKind}, special};
#[derive(Clone, Debug, Default)]
pub struct ChestState {
    pub position: [f64; 3],
    pub metadata: i32,
    /// Adjacent chests in vanilla field order: Z-, X-, Z+, X+.
    pub adjacent: [bool; 4],
    pub lid: f32,
    pub prev_lid: f32,
    pub partial: f32,
}
pub fn render(s: &ChestState) -> Fixed {
    let mut g = Fixed::new(1.0);
    if s.adjacent[0] || s.adjacent[2] { return g; }
    let large = s.adjacent[1] || s.adjacent[3];
    let mut model = Model::new(if large { ModelKind::LargeChest } else { ModelKind::Chest });
    g.texture = if large { 32 } else { 31 };
    g.push();
    g.enable(2, true);
    g.material = [1.0; 4];
    g.translate(s.position[0] as f32, s.position[1] as f32 + 1.0, s.position[2] as f32 + 1.0);
    g.scale(1.0, -1.0, -1.0);
    g.translate(0.5, 0.5, 0.5);
    let angle = match s.metadata { 2 => 180.0, 4 => 90.0, 5 => -90.0, _ => 0.0 };
    if s.metadata == 2 && s.adjacent[1] { g.translate(1.0, 0.0, 0.0); }
    if s.metadata == 5 && s.adjacent[3] { g.translate(0.0, 0.0, -1.0); }
    g.rotate(angle, 0.0, 1.0, 0.0);
    g.translate(-0.5, -0.5, -0.5);
    let mut lid = s.prev_lid + (s.lid - s.prev_lid) * s.partial;
    lid = 1.0 - lid;
    lid = 1.0 - lid * lid * lid;
    let angle = -(lid * worldgen::math::PI_F32 / 2.0);
    model.parts.get_mut("field_35405_a").unwrap().rotate_angle_x = angle;
    model.parts.get_mut("field_35404_c").unwrap().rotate_angle_x = angle;
    special::model(&mut g, &model);
    g.enable(2, false);
    g.pop();
    g
}
/// A chest tile entity as the world thread sees it, for the live renderer.
#[derive(Clone, Debug)]
pub struct Tile {
    pub position: [i32; 3],
    pub metadata: i32,
    pub adjacent: [bool; 4],
    pub lid: f32,
    pub prev_lid: f32,
    /// `World.func_35451_b` at the chest (TileEntityRenderer's lightmap coordinates).
    pub light: u32,
}
/// The helper ignores its block, metadata and brightness arguments.
pub fn item() -> Fixed { render(&ChestState::default()) }
