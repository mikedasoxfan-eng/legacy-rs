//! GuiEnchantment's GUI state. All offers and item mutations remain world-owned.
use worldgen::{container::ItemStack, JavaRandom};

pub struct EnchantmentAnimation {
    pub ticks: i32,
    pub page: f32,
    pub previous_page: f32,
    pub target: f32,
    pub velocity: f32,
    pub opening: f32,
    pub previous_opening: f32,
    previous_item: Option<ItemStack>,
    pub random: JavaRandom,
}
impl EnchantmentAnimation {
    pub fn new(seed: i64) -> Self {
        Self {
            ticks: 0,
            page: 0.0,
            previous_page: 0.0,
            target: 0.0,
            velocity: 0.0,
            opening: 0.0,
            previous_opening: 0.0,
            previous_item: None,
            random: JavaRandom::new(seed),
        }
    }
    pub fn tick(&mut self, item: Option<ItemStack>, costs: [i32; 3]) {
        if item
            .zip(self.previous_item)
            .is_some_and(|(a, b)| a.same_object(b))
        {
            self.previous_item = item;
        }
        if !stacks_equal(item, self.previous_item) {
            // Vanilla retains the stack reference, including subsequent mutations.
            self.previous_item = item;
            loop {
                self.target += (self.random.next_int(4) - self.random.next_int(4)) as f32;
                if !(self.page <= self.target + 1.0 && self.page >= self.target - 1.0) {
                    break;
                }
            }
        }
        self.ticks = self.ticks.wrapping_add(1);
        self.previous_page = self.page;
        self.previous_opening = self.opening;
        if costs.iter().any(|&c| c != 0) {
            self.opening += 0.2;
        } else {
            self.opening -= 0.2;
        }
        if self.opening < 0.0 {
            self.opening = 0.0;
        }
        if self.opening > 1.0 {
            self.opening = 1.0;
        }
        let mut change = (self.target - self.page) * 0.4;
        if change < -0.2 {
            change = -0.2;
        }
        if change > 0.2 {
            change = 0.2;
        }
        self.velocity += (change - self.velocity) * 0.9;
        self.page += self.velocity;
    }
    /// Parameters passed to the render worker's ModelBook.
    pub fn pose(&self, partial: f32) -> [f32; 3] {
        let opening = self.previous_opening + (self.opening - self.previous_opening) * partial;
        let page = self.previous_page + (self.page - self.previous_page) * partial;
        let fold = |offset: f32| {
            let x = page + offset;
            let x = (x - worldgen::math::floor_double(x as f64) as f32) * 1.6 - 0.3;
            x.clamp(0.0, 1.0)
        };
        [fold(0.25), fold(0.75), opening]
    }
}
fn stacks_equal(a: Option<ItemStack>, b: Option<ItemStack>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => {
            a.id == b.id && a.count == b.count && a.damage == b.damage && a.tags_equal(&b)
        }
        _ => false,
    }
}
pub fn names(seed: i64) -> [String; 3] {
    let mut names = worldgen::enchantment::EnchantmentNameParts::new(seed);
    std::array::from_fn(|_| names.next_name())
}
pub fn offer(cost: i32, level: i32, hovered: bool) -> (i32, i32, i32) {
    if cost == 0 {
        (185, 0, 0)
    } else if level < cost {
        (185, (6839882 & 16711422) >> 1, 4226832)
    } else if hovered {
        (204, 16777088, 8453920)
    } else {
        (166, 6839882, 8453920)
    }
}
