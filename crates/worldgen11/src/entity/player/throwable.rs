//! The original snowball and egg methods share all arithmetic except hatching.
use crate::{
    entity::{projectile::ArrowState, Entity, EntityKind, MovingObjectPosition as Hit, Vec3D},
    math::{self, PI_F32 as PI},
    World,
};
impl Entity {
    pub fn write_throwable_nbt(&self) -> crate::nbt::Compound {
        let s = match &self.kind {
            EntityKind::Snowball(s) | EntityKind::Egg(s) | EntityKind::EnderPearl(s) | EntityKind::Potion(s, _) => s,
            _ => panic!("not a throwable"),
        };
        let mut n = crate::nbt::Compound::new();
        for (key, value) in [("xTile", s.tile[0]), ("yTile", s.tile[1]), ("zTile", s.tile[2])] {
            n.put(key, crate::nbt::Tag::Short(value as i16));
        }
        n.put("inTile", crate::nbt::Tag::Byte(s.block as i8));
        n.put("shake", crate::nbt::Tag::Byte(s.shake as i8));
        n.put("inGround", crate::nbt::Tag::Byte(s.in_ground as i8));
        n
    }
    pub fn read_throwable_nbt(&mut self, n: &crate::nbt::Compound) {
        let s = match &mut self.kind {
            EntityKind::Snowball(s) | EntityKind::Egg(s) | EntityKind::EnderPearl(s) | EntityKind::Potion(s, _) => s,
            _ => panic!("not a throwable"),
        };
        s.tile = [n.short("xTile") as i32, n.short("yTile") as i32, n.short("zTile") as i32];
        s.block = n.byte("inTile") as u8;
        s.shake = n.byte("shake") as u8 as i32;
        s.in_ground = n.byte("inGround") == 1;
    }
    pub fn throwable_at(w: &mut World, x: f64, y: f64, z: f64, egg: bool) -> Self {
        let mut e = Self::base(
            w,
            if egg {
                EntityKind::Egg(Default::default())
            } else {
                EntityKind::Snowball(Default::default())
            },
        );
        e.width = 0.25;
        e.height = 0.25;
        e.set_position(x, y, z);
        e.y_offset = 0.0;
        e
    }
    pub fn throwable(w: &mut World, owner: &Entity, egg: bool) -> Self {
        let state = ArrowState {
            owner: Some(owner.entity_id),
            ..Default::default()
        };
        let mut e = Self::base(
            w,
            if egg {
                EntityKind::Egg(state)
            } else {
                EntityKind::Snowball(state)
            },
        );
        e.width = 0.25;
        e.height = 0.25;
        e.set_location_and_angles(
            owner.pos.x,
            owner.pos.y + owner.eye_height() as f64,
            owner.pos.z,
            owner.yaw,
            owner.pitch,
        );
        e.pos.x -= (math::cos(e.yaw / 180.0 * PI) * 0.16) as f64;
        e.pos.y -= 0.1f32 as f64;
        e.pos.z -= (math::sin(e.yaw / 180.0 * PI) * 0.16) as f64;
        e.set_position(e.pos.x, e.pos.y, e.pos.z);
        e.y_offset = 0.0;
        e.set_arrow_heading(
            (-math::sin(e.yaw / 180.0 * PI) * math::cos(e.pitch / 180.0 * PI) * 0.4) as f64,
            (-math::sin(e.pitch / 180.0 * PI) * 0.4) as f64,
            (math::cos(e.yaw / 180.0 * PI) * math::cos(e.pitch / 180.0 * PI) * 0.4) as f64,
            1.5,
            1.0,
        );
        e
    }
    pub(in crate::entity) fn update_throwable(&mut self, w: &mut World) {
        self.last_tick_pos = self.pos;
        self.on_entity_update(w);
        let egg = matches!(self.kind, EntityKind::Egg(..));
        let pearl = matches!(self.kind, EntityKind::EnderPearl(..));
        let potion = if let EntityKind::Potion(_, metadata) = self.kind {
            Some(metadata)
        } else {
            None
        };
        let mut s = match &self.kind {
            EntityKind::Snowball(s) | EntityKind::Egg(s)
            | EntityKind::EnderPearl(s)
            | EntityKind::Potion(s, _) => s.clone(),
            _ => return,
        };
        if s.shake > 0 {
            s.shake -= 1;
        }
        if s.in_ground {
            if w.get_block_id(s.tile[0], s.tile[1], s.tile[2]) == s.block {
                s.ticks_ground += 1;
                if s.ticks_ground == 1200 {
                    self.dead = true;
                }
                self.store_throwable_state(s);
                return;
            }
            s.in_ground = false;
            self.motion.x *= (self.rand.next_float() * 0.2) as f64;
            self.motion.y *= (self.rand.next_float() * 0.2) as f64;
            self.motion.z *= (self.rand.next_float() * 0.2) as f64;
            s.ticks_ground = 0;
            s.ticks_air = 0;
        } else {
            s.ticks_air += 1;
        }
        let start = self.pos;
        let end = start.add(self.motion.x, self.motion.y, self.motion.z);
        let mut hit = w.ray_trace_blocks_options(start, end, false, false);
        let end = match hit {
            Some(Hit::Block { hit, .. }) => hit,
            _ => end,
        };
        let mut closest = 0.0;
        if !w.multiplayer_world {
        for id in w.entities_within_aabb(
            Some(self.entity_id),
            self.bounding_box
                .add_coord(self.motion.x, self.motion.y, self.motion.z)
                .expand(1.0, 1.0, 1.0),
        ) {
            let e = &w.entities[&id];
            if e.can_be_collided_with() && (Some(id) != s.owner || s.ticks_air >= 5) {
                if let Some(Hit::Block { hit: p, .. }) = e
                    .bounding_box
                    .expand(0.3f32 as f64, 0.3f32 as f64, 0.3f32 as f64)
                    .ray_trace(start, end)
                {
                    let d = start.distance(p);
                    if d < closest || closest == 0.0 {
                        closest = d;
                        hit = Some(Hit::Entity { id, hit: e.pos });
                    }
                }
            }
        }
        }
        if let Some(hit) = hit {
            if let Some(metadata) = potion {
                self.splash_potion(w, &hit, s.owner, metadata);
            } else {
            if let Hit::Entity { id, .. } = hit {
                if let Some(mut e) = w.entities.remove(&id) {
                    e.attack_from(
                        w,
                        crate::entity::damage::DamageSource::indirect(
                            crate::entity::damage::DamageKind::Thrown,
                            self.entity_id,
                            s.owner,
                        ),
                            if !egg && !pearl && matches!(e.kind, EntityKind::Blaze(..)) {
                                3
                            } else {
                        0
                            },
                    );
                    w.entities.insert(id, e);
                }
            }
            if !w.multiplayer_world && egg && self.rand.next_int(8) == 0 {
                let n = if self.rand.next_int(32) == 0 { 4 } else { 1 };
                for _ in 0..n {
                    let mut e = Entity::living(w, "Chicken").unwrap();
                    e.set_growing_age(-24000);
                        e.set_location_and_angles(self.pos.x, self.pos.y, self.pos.z, self.yaw, 0.0,);
                    w.entity_joined_world(e);
                }
            }
                if pearl {
                    for _ in 0..32 {
                        let p = self.pos.add(0.0, self.rand.next_double() * 2.0, 0.0);
                        let v =
                            Vec3D::new(self.rand.next_gaussian(), 0.0, self.rand.next_gaussian());
                        w.spawn_particle("portal", p, v);
                    }
                    if !w.multiplayer_world {
                        if let Some(mut owner) = s.owner.and_then(|id| w.entities.remove(&id)) {
                            owner.set_location_and_angles(
                                self.pos.x,
                                self.pos.y,
                                self.pos.z,
                                owner.yaw,
                                owner.pitch,
                            );
                            owner.fall_distance = 0.0;
                            owner.attack_from(
                                w,
                                crate::entity::damage::DamageSource::new(
                                    crate::entity::damage::DamageKind::Fall,
                                ),
                                5,
                            );
                            w.entities.insert(owner.entity_id, owner);
                        }
                    }
                } else {
            for _ in 0..8 {
                w.spawn_particle("snowballpoof", self.pos, Vec3D::default());
            }
                }
                if !w.multiplayer_world {
            self.dead = true;
                }
            }
        }
        self.projectile_advance(
            w,
            if self.in_water { 0.8 } else { 0.99 },
            if potion.is_some() { 0.05 } else {
            0.03 },
            Vec3D::default(),
        );
        self.store_throwable_state(s);
    }
}
