//! EntityPainting and ItemPainting, retaining the original 101-tick support check.
use super::{AxisAlignedBB as BB, Entity, EntityKind};
use crate::{World, block::mat, math::floor_double as floor};
#[derive(Clone, Copy, Debug)]
pub struct Art {
    pub title: &'static str,
    pub width: i32,
    pub height: i32,
    pub u: i32,
    pub v: i32,
}
pub const ARTS: [Art; 25] = [
    Art {
        title: "Kebab",
        width: 16,
        height: 16,
        u: 0,
        v: 0,
    },
    Art {
        title: "Aztec",
        width: 16,
        height: 16,
        u: 16,
        v: 0,
    },
    Art {
        title: "Alban",
        width: 16,
        height: 16,
        u: 32,
        v: 0,
    },
    Art {
        title: "Aztec2",
        width: 16,
        height: 16,
        u: 48,
        v: 0,
    },
    Art {
        title: "Bomb",
        width: 16,
        height: 16,
        u: 64,
        v: 0,
    },
    Art {
        title: "Plant",
        width: 16,
        height: 16,
        u: 80,
        v: 0,
    },
    Art {
        title: "Wasteland",
        width: 16,
        height: 16,
        u: 96,
        v: 0,
    },
    Art {
        title: "Pool",
        width: 32,
        height: 16,
        u: 0,
        v: 32,
    },
    Art {
        title: "Courbet",
        width: 32,
        height: 16,
        u: 32,
        v: 32,
    },
    Art {
        title: "Sea",
        width: 32,
        height: 16,
        u: 64,
        v: 32,
    },
    Art {
        title: "Sunset",
        width: 32,
        height: 16,
        u: 96,
        v: 32,
    },
    Art {
        title: "Creebet",
        width: 32,
        height: 16,
        u: 128,
        v: 32,
    },
    Art {
        title: "Wanderer",
        width: 16,
        height: 32,
        u: 0,
        v: 64,
    },
    Art {
        title: "Graham",
        width: 16,
        height: 32,
        u: 16,
        v: 64,
    },
    Art {
        title: "Match",
        width: 32,
        height: 32,
        u: 0,
        v: 128,
    },
    Art {
        title: "Bust",
        width: 32,
        height: 32,
        u: 32,
        v: 128,
    },
    Art {
        title: "Stage",
        width: 32,
        height: 32,
        u: 64,
        v: 128,
    },
    Art {
        title: "Void",
        width: 32,
        height: 32,
        u: 96,
        v: 128,
    },
    Art {
        title: "SkullAndRoses",
        width: 32,
        height: 32,
        u: 128,
        v: 128,
    },
    Art {
        title: "Fighters",
        width: 64,
        height: 32,
        u: 0,
        v: 96,
    },
    Art {
        title: "Pointer",
        width: 64,
        height: 64,
        u: 0,
        v: 192,
    },
    Art {
        title: "Pigscene",
        width: 64,
        height: 64,
        u: 64,
        v: 192,
    },
    Art {
        title: "BurningSkull",
        width: 64,
        height: 64,
        u: 128,
        v: 192,
    },
    Art {
        title: "Skeleton",
        width: 64,
        height: 48,
        u: 192,
        v: 64,
    },
    Art {
        title: "DonkeyKong",
        width: 64,
        height: 48,
        u: 192,
        v: 112,
    },
];
#[derive(Clone, Debug, Default)]
pub struct PaintingState {
    pub check_counter: i32,
    pub direction: i32,
    pub tile: [i32; 3],
    pub art: usize,
}
impl Entity {
    pub fn painting_base(w: &mut World) -> Self {
        let mut e = Self::base(w, EntityKind::Painting(Default::default()));
        e.y_offset = 0.0;
        e.width = 0.5;
        e.height = 0.5;
        e
    }
    pub fn painting(w: &mut World, x: i32, y: i32, z: i32, direction: i32) -> Self {
        let mut e = Self::painting_base(w);
        if let EntityKind::Painting(s) = &mut e.kind {
            s.tile = [x, y, z];
        }
        let mut fitting = Vec::new();
        for art in 0..ARTS.len() {
            if let EntityKind::Painting(s) = &mut e.kind {
                s.art = art;
            }
            e.set_painting_direction(direction);
            if e.painting_valid_surface(w) {
                fitting.push(art);
            }
        }
        if !fitting.is_empty() {
            let art = fitting[e.rand.next_int(fitting.len() as i32) as usize];
            if let EntityKind::Painting(s) = &mut e.kind {
                s.art = art;
            }
        }
        e.set_painting_direction(direction);
        e
    }
    pub fn set_painting_direction(&mut self, direction: i32) {
        let EntityKind::Painting(s) = &mut self.kind else {
            return;
        };
        s.direction = direction;
        let art = ARTS[s.art];
        self.yaw = (direction * 90) as f32;
        self.prev_yaw = self.yaw;
        let mut sx = art.width as f32;
        let sy = art.height as f32 / 32.0;
        let mut sz = art.width as f32;
        if direction != 0 && direction != 2 {
            sx = 0.5;
        } else {
            sz = 0.5;
        }
        sx /= 32.0;
        sz /= 32.0;
        let [tx, ty, tz] = s.tile;
        let mut x = tx as f32 + 0.5;
        let mut y = ty as f32 + 0.5;
        let mut z = tz as f32 + 0.5;
        let offset = |size| {
            if size == 32 || size == 64 {
                0.5f32
            } else {
                0.0
            }
        };
        match direction {
            0 => {
                z -= 9.0 / 16.0;
                x -= offset(art.width);
            }
            1 => {
                x -= 9.0 / 16.0;
                z += offset(art.width);
            }
            2 => {
                z += 9.0 / 16.0;
                x += offset(art.width);
            }
            3 => {
                x += 9.0 / 16.0;
                z -= offset(art.width);
            }
            _ => {}
        }
        y += offset(art.height);
        self.set_position(x as f64, y as f64, z as f64);
        let inset = -(0.1f32 / 16.0);
        self.bounding_box = BB::new(
            (x - sx - inset) as f64,
            (y - sy - inset) as f64,
            (z - sz - inset) as f64,
            (x + sx + inset) as f64,
            (y + sy + inset) as f64,
            (z + sz + inset) as f64,
        );
    }
    pub fn painting_valid_surface(&self, w: &mut World) -> bool {
        let EntityKind::Painting(s) = &self.kind else {
            return false;
        };
        if !w
            .get_colliding_bounding_boxes(Some(self.entity_id), self.bounding_box)
            .is_empty()
        {
            return false;
        }
        let art = ARTS[s.art];
        let [mut x, _, mut z] = s.tile;
        if s.direction == 0 || s.direction == 2 {
            x = floor(self.pos.x - (art.width as f32 / 32.0) as f64);
        } else {
            z = floor(self.pos.z - (art.width as f32 / 32.0) as f64);
        }
        let y = floor(self.pos.y - (art.height as f32 / 32.0) as f64);
        for dx in 0..art.width / 16 {
            for dy in 0..art.height / 16 {
                let m = if s.direction == 0 || s.direction == 2 {
                    w.get_block_material(x + dx, y + dy, s.tile[2])
                } else {
                    w.get_block_material(s.tile[0], y + dy, z + dx)
                };
                if !mat(m).is_solid {
                    return false;
                }
            }
        }
        !w.entities_within_aabb(Some(self.entity_id), self.bounding_box)
            .into_iter()
            .any(|id| matches!(w.entities[&id].kind, EntityKind::Painting(_)))
    }
    pub(super) fn update_painting(&mut self, w: &mut World) {
        let EntityKind::Painting(s) = &mut self.kind else {
            return;
        };
        let check = s.check_counter == 100;
        s.check_counter += 1;
        if check && !w.multiplayer_world {
            s.check_counter = 0;
            if !self.painting_valid_surface(w) {
                self.break_painting(w, false);
            }
        }
    }
    pub fn break_painting(&mut self, w: &mut World, attacked: bool) {
        self.dead = true;
        if attacked {
            self.been_attacked = true;
        }
        let e = Self::item(
            w,
            self.pos.x,
            self.pos.y,
            self.pos.z,
            crate::world::tile::ItemStack::new(321, 1, 0),
        );
        w.entity_joined_world(e);
    }
    /// Entity.addVelocity, overridden by EntityPainting even when already dead.
    pub fn add_velocity(&mut self, w: &mut World, x: f64, y: f64, z: f64) {
        if matches!(self.kind, EntityKind::Painting(_)) {
            if !w.multiplayer_world && x * x + y * y + z * z > 0.0 {
                self.break_painting(w, false);
            }
            return;
        }
        self.motion.x += x;
        self.motion.y += y;
        self.motion.z += z;
    }
    pub fn place_painting(
        &self,
        w: &mut World,
        stack: &mut super::player::ItemStack,
        x: i32,
        y: i32,
        z: i32,
        side: i32,
    ) -> bool {
        if side == 0 || side == 1 {
            return false;
        }
        if !self.can_edit_block181(x, y, z) {
            return false;
        }
        let direction = match side {
            4 => 1,
            3 => 2,
            5 => 3,
            _ => 0,
        };
        let e = Self::painting(w, x, y, z, direction);
        if e.painting_valid_surface(w) {
            if !w.multiplayer_world {
                w.entity_joined_world(e);
            }
            stack.count = stack.count.wrapping_sub(1);
        }
        true
    }
}
