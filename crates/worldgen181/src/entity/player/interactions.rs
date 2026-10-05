use super::*;
use crate::entity::{living::Target, AxisAlignedBB};
impl Entity {
    pub fn attack_with_item(&mut self, w: &mut World, id: i32) {
        let Some(mut target) = w.entities.remove(&id) else {
            return;
        };
        let mut damage = registry::attack_damage(self.held_item().map_or(-1, |s| s.id));
        if damage > 0 {
            if self.motion.y < 0.0 {
                damage += 1;
            }
            // Vanilla passes the live player object to attackEntityFrom.
            w.entities.insert(self.entity_id, self.clone());
            target.attack(w, Some(self.entity_id), damage);
            if let Some(player) = w.entities.remove(&self.entity_id) {
                *self = player;
            }
            if target.kind.is_living() {
                if let Some(s) = self.held_item() {
                    if matches!(s.id, 267 | 268 | 272 | 276 | 283) {
                        self.damage_held(1);
                        self.add_stat(stat::used(s.id), 1);
                    } else if matches!(s.id,256..=258|269..=271|273..=275|277..=279|284..=286) {
                        self.damage_held(2);
                        self.add_stat(stat::used(s.id), 1);
                    } else if s.id == 329 {
                        self.saddle(&mut target);
                    }
                }
                if target.living.health > 0 && !target.dead {
                    self.alert_wolves(w, &target, true);
                }
                self.add_stat(stat::DAMAGE_DEALT, damage);
            }
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
            if !occupied {
                w.entities.insert(id, target.clone());
                self.mount_entity(w, Some(id));
                target = w.entities.remove(&id).unwrap();
            }
            handled = true;
        }
        if let EntityKind::Minecart(s) = &mut target.kind {
            if s.cart_type == 1 {
                self.player_state_mut()
                    .unwrap()
                    .events
                    .push(PlayerEvent::OpenVehicleInventory { entity: id });
                handled = true;
            } else if s.cart_type == 2 {
                if held.is_some_and(|s| s.id == 263) {
                    self.consume_held();
                    s.fuel = s.fuel.wrapping_add(1200);
                }
                s.push_x = target.pos.x - self.pos.x;
                s.push_z = target.pos.z - self.pos.z;
                handled = true;
            }
        }
        if matches!(target.kind, EntityKind::Pig { saddled: true })
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
                self.damage_held(1);
            }
        }
        if matches!(target.kind, EntityKind::Cow) && held.is_some_and(|s| s.id == 325) {
            let inv = &mut self.player_state_mut().unwrap().inventory;
            inv.main[inv.current_item as usize] = Some(ItemStack::new(335, 1, 0));
            handled = true;
        }
        if let EntityKind::Wolf(s) = &mut target.kind {
            if s.flags & 4 == 0 {
                if held.is_some_and(|s| s.id == 352) && s.flags & 2 == 0 {
                    self.consume_held();
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
                s.flags ^= 1;
                target.living.is_jumping = false;
                target.living.path = None;
                handled = true;
            }
        }
        if !handled {
            if let Some(stack) = self.held_item() {
                if stack.id == 329 {
                    self.saddle(&mut target);
                }
                if stack.id == 351 {
                    if let EntityKind::Sheep {
                        sheared,
                        fleece_color,
                    } = &mut target.kind
                    {
                        let color = (!stack.damage & 15) as u8;
                        if !*sheared && *fleece_color != color {
                            *fleece_color = color;
                            self.consume_held();
                        }
                    }
                }
            }
        }
        w.entities.insert(id, target);
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
