//! TileEntityMobSpawnerRenderer: the small spinning mob inside a monster spawner.
use super::entity::{
    fixed::Op,
    living::{MobKind, RenderState},
};

/// A spawner tile as the world thread sees it.
#[derive(Clone, Debug)]
pub struct Snapshot {
    pub position: [i32; 3],
    pub mob: String,
    /// TileEntityMobSpawner.yaw / yaw2 (current and previous spin, degrees / 10).
    pub yaw: f64,
    pub prev_yaw: f64,
    /// TileEntityRenderer's lightmap coordinates (`World.func_35451_b` at the spawner).
    pub light: u32,
}

/// EntityList names the renderer can build (EntityList.createEntityInWorld).
pub fn mob_kind(name: &str) -> Option<MobKind> {
    Some(match name {
        "Pig" => MobKind::Pig,
        "Sheep" => MobKind::Sheep,
        "Cow" => MobKind::Cow,
        "Chicken" => MobKind::Chicken,
        "Squid" => MobKind::Squid,
        "Wolf" => MobKind::Wolf,
        "Creeper" => MobKind::Creeper,
        "Skeleton" => MobKind::Skeleton,
        "Spider" => MobKind::Spider,
        "Giant" => MobKind::Giant,
        "Zombie" => MobKind::Zombie,
        "Slime" => MobKind::Slime,
        "Ghast" => MobKind::Ghast,
        "PigZombie" => MobKind::PigZombie,
        "Enderman" => MobKind::Enderman,
        "CaveSpider" => MobKind::CaveSpider,
        "Silverfish" => MobKind::Silverfish,
        "Mob" | "Monster" => MobKind::Biped,
        _ => return None,
    })
}

/// The mob's render state: a fresh, never-ticked entity at the renderer's origin, under the
/// spawner's transform (translate to the block, spin, tilt 30 degrees, scale 7/16).
pub fn state(s: &Snapshot, camera: [f64; 3], partial: f32) -> Option<RenderState> {
    let kind = mob_kind(&s.mob)?;
    let mut st = RenderState::new(kind);
    st.position = [0.0; 3];
    st.partial = partial;
    st.age = partial;
    st.brightness = 1.0;
    let op = |code, args| Op { code, args };
    let spin = (s.prev_yaw + (s.yaw - s.prev_yaw) * partial as f64) as f32 * 10.0;
    let scale = 7.0f32 / 16.0;
    st.prefix = vec![
        op(0, [
            (s.position[0] as f64 - camera[0]) as f32 + 0.5,
            (s.position[1] as f64 - camera[1]) as f32,
            (s.position[2] as f64 - camera[2]) as f32 + 0.5,
            0.0,
        ]),
        op(0, [0.0, 0.4, 0.0, 0.0]),
        op(2, [spin, 0.0, 1.0, 0.0]),
        op(2, [-30.0, 1.0, 0.0, 0.0]),
        op(0, [0.0, -0.4, 0.0, 0.0]),
        op(1, [scale, scale, scale, 0.0]),
    ];
    Some(st)
}
