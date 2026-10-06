//! ContainerEnchantment's world-independent simulation and bookshelf queries.
use crate::{enchantment as eh, entity::player::ItemStack, JavaRandom, World};
#[derive(Clone, Debug)]
pub struct Enchanting {
    pub item: Option<ItemStack>,
    pub name_seed: i64,
    pub costs: [i32; 3],
    pub rng: JavaRandom,
}
impl Enchanting {
    pub fn new(seed: i64) -> Self {
        Self {
            item: None,
            name_seed: 0,
            costs: [0; 3],
            rng: JavaRandom::new(seed),
        }
    }
    pub fn changed(&mut self, multiplayer: bool, bookshelves: i32) {
        if let Some(item) = self.item.filter(ItemStack::is_enchantable) {
            self.name_seed = self.rng.next_long();
            if !multiplayer {
                for slot in 0..3 {
                    self.costs[slot] =
                        eh::calc_enchantability(&mut self.rng, slot as i32, bookshelves, item);
                }
            }
        } else {
            self.costs = [0; 3];
        }
    }
    pub fn enchant(
        &mut self,
        slot: usize,
        player_level: &mut i32,
        multiplayer: bool,
        bookshelves: i32,
    ) -> bool {
        let cost = self.costs[slot];
        let Some(item) = self.item.filter(|_| cost > 0 && *player_level >= cost) else {
            return false;
        };
        if !multiplayer {
            if let Some(list) = eh::build_list(&mut self.rng, item, cost) {
                *player_level = player_level.wrapping_sub(cost);
                for e in list {
                    item.add_enchantment(e.id, e.level);
                }
                self.changed(false, bookshelves);
            }
        }
        true
    }
    pub fn progress(&mut self, id: i32, value: i32) {
        if (0..=2).contains(&id) {
            self.costs[id as usize] = value;
        }
    }
    pub fn close(&self, multiplayer: bool) -> Option<ItemStack> {
        if multiplayer {
            None
        } else {
            self.item
        }
    }
}
pub fn bookshelf_count(
    mut block: impl FnMut(i32, i32, i32) -> u8,
    (x, y, z): (i32, i32, i32),
) -> i32 {
    let mut count = 0;
    for dz in -1..=1 {
        for dx in -1..=1 {
            if (dx != 0 || dz != 0)
                && block(x + dx, y, z + dz) == 0
                && block(x + dx, y + 1, z + dz) == 0
            {
                for dy in 0..=1 {
                    if block(x + dx * 2, y + dy, z + dz * 2) == 47 {
                        count += 1;
                    }
                }
                if dx != 0 && dz != 0 {
                    for (sx, sz) in [(dx * 2, dz), (dx, dz * 2)] {
                        for dy in 0..=1 {
                            if block(x + sx, y + dy, z + sz) == 47 {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    count
}
pub fn world_bookshelves(world: &mut World, position: (i32, i32, i32)) -> i32 {
    bookshelf_count(|x, y, z| world.get_block_id(x, y, z), position)
}

/// TileEntityEnchantmentTable animation fields, including the static RNG's draws.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EnchantmentTable {
    pub ticks: i32,
    pub page: f32,
    pub previous_page: f32,
    pub target_page: f32,
    pub page_velocity: f32,
    pub open: f32,
    pub previous_open: f32,
    pub rotation: f32,
    pub previous_rotation: f32,
    pub target_rotation: f32,
}
impl EnchantmentTable {
    pub fn tick(
        &mut self,
        position: (i32, i32, i32),
        player: Option<(f64, f64)>,
        rng: &mut JavaRandom,
    ) {
        self.previous_open = self.open;
        self.previous_rotation = self.rotation;
        if let Some((px, pz)) = player {
            self.target_rotation = crate::jmath::java_atan2(
                pz - (position.2 as f32 + 0.5) as f64,
                px - (position.0 as f32 + 0.5) as f64,
            ) as f32;
            self.open += 0.1f32;
            if self.open < 0.5 || rng.next_int(40) == 0 {
                let previous = self.target_page;
                loop {
                    self.target_page += (rng.next_int(4) - rng.next_int(4)) as f32;
                    if previous != self.target_page {
                        break;
                    }
                }
            }
        } else {
            self.target_rotation += 0.02f32;
            self.open -= 0.1f32;
        }
        let pi = std::f32::consts::PI;
        let tau = pi * 2.0f32;
        while self.rotation >= pi {
            self.rotation -= tau;
        }
        while self.rotation < -pi {
            self.rotation += tau;
        }
        while self.target_rotation >= pi {
            self.target_rotation -= tau;
        }
        while self.target_rotation < -pi {
            self.target_rotation += tau;
        }
        let mut delta = self.target_rotation - self.rotation;
        while delta >= pi {
            delta -= tau;
        }
        while delta < -pi {
            delta += tau;
        }
        self.rotation += delta * 0.4f32;
        if self.open < 0.0 {
            self.open = 0.0;
        }
        if self.open > 1.0 {
            self.open = 1.0;
        }
        self.ticks = self.ticks.wrapping_add(1);
        self.previous_page = self.page;
        let mut velocity = (self.target_page - self.page) * 0.4f32;
        if velocity < -0.2 {
            velocity = -0.2;
        }
        if velocity > 0.2 {
            velocity = 0.2;
        }
        self.page_velocity += (velocity - self.page_velocity) * 0.9f32;
        self.page += self.page_velocity;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shelves_and_client_rng() {
        assert_eq!(
            bookshelf_count(
                |x, y, z| if y < 2 && (x.abs() == 2 || z.abs() == 2) && x.abs() <= 2 && z.abs() <= 2
                {
                    47
                } else {
                    0
                },
                (0, 0, 0)
            ),
            32
        );
        let mut c = Enchanting::new(42);
        c.item = Some(ItemStack::new(276, 1, 0));
        let before = c.rng.state();
        c.changed(true, 30);
        assert_ne!(before, c.rng.state());
        assert_eq!(c.costs, [0; 3]);
        c.changed(false, 30);
        let mut level = 50;
        assert!(c.enchant(2, &mut level, false, 30));
        assert!(c.item.unwrap().is_enchanted());
        assert_eq!(c.costs, [0; 3]);
    }
}
