use super::*;
use crate::entity::MovingObjectPosition;
impl Entity {
    pub fn view_position(&self, partial: f32) -> Vec3D {
        if partial == 1.0 {
            self.pos
        } else {
            self.prev_pos.add(
                (self.pos.x - self.prev_pos.x) * partial as f64,
                (self.pos.y - self.prev_pos.y) * partial as f64,
                (self.pos.z - self.prev_pos.z) * partial as f64,
            )
        }
    }
    pub fn look(&self, partial: f32) -> Vec3D {
        let (yaw, pitch) = if partial == 1.0 {
            (self.yaw, self.pitch)
        } else {
            (
                self.prev_yaw + (self.yaw - self.prev_yaw) * partial,
                self.prev_pitch + (self.pitch - self.prev_pitch) * partial,
            )
        };
        let yaw = -yaw * (math::PI_F32 / 180.0) - math::PI_F32;
        let pitch = -pitch * (math::PI_F32 / 180.0);
        let c = math::cos(yaw);
        let s = math::sin(yaw);
        let p = -math::cos(pitch);
        Vec3D::new((s * p) as f64, math::sin(pitch) as f64, (c * p) as f64)
    }
    /// EntityRenderer.getMouseOver (1.0): creative blocks reach 5 and entities 6
    /// (`extendedReach`); fireballs widen their box by `getCollisionBorderSize` 1.0.
    pub fn mouse_over(&self, w: &mut World, partial: f32) -> Option<MovingObjectPosition> {
        let start = self.view_position(partial);
        let look = self.look(partial);
        let creative = self.player_state().is_some_and(|s| s.capabilities.creative);
        let block_reach = controller::PlayerControllerSP::block_reach(creative) as f64;
        let block = w.ray_trace_blocks(start, start.add(look.x * block_reach, look.y * block_reach, look.z * block_reach));
        let reach = if controller::PlayerControllerSP::has_extended_entity_reach(creative) { 6.0 } else { block
            .as_ref()
            .map_or(block_reach, |hit| match hit {
                MovingObjectPosition::Block { hit, .. } => hit.distance(start),
                _ => block_reach,
            })
            .min(3.0) };
        let end = start.add(look.x * reach, look.y * reach, look.z * reach);
        let mut pointed = None;
        let mut distance = 0.0;
        for id in w.entities_within_aabb(
            Some(self.entity_id),
            self.bounding_box
                .add_coord(look.x * reach, look.y * reach, look.z * reach)
                .expand(1.0, 1.0, 1.0),
        ) {
            let e = &w.entities[&id];
            if !e.can_be_collided_with() {
                continue;
            }
            let border = if matches!(e.kind, crate::entity::EntityKind::Fireball(..) | crate::entity::EntityKind::SmallFireball(..)) { 1.0f32 } else { 0.1f32 };
            let b = e
                .bounding_box
                .expand(border as f64, border as f64, border as f64);
            if b.contains(start) {
                if 0.0 < distance || distance == 0.0 {
                    pointed = Some(id);
                    distance = 0.0;
                }
            } else if let Some(MovingObjectPosition::Block { hit, .. }) = b.ray_trace(start, end) {
                let d = start.distance(hit);
                if d < distance || distance == 0.0 {
                    pointed = Some(id);
                    distance = d;
                }
            }
        }
        pointed
            .map(|id| MovingObjectPosition::Entity {
                id,
                hit: w.entities[&id].pos,
            })
            .or(block)
    }
    /// Minecraft.clickMiddleMouseButton, including zero block coordinates on entity hits.
    pub fn pick_block(&mut self, w: &mut World, hit: Option<MovingObjectPosition>) {
        let Some(hit) = hit else { return; };
        let (x,y,z) = match hit { MovingObjectPosition::Block {x,y,z,..} => (x,y,z), _ => (0,0,0) };
        let block = match w.get_block_id(x,y,z) { 2=>3,43=>44,7=>1,b=>b as i32 };
        // Release 1.1 permits null item registry entries (including air), treating
        // them as not having subtypes, and still invokes the inventory selector.
        // Item.itemsList[block].getHasSubtypes() selects by metadata too (ItemCloth, the log and
        // stone-brick ItemMetadata, ItemSlab, ItemSapling, ItemLeaves, tall grass ItemColored).
        let subtypes = matches!(block, 6 | 17 | 18 | 31 | 35 | 44 | 98);
        let damage = if subtypes { w.get_block_metadata(x,y,z) } else { 0 };
        let creative = self.player_state().unwrap().capabilities.creative;
        self.player_state_mut().unwrap().inventory.select_item(block, damage, subtypes, creative);
    }
}

#[cfg(test)]
mod release11_pick_tests {
 use super::*;use crate::world::chunk::idx;
 #[test]
 fn middle_click_matches_original_11_bytecode(){let mut w=crate::entity::verify181::world();let mut p=Entity::player(&mut w,"probe");
  for line in include_str!("../../fixtures/pickblock11.txt").lines(){let v:Vec<i32>=line.split_whitespace().map(|s|s.parse().unwrap()).collect();let target=match v[0]{2=>3,43=>44,7=>1,id=>id};let state=p.player_state_mut().unwrap();state.capabilities.creative=v[2]!=0;state.inventory.main.fill(None);state.inventory.current_item=3;if v[4]!=0 {state.inventory.main[if v[4]==2{9}else{7}]=Some(ItemStack::new(target,1,v[1]+if v[4]==3{1}else{0}));}let before:Vec<_>=state.inventory.main.iter().map(|s|s.map(|s|(s.id,s.count,s.damage))).collect();let chunk=w.chunks.get_mut(&(0,0)).unwrap();for (x,y,z) in [(1,64,1),(0,0,0)]{chunk.blocks[idx(x,y,z)]=v[0] as u8;chunk.data.set(x,y,z,v[1]);}let hit=match v[3]{0=>None,1=>Some(MovingObjectPosition::Block{x:1,y:64,z:1,side:0,hit:Vec3D::default()}),_=>Some(MovingObjectPosition::Entity{id:999,hit:Vec3D::default()})};p.pick_block(&mut w,hit);let state=p.player_state().unwrap();assert_eq!(state.inventory.current_item,v[5],"{line}");assert_eq!(state.inventory.main.iter().map(|s|s.map(|s|(s.id,s.count,s.damage))).collect::<Vec<_>>(),before,"pick does not change slots {line}");
  }
 }
}
