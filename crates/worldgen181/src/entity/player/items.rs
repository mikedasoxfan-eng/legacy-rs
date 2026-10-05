use super::*;
use crate::{block::*, entity::MovingObjectPosition};
impl Entity {
    pub fn use_on_block(&mut self, w: &mut World, x: i32, y: i32, z: i32, side: i32) -> bool {
        if self.activate_block(w, x, y, z) {
            return true;
        }
        let Some(mut stack) = self.held_item() else {
            return false;
        };
        let original=stack;
        let original_id = stack.id;
        let used = self.item_use_on_block181(w, &mut stack, x, y, z, side);
        // ItemStack.useItem: a successful onItemUse counts a use.
        if used {
            self.add_stat(stat::used(original_id), 1);
        }
        if self.player_state().unwrap().capabilities.creative {stack.count=original.count;stack.damage=original.damage;}
        let inv = &mut self.player_state_mut().unwrap().inventory;
        inv.main[inv.current_item as usize] = if stack.count == 0 { None } else { Some(stack) };
        used
    }
    /// Item.onItemUse, before ItemStack's statistics and the controller's creative restore.
    pub fn item_use_on_block181(&mut self, w: &mut World, stack: &mut ItemStack, x: i32, y: i32, z: i32, side: i32) -> bool {
        match stack.id {
            323 => self.place_sign(w, stack, x, y, z, side),
            321 => self.place_painting(w, stack, x, y, z, side),
            2256 | 2257 => {
                if w.get_block_id(x, y, z) != 84 || w.get_block_metadata(x, y, z) != 0 {
                    return false;
                }
                if w.multiplayer_world {
                    return true;
                }
                let used = w.insert_record(x, y, z, stack.id);
                if used {
                    stack.count = stack.count.wrapping_sub(1);
                }
                used
            }
            1..=255 => self.place_block_item(w, stack, x, y, z, side),
            324 | 330 => self.place_door(w, stack, x, y, z, side),
            355 => self.place_bed(w, stack, x, y, z, side),
            351 => self.dye_block(w, stack, x, y, z),
            328 | 342 | 343 => {
                if matches!(w.get_block_id(x, y, z), 27 | 28 | 66) {
                    let vehicle = match stack.id {
                        342 => VehicleKind::StorageMinecart,
                        343 => VehicleKind::PoweredMinecart,
                        _ => VehicleKind::Minecart,
                    };
                    self.player_state_mut()
                        .unwrap()
                        .events
                        .push(PlayerEvent::SpawnVehicle {
                            vehicle,
                            position: Vec3D::new(
                                (x as f32 + 0.5) as f64,
                                (y as f32 + 0.5) as f64,
                                (z as f32 + 0.5) as f64,
                            ),
                        });
                    let cart_type = match vehicle { VehicleKind::StorageMinecart=>1,VehicleKind::PoweredMinecart=>2,_=>0 };
                    let cart=Entity::minecart(w,(x as f32+0.5) as f64,(y as f32+0.5) as f64,(z as f32+0.5) as f64,cart_type);
                    w.entity_joined_world(cart);
                    stack.count -= 1;
                    true
                } else {
                    false
                }
            }
            259 => {
                let (x, y, z) = super::placement::offset(x, y, z, side);
                if !self.can_edit_block181(x, y, z) { return false; }
                if w.get_block_id(x, y, z) == 0 {
                    let pitch = w.item_random.next_float() * 0.4 + 0.8;
                    w.play_sound(
                        "fire.ignite",
                        Vec3D::new(x as f64 + 0.5, y as f64 + 0.5, z as f64 + 0.5),
                        1.0,
                        pitch,
                    );
                    w.set_block_with_notify(x, y, z, 51);
                }
                if stack.damage_item(1) {
                    self.add_stat(stat::broken(stack.id), 1);
                }
                true
            }
            290..=294 => {
                if !self.can_edit_block181(x, y, z) { return false; }
                let id = w.get_block_id(x, y, z);
                if (side == 0 || w.get_block_id(x, y + 1, z) != 0 || id != 2) && id != 3 {
                    false
                } else {
                    let (name, volume, pitch) = crate::world::events::step_sound(60);
                    w.play_sound(
                        name,
                        Vec3D::new(
                            (x as f32 + 0.5) as f64,
                            (y as f32 + 0.5) as f64,
                            (z as f32 + 0.5) as f64,
                        ),
                        (volume + 1.0) / 2.0,
                        pitch * 0.8,
                    );
                    w.set_block_with_notify(x, y, z, 60);
                    if stack.damage_item(1) {
                        self.add_stat(stat::broken(stack.id), 1);
                    }
                    true
                }
            }
            // ItemSeeds: wheat, pumpkin and melon seeds plant their crop or stem on farmland.
            295 | 361 | 362 => {
                if side == 1 && self.can_edit_block181(x,y,z) && self.can_edit_block181(x,y+1,z) && w.get_block_id(x, y, z) == 60 && w.get_block_id(x, y + 1, z) == 0 {
                    w.set_block_with_notify(x, y + 1, z, match stack.id { 361 => 104, 362 => 105, _ => 59 });
                    stack.count -= 1;
                    true
                } else {
                    false
                }
            }
            331 => {
                let snow = w.get_block_id(x, y, z) == SNOW;
                let (x, y, z) = if snow { (x, y, z) } else { super::placement::offset(x, y, z, side) };
                if !snow && w.get_block_id(x, y, z) != AIR { return false; }
                if !self.can_edit_block181(x, y, z) { return false; }
                if w.is_block_normal_cube(x, y - 1, z) {
                    stack.count = stack.count.wrapping_sub(1);
                    w.set_block_with_notify(x, y, z, 55);
                }
                true
            }
            338 | 354 | 356 => {
                let block = match stack.id {
                    338 => 83,
                    354 => 92,
                    _ => 93,
                };
                let side = if w.get_block_id(x, y, z) == 78 {
                    0
                } else {
                    side
                };
                let (x, y, z) = if matches!(w.get_block_id(x, y, z), 78 | VINE) {
                    (x, y, z)
                } else {
                    super::placement::offset(x, y, z, side)
                };
                if !self.can_edit_block181(x,y,z) || stack.count == 0 {
                    false
                } else {
                    w.entities.insert(self.entity_id, self.clone());
                    let place = super::placement::can_place(w, block, x, y, z, side);
                    w.entities.remove(&self.entity_id);
                    if place && w.set_block_with_notify(x, y, z, block) {
                        if w.get_block_id(x, y, z) == block { self.block_placed(w, block, x, y, z, side); }
                        let (name, volume, pitch) = crate::world::events::step_sound(block);
                        w.play_sound(
                            name,
                            Vec3D::new(
                                (x as f32 + 0.5) as f64,
                                (y as f32 + 0.5) as f64,
                                (z as f32 + 0.5) as f64,
                            ),
                            (volume + 1.0) / 2.0,
                            pitch * 0.8,
                        );
                        stack.count -= 1;
                    }
                    true
                }
            }
            _ => false,
        }
    }
    pub fn use_item(&mut self, w: &mut World) -> bool {
        let Some(mut stack) = self.held_item() else {
            return false;
        };
        let original = stack;
        if super::mechanics::food_value(stack.id).is_some() {
            let s = self.player_state().unwrap();
            if (stack.id == 322 || s.food.needs_food()) && !s.capabilities.disable_damage {
                self.start_item_use(stack, 32);
            }
        } else if matches!(stack.id, 267 | 268 | 272 | 276 | 283) {
            self.start_item_use(stack, 72000);
        } else if stack.id == 346 {
            self.use_fishing_rod(w, &mut stack);
        } else if stack.id == 261 {
            if self.player_state().unwrap().inventory.contains_item(262) {
                self.start_item_use(stack, 72000);
            }
        } else if matches!(stack.id, 332 | 344) {
            stack.count -= 1;
            let pitch = 0.4 / (w.item_random.next_float() * 0.4 + 0.8);
            w.play_sound_at_entity(self, "random.bow", 0.5, pitch);
            let projectile = Entity::throwable(w, self, stack.id == 344);
            w.entity_joined_world(projectile);
        } else if stack.id == 333 {
            let start = Vec3D::new(
                self.prev_pos.x + (self.pos.x - self.prev_pos.x),
                self.prev_pos.y + (self.pos.y - self.prev_pos.y) + 1.62 - self.y_offset as f64,
                self.prev_pos.z + (self.pos.z - self.prev_pos.z),
            );
            let look = self.look(1.0);
            if let Some(MovingObjectPosition::Block { x, mut y, z, .. }) = w
                .ray_trace_blocks_options(
                    start,
                    start.add(look.x * 5.0, look.y * 5.0, look.z * 5.0),
                    true,
                    false,
                )
            {
                if w.get_block_id(x, y, z) == 78 {
                    y -= 1;
                }
                self.player_state_mut()
                    .unwrap()
                    .events
                    .push(PlayerEvent::SpawnVehicle {
                        vehicle: VehicleKind::Boat,
                        position: Vec3D::new(
                            (x as f32 + 0.5) as f64,
                            (y as f32 + 1.0) as f64,
                            (z as f32 + 0.5) as f64,
                        ),
                    });
                let boat=Entity::boat(w,(x as f32+0.5) as f64,(y as f32+1.0) as f64,(z as f32+0.5) as f64);
                w.entity_joined_world(boat);
                stack.count -= 1;
            }
        } else if matches!(stack.id, 325..=327 | 335) {
            let full = match stack.id {
                326 => 8,
                327 => 10,
                335 => -1,
                _ => 0,
            };
            let start = Vec3D::new(
                self.prev_pos.x + (self.pos.x - self.prev_pos.x),
                self.prev_pos.y + (self.pos.y - self.prev_pos.y) + 1.62 - self.y_offset as f64,
                self.prev_pos.z + (self.pos.z - self.prev_pos.z),
            );
            let look = self.look(1.0);
            if let Some(MovingObjectPosition::Block { x, y, z, side, .. }) = w
                .ray_trace_blocks_options(
                    start,
                    start.add(look.x * 5.0, look.y * 5.0, look.z * 5.0),
                    full == 0,
                    false,
                )
            {
                if full == 0 {
                    let mat = w.get_block_material(x, y, z);
                    if matches!(mat, MAT_WATER | MAT_LAVA) && w.get_block_metadata(x, y, z) == 0 {
                        w.set_block_with_notify(x, y, z, 0);
                        stack = ItemStack::new(if mat == MAT_WATER { 326 } else { 327 }, 1, 0);
                    }
                } else if full < 0 {
                    stack = ItemStack::new(325, 1, 0);
                } else {
                    let (x, y, z) = super::placement::offset(x, y, z, side);
                    if w.get_block_id(x, y, z) == 0 || !mat(w.get_block_material(x, y, z)).is_solid
                    {
                        if w.dimension.is_hell_world() && full == 8 {
                            let pitch = 2.6 + (w.rand.next_float() - w.rand.next_float()) * 0.8;
                            w.play_sound(
                                "random.fizz",
                                Vec3D::new(x as f64 + 0.5, y as f64 + 0.5, z as f64 + 0.5),
                                0.5,
                                pitch,
                            );
                            for _ in 0..8 {
                                let p = Vec3D::new(
                                    x as f64 + w.math_random.next_double(),
                                    y as f64 + w.math_random.next_double(),
                                    z as f64 + w.math_random.next_double(),
                                );
                                w.spawn_particle("largesmoke", p, Vec3D::default());
                            }
                        } else {
                            w.set_block_and_metadata_with_notify(x, y, z, full as u8, 0);
                        }
                        if !self.player_state().unwrap().capabilities.creative { stack = ItemStack::new(325, 1, 0); }
                    }
                }
            }
        }
        let changed = !stack.same_object(original) || stack.count != original.count;
        let inv = &mut self.player_state_mut().unwrap().inventory;
        inv.main[inv.current_item as usize] = if changed && stack.count == 0 { None } else { Some(stack) };
        changed
    }
}
