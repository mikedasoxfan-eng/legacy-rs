//! Release 1.0 block callbacks and geometry.
use crate::{
    block,
    entity::{player::ItemStack, AxisAlignedBB as BB, Entity, Vec3D},
    world::{tile::TileEntity, RandSel},
    World,
};
pub static END_PORTAL_ALLOW: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);
pub fn texture(id: u8, side: i32, metadata: i32) -> Option<i32> {
    Some(match id {
        110 => match side {
            1 => 78,
            0 => 2,
            _ => 77,
        },
        111 => 76,
        112..=114 => 224,
        115 => {
            226 + if metadata >= 3 {
                2
            } else if metadata > 0 {
                1
            } else {
                0
            }
        }
        116 => match side {
            0 => 183,
            1 => 166,
            _ => 182,
        },
        117 => 157,
        118 => match side {
            1 => 138,
            0 => 155,
            _ => 154,
        },
        119 => 0,
        120 => match side {
            1 => 158,
            0 => 175,
            _ => 159,
        },
        121 => 175,
        122 => 167,
        _ => return None,
    })
}
pub fn initial_bounds(id: u8) -> Option<BB> {
    Some(match id {
        111 => BB::new(0.0, 0.0, 0.0, 1.0, 0.015625, 1.0),
        115 => BB::new(0.0, 0.0, 0.0, 1.0, 0.25, 1.0),
        116 => BB::new(0.0, 0.0, 0.0, 1.0, 0.75, 1.0),
        110..=122 => BB::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0),
        _ => return None,
    })
}
impl BB {
    pub fn translated_float_xz10(self, x: i32, y: i32, z: i32) -> Self {
        Self::new(
            (x as f32 + self.min_x as f32) as f64,
            y as f64,
            (z as f32 + self.min_z as f32) as f64,
            (x as f32 + self.max_x as f32) as f64,
            (y as f32 + self.max_y as f32) as f64,
            (z as f32 + self.max_z as f32) as f64,
        )
    }
}
impl World {
    /// World.func_41082_b: missing chunks return the supplied fallback.
    pub fn torch_support10(&mut self, x: i32, y: i32, z: i32) -> bool {
        if !self.block_exists(x, y, z) {
            return true;
        }
        let id = self.get_block_id(x, y, z);
        let p = block::props(id);
        p.exists && block::mat(p.material).translucent_solid && p.normal_render
    }
    pub fn trapdoor_support10(&mut self, x: i32, y: i32, z: i32) -> bool {
        let id = self.get_block_id(x, y, z);
        let p = block::props(id);
        (p.exists && block::mat(p.material).translucent_solid && p.normal_render) || id == 89
    }

    pub fn fence_connects10(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        let other = self.get_block_id(x, y, z);
        if other == id || other == 107 {
            return true;
        }
        let p = block::props(other);
        p.exists
            && block::mat(p.material).translucent_solid
            && p.normal_render
            && p.material != block::MAT_PUMPKIN
    }
    pub fn fence_bounds10(&mut self, id: u8, x: i32, y: i32, z: i32, height: f64) -> BB {
        let n = self.fence_connects10(id, x, y, z - 1);
        let s = self.fence_connects10(id, x, y, z + 1);
        let west = self.fence_connects10(id, x - 1, y, z);
        let east = self.fence_connects10(id, x + 1, y, z);
        BB::new(
            if west { 0.0 } else { 0.375 },
            0.0,
            if n { 0.0 } else { 0.375 },
            if east { 1.0 } else { 0.625 },
            height,
            if s { 1.0 } else { 0.625 },
        )
    }
}
pub fn multi_boxes(id: u8, metadata: i32) -> Option<Vec<BB>> {
    Some(match id {
        117 => vec![
            BB::new(0.4375, 0.0, 0.4375, 0.5625, 0.875, 0.5625),
            BB::new(0.0, 0.0, 0.0, 1.0, 0.125, 1.0),
        ],
        118 => vec![
            BB::new(0.0, 0.0, 0.0, 1.0, 0.3125, 1.0),
            BB::new(0.0, 0.0, 0.0, 0.125, 1.0, 1.0),
            BB::new(0.0, 0.0, 0.0, 1.0, 1.0, 0.125),
            BB::new(0.875, 0.0, 0.0, 1.0, 1.0, 1.0),
            BB::new(0.0, 0.0, 0.875, 1.0, 1.0, 1.0),
        ],
        120 => {
            let mut boxes = vec![BB::new(0.0, 0.0, 0.0, 1.0, 0.8125, 1.0)];
            if metadata & 4 != 0 {
                boxes.push(BB::new(0.3125, 0.8125, 0.3125, 0.6875, 1.0, 0.6875));
            }
            boxes
        }
        119 => vec![],
        _ => return None,
    })
}
pub fn added(w: &mut World, id: u8, x: i32, y: i32, z: i32) -> bool {
    match id {
        116 | 117 => w.container_added181(id, x, y, z),
        119 => {
            w.container_added181(id, x, y, z);
            if !END_PORTAL_ALLOW.load(std::sync::atomic::Ordering::Relaxed)
                && w.dimension != super::Dimension::Overworld
            {
                w.set_block_with_notify(x, y, z, 0);
            }
        }
        122 => w.schedule_block_update(x, y, z, id, 3),
        110..=121 => {}
        _ => return false,
    }
    true
}
pub fn removed(w: &mut World, id: u8, x: i32, y: i32, z: i32) -> bool {
    match id {
        117 => {
            if let Some(TileEntity::BrewingStand(b)) = w.get_block_tile_entity(x, y, z).cloned() {
                let tile_id = w.chunk(x >> 4, z >> 4).tile_entity_ids[&(x & 15, y, z & 15)];
                static RNG: std::sync::OnceLock<std::sync::Mutex<crate::JavaRandom>> =
                    std::sync::OnceLock::new();
                let mut rng = RNG
                    .get_or_init(|| {
                        std::sync::Mutex::new(crate::JavaRandom::new(
                            crate::entity::EntitySeedSource::Time.next_seed(),
                        ))
                    })
                    .lock()
                    .unwrap();
                for (slot, stack) in b.items.into_iter().enumerate() {
                    let Some(mut stack) = stack else { continue; };
                    let ox = rng.next_float() * 0.8 + 0.1;
                    let oy = rng.next_float() * 0.8 + 0.1;
                    let oz = rng.next_float() * 0.8 + 0.1;
                    while stack.count > 0 {
                        let count = (rng.next_int(21) + 10).min(stack.count);
                        stack.count -= count;
                        if let Some(TileEntity::BrewingStand(mut live)) = w.tile_object(tile_id).cloned() {
                            live.items[slot].as_mut().unwrap().count = stack.count;
                            w.update_tile_object(tile_id, TileEntity::BrewingStand(live));
                        }
                        let mut e = Entity::item(
                            w,
                            (x as f32 + ox) as f64,
                            (y as f32 + oy) as f64,
                            (z as f32 + oz) as f64,
                            crate::world::tile::ItemStack::new(stack.id, count, stack.damage),
                        );
                        e.motion = Vec3D::new(
                            (rng.next_gaussian() as f32 * 0.05) as f64,
                            (rng.next_gaussian() as f32 * 0.05 + 0.2) as f64,
                            (rng.next_gaussian() as f32 * 0.05) as f64,
                        );
                        w.entity_joined_world(e);
                    }
                }
            }
            w.remove_block_tile_entity(x, y, z);
        }
        116 | 119 => w.remove_block_tile_entity(x, y, z),
        110..=122 => {}
        _ => return false,
    }
    true
}
pub fn neighbor(w: &mut World, id: u8, x: i32, y: i32, z: i32) -> bool {
    match id {
        122 => w.schedule_block_update(x, y, z, id, 3),
        111 | 115 => check_plant(w, id, x, y, z),
        110..=121 => {}
        _ => return false,
    }
    true
}
pub fn can_stay(w: &mut World, id: u8, x: i32, y: i32, z: i32) -> bool {
    match id {
        111 => {
            y >= 0
                && y < 128
                && w.get_block_material(x, y - 1, z) == block::MAT_WATER
                && w.get_block_metadata(x, y - 1, z) == 0
        }
        115 => {
            (w.get_full_block_light_value(x, y, z) >= 8 || w.can_block_see_the_sky(x, y, z))
                && w.get_block_id(x, y - 1, z) == 88
        }
        _ => false,
    }
}
fn check_plant(w: &mut World, id: u8, x: i32, y: i32, z: i32) {
    if !can_stay(w, id, x, y, z) {
        let meta = w.get_block_metadata(x, y, z);
        super::behavior::drop_block_as_item(w, id, x, y, z, meta);
        w.set_block_with_notify(x, y, z, 0);
    }
}
pub fn tick(w: &mut World, id: u8, x: i32, y: i32, z: i32, rs: RandSel) -> bool {
    match id {
        110 => spread(w, id, x, y, z, rs),
        111 => check_plant(w, id, x, y, z),
        115 => {
            let meta = w.get_block_metadata(x, y, z);
            if meta < 3
                && w.terrain.biome_source.biome_at(x, z) == crate::Biome::Hell
                && w.rng(rs).next_int(15) == 0
            {
                w.set_block_metadata_with_notify(x, y, z, meta + 1);
            }
            check_plant(w, id, x, y, z);
        }
        122 => super::behavior::try_to_fall(w, id, x, y, z),
        112..=121 => {}
        _ => return false,
    }
    true
}
pub fn spread(w: &mut World, id: u8, x: i32, y: i32, z: i32, rs: RandSel) {
    if w.multiplayer_world {
        return;
    }
    if w.get_block_light_value(x, y + 1, z) < 4
        && block::light_opacity(w.get_block_id(x, y + 1, z)) > 2
    {
        w.set_block_with_notify(x, y, z, 3);
    } else if w.get_block_light_value(x, y + 1, z) >= 9 {
        for _ in 0..4 {
            let nx = x + w.rng(rs).next_int(3) - 1;
            let ny = y + w.rng(rs).next_int(5) - 3;
            let nz = z + w.rng(rs).next_int(3) - 1;
            let above = w.get_block_id(nx, ny + 1, nz);
            if w.get_block_id(nx, ny, nz) == 3
                && w.get_block_light_value(nx, ny + 1, nz) >= 4
                && block::light_opacity(above) <= 2
            {
                w.set_block_with_notify(nx, ny, nz, id);
            }
        }
    }
}
pub fn dragon_egg_teleport(w: &mut World, x: i32, y: i32, z: i32) {
    if w.get_block_id(x, y, z) != 122 || w.multiplayer_world {
        return;
    }
    for _ in 0..1000 {
        let nx = x + w.rand.next_int(16) - w.rand.next_int(16);
        let ny = y + w.rand.next_int(8) - w.rand.next_int(8);
        let nz = z + w.rand.next_int(16) - w.rand.next_int(16);
        if w.get_block_id(nx, ny, nz) != 0 {
            continue;
        }
        let meta = w.get_block_metadata(x, y, z);
        w.set_block_and_metadata_with_notify(nx, ny, nz, 122, meta);
        w.set_block_with_notify(x, y, z, 0);
        for _ in 0..128 {
            let t = w.rand.next_double();
            let vx = (w.rand.next_float() - 0.5) * 0.2;
            let vy = (w.rand.next_float() - 0.5) * 0.2;
            let vz = (w.rand.next_float() - 0.5) * 0.2;
            let px = nx as f64 + (x - nx) as f64 * t + (w.rand.next_double() - 0.5) * 1.0 + 0.5;
            let py = ny as f64 + (y - ny) as f64 * t + w.rand.next_double() * 1.0 - 0.5;
            let pz = nz as f64 + (z - nz) as f64 * t + (w.rand.next_double() - 0.5) * 1.0 + 0.5;
            w.spawn_particle(
                "portal",
                Vec3D::new(px, py, pz),
                Vec3D::new(vx as f64, vy as f64, vz as f64),
            );
        }
        return;
    }
}
pub fn cauldron_activate(player: &mut Entity, w: &mut World, x: i32, y: i32, z: i32) -> bool {
    if w.multiplayer_world {
        return true;
    }
    let Some(mut held) = player.held_item() else {
        return true;
    };
    let meta = w.get_block_metadata(x, y, z);
    if held.id == 326 {
        if meta < 3 {
            if !player.player_state().unwrap().capabilities.creative {
                let inv = &mut player.player_state_mut().unwrap().inventory;
                inv.main[inv.current_item as usize] = Some(ItemStack::new(325, 1, 0));
            }
            w.set_block_metadata_with_notify(x, y, z, 3);
        }
    } else if held.id == 374 && meta > 0 {
        let mut potion = ItemStack::new(373, 1, 0);
        if !player
            .player_state_mut()
            .unwrap()
            .inventory
            .add(&mut potion)
        {
            let item = Entity::item(
                w,
                x as f64 + 0.5,
                y as f64 + 1.5,
                z as f64 + 0.5,
                potion.into(),
            );
            w.entity_joined_world(item);
        }
        held.count -= 1;
        let inv = &mut player.player_state_mut().unwrap().inventory;
        inv.main[inv.current_item as usize] = if held.count <= 0 { None } else { Some(held) };
        w.set_block_metadata_with_notify(x, y, z, meta - 1);
    }
    true
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn source_geometry() {
        assert_eq!(multi_boxes(118, 0).unwrap().len(), 5);
        assert_eq!(multi_boxes(120, 4).unwrap().len(), 2);
        assert!(multi_boxes(119, 0).unwrap().is_empty());
        assert_eq!(texture(115, 1, 3), Some(228));
    }
}

pub fn display(w: &mut World, id: u8, x: i32, y: i32, z: i32, r: &mut crate::JavaRandom) -> bool {
    match id {
        110 => {
            if r.next_int(10) == 0 {
                let p = Vec3D::new(
                    (x as f32 + r.next_float()) as f64,
                    (y as f32 + 1.1) as f64,
                    (z as f32 + r.next_float()) as f64,
                );
                w.spawn_particle("townaura", p, Vec3D::default());
            }
        }
        117 => {
            let p = Vec3D::new(
                (x as f32 + 0.4 + r.next_float() * 0.2) as f64,
                (y as f32 + 0.7 + r.next_float() * 0.3) as f64,
                (z as f32 + 0.4 + r.next_float() * 0.2) as f64,
            );
            w.spawn_particle("smoke", p, Vec3D::default());
        }
        119 => {
            let p = Vec3D::new(
                (x as f32 + r.next_float()) as f64,
                (y as f32 + 0.8) as f64,
                (z as f32 + r.next_float()) as f64,
            );
            w.spawn_particle("smoke", p, Vec3D::default());
        }
        116 => {
            for bx in x - 2..=x + 2 {
                let mut bz = z - 2;
                while bz <= z + 2 {
                    if bx > x - 2 && bx < x + 2 && bz == z - 1 {
                        bz = z + 2;
                    }
                    if r.next_int(16) == 0 {
                        for by in y..=y + 1 {
                            if w.get_block_id(bx, by, bz) == 47 {
                                if !w.is_air_block((bx - x) / 2 + x, by, (bz - z) / 2 + z) {
                                    break;
                                }
                                let v = Vec3D::new(
                                    ((bx - x) as f32 + r.next_float()) as f64 - 0.5,
                                    ((by - y) as f32 - r.next_float() - 1.0) as f64,
                                    ((bz - z) as f32 + r.next_float()) as f64 - 0.5,
                                );
                                w.spawn_particle(
                                    "enchantmenttable",
                                    Vec3D::new(x as f64 + 0.5, y as f64 + 2.0, z as f64 + 0.5),
                                    v,
                                );
                            }
                        }
                    }
                    bz += 1;
                }
            }
        }
        111..=122 => {}
        _ => return false,
    }
    true
}

impl World {
    /// BlockPumpkin's snowman constructor is supplied by the entity area.
    pub fn pumpkin_added10(
        &mut self,
        x: i32,
        y: i32,
        z: i32,
        spawn: impl FnOnce(&mut World, Vec3D),
    ) {
        if self.get_block_id(x, y - 1, z) != 80 || self.get_block_id(x, y - 2, z) != 80 {
            return;
        }
        if !self.multiplayer_world {
            self.set_block_with_notify(x, y, z, 0);
            self.set_block_with_notify(x, y - 1, z, 0);
            self.set_block_with_notify(x, y - 2, z, 0);
            spawn(
                self,
                Vec3D::new(x as f64 + 0.5, y as f64 - 1.95, z as f64 + 0.5),
            );
        }
        for _ in 0..120 {
            let px = x as f64 + self.rand.next_double();
            let py = (y - 2) as f64 + self.rand.next_double() * 2.5;
            let pz = z as f64 + self.rand.next_double();
            self.spawn_particle("snowshovel", Vec3D::new(px, py, pz), Vec3D::default());
        }
    }
}
impl Entity {
    /// BlockEndPortal calls the entity area's dimension transition entry point.
    pub fn end_portal_collide10(&mut self, world: &World, change: impl FnOnce(&mut Entity, i32)) {
        if self.riding.is_none()
            && self.ridden_by.is_none()
            && self.player_state().is_some()
            && !world.multiplayer_world
        {
            change(self, 1);
        }
    }
}
