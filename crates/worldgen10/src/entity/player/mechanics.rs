//! EntityPlayer / EntityPlayerSP additions introduced in Beta 1.8.1.
use super::*;
impl Entity {
    /// EntityPlayer.getItemIcon: the item/renderer supplies the inherited base icon.
    pub fn player_item_icon(&self,item:ItemStack,pass:i32,base_icon:i32)->i32 {
        let state=self.player_state().expect("EntityPlayer");
        if item.id==346 && state.fish_entity.is_some() {return base_icon+16;}
        if item.id==373 {return if pass==1 {base_icon}else {141};}
        if state.item_in_use.is_some() && item.id==261 {
            let elapsed=registry::use_duration(item.id)-state.item_use_ticks;
            if elapsed>=18 {return 133;}if elapsed>13 {return 117;}if elapsed>0 {return 101;}
        }
        base_icon
    }
    pub fn enter_end(&mut self, w: &mut World, dimension: i32) {
        if w.multiplayer_world {
            return;
        }
        if self.player_state().unwrap().dimension == 1 && dimension == 1 {
            self.add_stat(5242880 + 23, 1);
            self.player_state_mut()
                .unwrap()
                .events
                .push(PlayerEvent::WinGame);
        } else {
            self.add_stat(5242880 + 22, 1);
            let pitch = self.rand.next_float() * 0.4 + 0.8;
            w.direct_sound("portal.travel", None, 1.0, pitch);
            self.player_state_mut()
                .unwrap()
                .events
                .push(PlayerEvent::ChangeDimensionTo { dimension: 1 });
        }
    }
    pub fn magic_critical_hit(&mut self, target: &Entity) {
        self.player_state_mut()
            .unwrap()
            .events
            .push(PlayerEvent::MagicCriticalHit {
                target: Box::new(target.clone()),
            });
    }
    pub fn open_enchantment(&mut self, position: (i32, i32, i32)) {
        self.player_state_mut()
            .unwrap()
            .events
            .push(PlayerEvent::OpenContainer {
                block: 116,
                position,
            });
    }
    pub fn open_brewing_stand(&mut self, position: (i32, i32, i32)) {
        self.player_state_mut()
            .unwrap()
            .events
            .push(PlayerEvent::OpenContainer {
                block: 117,
                position,
            });
    }
    /// EntityPlayerSP.setHealth: remote health changes bypass armor and blocking.
    pub fn set_player_health(&mut self, health: i32) {
        let damage = self.living.health.wrapping_sub(health);
        if damage <= 0 {
            self.living.health = health;
            if damage < 0 {
                self.living.hearts_life = self.living.hearts_halves_life / 2;
            }
        } else {
            self.living.last_damage = damage;
            self.set_entity_health( self.living.health);
            self.living.hearts_life = self.living.hearts_halves_life;
            self.damage_living_from(super::super::damage::DamageSource::new(super::super::damage::DamageKind::Generic), damage,);
            self.living.hurt_time = 10;
            self.living.max_hurt_time = 10;
        }
    }
    /// EntityPlayerSP.func_35219_c, whose argument order differs from its field order.
    pub fn set_player_experience(&mut self, points: f32, total: i32, level: i32) {
        self.player_state_mut().unwrap().experience = Experience { points, total, level, };
    }
    pub(super) fn player_swing_duration(&self) -> i32 {
        if let Some(effect) = self.potion_effect(3) {
            6i32.wrapping_sub(effect.amplifier.wrapping_add(1))
        } else if let Some(effect) = self.potion_effect(4) {
            6i32.wrapping_add(effect.amplifier.wrapping_add(1).wrapping_mul(2))
        } else { 6 }
    }
    pub fn add_exhaustion(&mut self, amount: f32) {
        if let Some(s) = self.player_state_mut() {
            if !s.capabilities.disable_damage { s.food.add_exhaustion(amount); }
        }
    }
    pub fn add_experience(&mut self, amount: i32) {
        if let Some(s) = self.player_state_mut() {
            s.score = s.score.wrapping_add(amount); s.experience.add(amount); }
    }
    pub fn set_creative(&mut self, enabled: bool) {
        if let Some(s) = self.player_state_mut() {
            s.capabilities.set_creative(enabled);
        }
    }
    /// PlayerControllerTest.func_6473_b: initialize capabilities and only empty hotbar slots.
    pub fn initialize_player_mode(&mut self, creative: bool) {
        self.set_creative(creative);
        if creative {
            let inventory=&mut self.player_state_mut().unwrap().inventory;
            for (slot,id) in [1,4,45,3,5,17,18,50,44].into_iter().enumerate() {
                if inventory.main[slot].is_none() {inventory.main[slot]=Some(ItemStack::new(id,1,0));}
            }
        }
    }
    pub fn set_sprinting(&mut self, enabled: bool) {
        let s = self.player_state_mut().unwrap();
        s.sprinting = enabled;
        s.sprint_ticks = if enabled { 600 } else { 0 };
        // One flag in Java (Entity.setSprinting): keep the entity's copy in step.
        self.sprinting = enabled;
    }
    pub(super) fn update_sprint_and_flight(&mut self) {
        let blind = self.potion_effect(15).is_some();
        let on_ground = self.on_ground;
        let collided = self.collided_horizontally;
        let s = self.player_state_mut().unwrap();
        if s.sprint_ticks > 0 {
            s.sprint_ticks -= 1;
            if s.sprint_ticks == 0 { s.sprinting = false; }
        }
        if s.sprint_toggle_timer > 0 { s.sprint_toggle_timer -= 1; }
        let (_, mut forward) = s.input.movement();
        if s.item_in_use.is_some() { forward *= 0.2; s.sprint_toggle_timer = 0; }
        let can_sprint = s.food.read().level as f32 > 6.0;
        if on_ground && s.previous_forward < 0.8 && forward >= 0.8 && !s.sprinting && can_sprint && s.item_in_use.is_none()
            && !blind {
            if s.sprint_toggle_timer == 0 { s.sprint_toggle_timer = 7; }
            else { s.sprinting = true; s.sprint_ticks = 600; s.sprint_toggle_timer = 0; }
        }
        if s.input.sneak {
            s.sprint_toggle_timer = 0;
        }
        if s.sprinting && (forward < 0.8 || collided || !can_sprint) { s.sprinting = false; s.sprint_ticks = 0; }
        if s.capabilities.allow_flying && !s.previous_jump && s.input.jump {
            if s.fly_toggle_timer == 0 { s.fly_toggle_timer = 7; }
            else { s.capabilities.flying = !s.capabilities.flying; s.fly_toggle_timer = 0; }
        }
        s.previous_forward = forward;
        s.previous_jump = s.input.jump;
        let sprinting = s.sprinting;
        let flying = s.capabilities.flying;
        let sneak = s.input.sneak;
        let jump = s.input.jump;
        // EntityPlayer.onLivingUpdate decrements this AFTER the SP toggle logic.
        if s.fly_toggle_timer > 0 { s.fly_toggle_timer -= 1; }
        // One flag in Java (Entity.setSprinting): keep the entity's copy in step.
        self.sprinting = sprinting;
        if flying {
            if sneak { self.motion.y -= 0.15; }
            if jump { self.motion.y += 0.15; }
        }
    }
    pub fn fov_multiplier(&self) -> f32 {
        let Some(s) = self.player_state() else { return 1.0; };
        let mut fov = if s.capabilities.flying { 1.1 } else { 1.0 };
        fov *= (s.ground_speed * self.potion_speed_multiplier() / 0.1 + 1.0) / 2.0;
        if s.item_in_use.is_some_and(|i| i.id == 261) {
            let mut draw = (72000 - s.item_use_ticks) as f32 / 20.0;
            if draw > 1.0 { draw = 1.0; } else { draw *= draw; }
            fov *= 1.0 - draw * 0.15;
        }
        fov
    }
    pub fn is_blocking(&self) -> bool {
        self.player_state().is_some_and(|s| { s.item_in_use.is_some_and(|i| matches!(i.id, 267 | 268 | 272 | 276 | 283))
        })
    }
    pub fn start_item_use(&mut self, item: ItemStack, duration: i32) {
        let s = self.player_state_mut().unwrap();
        if !s.item_in_use.is_some_and(|used| used.same_object(item)) {
            s.item_in_use = Some(item);
            s.item_use_slot = s.inventory.current_item;
            s.item_use_ticks = duration;
        }
    }
    pub fn clear_item_use(&mut self) {
        let s = self.player_state_mut().unwrap();
        s.item_in_use = None;
        s.item_use_ticks = 0;
    }
    pub fn stop_item_use(&mut self, w: &mut World) {
        let s=self.player_state().unwrap();
        if s.item_in_use.is_some_and(|i|i.id==261) && (s.capabilities.creative || s.inventory.contains_item(262)) {
            let mut charge=(72000-s.item_use_ticks) as f32/20.0;
            charge=(charge*charge+charge*2.0)/3.0;
            if charge as f64>=0.1 {
                if charge>1.0 {charge=1.0;}
                let mut arrow=Entity::arrow_from_owner_power(w,self,charge*2.0);
                if let EntityKind::Arrow(a) = &mut arrow.kind { a.critical = charge == 1.0; }
                // ItemBow damages the in-use stack in place: a broken bow stays in its
                // slot as a zero-size stack (nothing on this path clears it).
                if let Some(mut bow) = self.held_item() {
                    bow.damage_item10(1, self, w);
                    let inv = &mut self.player_state_mut().unwrap().inventory;
                    inv.main[inv.current_item as usize] = Some(bow);
                }
                let pitch=1.0/(w.item_random.next_float()*0.4+1.2)+charge*0.5;
                w.play_sound_at_entity(self,"random.bow",1.0,pitch);
                self.player_state_mut().unwrap().inventory.consume(262);
                if !w.multiplayer_world {w.entity_joined_world(arrow);}
            }
        }
        self.clear_item_use();
    }
    pub(super) fn update_item_use(&mut self, w: &mut World) {
        let Some(item) = self.player_state().unwrap().item_in_use else { return; };
        let s = self.player_state().unwrap();
        if !s.inventory.held().is_some_and(|held| held.same_object(item)) {
            self.clear_item_use();
            return;
        }
        let item = s.inventory.held().unwrap();
        self.player_state_mut().unwrap().item_in_use = Some(item);
        let s = self.player_state().unwrap();
        let remaining = s.item_use_ticks;
        if remaining <= 25 && remaining % 4 == 0 { self.eating_effect(w, item, 5); }
        self.player_state_mut().unwrap().item_use_ticks -= 1;
        // 1.0 EntityPlayer.onUpdate: the use finishes only on the authoritative world.
        if self.player_state().unwrap().item_use_ticks == 0 && !w.multiplayer_world {
            self.finish_item_use(w);
        }
    }
    /// EntityPlayer.func_35208_ae (onItemUseFinish).
    pub fn finish_item_use(&mut self, w: &mut World) {
        let Some(item) = self.player_state().unwrap().item_in_use else { return; };
        self.eating_effect(w, item, 16);
        if matches!(item.id,335|373) {
            let result=if item.id==335 {self.drink_milk10(w,item)}else{self.drink_potion10(w,item)};
            let inv=&mut self.player_state_mut().unwrap().inventory;
            inv.main[inv.current_item as usize]=Some(result);
        } else if let Some((amount, saturation)) = food_value(item.id) {
            let s = self.player_state_mut().unwrap();
            s.food.add_food(amount, saturation);
            let mut result = item;
            result.count -= 1;
            if item.id == 282 { result = ItemStack::new(281, 1, 0); }
            s.inventory.main[s.inventory.current_item as usize] = if result.count == 0 { None } else { Some(result) };
            let pitch=w.rand.next_float()*0.1+0.9;
            w.play_sound_at_entity(self,"random.burp",0.5,pitch);
            let effect = match item.id {
                322 => Some((super::super::potion::REGENERATION, 1.0)),
                365 => Some((super::super::potion::HUNGER, 0.3)),
                367 => Some((super::super::potion::HUNGER, 0.8)),
                375 => Some((super::super::potion::POISON, 1.0)),
                _ => None,
            };
            if let Some((id, probability)) = effect {
                if !w.multiplayer_world && w.rand.next_float() < probability {
                    self.add_potion_effect(super::super::potion::PotionEffect::new(id, if item.id==375 {100}else{600}, 0));
                }
            }
        }
        self.clear_item_use();
    }
    fn eating_effect(&mut self, w: &mut World, item: ItemStack, count: i32) {
        // EntityPlayer.func_35201_a: EnumAction.drink (potions and milk) uses World.rand.
        if matches!(item.id, 335 | 373) {
            let pitch = w.rand.next_float() * 0.1 + 0.9;
            w.play_sound_at_entity(self, "random.drink", 0.5, pitch);
        }
        if food_value(item.id).is_none() { return; }
        let pitch = -self.pitch * math::PI_F32 / 180.0;
        let yaw = -self.yaw * math::PI_F32 / 180.0;
        for _ in 0..count {
            let mut velocity = Vec3D::new((self.rand.next_float() as f64 - 0.5) * 0.1, w.math_random.next_double() * 0.1 + 0.1, 0.0,);
            velocity = rotate_eating(velocity, pitch, yaw);
            let mut position = Vec3D::new((self.rand.next_float() as f64 - 0.5) * 0.3, -self.rand.next_float() as f64 * 0.6 - 0.3, 0.6,);
            position = rotate_eating(position, pitch, yaw);
            position = position.add(self.pos.x, self.pos.y + self.eye_height() as f64, self.pos.z,);
            w.spawn_particle(&format!("iconcrack_{}", item.id), position, velocity.add(0.0, 0.05, 0.0),);
        }
        let volume = 0.5 + 0.5 * self.rand.next_int(2) as f32;
        let pitch = (self.rand.next_float() - self.rand.next_float()) * 0.2 + 1.0;
        w.play_sound_at_entity(self, "random.eat", volume, pitch);
    }
}
fn rotate_eating(v: Vec3D, pitch: f32, yaw: f32) -> Vec3D {
    let c = math::cos(pitch) as f64;
    let s = math::sin(pitch) as f64;
    let v = Vec3D::new(v.x, v.y * c + v.z * s, v.z * c - v.y * s);
    let c = math::cos(yaw) as f64;
    let s = math::sin(yaw) as f64;
    Vec3D::new(v.x * c + v.z * s, v.y, v.z * c - v.x * s)
}
pub fn food_value(id: i32) -> Option<(i32, f32)> {
    Some(match id {
        260 => (4, 0.3), 282 => (8, 0.6), 297 => (5, 0.6),
        319 | 363 => (3, 0.3), 320 | 364 => (8, 0.8), 322 => (10, 1.2),
        349 | 360 | 365 => (2, 0.3), 350 => (5, 0.6), 357 => (1, 0.1),
        375 => (2,0.8), 366 => (6, 0.6), 367 => (4, 0.1), _ => return None,
    })
}
