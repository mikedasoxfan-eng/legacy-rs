//! The exact Beta 1.8.1 EntityList registry, including its abstract-class entries.
use super::EntityKind;
#[derive(Clone,Copy,Debug)]
pub struct Entry {pub class: &'static str, pub name: &'static str, pub id: i32}
pub const ENTRIES: &[Entry] = &[
    Entry {class:"EntityItem",name:"Item",id:1},
    Entry {class:"EntityXPOrb",name:"XPOrb",id:2},
    Entry {class:"EntityArrow",name:"Arrow",id:10},
    Entry {class:"EntitySnowball",name:"Snowball",id:11},
    Entry {class:"EntityPainting",name:"Painting",id:9},
    Entry {class:"EntityLiving",name:"Mob",id:48},
    Entry {class:"EntityMob",name:"Monster",id:49},
    Entry {class:"EntityCreeper",name:"Creeper",id:50},
    Entry {class:"EntitySkeleton",name:"Skeleton",id:51},
    Entry {class:"EntitySpider",name:"Spider",id:52},
    Entry {class:"EntityGiantZombie",name:"Giant",id:53},
    Entry {class:"EntityZombie",name:"Zombie",id:54},
    Entry {class:"EntitySlime",name:"Slime",id:55},
    Entry {class:"EntityGhast",name:"Ghast",id:56},
    Entry {class:"EntityPigZombie",name:"PigZombie",id:57},
    Entry {class:"EntityEnderman",name:"Enderman",id:58},
    Entry {class:"EntityCaveSpider",name:"CaveSpider",id:59},
    Entry {class:"EntitySilverfish",name:"Silverfish",id:60},
    Entry {class:"EntityPig",name:"Pig",id:90},
    Entry {class:"EntitySheep",name:"Sheep",id:91},
    Entry {class:"EntityCow",name:"Cow",id:92},
    Entry {class:"EntityChicken",name:"Chicken",id:93},
    Entry {class:"EntitySquid",name:"Squid",id:94},
    Entry {class:"EntityWolf",name:"Wolf",id:95},
    Entry {class:"EntityTNTPrimed",name:"PrimedTnt",id:20},
    Entry {class:"EntityFallingSand",name:"FallingSand",id:21},
    Entry {class:"EntityMinecart",name:"Minecart",id:40},
    Entry {class:"EntityBoat",name:"Boat",id:41},
];
pub fn by_id(id:i32)->Option<&'static Entry> {ENTRIES.iter().find(|e|e.id==id)}
pub fn by_name(name:&str)->Option<&'static Entry> {ENTRIES.iter().find(|e|e.name==name)}
pub fn by_kind(kind:&EntityKind)->Option<&'static Entry> {by_name(kind.name())}
