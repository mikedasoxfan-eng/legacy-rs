//! Immutable Item/Block defaults extracted from the original bytecode.
//! Regenerate with tools/oracle/build_player_registry.py.
const DATA: &[u8] = include_bytes!("registry.bin");
const ITEM_SIZE: usize = 16 + 256 * 5;
fn int(at: usize) -> i32 {
    i32::from_le_bytes(DATA[at..at + 4].try_into().unwrap())
}
#[derive(Clone, Copy, Debug)]
pub struct ItemDefinition {
    pub max_stack: i32,
    pub durability: i32,
    pub subtypes: bool,
    pub armor: i32,
}
pub fn item(id: i32) -> Option<ItemDefinition> {
    if !(0..=2257).contains(&id) {
        return None;
    }
    let at = id as usize * ITEM_SIZE;
    let max_stack = int(at);
    (max_stack != 0).then(|| ItemDefinition {
        max_stack,
        durability: int(at + 4),
        subtypes: int(at + 8) != 0,
        armor: int(at + 12),
    })
}
pub fn strength(id: i32, block: u8) -> f32 {
    if item(id).is_none() {
        return 1.0;
    }
    f32::from_bits(int(id as usize * ITEM_SIZE + 16 + block as usize * 5) as u32)
}
pub fn can_harvest(id: i32, block: u8) -> bool {
    harvestable(block)
        || (item(id).is_some() && DATA[id as usize * ITEM_SIZE + 20 + block as usize * 5] != 0)
}
pub fn harvestable(block: u8) -> bool {
    DATA[2258 * ITEM_SIZE + block as usize * 5 + 4] != 0
}
pub fn hardness(block: u8) -> f32 {
    f32::from_bits(int(2258 * ITEM_SIZE + block as usize * 5) as u32)
}
pub fn attack_damage(id: i32) -> i32 {
    if item(id).is_none() {
        1
    } else {
        int(2258 * ITEM_SIZE + 256 * 5 + id as usize * 4)
    }
}
