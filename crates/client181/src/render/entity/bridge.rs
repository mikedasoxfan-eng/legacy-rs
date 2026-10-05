//! From the simulation's `Entity` to the renderer's `RenderState` at a partial tick: the
//! interpolations `RenderLiving.doRenderLiving` and the mob renderers perform.

use super::living::{MobKind, RenderState};
use super::models::WolfAnimation;
use worldgen::entity::{Entity, EntityKind};

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Living model state; object renderers use their own bridge.
pub fn render_state(e: &Entity, partial: f32, brightness: f32) -> Option<RenderState> {
    let kind = match &e.kind {
        EntityKind::Ghast(_) => MobKind::Ghast,
        EntityKind::PigZombie { .. } => MobKind::PigZombie,
        EntityKind::Giant => MobKind::Giant,
        EntityKind::Zombie => MobKind::Zombie,
        EntityKind::Skeleton => MobKind::Skeleton,
        EntityKind::Spider => MobKind::Spider,
        EntityKind::CaveSpider => MobKind::CaveSpider,
        EntityKind::Enderman(..) => MobKind::Enderman,
        EntityKind::Silverfish{..} => MobKind::Silverfish,
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
    if let EntityKind::Enderman(state)=&e.kind {s.carrying=state.carried>0;s.screaming=state.screaming;}
    s.riding = e.riding.is_some();
    s.sneak = e.sneaking;
    match &e.kind {
        EntityKind::Ghast(g) => {
            s.ghast_attack = g.prev_attack_counter as f32 + (g.attack_counter - g.prev_attack_counter) as f32 * partial;
            s.ghast_attacking = g.attacking;
        }
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

/// Weather effects use EntityLightningBolt.boltVertex, including seed changes on restrikes.
pub fn lightning(e: &Entity) -> Option<crate::render::weather::Lightning> {
    match e.kind {
        EntityKind::Lightning { state, seed, .. } if state >= 0 => {
            Some(crate::render::weather::Lightning { pos: [e.pos.x, e.pos.y, e.pos.z], bolt_vertex: seed })
        }
        _ => None,
    }
}

/// Object render inputs; the worker supplies neighbourhood brightness for falling blocks.
pub fn object_state(e: &Entity, partial: f32, brightness: f32, light: [f32; 7]) -> Option<super::objects::ObjectState> {
    use super::objects::{ObjectKind, ObjectState};
    let kind = match &e.kind {
        EntityKind::XPOrb(state)=>ObjectKind::XPOrb{value:state.value,ticks:state.ticks},
        EntityKind::Boat(..) | EntityKind::Minecart(..) => ObjectKind::Vehicle(super::vehicles::state(e,partial)?),
        EntityKind::Item { item, age, hover_start, .. } => {
            ObjectKind::Item { item: item.clone(), age: *age, hover: *hover_start }
        }
        EntityKind::FallingSand { block, .. } => ObjectKind::Falling { block: *block, light },
        EntityKind::Tnt { fuse } => ObjectKind::Tnt { fuse: *fuse },
        EntityKind::Arrow(a) => ObjectKind::Arrow {
            yaw: lerp(e.prev_yaw, e.yaw, partial),
            pitch: lerp(e.prev_pitch, e.pitch, partial),
            shake: a.shake,
            visible: e.prev_yaw != 0.0 || e.prev_pitch != 0.0,
        },
        EntityKind::Fireball(_) => ObjectKind::Sprite { icon: 14, scale: 2.0 },
        _ => return None,
    };
    Some(ObjectState { kind, position: position(e, partial), partial, brightness })
}
pub fn position(e: &Entity, partial: f32) -> [f64; 3] {
    let p = partial as f64;
    [
        e.last_tick_pos.x + (e.pos.x - e.last_tick_pos.x) * p,
        e.last_tick_pos.y + (e.pos.y - e.last_tick_pos.y) * p,
        e.last_tick_pos.z + (e.pos.z - e.last_tick_pos.z) * p,
    ]
}
