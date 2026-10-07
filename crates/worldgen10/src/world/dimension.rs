//! Release `Minecraft.usePortal` and respawn across the Overworld, Nether and End.

use super::{Dimension, World};
use crate::scenario::LoadStep;
use crate::JavaRandom;

impl World {
    /// `new World(world, WorldProvider.getProviderForDimension(dimension))`: same save,
    /// a copy of the level info, its own chunks. `fresh_rand` stands in for the new
    /// world's unseeded `Random`s.
    pub fn for_dimension(&mut self, dimension: Dimension, fresh_rand: i64) -> World {
        let mut w = World::with_dimension(self.seed, fresh_rand, dimension);
        // new Random().nextInt(), then soundCounter = rand.nextInt(12000).
        w.update_lcg = JavaRandom::new(fresh_rand ^ 0x5DEECE66D).next_int_unbounded();
        w.sound_counter = w.rand.next_int(12000);
        w.info = self.info.clone();
        w.spawn = self.spawn;
        w.world_time = self.world_time;
        w.difficulty = self.difficulty;
        w.storage = self.storage.as_ref().and_then(|s| s.for_dimension(dimension).ok());
        std::mem::swap(&mut w.maps,&mut self.maps);w.map_id=self.map_id;
        // JVM-wide state that the original keeps in statics, not in the World.
        std::mem::swap(&mut w.next_entity_id, &mut self.next_entity_id);
        std::mem::swap(&mut w.entity_seed_source, &mut self.entity_seed_source);
        std::mem::swap(&mut w.explosion_seed_source, &mut self.explosion_seed_source);
        std::mem::swap(&mut w.chest_random, &mut self.chest_random);
        std::mem::swap(&mut w.item_random, &mut self.item_random);
        std::mem::swap(&mut w.math_random, &mut self.math_random);
        // calculateInitialSkylight, func_27163_E
        w.init_weather_and_light();
        w
    }

    /// `World.setEntityDead` for the player leaving through a portal: the entity leaves
    /// this world (its `isDead` is cleared again by the caller).
    fn take_player(&mut self, id: i32) -> Option<crate::entity::Entity> {
        let mut e = self.entities.remove(&id)?;
        if e.added_to_chunk {
            if let Some(c) = self.chunks.get_mut(&(e.chunk_x, e.chunk_z)) {
                c.entity_lists[e.chunk_y.clamp(0, 7) as usize].retain(|i| *i != id);
            }
        }
        e.added_to_chunk = false;
        self.loaded_entity_list.retain(|i| *i != id);
        // playerEntities.remove(player)
        if let Some(index) = e.player_state().and_then(|s| s.player_index) {
            if index < self.players.len() {
                self.players.remove(index);
                self.player_entities.remove(index);
            }
        }
        if let Some(s) = e.player_state_mut() {
            s.player_index = None;
        }
        Some(e)
    }

    /// `Minecraft.usePortal` for the singleplayer player `id`. Saves this world, returns the
    /// other dimension's world with the player in it (at, or in a new, portal). `fresh_rand`
    /// and `teleporter_rand` stand in for the unseeded `new World(...)` and `new Teleporter()`
    /// randoms.
    pub fn use_portal(
        &mut self,
        id: i32,
        fresh_rand: i64,
        teleporter_rand: i64,
        progress: &mut dyn FnMut(LoadStep),
    ) -> Option<World> {
        let dimension = if self.entities.get(&id)?.player_state()?.dimension == -1 {
            Dimension::Overworld
        } else { Dimension::Nether };
        self.use_portal_to(id, dimension, fresh_rand, teleporter_rand, progress)
    }

    /// Release Minecraft.usePortal(int): explicit destination, including the End.
    /// Client ChangeDimensionTo events should call this with Dimension::from_id.
    pub fn use_portal_to(
        &mut self, id: i32, dimension: Dimension, fresh_rand: i64,
        teleporter_rand: i64, progress: &mut dyn FnMut(LoadStep),
    ) -> Option<World> {
        let mut e = self.take_player(id)?;
        let old_dimension = e.player_state()?.dimension;
        e.player_state_mut()?.dimension = dimension.id();
        e.dead = false;
        let alive = |e: &crate::entity::Entity| !e.dead && e.living.health > 0;
        let scale = if old_dimension > -1 && dimension == Dimension::Nether { 0.125 }
            else if old_dimension == -1 && dimension != Dimension::Nether { 8.0 }
            else { 1.0 };
        let (mut x, mut z) = (e.pos.x * scale, e.pos.z * scale);
        if dimension == Dimension::Nether || dimension == Dimension::Overworld && alive(&e) {
            e.set_location_and_angles(x, e.pos.y, z, e.yaw, e.pitch);
            if alive(&e) { self.update_entity_with_optional_force(&mut e, false); }
        }
        let mut w = self.for_dimension(dimension, fresh_rand);
        if dimension == Dimension::End {
            // WorldProviderEnd.getEntrancePortalLocation / World.func_40472_j.
            x = 100.0; z = 0.0; e.pos.y = 50.0;
            e.set_location_and_angles(x, e.pos.y, z, 90.0, 0.0);
            if alive(&e) { w.update_entity_with_optional_force(&mut e, false); }
        }

        // changeWorld(world, title, player)
        let title = match dimension {
            Dimension::Nether => "Entering the Nether",
            Dimension::End => "Entering the End",
            Dimension::Overworld if old_dimension == -1 => "Leaving the Nether",
            Dimension::Overworld => "Leaving the End",
        };
        progress(LoadStep::Title(title));
        progress(LoadStep::Text(""));
        // saveWorldIndirectly: func_594_b("Saving level"), then "Saving chunks".
        progress(LoadStep::Text("Saving chunks"));
        self.save_world_indirectly();
        // func_6255_d(title) around the player.
        progress(LoadStep::Title(title));
        progress(LoadStep::Text("Building terrain"));
        // getSpawnPoint() with x, z replaced by (int) posX, (int) posZ.
        let level_spawn = w.spawn;
        w.spawn = (e.pos.x as i32, level_spawn.1, e.pos.z as i32);
        w.preload_spawn_area_with(&mut |p| progress(LoadStep::Progress(p)));
        w.spawn = level_spawn;
        progress(LoadStep::Text("Simulating world for a bit"));
        w.unload_oldest_chunks();
        // spawnPlayerWithLoadedChunks: the level's player tag was used up at the first
        // spawn, so this is entityJoinedWorld.
        w.entity_joined_world(e);

        // Back in usePortal: thePlayer.worldObj = theWorld.
        let e = w.entities.get(&id)?.clone();
        if alive(&e) && old_dimension < 1 {
            let mut e = e;
            let y = e.pos.y;
            let (yaw, pitch) = (e.yaw, e.pitch);
            e.set_location_and_angles(x, y, z, yaw, pitch);
            w.update_entity_with_optional_force(&mut e, false);
            w.entities.insert(id, e);
            let mut rand = JavaRandom::new(teleporter_rand);
            w.teleport_to_portal(id, &mut rand);
        }
        Some(w)
    }

    /// `World.setSpawnLocation`: move the spawn sideways until the column isn't air.
    pub fn set_spawn_location(&mut self) {
        if self.spawn.1 <= 0 {
            self.spawn.1 = self.height10 / 2;
        }
        let (mut x, _, mut z) = self.spawn;
        let mut attempts = 0;
        while self.get_first_uncovered_block(x, z) == 0 {
            x += self.rand.next_int(8) - self.rand.next_int(8);
            z += self.rand.next_int(8) - self.rand.next_int(8);
            attempts += 1;
            if attempts == 10000 { break; }
        }
        self.spawn.0 = x;
        self.spawn.2 = z;
        self.info.spawn = self.spawn;
    }

    /// `World.updateEntityList`: drop dead and unloaded entities now (the player object
    /// stays for `Minecraft.thePlayer`, as in `update_entities`).
    pub fn update_entity_list(&mut self) {
        let unloaded = std::mem::take(&mut self.unloaded_entity_list);
        for id in unloaded {
            self.loaded_entity_list.retain(|v| *v != id);
            if let Some(e) = self.entities.get(&id) {
                if e.added_to_chunk {
                    let (cx, cy, cz) = (e.chunk_x, e.chunk_y, e.chunk_z);
                    if let Some(c) = self.chunks.get_mut(&(cx, cz)) {
                        c.entity_lists[cy.clamp(0, 7) as usize].retain(|v| *v != id);
                    }
                }
            }
        }
        let mut i = 0;
        while i < self.loaded_entity_list.len() {
            let id = self.loaded_entity_list[i];
            let Some(mut e) = self.entities.remove(&id) else {
                i += 1;
                continue;
            };
            if let Some(parent) = e.riding {
                if self.entities.get(&parent).is_some_and(|p| !p.dead && p.ridden_by == Some(id)) {
                    self.entities.insert(id, e);
                    i += 1;
                    continue;
                }
                if let Some(p) = self.entities.get_mut(&parent) {
                    p.ridden_by = None;
                }
                e.riding = None;
            }
            if e.dead {
                if e.added_to_chunk {
                    if let Some(c) = self.chunks.get_mut(&(e.chunk_x, e.chunk_z)) {
                        c.entity_lists[e.chunk_y.clamp(0, 7) as usize].retain(|v| *v != id);
                    }
                }
                self.loaded_entity_list.remove(i);
                if e.player_state().is_some() {
                    self.entities.insert(id, e);
                }
            } else {
                self.entities.insert(id, e);
                i += 1;
            }
        }
    }

    /// `Minecraft.respawn(false, 0)` in a dimension where players can respawn (the caller
    /// runs `use_portal` first from the Nether). The dead player `old` is replaced by a new
    /// one with the same entity id, at its bed if the bed is still there, else at spawn.
    /// Returns the chat message to show (`tile.bed.notValid`), if any.
    pub fn respawn_player(&mut self, old: i32, progress: &mut dyn FnMut(LoadStep)) -> Option<&'static str> {
        self.respawn_player_with_inventory(old, false, progress)
    }

    /// Minecraft.respawn's keep-inventory flag (GuiWinGame uses true).
    pub fn respawn_player_with_inventory(
        &mut self, old: i32, keep_inventory: bool, progress: &mut dyn FnMut(LoadStep),
    ) -> Option<&'static str> {
        let mut chat = None;
        let bed = self.entities.get(&old).and_then(|e| e.player_state()).and_then(|s| s.spawn);
        let mut target = None;
        if let Some((bx, by, bz)) = bed {
            // EntityPlayer.func_25060_a
            for (dx, dz) in [(-3, -3), (3, -3), (-3, 3), (3, 3)] {
                self.provide_chunk((bx + dx) >> 4, (bz + dz) >> 4);
            }
            if self.get_block_id(bx, by, bz) == 26 {
                // Block.blockBed
                target = crate::entity::Entity::bed_exit(self, bx, by, bz, 0);
            }
            if target.is_none() {
                chat = Some("tile.bed.notValid");
            }
        }
        let at_bed = target.is_some();
        self.set_spawn_location();
        self.update_entity_list();
        // world.setEntityDead(thePlayer)
        let previous = self.take_player(old);
        // playerController.createPlayer: a new EntityPlayerSP for the session's user.
        let username = previous.as_ref().and_then(|e| e.player_state()).map_or("Player", |s| s.username.as_str());
        let mut e = crate::entity::player::controller::PlayerControllerSP::create_player(self, username);
        if keep_inventory {
            if let Some(previous) = previous.as_ref() { e.copy_player_from(previous); }
        }
        if let Some(s) = e.player_state_mut() {
            s.dimension = 0;
        }
        e.prepare_player_to_spawn(self);
        if let (true, Some((x, y, z))) = (at_bed, target) {
            if let Some(s) = e.player_state_mut() {
                s.spawn = bed;
            }
            e.set_location_and_angles(
                (x as f32 + 0.5) as f64,
                (y as f32 + 0.1) as f64,
                (z as f32 + 0.5) as f64,
                0.0,
                0.0,
            );
        }
        // flipPlayer
        crate::entity::player::controller::PlayerControllerSP::flip_player(&mut e,
            crate::entity::player::controller::PlayerControllerSP::saved_world_is_creative(self.info.game_type));
        // spawnPlayerWithLoadedChunks; then thePlayer.entityId = the old id.
        e.initialize_player_mode(crate::entity::player::controller::PlayerControllerSP::saved_world_is_creative(self.info.game_type));
        e.entity_id = old;
        let (px, pz) = (e.pos.x, e.pos.z);
        self.entity_joined_world(e);
        // func_6255_d("Respawning") around the new player.
        progress(LoadStep::Title("Respawning"));
        progress(LoadStep::Text("Building terrain"));
        let level_spawn = self.spawn;
        self.spawn = (px as i32, level_spawn.1, pz as i32);
        self.preload_spawn_area_with(&mut |p| progress(LoadStep::Progress(p)));
        self.spawn = level_spawn;
        progress(LoadStep::Text("Simulating world for a bit"));
        self.unload_oldest_chunks();
        chat
    }
}

impl World {
    /// Complete GuiWinGame's respawn(false, 0, true) simulation operation.
    /// Returns the replacement world and the optional invalid-bed chat message.
    pub fn return_from_end(
        &mut self, id: i32, fresh_rand: i64, teleporter_rand: i64,
        progress: &mut dyn FnMut(LoadStep),
    ) -> Option<(World, Option<&'static str>)> {
        let mut world = self.use_portal_to(id, Dimension::Overworld, fresh_rand, teleporter_rand, progress)?;
        let chat = world.respawn_player_with_inventory(id, true, progress);
        Some((world, chat))
    }
}

#[cfg(test)]
mod end_portal_travel {
    use crate::entity::player::PlayerEvent;
    use crate::world::{Dimension, World};

    /// BlockEndPortal.onEntityCollidedWithBlock → EntityPlayerSP.func_40182_b(1) raises the event the
    /// client turns into Minecraft.usePortal(1); that must land the player on the End's obsidian platform.
    #[test]
    fn standing_in_an_end_portal_reaches_the_end() {
        let mut w = World::new(42, 42);
        let id = w.spawn_player("Player");
        let mut p = w.entities.remove(&id).unwrap();
        let (x, y, z) = (p.pos.x.floor() as i32, p.pos.y.floor() as i32, p.pos.z.floor() as i32);
        p.on_entity_collided_with_block(&mut w, crate::block::END_PORTAL, x, y, z);
        let to = p.player_state().unwrap().events.iter().find_map(|e| match e {
            PlayerEvent::ChangeDimensionTo { dimension } => Some(*dimension),
            _ => None,
        });
        assert_eq!(to, Some(1));
        w.entities.insert(id, p);
        let mut end = w.use_portal_to(id, Dimension::from_id(1).unwrap(), 1, 2, &mut |_| {}).unwrap();
        assert_eq!(end.dimension, Dimension::End);
        let p = &end.entities[&id];
        assert_eq!(p.player_state().unwrap().dimension, 1);
        // Teleporter.placeInPortal (End): obsidian one below the feet; posY is the eye height (+1.62).
        let (px, py, pz) = (p.pos.x.floor() as i32, (p.pos.y - 1.62).round() as i32, p.pos.z.floor() as i32);
        eprintln!("End arrival at {:?}", (p.pos.x, p.pos.y, p.pos.z));
        assert_eq!(end.get_block_id(px, py - 1, pz), crate::block::OBSIDIAN);
    }
}
