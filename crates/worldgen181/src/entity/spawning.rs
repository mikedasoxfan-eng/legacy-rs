use super::{AxisAlignedBB as BB, Entity, EntityKind, Vec3D};
use crate::{biome::Biome, block::*, math::floor_double as floor, JavaRandom, World};
impl Entity {
    pub fn get_can_spawn_here(&mut self, w: &mut World) -> bool {
        let (x, y, z) = (
            floor(self.pos.x),
            floor(self.bounding_box.min_y),
            floor(self.pos.z),
        );
        if let EntityKind::Slime { size, .. } = self.kind {
            let cx = x >> 4;
            let cz = z >> 4;
            let seed = w
                .seed
                .wrapping_add(cx.wrapping_mul(cx).wrapping_mul(4987142) as i64)
                .wrapping_add(cx.wrapping_mul(5947611) as i64)
                .wrapping_add((cz.wrapping_mul(cz) as i64).wrapping_mul(4392871))
                .wrapping_add(cz.wrapping_mul(389711) as i64)
                ^ 987234911;
            return (size == 1 || w.difficulty > 0)
                && self.rand.next_int(10) == 0
                && JavaRandom::new(seed).next_int(10) == 0
                && self.pos.y < 16.0;
        }
        if matches!(self.kind, EntityKind::Squid(..)) {
            return w.check_if_aabb_is_clear(self.bounding_box);
        }
        if matches!(self.kind, EntityKind::Ghast(..)) {
            if self.rand.next_int(20) != 0 {
                return false;
            }
            return self.base_spawn_check(w) && w.difficulty > 0;
        }
        if matches!(self.kind, EntityKind::PigZombie { .. }) {
            return w.difficulty > 0 && self.base_spawn_check(w);
        }
        if self.kind.is_mob() {
            if w.get_saved_light_value(crate::world::chunk::LightKind::Sky, x, y, z)
                > self.rand.next_int(32)
            {
                return false;
            }
            let mut light = w.get_block_light_value(x, y, z);
            if w.is_thundering() {
                let old = w.skylight_subtracted;
                w.skylight_subtracted = 10;
                light = w.get_block_light_value(x, y, z);
                w.skylight_subtracted = old;
            }
            if light > self.rand.next_int(8) {
                return false;
            }
        } else if w.get_block_id(x, y - 1, z) != GRASS || w.get_full_block_light_value(x, y, z) <= 8
        {
            return false;
        }
        self.base_spawn_check(w) && self.block_path_weight(w, x, y, z) >= 0.0
    }
    fn base_spawn_check(&self, w: &mut World) -> bool {
        w.check_if_aabb_is_clear(self.bounding_box)
            && w.get_colliding_bounding_boxes(Some(self.entity_id), self.bounding_box)
                .is_empty()
            && !w.get_is_any_liquid(self.bounding_box)
    }
    pub fn max_spawned_in_chunk(&self) -> i32 {
        match self.kind {
            EntityKind::Wolf(..) => 8,
            EntityKind::Ghast(..) => 1,
            _ => 4,
        }
    }
}
/// Original b1.8.1 biome IDs, independent of the terrain worker's biome implementation.
pub fn spawn_list181(biome: i32, category: i32) -> Vec<(&'static str, i32, i32, i32)> {
    if biome == 8 {
        return if category == 0 {
            vec![("Ghast", 10, 4, 4), ("PigZombie", 10, 4, 4)]
        } else {
            vec![]
        };
    }
    if biome == 9 {
        return if category == 1 {
            vec![("Chicken", 10, 4, 4)]
        } else {
            vec![]
        };
    }
    match category {
        0 => vec![
            ("Spider", 10, 4, 4),
            ("Zombie", 10, 4, 4),
            ("Skeleton", 10, 4, 4),
            ("Creeper", 10, 4, 4),
            ("Slime", 10, 4, 4),
            ("Enderman", 2, 4, 4),
        ],
        1 => {
            if matches!(biome, 0 | 2 | 7) {
                return vec![];
            }
            let mut list = vec![
                ("Sheep", 12, 4, 4),
                ("Pig", 10, 4, 4),
                ("Chicken", 10, 4, 4),
                ("Cow", 8, 4, 4),
            ];
            if biome == 4 {
                list.push(("Wolf", 5, 4, 4));
            }
            if biome == 5 {
                list.push(("Wolf", 8, 4, 4));
            }
            list
        }
        2 => vec![("Squid", 10, 4, 4)],
        _ => vec![],
    }
}
fn spawn_list(biome: Biome, category: i32) -> Vec<(&'static str, i32)> {
    spawn_list181(biome as i32, category)
        .into_iter()
        .map(|(name, weight, _, _)| (name, weight))
        .collect()
}
fn category(kind: &EntityKind) -> i32 {
    if kind.is_mob() || matches!(kind, EntityKind::Slime { .. } | EntityKind::Ghast(..)) {
        0
    } else if matches!(kind, EntityKind::Squid(..)) {
        2
    } else if kind.is_living() {
        1
    } else {
        -1
    }
}
impl World {
    pub fn perform_spawning(&mut self, hostile: bool, peaceful: bool) -> i32 {
        self.perform_spawning_with_identity(hostile, peaceful, |x, z| {
            // Identity hashes are a VM-dependent input, just like clock-seeded RNG.
            // Stable per-coordinate identities keep the port's traversal repeatable.
            (((x as u32 & 0x7fff) << 16 | (z as u32 & 0xffff)).wrapping_add(1) & 0x7fffffff) as i32
        })
    }
    pub fn perform_spawning_with_identity(
        &mut self, hostile: bool, peaceful: bool, mut identity: impl FnMut(i32, i32) -> i32,
    ) -> i32 {
        if !hostile && !peaceful {
            return 0;
        }
        let mut eligible = std::mem::take(&mut self.spawn_eligible_chunks);
        eligible.clear();
        for &(x, _, z) in &self.players {
            let cx = floor(x / 16.0);
            let cz = floor(z / 16.0);
            for dx in -8..=8 {
                for dz in -8..=8 {
                    let key = (cx + dx, cz + dz);
                    eligible.insert_with_identity(key.0 ^ key.1, identity(key.0, key.1), key);
                }
            }
        }
        let mut total = 0;
        for cat in 0..3 {
            if (cat == 0 && !hostile) || (cat != 0 && !peaceful) {
                continue;
            }
            let count = self
                .entities
                .values()
                .filter(|e| category(&e.kind) == cat)
                .count();
            let max = match cat {
                0 => 70,
                1 => 15,
                _ => 5,
            };
            if count > max * eligible.len() / 256 {
                continue;
            }
            'chunks: for (cx, cz) in eligible.iter() {
                let biome = self.terrain.biome_source.biome_at(cx << 4, cz << 4);
                let list = spawn_list(biome, cat);
                if list.is_empty() {
                    continue;
                }
                let sum = list.iter().map(|(_, n)| *n).sum();
                let mut pick = self.rand.next_int(sum);
                let mut name = list[0].0;
                for &(id, weight) in &list {
                    pick -= weight;
                    if pick < 0 {
                        name = id;
                        break;
                    }
                }
                let x = cx * 16 + self.rand.next_int(16);
                let y = self.rand.next_int(128);
                let z = cz * 16 + self.rand.next_int(16);
                if self.is_block_normal_cube(x, y, z)
                    || self.get_block_material(x, y, z)
                        != if cat == 2 { MAT_WATER } else { MAT_AIR }
                {
                    continue;
                }
                let mut spawned = 0;
                for _ in 0..3 {
                    let (mut px, mut py, mut pz) = (x, y, z);
                    for _ in 0..4 {
                        px += self.rand.next_int(6) - self.rand.next_int(6);
                        py += self.rand.next_int(1) - self.rand.next_int(1);
                        pz += self.rand.next_int(6) - self.rand.next_int(6);
                        let valid = if cat == 2 {
                            mat(self.get_block_material(px, py, pz)).is_liquid
                                && !self.is_block_normal_cube(px, py + 1, pz)
                        } else {
                            self.is_block_normal_cube(px, py - 1, pz)
                                && !self.is_block_normal_cube(px, py, pz)
                                && !mat(self.get_block_material(px, py, pz)).is_liquid
                                && !self.is_block_normal_cube(px, py + 1, pz)
                        };
                        if !valid {
                            continue;
                        }
                        let (fx, fy, fz) = (px as f32 + 0.5, py as f32, pz as f32 + 0.5);
                        let pos = Vec3D::new(fx as f64, fy as f64, fz as f64);
                        if self.closest_player(pos, 24.0).is_some() {
                            continue;
                        }
                        let dx = fx - self.spawn.0 as f32;
                        let dy = fy - self.spawn.1 as f32;
                        let dz = fz - self.spawn.2 as f32;
                        if dx * dx + dy * dy + dz * dz < 576.0 {
                            continue;
                        }
                        let mut e = Entity::living(self, name).unwrap();
                        e.set_location_and_angles(
                            pos.x,
                            pos.y,
                            pos.z,
                            self.rand.next_float() * 360.0,
                            0.0,
                        );
                        if e.get_can_spawn_here(self) {
                            spawned += 1;
                            let limit = e.max_spawned_in_chunk();
                            let id = e.entity_id;
                            let yaw = e.yaw;
                            self.entity_joined_world(e);
                            if name == "Spider" && self.rand.next_int(100) == 0 {
                                let mut skeleton = Entity::living(self, "Skeleton").unwrap();
                                skeleton.set_location_and_angles(pos.x, pos.y, pos.z, yaw, 0.0);
                                skeleton.riding = Some(id);
                                let rider = skeleton.entity_id;
                                self.entity_joined_world(skeleton);
                                if let Some(spider) = self.entities.get_mut(&id) {
                                    spider.ridden_by = Some(rider);
                                }
                            } else if name == "Sheep" {
                                let n = self.rand.next_int(100);
                                let color = if n < 5 {
                                    15
                                } else if n < 10 {
                                    7
                                } else if n < 15 {
                                    8
                                } else if n < 18 {
                                    12
                                } else if self.rand.next_int(500) == 0 {
                                    6
                                } else {
                                    0
                                };
                                if let Some(e) = self.entities.get_mut(&id) {
                                    if let EntityKind::Sheep { fleece_color, .. } = &mut e.kind {
                                        *fleece_color = color;
                                    }
                                }
                            }
                            if spawned >= limit {
                                continue 'chunks;
                            }
                        }
                        total += spawned;
                    }
                }
            }
        }
        self.spawn_eligible_chunks = eligible;
        total
    }
    pub(super) fn tick_entity_tiles(&mut self) {
        self.scan_tile_entities(|w, id| w.tick_tile_entity(id));
    }
    /// Update one Java tile object; detached objects can still be valid.
    pub fn tick_tile_entity(&mut self, id: u64) {
        let (x, y, z) = self.tile_objects[&id].position;
        if let Some(mut te) = self.tile_object(id).cloned() {
            if matches!(&te, crate::world::tile::TileEntity::Chest(_)) {
                self.tick_chest181(id);
            }
            if let crate::world::tile::TileEntity::MobSpawner { mob, delay } = &mut te {
                self.tick_mob_spawner(x, y, z, mob, delay);
            }
            let mut transition = None;
            if let crate::world::tile::TileEntity::Furnace(f) = &mut te {
                let (lit, dirty) = f.tick(false);
                transition = lit;
                if dirty {
                    self.mark_container_dirty(x, z);
                }
            }
            if let crate::world::tile::TileEntity::Piston(p) = te {
                self.tick_piston(id, p);
            } else {
                self.update_tile_object(id, te);
            }
            if let Some(lit) = transition {
                self.swap_furnace_block(lit, x, y, z);
            }
        }
    }
    fn tick_mob_spawner(&mut self, x: i32, y: i32, z: i32, mob: &str, delay: &mut i32) {
        if self
            .closest_player(
                Vec3D::new(x as f64 + 0.5, y as f64 + 0.5, z as f64 + 0.5),
                16.0,
            )
            .is_none()
        {
            return;
        }
        let p = Vec3D::new(
            (x as f32 + self.rand.next_float()) as f64,
            (y as f32 + self.rand.next_float()) as f64,
            (z as f32 + self.rand.next_float()) as f64,
        );
        self.spawn_particle("smoke", p, Vec3D::default());
        self.spawn_particle("flame", p, Vec3D::default());
        let animation = self
            .spawner_animations
            .entry((x, y, z))
            .or_insert((0.0, 0.0));
        animation.1 = animation.0;
        animation.0 += (1000.0f32 / (*delay as f32 + 200.0)) as f64;
        while animation.0 > 360.0 {
            animation.0 -= 360.0;
            animation.1 -= 360.0;
        }
        if *delay == -1 {
            *delay = 200 + self.rand.next_int(600);
        }
        if *delay > 0 {
            *delay -= 1;
            return;
        }
        for _ in 0..4 {
            let Some(mut e) = Entity::living(self, mob) else {
                return;
            };
            let b = BB::new(
                x as f64,
                y as f64,
                z as f64,
                (x + 1) as f64,
                (y + 1) as f64,
                (z + 1) as f64,
            )
            .expand(8.0, 4.0, 8.0);
            let count = self
                .entities_within_aabb(None, b)
                .iter()
                .filter(|id| self.entities[id].kind.name() == mob)
                .count();
            if count >= 6 {
                *delay = 200 + self.rand.next_int(600);
                return;
            }
            let px = x as f64 + (self.rand.next_double() - self.rand.next_double()) * 4.0;
            let py = (y + self.rand.next_int(3) - 1) as f64;
            let pz = z as f64 + (self.rand.next_double() - self.rand.next_double()) * 4.0;
            let yaw = self.rand.next_float() * 360.0;
            e.set_location_and_angles(px, py, pz, yaw, 0.0);
            if e.get_can_spawn_here(self) {
                let id = e.entity_id;
                self.entity_joined_world(e);
                for _ in 0..20 {
                    let p = Vec3D::new(
                        x as f64 + 0.5 + ((self.rand.next_float() as f64) - 0.5) * 2.0,
                        y as f64 + 0.5 + ((self.rand.next_float() as f64) - 0.5) * 2.0,
                        z as f64 + 0.5 + ((self.rand.next_float() as f64) - 0.5) * 2.0,
                    );
                    self.spawn_particle("smoke", p, Vec3D::default());
                    self.spawn_particle("flame", p, Vec3D::default());
                }
                if let Some(mut e) = self.entities.remove(&id) {
                    e.explosion_particle(self);
                    self.entities.insert(id, e);
                }
                *delay = 200 + self.rand.next_int(600);
            }
        }
    }
}

impl World {
    /// SpawnerAnimals.func_35957_a: passive packs during chunk population.
    /// The biome ID is the b1.8.1 registry ID; the terrain owner supplies it.
    pub fn spawn_population_animals(
        &mut self,
        biome_id: i32,
        x: i32,
        z: i32,
        width: i32,
        depth: i32,
        random: &mut JavaRandom,
    ) {
        let list = spawn_list181(biome_id, 1);
        if list.is_empty() {
            return;
        }
        while random.next_float() < 0.1 {
            // Vanilla draws the weighted choice from World.rand, not the population RNG.
            let mut choice = self.rand.next_int(list.iter().map(|e| e.1).sum());
            let entry = list
                .iter()
                .find(|entry| {
                    choice -= entry.1;
                    choice < 0
                })
                .copied()
                .unwrap();
            let count = entry.2 + random.next_int(1 + entry.3 - entry.2);
            let mut px = x + random.next_int(width);
            let mut pz = z + random.next_int(depth);
            let start_x = px;
            let start_z = pz;
            for _ in 0..count {
                let mut spawned = false;
                for _ in 0..4 {
                    if spawned {
                        break;
                    }
                    let y = self.find_top_solid_block(px, pz);
                    if self.is_block_normal_cube(px, y - 1, pz)
                        && !self.is_block_normal_cube(px, y, pz)
                        && !mat(self.get_block_material(px, y, pz)).is_liquid
                        && !self.is_block_normal_cube(px, y + 1, pz)
                    {
                        let mut entity = Entity::living(self, entry.0).unwrap();
                        entity.set_location_and_angles(
                            (px as f32 + 0.5) as f64,
                            y as f32 as f64,
                            (pz as f32 + 0.5) as f64,
                            random.next_float() * 360.0,
                            0.0,
                        );
                        let id = entity.entity_id;
                        self.entity_joined_world(entity);
                        if entry.0 == "Sheep" {
                            let n = self.rand.next_int(100);
                            let color = if n < 5 {
                                15
                            } else if n < 10 {
                                7
                            } else if n < 15 {
                                8
                            } else if n < 18 {
                                12
                            } else if self.rand.next_int(500) == 0 {
                                6
                            } else {
                                0
                            };
                            if let Some(e) = self.entities.get_mut(&id) {
                                if let EntityKind::Sheep { fleece_color, .. } = &mut e.kind {
                                    *fleece_color = color;
                                }
                            }
                        }
                        spawned = true;
                    }
                    px += random.next_int(5) - random.next_int(5);
                    pz += random.next_int(5) - random.next_int(5);
                    // The Z range incorrectly uses width in the original, even for rectangular areas.
                    while px < x || px >= x + width || pz < z || pz >= z + width {
                        px = start_x + random.next_int(5) - random.next_int(5);
                        pz = start_z + random.next_int(5) - random.next_int(5);
                    }
                }
            }
        }
    }
}
