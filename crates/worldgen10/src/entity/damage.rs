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
    /// EntityDamageSourceIndirect overrides getEntity, but *not* getSourceOfDamage.
    /// In this version both accessors return the owner, not the projectile.
    pub indirect: bool,
}
impl DamageSource {
    pub const fn new(kind: DamageKind) -> Self {
        Self {
            kind,
            attacker: None,
            indirect: false,
        }
    }
    pub const fn entity(kind: DamageKind, attacker: Option<i32>) -> Self {
        Self {
            kind,
            attacker,
            indirect: false,
        }
    }
    pub const fn indirect(kind: DamageKind, _projectile: i32, owner: Option<i32>) -> Self {
        Self {
            kind,
            attacker: owner,
            indirect: true,
        }
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
