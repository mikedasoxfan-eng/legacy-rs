//! World-thread message handler. No renderer/app dependency.
use super::*;
use crate::{World, entity::player::PlayerEvent, world::tile::TileEntity};

#[derive(Clone, Debug, Default)]
pub struct ContainerSession {
    pub active: Option<Container>,
    pub next_window: i32,
}
impl World {
    pub fn container_command(
        &mut self,
        player: i32,
        command: ContainerCommand,
    ) -> Option<ContainerSnapshot> {
        self.entities.get(&player)?.player_state()?;
        let mut entity = self.entities.remove(&player)?;
        let mut session = self.container_sessions.remove(&player).unwrap_or_default();
        let p = &mut entity.player_state_mut()?.inventory;
        match command {
            ContainerCommand::OpenInventory | ContainerCommand::OpenBlock { .. } => {
                if let Some(mut old) = session.active.take() {
                    old.close(p, false);
                    self.deliver_container_events(&mut entity, &mut old);
                }
                let mut container = match command {
                    ContainerCommand::OpenInventory => {
                        Some(Container::new(ContainerKind::Player, 0))
                    }
                    ContainerCommand::OpenBlock {
                        block,
                        position: (x, y, z),
                    } => {
                        session.next_window = session.next_window % 100 + 1;
                        self.make_block_container(block, x, y, z, session.next_window)
                    }
                    _ => unreachable!(),
                };
                if let Some(c) = &mut container {
                    c.next_map_id = self.map_id;
                }
                if matches!(command, ContainerCommand::OpenInventory) {
                    entity
                        .player_state_mut()
                        .unwrap()
                        .events
                        .push(PlayerEvent::Container(ContainerEvent::Achievement(
                            "openInventory",
                        )));
                }
                session.active = container;
            }
            ContainerCommand::Click {
                window,
                slot,
                button,
                shift,
            } => {
                if let Some(c) = session.active.as_mut().filter(|c| c.window == window) {
                    self.read_container_tiles(c);
                    c.next_map_id = self.map_id;
                    c.slot_click(p, slot, button, shift);
                    self.map_id = c.next_map_id;
                    self.write_container_tiles(c);
                    self.deliver_container_events(&mut entity, c);
                }
            }
            ContainerCommand::Close { window } => {
                if session.active.as_ref().is_some_and(|c| c.window == window) {
                    let mut c = session.active.take().unwrap();
                    c.close(p, false);
                    self.deliver_container_events(&mut entity, &mut c);
                }
            }
        }
        let result = session.active.as_mut().map(|c| {
            self.read_container_tiles(c);
            let mut snapshot = c.snapshot(&entity.player_state().unwrap().inventory);
            snapshot.alive = !entity.dead && entity.living.health > 0;
            snapshot.preview = PlayerPreview::from_entity(&entity);
            snapshot
        });
        self.container_sessions.insert(player, session);
        self.entities.insert(player, entity);
        result
    }
    pub fn container_snapshot(&mut self, player: i32) -> Option<ContainerSnapshot> {
        let mut session = self.container_sessions.remove(&player)?;
        let result = session.active.as_mut().and_then(|c| {
            self.read_container_tiles(c);
            let e = self.entities.get(&player)?;
            let mut snapshot = c.snapshot(&e.player_state()?.inventory);
            snapshot.alive = !e.dead && e.living.health > 0;
            snapshot.preview = PlayerPreview::from_entity(e);
            Some(snapshot)
        });
        self.container_sessions.insert(player, session);
        result
    }
    fn deliver_container_events(&mut self, e: &mut crate::entity::Entity, c: &mut Container) {
        for event in c.events.drain(..) {
            if let ContainerEvent::Drop(stack) = event {
                e.drop_player_item(self, stack, false);
            } else {
                e.player_state_mut()
                    .unwrap()
                    .events
                    .push(PlayerEvent::Container(event));
            }
        }
    }
    pub(crate) fn validate_container_entity(&mut self, e: &mut crate::entity::Entity) {
        let Some(mut session) = self.container_sessions.remove(&e.entity_id) else {
            return;
        };
        let usable = session.active.as_ref().is_none_or(|c| {
            c.kind == ContainerKind::Player
                || c.positions.iter().all(|&(x, y, z)| {
                    let id = self.get_block_id(x, y, z);
                    let valid = match c.kind {
                        ContainerKind::Workbench => id == 58,
                        ContainerKind::Chest { .. } => id == 54,
                        ContainerKind::Furnace => matches!(id, 61 | 62),
                        ContainerKind::Dispenser => id == 23,
                        _ => true,
                    };
                    let dx = e.pos.x - (x as f64 + 0.5);
                    let dy = e.pos.y - (y as f64 + 0.5);
                    let dz = e.pos.z - (z as f64 + 0.5);
                    valid && dx * dx + dy * dy + dz * dz <= 64.0
                })
        });
        if !usable {
            if let Some(mut c) = session.active.take() {
                c.close(&mut e.player_state_mut().unwrap().inventory, false);
                let window = c.window;
                self.deliver_container_events(e, &mut c);
                e.player_state_mut()
                    .unwrap()
                    .events
                    .push(PlayerEvent::Container(ContainerEvent::Closed { window }));
            }
        }
        self.container_sessions.insert(e.entity_id, session);
    }
    pub fn make_block_container(
        &mut self,
        block: u8,
        x: i32,
        y: i32,
        z: i32,
        window: i32,
    ) -> Option<Container> {
        let actual = self.get_block_id(x, y, z);
        if actual != block && !(matches!(actual, 61 | 62) && matches!(block, 61 | 62)) {
            return None;
        }
        let kind = match block {
            58 => ContainerKind::Workbench,
            54 => ContainerKind::Chest { rows: 3 },
            61 | 62 => ContainerKind::Furnace,
            23 => ContainerKind::Dispenser,
            _ => return None,
        };
        let mut c = Container::new(kind, window);
        c.positions.push((x, y, z));
        if block == 54 {
            if self.is_block_normal_cube(x, y + 1, z) {
                return None;
            }
            // BlockChest.blockActivated wraps in this precise order. Negative
            // neighbors precede the clicked chest, positive neighbors follow it.
            for (dx, dz) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if self.get_block_id(x + dx, y, z + dz) == 54 {
                    if self.is_block_normal_cube(x + dx, y + 1, z + dz) {
                        return None;
                    }
                    if dx < 0 || dz < 0 {
                        c.positions.insert(0, (x + dx, y, z + dz));
                    } else {
                        c.positions.push((x + dx, y, z + dz));
                    }
                }
            }
            if c.positions.len() > 1 {
                let positions = c.positions;
                c = Container::new(
                    ContainerKind::Chest {
                        rows: positions.len() * 3,
                    },
                    window,
                );
                c.positions = positions;
            }
        }
        if block != 58 {
            self.read_container_tiles(&mut c);
        }
        Some(c)
    }
    pub fn container_usable(&mut self, player: i32) -> bool {
        let Some(session) = self.container_sessions.remove(&player) else {
            return false;
        };
        let result = session.active.as_ref().is_some_and(|c| {
            let Some(e) = self.entities.get(&player) else {
                return false;
            };
            let pos = e.pos;
            c.kind == ContainerKind::Player
                || c.positions.iter().all(|&(x, y, z)| {
                    let dx = pos.x - (x as f64 + 0.5);
                    let dy = pos.y - (y as f64 + 0.5);
                    let dz = pos.z - (z as f64 + 0.5);
                    let id = self.get_block_id(x, y, z);
                    let valid = match c.kind {
                        ContainerKind::Workbench => id == 58,
                        ContainerKind::Chest { .. } => id == 54,
                        ContainerKind::Furnace => matches!(id, 61 | 62),
                        ContainerKind::Dispenser => id == 23,
                        _ => true,
                    };
                    valid && dx * dx + dy * dy + dz * dz <= 64.0
                })
        });
        self.container_sessions.insert(player, session);
        result
    }
    fn read_container_tiles(&mut self, c: &mut Container) {
        let mut offset = 0;
        for &(x, y, z) in &c.positions {
            match self.get_block_tile_entity(x, y, z) {
                Some(TileEntity::Chest(items)) => {
                    for s in items.iter() {
                        c.tile[offset] = s.map(|s| {
                            ItemStack::new(s.id as i32, s.count as i8 as i32, s.damage as i32)
                        });
                        offset += 1;
                    }
                }
                Some(TileEntity::Furnace(f)) => {
                    c.tile.copy_from_slice(&f.items);
                    // GuiFurnace's progress read mutates a missing fuel duration
                    // to 200. Do that on the owning thread before publishing.
                    if f.burn > 0 && f.current_fuel == 0 {
                        f.burn_scaled(12);
                    }
                    c.progress = [f.cook, f.burn, f.current_fuel];
                }
                Some(TileEntity::Dispenser(items)) => c.tile.copy_from_slice(items.as_ref()),
                _ => {}
            }
        }
    }
    fn write_container_tiles(&mut self, c: &Container) {
        let mut offset = 0;
        for &(x, y, z) in &c.positions {
            match self.get_block_tile_entity(x, y, z) {
                Some(TileEntity::Chest(items)) => {
                    for s in items.iter_mut() {
                        *s = c.tile[offset].map(Into::into);
                        offset += 1;
                    }
                }
                Some(TileEntity::Furnace(f)) => f.items.copy_from_slice(&c.tile),
                Some(TileEntity::Dispenser(items)) => items.copy_from_slice(&c.tile),
                _ => continue,
            }
            self.mark_container_dirty(x, z);
        }
    }
}
