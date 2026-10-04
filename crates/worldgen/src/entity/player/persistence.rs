use super::*;
use crate::nbt::{Compound,Tag};
impl Entity {
    /// level.dat's Player compound does not contain an EntityList id.
    pub fn player_nbt(&self)->Option<Compound> {
        let s=self.player_state()?;let mut e=self.clone();e.dead=false;
        let mut n=e.write_entity_nbt()?;n.0.retain(|(key,_)|key!="id");
        n.put("Inventory",s.inventory.to_nbt());n.put("Dimension",Tag::Int(s.dimension));n.put("Sleeping",Tag::Byte(s.sleeping as i8));n.put("SleepTimer",Tag::Short(s.sleep_timer as i16));n.put("Score",Tag::Int(s.score));
        if let Some((x,y,z))=s.spawn {n.put("SpawnX",Tag::Int(x));n.put("SpawnY",Tag::Int(y));n.put("SpawnZ",Tag::Int(z));}Some(n)
    }
    pub fn player_from_nbt(w:&mut World,n:&Compound,username:impl Into<String>)->Option<Self> {
        let mut n=n.clone();n.put("id",Tag::String("Player".into()));let mut e=Self::from_nbt(w,&n)?;e.player_state_mut()?.username=username.into();Some(e)
    }
    pub(in crate::entity) fn read_player_tags(&mut self,w:&mut World,n:&Compound) {
        let bed=(floor(self.pos.x),floor(self.pos.y),floor(self.pos.z));
        let s=self.player_state_mut().unwrap();s.inventory.read_nbt(n.list("Inventory"));s.dimension=n.int("Dimension");s.sleeping=n.boolean("Sleeping");s.sleep_timer=n.short("SleepTimer") as i32;s.score=n.int("Score");
        if n.has("SpawnX")&&n.has("SpawnY")&&n.has("SpawnZ") {s.spawn=Some((n.int("SpawnX"),n.int("SpawnY"),n.int("SpawnZ")));}
        if s.sleeping {s.bed=Some(bed);self.wake_up(w,true,false);}
    }
}
impl World {
    /// Construct and join the singleplayer saved in level.dat, or a new player at spawn.
    pub fn spawn_player(&mut self,username:impl Into<String>)->i32 {
        let username=username.into();
        let mut e=if let Some(n)=self.info.player.clone() {Entity::player_from_nbt(self,&n,&username).unwrap_or_else(||Entity::player(self,&username))}else{Entity::player(self,&username)};
        e.player_state_mut().unwrap().dimension=if self.dimension.is_hell_world(){-1}else{0};
        let id=e.entity_id;self.join_entity_in_surroundings(e.pos);self.entity_joined_world(e);id
    }
}
