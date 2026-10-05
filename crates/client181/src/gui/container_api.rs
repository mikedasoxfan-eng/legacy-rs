//! Integration contract for the app/world channels (see CONTAINERS.md).
use super::{
    screens::container::GuiContainer,
    system::{GameAction, GuiSystem},
};
use worldgen::{
    container::{ContainerCommand, ContainerSnapshot},
    entity::player::PlayerEvent,
};
pub fn inventory_key_request() -> ContainerCommand {
    ContainerCommand::OpenInventory
}
pub fn activation_request(event: &PlayerEvent) -> Option<ContainerCommand> {
    if let PlayerEvent::OpenContainer { block, position } = event {
        Some(ContainerCommand::OpenBlock {
            block: *block,
            position: *position,
        })
    } else if let PlayerEvent::OpenVehicleInventory {entity}=event {
        Some(ContainerCommand::OpenVehicle {entity:*entity})
    } else {
        None
    }
}
impl GuiSystem {
    /// `EntityPlayerSP.addStat`: an achievement needs its parent and pops up the first time;
    /// any other stat just counts.
    pub fn add_stat(&mut self, id: i32, amount: i32) {
        let list = &crate::gui::stats::ACHIEVEMENTS;
        if let Some(i) = list.iter().position(|a| a.id == id) {
            if self.ui.stats.can_unlock(i) {
                if !self.ui.stats.unlocked(i) {
                    self.toast.queue_taken(&self.ui, i);
                }
                self.ui.stats.add(id, amount);
            }
        } else {
            self.ui.stats.add(id, amount);
        }
    }
    /// Called with the authoritative answer to an Open request, on the UI thread.
    pub fn open_container(&mut self, snapshot: ContainerSnapshot) -> Vec<GameAction> {
        let screen = Box::new(GuiContainer::new(&snapshot));
        self.ui.container_snapshot = Some(snapshot);
        let pending = self.ui.container_commands.len();
        let mut actions = Vec::new();
        self.display_gui_screen(Some(screen), &mut actions);
        // The Open request already closed the old world-side container. Its UI
        // callback must not close a newly reopened window zero.
        let mut generated = self.ui.container_commands.split_off(pending);
        generated.retain(|c| !matches!(c, ContainerCommand::Close { .. } | ContainerCommand::CloseFromGui { .. }));
        self.ui.container_commands.extend(generated);
        actions
    }
    /// Called after a click and once per unpaused world tick. Stale windows are ignored.
    pub fn update_container(&mut self, snapshot: ContainerSnapshot) {
        if self
            .ui
            .container_snapshot
            .as_ref()
            .is_some_and(|s| s.window == snapshot.window)
        {
            self.ui.container_snapshot = Some(snapshot);
        }
    }
    pub fn container_closed(&mut self, window: i32) {
        if let Some(s) = self
            .ui
            .container_snapshot
            .as_mut()
            .filter(|s| s.window == window)
        {
            s.alive = false;
        }
    }
    /// Drain onto the existing world-thread channel, including on screen close.
    pub fn take_container_commands(&mut self) -> Vec<ContainerCommand> {
        std::mem::take(&mut self.ui.container_commands)
    }
}
