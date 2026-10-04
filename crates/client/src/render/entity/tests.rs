use super::living::*;
use glam::{DVec3, Vec3};

#[test]
fn distant_entities_keep_sub_block_positions() {
    let mut near = RenderState::new(MobKind::Cow);
    near.position = [0.25, 2.0, -0.5];
    near.yaw = 17.0;
    near.body_yaw = -33.0;
    let origin = DVec3::new(1_000_000_000.0, 0.0, -1_000_000_000.0);
    let mut far = near.clone();
    far.position = (DVec3::from_array(near.position) + origin).to_array();
    let a = render_living(&near, DVec3::ZERO);
    let b = render_living(&far, origin);
    assert_eq!(
        bytemuck::cast_slice::<EntityVertex, u8>(&a[0].vertices),
        bytemuck::cast_slice::<EntityVertex, u8>(&b[0].vertices)
    );
}

#[test]
fn fleece_and_saddle_layers_follow_entity_flags() {
    let mut sheep = RenderState::new(MobKind::Sheep);
    sheep.fleece_color = 14;
    sheep.hurt_time = 3;
    let batches = render_living(&sheep, DVec3::ZERO);
    assert_eq!(
        batches.iter().map(|b| b.texture).collect::<Vec<_>>(),
        [Some("mob/sheep.png"), Some("mob/sheep_fur.png"), None, None]
    );
    assert!(batches[1].vertices.iter().all(|v| v.color[0] > v.color[1]));
    assert!(batches[2..].iter().all(|b| b.pass == Pass::Overlay));
    sheep.sheared = true;
    assert_eq!(render_living(&sheep, DVec3::ZERO).len(), 2);
    let mut pig = RenderState::new(MobKind::Pig);
    assert_eq!(render_living(&pig, DVec3::ZERO).len(), 1);
    pig.saddled = true;
    assert_eq!(render_living(&pig, DVec3::ZERO)[1].texture, Some("mob/saddle.png"));
}

#[test]
fn material_brightness_is_multiplied_before_gl_color_clamping() {
    let c = lit_color(Vec3::Y, [0.5, 0.25, 0.0, 0.4]);
    assert!(c[0] > 0.5 && c[0] < 1.0);
    assert_eq!(c[1], c[0] * 0.5);
    assert_eq!(c[2], 0.0);
    assert_eq!(c[3], 0.4);
    assert_eq!(lit_color(Vec3::Y, [1.0; 4]), [1.0; 4]);
}

#[test]
fn squid_corpse_override_and_equipped_item_overlays_are_preserved() {
    let mut squid = RenderState::new(MobKind::Squid);
    let live = render_living(&squid, DVec3::ZERO);
    squid.death_time = 15;
    let dead = render_living(&squid, DVec3::ZERO);
    assert_eq!(
        bytemuck::cast_slice::<EntityVertex, u8>(&live[0].vertices),
        bytemuck::cast_slice::<EntityVertex, u8>(&dead[0].vertices)
    );
    assert_eq!(dead[1].pass, Pass::Overlay);
    let mut skeleton = RenderState::new(MobKind::Skeleton);
    skeleton.hurt_time = 5;
    let b = render_living(&skeleton, DVec3::ZERO);
    assert_eq!(
        b.iter().map(|b| b.texture).collect::<Vec<_>>(),
        [Some("mob/skeleton.png"), Some("gui/items.png"), None]
    );
    // Front/back and four groups of 16 extruded strips, each a six-vertex quad.
    assert_eq!(b[1].vertices.len(), (2 + 4 * 16) * 6);
}

#[test]
fn slime_preserves_normalization_state_across_frames() {
    let mut normalize = false;
    scene_with_lighting(&[RenderState::new(MobKind::Pig)], DVec3::ZERO, &mut normalize);
    assert!(!normalize);
    scene_with_lighting(&[RenderState::new(MobKind::Slime)], DVec3::ZERO, &mut normalize);
    assert!(normalize);
    scene_with_lighting(&[], DVec3::ZERO, &mut normalize);
    assert!(normalize);
}

#[test]
fn simulation_bridge_covers_every_new_kind_and_uses_double_positions() {
    use worldgen::entity::{Entity, EntityKind, Vec3D};
    use worldgen::world::tile::ItemStack;
    let mut world = worldgen::World::new(0, 1);
    let mut e = Entity::item(&mut world, 1_000_000_000.25, 2.5, -1_000_000_000.5, ItemStack::new(264, 21, 0));
    e.last_tick_pos = Vec3D::new(1_000_000_000.0, 2.0, -1_000_000_000.0);
    e.prev_yaw = 170.0;
    e.yaw = -170.0;
    e.prev_pitch = 30.0;
    e.pitch = -10.0;
    let partial = 0.25;
    let object = super::bridge::object_state(&e, partial, 0.5, [0.75; 7]).unwrap();
    assert_eq!(object.position, [1_000_000_000.0625, 2.125, -1_000_000_000.125]);
    for kind in [
        EntityKind::FallingSand { block: 12, fall_time: 3 },
        EntityKind::Tnt { fuse: 9 },
        EntityKind::Arrow(Default::default()),
        EntityKind::Fireball(Default::default()),
    ] {
        e.kind = kind;
        let s = super::bridge::object_state(&e, partial, 0.5, [0.75; 7]).unwrap();
        assert!(!super::objects::render(&s, s.position, [37.0, -20.0]).draws.is_empty());
    }
    for kind in
        [EntityKind::Ghast(Default::default()), EntityKind::PigZombie { anger: 1, sound_delay: 0 }, EntityKind::Giant]
    {
        e.kind = kind;
        let s = super::bridge::render_state(&e, partial, 0.5).unwrap();
        assert_eq!(s.position, object.position);
        assert_eq!(s.yaw, 85.0);
        assert_eq!(s.pitch, 20.0);
        assert!(!render_living(&s, DVec3::from_array(s.position)).is_empty());
    }
}

#[test]
fn lightning_uses_live_entity_seed_and_range_gate_on_every_restrike() {
    use worldgen::entity::{Entity, EntityKind};
    let mut world = worldgen::World::new(0, 1);
    let mut e = Entity::lightning(&mut world, 1.25, 80.0, -3.5);
    e.kind = EntityKind::Lightning { state: 0, seed: 187, remaining: 2 };
    let a = super::bridge::lightning(&e).unwrap();
    assert_eq!(a.bolt_vertex, 187);
    assert_eq!(a.pos, [1.25, 80.0, -3.5]);
    e.kind = EntityKind::Lightning { state: -1, seed: 187, remaining: 2 };
    assert!(super::bridge::lightning(&e).is_none());
    e.kind = EntityKind::Lightning { state: 1, seed: -713, remaining: 1 };
    assert_eq!(super::bridge::lightning(&e).unwrap().bolt_vertex, -713);
}
