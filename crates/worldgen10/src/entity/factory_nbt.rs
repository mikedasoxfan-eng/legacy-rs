//! EntityList's NBT exception boundary. Constructors are attempted before the
//! entity read; read exceptions escape the registry's reflection catch.
use crate::nbt::{Compound, Tag};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FactoryError {
    NullPointer,
    ClassCast,
    IndexOutOfBounds,
    ArrayIndexOutOfBounds,
}
impl FactoryError {
    pub fn class(self) -> &'static str {
        match self {
            Self::NullPointer => "NullPointerException",
            Self::ClassCast => "ClassCastException",
            Self::IndexOutOfBounds => "IndexOutOfBoundsException",
            Self::ArrayIndexOutOfBounds => "ArrayIndexOutOfBoundsException",
        }
    }
}
fn check(n: &Compound, key: &str, ty: u8) -> Result<(), FactoryError> {
    if n.get(key).is_some_and(|tag| tag.id() != ty) {
        Err(FactoryError::ClassCast)
    } else {
        Ok(())
    }
}
pub fn name(n: Option<&Compound>) -> Result<String, FactoryError> {
    let n = n.ok_or(FactoryError::NullPointer)?;
    check(n, "id", 8)?;
    Ok(n.string("id"))
}
fn list<'a>(n: &'a Compound, key: &str) -> Result<&'a [Tag], FactoryError> {
    check(n, key, 9)?;
    Ok(n.list(key))
}
fn compound<'a>(tag: &'a Tag) -> Result<&'a Compound, FactoryError> {
    if let Tag::Compound(n) = tag {
        Ok(n)
    } else {
        Err(FactoryError::ClassCast)
    }
}
fn stack(n: &Compound) -> Result<(), FactoryError> {
    check(n, "id", 2)?;
    check(n, "Count", 1)?;
    check(n, "Damage", 2)?;
    if !(0..32000).contains(&(n.short("id") as i32)) {
        return Err(FactoryError::ArrayIndexOutOfBounds);
    }
    Ok(())
}
pub fn validate(name: &str, n: &Compound) -> Result<(), FactoryError> {
    // All three getTagList casts happen before any tagAt call.
    let pos = list(n, "Pos")?;
    let motion = list(n, "Motion")?;
    let rotation = list(n, "Rotation")?;
    for (values, count, ty) in [(motion, 3, 6), (pos, 3, 6), (rotation, 2, 5)] {
        for index in 0..count {
            let tag = values.get(index).ok_or(FactoryError::IndexOutOfBounds)?;
            if tag.id() != ty {
                return Err(FactoryError::ClassCast);
            }
        }
    }
    for (key, ty) in [
        ("FallDistance", 5),
        ("Fire", 2),
        ("Air", 2),
        ("OnGround", 1),
    ] {
        check(n, key, ty)?;
    }
    // EntityList ids 48..=120 are EntityLiving subclasses; EnderCrystal (200) is not.
    let living = super::registry::by_name(name).is_some_and(|e| (48..=120).contains(&e.id));
    if living {
        for key in ["Health", "HurtTime", "DeathTime", "AttackTime"] {
            check(n, key, 2)?;
        }
        if n.has("ActiveEffects") {
            for effect in list(n, "ActiveEffects")? {
                let effect = compound(effect)?;
                check(effect, "Id", 1)?;
                check(effect, "Amplifier", 1)?;
                check(effect, "Duration", 3)?;
            }
        }
    }
    let fields: &[(&str, u8)] = match name {
        "XPOrb" => &[("Health", 2), ("Age", 2), ("Value", 2)],
        "Enderman" => &[("carried", 2), ("carriedData", 2)],
        "PigZombie" => &[("Anger", 2)],
        // 1.0 EntityAnimal reads Age and InLove before the subclass fields.
        "Wolf" => &[("Age", 3), ("InLove", 3), ("Angry", 1), ("Sitting", 1), ("Owner", 8)],
        "Pig" => &[("Age", 3), ("InLove", 3), ("Saddle", 1)],
        "Sheep" => &[("Age", 3), ("InLove", 3), ("Sheared", 1), ("Color", 1)],
        "Cow" | "Chicken" | "MushroomCow" => &[("Age", 3), ("InLove", 3)],
        "Villager" => &[("Profession", 3)],
        "LavaSlime" => &[("Size", 3)],
        "Fireball" | "SmallFireball" => &[
            ("xTile", 2),
            ("yTile", 2),
            ("zTile", 2),
            ("inTile", 1),
            ("inGround", 1),
        ],
        "Creeper" => &[("powered", 1)],
        "Slime" => &[("Size", 3)],
        "PrimedTnt" => &[("Fuse", 1)],
        "FallingSand" => &[("Tile", 1)],
        "Painting" => &[
            ("Dir", 1),
            ("TileX", 3),
            ("TileY", 3),
            ("TileZ", 3),
            ("Motive", 8),
        ],
        "Arrow" => &[
            ("xTile", 2),
            ("yTile", 2),
            ("zTile", 2),
            ("inTile", 1),
            ("inData", 1),
            ("shake", 1),
            ("inGround", 1),
            ("player", 1),
        ],
        "Snowball" | "ThrownEnderpearl" => &[
            ("xTile", 2),
            ("yTile", 2),
            ("zTile", 2),
            ("inTile", 1),
            ("shake", 1),
            ("inGround", 1),
        ],
        "Item" => &[("Health", 2), ("Age", 2), ("Item", 10)],
        "Minecart" => &[("Type", 3)],
        _ => &[],
    };
    for &(key, ty) in fields {
        check(n, key, ty)?;
    }
    if name == "Item" {
        if let Some(item) = n.compound("Item") {
            stack(item)?;
        }
    }
    if name == "Minecart" {
        if n.int("Type") == 2 {
            for (key, ty) in [("PushX", 6), ("PushZ", 6), ("Fuel", 2)] {
                check(n, key, ty)?;
            }
        }
        if n.int("Type") == 1 {
            for item in list(n, "Items")? {
                let item = compound(item)?;
                check(item, "Slot", 1)?;
                if (item.byte("Slot") as u8) < 27 {
                    stack(item)?;
                }
            }
        }
    }
    Ok(())
}
