use super::{EffectRenderer, Kind, Particle};
use crate::render::weather::Snapshot;
use worldgen::JavaRandom;

#[test]
fn reduction_draw_precedes_distance_name_parsing_and_large_explosions() {
    let mut fx = EffectRenderer::new(1);
    fx.particle_setting = 1;
    let mut random = JavaRandom::new(42);
    let mut expected = random.clone();
    expected.next_int(3);
    assert_eq!(
        fx.try_spawn_return("iconcrack_bad", [100.0; 3], [0.0; 3], [0.0; 3], &mut random),
        Ok(None)
    );
    assert_eq!(random, expected);
    fx.particle_setting = 2;
    assert!(fx
        .try_spawn_return("largeexplode", [100.0; 3], [0.0; 3], [0.0; 3], &mut random)
        .unwrap()
        .is_some());
    assert_eq!(random, expected);
    assert_eq!(
        fx.try_spawn_return("iconcrack_bad", [0.0; 3], [0.0; 3], [0.0; 3], &mut random),
        Ok(None)
    );
    fx.particle_setting = 0;
    assert_eq!(
        fx.try_spawn_return("iconcrack_bad", [0.0; 3], [0.0; 3], [0.0; 3], &mut random),
        Err("NumberFormatException")
    );
    let (_, index) = fx
        .try_spawn_return("iconcrack_381", [0.0; 3], [0.0; 3], [0.0; 3], &mut random)
        .unwrap()
        .unwrap();
    assert_eq!(fx.layers[2][index].texture, 155);
    let (_, index) = fx
        .try_spawn_return("iconcrack_373", [0.0; 3], [0.0; 3], [0.0; 3], &mut random)
        .unwrap()
        .unwrap();
    assert_eq!(fx.layers[2][index].texture, 140);
}

#[test]
fn dripping_lava_hangs_for_forty_ticks_then_falls_and_keeps_packed_257() {
    let mut math = JavaRandom::new(1);
    let mut p = Particle::new(Kind::DripLava, [0.0, 64.0, 0.0], [4.0; 3], &mut math, 23);
    assert_eq!(p.motion, [0.0; 3]);
    assert_eq!(p.packed_brightness_from(0x900050, 0.5), 257);
    let max = p.max_age;
    for _ in 0..40 {
        p.tick(&Snapshot::default(), &mut math);
        assert_eq!(p.texture, 113);
    }
    assert_eq!(p.timer, 0);
    assert_eq!(p.max_age, max - 40);
    assert_eq!(p.age, 0);
    let hanging = p.motion[1];
    p.tick(&Snapshot::default(), &mut math);
    assert_eq!(p.texture, 112);
    assert_eq!(p.timer, -1);
    assert_eq!(p.motion[1], (hanging - 0.06f32 as f64) * 0.98f32 as f64);
    assert_eq!(p.color, [1.0, 16.0 / 56.0, 4.0 / 48.0]);
}

#[test]
fn enchantment_path_uses_one_minus_float_age_before_quartic_drop() {
    let mut math = JavaRandom::new(1);
    let mut p = Particle::new(
        Kind::Enchantment,
        [4.0, 64.0, 8.0],
        [2.0, 3.0, 5.0],
        &mut math,
        23,
    );
    assert!((225..=250).contains(&p.texture));
    p.age = p.max_age / 2;
    let age = p.age;
    let t = 1.0f32 - age as f32 / p.max_age as f32;
    let mut d = 1.0f32 - t;
    d *= d;
    d *= d;
    let expected = [
        4.0 + 2.0 * t as f64,
        64.0 + 3.0 * t as f64 - (d * 1.2f32) as f64,
        8.0 + 5.0 * t as f64,
    ];
    let bounds = p.bounds;
    p.tick(&Snapshot::default(), &mut math);
    assert_eq!(p.pos, expected);
    assert_eq!(p.bounds, bounds);
    assert_eq!(p.age, age + 1);
    p.age = p.max_age;
    p.tick(&Snapshot::default(), &mut math);
    assert!(p.dead);
    assert_eq!(p.pos, [4.0, 64.0 - 1.2f32 as f64, 8.0]);
}

#[test]
fn spell_keeps_constructor_scale_and_uses_the_next_texture_frame() {
    let mut math = JavaRandom::new(1);
    let mut p = Particle::new(
        Kind::InstantSpell,
        [0.0, 64.0, 0.0],
        [0.0; 3],
        &mut math,
        23,
    );
    let scale = p.scale;
    p.tick(&Snapshot::default(), &mut math);
    assert_eq!(p.texture, 144 + 7 - 8 / p.max_age);
    p.vertices(
        &Snapshot::default(),
        [0.0; 3],
        [1.0, 1.0, 0.0, 0.0, 0.0],
        0.25,
    );
    assert_eq!(p.scale, scale);
    let mob = Particle::new(
        Kind::MobSpell,
        [0.0, 64.0, 0.0],
        [0.2, 0.3, 0.4],
        &mut math,
        23,
    );
    assert_eq!(mob.color, [0.2, 0.3, 0.4]);
}
