//! Release 1.0 chest corridor and portal room, in Java placement order.
use super::*;
const CHEST_LOOT10: [T; 14] = [
    loot(368,1,1,10), loot(264,1,3,3), loot(265,1,5,10), loot(266,1,3,5),
    loot(331,4,9,5), loot(297,1,3,15), loot(260,1,3,15), loot(257,1,1,5),
    loot(267,1,1,5), loot(307,1,1,5), loot(306,1,1,5), loot(308,1,1,5),
    loot(309,1,1,5), loot(322,1,1,1),
];
impl StrongholdPiece {
    pub(super) fn generate_chest10(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        let c = self.component;
        if c.liquids(w, clip) { return false; }
        c.fill_stones(w, clip, [0,0,0,4,4,6], true, r, &mut BlockSelector::default());
        self.entrance(w,r,clip,self.door,1,1,0);
        self.entrance(w,r,clip,Door::Opening,1,1,6);
        c.fill(w,clip,[3,1,2,3,1,4],98,98,false);
        for [x,y,z] in [[3,1,1],[3,1,5],[3,2,2],[3,2,4],[2,1,2],[2,1,3],[2,1,4]] {
            c.put(w,clip,44,5,x,y,z);
        }
        let (x,y,z) = c.position(3,2,3);
        if !self.flags[0] && clip.contains(x,y,z) {
            self.flags[0] = true;
            let count = 2 + component::random(w,r).next_int(2);
            self.chest(w,clip,r,3,2,3,&CHEST_LOOT10,count);
        }
        true
    }
    pub(super) fn generate_portal10(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        let c = self.component;
        c.fill_stones(w,clip,[0,0,0,10,7,15],false,r,&mut BlockSelector::default());
        self.entrance(w,r,clip,Door::Grates,4,1,0);
        for a in [[1,6,1,1,6,14],[9,6,1,9,6,14],[2,6,1,8,6,2],[2,6,14,8,6,14],
                  [1,1,1,2,1,4],[8,1,1,9,1,4]] {
            c.fill_stones(w,clip,a,false,r,&mut BlockSelector::default());
        }
        c.fill(w,clip,[1,1,1,1,1,3],10,10,false);
        c.fill(w,clip,[9,1,1,9,1,3],10,10,false);
        c.fill_stones(w,clip,[3,1,8,7,1,12],false,r,&mut BlockSelector::default());
        c.fill(w,clip,[4,1,9,6,1,11],10,10,false);
        for z in (3..14).step_by(2) {
            c.fill(w,clip,[0,3,z,0,4,z],101,101,false);
            c.fill(w,clip,[10,3,z,10,4,z],101,101,false);
        }
        for x in (2..9).step_by(2) { c.fill(w,clip,[x,3,15,x,4,15],101,101,false); }
        let stairs = c.rotate(109,3);
        for a in [[4,1,5,6,1,7],[4,2,6,6,2,7],[4,3,7,6,3,7]] {
            c.fill_stones(w,clip,a,false,r,&mut BlockSelector::default());
        }
        for x in 4..=6 {
            c.put(w,clip,109,stairs,x,1,4);
            c.put(w,clip,109,stairs,x,2,5);
            c.put(w,clip,109,stairs,x,3,6);
        }
        let dirs = match c.orientation { 0 => [0,2,3,1], 1 => [1,3,0,2], 3 => [3,1,0,2], _ => [2,0,3,1] };
        for (side, positions) in [ [[4,3,8],[5,3,8],[6,3,8]], [[4,3,12],[5,3,12],[6,3,12]],
                                  [[3,3,9],[3,3,10],[3,3,11]], [[7,3,9],[7,3,10],[7,3,11]] ].into_iter().enumerate() {
            for [x,y,z] in positions {
                let meta = dirs[side] + if component::random(w,r).next_float() > 0.9f32 { 4 } else { 0 };
                c.put(w,clip,120,meta,x,y,z);
            }
        }
        let (x,y,z) = c.position(5,3,6);
        if !self.flags[0] && clip.contains(x,y,z) {
            self.flags[0] = true;
            w.set_block_with_notify(x,y,z,52);
            if let Some(crate::world::tile::TileEntity::MobSpawner { mob, .. }) = w.get_block_tile_entity(x,y,z) {
                *mob = "Silverfish".into();
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generated_starts_retry_until_the_portal_exists() {
        for seed in [0,1,42,-55] {
            let mut r = JavaRandom::new(seed);
            let start = loop {
                let s = StrongholdStart::new(&mut r, -8, 12);
                if s.portal_room.is_some() { break s; }
            };
            assert_eq!(start.pieces[start.portal_room.unwrap()].kind, 12);
            assert_eq!(start.pieces[1].kind, 7);
            assert!(start.pieces.iter().filter(|p| p.kind == 8).count() <= 2);
            assert!(start.pieces.iter().filter(|p| p.kind == 11).count() <= 4);
            assert!(start.pieces.iter().filter(|p| p.kind == 12).all(|p| p.component.depth > 5));
        }
    }
}
