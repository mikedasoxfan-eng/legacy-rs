//! Literal Release 1.0 fortress placement bodies from the local MCP source.
#![allow(unused_variables, unused_mut, unused_parens, unused_assignments)]
use super::*;
impl NetherPiece {
    pub(super) fn generate_0(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        self.component
            .fill(w, clip, [0, 3, 0, 4, 4, 18], 112, 112, false);
        self.component
            .fill(w, clip, [1, 5, 0, 3, 7, 18], 0, 0, false);
        self.component
            .fill(w, clip, [0, 5, 0, 0, 5, 18], 112, 112, false);
        self.component
            .fill(w, clip, [4, 5, 0, 4, 5, 18], 112, 112, false);
        self.component
            .fill(w, clip, [0, 2, 0, 4, 2, 5], 112, 112, false);
        self.component
            .fill(w, clip, [0, 2, 13, 4, 2, 18], 112, 112, false);
        self.component
            .fill(w, clip, [0, 0, 0, 4, 1, 3], 112, 112, false);
        self.component
            .fill(w, clip, [0, 0, 15, 4, 1, 18], 112, 112, false);
        for var4 in (0)..=(4) {
            for var5 in (0)..=(2) {
                self.component.fill_down(w, clip, 112, 0, var4, -1, var5);
                self.component
                    .fill_down(w, clip, 112, 0, var4, -1, 18 - var5);
            }
        }
        self.component
            .fill(w, clip, [0, 1, 1, 0, 4, 1], 113, 113, false);
        self.component
            .fill(w, clip, [0, 3, 4, 0, 4, 4], 113, 113, false);
        self.component
            .fill(w, clip, [0, 3, 14, 0, 4, 14], 113, 113, false);
        self.component
            .fill(w, clip, [0, 1, 17, 0, 4, 17], 113, 113, false);
        self.component
            .fill(w, clip, [4, 1, 1, 4, 4, 1], 113, 113, false);
        self.component
            .fill(w, clip, [4, 3, 4, 4, 4, 4], 113, 113, false);
        self.component
            .fill(w, clip, [4, 3, 14, 4, 4, 14], 113, 113, false);
        self.component
            .fill(w, clip, [4, 1, 17, 4, 4, 17], 113, 113, false);
        return true;
    }
    pub(super) fn generate_1(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        self.component
            .fill(w, clip, [7, 3, 0, 11, 4, 18], 112, 112, false);
        self.component
            .fill(w, clip, [0, 3, 7, 18, 4, 11], 112, 112, false);
        self.component
            .fill(w, clip, [8, 5, 0, 10, 7, 18], 0, 0, false);
        self.component
            .fill(w, clip, [0, 5, 8, 18, 7, 10], 0, 0, false);
        self.component
            .fill(w, clip, [7, 5, 0, 7, 5, 7], 112, 112, false);
        self.component
            .fill(w, clip, [7, 5, 11, 7, 5, 18], 112, 112, false);
        self.component
            .fill(w, clip, [11, 5, 0, 11, 5, 7], 112, 112, false);
        self.component
            .fill(w, clip, [11, 5, 11, 11, 5, 18], 112, 112, false);
        self.component
            .fill(w, clip, [0, 5, 7, 7, 5, 7], 112, 112, false);
        self.component
            .fill(w, clip, [11, 5, 7, 18, 5, 7], 112, 112, false);
        self.component
            .fill(w, clip, [0, 5, 11, 7, 5, 11], 112, 112, false);
        self.component
            .fill(w, clip, [11, 5, 11, 18, 5, 11], 112, 112, false);
        self.component
            .fill(w, clip, [7, 2, 0, 11, 2, 5], 112, 112, false);
        self.component
            .fill(w, clip, [7, 2, 13, 11, 2, 18], 112, 112, false);
        self.component
            .fill(w, clip, [7, 0, 0, 11, 1, 3], 112, 112, false);
        self.component
            .fill(w, clip, [7, 0, 15, 11, 1, 18], 112, 112, false);
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        for var4 in (7)..=(11) {
            for var5 in (0)..=(2) {
                self.component.fill_down(w, clip, 112, 0, var4, -1, var5);
                self.component
                    .fill_down(w, clip, 112, 0, var4, -1, 18 - var5);
            }
        }
        self.component
            .fill(w, clip, [0, 2, 7, 5, 2, 11], 112, 112, false);
        self.component
            .fill(w, clip, [13, 2, 7, 18, 2, 11], 112, 112, false);
        self.component
            .fill(w, clip, [0, 0, 7, 3, 1, 11], 112, 112, false);
        self.component
            .fill(w, clip, [15, 0, 7, 18, 1, 11], 112, 112, false);
        for var4 in (0)..=(2) {
            for var5 in (7)..=(11) {
                self.component.fill_down(w, clip, 112, 0, var4, -1, var5);
                self.component
                    .fill_down(w, clip, 112, 0, 18 - var4, -1, var5);
            }
        }
        return true;
    }
    pub(super) fn generate_2(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        self.component
            .fill(w, clip, [0, 0, 0, 6, 1, 6], 112, 112, false);
        self.component
            .fill(w, clip, [0, 2, 0, 6, 7, 6], 0, 0, false);
        self.component
            .fill(w, clip, [0, 2, 0, 1, 6, 0], 112, 112, false);
        self.component
            .fill(w, clip, [0, 2, 6, 1, 6, 6], 112, 112, false);
        self.component
            .fill(w, clip, [5, 2, 0, 6, 6, 0], 112, 112, false);
        self.component
            .fill(w, clip, [5, 2, 6, 6, 6, 6], 112, 112, false);
        self.component
            .fill(w, clip, [0, 2, 0, 0, 6, 1], 112, 112, false);
        self.component
            .fill(w, clip, [0, 2, 5, 0, 6, 6], 112, 112, false);
        self.component
            .fill(w, clip, [6, 2, 0, 6, 6, 1], 112, 112, false);
        self.component
            .fill(w, clip, [6, 2, 5, 6, 6, 6], 112, 112, false);
        self.component
            .fill(w, clip, [2, 6, 0, 4, 6, 0], 112, 112, false);
        self.component
            .fill(w, clip, [2, 5, 0, 4, 5, 0], 113, 113, false);
        self.component
            .fill(w, clip, [2, 6, 6, 4, 6, 6], 112, 112, false);
        self.component
            .fill(w, clip, [2, 5, 6, 4, 5, 6], 113, 113, false);
        self.component
            .fill(w, clip, [0, 6, 2, 0, 6, 4], 112, 112, false);
        self.component
            .fill(w, clip, [0, 5, 2, 0, 5, 4], 113, 113, false);
        self.component
            .fill(w, clip, [6, 6, 2, 6, 6, 4], 112, 112, false);
        self.component
            .fill(w, clip, [6, 5, 2, 6, 5, 4], 113, 113, false);
        for var4 in (0)..=(6) {
            for var5 in (0)..=(6) {
                self.component.fill_down(w, clip, 112, 0, var4, -1, var5);
            }
        }
        return true;
    }
    pub(super) fn generate_3(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        self.component
            .fill(w, clip, [0, 0, 0, 6, 1, 6], 112, 112, false);
        self.component
            .fill(w, clip, [0, 2, 0, 6, 10, 6], 0, 0, false);
        self.component
            .fill(w, clip, [0, 2, 0, 1, 8, 0], 112, 112, false);
        self.component
            .fill(w, clip, [5, 2, 0, 6, 8, 0], 112, 112, false);
        self.component
            .fill(w, clip, [0, 2, 1, 0, 8, 6], 112, 112, false);
        self.component
            .fill(w, clip, [6, 2, 1, 6, 8, 6], 112, 112, false);
        self.component
            .fill(w, clip, [1, 2, 6, 5, 8, 6], 112, 112, false);
        self.component
            .fill(w, clip, [0, 3, 2, 0, 5, 4], 113, 113, false);
        self.component
            .fill(w, clip, [6, 3, 2, 6, 5, 2], 113, 113, false);
        self.component
            .fill(w, clip, [6, 3, 4, 6, 5, 4], 113, 113, false);
        self.component.put(w, clip, 112, 0, 5, 2, 5);
        self.component
            .fill(w, clip, [4, 2, 5, 4, 3, 5], 112, 112, false);
        self.component
            .fill(w, clip, [3, 2, 5, 3, 4, 5], 112, 112, false);
        self.component
            .fill(w, clip, [2, 2, 5, 2, 5, 5], 112, 112, false);
        self.component
            .fill(w, clip, [1, 2, 5, 1, 6, 5], 112, 112, false);
        self.component
            .fill(w, clip, [1, 7, 1, 5, 7, 4], 112, 112, false);
        self.component
            .fill(w, clip, [6, 8, 2, 6, 8, 4], 0, 0, false);
        self.component
            .fill(w, clip, [2, 6, 0, 4, 8, 0], 112, 112, false);
        self.component
            .fill(w, clip, [2, 5, 0, 4, 5, 0], 113, 113, false);
        for var4 in (0)..=(6) {
            for var5 in (0)..=(6) {
                self.component.fill_down(w, clip, 112, 0, var4, -1, var5);
            }
        }
        return true;
    }
    pub(super) fn generate_4(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        self.component
            .fill(w, clip, [0, 2, 0, 6, 7, 7], 0, 0, false);
        self.component
            .fill(w, clip, [1, 0, 0, 5, 1, 7], 112, 112, false);
        self.component
            .fill(w, clip, [1, 2, 1, 5, 2, 7], 112, 112, false);
        self.component
            .fill(w, clip, [1, 3, 2, 5, 3, 7], 112, 112, false);
        self.component
            .fill(w, clip, [1, 4, 3, 5, 4, 7], 112, 112, false);
        self.component
            .fill(w, clip, [1, 2, 0, 1, 4, 2], 112, 112, false);
        self.component
            .fill(w, clip, [5, 2, 0, 5, 4, 2], 112, 112, false);
        self.component
            .fill(w, clip, [1, 5, 2, 1, 5, 3], 112, 112, false);
        self.component
            .fill(w, clip, [5, 5, 2, 5, 5, 3], 112, 112, false);
        self.component
            .fill(w, clip, [0, 5, 3, 0, 5, 8], 112, 112, false);
        self.component
            .fill(w, clip, [6, 5, 3, 6, 5, 8], 112, 112, false);
        self.component
            .fill(w, clip, [1, 5, 8, 5, 5, 8], 112, 112, false);
        self.component.put(w, clip, 113, 0, 1, 6, 3);
        self.component.put(w, clip, 113, 0, 5, 6, 3);
        self.component
            .fill(w, clip, [0, 6, 3, 0, 6, 8], 113, 113, false);
        self.component
            .fill(w, clip, [6, 6, 3, 6, 6, 8], 113, 113, false);
        self.component
            .fill(w, clip, [1, 6, 8, 5, 7, 8], 113, 113, false);
        self.component
            .fill(w, clip, [2, 8, 8, 4, 8, 8], 113, 113, false);
        let mut var4: i32 = 0;
        let mut var5: i32 = 0;
        self.spawner10(w, clip, 3, 5, 5, "Blaze");
        for var4 in (0)..=(6) {
            for var5 in (0)..=(6) {
                self.component.fill_down(w, clip, 112, 0, var4, -1, var5);
            }
        }
        return true;
    }
    pub(super) fn generate_5(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        self.component
            .fill(w, clip, [0, 3, 0, 12, 4, 12], 112, 112, false);
        self.component
            .fill(w, clip, [0, 5, 0, 12, 13, 12], 0, 0, false);
        self.component
            .fill(w, clip, [0, 5, 0, 1, 12, 12], 112, 112, false);
        self.component
            .fill(w, clip, [11, 5, 0, 12, 12, 12], 112, 112, false);
        self.component
            .fill(w, clip, [2, 5, 11, 4, 12, 12], 112, 112, false);
        self.component
            .fill(w, clip, [8, 5, 11, 10, 12, 12], 112, 112, false);
        self.component
            .fill(w, clip, [5, 9, 11, 7, 12, 12], 112, 112, false);
        self.component
            .fill(w, clip, [2, 5, 0, 4, 12, 1], 112, 112, false);
        self.component
            .fill(w, clip, [8, 5, 0, 10, 12, 1], 112, 112, false);
        self.component
            .fill(w, clip, [5, 9, 0, 7, 12, 1], 112, 112, false);
        self.component
            .fill(w, clip, [2, 11, 2, 10, 12, 10], 112, 112, false);
        self.component
            .fill(w, clip, [5, 8, 0, 7, 8, 0], 113, 113, false);
        let mut var4: i32 = 0;
        for var4 in ((1)..=(11)).step_by(2) {
            self.component
                .fill(w, clip, [var4, 10, 0, var4, 11, 0], 113, 113, false);
            self.component
                .fill(w, clip, [var4, 10, 12, var4, 11, 12], 113, 113, false);
            self.component
                .fill(w, clip, [0, 10, var4, 0, 11, var4], 113, 113, false);
            self.component
                .fill(w, clip, [12, 10, var4, 12, 11, var4], 113, 113, false);
            self.component.put(w, clip, 112, 0, var4, 13, 0);
            self.component.put(w, clip, 112, 0, var4, 13, 12);
            self.component.put(w, clip, 112, 0, 0, 13, var4);
            self.component.put(w, clip, 112, 0, 12, 13, var4);
            self.component.put(w, clip, 113, 0, var4 + 1, 13, 0);
            self.component.put(w, clip, 113, 0, var4 + 1, 13, 12);
            self.component.put(w, clip, 113, 0, 0, 13, var4 + 1);
            self.component.put(w, clip, 113, 0, 12, 13, var4 + 1);
        }
        self.component.put(w, clip, 113, 0, 0, 13, 0);
        self.component.put(w, clip, 113, 0, 0, 13, 12);
        self.component.put(w, clip, 113, 0, 0, 13, 0);
        self.component.put(w, clip, 113, 0, 12, 13, 0);
        for var4 in ((3)..=(9)).step_by(2) {
            self.component
                .fill(w, clip, [1, 7, var4, 1, 8, var4], 113, 113, false);
            self.component
                .fill(w, clip, [11, 7, var4, 11, 8, var4], 113, 113, false);
        }
        self.component
            .fill(w, clip, [4, 2, 0, 8, 2, 12], 112, 112, false);
        self.component
            .fill(w, clip, [0, 2, 4, 12, 2, 8], 112, 112, false);
        self.component
            .fill(w, clip, [4, 0, 0, 8, 1, 3], 112, 112, false);
        self.component
            .fill(w, clip, [4, 0, 9, 8, 1, 12], 112, 112, false);
        self.component
            .fill(w, clip, [0, 0, 4, 3, 1, 8], 112, 112, false);
        self.component
            .fill(w, clip, [9, 0, 4, 12, 1, 8], 112, 112, false);
        let mut var5: i32 = 0;
        for var4 in (4)..=(8) {
            for var5 in (0)..=(2) {
                self.component.fill_down(w, clip, 112, 0, var4, -1, var5);
                self.component
                    .fill_down(w, clip, 112, 0, var4, -1, 12 - var5);
            }
        }
        for var4 in (0)..=(2) {
            for var5 in (4)..=(8) {
                self.component.fill_down(w, clip, 112, 0, var4, -1, var5);
                self.component
                    .fill_down(w, clip, 112, 0, 12 - var4, -1, var5);
            }
        }
        self.component
            .fill(w, clip, [5, 5, 5, 7, 5, 7], 112, 112, false);
        self.component
            .fill(w, clip, [6, 1, 6, 6, 4, 6], 0, 0, false);
        self.component.put(w, clip, 112, 0, 6, 0, 6);
        self.component.put(w, clip, 10, 0, 6, 5, 6);
        self.lava10(w, r, clip);
        return true;
    }
    pub(super) fn generate_6(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        self.component
            .fill(w, clip, [0, 0, 0, 4, 1, 4], 112, 112, false);
        self.component
            .fill(w, clip, [0, 2, 0, 4, 5, 4], 0, 0, false);
        self.component
            .fill(w, clip, [0, 2, 0, 0, 5, 4], 112, 112, false);
        self.component
            .fill(w, clip, [4, 2, 0, 4, 5, 4], 112, 112, false);
        self.component
            .fill(w, clip, [0, 3, 1, 0, 4, 1], 113, 113, false);
        self.component
            .fill(w, clip, [0, 3, 3, 0, 4, 3], 113, 113, false);
        self.component
            .fill(w, clip, [4, 3, 1, 4, 4, 1], 113, 113, false);
        self.component
            .fill(w, clip, [4, 3, 3, 4, 4, 3], 113, 113, false);
        self.component
            .fill(w, clip, [0, 6, 0, 4, 6, 4], 112, 112, false);
        for var4 in (0)..=(4) {
            for var5 in (0)..=(4) {
                self.component.fill_down(w, clip, 112, 0, var4, -1, var5);
            }
        }
        return true;
    }
    pub(super) fn generate_7(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        self.component
            .fill(w, clip, [0, 0, 0, 4, 1, 4], 112, 112, false);
        self.component
            .fill(w, clip, [0, 2, 0, 4, 5, 4], 0, 0, false);
        self.component
            .fill(w, clip, [0, 2, 0, 0, 5, 0], 112, 112, false);
        self.component
            .fill(w, clip, [4, 2, 0, 4, 5, 0], 112, 112, false);
        self.component
            .fill(w, clip, [0, 2, 4, 0, 5, 4], 112, 112, false);
        self.component
            .fill(w, clip, [4, 2, 4, 4, 5, 4], 112, 112, false);
        self.component
            .fill(w, clip, [0, 6, 0, 4, 6, 4], 112, 112, false);
        for var4 in (0)..=(4) {
            for var5 in (0)..=(4) {
                self.component.fill_down(w, clip, 112, 0, var4, -1, var5);
            }
        }
        return true;
    }
    pub(super) fn generate_8(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        self.component
            .fill(w, clip, [0, 0, 0, 4, 1, 4], 112, 112, false);
        self.component
            .fill(w, clip, [0, 2, 0, 4, 5, 4], 0, 0, false);
        self.component
            .fill(w, clip, [0, 2, 0, 0, 5, 4], 112, 112, false);
        self.component
            .fill(w, clip, [0, 3, 1, 0, 4, 1], 113, 113, false);
        self.component
            .fill(w, clip, [0, 3, 3, 0, 4, 3], 113, 113, false);
        self.component
            .fill(w, clip, [4, 2, 0, 4, 5, 0], 112, 112, false);
        self.component
            .fill(w, clip, [1, 2, 4, 4, 5, 4], 112, 112, false);
        self.component
            .fill(w, clip, [1, 3, 4, 1, 4, 4], 113, 112, false);
        self.component
            .fill(w, clip, [3, 3, 4, 3, 4, 4], 113, 112, false);
        self.component
            .fill(w, clip, [0, 6, 0, 4, 6, 4], 112, 112, false);
        for var4 in (0)..=(4) {
            for var5 in (0)..=(4) {
                self.component.fill_down(w, clip, 112, 0, var4, -1, var5);
            }
        }
        return true;
    }
    pub(super) fn generate_9(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        self.component
            .fill(w, clip, [0, 0, 0, 4, 1, 4], 112, 112, false);
        self.component
            .fill(w, clip, [0, 2, 0, 4, 5, 4], 0, 0, false);
        self.component
            .fill(w, clip, [4, 2, 0, 4, 5, 4], 112, 112, false);
        self.component
            .fill(w, clip, [4, 3, 1, 4, 4, 1], 113, 113, false);
        self.component
            .fill(w, clip, [4, 3, 3, 4, 4, 3], 113, 113, false);
        self.component
            .fill(w, clip, [0, 2, 0, 0, 5, 0], 112, 112, false);
        self.component
            .fill(w, clip, [0, 2, 4, 3, 5, 4], 112, 112, false);
        self.component
            .fill(w, clip, [1, 3, 4, 1, 4, 4], 113, 112, false);
        self.component
            .fill(w, clip, [3, 3, 4, 3, 4, 4], 113, 112, false);
        self.component
            .fill(w, clip, [0, 6, 0, 4, 6, 4], 112, 112, false);
        for var4 in (0)..=(4) {
            for var5 in (0)..=(4) {
                self.component.fill_down(w, clip, 112, 0, var4, -1, var5);
            }
        }
        return true;
    }
    pub(super) fn generate_10(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        let mut var4: i32 = self.component.rotate(114, 2);
        for var5 in (0)..=(9) {
            let mut var6: i32 = max(1, 7 - var5);
            let mut var7: i32 = min(max(var6 + 5, 14 - var5), 13);
            let mut var8: i32 = var5;
            self.component
                .fill(w, clip, [0, 0, var5, 4, var6, var5], 112, 112, false);
            self.component
                .fill(w, clip, [1, var6 + 1, var5, 3, var7 - 1, var5], 0, 0, false);
            if var5 <= 6 {
                self.component.put(w, clip, 114, var4, 1, var6 + 1, var5);
                self.component.put(w, clip, 114, var4, 2, var6 + 1, var5);
                self.component.put(w, clip, 114, var4, 3, var6 + 1, var5);
            }
            self.component
                .fill(w, clip, [0, var7, var5, 4, var7, var5], 112, 112, false);
            self.component.fill(
                w,
                clip,
                [0, var6 + 1, var5, 0, var7 - 1, var5],
                112,
                112,
                false,
            );
            self.component.fill(
                w,
                clip,
                [4, var6 + 1, var5, 4, var7 - 1, var5],
                112,
                112,
                false,
            );
            if (var5 & 1) == 0 {
                self.component.fill(
                    w,
                    clip,
                    [0, var6 + 2, var5, 0, var6 + 3, var5],
                    113,
                    113,
                    false,
                );
                self.component.fill(
                    w,
                    clip,
                    [4, var6 + 2, var5, 4, var6 + 3, var5],
                    113,
                    113,
                    false,
                );
            }
            for var9 in (0)..=(4) {
                self.component.fill_down(w, clip, 112, 0, var9, -1, var8);
            }
        }
        return true;
    }
    pub(super) fn generate_11(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        self.component
            .fill(w, clip, [0, 0, 0, 8, 1, 8], 112, 112, false);
        self.component
            .fill(w, clip, [0, 2, 0, 8, 5, 8], 0, 0, false);
        self.component
            .fill(w, clip, [0, 6, 0, 8, 6, 5], 112, 112, false);
        self.component
            .fill(w, clip, [0, 2, 0, 2, 5, 0], 112, 112, false);
        self.component
            .fill(w, clip, [6, 2, 0, 8, 5, 0], 112, 112, false);
        self.component
            .fill(w, clip, [1, 3, 0, 1, 4, 0], 113, 113, false);
        self.component
            .fill(w, clip, [7, 3, 0, 7, 4, 0], 113, 113, false);
        self.component
            .fill(w, clip, [0, 2, 4, 8, 2, 8], 112, 112, false);
        self.component
            .fill(w, clip, [1, 1, 4, 2, 2, 4], 0, 0, false);
        self.component
            .fill(w, clip, [6, 1, 4, 7, 2, 4], 0, 0, false);
        self.component
            .fill(w, clip, [0, 3, 8, 8, 3, 8], 113, 113, false);
        self.component
            .fill(w, clip, [0, 3, 6, 0, 3, 7], 113, 113, false);
        self.component
            .fill(w, clip, [8, 3, 6, 8, 3, 7], 113, 113, false);
        self.component
            .fill(w, clip, [0, 3, 4, 0, 5, 5], 112, 112, false);
        self.component
            .fill(w, clip, [8, 3, 4, 8, 5, 5], 112, 112, false);
        self.component
            .fill(w, clip, [1, 3, 5, 2, 5, 5], 112, 112, false);
        self.component
            .fill(w, clip, [6, 3, 5, 7, 5, 5], 112, 112, false);
        self.component
            .fill(w, clip, [1, 4, 5, 1, 5, 5], 113, 113, false);
        self.component
            .fill(w, clip, [7, 4, 5, 7, 5, 5], 113, 113, false);
        for var4 in (0)..=(5) {
            for var5 in (0)..=(8) {
                self.component.fill_down(w, clip, 112, 0, var5, -1, var4);
            }
        }
        return true;
    }
    pub(super) fn generate_12(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        self.component
            .fill(w, clip, [0, 3, 0, 12, 4, 12], 112, 112, false);
        self.component
            .fill(w, clip, [0, 5, 0, 12, 13, 12], 0, 0, false);
        self.component
            .fill(w, clip, [0, 5, 0, 1, 12, 12], 112, 112, false);
        self.component
            .fill(w, clip, [11, 5, 0, 12, 12, 12], 112, 112, false);
        self.component
            .fill(w, clip, [2, 5, 11, 4, 12, 12], 112, 112, false);
        self.component
            .fill(w, clip, [8, 5, 11, 10, 12, 12], 112, 112, false);
        self.component
            .fill(w, clip, [5, 9, 11, 7, 12, 12], 112, 112, false);
        self.component
            .fill(w, clip, [2, 5, 0, 4, 12, 1], 112, 112, false);
        self.component
            .fill(w, clip, [8, 5, 0, 10, 12, 1], 112, 112, false);
        self.component
            .fill(w, clip, [5, 9, 0, 7, 12, 1], 112, 112, false);
        self.component
            .fill(w, clip, [2, 11, 2, 10, 12, 10], 112, 112, false);
        let mut var4: i32 = 0;
        for var4 in ((1)..=(11)).step_by(2) {
            self.component
                .fill(w, clip, [var4, 10, 0, var4, 11, 0], 113, 113, false);
            self.component
                .fill(w, clip, [var4, 10, 12, var4, 11, 12], 113, 113, false);
            self.component
                .fill(w, clip, [0, 10, var4, 0, 11, var4], 113, 113, false);
            self.component
                .fill(w, clip, [12, 10, var4, 12, 11, var4], 113, 113, false);
            self.component.put(w, clip, 112, 0, var4, 13, 0);
            self.component.put(w, clip, 112, 0, var4, 13, 12);
            self.component.put(w, clip, 112, 0, 0, 13, var4);
            self.component.put(w, clip, 112, 0, 12, 13, var4);
            self.component.put(w, clip, 113, 0, var4 + 1, 13, 0);
            self.component.put(w, clip, 113, 0, var4 + 1, 13, 12);
            self.component.put(w, clip, 113, 0, 0, 13, var4 + 1);
            self.component.put(w, clip, 113, 0, 12, 13, var4 + 1);
        }
        self.component.put(w, clip, 113, 0, 0, 13, 0);
        self.component.put(w, clip, 113, 0, 0, 13, 12);
        self.component.put(w, clip, 113, 0, 0, 13, 0);
        self.component.put(w, clip, 113, 0, 12, 13, 0);
        for var4 in ((3)..=(9)).step_by(2) {
            self.component
                .fill(w, clip, [1, 7, var4, 1, 8, var4], 113, 113, false);
            self.component
                .fill(w, clip, [11, 7, var4, 11, 8, var4], 113, 113, false);
        }
        var4 = self.component.rotate(114, 3);
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        for var5 in (0)..=(6) {
            var6 = var5 + 4;
            for var7 in (5)..=(7) {
                self.component.put(w, clip, 114, var4, var7, 5 + var5, var6);
            }
            if var6 >= 5 && var6 <= 8 {
                self.component
                    .fill(w, clip, [5, 5, var6, 7, var5 + 4, var6], 112, 112, false);
            } else if var6 >= 9 && var6 <= 10 {
                self.component
                    .fill(w, clip, [5, 8, var6, 7, var5 + 4, var6], 112, 112, false);
            }
            if var5 >= 1 {
                self.component
                    .fill(w, clip, [5, 6 + var5, var6, 7, 9 + var5, var6], 0, 0, false);
            }
        }
        for var5 in (5)..=(7) {
            self.component.put(w, clip, 114, var4, var5, 12, 11);
        }
        self.component
            .fill(w, clip, [5, 6, 7, 5, 7, 7], 113, 113, false);
        self.component
            .fill(w, clip, [7, 6, 7, 7, 7, 7], 113, 113, false);
        self.component
            .fill(w, clip, [5, 13, 12, 7, 13, 12], 0, 0, false);
        self.component
            .fill(w, clip, [2, 5, 2, 3, 5, 3], 112, 112, false);
        self.component
            .fill(w, clip, [2, 5, 9, 3, 5, 10], 112, 112, false);
        self.component
            .fill(w, clip, [2, 5, 4, 2, 5, 8], 112, 112, false);
        self.component
            .fill(w, clip, [9, 5, 2, 10, 5, 3], 112, 112, false);
        self.component
            .fill(w, clip, [9, 5, 9, 10, 5, 10], 112, 112, false);
        self.component
            .fill(w, clip, [10, 5, 4, 10, 5, 8], 112, 112, false);
        var5 = self.component.rotate(114, 0);
        var6 = self.component.rotate(114, 1);
        self.component.put(w, clip, 114, var6, 4, 5, 2);
        self.component.put(w, clip, 114, var6, 4, 5, 3);
        self.component.put(w, clip, 114, var6, 4, 5, 9);
        self.component.put(w, clip, 114, var6, 4, 5, 10);
        self.component.put(w, clip, 114, var5, 8, 5, 2);
        self.component.put(w, clip, 114, var5, 8, 5, 3);
        self.component.put(w, clip, 114, var5, 8, 5, 9);
        self.component.put(w, clip, 114, var5, 8, 5, 10);
        self.component
            .fill(w, clip, [3, 4, 4, 4, 4, 8], 88, 88, false);
        self.component
            .fill(w, clip, [8, 4, 4, 9, 4, 8], 88, 88, false);
        self.component
            .fill(w, clip, [3, 5, 4, 4, 5, 8], 115, 115, false);
        self.component
            .fill(w, clip, [8, 5, 4, 9, 5, 8], 115, 115, false);
        self.component
            .fill(w, clip, [4, 2, 0, 8, 2, 12], 112, 112, false);
        self.component
            .fill(w, clip, [0, 2, 4, 12, 2, 8], 112, 112, false);
        self.component
            .fill(w, clip, [4, 0, 0, 8, 1, 3], 112, 112, false);
        self.component
            .fill(w, clip, [4, 0, 9, 8, 1, 12], 112, 112, false);
        self.component
            .fill(w, clip, [0, 0, 4, 3, 1, 8], 112, 112, false);
        self.component
            .fill(w, clip, [9, 0, 4, 12, 1, 8], 112, 112, false);
        let mut var8: i32 = 0;
        for var7 in (4)..=(8) {
            for var8 in (0)..=(2) {
                self.component.fill_down(w, clip, 112, 0, var7, -1, var8);
                self.component
                    .fill_down(w, clip, 112, 0, var7, -1, 12 - var8);
            }
        }
        for var7 in (0)..=(2) {
            for var8 in (4)..=(8) {
                self.component.fill_down(w, clip, 112, 0, var7, -1, var8);
                self.component
                    .fill_down(w, clip, 112, 0, 12 - var7, -1, var8);
            }
        }
        return true;
    }
    pub(super) fn generate_13(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        let mut var4 = JavaRandom::new(self.end_seed as i64);
        let mut var5: i32 = 0;
        let mut var6: i32 = 0;
        let mut var7: i32 = 0;
        for var5 in (0)..=(4) {
            for var6 in (3)..=(4) {
                var7 = var4.next_int(8);
                self.component
                    .fill(w, clip, [var5, var6, 0, var5, var6, var7], 112, 112, false);
            }
        }
        var5 = var4.next_int(8);
        self.component
            .fill(w, clip, [0, 5, 0, 0, 5, var5], 112, 112, false);
        var5 = var4.next_int(8);
        self.component
            .fill(w, clip, [4, 5, 0, 4, 5, var5], 112, 112, false);
        for var5 in (0)..=(4) {
            var6 = var4.next_int(5);
            self.component
                .fill(w, clip, [var5, 2, 0, var5, 2, var6], 112, 112, false);
        }
        for var5 in (0)..=(4) {
            for var6 in (0)..=(1) {
                var7 = var4.next_int(3);
                self.component
                    .fill(w, clip, [var5, var6, 0, var5, var6, var7], 112, 112, false);
            }
        }
        return true;
    }
}
