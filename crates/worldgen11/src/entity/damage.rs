//! Beta 1.8.1 DamageSource and its two entity subclasses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DamageKind {
    InFire,
    OnFire,
    Lava,
    InWall,
    Drown,
    Starve,
    Cactus,
    Fall,
    OutOfWorld,
    Generic,
    Explosion,
    Magic,
    IndirectMagic,
    Mob,
    Player,
    Arrow,
    Fireball,
    Thrown,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DamageSource {
    pub kind: DamageKind,
    pub attacker: Option<i32>,
    /// Release 1.1 separates the direct projectile from its owning attacker.
    pub projectile: Option<i32>,
    pub indirect: bool,
}
impl DamageSource {
    pub const fn new(kind: DamageKind) -> Self {
        Self {
            kind,
            attacker: None,
            projectile: None,
            indirect: false,
        }
    }
    pub const fn entity(kind: DamageKind, attacker: Option<i32>) -> Self {
        Self {
            kind,
            attacker,
            projectile: None,
            indirect: false,
        }
    }
    pub const fn indirect(kind: DamageKind, projectile: i32, owner: Option<i32>) -> Self {
        Self {
            kind,
            attacker: owner,
            projectile: Some(projectile),
            indirect: true,
        }
    }
    pub fn source_of_damage(self) -> Option<i32> {
        if self.indirect { self.projectile } else { self.attacker }
    }
    pub fn name(self) -> &'static str {
        use DamageKind::*;
        match self.kind {
            InFire => "inFire",
            OnFire => "onFire",
            Lava => "lava",
            InWall => "inWall",
            Drown => "drown",
            Starve => "starve",
            Cactus => "cactus",
            Fall => "fall",
            OutOfWorld => "outOfWorld",
            Generic => "generic",
            Explosion => "explosion",
            Magic => "magic",
            IndirectMagic => "indirectMagic",
            Mob => "mob",
            Player => "player",
            Arrow => "arrow",
            Fireball => "fireball",
            Thrown => "thrown",
        }
    }
    pub fn bypasses_armor(self) -> bool {
        use DamageKind::*;
        matches!(
            self.kind,
            OnFire | InWall | Drown | Starve | Fall | OutOfWorld | Generic | Magic | IndirectMagic
        )
    }
    pub fn is_fire(self) -> bool {
        matches!(
            self.kind,
            DamageKind::InFire | DamageKind::OnFire | DamageKind::Lava | DamageKind::Fireball
        )
    }
    pub fn is_projectile(self) -> bool {
        matches!(
            self.kind,
            DamageKind::Arrow | DamageKind::Fireball | DamageKind::Thrown
        )
    }
    pub fn harms_creative(self) -> bool {
        self.kind == DamageKind::OutOfWorld
    }
    pub fn exhaustion(self) -> f32 {
        if self.bypasses_armor() {
            0.0
        } else {
            0.3
        }
    }
    pub fn is_entity(self) -> bool {
        matches!(
            self.kind,
            DamageKind::Mob
                | DamageKind::Player
                | DamageKind::Arrow
                | DamageKind::Fireball
                | DamageKind::Thrown
                | DamageKind::IndirectMagic
        )
    }
}

#[cfg(test)]
mod release11_tests {
 use super::*;use crate::entity::{Entity,EntityKind,Vec3D};
 #[test]
 fn indirect_source_and_skeleton_death_match_original_11_bytecode(){
  for line in include_str!("../fixtures/indirectdamage11.txt").lines(){let v:Vec<i64>=line.split_whitespace().map(|s|s.parse().unwrap()).collect();let mut w=crate::entity::verify181::world();w.multiplayer_world=v[3]!=0;let mut player=Entity::player(&mut w,"probe");player.pos=Vec3D::new(f64::from_bits(v[1] as u64),1000.0,f64::from_bits(v[2] as u64));let pid=player.entity_id;w.entities.insert(pid,player);let mut s=Entity::living(&mut w,"Skeleton").unwrap();s.set_position(0.0,64.0,0.0);s.rand.set_seed(42);let a=Entity::arrow(&mut w,0.0,64.0,0.0);let snow=Entity::arrow(&mut w,0.0,64.0,0.0);let ds=match v[0]{0=>DamageSource::indirect(DamageKind::Arrow,a.entity_id,Some(pid)),1=>DamageSource::indirect(DamageKind::Arrow,a.entity_id,Some(a.entity_id)),2=>DamageSource::entity(DamageKind::Player,Some(pid)),3=>DamageSource::indirect(DamageKind::Thrown,snow.entity_id,Some(pid)),_=>DamageSource::indirect(DamageKind::IndirectMagic,snow.entity_id,Some(pid))};let src=ds.source_of_damage();assert_eq!(if src==Some(a.entity_id){1}else if src==Some(snow.entity_id){2}else if src==Some(pid){3}else{0},v[4],"{line}");assert_eq!(ds.attacker==Some(pid),v[5]!=0,"{line}");w.rand.set_seed(42);s.on_living_death(&mut w,ds);assert_eq!(w.entities[&pid].player_state().unwrap().stat_log.iter().filter(|&&(id,_)|id==crate::entity::player::stat::SNIPE_SKELETON).map(|&(_,n)|n as i64).sum::<i64>(),v[6],"{line}");assert_eq!(s.rand.next_long(),v[7],"{line}");assert_eq!(w.rand.next_long(),v[8],"{line}");}
 }
 #[test]
 fn lethal_arrow_impact_delivers_original_11_sniper_callback(){let mut w=crate::entity::verify181::world();let mut p=Entity::player(&mut w,"probe");p.set_position(50.0,64.0,1.0);let pid=p.entity_id;w.entities.insert(pid,p);let mut s=Entity::living(&mut w,"Skeleton").unwrap();s.set_position(0.0,64.0,1.0);s.living.health=1;let sid=s.entity_id;assert!(w.entity_joined_world(s));let mut a=Entity::arrow(&mut w,0.0,64.5,0.0);a.motion=Vec3D::new(0.0,0.0,1.25);if let EntityKind::Arrow(state)=&mut a.kind{state.owner=Some(pid);}a.update_arrow(&mut w);assert!(w.entities[&sid].living.dying);assert!(a.dead);assert!(w.entities[&pid].player_state().unwrap().stat_log.contains(&(crate::entity::player::stat::SNIPE_SKELETON,1)));}
}
