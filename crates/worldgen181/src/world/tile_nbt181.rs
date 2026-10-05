//! Tile-local NBT casts. Missing keys default; wrong tag classes fail like Java.
//! Keep ItemStack implementation with its owner; validate the delegated load here.
use crate::nbt::{Compound, Tag};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileError181 {
    ClassCast,
    ArrayIndexOutOfBounds,
    NullPointer,
    MissingMapping,
}

/// Fields and methods of the original TileEntity superclass, also used by live
/// TileObjects so retained objects keep their independent caches and coordinates.
#[derive(Clone, Debug)]
pub struct TileBase181 {
    pub position: (i32, i32, i32),
    pub cached_metadata: i32,
    pub cached_block: Option<u8>,
    pub invalid: bool,
}
impl Default for TileBase181 {
    fn default() -> Self {
        Self {
            position: (0, 0, 0),
            cached_metadata: -1,
            cached_block: None,
            invalid: false,
        }
    }
}
impl TileBase181 {
    pub fn read_from_nbt(&mut self, n: &Compound) -> Result<(), TileError181> {
        read_position(&mut self.position, n)
    }
    pub fn write_to_nbt(
        &self,
        n: &mut Compound,
        mapping: Option<&str>,
    ) -> Result<(), TileError181> {
        let id = mapping.ok_or(TileError181::MissingMapping)?;
        n.put("id", Tag::String(id.into()));
        for (key, value) in [
            ("x", self.position.0),
            ("y", self.position.1),
            ("z", self.position.2),
        ] {
            n.put(key, Tag::Int(value));
        }
        Ok(())
    }
    pub fn metadata(&mut self, world: Option<&mut crate::World>) -> Result<i32, TileError181> {
        if self.cached_metadata == -1 {
            let w = world.ok_or(TileError181::NullPointer)?;
            self.cached_metadata =
                w.get_block_metadata(self.position.0, self.position.1, self.position.2);
        }
        Ok(self.cached_metadata)
    }
    pub fn block(&mut self, world: Option<&mut crate::World>) -> Result<Option<u8>, TileError181> {
        if self.cached_block.is_none() {
            let w = world.ok_or(TileError181::NullPointer)?;
            let id = w.get_block_id(self.position.0, self.position.1, self.position.2);
            self.cached_block = (id != 0
                && crate::entity::player::registry::item(id as i32).is_some())
            .then_some(id);
        }
        Ok(self.cached_block)
    }
    pub fn inventory_changed(&mut self, world: Option<&mut crate::World>) {
        if let Some(w) = world {
            let (x, y, z) = self.position;
            self.cached_metadata = w.get_block_metadata(x, y, z);
            if w.block_exists(x, y, z) {
                w.touch(x >> 4, z >> 4);
            }
        }
    }
    pub fn distance(&self, x: f64, y: f64, z: f64) -> f64 {
        let dx = self.position.0 as f64 + 0.5 - x;
        let dy = self.position.1 as f64 + 0.5 - y;
        let dz = self.position.2 as f64 + 0.5 - z;
        dx * dx + dy * dy + dz * dz
    }
    pub fn invalidate(&mut self) {
        self.invalid = true;
    }
    pub fn validate(&mut self) {
        self.invalid = false;
    }
    pub fn reset_cache(&mut self) {
        self.cached_block = None;
        self.cached_metadata = -1;
    }
    pub fn update_entity(&mut self) {}
    pub fn client_event(&mut self, _event: i32, _value: i32) {}
}

macro_rules! getter {
    ($name:ident, $variant:ident, $ty:ty, $default:expr) => {
        pub(crate) fn $name(n: &Compound, key: &str) -> Result<$ty, TileError181> {
            match n.get(key) {
                None => Ok($default),
                Some(Tag::$variant(v)) => Ok(*v),
                _ => Err(TileError181::ClassCast),
            }
        }
    };
}
getter!(byte, Byte, i8, 0);
getter!(short, Short, i16, 0);
getter!(int, Int, i32, 0);
getter!(float, Float, f32, 0.0);
pub(crate) fn string<'a>(n: &'a Compound, key: &str) -> Result<&'a str, TileError181> {
    match n.get(key) {
        None => Ok(""),
        Some(Tag::String(v)) => Ok(v),
        _ => Err(TileError181::ClassCast),
    }
}
pub(crate) fn list<'a>(n: &'a Compound, key: &str) -> Result<&'a [Tag], TileError181> {
    match n.get(key) {
        None => Ok(&[]),
        Some(Tag::List(_, v)) => Ok(v),
        _ => Err(TileError181::ClassCast),
    }
}
pub(crate) fn compound(t: &Tag) -> Result<&Compound, TileError181> {
    match t {
        Tag::Compound(n) => Ok(n),
        _ => Err(TileError181::ClassCast),
    }
}
pub(crate) fn item(n: &Compound) -> Result<Option<crate::container::ItemStack>, TileError181> {
    let id = short(n, "id")?;
    byte(n, "Count")?;
    short(n, "Damage")?;
    // Original func_35864_a reads all three fields before indexing itemsList.
    if !(0..32000).contains(&(id as i32)) {
        return Err(TileError181::ArrayIndexOutOfBounds);
    }
    Ok(crate::container::ItemStack::try_from_nbt181(n))
}

pub(crate) fn read_position(
    position: &mut (i32, i32, i32),
    n: &Compound,
) -> Result<(), TileError181> {
    position.0 = int(n, "x")?;
    position.1 = int(n, "y")?;
    position.2 = int(n, "z")?;
    Ok(())
}
