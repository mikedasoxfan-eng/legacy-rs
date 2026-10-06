//! Release 1.0 enchantments, material tables and EnchantmentHelper.
use crate::{entity::player::ItemStack, JavaRandom};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmorMaterial {
    Cloth,
    Chain,
    Iron,
    Gold,
    Diamond,
}
impl ArmorMaterial {
    pub fn durability(self, slot: usize) -> i32 {
        [11, 16, 15, 13][slot]
            * match self {
                Self::Cloth => 5,
                Self::Chain | Self::Iron => 15,
                Self::Gold => 7,
                Self::Diamond => 33,
            }
    }
    pub fn reduction(self, slot: usize) -> i32 {
        (match self {
            Self::Cloth => [1, 3, 2, 1],
            Self::Chain => [2, 5, 4, 1],
            Self::Iron => [2, 6, 5, 2],
            Self::Gold => [2, 5, 3, 1],
            Self::Diamond => [3, 8, 6, 3],
        })[slot]
    }
    pub fn enchantability(self) -> i32 {
        match self {
            Self::Cloth => 15,
            Self::Chain => 12,
            Self::Iron => 9,
            Self::Gold => 25,
            Self::Diamond => 10,
        }
    }
}
pub fn armor(id: i32) -> Option<(ArmorMaterial, usize)> {
    if !(298..=317).contains(&id) {
        return None;
    }
    Some((
        [
            ArmorMaterial::Cloth,
            ArmorMaterial::Chain,
            ArmorMaterial::Iron,
            ArmorMaterial::Diamond,
            ArmorMaterial::Gold,
        ][((id - 298) / 4) as usize],
        ((id - 298) % 4) as usize,
    ))
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnchantmentType {
    All,
    Armor,
    Feet,
    Legs,
    Torso,
    Head,
    Weapon,
    Digger,
}
pub fn is_sword(id: i32) -> bool {
    matches!(id, 267 | 268 | 272 | 276 | 283)
}
pub fn is_tool(id: i32) -> bool {
    matches!(id,256..=258|269..=271|273..=275|277..=279|284..=286)
}
impl EnchantmentType {
    pub fn can_enchant(self, id: i32) -> bool {
        if self == Self::All {
            return true;
        }
        if let Some((_, slot)) = armor(id) {
            return self == Self::Armor
                || self == [Self::Head, Self::Torso, Self::Legs, Self::Feet][slot];
        }
        if is_sword(id) {
            self == Self::Weapon
        } else {
            is_tool(id) && self == Self::Digger
        }
    }
}
pub fn item_enchantability(id: i32) -> i32 {
    if let Some((m, _)) = armor(id) {
        return m.enchantability();
    }
    match id {
        268..=271 => 15,
        272..=275 => 5,
        256..=258 | 267 => 14,
        276..=279 => 10,
        283..=286 => 22,
        _ => 0,
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Enchantment {
    pub id: i32,
    pub weight: i32,
    pub kind: EnchantmentType,
    pub name: &'static str,
}
pub const ENCHANTMENTS: [Enchantment; 17] = [
    Enchantment {
        id: 0,
        weight: 10,
        kind: EnchantmentType::Armor,
        name: "protect.all",
    },
    Enchantment {
        id: 1,
        weight: 5,
        kind: EnchantmentType::Armor,
        name: "protect.fire",
    },
    Enchantment {
        id: 2,
        weight: 5,
        kind: EnchantmentType::Feet,
        name: "protect.fall",
    },
    Enchantment {
        id: 3,
        weight: 2,
        kind: EnchantmentType::Armor,
        name: "protect.explosion",
    },
    Enchantment {
        id: 4,
        weight: 5,
        kind: EnchantmentType::Armor,
        name: "protect.projectile",
    },
    Enchantment {
        id: 5,
        weight: 2,
        kind: EnchantmentType::Head,
        name: "oxygen",
    },
    Enchantment {
        id: 6,
        weight: 2,
        kind: EnchantmentType::Head,
        name: "waterWorker",
    },
    Enchantment {
        id: 16,
        weight: 10,
        kind: EnchantmentType::Weapon,
        name: "damage.all",
    },
    Enchantment {
        id: 17,
        weight: 5,
        kind: EnchantmentType::Weapon,
        name: "damage.undead",
    },
    Enchantment {
        id: 18,
        weight: 5,
        kind: EnchantmentType::Weapon,
        name: "damage.arthropods",
    },
    Enchantment {
        id: 19,
        weight: 5,
        kind: EnchantmentType::Weapon,
        name: "knockback",
    },
    Enchantment {
        id: 20,
        weight: 2,
        kind: EnchantmentType::Weapon,
        name: "fire",
    },
    Enchantment {
        id: 21,
        weight: 2,
        kind: EnchantmentType::Weapon,
        name: "lootBonus",
    },
    Enchantment {
        id: 32,
        weight: 10,
        kind: EnchantmentType::Digger,
        name: "digging",
    },
    Enchantment {
        id: 33,
        weight: 1,
        kind: EnchantmentType::Digger,
        name: "untouching",
    },
    Enchantment {
        id: 34,
        weight: 5,
        kind: EnchantmentType::Digger,
        name: "durability",
    },
    Enchantment {
        id: 35,
        weight: 2,
        kind: EnchantmentType::Digger,
        name: "lootBonusDigger",
    },
];
pub fn get(id: i32) -> Option<Enchantment> {
    assert!((0..256).contains(&id), "ArrayIndexOutOfBoundsException");
    ENCHANTMENTS.iter().find(|e| e.id == id).copied()
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ProtectionSource {
    pub creative_harm: bool,
    pub fire: bool,
    pub fall: bool,
    pub explosion: bool,
    pub projectile: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CreatureAttribute {
    Undefined,
    Undead,
    Arthropod,
}
impl Enchantment {
    pub fn min_level(self) -> i32 {
        1
    }
    pub fn max_level(self) -> i32 {
        match self.id {
            0..=4 => 4,
            5 | 21 | 34 | 35 => 3,
            16..=18 | 32 => 5,
            19 | 20 => 2,
            _ => 1,
        }
    }
    pub fn min_enchantability(self, level: i32) -> i32 {
        let prev = level.wrapping_sub(1);
        match self.id {
            0..=4 => [1i32, 10, 5, 5, 3][self.id as usize]
                .wrapping_add(prev.wrapping_mul([16, 8, 6, 8, 6][self.id as usize])),
            5 => level.wrapping_mul(10),
            6 => 1,
            16 => 1i32.wrapping_add(prev.wrapping_mul(16)),
            17 | 18 => 5i32.wrapping_add(prev.wrapping_mul(8)),
            19 => 5i32.wrapping_add(prev.wrapping_mul(20)),
            20 => 10i32.wrapping_add(prev.wrapping_mul(20)),
            21 | 35 => 20i32.wrapping_add(prev.wrapping_mul(12)),
            32 => 1i32.wrapping_add(prev.wrapping_mul(15)),
            33 => 25,
            34 => 5i32.wrapping_add(prev.wrapping_mul(10)),
            _ => 1i32.wrapping_add(level.wrapping_mul(10)),
        }
    }
    pub fn max_enchantability(self, level: i32) -> i32 {
        match self.id {
            0..=4 => self
                .min_enchantability(level)
                .wrapping_add([20, 12, 10, 12, 15][self.id as usize]),
            5 => self.min_enchantability(level).wrapping_add(30),
            6 => 41,
            16..=18 => self.min_enchantability(level).wrapping_add(20),
            // These subclasses explicitly call super.getMinEnchantability.
            19..=21 | 32..=35 => level.wrapping_mul(10).wrapping_add(51),
            _ => self.min_enchantability(level).wrapping_add(5),
        }
    }
    pub fn compatible(self, other: Self) -> bool {
        if self.id <= 4 && other.id <= 4 {
            return self.id != other.id && (self.id == 2 || other.id == 2);
        }
        if (16..=18).contains(&self.id) {
            return !(16..=18).contains(&other.id);
        }
        self.id != other.id
            && match self.id {
                21 | 35 => other.id != 33,
                33 => other.id != 35,
                _ => true,
            }
    }
    pub fn modifier_damage(self, level: i32, source: ProtectionSource) -> i32 {
        if source.creative_harm || self.id > 4 {
            return 0;
        }
        let value = 6i32.wrapping_add(level.wrapping_mul(level)) / 2;
        match self.id {
            0 => value,
            1 if source.fire => value,
            2 if source.fall => value.wrapping_mul(2),
            3 if source.explosion => value,
            4 if source.projectile => value,
            _ => 0,
        }
    }
    pub fn modifier_living(self, level: i32, attribute: CreatureAttribute) -> i32 {
        match self.id {
            16 => level.wrapping_mul(3),
            17 if attribute == CreatureAttribute::Undead => level.wrapping_mul(4),
            18 if attribute == CreatureAttribute::Arthropod => level.wrapping_mul(4),
            _ => 0,
        }
    }
    pub fn translation_key(self) -> String {
        format!("enchantment.{}", self.name)
    }
    pub fn translated_name(self, level: i32, translate: impl Fn(&str) -> String) -> String {
        format!(
            "{} {}",
            translate(&self.translation_key()),
            translate(&format!("enchantment.level.{level}"))
        )
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnchantmentData {
    pub id: i32,
    pub level: i32,
}
impl crate::weighted::Weighted for EnchantmentData {
    fn weight(&self) -> i32 {
        get(self.id).unwrap().weight
    }
}
pub fn level(id: i32, stack: Option<ItemStack>) -> i32 {
    stack
        .and_then(|s| s.enchantments().into_iter().find(|e| e.id == id))
        .map_or(0, |e| e.level)
}
pub fn max_level(id: i32, stacks: &[Option<ItemStack>]) -> i32 {
    stacks.iter().fold(0, |n, s| n.max(level(id, *s)))
}
pub fn modifier_damage(
    rng: &mut JavaRandom,
    stacks: &[Option<ItemStack>],
    source: ProtectionSource,
) -> i32 {
    let mut value = 0i32;
    for s in stacks.iter().flatten() {
        for data in s.enchantments() {
            if let Some(e) = get(data.id) {
                value = value.wrapping_add(e.modifier_damage(data.level, source));
            }
        }
    }
    value = value.min(25);
    (value.wrapping_add(1) >> 1).wrapping_add(rng.next_int((value >> 1).wrapping_add(1)))
}
pub fn modifier_living(
    rng: &mut JavaRandom,
    stack: Option<ItemStack>,
    attribute: CreatureAttribute,
) -> i32 {
    let mut value = 0i32;
    if let Some(s) = stack {
        for data in s.enchantments() {
            if let Some(e) = get(data.id) {
                value = value.wrapping_add(e.modifier_living(data.level, attribute));
            }
        }
    }
    if value > 0 {
        1 + rng.next_int(value)
    } else {
        0
    }
}
pub fn calc_enchantability(
    rng: &mut JavaRandom,
    slot: i32,
    mut bookshelves: i32,
    stack: ItemStack,
) -> i32 {
    if item_enchantability(stack.id) <= 0 {
        return 0;
    }
    if bookshelves > 30 {
        bookshelves = 30;
    }
    bookshelves = 1 + rng.next_int((bookshelves >> 1) + 1) + rng.next_int(bookshelves + 1);
    let value = rng.next_int(5) + bookshelves;
    if slot == 0 {
        (value >> 1) + 1
    } else if slot == 1 {
        value * 2 / 3 + 1
    } else {
        value
    }
}
pub fn map_data(power: i32, stack: ItemStack) -> Option<Vec<EnchantmentData>> {
    let mut data = Vec::new();
    for e in ENCHANTMENTS {
        if e.kind.can_enchant(stack.id) {
            for level in e.min_level()..=e.max_level() {
                if power >= e.min_enchantability(level) && power <= e.max_enchantability(level) {
                    if let Some(d) = data
                        .iter_mut()
                        .find(|d: &&mut EnchantmentData| d.id == e.id)
                    {
                        d.level = level;
                    } else {
                        data.push(EnchantmentData { id: e.id, level });
                    }
                }
            }
        }
    }
    // java.util.HashMap<Integer,...> values: stable order in each of 16 buckets.
    data.sort_by_key(|d| ((d.id as u32) ^ (d.id as u32 >> 16)) & 15);
    if data.is_empty() {
        None
    } else {
        Some(data)
    }
}
pub fn build_list(
    rng: &mut JavaRandom,
    stack: ItemStack,
    power: i32,
) -> Option<Vec<EnchantmentData>> {
    let enchantability = item_enchantability(stack.id);
    if enchantability <= 0 {
        return None;
    }
    let bonus =
        1 + rng.next_int((enchantability >> 1) + 1) + rng.next_int((enchantability >> 1) + 1);
    let base = bonus.wrapping_add(power);
    let variation = (rng.next_float() + rng.next_float() - 1.0f32) * 0.25f32;
    let adjusted = (base as f32 * (1.0f32 + variation) + 0.5f32) as i32;
    let mut candidates = map_data(adjusted, stack)?;
    let first = *crate::weighted::choose(rng, &candidates)?;
    let mut result = vec![first];
    let mut remaining = adjusted >> 1;
    while rng.next_int(50) <= remaining {
        candidates.retain(|c| {
            result
                .iter()
                .all(|d| get(d.id).unwrap().compatible(get(c.id).unwrap()))
        });
        if !candidates.is_empty() {
            result.push(*crate::weighted::choose(rng, &candidates).unwrap());
        }
        remaining >>= 1;
    }
    Some(result)
}
pub struct EnchantmentNameParts {
    rng: JavaRandom,
}
impl EnchantmentNameParts {
    pub fn new(seed: i64) -> Self {
        Self {
            rng: JavaRandom::new(seed),
        }
    }
    pub fn set_seed(&mut self, seed: i64) {
        self.rng.set_seed(seed);
    }
    pub fn next_name(&mut self) -> String {
        let words:Vec<_>="the elder scrolls klaatu berata niktu xyzzy bless curse light darkness fire air earth water hot dry cold wet ignite snuff embiggen twist shorten stretch fiddle destroy imbue galvanize enchant free limited range of towards inside sphere cube self other ball mental physical grow shrink demon elemental spirit animal creature beast humanoid undead fresh stale".split(' ').collect();
        let count = self.rng.next_int(2) + 3;
        (0..count)
            .map(|_| words[self.rng.next_int(words.len() as i32) as usize])
            .collect::<Vec<_>>()
            .join(" ")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_materials_and_ranges() {
        assert_eq!(ArmorMaterial::Diamond.durability(1), 528);
        assert_eq!(get(32).unwrap().max_enchantability(1), 61);
        assert!(!get(0).unwrap().compatible(get(1).unwrap()));
        assert!(get(0).unwrap().compatible(get(2).unwrap()));
        assert!(!EnchantmentType::Digger.can_enchant(290));
    }
    #[test]
    fn candidate_levels_and_rng() {
        let s = ItemStack::new(276, 1, 0);
        let list = map_data(30, s).unwrap();
        assert_eq!(list.iter().find(|e| e.id == 16).unwrap().level, 2);
        let mut r = JavaRandom::new(42);
        let before = r.state();
        assert_eq!(
            calc_enchantability(&mut r, 0, 30, ItemStack::new(1, 1, 0)),
            0
        );
        assert_eq!(r.state(), before);
        let list = build_list(&mut r, s, 30).unwrap();
        for a in &list {
            for b in &list {
                if a.id != b.id {
                    assert!(get(a.id).unwrap().compatible(get(b.id).unwrap()));
                }
            }
        }
    }
}
