//! `World.tick` and what it drives: weather, scheduled block updates (`TickUpdates`) and
//! random block ticks (`updateBlocksAndPlayCaveSounds`).

use super::chunk::LightKind;
use super::{behavior, RandSel, TickKey, World};
use crate::biome::Biome;
use crate::block;
use crate::math;

pub use super::java_hash::JavaHashSet;

/// `BiomeGenBase.getEnableSnow`
fn biome_snows(b: Biome) -> bool {
    matches!(b, Biome::Taiga | Biome::IceDesert | Biome::Tundra)
}

/// `BiomeGenBase.canSpawnLightningBolt`: rain falls (not snow).
fn biome_rains(b: Biome) -> bool {
    !biome_snows(b)
        && !matches!(
            b,
            Biome::Desert | Biome::IceDesert | Biome::Hell | Biome::Sky
        )
}

/// `WorldProvider.calculateCelestialAngle`
pub fn celestial_angle(world_time: i64, partial: f32) -> f32 {
    let t = (world_time % 24000) as i32;
    let mut a = (t as f32 + partial) / 24000.0f32 - 0.25f32;
    if a < 0.0 {
        a += 1.0;
    }
    if a > 1.0 {
        a -= 1.0;
    }
    let raw = a;
    a = 1.0f32 - ((((a as f64) * std::f64::consts::PI).cos() + 1.0) / 2.0) as f32;
    raw + (a - raw) / 3.0f32
}

impl World {
    /// `func_27162_g`: rain strength at a partial tick.
    pub fn rain_strength(&self, partial: f32) -> f32 {
        self.prev_raining_strength + (self.raining_strength - self.prev_raining_strength) * partial
    }

    /// `func_27166_f`
    pub fn thunder_strength(&self, partial: f32) -> f32 {
        (self.prev_thundering_strength
            + (self.thundering_strength - self.prev_thundering_strength) * partial)
            * self.rain_strength(partial)
    }

    /// `func_27161_C`
    pub fn is_raining(&self) -> bool {
        self.rain_strength(1.0) as f64 > 0.2
    }

    /// `func_27160_B`
    pub fn is_thundering(&self) -> bool {
        self.thunder_strength(1.0) as f64 > 0.9
    }

    /// `calculateSkylightSubtracted`
    pub fn calculate_skylight_subtracted(&self, partial: f32) -> i32 {
        // WorldProviderHell.calculateCelestialAngle is a constant 0.5.
        let angle = if self.dimension.is_hell_world() {
            0.5
        } else {
            celestial_angle(self.world_time, partial)
        };
        let mut v =
            1.0f32 - (math::cos(angle * std::f64::consts::PI as f32 * 2.0f32) * 2.0f32 + 0.5f32);
        v = v.clamp(0.0, 1.0);
        v = 1.0f32 - v;
        v = (v as f64 * (1.0 - (self.rain_strength(partial) * 5.0f32) as f64 / 16.0)) as f32;
        v = (v as f64 * (1.0 - (self.thunder_strength(partial) * 5.0f32) as f64 / 16.0)) as f32;
        v = 1.0f32 - v;
        (v * 11.0f32) as i32
    }

    /// What the World constructor does after reading level.dat: `calculateInitialSkylight`
    /// and `func_27163_E` (start at full rain strength if it is raining).
    pub fn init_weather_and_light(&mut self) {
        self.skylight_subtracted = self.calculate_skylight_subtracted(1.0);
        if self.info.raining {
            self.raining_strength = 1.0;
            if self.info.thundering {
                self.thundering_strength = 1.0;
            }
        }
    }

    /// `getBlockLightValue_do(x, y, z, true)`: light with the current sky darkening.
    pub fn get_block_light_value(&mut self, x: i32, y: i32, z: i32) -> i32 {
        self.block_light_value_do(x, y, z, true)
    }

    fn block_light_value_do(&mut self, x: i32, y: i32, z: i32, special: bool) -> i32 {
        if !(x >= -32000000 && z >= -32000000 && x < 32000000 && z <= 32000000) {
            return 15;
        }
        if special {
            let id = self.get_block_id(x, y, z);
            if matches!(id, 44 | 60 | 67 | 53) {
                let mut v = self.block_light_value_do(x, y + 1, z, false);
                for n in [
                    self.block_light_value_do(x + 1, y, z, false),
                    self.block_light_value_do(x - 1, y, z, false),
                    self.block_light_value_do(x, y, z + 1, false),
                    self.block_light_value_do(x, y, z - 1, false),
                ] {
                    if n > v {
                        v = n;
                    }
                }
                return v;
            }
        }
        if y < 0 {
            return 0;
        }
        let y = y.min(127);
        if !self.access_chunk_pub(x >> 4, z >> 4) {
            return 0; // EmptyChunk
        }
        let sub = self.skylight_subtracted;
        self.chunk(x >> 4, z >> 4)
            .block_light_value(x & 15, y, z & 15, sub)
    }

    /// `canBlockBeRainedOn`
    pub fn can_block_be_rained_on(&mut self, x: i32, y: i32, z: i32) -> bool {
        if !self.is_raining() {
            return false;
        }
        if !self.can_block_see_the_sky(x, y, z) {
            return false;
        }
        if self.find_top_solid_block(x, z) > y {
            return false;
        }
        biome_rains(self.terrain.biome_source.biome_at(x, z))
    }

    /// `updateWeather`
    fn update_weather(&mut self) {
        if self.dimension.has_no_sky() {
            return;
        }
        if self.lightning_flash > 0 {
            self.lightning_flash -= 1;
        }
        let thunder = self.info.thunder_time;
        if thunder <= 0 {
            self.info.thunder_time = if self.info.thundering {
                self.rand.next_int(12000) + 3600
            } else {
                self.rand.next_int(168000) + 12000
            };
        } else {
            self.info.thunder_time = thunder - 1;
            if thunder - 1 <= 0 {
                self.info.thundering = !self.info.thundering;
            }
        }
        let rain = self.info.rain_time;
        if rain <= 0 {
            self.info.rain_time = if self.info.raining {
                self.rand.next_int(12000) + 12000
            } else {
                self.rand.next_int(168000) + 12000
            };
        } else {
            self.info.rain_time = rain - 1;
            if rain - 1 <= 0 {
                self.info.raining = !self.info.raining;
            }
        }
        self.prev_raining_strength = self.raining_strength;
        self.raining_strength = if self.info.raining {
            (self.raining_strength as f64 + 0.01) as f32
        } else {
            (self.raining_strength as f64 - 0.01) as f32
        };
        self.raining_strength = self.raining_strength.clamp(0.0, 1.0);
        self.prev_thundering_strength = self.thundering_strength;
        self.thundering_strength = if self.info.thundering {
            (self.thundering_strength as f64 + 0.01) as f32
        } else {
            (self.thundering_strength as f64 - 0.01) as f32
        };
        self.thundering_strength = self.thundering_strength.clamp(0.0, 1.0);
    }

    /// `World.tick` (no sleeping players or mob spawning yet).
    pub fn tick(&mut self) {
        self.update_weather();
        self.tick_sleeping_players();
        self.perform_spawning(self.spawn_hostile_mobs, self.spawn_peaceful_mobs);
        self.unload_oldest_chunks();
        let s = self.calculate_skylight_subtracted(1.0);
        if s != self.skylight_subtracted {
            self.skylight_subtracted = s;
            self.renderers_dirty = true; // updateAllRenderers
        }
        let t = self.world_time + 1;
        if t % 40 == 0 {
            self.save_world(false);
        }
        self.world_time = t;
        self.tick_updates(false);
        self.random_ticks();
    }

    /// `TickUpdates`: run due scheduled block updates, at most 1000 per tick.
    pub fn tick_updates(&mut self, all: bool) -> bool {
        let n = self.tick_order.len().min(1000);
        for _ in 0..n {
            let &(time, id, key) = self.tick_order.iter().next().expect("tick queue");
            if !all && time > self.world_time {
                break;
            }
            self.tick_order.remove(&(time, id, key));
            self.pending_ticks.remove(&key);
            let r = 8;
            if self.check_chunks_exist(
                key.x - r,
                key.y - r,
                key.z - r,
                key.x + r,
                key.y + r,
                key.z + r,
            ) {
                let cur = self.get_block_id(key.x, key.y, key.z);
                if cur as i32 == key.block && cur > 0 {
                    behavior::update_tick(self, cur, key.x, key.y, key.z, RandSel::World);
                }
            }
        }
        !self.tick_order.is_empty()
    }

    /// `updateBlocksAndPlayCaveSounds`: random ticks within 9 chunks of every player.
    fn random_ticks(&mut self) {
        let mut positions = std::mem::take(&mut self.positions_to_update);
        positions.clear();
        for &(px, _, pz) in &self.players {
            let cx = math::floor_double(px / 16.0);
            let cz = math::floor_double(pz / 16.0);
            for dx in -9..=9 {
                for dz in -9..=9 {
                    let (x, z) = (dx + cx, dz + cz);
                    positions.insert(super::server::chunk_xz_to_int(x, z), (x, z));
                }
            }
        }
        if self.sound_counter > 0 {
            self.sound_counter -= 1;
        }
        let list: Vec<(i32, i32)> = positions.iter().collect();
        self.positions_to_update = positions;

        for (cx, cz) in list {
            let (x0, z0) = (cx * 16, cz * 16);
            // getChunkFromChunkCoords: singleplayer loads (and generates) it here.
            let real = self.access_chunk_pub(cx, cz);
            let block_at = |w: &World, x: i32, y: i32, z: i32| {
                if real {
                    w.chunk(cx, cz).block(x, y, z)
                } else {
                    0
                }
            };

            if self.sound_counter == 0 {
                self.update_lcg = self.update_lcg.wrapping_mul(3).wrapping_add(1013904223);
                let v = self.update_lcg >> 2;
                let (x, z, y) = (v & 15, v >> 8 & 15, v >> 16 & 127);
                let id = block_at(self, x, y, z);
                let (bx, bz) = (x + x0, z + z0);
                if id == 0
                    && self.get_full_block_light_value(bx, y, bz) <= self.rand.next_int(8)
                    && self.get_saved_light_value(LightKind::Sky, bx, y, bz) <= 0
                {
                    let (sx, sy, sz) = (bx as f64 + 0.5, y as f64 + 0.5, bz as f64 + 0.5);
                    if let Some(d) = self.closest_player_dist_sq(sx, sy, sz, 8.0) {
                        if d > 4.0 {
                            // playSoundEffect("ambient.cave.cave", 0.7, 0.8 + rand.nextFloat() * 0.2)
                            let pitch = 0.8 + self.rand.next_float() * 0.2;
                            self.play_sound(
                                "ambient.cave.cave",
                                crate::entity::Vec3D::new(sx, sy, sz),
                                0.7,
                                pitch,
                            );
                            self.sound_counter = self.rand.next_int(12000) + 6000;
                        }
                    }
                }
            }

            if self.rand.next_int(100000) == 0 && self.is_raining() && self.is_thundering() {
                self.update_lcg = self.update_lcg.wrapping_mul(3).wrapping_add(1013904223);
                let v = self.update_lcg >> 2;
                let (x, z) = (x0 + (v & 15), z0 + (v >> 8 & 15));
                let y = self.find_top_solid_block(x, z);
                if self.can_block_be_rained_on(x, y, z) {
                    let bolt = crate::entity::Entity::lightning(self, x as f64, y as f64, z as f64);
                    self.weather_effects.push(bolt);
                    self.lightning_strikes.push((x, y, z));
                    self.lightning_flash = 2;
                }
            }

            if self.rand.next_int(16) == 0 {
                self.update_lcg = self.update_lcg.wrapping_mul(3).wrapping_add(1013904223);
                let v = self.update_lcg >> 2;
                let (x, z) = (v & 15, v >> 8 & 15);
                let y = self.find_top_solid_block(x + x0, z + z0);
                let biome = self.terrain.biome_source.biome_at(x + x0, z + z0);
                if biome_snows(biome) && y >= 0 && y < 128 {
                    let block_light = if real {
                        self.chunk(cx, cz).saved_light(LightKind::Block, x, y, z)
                    } else {
                        0
                    };
                    if block_light < 10 {
                        let below = block_at(self, x, y - 1, z);
                        let cur = block_at(self, x, y, z);
                        if self.is_raining()
                            && cur == 0
                            && behavior::snow_can_place(self, x + x0, y, z + z0)
                            && below != 0
                            && below != block::ICE
                            && block::mat(block::material_of(below)).blocks_movement
                        {
                            self.set_block_with_notify(x + x0, y, z + z0, block::SNOW);
                        }
                        let below_meta = if real {
                            self.chunk(cx, cz).data.get(x, y - 1, z)
                        } else {
                            0
                        };
                        if below == block::WATER_STILL && below_meta == 0 {
                            self.set_block_with_notify(x + x0, y - 1, z + z0, block::ICE);
                        }
                    }
                }
            }

            for _ in 0..80 {
                self.update_lcg = self.update_lcg.wrapping_mul(3).wrapping_add(1013904223);
                let v = self.update_lcg >> 2;
                let (x, z, y) = (v & 15, v >> 8 & 15, v >> 16 & 127);
                let id = block_at(self, x, y, z);
                if block::props(id).ticks {
                    behavior::update_tick(self, id, x + x0, y, z + z0, RandSel::World);
                }
            }
        }
    }

    /// `getClosestPlayer(x, y, z, maxDist)` as a squared distance.
    fn closest_player_dist_sq(&self, x: f64, y: f64, z: f64, max: f64) -> Option<f64> {
        let mut best: Option<f64> = None;
        for &(px, py, pz) in &self.players {
            let d = (px - x) * (px - x) + (py - y) * (py - y) + (pz - z) * (pz - z);
            if (max < 0.0 || d < max * max) && best.is_none_or(|b| d < b) {
                best = Some(d);
            }
        }
        best
    }

    /// `joinEntityInSurroundings` for a player: load the 5x5 chunks around it.
    pub fn join_player_surroundings(&mut self, px: f64, pz: f64) {
        let cx = math::floor_double(px / 16.0);
        let cz = math::floor_double(pz / 16.0);
        for x in cx - 2..=cx + 2 {
            for z in cz - 2..=cz + 2 {
                self.access_chunk_pub(x, z);
            }
        }
    }

    /// Queue a scheduled update entry (for loading saves etc.).
    pub fn queue_tick(&mut self, key: TickKey, time: i64) {
        let id = self.next_tick_entry_id;
        self.next_tick_entry_id += 1;
        if let std::collections::hash_map::Entry::Vacant(e) = self.pending_ticks.entry(key) {
            e.insert((time, id));
            self.tick_order.insert((time, id, key));
        }
    }
}
