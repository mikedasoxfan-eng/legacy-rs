//! Immutable release Item/Block defaults extracted from the original bytecode.
//! Regenerate with tools/oracle10/build_player_registry.py.
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
    if id==383 {return Some(ItemDefinition{max_stack:1,durability:0,subtypes:true,armor:0});}
    if let Some((material,slot))=crate::enchantment::armor(id) {return Some(ItemDefinition{max_stack:1,durability:material.durability(slot),subtypes:false,armor:material.reduction(slot)});}
    if (110..=122).contains(&id)||(369..=382).contains(&id)||(2258..=2266).contains(&id) {
        return Some(ItemDefinition{max_stack:if id==373||id>=2258 {1}else{64},durability:0,subtypes:id==373,armor:0});
    }
    if id==261{return Some(ItemDefinition{max_stack:1,durability:384,subtypes:false,armor:0});}
    if id==368{return Some(ItemDefinition{max_stack:16,durability:0,subtypes:false,armor:0});}
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
    if (369..=382).contains(&id) {
        return 1.0;
    }
    if item(id).is_none() {
        return 1.0;
    }
    if matches!(id,256|269|273|277|284)&&matches!(block,88|110) {return crate::item10::item_tool_efficiency(id);}
    if matches!(id,257|270|274|278|285)&&(matches!(crate::block::material_of(block),crate::block::MAT_ROCK|crate::block::MAT_IRON)||matches!(block,27|28|66)) {return crate::item10::item_tool_efficiency(id);}
    if matches!(id,258|271|275|279|286)&&(crate::block::material_of(block)==crate::block::MAT_WOOD||matches!(block,43|44|86|91)) {return crate::item10::item_tool_efficiency(id);}



    if id>2257||(110..=122).contains(&id)||id>=372{return 1.0;}
    f32::from_bits(int(id as usize * ITEM_SIZE + 16 + block as usize * 5) as u32)
}
pub fn can_harvest(id: i32, block: u8) -> bool {
    harvestable(block) || if matches!(id,257|270|274|278|285)&&block>=110 {matches!(crate::block::material_of(block),crate::block::MAT_ROCK|crate::block::MAT_IRON)}else{item(id).is_some()&&id<=2257&&DATA[id as usize * ITEM_SIZE + 20 + block as usize * 5]!=0}
}
pub fn harvestable(block: u8) -> bool {
    if block>=110{return !matches!(block,112..=114|116..=118|121);}
    if block==89{return true;}
    DATA[2258 * ITEM_SIZE + block as usize * 5 + 4] != 0
}
pub fn hardness(block: u8) -> f32 {
    if block==49{return 50.0;}
    if block>=110{return [0.6,0.0,2.0,2.0,2.0,0.0,5.0,0.5,2.0,-1.0,-1.0,3.0,3.0].get((block-110) as usize).copied().unwrap_or(0.0);}
    f32::from_bits(int(2258 * ITEM_SIZE + block as usize * 5) as u32)
}
pub fn attack_damage(id: i32) -> i32 {
    if id>2257||id>=372&&id<2256||item(id).is_none() {
        1
    } else if crate::enchantment::is_sword(id) {
        match id {267=>6,268|283=>4,272=>5,276=>7,_=>1}
    } else {
        int(2258 * ITEM_SIZE + 256 * 5 + id as usize * 4)
    }
}

/// EnumAction in the original declaration order.
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
#[repr(i32)]
pub enum ItemUseAction {#[default] None=0,Eat=1,Drink=2,Block=3,Bow=4}
pub fn use_action(id:i32)->ItemUseAction {
    match id {373|335=>ItemUseAction::Drink,375=>ItemUseAction::Eat,260|282|297|319|320|322|349|350|357|360|363..=367=>ItemUseAction::Eat,
        267|268|272|276|283=>ItemUseAction::Block,261=>ItemUseAction::Bow,_=>crate::entity::item::Item::defaults(id).use_action(None)}
}
pub fn use_duration(id:i32)->i32 {match use_action(id){ItemUseAction::Eat|ItemUseAction::Drink=>32,ItemUseAction::Block|ItemUseAction::Bow=>72000,ItemUseAction::None=>0}}
/// ItemBow's release calculation; None means the shot is too weak to fire.
pub fn bow_charge(remaining:i32)->Option<f32>{
    let mut f=72000i32.wrapping_sub(remaining) as f32/20.0f32;
    f=(f*f+f*2.0f32)/3.0f32;
    if (f as f64)<0.1f64 {None}else{if f>1.0{f=1.0;}Some(f)}
}

/// Metadata overrides on the registered ItemBlock subclasses. Keep the full
/// signed ItemStack damage until World writes its metadata nibble.
pub fn placed_block_metadata(id: i32, damage: i32) -> i32 {
    match id {
        6 | 17 | 31 | 35 | 44 | 98 | 106 |111 => damage,
        18 => damage | 4,
        29 | 33 => 7,
        _ => crate::entity::item::Item::defaults(id).placed_metadata(damage),
    }
}

/// The fixed-block ItemSlab, ItemCloth and ItemLeaves icon/color overrides.
/// None means the existing item/block renderer supplies the property.
pub fn block_item_properties(id: i32, damage: i32, fancy: bool) -> Option<(i32, u32)> {
    Some(match id {
        111 => (76,2129968),
        31 | 106 => {
            let item=VineItem::new(id as u8,id==31);
            (item.icon(damage,|block,side,damage| {
                debug_assert_eq!(side,0);
                if block==106 {143} else {match damage {0=>55,2=>56,_=>39}}
            }),item.color(damage,|block,damage|if block==106 || damage!=0 {4764952} else {16777215}) as u32)
        },
        17 | 98 => {
            let item=LogItem::new(Some(id as u8));
            (item.icon(damage,|block,side,damage| {
                debug_assert_eq!(side,2);
                match block {17=>match damage {1=>116,2=>117,_=>20,},98=>match damage {1=>100,2=>101,_=>54,},_=>unreachable!(),}
            }).unwrap(),0xffffff,)
        }
        44 => (match damage {0=>5,1=>192,2=>4,3=>16,4=>7,5=>54,_=>6,}, 0xffffff,),
        35 => {
            let meta = !damage & 15;
            let icon = if meta==0 {64} else {let inverted=!(meta&15);113+((inverted&8)>>3)+(inverted&7)*16};
            (icon,0xffffff)
        }
        18 => (52 + if damage&3==1 {80} else {0} + if fancy {0} else {1},
            if damage&1==1 {6396257} else if damage&2==2 {8431445} else {4764952},),
        _ => return None,
    })
}

/// ItemLog stores the constructor's Block reference, rather than always using
/// Block.wood. The base ItemBlock continues to own its placement block.
#[derive(Clone, Copy, Debug)]
pub struct LogItem { pub texture_block: Option<u8>, }
impl LogItem {
    pub fn new(texture_block: Option<u8>) -> Self { Self {texture_block} }
    pub fn icon(&self, damage: i32, mut texture: impl FnMut(u8,i32,i32)->i32,) -> Result<i32,&'static str> {
        let block=self.texture_block.ok_or("NullPointerException")?;
        Ok(texture(block,2,damage))
    }
    pub fn placed_metadata(&self, damage: i32) -> i32 { damage }
}

/// ItemVine captures its placement Block and asks that same object for texture
/// and inventory color. The subtype constructor flag is independent of damage.
#[derive(Clone, Copy, Debug)]
pub struct VineItem { pub block: u8, pub subtypes: bool, }
impl VineItem {
    pub fn new(block: u8, subtypes: bool) -> Self { Self {block,subtypes} }
    pub fn icon(&self, damage: i32, mut texture: impl FnMut(u8,i32,i32)->i32) -> i32 {texture(self.block,0,damage)}
    /// 1.0 ItemColored.getColorFromDamage: Block.getRenderColor(damage).
    pub fn color(&self, damage: i32, mut render_color: impl FnMut(u8,i32)->i32) -> i32 {render_color(self.block,damage)}
    pub fn placed_metadata(&self, damage: i32) -> i32 {damage}
}
/// ItemSlab clamps its name index (but not its icon index); ItemCloth uses
/// BlockCloth.getBlockFromDye, including signed and oversized damage.
pub fn block_item_name(id: i32, damage: i32) -> Option<String> {
    match id {
        31=>Some(if (0..3).contains(&damage){format!("tile.tallgrass.{}",["shrub","grass","fern"][damage as usize])}else{"tile.tallgrass".into()}),
        44 => {
            let names=["stone","sand","wood","cobble","brick","smoothStoneBrick",];
            Some(format!("tile.stoneSlab.{}",names.get(damage as usize).unwrap_or(&names[0])))
        }
        35 => Some(format!("tile.cloth.{}",super::dye::DYE_COLOR_NAMES[(!damage&15) as usize])),
        _ => None,
    }
}
