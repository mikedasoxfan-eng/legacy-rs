use super::{
    projectile::{ArrowState, FireballState},
    Entity, EntityKind, Vec3D,
};
use crate::{
    nbt::{Compound, Tag},
    world::tile::ItemStack,
    World,
};
impl Entity {
    pub fn to_nbt(&self) -> Option<Compound> {
        if matches!(self.kind, EntityKind::Player(..)) {
            return None;
        }
        self.write_entity_nbt()
    }
    pub(super) fn write_entity_nbt(&self) -> Option<Compound> {
        if self.dead
            || matches!(
                self.kind,
                EntityKind::Fireball(..)
                    | EntityKind::Egg(..)
                    | EntityKind::Lightning { .. }
                    | EntityKind::Fish(..)
            )
        {
            return None;
        }
        let mut n = Compound::new();
        n.put("id", Tag::String(self.kind.name().into()));
        n.put(
            "Pos",
            Tag::List(
                6,
                vec![
                    Tag::Double(self.pos.x),
                    Tag::Double(self.pos.y + self.y_size as f64),
                    Tag::Double(self.pos.z),
                ],
            ),
        );
        n.put(
            "Motion",
            Tag::List(
                6,
                vec![
                    Tag::Double(self.motion.x),
                    Tag::Double(self.motion.y),
                    Tag::Double(self.motion.z),
                ],
            ),
        );
        n.put(
            "Rotation",
            Tag::List(5, vec![Tag::Float(self.yaw), Tag::Float(self.pitch)]),
        );
        n.put("FallDistance", Tag::Float(self.fall_distance));
        n.put("Fire", Tag::Short(self.fire as i16));
        n.put("Air", Tag::Short(self.air as i16));
        n.put("OnGround", Tag::Byte(self.on_ground as i8));
        if self.kind.is_living() {
            for (key, val) in [
                ("Health", self.living.health),
                ("HurtTime", self.living.hurt_time),
                ("DeathTime", self.living.death_time),
                ("AttackTime", self.living.attack_time),
            ] {
                n.put(key, Tag::Short(val as i16));
            }
        }
        if !self.living.effects.is_empty() && self.kind.is_living() {
            let effects = self
                .living
                .effects
                .iter()
                .map(|e| {
                    let mut n = Compound::new();
                    n.put("Id", Tag::Byte(e.id as i8));
                    n.put("Amplifier", Tag::Byte(e.amplifier as i8));
                    n.put("Duration", Tag::Int(e.duration));
                    Tag::Compound(n)
                })
                .collect();
            n.put("ActiveEffects", Tag::List(10, effects));
        }
        match &self.kind {
            EntityKind::XPOrb(s) => {
                n.put("Health", Tag::Short(s.health as i8 as i16));
                n.put("Age", Tag::Short(s.age as i16));
                n.put("Value", Tag::Short(s.value as i16));
            }
            EntityKind::Enderman(s) => {
                n.put("carried", Tag::Short(s.carried as i16));
                n.put("carriedData", Tag::Short(s.carried_data as i16));
            }

            EntityKind::Painting(s) => {
                n.put("Dir", Tag::Byte(s.direction as i8));
                n.put(
                    "Motive",
                    Tag::String(super::painting::ARTS[s.art].title.into()),
                );
                for (key, v) in ["TileX", "TileY", "TileZ"].into_iter().zip(s.tile) {
                    n.put(key, Tag::Int(v));
                }
            }
            EntityKind::Minecart(s) => {
                n.put("Type", Tag::Int(s.cart_type));
                if s.cart_type == 2 {
                    n.put("PushX", Tag::Double(s.push_x));
                    n.put("PushZ", Tag::Double(s.push_z));
                    n.put("Fuel", Tag::Short(s.fuel as i16));
                } else if s.cart_type == 1 {
                    let items: Vec<_> = s
                        .items
                        .iter()
                        .enumerate()
                        .filter_map(|(slot, item)| {
                            let item = item.as_ref()?;
                            let mut n = Compound::new();
                            n.put("Slot", Tag::Byte(slot as i8));
                            n.put("id", Tag::Short(item.id as i16));
                            n.put("Count", Tag::Byte(item.count as i8));
                            n.put("Damage", Tag::Short(item.damage as i16));
                            Some(Tag::Compound(n))
                        })
                        .collect();
                    n.put(
                        "Items",
                        Tag::List(if items.is_empty() { 1 } else { 10 }, items),
                    );
                }
            }
            EntityKind::Item {
                item, age, health, ..
            } => {
                n.put("Health", Tag::Short(*health as i8 as i16));
                n.put("Age", Tag::Short(*age as i16));
                let mut i = Compound::new();
                i.put("id", Tag::Short(item.id));
                i.put("Count", Tag::Byte(item.count as i8));
                i.put("Damage", Tag::Short(item.damage));
                n.put("Item", Tag::Compound(i));
            }
            EntityKind::FallingSand { block, .. } => n.put("Tile", Tag::Byte(*block as i8)),
            EntityKind::Tnt { fuse } => n.put("Fuse", Tag::Byte(*fuse as i8)),
            EntityKind::Pig { saddled } => n.put("Saddle", Tag::Byte(*saddled as i8)),
            EntityKind::Sheep {
                sheared,
                fleece_color,
            } => {
                n.put("Sheared", Tag::Byte(*sheared as i8));
                n.put("Color", Tag::Byte(*fleece_color as i8));
            }
            EntityKind::Creeper { powered: true, .. } => n.put("powered", Tag::Byte(1)),
            EntityKind::Slime { size, .. } => n.put("Size", Tag::Int(*size - 1)),
            EntityKind::PigZombie { anger, .. } => n.put("Anger", Tag::Short(*anger as i16)),
            EntityKind::Wolf(s) => {
                n.put("Angry", Tag::Byte((s.flags & 2 != 0) as i8));
                n.put("Sitting", Tag::Byte((s.flags & 1 != 0) as i8));
                n.put("Owner", Tag::String(s.owner.clone()));
            }
            EntityKind::Arrow(s) => write_projectile(&mut n, s, true),
            EntityKind::Snowball(s) | EntityKind::Egg(s) => write_projectile(&mut n, s, false),
            _ => {}
        }
        Some(n)
    }
    pub fn from_nbt(w: &mut World, n: &Compound) -> Option<Self> {
        let mut e = super::registry::create(w, &n.string("id"))?;
        e.read_existing_nbt(w, n)?;
        Some(e)
    }
    pub(in crate::entity) fn read_existing_nbt(
        &mut self,
        w: &mut World,
        n: &Compound,
    ) -> Option<()> {
        fn doubles(n: &Compound, key: &str) -> Option<Vec<f64>> {
            n.list(key)
                .iter()
                .map(|t| {
                    if let Tag::Double(v) = t {
                        Some(*v)
                    } else {
                        None
                    }
                })
                .collect()
        }
        let pos = doubles(n, "Pos")?;
        let motion = doubles(n, "Motion")?;
        if pos.len() < 3 || motion.len() < 3 {
            return None;
        }
        let rotation: Vec<_> = n
            .list("Rotation")
            .iter()
            .filter_map(|t| {
                if let Tag::Float(v) = t {
                    Some(*v)
                } else {
                    None
                }
            })
            .collect();
        if rotation.len() < 2 {
            return None;
        }
        self.motion = Vec3D::new(
            if motion[0].abs() > 10.0 {
                0.0
            } else {
                motion[0]
            },
            if motion[1].abs() > 10.0 {
                0.0
            } else {
                motion[1]
            },
            if motion[2].abs() > 10.0 {
                0.0
            } else {
                motion[2]
            },
        );
        self.set_position(pos[0], pos[1], pos[2]);
        self.prev_pos = self.pos;
        self.last_tick_pos = self.pos;
        self.yaw = rotation[0];
        self.pitch = rotation[1];
        self.prev_yaw = self.yaw;
        self.prev_pitch = self.pitch;
        self.fall_distance = match n.get("FallDistance") {
            Some(Tag::Float(v)) => *v,
            _ => 0.0,
        };
        self.fire = n.short("Fire") as i32;
        self.air = n.short("Air") as i32;
        self.on_ground = n.boolean("OnGround");
        self.yaw %= 360.0;
        self.pitch %= 360.0;
        if self.kind.is_living() {
            self.living.health = if n.has("Health") {
                n.short("Health") as i32
            } else {
                10
            };
            self.living.hurt_time = n.short("HurtTime") as i32;
            self.living.death_time = n.short("DeathTime") as i32;
            self.living.attack_time = n.short("AttackTime") as i32;
        }
        if n.has("ActiveEffects") {
            for effect in n.list("ActiveEffects") {
                if let Tag::Compound(n) = effect {
                    let effect = super::potion::PotionEffect::new(
                        n.byte("Id") as i32,
                        n.int("Duration"),
                        n.byte("Amplifier") as i32,
                    );
                    if let Some(old) = self.living.effects.iter_mut().find(|e| e.id == effect.id) {
                        *old = effect;
                    } else {
                        self.living.effects.push(effect);
                        if self.living.effects.len() > self.living.effect_capacity * 3 / 4 {
                            self.living.effect_capacity *= 2;
                        }
                    }
                }
            }
        }
        match &mut self.kind {
            EntityKind::XPOrb(s) => {
                s.health = n.short("Health") as i32 & 255;
                s.age = n.short("Age") as i32;
                s.value = n.short("Value") as i32;
            }
            EntityKind::Enderman(s) => {
                s.carried = n.short("carried") as i8;
                s.carried_data = n.short("carryingData") as i8;
            }

            EntityKind::Minecart(s) => {
                s.cart_type = n.int("Type");
                if s.cart_type == 2 {
                    s.push_x = match n.get("PushX") {
                        Some(Tag::Double(x)) => *x,
                        _ => 0.0,
                    };
                    s.push_z = match n.get("PushZ") {
                        Some(Tag::Double(x)) => *x,
                        _ => 0.0,
                    };
                    s.fuel = n.short("Fuel") as i32;
                } else if s.cart_type == 1 {
                    s.items = vec![None; 27];
                    for item in n.list("Items") {
                        if let Tag::Compound(item) = item {
                            let slot = item.byte("Slot") as u8 as usize;
                            if slot < 27 {
                                s.items[slot] = Some(super::player::ItemStack::new(
                                    item.short("id") as i32,
                                    item.byte("Count") as i32,
                                    item.short("Damage") as i32,
                                ));
                            }
                        }
                    }
                }
            }
            EntityKind::Item {
                item, age, health, ..
            } => {
                let i = n.compound("Item")?;
                *item = ItemStack::new(
                    i.short("id") as i32,
                    i.byte("Count") as i32,
                    i.short("Damage") as i32,
                );
                *age = n.short("Age") as i32;
                *health = n.short("Health") as i32 & 255;
            }
            EntityKind::FallingSand { block, .. } => *block = n.byte("Tile") as u8,
            EntityKind::Tnt { fuse } => *fuse = n.byte("Fuse") as i32,
            EntityKind::Pig { saddled } => *saddled = n.boolean("Saddle"),
            EntityKind::Sheep {
                sheared,
                fleece_color,
            } => {
                *sheared = n.boolean("Sheared");
                *fleece_color = n.byte("Color") as u8 & 15;
            }
            EntityKind::Creeper { powered, .. } => *powered = n.boolean("powered"),
            EntityKind::PigZombie { anger, .. } => *anger = n.short("Anger") as i32,
            EntityKind::Wolf(s) => {
                s.flags = (n.boolean("Angry") as u8) * 2 + n.boolean("Sitting") as u8;
                s.owner = n.string("Owner");
                if !s.owner.is_empty() {
                    s.flags |= 4;
                }
            }
            EntityKind::Arrow(s) => *s = read_projectile(n, true),
            EntityKind::Snowball(s) | EntityKind::Egg(s) => *s = read_projectile(n, false),
            EntityKind::Fireball(s) => {
                *s = FireballState {
                    arrow: read_projectile(n, false),
                    acceleration: Vec3D::default(),
                }
            }
            _ => {}
        }
        if matches!(self.kind, EntityKind::Slime { .. }) {
            self.set_slime_size(n.int("Size") + 1);
        }
        if matches!(self.kind, EntityKind::Player(..)) {
            self.read_player_tags(w, n);
        }
        if let EntityKind::Painting(s) = &mut self.kind {
            s.direction = n.byte("Dir") as i32;
            s.tile = [n.int("TileX"), n.int("TileY"), n.int("TileZ")];
            s.art = super::painting::ARTS
                .iter()
                .position(|a| a.title == n.string("Motive"))
                .unwrap_or(0);
            let direction = s.direction;
            self.set_painting_direction(direction);
        }
        Some(())
    }
}
fn write_projectile(n: &mut Compound, s: &ArrowState, arrow: bool) {
    for (key, val) in [
        ("xTile", s.tile[0]),
        ("yTile", s.tile[1]),
        ("zTile", s.tile[2]),
    ] {
        n.put(key, Tag::Short(val as i16));
    }
    n.put("inTile", Tag::Byte(s.block as i8));
    if arrow {
        n.put("inData", Tag::Byte(s.metadata as i8));
        n.put("player", Tag::Byte(s.belongs_to_player as i8));
    }
    n.put("shake", Tag::Byte(s.shake as i8));
    n.put("inGround", Tag::Byte(s.in_ground as i8));
}
fn read_projectile(n: &Compound, arrow: bool) -> ArrowState {
    ArrowState {
        tile: [
            n.short("xTile") as i32,
            n.short("yTile") as i32,
            n.short("zTile") as i32,
        ],
        block: n.byte("inTile") as u8,
        metadata: if arrow {
            n.byte("inData") as u8 as i32
        } else {
            0
        },
        belongs_to_player: arrow && n.boolean("player"),
        shake: n.byte("shake") as u8 as i32,
        in_ground: n.byte("inGround") == 1,
        ..Default::default()
    }
}
impl World {
    pub fn load_chunk_entities(&mut self, cx: i32, cz: i32) {
        let tags = std::mem::take(&mut self.chunk_mut(cx, cz).raw_entities);
        for tag in tags {
            if let Tag::Compound(n) = &tag {
                if let Some(mut e) = Entity::from_nbt(self, n) {
                    let cy = crate::math::floor_double(e.pos.y / 16.0).clamp(0, 7);
                    e.added_to_chunk = true;
                    e.chunk_x = cx;
                    e.chunk_y = cy;
                    e.chunk_z = cz;
                    let id = e.entity_id;
                    self.chunk_mut(cx, cz).entity_lists[cy as usize].push(id);
                    self.entities.insert(id, e);
                    self.loaded_entity_list.push(id);
                    continue;
                }
            }
            self.chunk_mut(cx, cz).raw_entities.push(tag);
        }
        let mut tiles = crate::world::java_hash::JavaHashSet::default();
        for &(x, y, z) in &self.chunk(cx, cz).tile_entity_order {
            tiles.insert(
                x.wrapping_mul(8976890)
                    .wrapping_add(y.wrapping_mul(981131))
                    .wrapping_add(z),
                (x, y, z),
            );
        }
        for (x, y, z) in tiles.iter() {
            let key = (x, y, z);
            // onChunkLoad adds the existing objects, without validating them.
            // Disk-loaded chunks have no IDs; an unload/load of the same Chunk
            // retains its identities (and their invalid flags).
            let id = if let Some(&id) = self.chunk(cx, cz).tile_entity_ids.get(&key) {
                self.tile_objects.get_mut(&id).unwrap().detached = None;
                id
            } else {
                let te = self.chunk_mut(cx, cz).tile_entities.remove(&key).unwrap();
                let id = self.new_tile_object((cx * 16 + x, y, cz * 16 + z), te);
                let te = self
                    .tile_objects
                    .get_mut(&id)
                    .unwrap()
                    .detached
                    .take()
                    .unwrap();
                self.chunk_mut(cx, cz).tile_entities.insert(key, te);
                self.chunk_mut(cx, cz).tile_entity_ids.insert(key, id);
                id
            };
            if self.scanning_tiles {
                self.pending_tile_entities.push(id);
            } else {
                self.loaded_tile_entities.push(id);
            }
        }
    }
    pub(crate) fn chunk_entity_nbt(&self, cx: i32, cz: i32) -> Vec<Tag> {
        let c = self.chunk(cx, cz);
        let mut tags = c.raw_entities.clone();
        for list in &c.entity_lists {
            for id in list {
                if let Some(n) = self.entities.get(id).and_then(|e| e.to_nbt()) {
                    tags.push(Tag::Compound(n));
                }
            }
        }
        tags
    }
    pub fn unload_chunk_entities(&mut self, cx: i32, cz: i32) {
        if let Some(c) = self.chunks.get_mut(&(cx, cz)) {
            c.loaded = false;
            for list in &c.entity_lists {
                self.unloaded_entity_list.extend_from_slice(list);
            }
        }
        if let Some(c) = self.chunks.get_mut(&(cx, cz)) {
            for (key, id) in &c.tile_entity_ids {
                let o = self.tile_objects.get_mut(id).unwrap();
                // b1.8.1 queues removal from the world list without invalidate().
                // The old object gets its final update before removeAll runs.
                o.pending_unload = true;
                o.detached = c.tile_entities.get(key).cloned();
            }
        }
        self.spawner_animations
            .retain(|&(x, _, z), _| x >> 4 != cx || z >> 4 != cz);
    }
}
