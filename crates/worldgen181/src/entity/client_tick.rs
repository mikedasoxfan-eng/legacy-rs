use super::Vec3D;
use crate::{math::floor_double as floor, World};
impl World {
    /// BlockRedstoneOre's shared click, walk and activation callback.
    pub fn activate_redstone_ore181(&mut self, id: u8, x: i32, y: i32, z: i32) {
        self.redstone_display(x, y, z);
        if id == crate::block::ORE_REDSTONE { self.set_block_with_notify(x, y, z, 74); }
    }
    /// The singleplayer client tick, for a real player or the legacy camera stand-in.
    pub fn client_tick(&mut self, player: usize, client_ticks: i32) {
        if let Some(&(x, y, z)) = self.players.get(player) {
            let pos = Vec3D::new(x, y, z);
            if client_ticks % 30 == 0 {
                self.join_entity_in_surroundings(pos);
            }
            self.update_entities();
            self.tick();
            let (x, y, z) = self.players[player];
            self.random_display_updates(floor(x), floor(y), floor(z));
        }
    }
    pub fn random_display_updates(&mut self, x: i32, y: i32, z: i32) {
        let mut display_rand = crate::JavaRandom::new(self.display_seed_source.next_seed());
        for _ in 0..1000 {
            let bx = x + self.rand.next_int(16) - self.rand.next_int(16);
            let by = y + self.rand.next_int(16) - self.rand.next_int(16);
            let bz = z + self.rand.next_int(16) - self.rand.next_int(16);
            let id = self.get_block_id(bx, by, bz);
            // b1.8.1: dark specks in the air near the bottom of the world.
            if self.rand.next_int(8) > by && id == 0 {
                let p = Vec3D::new(
                    (bx as f32 + self.rand.next_float()) as f64,
                    (by as f32 + self.rand.next_float()) as f64,
                    (bz as f32 + self.rand.next_float()) as f64,
                );
                self.spawn_particle("depthsuspend", p, Vec3D::default());
            }
            if id > 0 {
                self.block_display_tick(id, bx, by, bz, &mut display_rand);
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
            let mut p = Vec3D::new(
                (x as f32 + self.rand.next_float()) as f64,
                (y as f32 + self.rand.next_float()) as f64,
                (z as f32 + self.rand.next_float()) as f64,
            );
            if !crate::block::opaque_cube_lookup(self.get_block_id(x + dx, y + dy, z + dz)) {
                if dx != 0 {
                    p.x = (x + if dx > 0 { 1 } else { 0 }) as f64 + dx as f64 / 16.0;
                }
                if dy != 0 {
                    p.y = (y + if dy > 0 { 1 } else { 0 }) as f64 + dy as f64 / 16.0;
                }
                if dz != 0 {
                    p.z = (z + if dz > 0 { 1 } else { 0 }) as f64 + dz as f64 / 16.0;
                }
            }
            // Vanilla compares Y to zero, rather than to the block's Y.
            if p.x < x as f64
                || p.x > (x + 1) as f64
                || p.y < 0.0
                || p.y > (y + 1) as f64
                || p.z < z as f64
                || p.z > (z + 1) as f64
            {
                self.spawn_particle("reddust", p, Vec3D::new(0.0, 0.0, 0.0));
            }
        }
    }
}

impl World {
    /// Literal Block.randomDisplayTick overrides. The supplied Random belongs to this call.
    pub fn block_display_tick(
        &mut self,
        id: u8,
        x: i32,
        y: i32,
        z: i32,
        r: &mut crate::JavaRandom,
    ) {
        let zero = Vec3D::default();
        match id {
            50 | 76 => {
                let m = self.get_block_metadata(x, y, z);
                let mut p = Vec3D::new(
                    (x as f32 + 0.5) as f64,
                    (y as f32 + 0.7) as f64,
                    (z as f32 + 0.5) as f64,
                );
                if id == 76 {
                    p.x += (r.next_float() - 0.5) as f64 * 0.2;
                    p.y += (r.next_float() - 0.5) as f64 * 0.2;
                    p.z += (r.next_float() - 0.5) as f64 * 0.2;
                }
                match m {
                    1 => {
                        p.x -= 0.27f32 as f64;
                        p.y += 0.22f32 as f64
                    }
                    2 => {
                        p.x += 0.27f32 as f64;
                        p.y += 0.22f32 as f64
                    }
                    3 => {
                        p.z -= 0.27f32 as f64;
                        p.y += 0.22f32 as f64
                    }
                    4 => {
                        p.z += 0.27f32 as f64;
                        p.y += 0.22f32 as f64
                    }
                    _ => {}
                }
                if id == 50 {
                    self.spawn_particle("smoke", p, zero);
                    self.spawn_particle("flame", p, zero);
                } else {
                    self.spawn_particle("reddust", p, zero);
                }
            }
            74 => self.redstone_display(x, y, z),
            8 | 9 => {
                if r.next_int(10)==0 {let m=self.get_block_metadata(x,y,z);if m<=0||m>=8 {let p=Vec3D::new((x as f32+r.next_float()) as f64,(y as f32+r.next_float()) as f64,(z as f32+r.next_float()) as f64);self.spawn_particle("suspended",p,zero);}}
                if r.next_int(64) == 0 {
                    let m = self.get_block_metadata(x, y, z);
                    if m > 0 && m < 8 {
                        let v = r.next_float() * 0.25 + 12.0 / 16.0;
                        let p = r.next_float() * 1.0 + 0.5;
                        self.play_sound(
                            "liquid.water",
                            Vec3D::new(
                                (x as f32 + 0.5) as f64,
                                (y as f32 + 0.5) as f64,
                                (z as f32 + 0.5) as f64,
                            ),
                            v,
                            p,
                        );
                    }
                }
            }
            10 | 11 => {
                if self.get_block_material(x, y + 1, z) == crate::block::MAT_AIR
                    && !crate::block::opaque_cube_lookup(self.get_block_id(x, y + 1, z))
                    && r.next_int(100) == 0
                {
                    let p = Vec3D::new(
                        (x as f32 + r.next_float()) as f64,
                        y as f64 + self.block_bounds[id as usize].max_y,
                        (z as f32 + r.next_float()) as f64,
                    );
                    self.spawn_particle("lava", p, zero);
                }
            }
            90 => {
                if r.next_int(100) == 0 {
                    let pitch = r.next_float() * 0.4 + 0.8;
                    self.play_sound(
                        "portal.portal",
                        Vec3D::new(x as f64 + 0.5, y as f64 + 0.5, z as f64 + 0.5),
                        1.0,
                        pitch,
                    );
                }
                for _ in 0..4 {
                    let mut p = Vec3D::new(
                        (x as f32 + r.next_float()) as f64,
                        (y as f32 + r.next_float()) as f64,
                        (z as f32 + r.next_float()) as f64,
                    );
                    let sign = r.next_int(2) * 2 - 1;
                    let mut v = Vec3D::new(
                        (r.next_float() as f64 - 0.5) * 0.5,
                        (r.next_float() as f64 - 0.5) * 0.5,
                        (r.next_float() as f64 - 0.5) * 0.5,
                    );
                    if self.get_block_id(x - 1, y, z) != id && self.get_block_id(x + 1, y, z) != id
                    {
                        p.x = x as f64 + 0.5 + 0.25 * sign as f64;
                        v.x = (r.next_float() * 2.0 * sign as f32) as f64;
                    } else {
                        p.z = z as f64 + 0.5 + 0.25 * sign as f64;
                        v.z = (r.next_float() * 2.0 * sign as f32) as f64;
                    }
                    self.spawn_particle("portal", p, v);
                }
            }
            55 => {
                let m = self.get_block_metadata(x, y, z);
                if m > 0 {
                    let px = x as f64 + 0.5 + (r.next_float() as f64 - 0.5) * 0.2;
                    let py = (y as f32 + 1.0 / 16.0) as f64;
                    let pz = z as f64 + 0.5 + (r.next_float() as f64 - 0.5) * 0.2;
                    let f = m as f32 / 15.0;
                    self.spawn_particle(
                        "reddust",
                        Vec3D::new(px, py, pz),
                        Vec3D::new(
                            (f * 0.6 + 0.4) as f64,
                            (f * f * 0.7 - 0.5).max(0.0) as f64,
                            (f * f * 0.6 - 0.7).max(0.0) as f64,
                        ),
                    );
                }
            }
            94 => {
                let m = self.get_block_metadata(x, y, z);
                let mut p = Vec3D::new(
                    (x as f32 + 0.5) as f64 + (r.next_float() - 0.5) as f64 * 0.2,
                    (y as f32 + 0.4) as f64 + (r.next_float() - 0.5) as f64 * 0.2,
                    (z as f32 + 0.5) as f64 + (r.next_float() - 0.5) as f64 * 0.2,
                );
                let distance = if r.next_int(2) == 0 {
                    -0.3125
                } else {
                    [-0.0625, 0.0625, 0.1875, 0.3125][((m & 12) >> 2) as usize]
                };
                match m & 3 {
                    0 => p.z += distance,
                    1 => p.x -= distance,
                    2 => p.z -= distance,
                    _ => p.x += distance,
                };
                self.spawn_particle("reddust", p, zero);
            }
            62 => {
                let m = self.get_block_metadata(x, y, z);
                let px = x as f32 + 0.5;
                let py = y as f32 + 0.0 + r.next_float() * 6.0 / 16.0;
                let pz = z as f32 + 0.5;
                let offset = r.next_float() * 0.6 - 0.3;
                let p = match m {
                    4 => Some(Vec3D::new(
                        (px - 0.52) as f64,
                        py as f64,
                        (pz + offset) as f64,
                    )),
                    5 => Some(Vec3D::new(
                        (px + 0.52) as f64,
                        py as f64,
                        (pz + offset) as f64,
                    )),
                    2 => Some(Vec3D::new(
                        (px + offset) as f64,
                        py as f64,
                        (pz - 0.52) as f64,
                    )),
                    3 => Some(Vec3D::new(
                        (px + offset) as f64,
                        py as f64,
                        (pz + 0.52) as f64,
                    )),
                    _ => None,
                };
                if let Some(p) = p {
                    self.spawn_particle("smoke", p, zero);
                    self.spawn_particle("flame", p, zero);
                }
            }
            51 => {
                if r.next_int(24) == 0 {
                    let volume = 1.0 + r.next_float();
                    let pitch = r.next_float() * 0.7 + 0.3;
                    self.play_sound(
                        "fire.fire",
                        Vec3D::new(
                            (x as f32 + 0.5) as f64,
                            (y as f32 + 0.5) as f64,
                            (z as f32 + 0.5) as f64,
                        ),
                        volume,
                        pitch,
                    );
                }
                if !self.is_block_normal_cube(x, y - 1, z)
                    && !crate::world::ticks_blocks::can_catch_fire(self, x, y - 1, z)
                {
                    for (dx, dy, dz) in [(-1, 0, 0), (1, 0, 0), (0, 0, -1), (0, 0, 1), (0, 1, 0)] {
                        if crate::world::ticks_blocks::can_catch_fire(self, x + dx, y + dy, z + dz)
                        {
                            for _ in 0..2 {
                                let rx = r.next_float();
                                let ry = r.next_float();
                                let rz = r.next_float();
                                let px = if dx < 0 {
                                    x as f32 + rx * 0.1
                                } else if dx > 0 {
                                    (x + 1) as f32 - rx * 0.1
                                } else {
                                    x as f32 + rx
                                };
                                let py = if dy > 0 {
                                    (y + 1) as f32 - ry * 0.1
                                } else {
                                    y as f32 + ry
                                };
                                let pz = if dz < 0 {
                                    z as f32 + rz * 0.1
                                } else if dz > 0 {
                                    (z + 1) as f32 - rz * 0.1
                                } else {
                                    z as f32 + rz
                                };
                                self.spawn_particle(
                                    "largesmoke",
                                    Vec3D::new(px as f64, py as f64, pz as f64),
                                    zero,
                                );
                            }
                        }
                    }
                } else {
                    for _ in 0..3 {
                        let p = Vec3D::new(
                            (x as f32 + r.next_float()) as f64,
                            (y as f32 + r.next_float() * 0.5 + 0.5) as f64,
                            (z as f32 + r.next_float()) as f64,
                        );
                        self.spawn_particle("largesmoke", p, zero);
                    }
                }
            }
            _ => {}
        }
    }
}
