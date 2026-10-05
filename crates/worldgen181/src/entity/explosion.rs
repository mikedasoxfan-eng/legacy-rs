use super::{AxisAlignedBB as BB, Entity, Vec3D};
use crate::{
    block::*,
    math::floor_double as floor,
    world::{behavior, java_hash::JavaHashSet},
    World,
};
#[derive(Clone, Debug)]
pub struct Explosion {
    pub position: Vec3D,
    pub size: f32,
    pub flaming: bool,
    pub destroyed_blocks: Vec<(i32, i32, i32)>,
}
impl World {
    pub fn block_density(&mut self, origin: Vec3D, b: BB) -> f32 {
        let sx = 1.0 / ((b.max_x - b.min_x) * 2.0 + 1.0);
        let sy = 1.0 / ((b.max_y - b.min_y) * 2.0 + 1.0);
        let sz = 1.0 / ((b.max_z - b.min_z) * 2.0 + 1.0);
        let (mut hit, mut total) = (0, 0);
        let mut x = 0.0f32;
        while x <= 1.0 {
            let mut y = 0.0f32;
            while y <= 1.0 {
                let mut z = 0.0f32;
                while z <= 1.0 {
                    let p = Vec3D::new(
                        b.min_x + (b.max_x - b.min_x) * x as f64,
                        b.min_y + (b.max_y - b.min_y) * y as f64,
                        b.min_z + (b.max_z - b.min_z) * z as f64,
                    );
                    if self.ray_trace_blocks(p, origin).is_none() {
                        hit += 1;
                    }
                    total += 1;
                    z = (z as f64 + sz) as f32;
                }
                y = (y as f64 + sy) as f32;
            }
            x = (x as f64 + sx) as f32;
        }
        hit as f32 / total as f32
    }
    pub fn create_explosion(&mut self, exploder: Option<i32>, pos: Vec3D, size: f32) -> Explosion {
        self.new_explosion(exploder, pos, size, false)
    }
    pub fn new_explosion(
        &mut self,
        exploder: Option<i32>,
        pos: Vec3D,
        size: f32,
        flaming: bool,
    ) -> Explosion {
        let mut rng = crate::JavaRandom::new(self.explosion_seed_source.next_seed());
        let mut blocks = JavaHashSet::default();
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    if x != 0 && x != 15 && y != 0 && y != 15 && z != 0 && z != 15 {
                        continue;
                    }
                    let mut dx = (x as f32 / 15.0 * 2.0 - 1.0) as f64;
                    let mut dy = (y as f32 / 15.0 * 2.0 - 1.0) as f64;
                    let mut dz = (z as f32 / 15.0 * 2.0 - 1.0) as f64;
                    let length = (dx * dx + dy * dy + dz * dz).sqrt();
                    dx /= length;
                    dy /= length;
                    dz /= length;
                    let mut strength = size * (0.7 + self.rand.next_float() * 0.6);
                    let mut p = pos;
                    let step = 0.3f32;
                    while strength > 0.0 {
                        let cell = (floor(p.x), floor(p.y), floor(p.z));
                        let id = self.get_block_id(cell.0, cell.1, cell.2);
                        if id > 0 {
                            strength -= (super::block_table::RESISTANCE[id as usize] + 0.3) * step;
                        }
                        if strength > 0.0 {
                            let h = cell
                                .0
                                .wrapping_mul(8976890)
                                .wrapping_add(cell.1.wrapping_mul(981131))
                                .wrapping_add(cell.2);
                            blocks.insert(h, cell);
                        }
                        p.x += dx * step as f64;
                        p.y += dy * step as f64;
                        p.z += dz * step as f64;
                        strength -= step * 0.75;
                    }
                }
            }
        }
        let radius = size * 2.0;
        let b = BB::new(
            floor(pos.x - radius as f64 - 1.0) as f64,
            floor(pos.y - radius as f64 - 1.0) as f64,
            floor(pos.z - radius as f64 - 1.0) as f64,
            floor(pos.x + radius as f64 + 1.0) as f64,
            floor(pos.y + radius as f64 + 1.0) as f64,
            floor(pos.z + radius as f64 + 1.0) as f64,
        );
        for id in self.entities_within_aabb(exploder, b) {
            if let Some(mut e) = self.entities.remove(&id) {
                let distance = e.pos.distance(pos) / radius as f64;
                if distance <= 1.0 {
                    let mut d = e.pos.add(-pos.x, -pos.y, -pos.z);
                    let length = d.length();
                    d.x /= length;
                    d.y /= length;
                    d.z /= length;
                    let density = self.block_density(pos, e.bounding_box);
                    let influence = (1.0 - distance) * density as f64;
                    e.attack_from(
                        self,
                        super::damage::DamageSource::new(super::damage::DamageKind::Explosion),
                        ((influence * influence + influence) / 2.0 * 8.0 * radius as f64 + 1.0)
                            as i32,
                    );
                    e.motion.x += d.x * influence;
                    e.motion.y += d.y * influence;
                    e.motion.z += d.z * influence;
                }
                self.entities.insert(id, e);
            }
        }
        let destroyed_blocks: Vec<_> = blocks.iter().collect();
        if flaming {
            for &(x, y, z) in destroyed_blocks.iter().rev() {
                if self.get_block_id(x, y, z) == 0
                    && opaque_cube_lookup(self.get_block_id(x, y - 1, z))
                    && rng.next_int(3) == 0
                {
                    self.set_block_with_notify(x, y, z, FIRE);
                }
            }
        }
        let pitch = (1.0 + (self.rand.next_float() - self.rand.next_float()) * 0.2) * 0.7;
        self.play_sound("random.explode", pos, 4.0, pitch);
        self.spawn_particle("hugeexplosion", pos, Vec3D::default());
        for &(x, y, z) in destroyed_blocks.iter().rev() {
            let id = self.get_block_id(x, y, z);
            let p = Vec3D::new(
                (x as f32 + self.rand.next_float()) as f64,
                (y as f32 + self.rand.next_float()) as f64,
                (z as f32 + self.rand.next_float()) as f64,
            );
            let d = p.add(-pos.x, -pos.y, -pos.z);
            let length = d.length() as f32 as f64;
            let mut speed = 0.5 / (length / size as f64 + 0.1);
            speed *= (self.rand.next_float() * self.rand.next_float() + 0.3) as f64;
            let v = Vec3D::new(
                d.x / length * speed,
                d.y / length * speed,
                d.z / length * speed,
            );
            self.spawn_particle(
                "explode",
                Vec3D::new(
                    (p.x + pos.x * 1.0) / 2.0,
                    (p.y + pos.y * 1.0) / 2.0,
                    (p.z + pos.z * 1.0) / 2.0,
                ),
                v,
            );
            self.spawn_particle("smoke", p, v);
            if id > 0 {
                let meta = self.get_block_metadata(x, y, z);
                behavior::drop_block_with_chance(self, id, x, y, z, meta, 0.3);
                self.set_block_with_notify(x, y, z, 0);
                if id == 46 {
                    let mut e = Entity::tnt(
                        self,
                        (x as f32 + 0.5) as f64,
                        (y as f32 + 0.5) as f64,
                        (z as f32 + 0.5) as f64,
                    );
                    let fuse = self.rand.next_int(20) + 10;
                    if let super::EntityKind::Tnt { fuse: f } = &mut e.kind {
                        *f = fuse;
                    }
                    self.entity_joined_world(e);
                }
            }
        }
        Explosion {
            position: pos,
            size,
            flaming,
            destroyed_blocks,
        }
    }
}
