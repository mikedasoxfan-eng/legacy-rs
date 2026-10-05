//! Immutable Item/Block defaults extracted from the original bytecode.
//! Regenerate with tools/oracle181/oracle.sh wbregistry registry.bin.
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

/// EnumAction in the original declaration order.
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
#[repr(i32)]
pub enum ItemUseAction {#[default] None=0,Eat=1,Block=2,Bow=3}
pub fn use_action(id:i32)->ItemUseAction {
    match id {260|282|297|319|320|322|349|350|357|360|363..=367=>ItemUseAction::Eat,
        267|268|272|276|283=>ItemUseAction::Block,261=>ItemUseAction::Bow,_=>ItemUseAction::None}
}
pub fn use_duration(id:i32)->i32 {match use_action(id){ItemUseAction::Eat=>32,ItemUseAction::Block|ItemUseAction::Bow=>72000,ItemUseAction::None=>0}}
/// ItemBow's release calculation; None means the shot is too weak to fire.
pub fn bow_charge(remaining:i32)->Option<f32>{
    let mut f=72000i32.wrapping_sub(remaining) as f32/20.0f32;
    f=(f*f+f*2.0f32)/3.0f32;
    if (f as f64)<0.1f64 {None}else{if f>1.0{f=1.0;}Some(f)}
}
