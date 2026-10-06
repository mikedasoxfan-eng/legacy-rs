use super::*;
use crate::nbt::{Compound, Tag};
impl Entity {
    /// EntityPlayer.preparePlayerToSpawn, before the controller's yaw flip.
    pub fn prepare_player_to_spawn(&mut self, w: &mut World) {
        self.y_offset = 1.62;
        self.width = 0.6;
        self.height = 1.8;
        while self.pos.y > 0.0 {
            self.set_position(self.pos.x, self.pos.y, self.pos.z);
            if w.get_colliding_bounding_boxes(Some(self.entity_id), self.bounding_box)
                .is_empty()
            {
                break;
            }
            self.pos.y += 1.0;
        }
        self.motion = Vec3D::default();
        self.pitch = 0.0;
        self.living.health = 20;
        self.living.death_time = 0;
    }
    /// level.dat's Player compound does not contain an EntityList id.
    pub fn player_nbt(&self) -> Option<Compound> {
        let s = self.player_state()?;
        let mut e = self.clone();
        e.dead = false;
        let mut n = e.write_entity_nbt()?;
        n.0.retain(|(key, _)| key != "id");
        n.put("Inventory", s.inventory.to_nbt());
        n.put("Dimension", Tag::Int(s.dimension));
        n.put("Sleeping", Tag::Byte(s.sleeping as i8));
        n.put("SleepTimer", Tag::Short(s.sleep_timer as i16));
        // EntityPlayer.writeEntityToNBT's order matters: Java's HashMap keeps colliding keys in
        // insertion order. EntityPlayerSP adds the score last.
        s.experience.write_nbt(&mut n);
        if let Some((x, y, z)) = s.spawn {
            n.put("SpawnX", Tag::Int(x));
            n.put("SpawnY", Tag::Int(y));
            n.put("SpawnZ", Tag::Int(z));
        }
        s.food.write_nbt(&mut n);
        n.put("Score", Tag::Int(s.score));
        Some(n)
    }
    pub fn player_from_nbt(
        w: &mut World,
        n: &Compound,
        username: impl Into<String>,
    ) -> Option<Self> {
        let mut e = Self::player(w, username);
        e.read_existing_nbt(w, n)?;
        Some(e)
    }
    pub(in crate::entity) fn read_player_tags(&mut self, w: &mut World, n: &Compound) {
        let bed = (floor(self.pos.x), floor(self.pos.y), floor(self.pos.z));
        let s = self.player_state_mut().unwrap();
        s.inventory.read_nbt(n.list("Inventory"));
        s.dimension = n.int("Dimension");
        s.sleeping = n.boolean("Sleeping");
        s.sleep_timer = n.short("SleepTimer") as i32;
        s.score = n.int("Score");
        s.food.read_nbt(n);
        s.experience.read_nbt(n);
        if n.has("SpawnX") && n.has("SpawnY") && n.has("SpawnZ") {
            s.spawn = Some((n.int("SpawnX"), n.int("SpawnY"), n.int("SpawnZ")));
        }
        if s.sleeping {
            s.bed = Some(bed);
            self.wake_up(w, true, false);
        }
    }
}
impl World {
    /// Construct and join the singleplayer saved in level.dat, or a new player at spawn.
    pub fn spawn_player(&mut self, username: impl Into<String>) -> i32 {
        let username = username.into();
        let saved = self.info.player.take();
        let mut e = super::controller::PlayerControllerSP::create_player(self, &username);
        e.prepare_player_to_spawn(self);
        super::controller::PlayerControllerSP::flip_player(&mut e,
            super::controller::PlayerControllerSP::saved_world_is_creative(self.info.game_type));
        if let Some(n) = saved {
            let _ = e.read_existing_nbt(self, &n);
        }
        e.player_state_mut().unwrap().dimension = if self.dimension.is_hell_world() {
            -1
        } else {
            0
        };
        let id = e.entity_id;
        self.join_entity_in_surroundings(e.pos);
        self.entity_joined_world(e);
        id
    }
}
