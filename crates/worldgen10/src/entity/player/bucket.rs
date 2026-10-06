//! ItemBucket: literal right-click ray and result identity, before controller restore.
use super::*;
use crate::{block::*, entity::MovingObjectPosition};
impl Entity {
    pub fn bucket_ray181(&self) -> (Vec3D, Vec3D) {
        let pitch = self.prev_pitch + (self.pitch - self.prev_pitch) * 1.0f32;
        let yaw = self.prev_yaw + (self.yaw - self.prev_yaw) * 1.0f32;
        let start = Vec3D::new(
            self.prev_pos.x + (self.pos.x - self.prev_pos.x) * 1.0,
            self.prev_pos.y + (self.pos.y - self.prev_pos.y) * 1.0 + 1.62 - self.y_offset as f64,
            self.prev_pos.z + (self.pos.z - self.prev_pos.z) * 1.0,
        );
        let cy = math::cos(-yaw * (math::PI_F32 / 180.0) - math::PI_F32);
        let sy = math::sin(-yaw * (math::PI_F32 / 180.0) - math::PI_F32);
        let cp = -math::cos(-pitch * (math::PI_F32 / 180.0));
        let sp = math::sin(-pitch * (math::PI_F32 / 180.0));
        (
            start,
            start.add(
                (sy * cp) as f64 * 5.0,
                sp as f64 * 5.0,
                (cy * cp) as f64 * 5.0,
            ),
        )
    }
    pub fn bucket_right_click181(&self, w: &mut World, stack: ItemStack, full: i32) -> ItemStack {
        let (start, end) = self.bucket_ray181();
        let hit = w.ray_trace_blocks_options(start, end, full == 0, false);
        self.bucket_hit181(w, stack, full, start, hit)
    }
    /// Original callback after World.rayTraceBlocks_do. World.canMineBlock and
    /// EntityPlayer.canEdit are true in the singleplayer base classes.
    pub fn bucket_hit181(
        &self,
        w: &mut World,
        stack: ItemStack,
        full: i32,
        start: Vec3D,
        hit: Option<MovingObjectPosition>,
    ) -> ItemStack {
        self.bucket_hit_with_writes181(
            w,
            stack,
            full,
            start,
            hit,
            |w, x, y, z| {
                w.set_block_with_notify(x, y, z, 0);
            },
            |w, x, y, z, full| {
                w.set_block_and_metadata_with_notify(x, y, z, full as u8, 0);
            },
        )
    }
    /// The World callbacks are supplied separately so original-bytecode probes can
    /// record requested writes without exercising another owner's liquid callbacks.
    pub fn bucket_hit_with_writes181(
        &self,
        w: &mut World,
        stack: ItemStack,
        full: i32,
        start: Vec3D,
        hit: Option<MovingObjectPosition>,
        mut remove: impl FnMut(&mut World, i32, i32, i32),
        mut place: impl FnMut(&mut World, i32, i32, i32, i32),
    ) -> ItemStack {
        match hit {
            Some(MovingObjectPosition::Block { x, y, z, side, .. }) => {
                if full == 0 {
                    if !self.can_edit_block181(x, y, z) {
                        return stack;
                    }
                    let material = w.get_block_material(x, y, z);
                    if matches!(material, MAT_WATER | MAT_LAVA)
                        && w.get_block_metadata(x, y, z) == 0
                    {
                        remove(w, x, y, z);
                        return ItemStack::new(if material == MAT_WATER { 326 } else { 327 }, 1, 0);
                    }
                } else {
                    if full < 0 {
                        return ItemStack::new(325, 1, 0);
                    }
                    let (x, y, z) = super::placement::offset(x, y, z, side);
                    if !self.can_edit_block181(x, y, z) {
                        return stack;
                    }
                    if w.get_block_id(x, y, z) == 0 || !mat(w.get_block_material(x, y, z)).is_solid
                    {
                        if w.dimension.is_hell_world() && full == 8 {
                            let pitch = 2.6 + (w.rand.next_float() - w.rand.next_float()) * 0.8;
                            w.play_sound("random.fizz", start.add(0.5, 0.5, 0.5), 0.5, pitch);
                            for _ in 0..8 {
                                let p = Vec3D::new(
                                    x as f64 + w.math_random.next_double(),
                                    y as f64 + w.math_random.next_double(),
                                    z as f64 + w.math_random.next_double(),
                                );
                                w.spawn_particle("largesmoke", p, Vec3D::default());
                            }
                        } else {
                            place(w, x, y, z, full);
                        }
                        if self.player_state().unwrap().capabilities.creative {
                            return stack;
                        }
                        return ItemStack::new(325, 1, 0);
                    }
                }
            }
            Some(MovingObjectPosition::Entity { id, .. })
                if full == 0
                    && w.entities
                        .get(&id)
                        .is_some_and(|e| matches!(e.kind, EntityKind::Cow)) =>
            {
                return ItemStack::new(335, 1, 0)
            }
            _ => {}
        }
        stack
    }
}
