//! Platform-independent client input. Call once for each unpaused game tick.
use super::*;
use crate::entity::MovingObjectPosition as Hit;
#[derive(Clone, Copy, Debug, Default)]
pub enum PlayerTarget {
    /// EntityRenderer.getMouseOver(1), after the supplied mouse-look deltas.
    #[default]
    RayTrace,
    /// A renderer-provided objectMouseOver, including an explicit miss.
    ObjectMouseOver(Option<Hit>),
}
#[derive(Clone, Copy, Debug, Default)]
pub struct PlayerTickInput {
    pub movement: PlayerInput,
    pub target: PlayerTarget,
    pub left_pressed: bool,
    pub attack_presses: i32,
    pub left_held: bool,
    pub right_pressed: bool,
    pub use_presses: i32,
    pub pick_presses: i32,
    pub right_held: bool,
    pub drop_pressed: bool,
    pub drop_presses: i32,
    pub hotbar_slot: Option<u8>,
    pub scroll: i32,
}
#[derive(Clone, Debug)]
pub struct PlayerHud {
    pub food: FoodStats,
    pub experience: Experience,
    pub capabilities: PlayerCapabilities,
    pub fov_multiplier: f32,
    pub health: i32,
    pub prev_health: i32,
    pub hearts_life: i32,
    pub air: i32,
    pub armor: i32,
    pub inventory: InventoryPlayer,
    pub held_item: Option<ItemStack>,
    pub time_in_portal: f32,
    pub prev_time_in_portal: f32,
    pub sleep_timer: i32,
    pub sleeping: bool,
    pub brightness: f32,
    pub eye_height: f32,
    pub camera_yaw: f32,
    pub prev_camera_yaw: f32,
    pub camera_pitch: f32,
    pub prev_camera_pitch: f32,
    pub distance_walked: f32,
    pub prev_distance_walked: f32,
    pub swing_progress: f32,
    pub mining_progress: f32,
}
impl Entity {
    fn click_player(
        &mut self,
        w: &mut World,
        c: &mut controller::PlayerControllerSP,
        hit: Option<Hit>,
        right: bool,
    ) {
        if !right && self.player_state().unwrap().left_click_counter > 0 {
            return;
        }
        if !right {
            self.swing_item();
        } else {
            self.player_state_mut().unwrap().right_click_delay = 6;
        }
        let mut use_item = true;
        match hit {
            None => {
                if !right && !self.player_state().unwrap().capabilities.creative {
                    self.player_state_mut().unwrap().left_click_counter = 10;
                }
            }
            Some(Hit::Entity { id, .. }) => {
                if right {
                    self.use_on_entity(w, id);
                } else {
                    self.attack_with_item(w, id);
                }
            }
            Some(Hit::Block { x, y, z, side, .. }) => {
                if right {
                    let held = self.held_item();
                    if self.use_on_block(w, x, y, z, side) {
                        use_item = false;
                        self.swing_item();
                    }
                    if held.is_none() {
                        return;
                    }
                } else {
                    c.click_block(self, w, x, y, z, side);
                }
            }
        }
        if right && use_item {
            self.use_item(w);
        }
    }
    pub fn player_hud(&self, w: &mut World, partial: f32) -> Option<PlayerHud> {
        let s = self.player_state()?;
        Some(PlayerHud {
            food: s.food.clone(),
            experience: s.experience,
            capabilities: s.capabilities,
            fov_multiplier: self.fov_multiplier(),
            health: self.living.health,
            prev_health: self.living.prev_health,
            hearts_life: self.living.hearts_life,
            air: self.air,
            armor: s.inventory.armor_value(),
            inventory: s.inventory.clone(),
            held_item: self.held_item(),
            time_in_portal: s.time_in_portal,
            prev_time_in_portal: s.prev_time_in_portal,
            sleep_timer: s.sleep_timer,
            sleeping: s.sleeping,
            brightness: self.brightness(w),
            eye_height: self.eye_height(),
            camera_yaw: s.camera_yaw,
            prev_camera_yaw: s.prev_camera_yaw,
            camera_pitch: s.camera_pitch,
            prev_camera_pitch: s.prev_camera_pitch,
            distance_walked: self.distance_walked,
            prev_distance_walked: self.prev_distance_walked,
            swing_progress: self.swing_progress(partial),
            mining_progress: s.controller.damage_partial(partial),
        })
    }
}
impl World {
    /// Apply controller/input actions before World.updateEntities. Pressed bits
    /// are edges; held bits are the button state after those edges.
    pub fn apply_player_tick_input(
        &mut self,
        id: i32,
        input: PlayerTickInput,
        _ticks: i32,
    ) -> Option<Hit> {
        let mut p = self.entities.remove(&id)?;
        if p.player_state().is_none() {
            self.entities.insert(id, p);
            return None;
        }
        {
            let s=p.player_state_mut().unwrap();
            if s.left_click_counter>0 {s.left_click_counter-=1;}
            if s.right_click_delay>0 {s.right_click_delay-=1;}
            s.pending_attack_presses += input.attack_presses + input.left_pressed as i32;
            s.pending_use_presses += input.use_presses + input.right_pressed as i32;
            s.pending_pick_presses += input.pick_presses;
        }
        p.set_player_input(input.movement);
        let hit = match input.target {
            PlayerTarget::RayTrace => p.mouse_over(self, 1.0),
            PlayerTarget::ObjectMouseOver(hit) => hit,
        };
        let mut controller = std::mem::take(&mut p.player_state_mut().unwrap().controller);
        controller.update_for_mode(p.player_state().unwrap().capabilities.creative);
        let inventory = &mut p.player_state_mut().unwrap().inventory;
        if let Some(slot) = input.hotbar_slot.filter(|&slot| slot < 9) {
            inventory.current_item = slot as i32;
        }
        inventory.change_current_item(input.scroll);
        for _ in 0..input.drop_presses + input.drop_pressed as i32 {
            let inventory = &mut p.player_state_mut().unwrap().inventory;
            let slot = inventory.current_item as usize;
            if let Some(stack) = inventory.decrement(slot, 1) {
                p.drop_player_item(self, stack, false);
            }
        }
        let using = p.player_state().unwrap().item_in_use.is_some();
        if using && !input.right_held { p.stop_item_use(self); }
        if !using {
            let s=p.player_state_mut().unwrap();
            let (attacks,uses,picks)=(s.pending_attack_presses,s.pending_use_presses,s.pending_pick_presses);
            s.pending_attack_presses=0;s.pending_use_presses=0;s.pending_pick_presses=0;
            for _ in 0..attacks {p.click_player(self,&mut controller,hit,false);}
            for _ in 0..uses {p.click_player(self,&mut controller,hit,true);}
            for _ in 0..picks {p.pick_block(self,hit);}
        }
        if input.right_held && p.player_state().unwrap().right_click_delay == 0 && p.player_state().unwrap().item_in_use.is_none() {
            p.click_player(self,&mut controller,hit,true);
        }
        if !input.left_held {
            p.player_state_mut().unwrap().left_click_counter = 0;
        }
        if p.player_state().unwrap().left_click_counter <= 0 {
            if input.left_held {
                if let Some(Hit::Block { x, y, z, side, .. }) = hit {
                    controller.dig(&mut p, self, x, y, z, side);
                    let block = self.get_block_id(x, y, z);
                    let metadata = self.get_block_metadata(x, y, z);
                    self.events
                        .push(crate::world::events::WorldEvent::BlockHit {
                            position: (x, y, z),
                            side,
                            block,
                            metadata,
                            bounds: {
                                let b = self.block_bounds[block as usize];
                                [b.min_x, b.min_y, b.min_z, b.max_x, b.max_y, b.max_z]
                            },
                        });
                    p.swing_item();
                } else {
                    if !p.player_state().unwrap().capabilities.creative { controller.reset(); }
                }
            } else {
                if !p.player_state().unwrap().capabilities.creative { controller.reset(); }
            }
        }
        p.player_state_mut().unwrap().controller = controller;
        if let Some(index) = p.player_state().and_then(|s| s.player_index) {
            self.players[index] = (p.pos.x, p.pos.y, p.pos.z);
        }
        self.entities.insert(id, p);
        hit
    }
    /// Complete SP tick: controller, actions, entity/tile ticks, world, display RNG.
    pub fn player_tick(&mut self, id: i32, input: PlayerTickInput, ticks: i32) -> Option<Hit> {
        let hit = self.apply_player_tick_input(id, input, ticks);
        let index = self.entities.get(&id)?.player_state()?.player_index?;
        if self.lightning_sky_flash > 0 {
            self.lightning_sky_flash -= 1;
        }
        self.client_tick(index, ticks);
        hit
    }
    pub fn take_player_events(&mut self, id: i32) -> Vec<PlayerEvent> {
        self.entities
            .get_mut(&id)
            .and_then(Entity::player_state_mut)
            .map_or_else(Vec::new, |s| {
                // The oracle's per-snapshot addStat log is not kept while playing.
                s.stat_log.clear();
                std::mem::take(&mut s.events)
            })
    }
    pub fn player_hud(&mut self, id: i32, partial: f32) -> Option<PlayerHud> {
        self.entities.get(&id)?.clone().player_hud(self, partial)
    }
}
