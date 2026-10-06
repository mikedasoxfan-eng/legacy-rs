//! Original CraftingManager's stable sorted registration order, extracted from bytecode.
use super::ItemStack;
use std::sync::OnceLock;

#[derive(Clone, Debug)]
pub struct Recipe {
    pub width: usize,
    pub height: usize,
    pub shaped: bool,
    pub output: ItemStack,
    pub ingredients: Vec<Option<ItemStack>>,
}
pub struct CraftingManager {
    pub recipes: Vec<Recipe>,
    pub smelting: Vec<Option<ItemStack>>,
    pub container_items: Vec<i32>,
    pub armor_types: Vec<i32>,
    pub wood: Vec<bool>,
}
impl CraftingManager {
    pub fn instance() -> &'static Self {
        static INSTANCE: OnceLock<CraftingManager> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            let mut r = Reader(include_bytes!("recipes.bin"));
            let n = r.int();
            let mut recipes = Vec::new();
            for _ in 0..n {
                let shaped = r.byte();
                let width = r.int() as usize;
                let height = r.int() as usize;
                let output = r.stack().unwrap();
                let n = r.int();
                let ingredients = (0..n).map(|_| r.stack()).collect();
                recipes.push(Recipe {
                    shaped,
                    width,
                    height,
                    output,
                    ingredients,
                });
            }
            let (mut smelting, mut container_items, mut armor_types, mut wood) =
                (Vec::new(), Vec::new(), Vec::new(), Vec::new());
            for _ in 0..2258 {
                smelting.push(r.stack());
                container_items.push(r.int());
                armor_types.push(r.int());
                wood.push(r.byte());
            }
            assert!(r.0.is_empty());
            // All release registrations and their stable sort order come from original bytecode.
            Self {
                recipes,
                smelting,
                container_items,
                armor_types,
                wood,
            }
        })
    }
    pub fn find_matching_recipe(
        &self,
        grid: &[Option<ItemStack>],
        width: usize,
    ) -> Option<ItemStack> {
        let items:Vec<_>=grid.iter().flatten().copied().collect();
        if items.len()==2 && items[0].id==items[1].id && items[0].count==1 && items[1].count==1
            && items[0].max_damage()>0 {
            let max=items[0].max_damage();
            let remaining=max.wrapping_sub(items[0].damage).wrapping_add(max.wrapping_sub(items[1].damage))
                .wrapping_add(max.wrapping_mul(10)/100);
            return Some(ItemStack::new(items[0].id,1,max.wrapping_sub(remaining).max(0)));
        }
        self.recipes
            .iter()
            .find(|r| r.matches(grid, width))
            .map(|r| r.output.copy_stack181())
    }
    pub fn smelting_result(&self, id: i32) -> Option<ItemStack> {
        self.smelting.get(id as usize).copied().flatten()
    }
    pub fn fuel(&self, stack: Option<ItemStack>) -> i32 {
        let Some(s) = stack else { return 0 };
        if self.wood.get(s.id as usize) == Some(&true) {
            300
        } else {
            match s.id {
                280 | 6 => 100,
                263 => 1600,
                327 => 20000,
                369 => 2400,
                _ => 0,
            }
        }
    }
}
impl Recipe {
    pub fn matches(&self, grid: &[Option<ItemStack>], width: usize) -> bool {
        // InventoryCrafting.func_21103_b checks X and total index, not Y.
        let at = |x: usize, y: usize| {
            if x < width {
                grid.get(x + y * width).copied().flatten()
            } else {
                None
            }
        };
        if self.shaped {
            for ox in 0..=3 - self.width {
                for oy in 0..=3 - self.height {
                    for mirror in [true, false] {
                        let mut matches = true;
                        for x in 0..3 {
                            for y in 0..3 {
                                let expected = if x >= ox
                                    && y >= oy
                                    && x - ox < self.width
                                    && y - oy < self.height
                                {
                                    let rx = if mirror {
                                        self.width - (x - ox) - 1
                                    } else {
                                        x - ox
                                    };
                                    self.ingredients[rx + (y - oy) * self.width]
                                } else {
                                    None
                                };
                                matches &= match (expected, at(x, y)) {
                                    (None, None) => true,
                                    (Some(a), Some(b)) => {
                                        a.id == b.id && (a.damage == -1 || a.damage == b.damage)
                                    }
                                    _ => false,
                                };
                            }
                        }
                        if matches {
                            return true;
                        }
                    }
                }
            }
            false
        } else {
            let mut remaining = self.ingredients.clone();
            for y in 0..3 {
                for x in 0..3 {
                    if let Some(s) = at(x, y) {
                        let Some(i) = remaining.iter().position(|a| {
                            a.is_some_and(|a| {
                                a.id == s.id && (a.damage == -1 || a.damage == s.damage)
                            })
                        }) else {
                            return false;
                        };
                        remaining.remove(i);
                    }
                }
            }
            remaining.is_empty()
        }
    }
}
struct Reader<'a>(&'a [u8]);
impl Reader<'_> {
    fn byte(&mut self) -> bool {
        let b = self.0[0];
        self.0 = &self.0[1..];
        b != 0
    }
    fn int(&mut self) -> i32 {
        let v = i32::from_be_bytes(self.0[..4].try_into().unwrap());
        self.0 = &self.0[4..];
        v
    }
    fn stack(&mut self) -> Option<ItemStack> {
        if self.byte() {
            Some(ItemStack::new(self.int(), self.int(), self.int()))
        } else {
            None
        }
    }
}
