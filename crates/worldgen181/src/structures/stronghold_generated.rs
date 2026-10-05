//! Literal placement bodies generated from the local Beta 1.8.1 source.
#![allow(unused_variables, unused_mut, unused_parens)]
use super::*;
impl StrongholdPiece {
    pub(super) fn generate_0(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        if self.component.liquids(w, clip) {
            return false;
        } else {
            self.component.fill_stones(
                w,
                clip,
                [0, 0, 0, 4, 4, 6],
                true,
                r,
                &mut BlockSelector::default(),
            );
            self.entrance(w, r, clip, self.door, 1, 1, 0);
            self.entrance(w, r, clip, Door::Opening, 1, 1, 6);
            self.component
                .random_put(w, clip, r, 0.1f32, 50, 0, 1, 2, 1);
            self.component
                .random_put(w, clip, r, 0.1f32, 50, 0, 3, 2, 1);
            self.component
                .random_put(w, clip, r, 0.1f32, 50, 0, 1, 2, 5);
            self.component
                .random_put(w, clip, r, 0.1f32, 50, 0, 3, 2, 5);
            if self.flags[0] {
                self.component
                    .fill(w, clip, [0, 1, 2, 0, 3, 4], 0, 0, false);
            }
            if self.flags[1] {
                self.component
                    .fill(w, clip, [4, 1, 2, 4, 3, 4], 0, 0, false);
            }
            return true;
        }
    }
    pub(super) fn generate_1(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        if self.component.liquids(w, clip) {
            return false;
        } else {
            self.component.fill_stones(
                w,
                clip,
                [0, 0, 0, 8, 4, 10],
                true,
                r,
                &mut BlockSelector::default(),
            );
            self.entrance(w, r, clip, self.door, 1, 1, 0);
            self.component
                .fill(w, clip, [1, 1, 10, 3, 3, 10], 0, 0, false);
            self.component.fill_stones(
                w,
                clip,
                [4, 1, 1, 4, 3, 1],
                false,
                r,
                &mut BlockSelector::default(),
            );
            self.component.fill_stones(
                w,
                clip,
                [4, 1, 3, 4, 3, 3],
                false,
                r,
                &mut BlockSelector::default(),
            );
            self.component.fill_stones(
                w,
                clip,
                [4, 1, 7, 4, 3, 7],
                false,
                r,
                &mut BlockSelector::default(),
            );
            self.component.fill_stones(
                w,
                clip,
                [4, 1, 9, 4, 3, 9],
                false,
                r,
                &mut BlockSelector::default(),
            );
            self.component
                .fill(w, clip, [4, 1, 4, 4, 3, 6], 101, 101, false);
            self.component
                .fill(w, clip, [5, 1, 5, 7, 3, 5], 101, 101, false);
            self.component.put(w, clip, 101, 0, 4, 3, 2);
            self.component.put(w, clip, 101, 0, 4, 3, 8);
            self.component
                .put(w, clip, 71, self.component.rotate(71, 3), 4, 1, 2);
            self.component
                .put(w, clip, 71, self.component.rotate(71, 3) + 8, 4, 2, 2);
            self.component
                .put(w, clip, 71, self.component.rotate(71, 3), 4, 1, 8);
            self.component
                .put(w, clip, 71, self.component.rotate(71, 3) + 8, 4, 2, 8);
            return true;
        }
    }
    pub(super) fn generate_2(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        if self.component.liquids(w, clip) {
            return false;
        } else {
            self.component.fill_stones(
                w,
                clip,
                [0, 0, 0, 4, 4, 4],
                true,
                r,
                &mut BlockSelector::default(),
            );
            self.entrance(w, r, clip, self.door, 1, 1, 0);
            if self.component.orientation != 2 && self.component.orientation != 3 {
                self.component
                    .fill(w, clip, [4, 1, 1, 4, 3, 3], 0, 0, false);
            } else {
                self.component
                    .fill(w, clip, [0, 1, 1, 0, 3, 3], 0, 0, false);
            }
            return true;
        }
    }
    pub(super) fn generate_3(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        if self.component.liquids(w, clip) {
            return false;
        } else {
            self.component.fill_stones(
                w,
                clip,
                [0, 0, 0, 4, 4, 4],
                true,
                r,
                &mut BlockSelector::default(),
            );
            self.entrance(w, r, clip, self.door, 1, 1, 0);
            if self.component.orientation != 2 && self.component.orientation != 3 {
                self.component
                    .fill(w, clip, [0, 1, 1, 0, 3, 3], 0, 0, false);
            } else {
                self.component
                    .fill(w, clip, [4, 1, 1, 4, 3, 3], 0, 0, false);
            }
            return true;
        }
    }
    pub(super) fn generate_4(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        if self.component.liquids(w, clip) {
            return false;
        } else {
            self.component.fill_stones(
                w,
                clip,
                [0, 0, 0, 10, 6, 10],
                true,
                r,
                &mut BlockSelector::default(),
            );
            self.entrance(w, r, clip, self.door, 4, 1, 0);
            self.component
                .fill(w, clip, [4, 1, 10, 6, 3, 10], 0, 0, false);
            self.component
                .fill(w, clip, [0, 1, 4, 0, 3, 6], 0, 0, false);
            self.component
                .fill(w, clip, [10, 1, 4, 10, 3, 6], 0, 0, false);
            let mut var4: i32;
            if self.style == 0 {
                self.component.put(w, clip, 98, 0, 5, 1, 5);
                self.component.put(w, clip, 98, 0, 5, 2, 5);
                self.component.put(w, clip, 98, 0, 5, 3, 5);
                self.component.put(w, clip, 50, 0, 4, 3, 5);
                self.component.put(w, clip, 50, 0, 6, 3, 5);
                self.component.put(w, clip, 50, 0, 5, 3, 4);
                self.component.put(w, clip, 50, 0, 5, 3, 6);
                self.component.put(w, clip, 44, 0, 4, 1, 4);
                self.component.put(w, clip, 44, 0, 4, 1, 5);
                self.component.put(w, clip, 44, 0, 4, 1, 6);
                self.component.put(w, clip, 44, 0, 6, 1, 4);
                self.component.put(w, clip, 44, 0, 6, 1, 5);
                self.component.put(w, clip, 44, 0, 6, 1, 6);
                self.component.put(w, clip, 44, 0, 5, 1, 4);
                self.component.put(w, clip, 44, 0, 5, 1, 6);
            } else if self.style == 1 {
                for var4 in (0)..(5) {
                    self.component.put(w, clip, 98, 0, 3, 1, 3 + var4);
                    self.component.put(w, clip, 98, 0, 7, 1, 3 + var4);
                    self.component.put(w, clip, 98, 0, 3 + var4, 1, 3);
                    self.component.put(w, clip, 98, 0, 3 + var4, 1, 7);
                }
                self.component.put(w, clip, 98, 0, 5, 1, 5);
                self.component.put(w, clip, 98, 0, 5, 2, 5);
                self.component.put(w, clip, 98, 0, 5, 3, 5);
                self.component.put(w, clip, 8, 0, 5, 4, 5);
            } else if self.style == 2 {
                for var4 in (1)..=(9) {
                    self.component.put(w, clip, 4, 0, 1, 3, var4);
                    self.component.put(w, clip, 4, 0, 9, 3, var4);
                }
                for var4 in (1)..=(9) {
                    self.component.put(w, clip, 4, 0, var4, 3, 1);
                    self.component.put(w, clip, 4, 0, var4, 3, 9);
                }
                self.component.put(w, clip, 4, 0, 5, 1, 4);
                self.component.put(w, clip, 4, 0, 5, 1, 6);
                self.component.put(w, clip, 4, 0, 5, 3, 4);
                self.component.put(w, clip, 4, 0, 5, 3, 6);
                self.component.put(w, clip, 4, 0, 4, 1, 5);
                self.component.put(w, clip, 4, 0, 6, 1, 5);
                self.component.put(w, clip, 4, 0, 4, 3, 5);
                self.component.put(w, clip, 4, 0, 6, 3, 5);
                for var4 in (1)..=(3) {
                    self.component.put(w, clip, 4, 0, 4, var4, 4);
                    self.component.put(w, clip, 4, 0, 6, var4, 4);
                    self.component.put(w, clip, 4, 0, 4, var4, 6);
                    self.component.put(w, clip, 4, 0, 6, var4, 6);
                }
                self.component.put(w, clip, 50, 0, 5, 3, 5);
                for var4 in (2)..=(8) {
                    self.component.put(w, clip, 5, 0, 2, 3, var4);
                    self.component.put(w, clip, 5, 0, 3, 3, var4);
                    if var4 <= 3 || var4 >= 7 {
                        self.component.put(w, clip, 5, 0, 4, 3, var4);
                        self.component.put(w, clip, 5, 0, 5, 3, var4);
                        self.component.put(w, clip, 5, 0, 6, 3, var4);
                    }
                    self.component.put(w, clip, 5, 0, 7, 3, var4);
                    self.component.put(w, clip, 5, 0, 8, 3, var4);
                }
                self.component
                    .put(w, clip, 65, self.component.rotate(65, 4), 9, 1, 3);
                self.component
                    .put(w, clip, 65, self.component.rotate(65, 4), 9, 2, 3);
                self.component
                    .put(w, clip, 65, self.component.rotate(65, 4), 9, 3, 3);
                let loot_count = 1 + component::random(w, r).next_int(4);
                self.chest(w, clip, r, 3, 4, 8, &ROOM_LOOT, loot_count);
            }
            return true;
        }
    }
    pub(super) fn generate_5(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        if self.component.liquids(w, clip) {
            return false;
        } else {
            self.component.fill_stones(
                w,
                clip,
                [0, 0, 0, 4, 10, 7],
                true,
                r,
                &mut BlockSelector::default(),
            );
            self.entrance(w, r, clip, self.door, 1, 7, 0);
            self.entrance(w, r, clip, Door::Opening, 1, 1, 7);
            let mut var4: i32 = self.component.rotate(67, 2);
            for var5 in (0)..(6) {
                self.component.put(w, clip, 67, var4, 1, 6 - var5, 1 + var5);
                self.component.put(w, clip, 67, var4, 2, 6 - var5, 1 + var5);
                self.component.put(w, clip, 67, var4, 3, 6 - var5, 1 + var5);
                if var5 < 5 {
                    self.component.put(w, clip, 98, 0, 1, 5 - var5, 1 + var5);
                    self.component.put(w, clip, 98, 0, 2, 5 - var5, 1 + var5);
                    self.component.put(w, clip, 98, 0, 3, 5 - var5, 1 + var5);
                }
            }
            return true;
        }
    }
    pub(super) fn generate_6(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        if self.component.liquids(w, clip) {
            return false;
        } else {
            if self.flags[0] {}
            self.component.fill_stones(
                w,
                clip,
                [0, 0, 0, 4, 10, 4],
                true,
                r,
                &mut BlockSelector::default(),
            );
            self.entrance(w, r, clip, self.door, 1, 7, 0);
            self.entrance(w, r, clip, Door::Opening, 1, 1, 4);
            self.component.put(w, clip, 98, 0, 2, 6, 1);
            self.component.put(w, clip, 98, 0, 1, 5, 1);
            self.component.put(w, clip, 44, 0, 1, 6, 1);
            self.component.put(w, clip, 98, 0, 1, 5, 2);
            self.component.put(w, clip, 98, 0, 1, 4, 3);
            self.component.put(w, clip, 44, 0, 1, 5, 3);
            self.component.put(w, clip, 98, 0, 2, 4, 3);
            self.component.put(w, clip, 98, 0, 3, 3, 3);
            self.component.put(w, clip, 44, 0, 3, 4, 3);
            self.component.put(w, clip, 98, 0, 3, 3, 2);
            self.component.put(w, clip, 98, 0, 3, 2, 1);
            self.component.put(w, clip, 44, 0, 3, 3, 1);
            self.component.put(w, clip, 98, 0, 2, 2, 1);
            self.component.put(w, clip, 98, 0, 1, 1, 1);
            self.component.put(w, clip, 44, 0, 1, 2, 1);
            self.component.put(w, clip, 98, 0, 1, 1, 2);
            self.component.put(w, clip, 44, 0, 1, 1, 3);
            return true;
        }
    }
    pub(super) fn generate_7(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        if self.component.liquids(w, clip) {
            return false;
        } else {
            self.component.fill_stones(
                w,
                clip,
                [0, 0, 0, 9, 8, 10],
                true,
                r,
                &mut BlockSelector::default(),
            );
            self.entrance(w, r, clip, self.door, 4, 3, 0);
            if self.flags[0] {
                self.component
                    .fill(w, clip, [0, 3, 1, 0, 5, 3], 0, 0, false);
            }
            if self.flags[2] {
                self.component
                    .fill(w, clip, [9, 3, 1, 9, 5, 3], 0, 0, false);
            }
            if self.flags[1] {
                self.component
                    .fill(w, clip, [0, 5, 7, 0, 7, 9], 0, 0, false);
            }
            if self.flags[3] {
                self.component
                    .fill(w, clip, [9, 5, 7, 9, 7, 9], 0, 0, false);
            }
            self.component
                .fill(w, clip, [5, 1, 10, 7, 3, 10], 0, 0, false);
            self.component.fill_stones(
                w,
                clip,
                [1, 2, 1, 8, 2, 6],
                false,
                r,
                &mut BlockSelector::default(),
            );
            self.component.fill_stones(
                w,
                clip,
                [4, 1, 5, 4, 4, 9],
                false,
                r,
                &mut BlockSelector::default(),
            );
            self.component.fill_stones(
                w,
                clip,
                [8, 1, 5, 8, 4, 9],
                false,
                r,
                &mut BlockSelector::default(),
            );
            self.component.fill_stones(
                w,
                clip,
                [1, 4, 7, 3, 4, 9],
                false,
                r,
                &mut BlockSelector::default(),
            );
            self.component.fill_stones(
                w,
                clip,
                [1, 3, 5, 3, 3, 6],
                false,
                r,
                &mut BlockSelector::default(),
            );
            self.component
                .fill(w, clip, [1, 3, 4, 3, 3, 4], 44, 44, false);
            self.component
                .fill(w, clip, [1, 4, 6, 3, 4, 6], 44, 44, false);
            self.component.fill_stones(
                w,
                clip,
                [5, 1, 7, 7, 1, 8],
                false,
                r,
                &mut BlockSelector::default(),
            );
            self.component
                .fill(w, clip, [5, 1, 9, 7, 1, 9], 44, 44, false);
            self.component
                .fill(w, clip, [5, 2, 7, 7, 2, 7], 44, 44, false);
            self.component
                .fill(w, clip, [4, 5, 7, 4, 5, 9], 44, 44, false);
            self.component
                .fill(w, clip, [8, 5, 7, 8, 5, 9], 44, 44, false);
            self.component
                .fill(w, clip, [5, 5, 7, 7, 5, 9], 43, 43, false);
            self.component.put(w, clip, 50, 0, 6, 5, 6);
            return true;
        }
    }
    pub(super) fn generate_8(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        if self.component.liquids(w, clip) {
            return false;
        } else {
            let mut var4: i32 = 11;
            if !self.flags[0] {
                var4 = 6;
            }
            self.component.fill_stones(
                w,
                clip,
                [0, 0, 0, 13, var4 - 1, 14],
                true,
                r,
                &mut BlockSelector::default(),
            );
            self.entrance(w, r, clip, self.door, 4, 1, 0);
            self.component
                .random_fill(w, clip, r, 0.07f32, [2, 1, 1, 11, 4, 13], 30, 30, false);
            let mut var7: i32;
            for var7 in (1)..=(13) {
                if (var7 - 1) % 4 == 0 {
                    self.component
                        .fill(w, clip, [1, 1, var7, 1, 4, var7], 5, 5, false);
                    self.component
                        .fill(w, clip, [12, 1, var7, 12, 4, var7], 5, 5, false);
                    self.component.put(w, clip, 50, 0, 2, 3, var7);
                    self.component.put(w, clip, 50, 0, 11, 3, var7);
                    if self.flags[0] {
                        self.component
                            .fill(w, clip, [1, 6, var7, 1, 9, var7], 5, 5, false);
                        self.component
                            .fill(w, clip, [12, 6, var7, 12, 9, var7], 5, 5, false);
                    }
                } else {
                    self.component
                        .fill(w, clip, [1, 1, var7, 1, 4, var7], 47, 47, false);
                    self.component
                        .fill(w, clip, [12, 1, var7, 12, 4, var7], 47, 47, false);
                    if self.flags[0] {
                        self.component
                            .fill(w, clip, [1, 6, var7, 1, 9, var7], 47, 47, false);
                        self.component
                            .fill(w, clip, [12, 6, var7, 12, 9, var7], 47, 47, false);
                    }
                }
            }
            for var7 in ((3)..(12)).step_by(2) {
                self.component
                    .fill(w, clip, [3, 1, var7, 4, 3, var7], 47, 47, false);
                self.component
                    .fill(w, clip, [6, 1, var7, 7, 3, var7], 47, 47, false);
                self.component
                    .fill(w, clip, [9, 1, var7, 10, 3, var7], 47, 47, false);
            }
            if self.flags[0] {
                self.component
                    .fill(w, clip, [1, 5, 1, 3, 5, 13], 5, 5, false);
                self.component
                    .fill(w, clip, [10, 5, 1, 12, 5, 13], 5, 5, false);
                self.component
                    .fill(w, clip, [4, 5, 1, 9, 5, 2], 5, 5, false);
                self.component
                    .fill(w, clip, [4, 5, 12, 9, 5, 13], 5, 5, false);
                self.component.put(w, clip, 5, 0, 9, 5, 11);
                self.component.put(w, clip, 5, 0, 8, 5, 11);
                self.component.put(w, clip, 5, 0, 9, 5, 10);
                self.component
                    .fill(w, clip, [3, 6, 2, 3, 6, 12], 85, 85, false);
                self.component
                    .fill(w, clip, [10, 6, 2, 10, 6, 10], 85, 85, false);
                self.component
                    .fill(w, clip, [4, 6, 2, 9, 6, 2], 85, 85, false);
                self.component
                    .fill(w, clip, [4, 6, 12, 8, 6, 12], 85, 85, false);
                self.component.put(w, clip, 85, 0, 9, 6, 11);
                self.component.put(w, clip, 85, 0, 8, 6, 11);
                self.component.put(w, clip, 85, 0, 9, 6, 10);
                var7 = self.component.rotate(65, 3);
                self.component.put(w, clip, 65, var7, 10, 1, 13);
                self.component.put(w, clip, 65, var7, 10, 2, 13);
                self.component.put(w, clip, 65, var7, 10, 3, 13);
                self.component.put(w, clip, 65, var7, 10, 4, 13);
                self.component.put(w, clip, 65, var7, 10, 5, 13);
                self.component.put(w, clip, 65, var7, 10, 6, 13);
                self.component.put(w, clip, 65, var7, 10, 7, 13);
                let mut var8: i32 = 7;
                let mut var9: i32 = 7;
                self.component.put(w, clip, 85, 0, var8 - 1, 9, var9);
                self.component.put(w, clip, 85, 0, var8, 9, var9);
                self.component.put(w, clip, 85, 0, var8 - 1, 8, var9);
                self.component.put(w, clip, 85, 0, var8, 8, var9);
                self.component.put(w, clip, 85, 0, var8 - 1, 7, var9);
                self.component.put(w, clip, 85, 0, var8, 7, var9);
                self.component.put(w, clip, 85, 0, var8 - 2, 7, var9);
                self.component.put(w, clip, 85, 0, var8 + 1, 7, var9);
                self.component.put(w, clip, 85, 0, var8 - 1, 7, var9 - 1);
                self.component.put(w, clip, 85, 0, var8 - 1, 7, var9 + 1);
                self.component.put(w, clip, 85, 0, var8, 7, var9 - 1);
                self.component.put(w, clip, 85, 0, var8, 7, var9 + 1);
                self.component.put(w, clip, 50, 0, var8 - 2, 8, var9);
                self.component.put(w, clip, 50, 0, var8 + 1, 8, var9);
                self.component.put(w, clip, 50, 0, var8 - 1, 8, var9 - 1);
                self.component.put(w, clip, 50, 0, var8 - 1, 8, var9 + 1);
                self.component.put(w, clip, 50, 0, var8, 8, var9 - 1);
                self.component.put(w, clip, 50, 0, var8, 8, var9 + 1);
            }
            let loot_count = 1 + component::random(w, r).next_int(4);
            self.chest(w, clip, r, 3, 3, 5, &LIBRARY_LOOT, loot_count);
            if self.flags[0] {
                self.component.put(w, clip, 0, 0, 12, 9, 1);
                let loot_count = 1 + component::random(w, r).next_int(4);
                self.chest(w, clip, r, 12, 8, 1, &LIBRARY_LOOT, loot_count);
            }
            return true;
        }
    }
    pub(super) fn generate_9(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        if self.component.liquids(w, clip) {
            return false;
        } else {
            for var4 in (0)..(self.length) {
                self.component.put(w, clip, 98, 0, 0, 0, var4);
                self.component.put(w, clip, 98, 0, 1, 0, var4);
                self.component.put(w, clip, 98, 0, 2, 0, var4);
                self.component.put(w, clip, 98, 0, 3, 0, var4);
                self.component.put(w, clip, 98, 0, 4, 0, var4);
                for var5 in (1)..=(3) {
                    self.component.put(w, clip, 98, 0, 0, var5, var4);
                    self.component.put(w, clip, 0, 0, 1, var5, var4);
                    self.component.put(w, clip, 0, 0, 2, var5, var4);
                    self.component.put(w, clip, 0, 0, 3, var5, var4);
                    self.component.put(w, clip, 98, 0, 4, var5, var4);
                }
                self.component.put(w, clip, 98, 0, 0, 4, var4);
                self.component.put(w, clip, 98, 0, 1, 4, var4);
                self.component.put(w, clip, 98, 0, 2, 4, var4);
                self.component.put(w, clip, 98, 0, 3, 4, var4);
                self.component.put(w, clip, 98, 0, 4, 4, var4);
            }
            return true;
        }
    }
}
