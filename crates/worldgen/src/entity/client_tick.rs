use super::Vec3D;
use crate::{math::floor_double as floor, World};
impl World {
    /// The singleplayer client tick, with its non-ticking player/camera stand-in.
    pub fn client_tick(&mut self, player: usize, client_ticks: i32) {
        if let Some(&(x, y, z)) = self.players.get(player) {
            let pos = Vec3D::new(x, y, z);
            if client_ticks % 30 == 0 {
                self.join_entity_in_surroundings(pos);
            }
            self.update_entities();
            self.tick();
            let (x,y,z)=self.players[player];
            self.random_display_updates(floor(x), floor(y), floor(z));
        }
    }
    pub fn random_display_updates(&mut self, x: i32, y: i32, z: i32) {
        for _ in 0..1000 {
            let bx = x + self.rand.next_int(16) - self.rand.next_int(16);
            let by = y + self.rand.next_int(16) - self.rand.next_int(16);
            let bz = z + self.rand.next_int(16) - self.rand.next_int(16);
            let id = self.get_block_id(bx, by, bz);
            // Every display override uses the call-local time-seeded Random,
            // except glowing ore, whose particle method uses World.rand.
            if id == 74 {
                self.redstone_display(bx, by, bz);
            }
        }
    }
    pub(super) fn redstone_display(&mut self, x: i32, y: i32, z: i32) {
        for (dx, dy, dz) in [
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
            (1, 0, 0),
            (-1, 0, 0),
        ] {
            for _ in 0..3 {
                self.rand.next_float();
            }
            self.get_block_id(x + dx, y + dy, z + dz);
        }
    }
}
