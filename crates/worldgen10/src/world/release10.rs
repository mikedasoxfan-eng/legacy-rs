//! Release 1.0 additions to World. Names reference the original MCP source.
use super::{chunk::LightKind, TickKey, World};

/// Block.useNeighborBrightness, initialized by Block's static initializer.
pub(crate) fn use_neighbor_brightness(id: u8) -> bool {
    matches!(id, 43 | 44 | 53 | 60 | 67 | 108 | 109 | 114)
}

impl super::Dimension {
    pub fn can_respawn_here(self) -> bool { self == Self::Overworld }
    pub fn is_surface_world(self) -> bool { self == Self::Overworld }
    pub fn cloud_height(self) -> f32 { if self == Self::End { 8.0 } else { 128.0 } }
    pub fn fixed_spawn(self) -> Option<(i32, i32, i32)> {
        (self == Self::End).then_some((100, 50, 0))
    }
    pub fn moon_phase(self, time: i64) -> i32 { (time / 24000) as i32 % 8 }
    pub fn end_fog_color(self, angle: f32) -> Option<crate::entity::Vec3D> {
        if self != Self::End { return None; }
        let brightness = (crate::math::cos(angle * std::f64::consts::PI as f32 * 2.0) * 2.0 + 0.5).clamp(0.0, 1.0);
        let scale = brightness * 0.0f32 + 0.15f32;
        Some(crate::entity::Vec3D::new(((128.0f32 / 255.0) * scale) as f64,
            ((128.0f32 / 255.0) * scale) as f64, ((160.0f32 / 255.0) * scale) as f64))
    }
}

impl World {
    /// IChunkProvider.func_40377_a, category 0=monster, 1=creature, 2=waterCreature.
    pub fn provider_spawn_list10(&mut self, category:i32,x:i32,y:i32,z:i32)->Vec<crate::weighted::SpawnListEntry> {
        if self.dimension==super::Dimension::Nether&&category==0&&self.nether_bridges10.contains(x,y,z) {
            return [("Blaze",10,2,3),("PigZombie",10,4,4),("MagmaCube",3,4,4)].into_iter()
                .map(|(entity,weight,min,max)|crate::weighted::SpawnListEntry{entity,weight,min,max}).collect();
        }
        let biome=self.terrain.biome_source.biome_at((x>>4)<<4,(z>>4)<<4);
        match category {0=>crate::weighted::monsters(biome),1=>crate::weighted::creatures(biome),2=>crate::weighted::water_creatures(biome),_=>vec![]}
    }
    pub fn closest_structure10(&mut self,name:&str,x:i32,y:i32,z:i32)->Option<[i32;3]> {
        if self.dimension!=super::Dimension::Overworld||name!="Stronghold" {return None;}
        self.strongholds.nearest(self.seed,&self.terrain.biome_source,x,y,z)
    }
    /// World.func_40474_a uses World.rand and makes no draw for an empty list.
    pub fn choose_spawn_entry10(&mut self, category:i32,x:i32,y:i32,z:i32)->Option<crate::weighted::SpawnListEntry> {
        let entries = self.provider_spawn_list10(category,x,y,z);
        if entries.is_empty() { None } else { crate::weighted::choose(&mut self.rand,&entries).copied() }
    }
    /// World.saveWorldIndirectly waits for the singleton File IO Thread.
    pub fn save_world_indirectly(&mut self) {
        self.save_world(true);
        crate::threaded_io10::FILE_IO.flush();
    }
    pub fn get_top_solid_or_liquid_block(&mut self, x: i32, z: i32) -> i32 {
        self.access_chunk_pub(x >> 4, z >> 4);
        for y in (1..128).rev() {
            let material = self.get_block_material(x, y, z);
            if crate::block::mat(material).blocks_movement && material != crate::block::MAT_LEAVES {
                return y + 1;
            }
        }
        -1
    }
    /// World.func_40476_b; `edge_only` is true for weather, false for population.
    pub fn can_block_freeze(&mut self, x: i32, y: i32, z: i32, edge_only: bool) -> bool {
        if self.terrain.biome_source.temperature_at_height(x, y, z) > 0.15 {
            return false;
        }
        if !(0..128).contains(&y) || self.get_saved_light_value(LightKind::Block, x, y, z) >= 10 {
            return false;
        }
        if !matches!(self.get_block_id(x, y, z), 8 | 9) || self.get_block_metadata(x, y, z) != 0 {
            return false;
        }
        !edge_only || [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter()
            .any(|(dx, dz)| self.get_block_material(x + dx, y, z + dz) != crate::block::MAT_WATER)
    }

    /// World.func_40478_r, shared by weather and ChunkProviderGenerate.populate.
    pub fn can_snow_at(&mut self, x: i32, y: i32, z: i32) -> bool {
        if self.terrain.biome_source.temperature_at_height(x, y, z) > 0.15 {
            return false;
        }
        if !(0..128).contains(&y) || self.get_saved_light_value(LightKind::Block, x, y, z) >= 10 {
            return false;
        }
        let below = self.get_block_id(x, y - 1, z);
        self.get_block_id(x, y, z) == 0
            && super::behavior::snow_can_place(self, x, y, z)
            && below != 0 && below != crate::block::ICE
            && crate::block::mat(crate::block::material_of(below)).is_solid
    }

    /// World.func_41083_e: restore a tick without checking whether chunks exist.
    pub fn schedule_block_update_from_load(&mut self, x: i32, y: i32, z: i32, block: i32, delay: i32) {
        let time = if block > 0 { self.world_time.wrapping_add(delay as i64) } else { 0 };
        self.queue_tick(TickKey { x, y, z, block }, time);
    }

    /// World.func_41081_a. Java returns null when no entries match the chunk.
    /// Iterate scheduledTickSet, retaining its Java HashSet order.
    pub fn pending_chunk_ticks(&mut self, cx: i32, cz: i32, remove: bool) -> Option<Vec<(TickKey, i64)>> {
        let (x, z) = (cx.wrapping_shl(4), cz.wrapping_shl(4));
        let keys: Vec<_> = self.scheduled_tick_set10.iter().filter(|k|
            k.x >= x && k.x < x.wrapping_add(16) && k.z >= z && k.z < z.wrapping_add(16)).collect();
        if keys.is_empty() { return None; }
        let mut result = Vec::with_capacity(keys.len());
        for key in keys {
            let (time, id) = self.pending_ticks[&key];
            result.push((key, time));
            if remove {
                self.pending_ticks.remove(&key);
                self.tick_order.remove(&(time, id, key));
                self.scheduled_tick_set10.remove(key.java_hash181(), key);
            }
        }
        Some(result)
    }

    /// World.getClosestVulnerablePlayer, preserving playerEntities order and ties.
    pub fn closest_vulnerable_player(&self, pos: crate::entity::Vec3D, radius: f64) -> Option<i32> {
        let mut best = None;
        let mut distance = -1.0;
        for id in self.player_entities.iter().flatten() {
            let Some(entity) = self.entities.get(id) else { continue; };
            let Some(player) = entity.player_state() else { continue; };
            if player.capabilities.disable_damage { continue; }
            let d = entity.pos.square_distance(pos);
            if (radius < 0.0 || d < radius * radius) && (distance == -1.0 || d < distance) {
                distance = d;
                best = Some(*id);
            }
        }
        best
    }

    /// World.func_41082_b: missing/empty chunks and out-of-range x/z use the caller's default.
    pub fn normal_cube_with_default(&mut self, x: i32, y: i32, z: i32, default: bool) -> bool {
        if !(-30000000..30000000).contains(&x) || !(-30000000..30000000).contains(&z)
            || !self.access_chunk_pub(x >> 4, z >> 4) { return default; }
        let p = crate::block::props(self.get_block_id(x, y, z));
        p.exists && crate::block::mat(p.material).blocks_movement && p.normal_render
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn loaded_ticks_deduplicate_and_remove_both_indexes() {
        let mut world = World::new(42, 7);
        world.world_time = 100;
        world.schedule_block_update_from_load(-1, 65, 0, 1, 20);
        world.schedule_block_update_from_load(-1, 65, 0, 1, 30);
        world.schedule_block_update_from_load(0, 65, 0, 1, -3);
        let ticks = world.pending_chunk_ticks(-1, 0, true).unwrap();
        assert_eq!(ticks.len(), 1);
        assert_eq!(ticks[0].1, 120);
        assert_eq!(world.pending_ticks.len(), 1);
        assert_eq!(world.tick_order.len(), 1);
        assert_eq!(world.scheduled_tick_set10.len(), 1);
        assert!(world.pending_chunk_ticks(-1, 0, false).is_none());
        assert_eq!(world.pending_chunk_ticks(0, 0, false).unwrap()[0].1, 97);
    }
    #[test]
    fn no_sky_provider_has_zero_sky_brightness_at_all_heights() {
        let mut world = World::with_dimension(42, 7, super::super::Dimension::Nether);
        for y in [-1, 64, 128, 1000] {
            assert_eq!(world.saved_light_with_neighbors181(LightKind::Sky, 0, y, 0), 0);
        }
    }
}
