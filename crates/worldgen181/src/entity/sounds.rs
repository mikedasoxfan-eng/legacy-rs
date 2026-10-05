use super::{Entity, EntityKind};
use crate::{world::events::step_sound, World};
impl Entity {
    pub fn sound_volume(&self) -> f32 {
        match self.kind {
            EntityKind::Cow | EntityKind::Wolf(..) | EntityKind::Squid(..) => 0.4,
            EntityKind::Ghast(..) => 10.0,
            EntityKind::Slime { .. } => 0.6,
            _ => 1.0,
        }
    }
    pub fn living_sound(&mut self) -> Option<&'static str> {
        Some(match &self.kind {
            EntityKind::Pig { .. } => "mob.pig",
            EntityKind::Cow => "mob.cow",
            EntityKind::Sheep { .. } => "mob.sheep",
            EntityKind::Chicken { .. } => "mob.chicken",
            EntityKind::Zombie | EntityKind::Enderman(..) => "mob.zombie",
            EntityKind::Skeleton => "mob.skeleton",
            EntityKind::Spider | EntityKind::CaveSpider | EntityKind::Silverfish { .. } => "mob.spider",
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
            EntityKind::Pig { .. } => {
                if death {
                    "mob.pigdeath"
                } else {
                    "mob.pig"
                }
            }
            EntityKind::Cow => "mob.cowhurt",
            EntityKind::Sheep { .. } => "mob.sheep",
            EntityKind::Chicken { .. } => "mob.chickenhurt",
            EntityKind::Zombie | EntityKind::Enderman(..) => {
                if death {
                    "mob.zombiedeath"
                } else {
                    "mob.zombiehurt"
                }
            }
            EntityKind::Skeleton => "mob.skeletonhurt",
            EntityKind::Spider | EntityKind::CaveSpider | EntityKind::Silverfish { .. } => {
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
            EntityKind::Slime { .. } => "mob.slime",
            _ => "random.hurt",
        }
    }
    pub fn play_step_sound(&self, w: &mut World, block: u8, volume: f32, pitch: f32) {
        let (name, v, p) = step_sound(block);
        w.play_sound_at_entity(self, name, v * volume, p * pitch);
    }
}
