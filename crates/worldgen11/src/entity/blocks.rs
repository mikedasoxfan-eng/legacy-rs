//! Block geometry and fluids, using tables extracted from the original bytecode.
use super::{block_table as table, AxisAlignedBB as BB, MovingObjectPosition, Vec3D};
use crate::{block::*, math::floor_double as floor, World};

impl World {
    /// Original selected box (ladder changes singleton bounds here, not in collision).
    pub fn block_selected_box11(&mut self,id:u8,x:i32,y:i32,z:i32)->BB {
        if id==LADDER {
            let meta=self.get_block_metadata(x,y,z) as usize;
            if (2..=5).contains(&meta) {self.block_bounds[id as usize]=table::COLLISION[id as usize][meta].unwrap();}
        } else {self.set_block_bounds_based_on_state(id,x,y,z);}
        self.block_bounds[id as usize].translated(x as f64,y as f64,z as f64)
    }
    /// The moving-piston collision dependency of ItemBlock. Preserve Chunk's
    /// unguarded byte-array access for missing tiles and the original failure
    /// order; the ordinary world/collision APIs retain their existing interface.
    pub fn placement_collision_box181(&mut self, id: u8, x: i32, y: i32, z: i32)
        -> Result<Option<BB>, super::item::ItemError>
    {
        use super::item::ItemError;
        use crate::world::tile::TileEntity;
        if id != PISTON_MOVING { return Ok(self.block_collision_box(id, x, y, z)); }
        if !self.access_chunk_pub(x >> 4, z >> 4) { return Ok(None); }
        let key = (x & 15, y, z & 15);
        if !self.chunk(x >> 4, z >> 4).tile_entities.contains_key(&key) {
            let index = ((x & 15) << 11) | ((z & 15) << 7) | y;
            self.chunk(x >> 4, z >> 4).blocks.get(index as usize)
                .ok_or(ItemError::ArrayIndexOutOfBounds)?;
        }
        let Some(TileEntity::Piston(p)) = self.get_block_tile_entity(x, y, z).cloned() else { return Ok(None); };
        if p.block_id == 0 || p.block_id == PISTON_MOVING as i32 { return Ok(None); }
        // Block.blocksList[storedBlockID]: outside 0..256 throws, an empty slot is null.
        let id = u8::try_from(p.block_id).map_err(|_| ItemError::ArrayIndexOutOfBounds)?;
        if !props(id).exists { return Err(ItemError::NullPointer); }
        let progress = if p.extending { 1.0 - p.progress(0.0) } else { p.progress(0.0) };
        // A null carried-block collision returns before indexing the direction.
        let Some(mut b) = self.block_collision_box(id, x, y, z) else { return Ok(None); };
        let &(dx, dy, dz) = crate::world::pistons::DIRECTIONS.get(p.facing as usize)
            .ok_or(ItemError::ArrayIndexOutOfBounds)?;
        let (ox, oy, oz) = ((dx as f32 * progress) as f64,
            (dy as f32 * progress) as f64, (dz as f32 * progress) as f64);
        b.min_x -= ox; b.max_x -= ox; b.min_y -= oy; b.max_y -= oy; b.min_z -= oz; b.max_z -= oz;
        Ok(Some(b))
    }
    pub fn set_block_bounds_based_on_state(&mut self, id: u8, x: i32, y: i32, z: i32) {
        if id == FENCE_GATE {
            self.block_bounds[id as usize] = if self.get_block_metadata(x,y,z) & 3 == 0 || self.get_block_metadata(x,y,z) & 3 == 2 {
                BB::new(0.0,0.0,0.375,1.0,1.0,0.625)
            } else { BB::new(0.375,0.0,0.0,0.625,1.0,1.0) };
            return;
        }
        if matches!(id,85|113) {self.block_bounds[id as usize]=self.fence_bounds10(id,x,y,z,1.0);return;}
        if id==119 {self.block_bounds[id as usize]=BB::new(0.0,0.0,0.0,1.0,0.0625,1.0);return;}
        // BlockSign declares a state hook for both instances, but the standing
        // instance does nothing and retains the singleton's current bounds.
        if id == 63 {return;}
        if id == CAKE {
            self.block_bounds[id as usize] = crate::block::visual181::cake_bounds181(self.get_block_metadata(x,y,z));
            return;
        }
        if id == PISTON_MOVING {
            if let Some(crate::world::tile::TileEntity::Piston(p)) =
                self.get_block_tile_entity(x, y, z).cloned()
            {
                let carried: u8=p.block_id.try_into().expect("BlockPistonMoving block index");
                if carried != 0 && carried != PISTON_MOVING && props(carried).exists {
                    self.set_block_bounds_based_on_state(carried, x, y, z);
                    let mut b = self.block_bounds[p.block_id as usize];
                    let progress = if p.extending {
                        1.0 - p.progress(0.0)
                    } else {
                        p.progress(0.0)
                    };
                    let (dx, dy, dz) = crate::world::pistons::DIRECTIONS[p.facing as usize];
                    let (ox, oy, oz) = (
                        (dx as f32 * progress) as f64,
                        (dy as f32 * progress) as f64,
                        (dz as f32 * progress) as f64,
                    );
                    b.min_x -= ox;
                    b.max_x -= ox;
                    b.min_y -= oy;
                    b.max_y -= oy;
                    b.min_z -= oz;
                    b.max_z -= oz;
                    self.block_bounds[id as usize] = b;
                }
            }
            return;
        }
        if self.thin_block_bounds(id,x,y,z) { return; }
        if id == TRAPDOOR {
            self.block_bounds[id as usize] = crate::block::visual181::trapdoor_bounds181(self.get_block_metadata(x,y,z));
            return;
        }
        let m = self.get_block_metadata(x, y, z) as usize;
        if id == PORTAL {
            self.block_bounds[id as usize] =
                if self.get_block_id(x - 1, y, z) != id && self.get_block_id(x + 1, y, z) != id {
                    BB::new(0.375, 0.0, 0.0, 0.625, 1.0, 1.0)
                } else {
                    BB::new(0.0, 0.0, 0.375, 1.0, 1.0, 0.625)
                };
        } else if table::HAS_STATE_BOUNDS[id as usize] {
            // Invalid metadata leaves singleton instance fields untouched in Java.
            let retain = (matches!(id, PISTON_BASE | PISTON_STICKY_BASE | PISTON_EXTENSION)
                && m & 7 > 5)
                || (id == 77 && !(1..=4).contains(&(m & 7)));
            if !retain {
                self.block_bounds[id as usize] = table::STATE_BOUNDS[id as usize][m];
            }
        }
    }
    pub fn block_collision_box(&mut self, id: u8, x: i32, y: i32, z: i32) -> Option<BB> {
        if id == LADDER {return None;}
        if id == FENCE_GATE {
            let meta=self.get_block_metadata(x,y,z);
            if meta & 4 != 0 {return None;}
            return Some(if meta == 0 || meta == 2 {
                BB::new(x as f64,y as f64,(z as f32+0.375) as f64,(x+1) as f64,(y as f32+1.5) as f64,(z as f32+0.625) as f64)
            } else {
                BB::new((x as f32+0.375) as f64,y as f64,z as f64,(x as f32+0.625) as f64,(y as f32+1.5) as f64,(z+1) as f64)
            });
        }
        if matches!(id,85|113){return Some(self.fence_bounds10(id,x,y,z,1.5).translated_float_xz10(x,y,z));}
        if matches!(id,110..=122) {return match id {115=>None,_=>Some(self.block_bounds[id as usize].translated(x as f64,y as f64,z as f64))};}
        if id == CACTUS {return Some(crate::block::visual181::cactus_box181(x,y,z,false));}
        if matches!(id, 97 | 98 | 99 | 100 | 103) {
            return Some(BB::new(
                x as f64,
                y as f64,
                z as f64,
                (x + 1) as f64,
                (y + 1) as f64,
                (z + 1) as f64,
            ));
        }
        if !props(id).exists {
            return None;
        }
        if id == PISTON_MOVING {
            let Some(crate::world::tile::TileEntity::Piston(p)) =
                self.get_block_tile_entity(x, y, z).cloned()
            else {
                return None;
            };
            let progress = if p.extending {
                1.0 - p.progress(0.0)
            } else {
                p.progress(0.0)
            };
            return self.moving_piston_box(x, y, z, p.block_id, progress, p.facing);
        }
        let m = self.get_block_metadata(x, y, z) as usize;
        if matches!(id, DOOR_WOOD | DOOR_STEEL | TRAPDOOR) {
            self.set_block_bounds_based_on_state(id, x, y, z);
        }
        if id == LADDER && (2..=5).contains(&m) {
            self.block_bounds[id as usize] = table::COLLISION[id as usize][m].unwrap();
        }
        if table::BASE_COLLISION[id as usize]
            || matches!(
                id,
                LADDER | DOOR_WOOD | DOOR_STEEL | TRAPDOOR | STAIRS_WOOD | STAIRS_COBBLESTONE | STAIRS_BRICK | STAIRS_STONE_BRICK
            )
        {
            return Some(self.block_bounds[id as usize].translated(x as f64, y as f64, z as f64));
        }
        // These overrides perform addition in float before promotion to double.
        if id == FENCE_GATE && m & 4 != 0 { return None; }
        if id == FENCE || id == FENCE_GATE {
            return Some(BB::new(
                x as f64,
                y as f64,
                z as f64,
                (x + 1) as f64,
                (y as f32 + 1.5) as f64,
                (z + 1) as f64,
            ));
        }
        if id == SOUL_SAND {
            return Some(BB::new(
                x as f64,
                y as f64,
                z as f64,
                (x + 1) as f64,
                ((y + 1) as f32 - 0.125) as f64,
                (z + 1) as f64,
            ));
        }
        if id == CAKE {
            return Some(crate::block::visual181::cake_box181(x,y,z,m as i32,false));
        }
        if id == SNOW {
            if m & 7 < 3 {
                return None;
            }
            let mut b = self.block_bounds[id as usize].translated(x as f64, y as f64, z as f64);
            b.max_y = (y as f32 + 0.5) as f64;
            return Some(b);
        }
        table::COLLISION[id as usize][m].map(|b| b.translated(x as f64, y as f64, z as f64))
    }
    pub fn get_colliding_bounding_boxes(&mut self, entity: Option<i32>, query: BB) -> Vec<BB> {
        let mut boxes = Vec::new();
        for x in floor(query.min_x)..floor(query.max_x + 1.0) {
            for z in floor(query.min_z)..floor(query.max_z + 1.0) {
                if !self.block_exists(x, 64, z) {
                    continue;
                }
                for y in floor(query.min_y) - 1..floor(query.max_y + 1.0) {
                    let id = self.get_block_id(x, y, z);
                    let m = self.get_block_metadata(x, y, z) as usize;
                    if let Some(bounds)=crate::world::blocks10::multi_boxes(id,m as i32) {
                        for b in bounds {self.block_bounds[id as usize]=b;let b=b.translated(x as f64,y as f64,z as f64);if b.intersects(query){boxes.push(b);}}
                        self.block_bounds[id as usize]=match id {117=>BB::new(0.0,0.0,0.0,1.0,0.125,1.0),120=>BB::new(0.0,0.0,0.0,1.0,0.8125,1.0),_=>BB::new(0.0,0.0,0.0,1.0,1.0,1.0)};
                    }else if matches!(id, IRON_BARS | GLASS_PANE) {
                        for b in self.pane_boxes(id,x,y,z) {
                            self.block_bounds[id as usize]=b;
                            let b=b.translated(x as f64,y as f64,z as f64);
                            if b.intersects(query) { boxes.push(b); }
                        }
                    } else if id==114 {
                        for b in table::MULTI_COLLISION[67][m] {let b=b.translated(x as f64,y as f64,z as f64);if b.intersects(query){boxes.push(b);}}self.block_bounds[id as usize]=BB::new(0.0,0.0,0.0,1.0,1.0,1.0);
                    } else if matches!(id, STAIRS_WOOD | STAIRS_COBBLESTONE | STAIRS_BRICK | STAIRS_STONE_BRICK | PISTON_EXTENSION) {
                        for b in table::MULTI_COLLISION[id as usize][m] {
                            let b = b.translated(x as f64, y as f64, z as f64);
                            if b.intersects(query) {
                                boxes.push(b);
                            }
                        }
                        self.block_bounds[id as usize] = BB::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
                    } else {
                        if matches!(id, PISTON_BASE | PISTON_STICKY_BASE) {
                            self.block_bounds[id as usize] = BB::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
                        }
                        if let Some(b) = self.block_collision_box(id, x, y, z) {
                            if b.intersects(query) {
                                boxes.push(b);
                            }
                        }
                    }
                }
            }
        }
        let vehicle = entity
            .and_then(|id| self.entities.get(&id))
            .is_some_and(|e| {
                matches!(
                    e.kind,
                    super::EntityKind::Minecart(..) | super::EntityKind::Boat(..)
                )
            });
        for id in self.entities_within_aabb(entity, query.expand(0.25, 0.25, 0.25)) {
            let other = &self.entities[&id];
            if matches!(other.kind, super::EntityKind::Boat(..))
                && other.bounding_box.intersects(query)
            {
                boxes.push(other.bounding_box);
            }
            if vehicle && other.bounding_box.intersects(query) {
                boxes.push(other.bounding_box);
            }
        }
        boxes
    }
    pub fn collision_ray_trace(
        &mut self,
        x: i32,
        y: i32,
        z: i32,
        start: Vec3D,
        end: Vec3D,
        liquids: bool,
        ignore_no_box: bool,
    ) -> Option<MovingObjectPosition> {
        let id = self.get_block_id(x, y, z);
        let m = self.get_block_metadata(x, y, z);
        if ignore_no_box && self.block_collision_box(id, x, y, z).is_none() {
            return None;
        }
        if !props(id).exists
            || id == FIRE
            || (mat(material_of(id)).is_liquid && (!liquids || m != 0))
        {
            return None;
        }
        self.set_block_bounds_based_on_state(id, x, y, z);
        if matches!(id, 50 | 75 | 76) {
            let b = match m & 7 {
                1 => [0.0f32, 0.2, 0.35, 0.3, 0.8, 0.65],
                2 => [0.7, 0.2, 0.35, 1.0, 0.8, 0.65],
                3 => [0.35, 0.2, 0.0, 0.65, 0.8, 0.3],
                4 => [0.35, 0.2, 0.7, 0.65, 0.8, 1.0],
                _ => [0.4, 0.0, 0.4, 0.6, 0.6, 0.6],
            };
            self.block_bounds[id as usize] = BB::new(
                b[0] as f64,
                b[1] as f64,
                b[2] as f64,
                b[3] as f64,
                b[4] as f64,
                b[5] as f64,
            );
        }
        let start = start.add(-(x as f64), -(y as f64), -(z as f64));
        let end = end.add(-(x as f64), -(y as f64), -(z as f64));
        // Ray tracing uses selection bounds, not the physical collision box.
        self.block_bounds[id as usize]
            .ray_trace(start, end)
            .map(|h| match h {
                MovingObjectPosition::Block { side, hit, .. } => MovingObjectPosition::Block {
                    x,
                    y,
                    z,
                    side,
                    hit: hit.add(x as f64, y as f64, z as f64),
                },
                _ => unreachable!(),
            })
    }
    pub fn ray_trace_blocks(&mut self, start: Vec3D, end: Vec3D) -> Option<MovingObjectPosition> {
        self.ray_trace_blocks_options(start, end, false, false)
    }
    pub fn ray_trace_blocks_options(
        &mut self,
        start: Vec3D,
        end: Vec3D,
        liquids: bool,
        ignore_no_box: bool,
    ) -> Option<MovingObjectPosition> {
        let mut cursor = start;
        self.ray_trace_blocks_mut(&mut cursor, end, liquids, ignore_no_box)
    }
    /// Java rayTraceBlocks mutates its start vector; item consumers can observe it.
    pub fn ray_trace_blocks_mut(
        &mut self,
        cursor: &mut Vec3D,
        end: Vec3D,
        liquids: bool,
        ignore_no_box: bool,
    ) -> Option<MovingObjectPosition> {
        let mut start = *cursor;
        let nan = |v: Vec3D| v.x.is_nan() || v.y.is_nan() || v.z.is_nan();
        if nan(start) || nan(end) {
            return None;
        }
        let target = [floor(end.x), floor(end.y), floor(end.z)];
        let mut cell = [floor(start.x), floor(start.y), floor(start.z)];
        if let Some(h) = self.collision_ray_trace(
            cell[0],
            cell[1],
            cell[2],
            start,
            end,
            liquids,
            ignore_no_box,
        ) {
            return Some(h);
        }
        for _ in 0..201 {
            if nan(start) || cell == target {
                return None;
            }
            let s = [start.x, start.y, start.z];
            let d = [end.x - start.x, end.y - start.y, end.z - start.z];
            let mut planes = [999.0; 3];
            let mut times = [999.0; 3];
            for a in 0..3 {
                if target[a] > cell[a] {
                    planes[a] = cell[a] as f64 + 1.0;
                    times[a] = (planes[a] - s[a]) / d[a];
                } else if target[a] < cell[a] {
                    planes[a] = cell[a] as f64 + 0.0;
                    times[a] = (planes[a] - s[a]) / d[a];
                }
            }
            let a = if times[0] < times[1] && times[0] < times[2] {
                0
            } else if times[1] < times[2] {
                1
            } else {
                2
            };
            let side = if target[a] > cell[a] {
                [4, 0, 2][a]
            } else {
                [5, 1, 3][a]
            };
            let t = times[a];
            let mut next = [s[0] + d[0] * t, s[1] + d[1] * t, s[2] + d[2] * t];
            next[a] = planes[a];
            start = Vec3D::new(next[0], next[1], next[2]);
            *cursor = start;
            cell = [floor(start.x), floor(start.y), floor(start.z)];
            if side == 5 {
                cell[0] -= 1;
            }
            if side == 1 {
                cell[1] -= 1;
            }
            if side == 3 {
                cell[2] -= 1;
            }
            if let Some(h) = self.collision_ray_trace(
                cell[0],
                cell[1],
                cell[2],
                start,
                end,
                liquids,
                ignore_no_box,
            ) {
                return Some(h);
            }
        }
        None
    }
    pub fn is_material_in_bb(&mut self, b: BB, material: u8) -> bool {
        for x in floor(b.min_x)..floor(b.max_x + 1.0) {
            for y in floor(b.min_y)..floor(b.max_y + 1.0) {
                for z in floor(b.min_z)..floor(b.max_z + 1.0) {
                    if self.get_block_material(x, y, z) == material {
                        return true;
                    }
                }
            }
        }
        false
    }
    pub fn is_aabb_in_material(&mut self, b: BB, material: u8) -> bool {
        for x in floor(b.min_x)..floor(b.max_x + 1.0) {
            for y in floor(b.min_y)..floor(b.max_y + 1.0) {
                for z in floor(b.min_z)..floor(b.max_z + 1.0) {
                    if self.get_block_material(x, y, z) == material {
                        let m = self.get_block_metadata(x, y, z);
                        let surface = if m < 8 {
                            (y + 1) as f64 - m as f64 / 8.0
                        } else {
                            (y + 1) as f64
                        };
                        if surface >= b.min_y {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
    pub fn get_is_any_liquid(&mut self, b: BB) -> bool {
        let x0 = floor(b.min_x) - i32::from(b.min_x < 0.0);
        let y0 = floor(b.min_y) - i32::from(b.min_y < 0.0);
        let z0 = floor(b.min_z) - i32::from(b.min_z < 0.0);
        for x in x0..floor(b.max_x + 1.0) {
            for y in y0..floor(b.max_y + 1.0) {
                for z in z0..floor(b.max_z + 1.0) {
                    if mat(self.get_block_material(x, y, z)).is_liquid {
                        return true;
                    }
                }
            }
        }
        false
    }
    pub fn is_bounding_box_burning(&mut self, b: BB) -> bool {
        let (x0, y0, z0, x1, y1, z1) = (
            floor(b.min_x),
            floor(b.min_y),
            floor(b.min_z),
            floor(b.max_x + 1.0),
            floor(b.max_y + 1.0),
            floor(b.max_z + 1.0),
        );
        if !self.check_chunks_exist(x0, y0, z0, x1, y1, z1) {
            return false;
        }
        for x in x0..x1 {
            for y in y0..y1 {
                for z in z0..z1 {
                    if matches!(self.get_block_id(x, y, z), FIRE | LAVA_MOVING | LAVA_STILL) {
                        return true;
                    }
                }
            }
        }
        false
    }
    fn effective_flow_decay(&mut self, x: i32, y: i32, z: i32, material: u8) -> i32 {
        if self.get_block_material(x, y, z) != material {
            -1
        } else {
            let m = self.get_block_metadata(x, y, z);
            if m >= 8 {
                0
            } else {
                m
            }
        }
    }
    pub fn fluid_flow(&mut self, x: i32, y: i32, z: i32, material: u8) -> Vec3D {
        let mut v = Vec3D::default();
        let level = self.effective_flow_decay(x, y, z, material);
        for (dx, dz) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let mut decay = self.effective_flow_decay(x + dx, y, z + dz, material);
            if decay < 0 {
                if !mat(self.get_block_material(x + dx, y, z + dz)).blocks_movement {
                    decay = self.effective_flow_decay(x + dx, y - 1, z + dz, material);
                    if decay >= 0 {
                        let amount = decay - (level - 8);
                        v = v.add((dx * amount) as f64, 0.0, (dz * amount) as f64);
                    }
                }
            } else {
                let amount = decay - level;
                v = v.add((dx * amount) as f64, 0.0, (dz * amount) as f64);
            }
        }
        if self.get_block_metadata(x, y, z) >= 8 {
            let mut solid = false;
            for (dx, dy, dz) in [
                (0, 0, -1),
                (0, 0, 1),
                (-1, 0, 0),
                (1, 0, 0),
                (0, 1, -1),
                (0, 1, 1),
                (-1, 1, 0),
                (1, 1, 0),
            ] {
                if !solid {
                    let m = self.get_block_material(x + dx, y + dy, z + dz);
                    solid = m != material && m != MAT_ICE && mat(m).is_solid;
                }
            }
            if solid {
                v = v.normalize().add(0.0, -6.0, 0.0);
            }
        }
        v.normalize()
    }
    pub fn handle_material_acceleration(
        &mut self,
        b: BB,
        material: u8,
        motion: &mut Vec3D,
    ) -> bool {
        let (x0, y0, z0, x1, y1, z1) = (
            floor(b.min_x),
            floor(b.min_y),
            floor(b.min_z),
            floor(b.max_x + 1.0),
            floor(b.max_y + 1.0),
            floor(b.max_z + 1.0),
        );
        if !self.check_chunks_exist(x0, y0, z0, x1, y1, z1) {
            return false;
        }
        let mut found = false;
        let mut v = Vec3D::default();
        for x in x0..x1 {
            for y in y0..y1 {
                for z in z0..z1 {
                    if self.get_block_material(x, y, z) == material {
                        let meta = self.get_block_metadata(x, y, z);
                        let percent = (if meta >= 8 { 1 } else { meta + 1 }) as f32 / 9.0;
                        if y1 as f64 >= ((y + 1) as f32 - percent) as f64 {
                            found = true;
                            let f = self.fluid_flow(x, y, z, material);
                            v.x += f.x;
                            v.y += f.y;
                            v.z += f.z;
                        }
                    }
                }
            }
        }
        if v.length() > 0.0 {
            v = v.normalize();
            motion.x += v.x * 0.014;
            motion.y += v.y * 0.014;
            motion.z += v.z * 0.014;
        }
        found
    }
}
