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
            // The inherited table is already stably sorted. Insert each new
            // registration beside its predecessor in the same comparator bucket.
            // A Block ingredient uses the Java shaped-recipe wildcard damage -1.
            let shaped = |w,h,id,count,ids:Vec<i32>| Recipe {
                width:w,height:h,shaped:true,output:ItemStack::new(id,count,0),
                ingredients:ids.into_iter().map(|id|if id==0 {None}else{Some(ItemStack::new(id,1,if id<256 {-1}else{0}))}).collect(),
            };
            let after = |recipes:&mut Vec<Recipe>,recipe:Recipe,anchor:i32| {
                let index=recipes.iter().rposition(|r|r.shaped==recipe.shaped && r.ingredients.len()==recipe.ingredients.len() && r.output.id==anchor).expect("source registration predecessor")+1;
                recipes.insert(index,recipe);
            };
            after(&mut recipes,shaped(3,3,266,1,vec![371;9]),22);
            // The last ingot unpacking recipe returns nine lapis dyes.
            let lapis=recipes.iter().position(|r|r.shaped&&r.width==1&&r.height==1&&r.output.id==351&&r.output.count==9).unwrap()+1;
            recipes.insert(lapis,shaped(1,1,371,9,vec![266]));
            after(&mut recipes,shaped(3,2,113,6,vec![112;6]),85);
            after(&mut recipes,shaped(3,2,374,3,vec![20,0,20,0,20,0]),281);
            after(&mut recipes,shaped(3,3,380,1,vec![265,0,265,265,0,265,265,265,265]),28);
            after(&mut recipes,shaped(3,2,379,1,vec![0,369,0,4,4,4]),328);
            after(&mut recipes,shaped(3,3,114,4,vec![112,0,0,112,112,0,112,112,112]),109);
            after(&mut recipes,shaped(3,3,116,1,vec![0,340,0,264,49,264,49,49,49]),33);
            let shapeless=|id,count,ids:Vec<i32>|Recipe{width:0,height:0,shaped:false,output:ItemStack::new(id,count,0),ingredients:ids.into_iter().map(|id|Some(ItemStack::new(id,1,0))).collect()};
            recipes.retain(|r|!(r.shaped&&r.output.id==282));
            after(&mut recipes,shaped(1,1,361,4,vec![86]),362);
            let soup=shapeless(282,1,vec![39,40,281]);
            let index=recipes.iter().position(|r|!r.shaped&&r.ingredients.len()==3).unwrap();
            recipes.insert(index,soup);
            after(&mut recipes,shapeless(376,1,vec![375,39,353]),282);
            for (id,count,ids) in [(382,1,vec![360,371]),(378,1,vec![377,341]),(377,2,vec![369])] {
                let recipe=shapeless(id,count,ids);
                let index=recipes.iter().position(|r|!r.shaped&&r.ingredients.len()==recipe.ingredients.len()).unwrap_or(recipes.len());
                let index=if id==378 {index+1}else{index};
                recipes.insert(index,recipe);
            }
            let eye=shapeless(381,1,vec![368,377]);
            let index=recipes.iter().rposition(|r|!r.shaped&&r.ingredients.len()==2).unwrap()+1;
            recipes.insert(index,eye);
            smelting[16]=Some(ItemStack::new(263,1,0));
            smelting[73]=Some(ItemStack::new(331,1,0));
            smelting[21]=Some(ItemStack::new(351,1,4));
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
