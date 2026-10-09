use super::*;
impl World {
    pub fn all_players_fully_asleep(&self) -> bool {
        !self.players.is_empty()
            && self.player_entities.len() == self.players.len()
            && self.player_entities.iter().all(|id| {
                id.and_then(|id| self.entities.get(&id))
                    .and_then(Entity::player_state)
                    .is_some_and(|s| s.sleeping && s.sleep_timer >= 100)
            })
    }
    pub(crate) fn tick_sleeping_players(&mut self) {
        if !self.all_players_fully_asleep() {
            return;
        }
        let time = self.world_time.wrapping_add(24000);
        self.world_time = time - time % 24000;
        for id in self.player_entities.clone().into_iter().flatten() {
            if let Some(mut p) = self.entities.remove(&id) {
                if p.player_state().is_some_and(|s| s.sleeping) {
                    p.wake_up(self, false, true);
                }
                if let Some(i) = p.player_state().and_then(|s| s.player_index) {
                    self.players[i] = (p.pos.x, p.pos.y, p.pos.z);
                }
                self.entities.insert(id, p);
            }
        }
        self.info.rain_time = 0;
        self.info.raining = false;
        self.info.thunder_time = 0;
        self.info.thundering = false;
    }
}
