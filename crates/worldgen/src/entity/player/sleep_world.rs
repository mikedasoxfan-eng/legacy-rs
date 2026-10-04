use super::*;
impl World {
    pub fn all_players_fully_asleep(&self) -> bool {
        !self.players.is_empty()
            && self.player_entities.len() == self.players.len()
            && self.player_entities.iter().all(|id| {
                id.and_then(|id| self.entities.get(&id))
                    .and_then(Entity::player_state)
                    .is_some_and(|s| s.sleeping && s.sleep_timer >= 100)
            })
    }
    pub(crate) fn tick_sleeping_players(&mut self) {
        if !self.all_players_fully_asleep() {
            return;
        }
        if self.spawn_hostile_mobs && self.difficulty >= 1 && self.perform_sleep_spawning() {
            return;
        }
        let time = self.world_time.wrapping_add(24000);
        self.world_time = time - time % 24000;
        for id in self.player_entities.clone().into_iter().flatten() {
            if let Some(mut p) = self.entities.remove(&id) {
                if p.player_state().is_some_and(|s| s.sleeping) {
                    p.wake_up(self, false, true);
                }
                if let Some(i) = p.player_state().and_then(|s| s.player_index) {
                    self.players[i] = (p.pos.x, p.pos.y, p.pos.z);
                }
                self.entities.insert(id, p);
            }
        }
        self.info.rain_time = 0;
        self.info.raining = false;
        self.info.thunder_time = 0;
        self.info.thundering = false;
    }
    /// SpawnerAnimals.performSleepSpawning, including the spider jockey draw.
    pub fn perform_sleep_spawning(&mut self) -> bool {
        let mut spawned = false;
        for id in self.player_entities.clone().into_iter().flatten() {
            let Some(player) = self.entities.get(&id).cloned() else {
                continue;
            };
            for _ in 0..20 {
                let x = floor(player.pos.x) + self.rand.next_int(32) - self.rand.next_int(32);
                let z = floor(player.pos.z) + self.rand.next_int(32) - self.rand.next_int(32);
                let y = (floor(player.pos.y) + self.rand.next_int(16) - self.rand.next_int(16))
                    .clamp(1, 128);
                let name = ["Spider", "Zombie", "Skeleton"][self.rand.next_int(3) as usize];
                let mut sy = y;
                while sy > 2 && !self.is_block_normal_cube(x, sy - 1, z) {
                    sy -= 1;
                }
                while !self.night_spawn_location(x, sy, z) && sy < y + 16 && sy < 128 {
                    sy += 1;
                }
                if sy >= y + 16 || sy >= 128 {
                    continue;
                }
                let mut e = Entity::living(self, name).unwrap();
                e.set_location_and_angles(
                    (x as f32 + 0.5) as f64,
                    sy as f32 as f64,
                    (z as f32 + 0.5) as f64,
                    self.rand.next_float() * 360.0,
                    0.0,
                );
                if !e.get_can_spawn_here(self) {
                    continue;
                }
                let Some(path) = self.entity_path_world(&e, &player, 32.0) else {
                    continue;
                };
                if path.points.len() <= 1 {
                    continue;
                }
                let end = *path.points.last().unwrap();
                if (end[0] as f64 - player.pos.x).abs() >= 1.5
                    || (end[2] as f64 - player.pos.z).abs() >= 1.5
                    || (end[1] as f64 - player.pos.y).abs() >= 1.5
                {
                    continue;
                }
                let (bx, by, bz) = Entity::bed_exit(
                    self,
                    floor(player.pos.x),
                    floor(player.pos.y),
                    floor(player.pos.z),
                    1,
                )
                .unwrap_or((x, sy + 1, z));
                e.set_location_and_angles(
                    (bx as f32 + 0.5) as f64,
                    by as f64,
                    (bz as f32 + 0.5) as f64,
                    0.0,
                    0.0,
                );
                let mob = e.entity_id;
                self.entity_joined_world(e);
                if name == "Spider" && self.rand.next_int(100) == 0 {
                    let mut rider = Entity::living(self, "Skeleton").unwrap();
                    rider.set_location_and_angles(
                        (bx as f32 + 0.5) as f64,
                        by as f64,
                        (bz as f32 + 0.5) as f64,
                        0.0,
                        0.0,
                    );
                    rider.mount_entity(self, Some(mob));
                    self.entity_joined_world(rider);
                }
                if let Some(mut p) = self.entities.remove(&id) {
                    p.wake_up(self, true, false);
                    self.entities.insert(id, p);
                }
                if name != "Spider" {
                    if let Some(e) = self.entities.get_mut(&mob) {
                        e.rand.next_float();
                        e.rand.next_float();
                    }
                }
                spawned = true;
                break;
            }
        }
        spawned
    }
    fn night_spawn_location(&mut self, x: i32, y: i32, z: i32) -> bool {
        self.is_block_normal_cube(x, y - 1, z)
            && !self.is_block_normal_cube(x, y, z)
            && !crate::block::mat(self.get_block_material(x, y, z)).is_liquid
            && !self.is_block_normal_cube(x, y + 1, z)
    }
}
