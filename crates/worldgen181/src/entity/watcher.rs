//! Persistent Entity DataWatcher and incoming metadata packet application.
use super::{
    data_watcher::{Value, WatchableObject},
    Entity, EntityKind,
};
impl Entity {
    pub(super) fn init_data_watcher(&mut self) {
        let w = &mut self.data_watcher;
        w.add(0, Value::Byte(0)).unwrap();
        match self.kind {
            EntityKind::Creeper { .. } => {
                w.add(16, Value::Byte(-1)).unwrap();
                w.add(17, Value::Byte(0)).unwrap();
            }
            EntityKind::Enderman(..) | EntityKind::Player(..) => {
                w.add(16, Value::Byte(0)).unwrap();
                w.add(17, Value::Byte(0)).unwrap();
            }
            EntityKind::Ghast(..) | EntityKind::Pig { .. } | EntityKind::Sheep { .. } => {
                w.add(16, Value::Byte(0)).unwrap();
            }
            EntityKind::Slime { .. } => {
                w.add(16, Value::Byte(1)).unwrap();
            }
            EntityKind::Wolf(..) => {
                w.add(16, Value::Byte(0)).unwrap();
                w.add(17, Value::String(Vec::new())).unwrap();
                // entityInit is virtual inside Entity's constructor, before EntityLiving.health=10.
                w.add(18, Value::Int(0)).unwrap();
            }
            _ => {}
        }
    }
    pub fn entity_flag(&self, bit: i32) -> bool {
        (self.data_watcher.byte(0).unwrap() as i32 & 1i32.wrapping_shl(bit as u32)) != 0
    }
    pub fn set_entity_flag(&mut self, bit: i32, value: bool) {
        let mask = 1i32.wrapping_shl(bit as u32);
        let flags = self.data_watcher.byte(0).unwrap() as i32;
        self.data_watcher
            .update(
                0,
                Value::Byte(if value { flags | mask } else { flags & !mask } as i8),
            )
            .unwrap();
        self.sneaking = self.entity_flag(1);
        self.sprinting = self.entity_flag(3);
    }
    /// Packet metadata replaces values without marking them changed, as in DataWatcher.
    pub fn apply_watcher_list(&mut self, values: &[WatchableObject]) {
        self.data_watcher.update_from_list(values);
        self.sneaking = self.entity_flag(1);
        self.sprinting = self.entity_flag(3);
        let byte = |id| self.data_watcher.byte(id).unwrap_or(0);
        match &mut self.kind {
            EntityKind::Creeper { state, powered, .. } => {
                *state = byte(16) as i32;
                *powered = byte(17) == 1;
            }
            EntityKind::Enderman(s) => {
                s.carried = byte(16);
                s.carried_data = byte(17);
            }
            EntityKind::Ghast(s) => s.attacking = byte(16) == 1,
            EntityKind::Pig { saddled } => *saddled = byte(16) & 1 != 0,
            EntityKind::Sheep {
                sheared,
                fleece_color,
            } => {
                *sheared = byte(16) & 16 != 0;
                *fleece_color = byte(16) as u8 & 15;
            }
            EntityKind::Wolf(s) => {
                s.flags = byte(16) as u8;
                s.owner = String::from_utf16_lossy(self.data_watcher.string(17).unwrap());
                s.watched_health = self.data_watcher.int(18).unwrap();
            }
            // Slime metadata does not call setSlimeSize (no health/size side effects).
            EntityKind::Slime { size, .. } => *size = byte(16) as i32,
            _ => {}
        }
    }
    /// Publish the existing simulation state through the same persistent typed watcher.
    pub fn sync_mob_watcher(&mut self) {
        let mut values = Vec::new();
        match &self.kind {
            EntityKind::Creeper { state, powered, .. } => {
                values.push((16, Value::Byte(*state as i8)));
                if (self.data_watcher.byte(17).unwrap() == 1) != *powered {
                    values.push((17, Value::Byte(*powered as i8)));
                }
            }
            EntityKind::Enderman(s) => {
                values.push((16, Value::Byte(s.carried as i8)));
                values.push((17, Value::Byte(s.carried_data as i8)));
            }
            EntityKind::Ghast(s) => {
                if (self.data_watcher.byte(16).unwrap() == 1) != s.attacking {
                    values.push((16, Value::Byte(s.attacking as i8)));
                }
            }
            EntityKind::Pig { saddled } => {
                let old = self.data_watcher.byte(16).unwrap();
                values.push((16, Value::Byte((old & !1) | *saddled as i8)));
            }
            EntityKind::Sheep {
                sheared,
                fleece_color,
            } => {
                let old = self.data_watcher.byte(16).unwrap();
                values.push((
                    16,
                    Value::Byte(
                        (old & !31) | (*fleece_color as i8 & 15) | if *sheared { 16 } else { 0 },
                    ),
                ));
            }
            EntityKind::Wolf(s) => {
                values.push((16, Value::Byte(s.flags as i8)));
                if String::from_utf16_lossy(self.data_watcher.string(17).unwrap()) != s.owner {
                    values.push((17, Value::String(s.owner.encode_utf16().collect())));
                }
                values.push((18, Value::Int(s.watched_health)));
            }
            EntityKind::Slime { size, .. } => values.push((16, Value::Byte(*size as i8))),
            _ => {}
        }
        for (id, value) in values {
            self.data_watcher.update(id, value).unwrap();
        }
    }
}
