//! BlockSign, ItemSign, TileEntitySign and GuiEditSign's shared editing state.
use crate::{World, block::mat, entity::{Entity, player::{ItemStack, PlayerEvent}}, nbt::{Compound, Tag}, world::tile::TileEntity};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sign {
    pub text: [String; 4],
    pub line_being_edited: i32,
    pub editable: bool,
}
impl Default for Sign {
    fn default() -> Self {
        Self { text: Default::default(), line_being_edited: -1, editable: true }
    }
}
impl Sign {
    pub fn from_nbt(n: &Compound) -> Self {
        Self {
            // Java counts UTF-16 code units, rather than Unicode scalar values.
            text: std::array::from_fn(|i| String::from_utf16_lossy(&n.string(&format!("Text{}", i+1)).encode_utf16().take(15).collect::<Vec<_>>())),
            line_being_edited: -1,
            editable: false,
        }
    }
    pub fn to_nbt(&self, x: i32, y: i32, z: i32) -> Compound {
        let mut n = Compound::new();
        n.put("id", Tag::String("Sign".into()));
        for (key, v) in [("x", x), ("y", y), ("z", z)] { n.put(key, Tag::Int(v)); }
        for i in 0..4 { n.put(&format!("Text{}", i+1), Tag::String(self.text[i].clone())); }
        n
    }
}

#[derive(Clone, Debug, Default)]
pub struct Editor {
    pub sign: Sign,
    pub edit_line: usize,
    pub update_counter: i32,
}
impl Editor {
    pub fn key_typed(&mut self, c: char, key: i32, allowed: bool) {
        if key == 200 { self.edit_line = self.edit_line.wrapping_sub(1) & 3; }
        if key == 208 || key == 28 { self.edit_line = (self.edit_line + 1) & 3; }
        let text = &mut self.sign.text[self.edit_line];
        if key == 14 && !text.is_empty() { text.pop(); }
        if allowed && text.encode_utf16().count() < 15 { text.push(c); }
    }
    pub fn blinking_line(&self) -> i32 {
        if self.update_counter / 6 % 2 == 0 { self.edit_line as i32 } else { -1 }
    }
}

pub fn neighbor(w: &mut World, id: u8, x: i32, y: i32, z: i32) {
    let meta = w.get_block_metadata(x,y,z);
    let support = if id == 63 { Some((x,y-1,z)) } else {
        match meta { 2 => Some((x,y,z+1)), 3 => Some((x,y,z-1)), 4 => Some((x+1,y,z)), 5 => Some((x-1,y,z)), _ => None }
    };
    if !support.is_some_and(|(x,y,z)| mat(w.get_block_material(x,y,z)).is_solid) {
        crate::world::behavior::drop_block_as_item(w,id,x,y,z,meta);
        w.set_block_with_notify(x,y,z,0);
    }
}

impl Entity {
    pub fn place_sign(&mut self, w: &mut World, stack: &mut ItemStack, x: i32, y: i32, z: i32, side: i32) -> bool {
        if side == 0 || !mat(w.get_block_material(x,y,z)).is_solid { return false; }
        let (x,y,z) = crate::entity::player::placement::offset(x,y,z,side);
        // ItemSign calls Block.canPlaceBlockAt, without ItemBlock's entity collision test.
        if !(0..128).contains(&y) { return false; }
        let existing=w.get_block_id(x,y,z);
        if existing!=0 && !mat(w.get_block_material(x,y,z)).ground_cover { return false; }
        let (id, meta) = if side == 1 {
            (63, crate::math::floor_double(((self.yaw + 180.0) * 16.0 / 360.0) as f64 + 0.5) & 15)
        } else { (68, side) };
        w.set_block_and_metadata_with_notify(x,y,z,id,meta);
        stack.count -= 1;
        if let Some(TileEntity::Sign(sign)) = w.get_block_tile_entity(x,y,z) {
            let sign = sign.clone();
            self.player_state_mut().unwrap().events.push(PlayerEvent::EditSign { position: (x,y,z), block: id, metadata: meta, sign });
        }
        true
    }
}
