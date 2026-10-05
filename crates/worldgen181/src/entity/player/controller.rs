use super::*;
use crate::block::*;
#[derive(Clone, Debug)]
pub struct PlayerControllerSP {
    pub target: (i32, i32, i32),
    pub cur_block_damage: f32,
    pub prev_block_damage: f32,
    pub step_sound_counter: f32,
    pub block_hit_wait: i32,
}
impl Default for PlayerControllerSP {
    fn default() -> Self {
        Self {
            target: (-1, -1, -1),
            cur_block_damage: 0.0,
            prev_block_damage: 0.0,
            step_sound_counter: 0.0,
            block_hit_wait: 0,
        }
    }
}
impl PlayerControllerSP {
    pub fn update(&mut self) {
        self.prev_block_damage = self.cur_block_damage;
    }
    pub fn reset(&mut self) {
        self.cur_block_damage = 0.0;
        self.block_hit_wait = 0;
    }
    pub fn damage_partial(&self, partial: f32) -> f32 {
        if self.cur_block_damage <= 0.0 {
            0.0
        } else {
            self.prev_block_damage + (self.cur_block_damage - self.prev_block_damage) * partial
        }
    }
    pub fn click_block(
        &mut self,
        p: &mut Entity,
        w: &mut World,
        x: i32,
        y: i32,
        z: i32,
        side: i32,
    ) {
        if p.player_state().unwrap().capabilities.creative {
            self.creative_remove(p,w,x,y,z,side);self.block_hit_wait=5;return;
        }
        let (fx, fy, fz) = super::placement::offset(x, y, z, side);
        if w.get_block_id(fx, fy, fz) == FIRE {
            w.auxiliary_effect(1004, (fx, fy, fz), 0);
            w.set_block_with_notify(fx, fy, fz, 0);
        }
        let id = w.get_block_id(x, y, z);
        if id > 0 && self.cur_block_damage == 0.0 {
            p.block_clicked(w, id, x, y, z);
        }
        if id > 0 && p.relative_block_hardness(w, id) >= 1.0 {
            self.remove_block(p, w, x, y, z, side);
        }
    }
    pub fn dig(&mut self, p: &mut Entity, w: &mut World, x: i32, y: i32, z: i32, side: i32) {
        if p.player_state().unwrap().capabilities.creative {
            self.block_hit_wait-=1;
            if self.block_hit_wait<=0 {self.block_hit_wait=5;self.creative_remove(p,w,x,y,z,side);}return;
        }
        if self.block_hit_wait > 0 {
            self.block_hit_wait -= 1;
            return;
        }
        if self.target == (x, y, z) {
            let id = w.get_block_id(x, y, z);
            if id == 0 {
                return;
            }
            self.cur_block_damage += p.relative_block_hardness(w, id);
            if self.step_sound_counter % 4.0 == 0.0 {
                let (name, volume, pitch) = crate::world::events::step_sound(id);
                w.direct_sound(
                    name,
                    Some(Vec3D::new(
                        (x as f32 + 0.5) as f64,
                        (y as f32 + 0.5) as f64,
                        (z as f32 + 0.5) as f64,
                    )),
                    (volume + 1.0) / 8.0,
                    pitch * 0.5,
                );
            }
            self.step_sound_counter += 1.0;
            if self.cur_block_damage >= 1.0 {
                self.remove_block(p, w, x, y, z, side);
                self.cur_block_damage = 0.0;
                self.prev_block_damage = 0.0;
                self.step_sound_counter = 0.0;
                self.block_hit_wait = 5;
            }
        } else {
            self.cur_block_damage = 0.0;
            self.prev_block_damage = 0.0;
            self.step_sound_counter = 0.0;
            self.target = (x, y, z);
        }
    }
    fn creative_remove(&mut self,_p:&mut Entity,w:&mut World,x:i32,y:i32,z:i32,side:i32)->bool {
        let (fx,fy,fz)=super::placement::offset(x,y,z,side);
        if w.get_block_id(fx,fy,fz)==FIRE {w.auxiliary_effect(1004,(fx,fy,fz),0);w.set_block_with_notify(fx,fy,fz,0);}
        let id=w.get_block_id(x,y,z);if id==0{return false;}
        let meta=w.get_block_metadata(x,y,z);w.auxiliary_effect(2001,(x,y,z),id as i32+meta*256);
        let removed=w.set_block_with_notify(x,y,z,0);
        if removed && id==46 && meta&1!=0 {w.ignite_tnt(x,y,z);}removed
    }
    pub fn remove_block(
        &mut self,
        p: &mut Entity,
        w: &mut World,
        x: i32,
        y: i32,
        z: i32,
        _side: i32,
    ) -> bool {
        let id = w.get_block_id(x, y, z);
        if id == 0 {
            return false;
        }
        let meta = w.get_block_metadata(x, y, z);
        w.auxiliary_effect(2001, (x, y, z), id as i32 + meta * 256);
        let removed = w.set_block_with_notify(x, y, z, 0);
        if removed && id == 46 && meta & 1 != 0 {
            w.ignite_tnt(x, y, z);
        }
        if removed && id == 97 {w.silverfish_destroyed181(x,y,z);}
        let can = p.can_harvest(id);
        if let Some(s) = p.held_item() {
            let cost = if matches!(s.id, 267 | 268 | 272 | 276 | 283) {
                2
            } else if matches!(s.id,256..=258|269..=271|273..=275|277..=279|284..=286)
                || (s.id == 359 && matches!(id, 18 | 30 | 31 | 106))
            {
                1
            } else {
                0
            };
            p.damage_held(cost);
            // ItemStack.onDestroyBlock: ItemTool and ItemSword report a use.
            if matches!(s.id, 267 | 268 | 272 | 276 | 283 | 256..=258 | 269..=271 | 273..=275 | 277..=279 | 284..=286) {
                p.add_stat(super::stat::used(s.id), 1);
            }
        }
        if removed && can {
            p.harvest_block(w, id, x, y, z, meta);
        }
        removed
    }
}
impl Entity {
    pub fn held_item(&self) -> Option<ItemStack> {
        self.player_state()?.inventory.held()
    }
    pub fn can_harvest(&self, block: u8) -> bool {
        registry::can_harvest(self.held_item().map_or(-1, |s| s.id), block)
    }
    pub fn relative_block_hardness(&self, w: &mut World, id: u8) -> f32 {
        let hardness = registry::hardness(id);
        if hardness < 0.0 {
            0.0
        } else if !self.can_harvest(id) {
            1.0 / hardness / 100.0
        } else {
            let mut strength = registry::strength(self.held_item().map_or(-1, |s| s.id), id);
            if self.is_inside_material(w, MAT_WATER) {
                strength /= 5.0;
            }
            if !self.on_ground {
                strength /= 5.0;
            }
            strength / hardness / 30.0
        }
    }
    pub fn damage_held(&mut self, amount: i32) {
        let inv = &mut self.player_state_mut().unwrap().inventory;
        let mut broke = None;
        if let Some(s) = inv.main[inv.current_item as usize].as_mut() {
            if s.damage_item(amount) {
                broke = Some(s.id);
            }
            if s.count == 0 {
                inv.main[inv.current_item as usize] = None;
            }
        }
        if let Some(id) = broke {
            self.add_stat(super::stat::broken(id), 1);
        }
    }
    pub fn block_clicked(&mut self, w: &mut World, id: u8, x: i32, y: i32, z: i32) {
        if id==25 { w.trigger_note(x,y,z); }
        if matches!(id, 64 | 71 | 69 | 77 | 96) {
            self.activate_block(w, x, y, z);
        }
        if matches!(id, 73 | 74) {
            w.redstone_display(x, y, z);
            if id == 73 {
                w.set_block_with_notify(x, y, z, 74);
            }
        }
    }
    pub fn harvest_block(&mut self, w: &mut World, id: u8, x: i32, y: i32, z: i32, meta: i32) {
        // Block.harvestBlock counts the block first; BlockSnow counts it after dropping.
        if id != 78 {
            self.add_stat(super::stat::mine(id as i32), 1);
        }
        if matches!(id,18|106) && self.held_item().is_some_and(|s| s.id == 359) {
            super::block_hooks::drop_stack(w, x, y, z, ItemStack::new(id as i32, 1, if id==18 {meta & 3}else{meta}));
        } else if id == 78 {
            super::block_hooks::drop_stack(w, x, y, z, ItemStack::new(332, 1, 0));
            self.add_stat(super::stat::mine(78), 1);
        } else {
            crate::world::behavior::drop_block_as_item(w, id, x, y, z, meta);
        }
        if id == 79 {
            let m = mat(w.get_block_material(x, y - 1, z));
            if m.blocks_movement || m.is_liquid {
                w.set_block_with_notify(x, y, z, 8);
            }
        }
    }
}
