//! Literal singleplayer BlockPistonBase/Extension/Moving and TileEntityPiston.
use super::{tile::TileEntity, World};
use crate::{
    block::*,
    entity::AxisAlignedBB as BB,
    nbt::{Compound, Tag},
};

pub static IGNORE_UPDATES:std::sync::atomic::AtomicBool=std::sync::atomic::AtomicBool::new(false);

/// PistonBlockTextures.field_31057_a.
pub const OPPOSITE_SIDES: [i32; 6] = [1, 0, 3, 2, 5, 4];

pub const DIRECTIONS: [(i32, i32, i32); 6] = [
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
    (-1, 0, 0),
    (1, 0, 0),
];

#[derive(Clone, Debug, PartialEq)]
pub struct Piston {
    pub block_id: i32,
    pub block_data: i32,
    pub facing: i32,
    pub extending: bool,
    pub render_head: bool,
    pub progress: f32,
    pub last_progress: f32,
}
impl Piston {
    pub fn new(
        block_id: impl Into<i32>,
        block_data: i32,
        facing: i32,
        extending: bool,
        render_head: bool,
    ) -> Self {
        Self {
            block_id: block_id.into(),
            block_data,
            facing,
            extending,
            render_head,
            progress: 0.0,
            last_progress: 0.0,
        }
    }
    pub fn progress(&self, partial: f32) -> f32 {
        let partial = if partial > 1.0 { 1.0 } else { partial };
        self.last_progress + (self.progress - self.last_progress) * partial
    }
    pub fn offset(&self, partial: f32) -> [f32; 3] {
        self.offset181(partial).expect("TileEntityPiston direction index")
    }
    pub fn offset181(&self, partial: f32) -> Result<[f32; 3], super::tile_nbt181::TileError181> {
        let p = if self.extending {
            self.progress(partial) - 1.0
        } else {
            1.0 - self.progress(partial)
        };
        let &(x, y, z) = DIRECTIONS.get(self.facing as usize).ok_or(super::tile_nbt181::TileError181::ArrayIndexOutOfBounds)?;
        Ok([p * x as f32, p * y as f32, p * z as f32])
    }
    pub fn from_nbt(n: &Compound) -> Self {
        let mut p = Self::new(0,0,0,false,false);
        p.read_from_nbt181(n).expect("TileEntityPiston.readFromNBT");
        p
    }
    pub fn read_from_nbt181(&mut self,n:&Compound) -> Result<(), super::tile_nbt181::TileError181> {
        use super::tile_nbt181 as tn;
        self.block_id=tn::int(n,"blockId")?;
        self.block_data=tn::int(n,"blockData")?;
        self.facing=tn::int(n,"facing")?;
        self.progress=tn::float(n,"progress")?;self.last_progress=self.progress;
        self.extending=tn::byte(n,"extending")?!=0;
        // render_head is constructor-only; loading into a retained object leaves it alone.
        Ok(())
    }
    pub fn to_nbt(&self, x: i32, y: i32, z: i32) -> Compound {
        let mut n = super::notes::tile_nbt("Piston", x, y, z);
        n.put("blockId", Tag::Int(self.block_id as i32));
        n.put("blockData", Tag::Int(self.block_data));
        n.put("facing", Tag::Int(self.facing));
        n.put("progress", Tag::Float(if self.last_progress.is_nan() {f32::NAN}else{self.last_progress}));
        n.put("extending", Tag::Byte(self.extending as i8));
        n
    }
}

/// Material.getMaterialMobility with the BlockBed/Door/Ice/Rail overrides.
pub fn mobility(id: u8) -> i32 {
    match id {
        26 | 64 | 71 | 70 | 72 => 1,
        27 | 28 | 66 | 79 => 0,
        _ => match material_of(id) {
            MAT_WATER | MAT_LAVA | MAT_LEAVES | MAT_PLANTS | MAT_FIRE | MAT_CIRCUITS
            | MAT_UNUSED | MAT_SNOW | MAT_CACTUS | MAT_PUMPKIN | MAT_CAKEMATERIAL
            | MAT_WEB | MAT_VINE | 29 => 1,
            MAT_PORTAL | MAT_PISTON => 2,
            _ => 0,
        },
    }
}
impl World {
    fn restore_piston_block181(&mut self,x:i32,y:i32,z:i32,id:i32,metadata:i32)->Result<(),super::tile_nbt181::TileError181> {
        if !super::in_xz_bounds(x,z) || !(0..128).contains(&y) {return Ok(());}
        if self.access_chunk(x>>4,z>>4) {
            if self.chunk_set_block_with_metadata_raw181(x>>4,z>>4,x&15,y,z&15,id,metadata)? {
                self.update_all_light181(x,y,z);
                self.notify_blocks_of_neighbor_change(x,y,z,id as u8);
            } else {self.update_all_light181(x,y,z);}
        }
        Ok(())
    }
    fn piston_can_push(&mut self, id: u8, x: i32, y: i32, z: i32, breaking: bool) -> bool {
        if id == OBSIDIAN {
            return false;
        }
        if matches!(id, PISTON_BASE | PISTON_STICKY_BASE) {
            if self.get_block_metadata(x, y, z) & 8 != 0 {
                return false;
            }
        } else if crate::entity::player::registry::hardness(id) == -1.0
            || mobility(id) == 2
            || (!breaking && mobility(id) == 1)
        {
            return false;
        }
        !props(id).container
    }
    fn piston_push_end(&mut self, x: i32, y: i32, z: i32, facing: i32) -> Option<(i32, i32, i32)> {
        let (dx, dy, dz) = DIRECTIONS[facing as usize];
        let (mut px, mut py, mut pz) = (x + dx, y + dy, z + dz);
        for count in 0..13 {
            if py <= 0 || py >= 127 {
                return None;
            }
            let id = self.get_block_id(px, py, pz);
            if id == 0 {
                return Some((px, py, pz));
            }
            if !self.piston_can_push(id, px, py, pz, true) {
                return None;
            }
            if mobility(id) == 1 {
                return Some((px, py, pz));
            }
            if count == 12 {
                return None;
            }
            px += dx;
            py += dy;
            pz += dz;
        }
        Some((px, py, pz))
    }
    pub(crate) fn piston_update(&mut self, _id: u8, x: i32, y: i32, z: i32) {
        // All three BlockPistonBase entry points suppress state updates remotely.
        if self.multiplayer_world || IGNORE_UPDATES.load(std::sync::atomic::Ordering::Relaxed) { return; }
        let meta = self.get_block_metadata(x, y, z);
        let facing = meta & 7;
        // The original tests power before checking metadata == 7.
        let powered = self.piston_powered(x, y, z, facing);
        if meta == 7 {
            return;
        }
        if powered && meta & 8 == 0 {
            if self.piston_push_end(x, y, z, facing).is_some() {
                self.set_block_metadata(x, y, z, facing | 8);
                self.play_note_at(x, y, z, 0, facing);
            }
        } else if !powered && meta & 8 != 0 {
            self.set_block_metadata(x, y, z, facing);
            self.play_note_at(x, y, z, 1, facing);
        }
    }
    fn piston_powered(&mut self, x: i32, y: i32, z: i32, facing: i32) -> bool {
        for side in [0, 1, 2, 3, 5, 4] {
            let (dx, dy, dz) = DIRECTIONS[side as usize];
            if side != facing && self.indirectly_provides_power(x + dx, y + dy, z + dz, side) {
                return true;
            }
        }
        for (dx, dy, dz, side) in [
            (0, 0, 0, 0),
            (0, 2, 0, 1),
            (0, 1, -1, 2),
            (0, 1, 1, 3),
            (-1, 1, 0, 4),
            (1, 1, 0, 5),
        ] {
            if self.indirectly_provides_power(x + dx, y + dy, z + dz, side) {
                return true;
            }
        }
        false
    }
    fn piston_extend(&mut self, id: u8, x: i32, y: i32, z: i32, facing: i32) -> bool {
        let Some((mut px, mut py, mut pz)) = self.piston_push_end(x, y, z, facing) else {
            return false;
        };
        let (dx, dy, dz) = DIRECTIONS[facing as usize];
        let end = self.get_block_id(px, py, pz);
        if end != 0 {
            let meta = self.get_block_metadata(px, py, pz);
            super::behavior::drop_block_as_item(self, end, px, py, pz, meta);
            self.set_block_with_notify(px, py, pz, 0);
        }
        while (px, py, pz) != (x, y, z) {
            let (sx, sy, sz) = (px - dx, py - dy, pz - dz);
            let stored = self.get_block_id(sx, sy, sz);
            let meta = self.get_block_metadata(sx, sy, sz);
            let (stored, meta) = if stored == id && (sx, sy, sz) == (x, y, z) {
                (
                    PISTON_EXTENSION,
                    facing | if id == PISTON_STICKY_BASE { 8 } else { 0 },
                )
            } else {
                (stored, meta)
            };
            self.set_block_and_metadata(px, py, pz, PISTON_MOVING, meta);
            self.set_block_tile_entity(
                px,
                py,
                pz,
                TileEntity::Piston(Piston::new(stored, meta, facing, true, false)),
            );
            (px, py, pz) = (sx, sy, sz);
        }
        true
    }
    pub(crate) fn piston_play(
        &mut self,
        id: u8,
        mut x: i32,
        mut y: i32,
        mut z: i32,
        event: i32,
        facing: i32,
    ) {
        IGNORE_UPDATES.store(true,std::sync::atomic::Ordering::Relaxed);
        let (dx, dy, dz) = DIRECTIONS[facing as usize];
        if event == 0 {
            if self.piston_extend(id, x, y, z, facing) {
                self.set_block_metadata_with_notify(x, y, z, facing | 8);
                let pitch = self.rand.next_float() * 0.25 + 0.6;
                self.block_events
                    .push(super::notes::BlockEvent::PistonSound {
                        position: (x, y, z),
                        extending: true,
                        pitch,
                    });
                self.play_sound("tile.piston.out", crate::entity::Vec3D::new(x as f64 + 0.5, y as f64 + 0.5, z as f64 + 0.5), 0.5, pitch);
            } else {self.set_block_metadata(x,y,z,facing);}
        } else if event == 1 {
            self.finish_piston_at(x + dx, y + dy, z + dz);
            self.set_block_and_metadata(x, y, z, PISTON_MOVING, facing);
            self.set_block_tile_entity(
                x,
                y,
                z,
                TileEntity::Piston(Piston::new(id, facing, facing, false, true)),
            );
            if id == PISTON_STICKY_BASE {
                let (px, py, pz) = (x + dx * 2, y + dy * 2, z + dz * 2);
                let mut stored = self.get_block_id(px, py, pz);
                let mut meta = self.get_block_metadata(px, py, pz);
                let mut completed = false;
                if stored == PISTON_MOVING {
                    if let Some(TileEntity::Piston(p)) =
                        self.get_block_tile_entity(px, py, pz).cloned()
                    {
                        if p.facing == facing && p.extending {
                            self.finish_piston_at(px, py, pz);
                            stored = p.block_id.try_into().expect("BlockPistonBase carried block ID");
                            meta = p.block_data;
                            completed = true;
                        }
                    }
                }
                if completed
                    || stored == 0
                    || !self.piston_can_push(stored, px, py, pz, false)
                    || (mobility(stored) != 0
                        && !matches!(stored, PISTON_BASE | PISTON_STICKY_BASE))
                {
                    if !completed {
                        IGNORE_UPDATES.store(false,std::sync::atomic::Ordering::Relaxed);
                        self.set_block_with_notify(x + dx, y + dy, z + dz, 0);
                        IGNORE_UPDATES.store(true,std::sync::atomic::Ordering::Relaxed);
                    }
                } else {
                    x += dx;
                    y += dy;
                    z += dz;
                    self.set_block_and_metadata(x, y, z, PISTON_MOVING, meta);
                    self.set_block_tile_entity(
                        x,
                        y,
                        z,
                        TileEntity::Piston(Piston::new(stored, meta, facing, false, false)),
                    );
                    IGNORE_UPDATES.store(false,std::sync::atomic::Ordering::Relaxed);
                    self.set_block_with_notify(px,py,pz,0);
                    IGNORE_UPDATES.store(true,std::sync::atomic::Ordering::Relaxed);
                }
            } else {
                IGNORE_UPDATES.store(false,std::sync::atomic::Ordering::Relaxed);
                self.set_block_with_notify(x + dx, y + dy, z + dz, 0);
                IGNORE_UPDATES.store(true,std::sync::atomic::Ordering::Relaxed);
            }
            let pitch = self.rand.next_float() * 0.15 + 0.6;
            self.block_events
                .push(super::notes::BlockEvent::PistonSound {
                    position: (x, y, z),
                    extending: false,
                    pitch,
                });
            self.play_sound("tile.piston.in", crate::entity::Vec3D::new(x as f64 + 0.5, y as f64 + 0.5, z as f64 + 0.5), 0.5, pitch);
        }
        IGNORE_UPDATES.store(false,std::sync::atomic::Ordering::Relaxed);
    }
    pub(crate) fn piston_head_removed(&mut self, x: i32, y: i32, z: i32) {
        let facing = self.get_block_metadata(x, y, z) & 7;
        let (dx, dy, dz) = DIRECTIONS[facing as usize];
        let (x, y, z) = (x - dx, y - dy, z - dz);
        let id = self.get_block_id(x, y, z);
        if matches!(id, PISTON_BASE | PISTON_STICKY_BASE) {
            let meta = self.get_block_metadata(x, y, z);
            if meta & 8 != 0 {
                super::behavior::drop_block_as_item(self, id, x, y, z, meta);
                self.set_block_with_notify(x, y, z, 0);
            }
        }
    }
    pub(crate) fn piston_head_neighbor(&mut self, x: i32, y: i32, z: i32, changed: u8) {
        let facing = self.get_block_metadata(x, y, z) & 7;
        let (dx, dy, dz) = DIRECTIONS[facing as usize];
        let id = self.get_block_id(x - dx, y - dy, z - dz);
        if !matches!(id, PISTON_BASE | PISTON_STICKY_BASE) {
            self.set_block_with_notify(x, y, z, 0);
        } else {
            super::behavior::on_neighbor_block_change(self, id, x - dx, y - dy, z - dz, changed);
        }
    }
    pub fn finish_piston_at(&mut self, x: i32, y: i32, z: i32) {
        let Some(TileEntity::Piston(_)) = self.get_block_tile_entity(x, y, z) else {
            return;
        };
        let id = self.chunk(x >> 4, z >> 4).tile_entity_ids[&(x & 15, y, z & 15)];
        self.clear_piston_tile181(id).expect("TileEntityPiston.clearPistonTileEntity");
    }
    pub fn clear_piston_tile181(&mut self,id:u64)->Result<(),super::tile_nbt181::TileError181> {
        let Some(TileEntity::Piston(mut p))=self.tile_object(id).cloned() else{panic!("not a piston");};
        let (x,y,z)=self.tile_objects[&id].position;
        if p.last_progress < 1.0 {
            p.progress = 1.0;
            p.last_progress = 1.0;
            self.update_tile_object(id, TileEntity::Piston(p.clone()));
            self.remove_block_tile_entity(x, y, z);
            self.tile_objects.get_mut(&id).unwrap().invalid = true;
            if self.get_block_id(x, y, z) == PISTON_MOVING {
                self.restore_piston_block181(x, y, z, p.block_id, p.block_data)?;
            }
        }
        Ok(())
    }
    pub fn moving_piston_box(
        &mut self,
        x: i32,
        y: i32,
        z: i32,
        stored: impl Into<i32>,
        progress: f32,
        facing: i32,
    ) -> Option<BB> {
        self.moving_piston_box181(x,y,z,stored.into(),progress,facing).expect("BlockPistonMoving collision")
    }
    pub fn moving_piston_box181(&mut self,x:i32,y:i32,z:i32,stored:i32,progress:f32,facing:i32)->Result<Option<BB>,super::tile_nbt181::TileError181> {
        use super::tile_nbt181::TileError181 as E;
        if stored == 0 || stored == PISTON_MOVING as i32 {
            return Ok(None);
        }
        let block:u8=stored.try_into().map_err(|_|E::ArrayIndexOutOfBounds)?;
        if !props(block).exists{return Err(E::NullPointer);}
        let Some(mut b) = self.block_collision_box(block, x, y, z) else {return Ok(None);};
        let &(dx,dy,dz)=DIRECTIONS.get(facing as usize).ok_or(E::ArrayIndexOutOfBounds)?;
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
        Ok(Some(b))
    }
    fn piston_push_entities(
        &mut self,
        x: i32,
        y: i32,
        z: i32,
        p: &Piston,
        mut progress: f32,
        distance: f32,
    ) -> Result<(),super::tile_nbt181::TileError181> {
        if !p.extending {
            progress -= 1.0;
        } else {
            progress = 1.0 - progress;
        }
        if let Some(b) = self.moving_piston_box181(x, y, z, p.block_id, progress, p.facing)? {
            let entities = self.entities_within_aabb(None, b);
            let (dx, dy, dz) = DIRECTIONS[p.facing as usize];
            for id in entities {
                if let Some(mut e) = self.entities.remove(&id) {
                    e.move_entity(
                        self,
                        (distance * dx as f32) as f64,
                        (distance * dy as f32) as f64,
                        (distance * dz as f32) as f64,
                    );
                    self.entities.insert(id, e);
                }
            }
        }
        Ok(())
    }
    pub(crate) fn tick_piston(&mut self, id: u64, mut p: Piston) {
        self.tick_piston181(id,&mut p).expect("TileEntityPiston.updateEntity");
    }
    pub fn tick_piston181(&mut self, id: u64, p: &mut Piston) -> Result<(),super::tile_nbt181::TileError181> {
        let (x, y, z) = self.tile_objects[&id].position;
        p.last_progress = p.progress;
        if p.last_progress >= 1.0 {
            self.update_tile_object(id, TileEntity::Piston(p.clone()));
            self.piston_push_entities(x, y, z, p, 1.0, 0.25)?;
            self.remove_block_tile_entity(x, y, z);
            self.tile_objects.get_mut(&id).unwrap().invalid = true;
            if self.get_block_id(x, y, z) == PISTON_MOVING {
                self.restore_piston_block181(x, y, z, p.block_id, p.block_data)?;
            }
        } else {
            p.progress += 0.5;
            if p.progress >= 1.0 { p.progress = 1.0; }
            self.update_tile_object(id, TileEntity::Piston(p.clone()));
            if p.extending {
                self.piston_push_entities(
                    x,
                    y,
                    z,
                    p,
                    p.progress,
                    p.progress - p.last_progress + 0.0625,
                )?;
            }
        }
        Ok(())
    }
}
