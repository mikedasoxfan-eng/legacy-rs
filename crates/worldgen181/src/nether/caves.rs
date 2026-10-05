//! `MapGenBase` + `MapGenCavesHell`: carves tunnels into a raw chunk using the
//! cave starts of every chunk within 8 chunks.

use crate::block;
use crate::math::{self, PI_F32};
use crate::random::JavaRandom;

pub struct CaveCarver {
    range: i32,
    rand: JavaRandom,
}

impl Default for CaveCarver {
    fn default() -> Self {
        CaveCarver {
            range: 8,
            rand: JavaRandom::new(0),
        }
    }
}

impl CaveCarver {
    /// `func_867_a`
    pub fn carve(&mut self, world_seed: i64, chunk_x: i32, chunk_z: i32, blocks: &mut [u8]) {
        let r = self.range;
        self.rand.set_seed(world_seed);
        let mx = self.rand.next_long();
        let mz = self.rand.next_long();
        for x in chunk_x - r..=chunk_x + r {
            for z in chunk_z - r..=chunk_z + r {
                self.rand.set_seed(
                    (x as i64).wrapping_mul(mx) ^ (z as i64).wrapping_mul(mz) ^ world_seed,
                );
                self.starts(x, z, chunk_x, chunk_z, blocks);
            }
        }
    }

    /// `func_868_a`
    fn starts(&mut self, x: i32, z: i32, chunk_x: i32, chunk_z: i32, blocks: &mut [u8]) {
        let a = self.rand.next_int(10) + 1;
        let b = self.rand.next_int(a) + 1;
        let mut count = self.rand.next_int(b);
        if self.rand.next_int(5) != 0 {
            count = 0;
        }
        for _ in 0..count {
            let px = (x * 16 + self.rand.next_int(16)) as f64;
            let py = self.rand.next_int(128) as f64;
            let pz = (z * 16 + self.rand.next_int(16)) as f64;
            let mut tunnels = 1;
            if self.rand.next_int(4) == 0 {
                self.room(chunk_x, chunk_z, blocks, px, py, pz);
                tunnels += self.rand.next_int(4);
            }
            for _ in 0..tunnels {
                let yaw = self.rand.next_float() * PI_F32 * 2.0;
                let pitch = (self.rand.next_float() - 0.5) * 2.0 / 8.0;
                let width = self.rand.next_float() * 2.0 + self.rand.next_float();
                self.tunnel(
                    chunk_x,
                    chunk_z,
                    blocks,
                    px,
                    py,
                    pz,
                    width * 2.0,
                    yaw,
                    pitch,
                    0,
                    0,
                    0.5,
                );
            }
        }
    }

    /// `func_4129_a`
    fn room(&mut self, chunk_x: i32, chunk_z: i32, blocks: &mut [u8], x: f64, y: f64, z: f64) {
        let width = 1.0 + self.rand.next_float() * 6.0;
        self.tunnel(
            chunk_x, chunk_z, blocks, x, y, z, width, 0.0, 0.0, -1, -1, 0.5,
        );
    }

    /// `func_4128_a`: walk one tunnel, recursing once to branch.
    #[allow(clippy::too_many_arguments)]
    fn tunnel(
        &mut self,
        chunk_x: i32,
        chunk_z: i32,
        blocks: &mut [u8],
        mut x: f64,
        mut y: f64,
        mut z: f64,
        width: f32,
        mut yaw: f32,
        mut pitch: f32,
        mut step: i32,
        mut length: i32,
        y_scale: f64,
    ) {
        let center_x = (chunk_x * 16 + 8) as f64;
        let center_z = (chunk_z * 16 + 8) as f64;
        let mut yaw_delta = 0.0f32;
        let mut pitch_delta = 0.0f32;
        let mut rand = JavaRandom::new(self.rand.next_long());
        if length <= 0 {
            let max = self.range * 16 - 16;
            length = max - rand.next_int(max / 4);
        }
        let mut is_room = false;
        if step == -1 {
            step = length / 2;
            is_room = true;
        }
        let branch_at = rand.next_int(length / 2) + length / 4;
        let steep = rand.next_int(6) == 0;
        while step < length {
            let radius =
                1.5 + (math::sin(step as f32 * PI_F32 / length as f32) * width * 1.0) as f64;
            let radius_y = radius * y_scale;
            let cp = math::cos(pitch);
            let sp = math::sin(pitch);
            x += (math::cos(yaw) * cp) as f64;
            y += sp as f64;
            z += (math::sin(yaw) * cp) as f64;
            if steep {
                pitch *= 0.92;
            } else {
                pitch *= 0.7;
            }
            pitch += pitch_delta * 0.1;
            yaw += yaw_delta * 0.1;
            pitch_delta *= 0.9;
            yaw_delta *= 0.75;
            pitch_delta += (rand.next_float() - rand.next_float()) * rand.next_float() * 2.0;
            yaw_delta += (rand.next_float() - rand.next_float()) * rand.next_float() * 4.0;

            if !is_room && step == branch_at && width > 1.0 {
                let w1 = rand.next_float() * 0.5 + 0.5;
                self.tunnel(
                    chunk_x,
                    chunk_z,
                    blocks,
                    x,
                    y,
                    z,
                    w1,
                    yaw - PI_F32 * 0.5,
                    pitch / 3.0,
                    step,
                    length,
                    1.0,
                );
                let w2 = rand.next_float() * 0.5 + 0.5;
                self.tunnel(
                    chunk_x,
                    chunk_z,
                    blocks,
                    x,
                    y,
                    z,
                    w2,
                    yaw + PI_F32 * 0.5,
                    pitch / 3.0,
                    step,
                    length,
                    1.0,
                );
                return;
            }

            if is_room || rand.next_int(4) != 0 {
                let dx = x - center_x;
                let dz = z - center_z;
                let remaining = (length - step) as f64;
                let reach = (width + 2.0 + 16.0) as f64;
                if dx * dx + dz * dz - remaining * remaining > reach * reach {
                    return;
                }

                if x >= center_x - 16.0 - radius * 2.0
                    && z >= center_z - 16.0 - radius * 2.0
                    && x <= center_x + 16.0 + radius * 2.0
                    && z <= center_z + 16.0 + radius * 2.0
                {
                    let mut x0 = math::floor_double(x - radius) - chunk_x * 16 - 1;
                    let mut x1 = math::floor_double(x + radius) - chunk_x * 16 + 1;
                    let mut y0 = math::floor_double(y - radius_y) - 1;
                    let mut y1 = math::floor_double(y + radius_y) + 1;
                    let mut z0 = math::floor_double(z - radius) - chunk_z * 16 - 1;
                    let mut z1 = math::floor_double(z + radius) - chunk_z * 16 + 1;
                    if x0 < 0 {
                        x0 = 0;
                    }
                    if x1 > 16 {
                        x1 = 16;
                    }
                    if y0 < 1 {
                        y0 = 1;
                    }
                    if y1 > 120 {
                        y1 = 120;
                    }
                    if z0 < 0 {
                        z0 = 0;
                    }
                    if z1 > 16 {
                        z1 = 16;
                    }

                    // Abort this step if the carve volume's shell touches lava.
                    let mut lava = false;
                    let mut bx = x0;
                    while !lava && bx < x1 {
                        let mut bz = z0;
                        while !lava && bz < z1 {
                            let mut by = y1 + 1;
                            while !lava && by >= y0 - 1 {
                                let i = ((bx * 16 + bz) * 128 + by) as usize;
                                if (0..128).contains(&by) {
                                    if blocks[i] == block::LAVA_MOVING
                                        || blocks[i] == block::LAVA_STILL
                                    {
                                        lava = true;
                                    }
                                    if by != y0 - 1
                                        && bx != x0
                                        && bx != x1 - 1
                                        && bz != z0
                                        && bz != z1 - 1
                                    {
                                        by = y0;
                                    }
                                }
                                by -= 1;
                            }
                            bz += 1;
                        }
                        bx += 1;
                    }

                    if !lava {
                        for bx in x0..x1 {
                            let nx = ((bx + chunk_x * 16) as f64 + 0.5 - x) / radius;
                            for bz in z0..z1 {
                                let nz = ((bz + chunk_z * 16) as f64 + 0.5 - z) / radius;
                                let mut i = ((bx * 16 + bz) * 128 + y1) as usize;
                                let mut by = y1 - 1;
                                while by >= y0 {
                                    let ny = (by as f64 + 0.5 - y) / radius_y;
                                    if ny > -0.7 && nx * nx + ny * ny + nz * nz < 1.0 {
                                        let b = blocks[i];
                                        if b == block::NETHERRACK
                                            || b == block::DIRT
                                            || b == block::GRASS
                                        {
                                            blocks[i] = block::AIR;
                                        }
                                    }
                                    i -= 1;
                                    by -= 1;
                                }
                            }
                        }
                        if is_room {
                            break;
                        }
                    }
                }
            }
            step += 1;
        }
    }
}
