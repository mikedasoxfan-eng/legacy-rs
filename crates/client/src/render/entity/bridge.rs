//! From the simulation's `Entity` to the renderer's `RenderState` at a partial tick: the
//! interpolations `RenderLiving.doRenderLiving` and the mob renderers perform.

use super::living::{MobKind, RenderState};
use super::models::WolfAnimation;
use worldgen::entity::{Entity, EntityKind};

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// `None` for entities this renderer has no model for yet (items, falling blocks, TNT,
/// arrows, fireballs, lightning, ghasts, pig zombies, giants).
pub fn render_state(e: &Entity, partial: f32, brightness: f32) -> Option<RenderState> {
    let kind = match &e.kind {
        EntityKind::Zombie => MobKind::Zombie,
        EntityKind::Skeleton => MobKind::Skeleton,
        EntityKind::Spider => MobKind::Spider,
        EntityKind::Creeper { .. } => MobKind::Creeper,
        EntityKind::Pig { .. } => MobKind::Pig,
        EntityKind::Sheep { .. } => MobKind::Sheep,
        EntityKind::Cow => MobKind::Cow,
        EntityKind::Chicken { .. } => MobKind::Chicken,
        EntityKind::Squid(_) => MobKind::Squid,
        EntityKind::Slime { .. } => MobKind::Slime,
        EntityKind::Wolf(_) => MobKind::Wolf,
        _ => return None,
    };
    let mut s = RenderState::new(kind);
    let p = partial as f64;
    // RenderManager: lastTickPos + (pos - lastTickPos) * partialTicks.
    s.position = [
        e.last_tick_pos.x + (e.pos.x - e.last_tick_pos.x) * p,
        e.last_tick_pos.y + (e.pos.y - e.last_tick_pos.y) * p,
        e.last_tick_pos.z + (e.pos.z - e.last_tick_pos.z) * p,
    ];
    let l = &e.living;
    s.yaw = lerp(e.prev_yaw, e.yaw, partial);
    s.body_yaw = lerp(l.prev_render_yaw_offset, l.render_yaw_offset, partial);
    s.pitch = lerp(e.prev_pitch, e.pitch, partial);
    s.limb_amount = lerp(l.prev_limb_amount, l.limb_amount, partial);
    s.limb_swing = l.limb_swing - l.limb_amount * (1.0 - partial);
    s.age = e.ticks_existed as f32 + partial;
    s.attack = e.swing_progress(partial);
    s.partial = partial;
    s.hurt_time = l.hurt_time;
    s.death_time = l.death_time;
    s.brightness = brightness;
    s.spider_eye_brightness = brightness;
    s.riding = e.riding.is_some();
    s.sneak = e.sneaking;
    match &e.kind {
        EntityKind::Pig { saddled } => s.saddled = *saddled,
        EntityKind::Sheep { sheared, fleece_color } => {
            s.sheared = *sheared;
            s.fleece_color = *fleece_color as usize & 15;
        }
        EntityKind::Creeper { powered, .. } => {
            s.creeper_flash = e.creeper_flash(partial);
            s.powered = *powered;
        }
        EntityKind::Slime { size, squish, prev_squish, .. } => {
            s.slime_size = *size;
            s.slime_squish = lerp(*prev_squish, *squish, partial);
        }
        EntityKind::Chicken { flap, prev_flap, flap_amount, prev_flap_amount, .. } => {
            s.chicken_flap = lerp(*prev_flap, *flap, partial);
            s.chicken_flap_amount = lerp(*prev_flap_amount, *flap_amount, partial);
        }
        EntityKind::Squid(q) => {
            s.squid_pitch = lerp(q.prev_pitch, q.pitch, partial);
            s.squid_yaw = lerp(q.prev_yaw, q.yaw, partial);
            s.tentacle_angle = lerp(q.prev_tentacle_angle, q.tentacle_angle, partial);
        }
        EntityKind::Wolf(w) => {
            // EntityWolf data watcher flags: 1 sitting, 2 angry, 4 tamed.
            s.wolf = WolfAnimation {
                angry: w.flags & 2 != 0,
                sitting: w.flags & 1 != 0,
                interested: [w.prev_interest, w.interest],
                shake: [w.prev_shake_time, w.shake_time],
            };
            s.wolf_tamed = w.flags & 4 != 0;
            s.wolf_health = w.watched_health;
            s.wolf_shaking = w.shaking;
        }
        _ => {}
    }
    Some(s)
}
