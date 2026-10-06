//! ItemMap, MapData and singleplayer MapStorage for Beta 1.8.1.
use crate::{
    World,
    block::{mat, material_of},
    entity::{Entity, player::ItemStack},
    math::floor_double as floor,
    nbt::{Compound, Tag},
};
use std::{collections::HashMap, io, path::Path};
pub const PALETTE: [u32; 14] = [
    0, 8368696, 16247203, 10987431, 16711680, 10526975, 10987431, 31744, 16777215, 10791096,
    12020271, 7368816, 4210943, 6837042,
];
/// Material.materialMapColor by material id; b1.8.1 adds the vine material at index 10.
pub const MATERIAL_COLORS: [u8; 29] = [
    0, 1, 10, 13, 11, 6, 12, 4, 7, 7, 7, 3, 3, 0, 2, 0, 0, 4, 7, 5, 8, 8, 7, 9, 7, 0, 0, 3, 11,
];
pub fn block_color(id: u8) -> u8 {
    MATERIAL_COLORS[material_of(id) as usize]
}
#[derive(Clone, Debug)]
pub struct MapInfo {
    pub player: i32,
    pub min: [i32; 128],
    pub max: [i32; 128],
}
#[derive(Clone, Debug)]
pub struct MapData {
    pub center: [i32; 2],
    pub dimension: i8,
    pub scale: i8,
    pub colors: Vec<u8>,
    pub counter: i32,
    pub players: Vec<MapInfo>,
    pub icons: Vec<[i8; 4]>,
    pub dirty: bool,
}
impl Default for MapData {
    fn default() -> Self {
        Self {
            center: [0; 2],
            dimension: 0,
            scale: 0,
            colors: vec![0; 16384],
            counter: 0,
            players: Vec::new(),
            icons: Vec::new(),
            dirty: false,
        }
    }
}
impl MapData {
    pub fn from_nbt(n: &Compound) -> io::Result<Self> {
        let mut data = Self {
            center: [n.int("xCenter"), n.int("zCenter")],
            dimension: n.byte("dimension"),
            scale: n.byte("scale").clamp(0, 4),
            ..Default::default()
        };
        let (width, height) = (n.short("width") as i32, n.short("height") as i32);
        let colors = n.bytes("colors").unwrap_or(&[]);
        if width == 128 && height == 128 {
            data.colors = colors.to_vec();
        } else {
            let (dx, dy) = ((128 - width) / 2, (128 - height) / 2);
            for y in 0..height {
                let yy = y + dy;
                if yy >= 0 || yy < 128 {
                    for x in 0..width {
                        let xx = x + dx;
                        if xx >= 0 || xx < 128 {
                            // Preserve the original OR bounds tests: invalid legacy sizes fail loading.
                            let source = colors.get((x + y * width) as usize).ok_or_else(|| {
                                io::Error::new(io::ErrorKind::InvalidData, "map source bounds")
                            })?;
                            let dest =
                                data.colors
                                    .get_mut((xx + yy * 128) as usize)
                                    .ok_or_else(|| {
                                        io::Error::new(
                                            io::ErrorKind::InvalidData,
                                            "map destination bounds",
                                        )
                                    })?;
                            *dest = *source;
                        }
                    }
                }
            }
        }
        Ok(data)
    }
    pub fn to_nbt(&self) -> Compound {
        let mut n = Compound::new();
        n.put("dimension", Tag::Byte(self.dimension));
        n.put("xCenter", Tag::Int(self.center[0]));
        n.put("zCenter", Tag::Int(self.center[1]));
        n.put("scale", Tag::Byte(self.scale));
        n.put("width", Tag::Short(128));
        n.put("height", Tag::Short(128));
        n.put("colors", Tag::ByteArray(self.colors.clone()));
        n
    }
    pub fn mark_column(&mut self, x: usize, min: i32, max: i32) {
        self.dirty = true;
        for p in &mut self.players {
            if p.min[x] < 0 || p.min[x] > min {
                p.min[x] = min;
            }
            if p.max[x] < 0 || p.max[x] < max {
                p.max[x] = max;
            }
        }
    }
    pub fn update_markers(&mut self, w: &World, player: &Entity, stack: &ItemStack) {
        if !self.players.iter().any(|p| p.player == player.entity_id) {
            self.players.push(MapInfo {
                player: player.entity_id,
                min: [0; 128],
                max: [127; 128],
            });
        }
        self.icons.clear();
        let mut i = 0;
        while i < self.players.len() {
            let id = self.players[i].player;
            let p = if id == player.entity_id {
                Some(player)
            } else {
                w.entities.get(&id)
            };
            let alive = p.is_some_and(|p| {
                !p.dead
                    && p.player_state().is_some_and(|s| {
                        s.inventory
                            .main
                            .iter()
                            .chain(s.inventory.armor.iter())
                            .flatten()
                            .any(|s| s.id == stack.id && s.damage == stack.damage)
                    })
            });
            if alive {
                let p = p.unwrap();
                let x = (p.pos.x - self.center[0] as f64) as f32 / (1i32 << self.scale) as f32;
                let z = (p.pos.z - self.center[1] as f64) as f32 / (1i32 << self.scale) as f32;
                if x >= -64.0 && z >= -64.0 && x <= 64.0 && z <= 64.0 {
                    let mut rotation = ((player.yaw * 16.0 / 360.0) as f64 + 0.5) as i32 as i8;
                    if self.dimension < 0 {
                        let t = self.counter / 10;
                        rotation = (t
                            .wrapping_mul(t)
                            .wrapping_mul(34187121)
                            .wrapping_add(t.wrapping_mul(121))
                            >> 15
                            & 15) as i8;
                    }
                    if p.player_state().unwrap().dimension == self.dimension as i32 {
                        self.icons.push([
                            0,
                            ((x * 2.0) as f64 + 0.5) as i32 as i8,
                            ((z * 2.0) as f64 + 0.5) as i32 as i8,
                            rotation,
                        ]);
                    }
                }
            } else {
                self.players.remove(i);
            }
            // ArrayList.remove followed by the for-loop increment skips its successor.
            i += 1;
        }
    }
    pub fn update_terrain(&mut self, w: &mut World, player: &Entity) {
        let dimension = if w.dimension.is_hell_world() { -1 } else { 0 };
        if dimension != self.dimension {
            return;
        }
        let factor = 1i32 << self.scale;
        let [cx, cz] = self.center;
        let px = floor(player.pos.x - cx as f64) / factor + 64;
        let pz = floor(player.pos.z - cz as f64) / factor + 64;
        let mut radius = 128 / factor;
        if w.dimension.has_no_sky() {
            radius /= 2;
        }
        self.counter = self.counter.wrapping_add(1);
        for x in px - radius + 1..px + radius {
            if x & 15 != self.counter & 15 {
                continue;
            }
            let mut min = 255;
            let mut max = 0;
            let mut previous_height = 0.0;
            for z in pz - radius - 1..pz + radius {
                if x < 0 || z < -1 || x >= 128 || z >= 128 {
                    continue;
                }
                let dx = x - px;
                let dz = z - pz;
                let edge = dx * dx + dz * dz > (radius - 2) * (radius - 2);
                let wx = (cx / factor + x - 64) * factor;
                let wz = (cz / factor + z - 64) * factor;
                w.provide_chunk(wx >> 4, wz >> 4);
                let chunk = w.chunk(wx >> 4, wz >> 4);
                let bx = wx & 15;
                let bz = wz & 15;
                let mut counts = [0i32; 256];
                let mut depth = 0;
                let mut height = 0.0;
                if w.dimension.has_no_sky() {
                    let n = wx.wrapping_add(wz.wrapping_mul(231871));
                    let n = n
                        .wrapping_mul(n)
                        .wrapping_mul(31287121)
                        .wrapping_add(n.wrapping_mul(11));
                    counts[if n >> 20 & 1 == 0 { 3 } else { 1 }] += 10;
                    height = 100.0;
                } else {
                    for sx in 0..factor {
                        for sz in 0..factor {
                            let mut y =
                                chunk.height_map[((bz + sz) << 4 | (bx + sx)) as usize] as i32 + 1;
                            let mut block = 0;
                            if y > 1 {
                                loop {
                                    block = if (0..128).contains(&(y-1)) {chunk.block(bx+sx,y-1,bz+sz)} else {0};
                                    let visible = block != 0 && (y <= 0 || block_color(block) != 0);
                                    if !visible {
                                        y-=1;
                                        block=if (0..128).contains(&(y-1)) {chunk.block(bx+sx,y-1,bz+sz)} else {0};
                                    }
                                    if y<=0 || visible {break;}
                                }
                                if y>0 && block!=0 && mat(material_of(block)).is_liquid {
                                    let mut yy=y-1;
                                    loop {
                                        let b=if (0..128).contains(&yy) {chunk.block(bx+sx,yy,bz+sz)} else {0};
                                        yy-=1;
                                        depth+=1;
                                        if yy<=0 || b==0 || !mat(material_of(b)).is_liquid {break;}
                                    }
                                }
                            }
                            height += y as f64 / (factor * factor) as f64;
                            counts[block as usize] += 1;
                        }
                    }
                }
                depth /= factor * factor;
                let mut best = 0;
                let mut selected = 0;
                for (block, &count) in counts.iter().enumerate() {
                    if count > best {
                        selected = block as u8;
                        best = count;
                    }
                }
                let mut slope = (height - previous_height) * 4.0 / (factor + 4) as f64
                    + ((x + z & 1) as f64 - 0.5) * 0.4;
                let mut shade = 1;
                if slope > 0.6 {
                    shade = 2;
                }
                if slope < -0.6 {
                    shade = 0;
                }
                let color = if selected > 0 {
                    block_color(selected)
                } else {
                    0
                };
                if color == 12 {
                    slope = depth as f64 * 0.1 + (x + z & 1) as f64 * 0.2;
                    shade = 1;
                    if slope < 0.5 {
                        shade = 2;
                    }
                    if slope > 0.9 {
                        shade = 0;
                    }
                }
                previous_height = height;
                if z >= 0 && dx * dx + dz * dz < radius * radius && (!edge || (x + z & 1) != 0) {
                    let index = (x + z * 128) as usize;
                    let value = color * 4 + shade;
                    if self.colors[index] != value {
                        min = min.min(z);
                        max = max.max(z);
                        self.colors[index] = value;
                    }
                }
            }
            if min <= max {
                self.mark_column(x as usize, min, max);
            }
        }
    }
}
#[derive(Default)]
pub struct MapStorage {
    pub data: HashMap<i32, MapData>,
    pub idcounts: Compound,
    pub counts_loaded: bool,
}
impl MapStorage {
    pub fn load(&mut self, dir: Option<&Path>, id: i32) -> bool {
        if self.data.contains_key(&id) {
            return true;
        }
        let Some(dir) = dir else { return false };
        let read = || -> io::Result<MapData> {
            let bytes = std::fs::read(dir.join("data").join(format!("map_{id}.dat")))?;
            let root = crate::nbt::read_gzip(&bytes)?;
            MapData::from_nbt(root.compound("data").unwrap_or(&Compound::default()))
        };
        if let Ok(data) = read() {
            self.data.insert(id, data);
            true
        } else {
            false
        }
    }
    pub fn save(&mut self, dir: &Path) -> io::Result<()> {
        std::fs::create_dir_all(dir.join("data"))?;
        for (id, map) in &mut self.data {
            if map.dirty {
                let mut root = Compound::new();
                root.put("data", Tag::Compound(map.to_nbt()));
                std::fs::write(
                    dir.join("data").join(format!("map_{id}.dat")),
                    crate::nbt::write_gzip(&root)?,
                )?;
                map.dirty = false;
            }
        }
        Ok(())
    }
}
impl World {
    pub fn load_map_counts(&mut self) {
        if self.maps.counts_loaded {
            return;
        }
        self.maps.counts_loaded = true;
        if let Some(s) = &self.storage {
            if let Ok(bytes) = std::fs::read(s.dir.join("data/idcounts.dat")) {
                if let Ok(mut n) = crate::nbt::read_root(&mut &bytes[..]) {
                    n.0.retain(|(_, v)| matches!(v, Tag::Short(_)));
                    if n.has("map") {
                        self.map_id = n.short("map") as i32;
                    }
                    self.maps.idcounts = n;
                }
            }
        }
    }
    pub fn save_map_count(&mut self) {
        self.maps
            .idcounts
            .put("map", Tag::Short(self.map_id as i16));
        if let Some(s) = &self.storage {
            let save = || -> io::Result<()> {
                std::fs::create_dir_all(s.dir.join("data"))?;
                let mut bytes = Vec::new();
                crate::nbt::write_root(&mut bytes, &self.maps.idcounts)?;
                std::fs::write(s.dir.join("data/idcounts.dat"), bytes)
            };
            if let Err(e) = save() {
                eprintln!("saving map ids: {e}");
            }
        }
    }
    pub fn create_map(&mut self, stack: &mut ItemStack, player: &Entity) {
        self.try_create_map(stack, Some(player)).expect("ItemMap.onCreated");
    }
    pub fn try_create_map(&mut self, stack: &mut ItemStack, player: Option<&Entity>) -> Result<(), &'static str> {
        self.load_map_counts();
        self.map_id = (self.map_id as i16).wrapping_add(1) as i32;
        stack.damage = self.map_id;
        // setItemData precedes the first player dereference in vanilla.
        self.maps.data.insert(self.map_id, MapData::default());
        self.save_map_count();
        let player = player.ok_or("NullPointerException")?;
        self.map_created(self.map_id, player);
        Ok(())
    }
    /// ItemMap.func_28013_a: unlike the stack lookup, a missing static lookup
    /// allocates an entirely default MapData, with no spawn/scale/dirty setup.
    /// Return its storage key, which need not equal the requested short ID.
    pub fn get_map_by_id(&mut self, id: i16) -> i32 {
        self.load_map_counts();
        let id = id as i32;
        if self.maps.load(self.storage.as_ref().map(|s| s.dir.as_path()), id) {
            return id;
        }
        self.map_id = (self.map_id as i16).wrapping_add(1) as i32;
        self.maps.data.insert(self.map_id, MapData::default());
        self.save_map_count();
        self.map_id
    }
    /// ItemMap.onUpdate, including the client guard and non-player entities.
    /// The inventory tick delegates here; marker updates remain player-only.
    pub fn update_map_item(&mut self, stack: &mut ItemStack, entity: &Entity, selected: bool) {
        self.try_update_map_item(stack, Some(entity), selected).expect("ItemMap.onUpdate");
    }
    pub fn try_update_map_item(&mut self, stack: &mut ItemStack, entity: Option<&Entity>, selected: bool) -> Result<(), &'static str> {
        if self.multiplayer_world { return Ok(()); }
        self.get_map(stack);
        self.update_loaded_map_item(stack, entity, selected)
    }
    /// Shared callback tail after get_map has published the stack's damage.
    fn update_loaded_map_item(&mut self, stack: &ItemStack, entity: Option<&Entity>, selected: bool) -> Result<(), &'static str> {
        let mut map = self.maps.data.remove(&stack.damage).unwrap();
        if let Some(entity) = entity.filter(|e| e.player_state().is_some()) { map.update_markers(self, entity, stack); }
        let result = if selected {
            if let Some(entity) = entity { map.update_terrain(self, entity); Ok(()) }
            else if map.dimension as i32 == if self.dimension.is_hell_world() {-1} else {0} { Err("NullPointerException") }
            else { Ok(()) }
        } else { Ok(()) };
        self.maps.data.insert(stack.damage, map);
        result
    }
    pub fn map_created(&mut self, id: i32, player: &Entity) {
        self.maps.data.insert(
            id,
            MapData {
                center: [floor(player.pos.x), floor(player.pos.z)],
                scale: 3,
                dimension: if self.dimension.is_hell_world() {
                    -1
                } else {
                    0
                },
                dirty: true,
                ..Default::default()
            },
        );
    }
    pub fn get_map(&mut self, stack: &mut ItemStack) {
        self.load_map_counts();
        if self
            .maps
            .load(self.storage.as_ref().map(|s| s.dir.as_path()), stack.damage)
        {
            return;
        }
        self.map_id = (self.map_id as i16).wrapping_add(1) as i32;
        stack.damage = self.map_id;
        self.maps.data.insert(
            stack.damage,
            MapData {
                center: [self.spawn.0, self.spawn.2],
                scale: 3,
                dimension: if self.dimension.is_hell_world() {
                    -1
                } else {
                    0
                },
                dirty: true,
                ..Default::default()
            },
        );
        self.save_map_count();
    }
}
impl Entity {
    pub fn update_maps(&mut self, w: &mut World) {
        if w.multiplayer_world { return; }
        for slot in 0..36 {
            let Some(mut stack) = self
                .player_state()
                .and_then(|s| s.inventory.main[slot])
                .filter(|s| s.id == 358)
            else {
                continue;
            };
            // Java's callback mutates the inventory's shared ItemStack before
            // update_markers examines that inventory. Publish the same damage.
            w.get_map(&mut stack);
            self.player_state_mut().unwrap().inventory.main[slot] = Some(stack);
            let selected = self.player_state().unwrap().inventory.current_item as usize == slot;
            w.update_loaded_map_item(&stack, Some(self), selected).expect("ItemMap.onUpdate");
            self.player_state_mut().unwrap().inventory.main[slot] = Some(stack);
        }
    }
}
