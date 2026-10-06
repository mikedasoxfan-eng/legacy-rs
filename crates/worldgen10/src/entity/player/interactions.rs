use super::*;
use crate::entity::{living::Target, AxisAlignedBB};
impl Entity {
    /// ItemDye.saddleEntity: its signed stack decrement happens even on clients.
    pub fn dye_sheep(&mut self, stack: &mut ItemStack) {
        if let EntityKind::Sheep { sheared, fleece_color, } = &mut self.kind {
            let color = (!stack.damage & 15) as u8;
            if !*sheared && *fleece_color != color {
                *fleece_color = color;
                stack.count = stack.count.wrapping_sub(1);
                self.sync_mob_watcher();
            }
        }
    }
    pub fn attack_with_item(&mut self, w: &mut World, id: i32) {
        let Some(mut target) = w.entities.remove(&id) else {
            return;
        };
        let mut damage = registry::attack_damage(self.held_item().map_or(-1, |s| s.id));
        damage = self.potion_attack_damage(damage);
        let mut knockback = 0i32;
        let mut bonus = 0i32;
        if target.kind.is_living() {
            let attribute = match target.creature_attribute() {
                crate::entity::living10::EnumCreatureAttribute::Undefined => crate::enchantment::CreatureAttribute::Undefined,
                crate::entity::living10::EnumCreatureAttribute::Undead => crate::enchantment::CreatureAttribute::Undead,
                crate::entity::living10::EnumCreatureAttribute::Arthropod => crate::enchantment::CreatureAttribute::Arthropod,
            };
            bonus = crate::enchantment::with_modifier_random(|rng|
                crate::enchantment::modifier_living(rng, self.held_item(), attribute));
            knockback = crate::enchantment::level(19, self.held_item());
        }
        if self.player_state().unwrap().sprinting { knockback = knockback.wrapping_add(1); }
        if damage > 0 || bonus > 0 {
            let critical = self.fall_distance > 0.0 && !self.on_ground
                && !self.is_on_ladder(w) && !self.in_water
                && self.potion_effect(15).is_none()
                && self.riding.is_none()
                && target.kind.is_living();
            if critical {
                damage = damage.wrapping_add(self.rand.next_int(damage / 2 + 2));
            }
            damage = damage.wrapping_add(bonus);
            // Vanilla passes the live player object to attackEntityFrom.
            w.entities.insert(self.entity_id, self.clone());
            let hit = target.attack_from(w, crate::entity::damage::DamageSource::entity(
                crate::entity::damage::DamageKind::Player, Some(self.entity_id),), damage,);
            if let Some(player) = w.entities.remove(&self.entity_id) {
                *self = player;
            }
            if hit && knockback > 0 {
                let yaw = self.yaw * math::PI_F32 / 180.0;
                target.add_velocity(w, (-math::sin(yaw) * knockback as f32 * 0.5) as f64, 0.1,
                    (math::cos(yaw) * knockback as f32 * 0.5) as f64,);
                target.velocity_changed = true;
                self.motion.x *= 0.6;
                self.motion.z *= 0.6;
                self.set_sprinting(false);
            }
            if hit && critical {
                self.player_state_mut().unwrap().events.push(PlayerEvent::CriticalHit { target: Box::new(target.clone()), });
            }
            if hit && bonus > 0 { self.magic_critical_hit(&target); }
            if hit && damage >= 18 { self.add_stat(5242880 + 25, 1); }
            if target.kind.is_living() {
                if let Some(s) = self.held_item() {
                    if matches!(s.id, 267 | 268 | 272 | 276 | 283) {
                        self.damage_held10(w, 1);
                        self.add_stat(stat::used(s.id), 1);
                    } else if matches!(s.id,256..=258|269..=271|273..=275|277..=279|284..=286) {
                        self.damage_held10(w, 2);
                        self.add_stat(stat::used(s.id), 1);
                    } else if s.id == 329 {
                        self.saddle(&mut target);
                        self.add_stat(stat::used(s.id), 1);
                    }
                }
                if target.living.health > 0 && !target.dead {
                    self.alert_wolves(w, &target, true);
                }
                self.add_stat(stat::DAMAGE_DEALT, damage);
                let fire_aspect = crate::enchantment::level(20, self.held_item());
                if fire_aspect > 0 { target.set_fire_seconds(fire_aspect.wrapping_mul(4)); }
            }
            self.add_exhaustion(0.3);
        }
        w.entities.insert(id, target);
    }
    fn saddle(&mut self, target: &mut Entity) {
        if let EntityKind::Pig { saddled } = &mut target.kind {
            if !*saddled {
                *saddled = true;
                self.consume_held();
            }
        }
    }
    fn consume_held(&mut self) {
        let inv = &mut self.player_state_mut().unwrap().inventory;
        inv.decrement(inv.current_item as usize, 1);
    }
    pub fn use_on_entity(&mut self, w: &mut World, id: i32) -> bool {
        let Some(mut target) = w.entities.remove(&id) else {
            return false;
        };
        let held = self.held_item();
        let mut handled = false;
        if matches!(target.kind, EntityKind::Boat(..))
            || matches!(&target.kind,EntityKind::Minecart(s) if s.cart_type==0)
        {
            let occupied = target
                .ridden_by
                .and_then(|r| w.entities.get(&r))
                .is_some_and(|r| r.player_state().is_some() && r.entity_id != self.entity_id);
            if !occupied && !w.multiplayer_world {
                w.entities.insert(id, target.clone());
                self.mount_entity(w, Some(id));
                target = w.entities.remove(&id).unwrap();
            }
            handled = true;
        }
        if let EntityKind::Minecart(s) = &mut target.kind {
            if s.cart_type == 1 {
                if !w.multiplayer_world { self.player_state_mut()
                    .unwrap()
                    .events
                    .push(PlayerEvent::OpenVehicleInventory { entity: id }); }
                handled = true;
            } else if s.cart_type == 2 {
                if held.is_some_and(|s| s.id == 263) {
                    let inv = &mut self.player_state_mut().unwrap().inventory;
                    let slot = inv.current_item as usize;
                    if let Some(stack) = &mut inv.main[slot] {
                        stack.count = stack.count.wrapping_sub(1);
                        if stack.count == 0 { inv.main[slot] = None; }
                    }
                    s.fuel = s.fuel.wrapping_add(3600);
                }
                s.push_x = target.pos.x - self.pos.x;
                s.push_z = target.pos.z - self.pos.z;
                handled = true;
            }
        }
        if matches!(target.kind, EntityKind::Pig { .. }) {
            handled = target.feed_animal(w, self);
        }
        if !handled
            && matches!(target.kind, EntityKind::Pig { saddled: true })
            && (target.ridden_by.is_none() || target.ridden_by == Some(self.entity_id))
        {
            w.entities.insert(id, target.clone());
            self.mount_entity(w, Some(id));
            target = w.entities.remove(&id).unwrap();
            handled = true;
        }
        if let EntityKind::Sheep {
            sheared,
            fleece_color,
        } = &mut target.kind
        {
            if held.is_some_and(|s| s.id == 359) && !*sheared {
                *sheared = true;
                let color = *fleece_color;
                let n = 2 + target.rand.next_int(3);
                for _ in 0..n {
                    let item = target.drop_item(w, 35, 1, color as i32, 1.0);
                    let dy = (target.rand.next_float() * 0.05) as f64;
                    let dx = ((target.rand.next_float() - target.rand.next_float()) * 0.1) as f64;
                    let dz = ((target.rand.next_float() - target.rand.next_float()) * 0.1) as f64;
                    if let Some(e) = w.entities.get_mut(&item) {
                        e.motion.y += dy;
                        e.motion.x += dx;
                        e.motion.z += dz;
                    }
                }
                self.damage_held10(w, 1);
            }
        }
        if matches!(target.kind, EntityKind::Mooshroom) && target.living.growing_age >= 0 {
            if held.is_some_and(|s| s.id == 281) {
                let inv = &mut self.player_state_mut().unwrap().inventory;
                inv.main[inv.current_item as usize] = Some(ItemStack::new(282, 1, 0));
                handled = true;
            } else if held.is_some_and(|s| s.id == 359) {
                target.dead = true;
                let mut cow = Entity::living(w, "Cow").unwrap();
                cow.set_location_and_angles(
                    target.pos.x,
                    target.pos.y,
                    target.pos.z,
                    target.yaw,
                    target.pitch,
                );
                cow.set_entity_health(target.living.health);
                cow.living.render_yaw_offset = target.living.render_yaw_offset;
                w.entity_joined_world(cow);
                w.spawn_particle(
                    "largeexplode",
                    target.pos.add(0.0, (target.height / 2.0) as f64, 0.0),
                    Vec3D::default(),
                );
                for _ in 0..5 {
                    let item = Entity::item(
                        w,
                        target.pos.x,
                        target.pos.y + target.height as f64,
                        target.pos.z,
                        crate::world::tile::ItemStack::new(40, 1, 0),
                    );
                    w.entity_joined_world(item);
                }
                handled = true;
            }
        }
        if matches!(target.kind, EntityKind::Cow | EntityKind::Mooshroom) && held.is_some_and(|s| s.id == 325) {
            let inv = &mut self.player_state_mut().unwrap().inventory;
            inv.main[inv.current_item as usize] = Some(ItemStack::new(335, 1, 0));
            handled = true;
        }
        if let EntityKind::Wolf(s) = &mut target.kind {
            if s.flags & 4 == 0 {
                if held.is_some_and(|s| s.id == 352) && s.flags & 2 == 0 {
                    self.consume_held();
                    if !w.multiplayer_world {
                    let tamed = target.rand.next_int(3) == 0;
                    if tamed {
                        s.flags |= 5;
                        s.owner = self.player_state().unwrap().username.clone();
                        target.living.path = None;
                        target.living.health = 20;
                    }
                    for _ in 0..7 {
                        let v = Vec3D::new(
                            target.rand.next_gaussian() * 0.02,
                            target.rand.next_gaussian() * 0.02,
                            target.rand.next_gaussian() * 0.02,
                        );
                        let p = Vec3D::new(
                            target.pos.x + (target.rand.next_float() * target.width * 2.0) as f64
                                - target.width as f64,
                            target.pos.y + 0.5 + (target.rand.next_float() * target.height) as f64,
                            target.pos.z + (target.rand.next_float() * target.width * 2.0) as f64
                                - target.width as f64,
                        );
                        w.spawn_particle(if tamed { "heart" } else { "smoke" }, p, v);
                    }
                    }
                    handled = true;
                }
            } else if held.is_some_and(|s| matches!(s.id, 319 | 320 | 363 | 364 | 365 | 366 | 367))
                && s.watched_health < 20
            {
                self.consume_held();
                target.heal(match held.unwrap().id {
                    320 | 364 => 8,
                    366 => 6,
                    367 => 4,
                    365 => 2,
                    _ => 3,
                });
                handled = true;
            } else if s
                .owner
                .eq_ignore_ascii_case(&self.player_state().unwrap().username)
            {
                if !w.multiplayer_world {
                    s.flags ^= 1;
                    target.living.is_jumping = false;
                    target.living.path = None;
                }
                handled = true;
            }
        }
        if !handled {
            handled = target.feed_animal(w, self);
        }
        if !handled {
            if let Some(stack) = self.held_item() {
                if stack.id == 329 {
                    self.saddle(&mut target);
                }
                if stack.id == 351 {
                    let mut dyed = stack;
                    target.dye_sheep(&mut dyed);
                    let inv = &mut self.player_state_mut().unwrap().inventory;
                    if target.kind.is_living() {
                        inv.main[inv.current_item as usize] = if dyed.count <= 0 { None } else { Some(dyed) };
                    }
                }
            }
        }
        target.sync_mob_watcher();
        w.entities.insert(id, target);
        if w.entities.contains_key(&self.entity_id) {
            w.entities.insert(self.entity_id, self.clone());
        }
        handled
    }
    pub(super) fn alert_wolves(&self, w: &mut World, target: &Entity, attack: bool) {
        let name = &self.player_state().unwrap().username;
        if matches!(
            target.kind,
            EntityKind::Creeper { .. } | EntityKind::Ghast(..) | EntityKind::Player(..)
        ) || matches!(&target.kind,EntityKind::Wolf(s) if s.flags&4!=0&&s.owner==*name)
        {
            return;
        }
        let bb = AxisAlignedBB::new(
            self.pos.x,
            self.pos.y,
            self.pos.z,
            self.pos.x + 1.0,
            self.pos.y + 1.0,
            self.pos.z + 1.0,
        )
        .expand(16.0, 4.0, 16.0);
        for id in w.entities_within_aabb(None, bb) {
            if let Some(e) = w.entities.get_mut(&id) {
                if let EntityKind::Wolf(s) = &mut e.kind {
                    if s.flags & 4 != 0
                        && e.living.target.is_none()
                        && s.owner == *name
                        && (!attack || s.flags & 1 == 0)
                    {
                        s.flags &= !1;
                        e.living.target = Some(Target::Entity(target.entity_id));
                    }
                }
            }
        }
    }
    /// Entity.mountEntity, including the vanilla toggle when mounting the same entity.
    pub fn mount_entity(&mut self, w: &mut World, target: Option<i32>) {
        self.rider_pitch_delta = 0.0;
        self.rider_yaw_delta = 0.0;
        if target.is_none() || target == self.riding {
            if let Some(parent) = self.riding.and_then(|id| w.entities.get_mut(&id)) {
                self.set_location_and_angles(
                    parent.pos.x,
                    parent.bounding_box.min_y + parent.height as f64,
                    parent.pos.z,
                    self.yaw,
                    self.pitch,
                );
                parent.ridden_by = None;
            }
            self.riding = None;
        } else {
            if let Some(parent) = self.riding.and_then(|id| w.entities.get_mut(&id)) {
                parent.ridden_by = None;
            }
            if let Some(parent) = target.and_then(|id| w.entities.get(&id)).cloned() {
                if let Some(rider) = parent.ridden_by.and_then(|id| w.entities.get_mut(&id)) {
                    rider.riding = None;
                }
                self.riding = target;
                w.entities.get_mut(&parent.entity_id).unwrap().ridden_by = Some(self.entity_id);
            }
        }
    }
}
