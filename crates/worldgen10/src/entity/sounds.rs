use super::{Entity, EntityKind};
use crate::{world::events::step_sound, World};
impl Entity {
    pub fn sound_volume(&self) -> f32 {
        match self.kind {
            EntityKind::Cow | EntityKind::Mooshroom
            | EntityKind::Wolf(..) | EntityKind::Squid(..) => 0.4,
            EntityKind::Ghast(..) => 10.0,
            EntityKind::Slime { .. } | EntityKind::MagmaCube { .. } => match self.kind {
                EntityKind::Slime { size, .. } | EntityKind::MagmaCube { size, .. } => { 0.4 * size as f32
                }
                _ => unreachable!(),
            },
            _ => 1.0,
        }
    }
    pub fn living_sound(&mut self) -> Option<&'static str> {
        Some(match &self.kind {
            EntityKind::Blaze(..) => "mob.blaze.breathe",
            EntityKind::Snowman => "none",
            EntityKind::Villager { .. } => "mob.villager.default",
            EntityKind::Pig { .. } => "mob.pig",
            EntityKind::Cow | EntityKind::Mooshroom => "mob.cow",
            EntityKind::Sheep { .. } => "mob.sheep",
            EntityKind::Chicken { .. } => "mob.chicken",
            EntityKind::Enderman(..) => "mob.endermen.idle",
            EntityKind::Zombie => "mob.zombie",
            EntityKind::Skeleton => "mob.skeleton",
            EntityKind::Spider | EntityKind::CaveSpider => "mob.spider", EntityKind::Silverfish { .. } => "mob.silverfish.say",
            EntityKind::PigZombie { .. } => "mob.zombiepig.zpig",
            EntityKind::Ghast(..) => "mob.ghast.moan",
            EntityKind::Wolf(s) => {
                if s.flags & 2 != 0 {
                    "mob.wolf.growl"
                } else if self.rand.next_int(3) == 0 {
                    if s.flags & 4 != 0 && s.watched_health < 10 {
                        "mob.wolf.whine"
                    } else {
                        "mob.wolf.panting"
                    }
                } else {
                    "mob.wolf.bark"
                }
            }
            _ => return None,
        })
    }
    pub fn damage_sound(&self, death: bool) -> &'static str {
        match self.kind {
            EntityKind::Blaze(..) => {
                if death {
                    "mob.blaze.death"
                } else {
                    "mob.blaze.hit"
                }
            }
            EntityKind::Snowman => "none",
            EntityKind::Villager { .. } => {
                if death {
                    "mob.villager.defaultdeath"
                } else {
                    "mob.villager.defaulthurt"
                }
            }
            EntityKind::Pig { .. } => {
                if death {
                    "mob.pigdeath"
                } else {
                    "mob.pig"
                }
            }
            EntityKind::Cow | EntityKind::Mooshroom => "mob.cowhurt",
            EntityKind::Sheep { .. } => "mob.sheep",
            EntityKind::Chicken { .. } => "mob.chickenhurt",
            EntityKind::Enderman(..) => {
                if death {
                    "mob.endermen.death"
                } else {
                    "mob.endermen.hit"
                }
            }
            EntityKind::Silverfish { .. } => {
                if death {
                    "mob.silverfish.kill"
                } else {
                    "mob.silverfish.hit"
                }
            }
            EntityKind::Zombie => {
                if death {
                    "mob.zombiedeath"
                } else {
                    "mob.zombiehurt"
                }
            }
            EntityKind::Skeleton => "mob.skeletonhurt",
            EntityKind::Spider | EntityKind::CaveSpider => {
                if death {
                    "mob.spiderdeath"
                } else {
                    "mob.spider"
                }
            }
            EntityKind::Creeper { .. } => {
                if death {
                    "mob.creeperdeath"
                } else {
                    "mob.creeper"
                }
            }
            EntityKind::PigZombie { .. } => {
                if death {
                    "mob.zombiepig.zpigdeath"
                } else {
                    "mob.zombiepig.zpighurt"
                }
            }
            EntityKind::Wolf(..) => {
                if death {
                    "mob.wolf.death"
                } else {
                    "mob.wolf.hurt"
                }
            }
            EntityKind::Ghast(..) => {
                if death {
                    "mob.ghast.death"
                } else {
                    "mob.ghast.scream"
                }
            }
            EntityKind::Slime { .. } | EntityKind::MagmaCube { .. } => "mob.slime",
            _ => "damage.hurtflesh",
        }
    }
    pub fn play_step_sound(&self, w: &mut World, block: u8, volume: f32, pitch: f32) {
        let (name, v, p) = step_sound(block);
        w.play_sound_at_entity(self, name, v * volume, p * pitch);
    }
}
