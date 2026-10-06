use super::{AxisAlignedBB as BB, Entity, EntitySeedSource, Vec3D};
use crate::{math::floor_double as floor, JavaRandom, World};
impl World {
    pub fn set_entity_determinism(&mut self, entity_seed: i64, math_seed: i64, next_id: i32) {
        self.chest_random = JavaRandom::new(entity_seed + 400);
        self.item_random = JavaRandom::new(entity_seed + 500);
        self.explosion_seed_source = EntitySeedSource::Deterministic(entity_seed + 300);
        self.entity_seed_source = EntitySeedSource::Deterministic(entity_seed);
        self.display_seed_source = EntitySeedSource::Deterministic(10000);
        self.math_random = JavaRandom::new(math_seed);
        self.next_entity_id = next_id;
    }
    pub fn set_allowed_mob_spawns(&mut self, hostile: bool, peaceful: bool) {
        self.spawn_hostile_mobs = hostile;
        self.spawn_peaceful_mobs = peaceful;
    }
    pub fn closest_player(&self, pos: Vec3D, radius: f64) -> Option<usize> {
        let mut best = None;
        let mut distance = -1.0;
        for (i, &(x, y, z)) in self.players.iter().enumerate() {
            let d = pos.square_distance(Vec3D::new(x, y, z));
            if (radius < 0.0 || d < radius * radius) && (distance < 0.0 || d < distance) {
                distance = d;
                best = Some(i);
            }
        }
        best
    }
    pub fn closest_vulnerable_player(&self, pos: Vec3D, radius: f64) -> Option<usize> {
        let mut best = None;
        let mut distance = -1.0;
        for (i, &(x, y, z)) in self.players.iter().enumerate() {
            if self
                .player_entities
                .get(i)
                .and_then(|id| *id)
                .and_then(|id| self.entities.get(&id))
                .is_some_and(|e| {
                    e.player_state()
                        .is_some_and(|s| s.capabilities.disable_damage)
                })
            {
                continue;
            }
            let d = pos.square_distance(Vec3D::new(x, y, z));
            if (radius < 0.0 || d < radius * radius) && (distance < 0.0 || d < distance) {
                distance = d;
                best = Some(i);
            }
        }
        best
    }
    pub fn entity_joined_world(&mut self, mut e: Entity) -> bool {
        let cx = floor(e.pos.x / 16.0);
        let cz = floor(e.pos.z / 16.0);
        let player = matches!(e.kind, super::EntityKind::Player(..));
        if !player && !self.chunk_exists(cx, cz) {
            return false;
        }
        if player {
            self.provide_chunk(cx, cz);
            let index = self.players.len();
            self.player_entities.resize(index, None);
            self.player_entities.push(Some(e.entity_id));
            self.players.push((e.pos.x, e.pos.y, e.pos.z));
            e.player_state_mut().unwrap().player_index = Some(index);
        }
        self.file_entity(&mut e);
        let id = e.entity_id;
        self.entities.insert(id, e);
        self.loaded_entity_list.push(id);
        true
    }
    fn file_entity(&mut self, e: &mut Entity) {
        let cx = floor(e.pos.x / 16.0);
        let cy = floor(e.pos.y / 16.0).clamp(0, 7);
        let cz = floor(e.pos.z / 16.0);
        e.added_to_chunk = true;
        e.chunk_x = cx;
        e.chunk_y = cy;
        e.chunk_z = cz;
        let c = self.chunk_mut(cx, cz);
        c.has_entities = true;
        c.entity_lists[cy as usize].push(e.entity_id);
    }
    fn unfile_entity(&mut self, e: &Entity) {
        if e.added_to_chunk {
            if let Some(c) = self.chunks.get_mut(&(e.chunk_x, e.chunk_z)) {
                c.entity_lists[e.chunk_y.clamp(0, 7) as usize].retain(|id| *id != e.entity_id);
            }
        }
    }
    pub fn entities_within_aabb(&self, exclude: Option<i32>, b: BB) -> Vec<i32> {
        let mut result = Vec::new();
        for cx in floor((b.min_x - 2.0) / 16.0)..=floor((b.max_x + 2.0) / 16.0) {
            for cz in floor((b.min_z - 2.0) / 16.0)..=floor((b.max_z + 2.0) / 16.0) {
                if let Some(c) = self.chunks.get(&(cx, cz)) {
                    for cy in
                        floor((b.min_y - 2.0) / 16.0).max(0)..=floor((b.max_y + 2.0) / 16.0).min(7)
                    {
                        for id in &c.entity_lists[cy as usize] {
                            if Some(*id) != exclude {
                                if let Some(e) = self.entities.get(id) {
                                    if e.bounding_box.intersects(b) {
                                        result.push(*id);
                                        if let super::EntityKind::Dragon(s) = &e.kind {
                                            for part in s.parts {
                                                if Some(part) != exclude
                                                    && self.entities.get(&part).is_some_and(|p| {
                                                        p.bounding_box.intersects(b)
                                                    })
                                                {
                                                    result.push(part);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        result
    }
    pub fn check_if_aabb_is_clear(&self, b: BB) -> bool {
        self.entities_within_aabb(None, b).iter().all(|id| {
            let e = &self.entities[id];
            e.dead || !e.prevent_entity_spawning
        })
    }
    pub fn update_entity_with_optional_force(&mut self, e: &mut Entity, force: bool) {
        let x = floor(e.pos.x);
        let z = floor(e.pos.z);
        if force && !self.check_chunks_exist(x - 32, 0, z - 32, x + 32, 128, z + 32) {
            return;
        }
        e.last_tick_pos = e.pos;
        e.prev_yaw = e.yaw;
        e.prev_pitch = e.pitch;
        if force && e.added_to_chunk {
            if e.riding.is_some() {
                e.update_ridden(self);
            } else {
                e.on_update(self);
            }
        }
        if !e.pos.x.is_finite() {
            e.pos.x = e.last_tick_pos.x;
        }
        if !e.pos.y.is_finite() {
            e.pos.y = e.last_tick_pos.y;
        }
        if !e.pos.z.is_finite() {
            e.pos.z = e.last_tick_pos.z;
        }
        if !e.yaw.is_finite() {
            e.yaw = e.prev_yaw;
        }
        if !e.pitch.is_finite() {
            e.pitch = e.prev_pitch;
        }
        if let Some(index) = e.player_state().and_then(|s| s.player_index) {
            self.players[index] = (e.pos.x, e.pos.y, e.pos.z);
        }
        crate::profiler::start_section("chunkCheck");
        let (cx, cy, cz) = (
            floor(e.pos.x / 16.0),
            floor(e.pos.y / 16.0),
            floor(e.pos.z / 16.0),
        );
        if !e.added_to_chunk || (cx, cy, cz) != (e.chunk_x, e.chunk_y, e.chunk_z) {
            self.unfile_entity(e);
            if self.chunk_exists(cx, cz) {
                self.file_entity(e);
            } else {
                e.added_to_chunk = false;
            }
        }
        crate::profiler::end_section();
        if force && e.added_to_chunk {
            if let Some(id) = e.ridden_by {
                if let Some(mut rider) = self.entities.remove(&id) {
                    if !rider.dead && rider.riding == Some(e.entity_id) {
                        self.entities.insert(e.entity_id, e.clone());
                        self.update_entity_with_optional_force(&mut rider, true);
                        if let Some(parent) = self.entities.remove(&e.entity_id) {
                            *e = parent;
                        }
                    } else {
                        rider.riding = None;
                        e.ridden_by = None;
                    }
                    self.entities.insert(id, rider);
                } else {
                    e.ridden_by = None;
                }
            }
        }
    }
    pub fn update_entities(&mut self) {
        crate::profiler::start_section("entities");
        crate::profiler::start_section("global");
        let effects = std::mem::take(&mut self.weather_effects);
        for mut e in effects {
            e.on_update(self);
            if !e.dead {
                self.weather_effects.push(e);
            }
        }
        crate::profiler::end_start_section("remove");
        let unloaded = std::mem::take(&mut self.unloaded_entity_list);
        for id in unloaded {
            if let Some(e) = self.entities.remove(&id) {
                self.unfile_entity(&e);
                if self.fish_has_owner_reference(&e) {
                    self.entities.insert(id, e);
                }
            }
            self.loaded_entity_list.retain(|v| *v != id);
        }
        crate::profiler::end_start_section("regular");
        let mut i = 0;
        // Re-read length: entities appended by an onUpdate tick this same frame.
        while i < self.loaded_entity_list.len() {
            let id = self.loaded_entity_list[i];
            let Some(mut e) = self.entities.remove(&id) else {
                i += 1;
                continue;
            };
            if let Some(parent) = e.riding {
                if self
                    .entities
                    .get(&parent)
                    .is_some_and(|p| !p.dead && p.ridden_by == Some(id))
                {
                    self.entities.insert(id, e);
                    i += 1;
                    continue;
                }
                if let Some(p) = self.entities.get_mut(&parent) {
                    p.ridden_by = None;
                }
                e.riding = None;
            }
            if !e.dead {
                self.update_entity_with_optional_force(&mut e, true);
            }
            if let Some(index) = e.player_state().and_then(|s| s.player_index) {
                self.players[index] = (e.pos.x, e.pos.y, e.pos.z);
            }
            crate::profiler::start_section("remove");
            if e.dead {
                self.unfile_entity(&e);
                self.loaded_entity_list.remove(i);
                // PlayerEntities and Minecraft.thePlayer keep the dead object
                // until explicit removal/respawn, including its level.dat data.
                if e.player_state().is_some() || self.fish_has_owner_reference(&e) {
                    self.entities.insert(id, e);
                }
            } else {
                self.entities.insert(id, e);
                i += 1;
            }
            crate::profiler::end_section();
        }
        crate::profiler::end_start_section("tileEntities");
        self.tick_entity_tiles();
        crate::profiler::end_section();
        crate::profiler::end_section();
    }
    // Tile entities update after the entity list, including newly joined mobs next frame.
    pub fn join_entity_in_surroundings(&mut self, pos: Vec3D) {
        let cx = floor(pos.x / 16.0);
        let cz = floor(pos.z / 16.0);
        for x in cx - 2..=cx + 2 {
            for z in cz - 2..=cz + 2 {
                self.provide_chunk(x, z);
            }
        }
    }
    pub fn target_entity_id(&self, target: super::living::Target) -> Option<i32> {
        match target {
            super::living::Target::Entity(id) => Some(id),
            super::living::Target::Player(i) => self.player_entities.get(i).copied().flatten(),
        }
    }
}
