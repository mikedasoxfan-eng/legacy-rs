//! Literal block-placement bodies from the Beta 1.8.1 village classes.
#![allow(unused_variables, unused_mut)]
use super::*;
impl VillagePiece {
    pub(super) fn generate_0(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        if self.height < 0 {
            self.height = average_height(w, clip, self.component.bounds);
            if self.height < 0 {
                return true;
            }
            self.component
                .bounds
                .offset(0, self.height - self.component.bounds.max_y + 6 - 1, 0);
        }
        self.component
            .fill(w, clip, [0, 0, 0, 4, 0, 4], 4, 4, false);
        self.component
            .fill(w, clip, [0, 4, 0, 4, 4, 4], 17, 17, false);
        self.component
            .fill(w, clip, [1, 4, 1, 3, 4, 3], 5, 5, false);
        self.component.put(w, clip, 4, 0, 0, 1, 0);
        self.component.put(w, clip, 4, 0, 0, 2, 0);
        self.component.put(w, clip, 4, 0, 0, 3, 0);
        self.component.put(w, clip, 4, 0, 4, 1, 0);
        self.component.put(w, clip, 4, 0, 4, 2, 0);
        self.component.put(w, clip, 4, 0, 4, 3, 0);
        self.component.put(w, clip, 4, 0, 0, 1, 4);
        self.component.put(w, clip, 4, 0, 0, 2, 4);
        self.component.put(w, clip, 4, 0, 0, 3, 4);
        self.component.put(w, clip, 4, 0, 4, 1, 4);
        self.component.put(w, clip, 4, 0, 4, 2, 4);
        self.component.put(w, clip, 4, 0, 4, 3, 4);
        self.component
            .fill(w, clip, [0, 1, 1, 0, 3, 3], 5, 5, false);
        self.component
            .fill(w, clip, [4, 1, 1, 4, 3, 3], 5, 5, false);
        self.component
            .fill(w, clip, [1, 1, 4, 3, 3, 4], 5, 5, false);
        self.component.put(w, clip, 102, 0, 0, 2, 2);
        self.component.put(w, clip, 102, 0, 2, 2, 4);
        self.component.put(w, clip, 102, 0, 4, 2, 2);
        self.component.put(w, clip, 5, 0, 1, 1, 0);
        self.component.put(w, clip, 5, 0, 1, 2, 0);
        self.component.put(w, clip, 5, 0, 1, 3, 0);
        self.component.put(w, clip, 5, 0, 2, 3, 0);
        self.component.put(w, clip, 5, 0, 3, 3, 0);
        self.component.put(w, clip, 5, 0, 3, 2, 0);
        self.component.put(w, clip, 5, 0, 3, 1, 0);
        if self.component.get(w, clip, 2, 0, -1) == 0 && self.component.get(w, clip, 2, -1, -1) != 0
        {
            self.component
                .put(w, clip, 67, self.component.rotate(67, 3), 2, 0, -1);
        }
        self.component
            .fill(w, clip, [1, 1, 1, 3, 3, 3], 0, 0, false);
        if self.flag {
            self.component.put(w, clip, 85, 0, 0, 5, 0);
            self.component.put(w, clip, 85, 0, 1, 5, 0);
            self.component.put(w, clip, 85, 0, 2, 5, 0);
            self.component.put(w, clip, 85, 0, 3, 5, 0);
            self.component.put(w, clip, 85, 0, 4, 5, 0);
            self.component.put(w, clip, 85, 0, 0, 5, 4);
            self.component.put(w, clip, 85, 0, 1, 5, 4);
            self.component.put(w, clip, 85, 0, 2, 5, 4);
            self.component.put(w, clip, 85, 0, 3, 5, 4);
            self.component.put(w, clip, 85, 0, 4, 5, 4);
            self.component.put(w, clip, 85, 0, 4, 5, 1);
            self.component.put(w, clip, 85, 0, 4, 5, 2);
            self.component.put(w, clip, 85, 0, 4, 5, 3);
            self.component.put(w, clip, 85, 0, 0, 5, 1);
            self.component.put(w, clip, 85, 0, 0, 5, 2);
            self.component.put(w, clip, 85, 0, 0, 5, 3);
        }
        let mut var4: i32;
        if self.flag {
            var4 = self.component.rotate(65, 3);
            self.component.put(w, clip, 65, var4, 3, 1, 3);
            self.component.put(w, clip, 65, var4, 3, 2, 3);
            self.component.put(w, clip, 65, var4, 3, 3, 3);
            self.component.put(w, clip, 65, var4, 3, 4, 3);
        }
        self.component.put(w, clip, 50, 0, 2, 3, 1);
        for var4 in (0)..(5) {
            for var5 in (0)..(5) {
                self.component.clear_up(w, clip, var5, 6, var4);
                self.component.fill_down(w, clip, 4, 0, var5, -1, var4);
            }
        }
        self.spawn_villagers10(w,clip,1,1,2,1);
            return true;
    }
    pub(super) fn generate_1(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        if self.height < 0 {
            self.height = average_height(w, clip, self.component.bounds);
            if self.height < 0 {
                return true;
            }
            self.component
                .bounds
                .offset(0, self.height - self.component.bounds.max_y + 12 - 1, 0);
        }
        self.component
            .fill(w, clip, [1, 1, 1, 3, 3, 7], 0, 0, false);
        self.component
            .fill(w, clip, [1, 5, 1, 3, 9, 3], 0, 0, false);
        self.component
            .fill(w, clip, [1, 0, 0, 3, 0, 8], 4, 4, false);
        self.component
            .fill(w, clip, [1, 1, 0, 3, 10, 0], 4, 4, false);
        self.component
            .fill(w, clip, [0, 1, 1, 0, 10, 3], 4, 4, false);
        self.component
            .fill(w, clip, [4, 1, 1, 4, 10, 3], 4, 4, false);
        self.component
            .fill(w, clip, [0, 0, 4, 0, 4, 7], 4, 4, false);
        self.component
            .fill(w, clip, [4, 0, 4, 4, 4, 7], 4, 4, false);
        self.component
            .fill(w, clip, [1, 1, 8, 3, 4, 8], 4, 4, false);
        self.component
            .fill(w, clip, [1, 5, 4, 3, 10, 4], 4, 4, false);
        self.component
            .fill(w, clip, [1, 5, 5, 3, 5, 7], 4, 4, false);
        self.component
            .fill(w, clip, [0, 9, 0, 4, 9, 4], 4, 4, false);
        self.component
            .fill(w, clip, [0, 4, 0, 4, 4, 4], 4, 4, false);
        self.component.put(w, clip, 4, 0, 0, 11, 2);
        self.component.put(w, clip, 4, 0, 4, 11, 2);
        self.component.put(w, clip, 4, 0, 2, 11, 0);
        self.component.put(w, clip, 4, 0, 2, 11, 4);
        self.component.put(w, clip, 4, 0, 1, 1, 6);
        self.component.put(w, clip, 4, 0, 1, 1, 7);
        self.component.put(w, clip, 4, 0, 2, 1, 7);
        self.component.put(w, clip, 4, 0, 3, 1, 6);
        self.component.put(w, clip, 4, 0, 3, 1, 7);
        self.component
            .put(w, clip, 67, self.component.rotate(67, 3), 1, 1, 5);
        self.component
            .put(w, clip, 67, self.component.rotate(67, 3), 2, 1, 6);
        self.component
            .put(w, clip, 67, self.component.rotate(67, 3), 3, 1, 5);
        self.component
            .put(w, clip, 67, self.component.rotate(67, 1), 1, 2, 7);
        self.component
            .put(w, clip, 67, self.component.rotate(67, 0), 3, 2, 7);
        self.component.put(w, clip, 102, 0, 0, 2, 2);
        self.component.put(w, clip, 102, 0, 0, 3, 2);
        self.component.put(w, clip, 102, 0, 4, 2, 2);
        self.component.put(w, clip, 102, 0, 4, 3, 2);
        self.component.put(w, clip, 102, 0, 0, 6, 2);
        self.component.put(w, clip, 102, 0, 0, 7, 2);
        self.component.put(w, clip, 102, 0, 4, 6, 2);
        self.component.put(w, clip, 102, 0, 4, 7, 2);
        self.component.put(w, clip, 102, 0, 2, 6, 0);
        self.component.put(w, clip, 102, 0, 2, 7, 0);
        self.component.put(w, clip, 102, 0, 2, 6, 4);
        self.component.put(w, clip, 102, 0, 2, 7, 4);
        self.component.put(w, clip, 102, 0, 0, 3, 6);
        self.component.put(w, clip, 102, 0, 4, 3, 6);
        self.component.put(w, clip, 102, 0, 2, 3, 8);
        self.component.put(w, clip, 50, 0, 2, 4, 7);
        self.component.put(w, clip, 50, 0, 1, 4, 6);
        self.component.put(w, clip, 50, 0, 3, 4, 6);
        self.component.put(w, clip, 50, 0, 2, 4, 5);
        let mut var4: i32 = self.component.rotate(65, 4);
        let mut var5: i32;
        for var5 in (1)..=(9) {
            self.component.put(w, clip, 65, var4, 3, var5, 3);
        }
        self.component.put(w, clip, 0, 0, 2, 1, 0);
        self.component.put(w, clip, 0, 0, 2, 2, 0);
        self.component
            .door(w, clip, 2, 1, 0, self.component.rotate(64, 1));
        if self.component.get(w, clip, 2, 0, -1) == 0 && self.component.get(w, clip, 2, -1, -1) != 0
        {
            self.component
                .put(w, clip, 67, self.component.rotate(67, 3), 2, 0, -1);
        }
        for var5 in (0)..(9) {
            for var6 in (0)..(5) {
                self.component.clear_up(w, clip, var6, 12, var5);
                self.component.fill_down(w, clip, 4, 0, var6, -1, var5);
            }
        }
        self.spawn_villagers10(w,clip,2,1,2,1);
            return true;
    }
    pub(super) fn generate_2(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        if self.height < 0 {
            self.height = average_height(w, clip, self.component.bounds);
            if self.height < 0 {
                return true;
            }
            self.component
                .bounds
                .offset(0, self.height - self.component.bounds.max_y + 9 - 1, 0);
        }
        self.component
            .fill(w, clip, [1, 1, 1, 7, 5, 4], 0, 0, false);
        self.component
            .fill(w, clip, [0, 0, 0, 8, 0, 5], 4, 4, false);
        self.component
            .fill(w, clip, [0, 5, 0, 8, 5, 5], 4, 4, false);
        self.component
            .fill(w, clip, [0, 6, 1, 8, 6, 4], 4, 4, false);
        self.component
            .fill(w, clip, [0, 7, 2, 8, 7, 3], 4, 4, false);
        let mut var4: i32 = self.component.rotate(53, 3);
        let mut var5: i32 = self.component.rotate(53, 2);
        let mut var6: i32;
        let mut var7: i32;
        for var6 in (-1)..=(2) {
            for var7 in (0)..=(8) {
                self.component.put(w, clip, 53, var4, var7, 6 + var6, var6);
                self.component
                    .put(w, clip, 53, var5, var7, 6 + var6, 5 - var6);
            }
        }
        self.component
            .fill(w, clip, [0, 1, 0, 0, 1, 5], 4, 4, false);
        self.component
            .fill(w, clip, [1, 1, 5, 8, 1, 5], 4, 4, false);
        self.component
            .fill(w, clip, [8, 1, 0, 8, 1, 4], 4, 4, false);
        self.component
            .fill(w, clip, [2, 1, 0, 7, 1, 0], 4, 4, false);
        self.component
            .fill(w, clip, [0, 2, 0, 0, 4, 0], 4, 4, false);
        self.component
            .fill(w, clip, [0, 2, 5, 0, 4, 5], 4, 4, false);
        self.component
            .fill(w, clip, [8, 2, 5, 8, 4, 5], 4, 4, false);
        self.component
            .fill(w, clip, [8, 2, 0, 8, 4, 0], 4, 4, false);
        self.component
            .fill(w, clip, [0, 2, 1, 0, 4, 4], 5, 5, false);
        self.component
            .fill(w, clip, [1, 2, 5, 7, 4, 5], 5, 5, false);
        self.component
            .fill(w, clip, [8, 2, 1, 8, 4, 4], 5, 5, false);
        self.component
            .fill(w, clip, [1, 2, 0, 7, 4, 0], 5, 5, false);
        self.component.put(w, clip, 102, 0, 4, 2, 0);
        self.component.put(w, clip, 102, 0, 5, 2, 0);
        self.component.put(w, clip, 102, 0, 6, 2, 0);
        self.component.put(w, clip, 102, 0, 4, 3, 0);
        self.component.put(w, clip, 102, 0, 5, 3, 0);
        self.component.put(w, clip, 102, 0, 6, 3, 0);
        self.component.put(w, clip, 102, 0, 0, 2, 2);
        self.component.put(w, clip, 102, 0, 0, 2, 3);
        self.component.put(w, clip, 102, 0, 0, 3, 2);
        self.component.put(w, clip, 102, 0, 0, 3, 3);
        self.component.put(w, clip, 102, 0, 8, 2, 2);
        self.component.put(w, clip, 102, 0, 8, 2, 3);
        self.component.put(w, clip, 102, 0, 8, 3, 2);
        self.component.put(w, clip, 102, 0, 8, 3, 3);
        self.component.put(w, clip, 102, 0, 2, 2, 5);
        self.component.put(w, clip, 102, 0, 3, 2, 5);
        self.component.put(w, clip, 102, 0, 5, 2, 5);
        self.component.put(w, clip, 102, 0, 6, 2, 5);
        self.component
            .fill(w, clip, [1, 4, 1, 7, 4, 1], 5, 5, false);
        self.component
            .fill(w, clip, [1, 4, 4, 7, 4, 4], 5, 5, false);
        self.component
            .fill(w, clip, [1, 3, 4, 7, 3, 4], 47, 47, false);
        self.component.put(w, clip, 5, 0, 7, 1, 4);
        self.component
            .put(w, clip, 53, self.component.rotate(53, 0), 7, 1, 3);
        var6 = self.component.rotate(53, 3);
        self.component.put(w, clip, 53, var6, 6, 1, 4);
        self.component.put(w, clip, 53, var6, 5, 1, 4);
        self.component.put(w, clip, 53, var6, 4, 1, 4);
        self.component.put(w, clip, 53, var6, 3, 1, 4);
        self.component.put(w, clip, 85, 0, 6, 1, 3);
        self.component.put(w, clip, 72, 0, 6, 2, 3);
        self.component.put(w, clip, 85, 0, 4, 1, 3);
        self.component.put(w, clip, 72, 0, 4, 2, 3);
        self.component.put(w, clip, 58, 0, 7, 1, 1);
        self.component.put(w, clip, 0, 0, 1, 1, 0);
        self.component.put(w, clip, 0, 0, 1, 2, 0);
        self.component
            .door(w, clip, 1, 1, 0, self.component.rotate(64, 1));
        if self.component.get(w, clip, 1, 0, -1) == 0 && self.component.get(w, clip, 1, -1, -1) != 0
        {
            self.component
                .put(w, clip, 67, self.component.rotate(67, 3), 1, 0, -1);
        }
        for var7 in (0)..(6) {
            for var8 in (0)..(9) {
                self.component.clear_up(w, clip, var8, 9, var7);
                self.component.fill_down(w, clip, 4, 0, var8, -1, var7);
            }
        }
        self.spawn_villagers10(w,clip,2,1,2,1);
            return true;
    }
    pub(super) fn generate_3(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        if self.height < 0 {
            self.height = average_height(w, clip, self.component.bounds);
            if self.height < 0 {
                return true;
            }
            self.component
                .bounds
                .offset(0, self.height - self.component.bounds.max_y + 6 - 1, 0);
        }
        self.component
            .fill(w, clip, [1, 1, 1, 3, 5, 4], 0, 0, false);
        self.component
            .fill(w, clip, [0, 0, 0, 3, 0, 4], 4, 4, false);
        self.component
            .fill(w, clip, [1, 0, 1, 2, 0, 3], 3, 3, false);
        if self.flag {
            self.component
                .fill(w, clip, [1, 4, 1, 2, 4, 3], 17, 17, false);
        } else {
            self.component
                .fill(w, clip, [1, 5, 1, 2, 5, 3], 17, 17, false);
        }
        self.component.put(w, clip, 17, 0, 1, 4, 0);
        self.component.put(w, clip, 17, 0, 2, 4, 0);
        self.component.put(w, clip, 17, 0, 1, 4, 4);
        self.component.put(w, clip, 17, 0, 2, 4, 4);
        self.component.put(w, clip, 17, 0, 0, 4, 1);
        self.component.put(w, clip, 17, 0, 0, 4, 2);
        self.component.put(w, clip, 17, 0, 0, 4, 3);
        self.component.put(w, clip, 17, 0, 3, 4, 1);
        self.component.put(w, clip, 17, 0, 3, 4, 2);
        self.component.put(w, clip, 17, 0, 3, 4, 3);
        self.component
            .fill(w, clip, [0, 1, 0, 0, 3, 0], 17, 17, false);
        self.component
            .fill(w, clip, [3, 1, 0, 3, 3, 0], 17, 17, false);
        self.component
            .fill(w, clip, [0, 1, 4, 0, 3, 4], 17, 17, false);
        self.component
            .fill(w, clip, [3, 1, 4, 3, 3, 4], 17, 17, false);
        self.component
            .fill(w, clip, [0, 1, 1, 0, 3, 3], 5, 5, false);
        self.component
            .fill(w, clip, [3, 1, 1, 3, 3, 3], 5, 5, false);
        self.component
            .fill(w, clip, [1, 1, 0, 2, 3, 0], 5, 5, false);
        self.component
            .fill(w, clip, [1, 1, 4, 2, 3, 4], 5, 5, false);
        self.component.put(w, clip, 102, 0, 0, 2, 2);
        self.component.put(w, clip, 102, 0, 3, 2, 2);
        if self.style > 0 {
            self.component.put(w, clip, 85, 0, self.style, 1, 3);
            self.component.put(w, clip, 72, 0, self.style, 2, 3);
        }
        self.component.put(w, clip, 0, 0, 1, 1, 0);
        self.component.put(w, clip, 0, 0, 1, 2, 0);
        self.component
            .door(w, clip, 1, 1, 0, self.component.rotate(64, 1));
        if self.component.get(w, clip, 1, 0, -1) == 0 && self.component.get(w, clip, 1, -1, -1) != 0
        {
            self.component
                .put(w, clip, 67, self.component.rotate(67, 3), 1, 0, -1);
        }
        for var4 in (0)..(5) {
            for var5 in (0)..(4) {
                self.component.clear_up(w, clip, var5, 6, var4);
                self.component.fill_down(w, clip, 4, 0, var5, -1, var4);
            }
        }
        self.spawn_villagers10(w,clip,1,1,2,1);
            return true;
    }
    pub(super) fn generate_4(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        if self.height < 0 {
            self.height = average_height(w, clip, self.component.bounds);
            if self.height < 0 {
                return true;
            }
            self.component
                .bounds
                .offset(0, self.height - self.component.bounds.max_y + 7 - 1, 0);
        }
        self.component
            .fill(w, clip, [1, 1, 1, 7, 4, 4], 0, 0, false);
        self.component
            .fill(w, clip, [2, 1, 6, 8, 4, 10], 0, 0, false);
        self.component
            .fill(w, clip, [2, 0, 6, 8, 0, 10], 3, 3, false);
        self.component.put(w, clip, 4, 0, 6, 0, 6);
        self.component
            .fill(w, clip, [2, 1, 6, 2, 1, 10], 85, 85, false);
        self.component
            .fill(w, clip, [8, 1, 6, 8, 1, 10], 85, 85, false);
        self.component
            .fill(w, clip, [3, 1, 10, 7, 1, 10], 85, 85, false);
        self.component
            .fill(w, clip, [1, 0, 1, 7, 0, 4], 5, 5, false);
        self.component
            .fill(w, clip, [0, 0, 0, 0, 3, 5], 4, 4, false);
        self.component
            .fill(w, clip, [8, 0, 0, 8, 3, 5], 4, 4, false);
        self.component
            .fill(w, clip, [1, 0, 0, 7, 1, 0], 4, 4, false);
        self.component
            .fill(w, clip, [1, 0, 5, 7, 1, 5], 4, 4, false);
        self.component
            .fill(w, clip, [1, 2, 0, 7, 3, 0], 5, 5, false);
        self.component
            .fill(w, clip, [1, 2, 5, 7, 3, 5], 5, 5, false);
        self.component
            .fill(w, clip, [0, 4, 1, 8, 4, 1], 5, 5, false);
        self.component
            .fill(w, clip, [0, 4, 4, 8, 4, 4], 5, 5, false);
        self.component
            .fill(w, clip, [0, 5, 2, 8, 5, 3], 5, 5, false);
        self.component.put(w, clip, 5, 0, 0, 4, 2);
        self.component.put(w, clip, 5, 0, 0, 4, 3);
        self.component.put(w, clip, 5, 0, 8, 4, 2);
        self.component.put(w, clip, 5, 0, 8, 4, 3);
        let mut var4: i32 = self.component.rotate(53, 3);
        let mut var5: i32 = self.component.rotate(53, 2);
        let mut var6: i32;
        let mut var7: i32;
        for var6 in (-1)..=(2) {
            for var7 in (0)..=(8) {
                self.component.put(w, clip, 53, var4, var7, 4 + var6, var6);
                self.component
                    .put(w, clip, 53, var5, var7, 4 + var6, 5 - var6);
            }
        }
        self.component.put(w, clip, 17, 0, 0, 2, 1);
        self.component.put(w, clip, 17, 0, 0, 2, 4);
        self.component.put(w, clip, 17, 0, 8, 2, 1);
        self.component.put(w, clip, 17, 0, 8, 2, 4);
        self.component.put(w, clip, 102, 0, 0, 2, 2);
        self.component.put(w, clip, 102, 0, 0, 2, 3);
        self.component.put(w, clip, 102, 0, 8, 2, 2);
        self.component.put(w, clip, 102, 0, 8, 2, 3);
        self.component.put(w, clip, 102, 0, 2, 2, 5);
        self.component.put(w, clip, 102, 0, 3, 2, 5);
        self.component.put(w, clip, 102, 0, 5, 2, 0);
        self.component.put(w, clip, 102, 0, 6, 2, 5);
        self.component.put(w, clip, 85, 0, 2, 1, 3);
        self.component.put(w, clip, 72, 0, 2, 2, 3);
        self.component.put(w, clip, 5, 0, 1, 1, 4);
        self.component
            .put(w, clip, 53, self.component.rotate(53, 3), 2, 1, 4);
        self.component
            .put(w, clip, 53, self.component.rotate(53, 1), 1, 1, 3);
        self.component
            .fill(w, clip, [5, 0, 1, 7, 0, 3], 43, 43, false);
        self.component.put(w, clip, 43, 0, 6, 1, 1);
        self.component.put(w, clip, 43, 0, 6, 1, 2);
        self.component.put(w, clip, 0, 0, 2, 1, 0);
        self.component.put(w, clip, 0, 0, 2, 2, 0);
        self.component.put(w, clip, 50, 0, 2, 3, 1);
        self.component
            .door(w, clip, 2, 1, 0, self.component.rotate(64, 1));
        if self.component.get(w, clip, 2, 0, -1) == 0 && self.component.get(w, clip, 2, -1, -1) != 0
        {
            self.component
                .put(w, clip, 67, self.component.rotate(67, 3), 2, 0, -1);
        }
        self.component.put(w, clip, 0, 0, 6, 1, 5);
        self.component.put(w, clip, 0, 0, 6, 2, 5);
        self.component.put(w, clip, 50, 0, 6, 3, 4);
        self.component
            .door(w, clip, 6, 1, 5, self.component.rotate(64, 1));
        for var6 in (0)..(5) {
            for var7 in (0)..(9) {
                self.component.clear_up(w, clip, var7, 7, var6);
                self.component.fill_down(w, clip, 4, 0, var7, -1, var6);
            }
        }
        self.spawn_villagers10(w,clip,4,1,2,2);
            return true;
    }
    pub(super) fn generate_5(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        if self.height < 0 {
            self.height = average_height(w, clip, self.component.bounds);
            if self.height < 0 {
                return true;
            }
            self.component
                .bounds
                .offset(0, self.height - self.component.bounds.max_y + 4 - 1, 0);
        }
        self.component
            .fill(w, clip, [0, 1, 0, 12, 4, 8], 0, 0, false);
        self.component
            .fill(w, clip, [1, 0, 1, 2, 0, 7], 60, 60, false);
        self.component
            .fill(w, clip, [4, 0, 1, 5, 0, 7], 60, 60, false);
        self.component
            .fill(w, clip, [7, 0, 1, 8, 0, 7], 60, 60, false);
        self.component
            .fill(w, clip, [10, 0, 1, 11, 0, 7], 60, 60, false);
        self.component
            .fill(w, clip, [0, 0, 0, 0, 0, 8], 17, 17, false);
        self.component
            .fill(w, clip, [6, 0, 0, 6, 0, 8], 17, 17, false);
        self.component
            .fill(w, clip, [12, 0, 0, 12, 0, 8], 17, 17, false);
        self.component
            .fill(w, clip, [1, 0, 0, 11, 0, 0], 17, 17, false);
        self.component
            .fill(w, clip, [1, 0, 8, 11, 0, 8], 17, 17, false);
        self.component
            .fill(w, clip, [3, 0, 1, 3, 0, 7], 8, 8, false);
        self.component
            .fill(w, clip, [9, 0, 1, 9, 0, 7], 8, 8, false);
        let mut var4: i32;
        for var4 in (1)..=(7) {
            let crop_metadata = inclusive(crate::structures::component::random(w, r), 2, 7);
            self.component.put(w, clip, 59, crop_metadata, 1, 1, var4);
            let crop_metadata = inclusive(crate::structures::component::random(w, r), 2, 7);
            self.component.put(w, clip, 59, crop_metadata, 2, 1, var4);
            let crop_metadata = inclusive(crate::structures::component::random(w, r), 2, 7);
            self.component.put(w, clip, 59, crop_metadata, 4, 1, var4);
            let crop_metadata = inclusive(crate::structures::component::random(w, r), 2, 7);
            self.component.put(w, clip, 59, crop_metadata, 5, 1, var4);
            let crop_metadata = inclusive(crate::structures::component::random(w, r), 2, 7);
            self.component.put(w, clip, 59, crop_metadata, 7, 1, var4);
            let crop_metadata = inclusive(crate::structures::component::random(w, r), 2, 7);
            self.component.put(w, clip, 59, crop_metadata, 8, 1, var4);
            let crop_metadata = inclusive(crate::structures::component::random(w, r), 2, 7);
            self.component.put(w, clip, 59, crop_metadata, 10, 1, var4);
            let crop_metadata = inclusive(crate::structures::component::random(w, r), 2, 7);
            self.component.put(w, clip, 59, crop_metadata, 11, 1, var4);
        }
        for var4 in (0)..(9) {
            for var5 in (0)..(13) {
                self.component.clear_up(w, clip, var5, 4, var4);
                self.component.fill_down(w, clip, 3, 0, var5, -1, var4);
            }
        }
        return true;
    }
    pub(super) fn generate_6(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        if self.height < 0 {
            self.height = average_height(w, clip, self.component.bounds);
            if self.height < 0 {
                return true;
            }
            self.component
                .bounds
                .offset(0, self.height - self.component.bounds.max_y + 4 - 1, 0);
        }
        self.component
            .fill(w, clip, [0, 1, 0, 6, 4, 8], 0, 0, false);
        self.component
            .fill(w, clip, [1, 0, 1, 2, 0, 7], 60, 60, false);
        self.component
            .fill(w, clip, [4, 0, 1, 5, 0, 7], 60, 60, false);
        self.component
            .fill(w, clip, [0, 0, 0, 0, 0, 8], 17, 17, false);
        self.component
            .fill(w, clip, [6, 0, 0, 6, 0, 8], 17, 17, false);
        self.component
            .fill(w, clip, [1, 0, 0, 5, 0, 0], 17, 17, false);
        self.component
            .fill(w, clip, [1, 0, 8, 5, 0, 8], 17, 17, false);
        self.component
            .fill(w, clip, [3, 0, 1, 3, 0, 7], 8, 8, false);
        let mut var4: i32;
        for var4 in (1)..=(7) {
            let crop_metadata = inclusive(crate::structures::component::random(w, r), 2, 7);
            self.component.put(w, clip, 59, crop_metadata, 1, 1, var4);
            let crop_metadata = inclusive(crate::structures::component::random(w, r), 2, 7);
            self.component.put(w, clip, 59, crop_metadata, 2, 1, var4);
            let crop_metadata = inclusive(crate::structures::component::random(w, r), 2, 7);
            self.component.put(w, clip, 59, crop_metadata, 4, 1, var4);
            let crop_metadata = inclusive(crate::structures::component::random(w, r), 2, 7);
            self.component.put(w, clip, 59, crop_metadata, 5, 1, var4);
        }
        for var4 in (0)..(9) {
            for var5 in (0)..(7) {
                self.component.clear_up(w, clip, var5, 4, var4);
                self.component.fill_down(w, clip, 3, 0, var5, -1, var4);
            }
        }
        return true;
    }
    pub(super) fn generate_7(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        if self.height < 0 {
            self.height = average_height(w, clip, self.component.bounds);
            if self.height < 0 {
                return true;
            }
            self.component
                .bounds
                .offset(0, self.height - self.component.bounds.max_y + 6 - 1, 0);
        }
        self.component
            .fill(w, clip, [0, 1, 0, 9, 4, 6], 0, 0, false);
        self.component
            .fill(w, clip, [0, 0, 0, 9, 0, 6], 4, 4, false);
        self.component
            .fill(w, clip, [0, 4, 0, 9, 4, 6], 4, 4, false);
        self.component
            .fill(w, clip, [0, 5, 0, 9, 5, 6], 44, 44, false);
        self.component
            .fill(w, clip, [1, 5, 1, 8, 5, 5], 0, 0, false);
        self.component
            .fill(w, clip, [1, 1, 0, 2, 3, 0], 5, 5, false);
        self.component
            .fill(w, clip, [0, 1, 0, 0, 4, 0], 17, 17, false);
        self.component
            .fill(w, clip, [3, 1, 0, 3, 4, 0], 17, 17, false);
        self.component
            .fill(w, clip, [0, 1, 6, 0, 4, 6], 17, 17, false);
        self.component.put(w, clip, 5, 0, 3, 3, 1);
        self.component
            .fill(w, clip, [3, 1, 2, 3, 3, 2], 5, 5, false);
        self.component
            .fill(w, clip, [4, 1, 3, 5, 3, 3], 5, 5, false);
        self.component
            .fill(w, clip, [0, 1, 1, 0, 3, 5], 5, 5, false);
        self.component
            .fill(w, clip, [1, 1, 6, 5, 3, 6], 5, 5, false);
        self.component
            .fill(w, clip, [5, 1, 0, 5, 3, 0], 85, 85, false);
        self.component
            .fill(w, clip, [9, 1, 0, 9, 3, 0], 85, 85, false);
        self.component
            .fill(w, clip, [6, 1, 4, 9, 4, 6], 4, 4, false);
        self.component.put(w, clip, 10, 0, 7, 1, 5);
        self.component.put(w, clip, 10, 0, 8, 1, 5);
        self.component.put(w, clip, 101, 0, 9, 2, 5);
        self.component.put(w, clip, 101, 0, 9, 2, 4);
        self.component
            .fill(w, clip, [7, 2, 4, 8, 2, 5], 0, 0, false);
        self.component.put(w, clip, 4, 0, 6, 1, 3);
        self.component.put(w, clip, 61, 0, 6, 2, 3);
        self.component.put(w, clip, 61, 0, 6, 3, 3);
        self.component.put(w, clip, 43, 0, 8, 1, 1);
        self.component.put(w, clip, 102, 0, 0, 2, 2);
        self.component.put(w, clip, 102, 0, 0, 2, 4);
        self.component.put(w, clip, 102, 0, 2, 2, 6);
        self.component.put(w, clip, 102, 0, 4, 2, 6);
        self.component.put(w, clip, 85, 0, 2, 1, 4);
        self.component.put(w, clip, 72, 0, 2, 2, 4);
        self.component.put(w, clip, 5, 0, 1, 1, 5);
        self.component
            .put(w, clip, 53, self.component.rotate(53, 3), 2, 1, 5);
        self.component
            .put(w, clip, 53, self.component.rotate(53, 1), 1, 1, 4);
        let mut var4: i32;
        for var4 in (6)..=(8) {
            if self.component.get(w, clip, var4, 0, -1) == 0
                && self.component.get(w, clip, var4, -1, -1) != 0
            {
                self.component
                    .put(w, clip, 67, self.component.rotate(67, 3), var4, 0, -1);
            }
        }
        for var4 in (0)..(7) {
            for var5 in (0)..(10) {
                self.component.clear_up(w, clip, var5, 6, var4);
                self.component.fill_down(w, clip, 4, 0, var5, -1, var4);
            }
        }
        self.spawn_villagers10(w,clip,7,1,1,1);
            return true;
    }
    pub(super) fn generate_8(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        if self.height < 0 {
            self.height = average_height(w, clip, self.component.bounds);
            if self.height < 0 {
                return true;
            }
            self.component
                .bounds
                .offset(0, self.height - self.component.bounds.max_y + 7 - 1, 0);
        }
        self.component
            .fill(w, clip, [1, 1, 1, 7, 4, 4], 0, 0, false);
        self.component
            .fill(w, clip, [2, 1, 6, 8, 4, 10], 0, 0, false);
        self.component
            .fill(w, clip, [2, 0, 5, 8, 0, 10], 5, 5, false);
        self.component
            .fill(w, clip, [1, 0, 1, 7, 0, 4], 5, 5, false);
        self.component
            .fill(w, clip, [0, 0, 0, 0, 3, 5], 4, 4, false);
        self.component
            .fill(w, clip, [8, 0, 0, 8, 3, 10], 4, 4, false);
        self.component
            .fill(w, clip, [1, 0, 0, 7, 2, 0], 4, 4, false);
        self.component
            .fill(w, clip, [1, 0, 5, 2, 1, 5], 4, 4, false);
        self.component
            .fill(w, clip, [2, 0, 6, 2, 3, 10], 4, 4, false);
        self.component
            .fill(w, clip, [3, 0, 10, 7, 3, 10], 4, 4, false);
        self.component
            .fill(w, clip, [1, 2, 0, 7, 3, 0], 5, 5, false);
        self.component
            .fill(w, clip, [1, 2, 5, 2, 3, 5], 5, 5, false);
        self.component
            .fill(w, clip, [0, 4, 1, 8, 4, 1], 5, 5, false);
        self.component
            .fill(w, clip, [0, 4, 4, 3, 4, 4], 5, 5, false);
        self.component
            .fill(w, clip, [0, 5, 2, 8, 5, 3], 5, 5, false);
        self.component.put(w, clip, 5, 0, 0, 4, 2);
        self.component.put(w, clip, 5, 0, 0, 4, 3);
        self.component.put(w, clip, 5, 0, 8, 4, 2);
        self.component.put(w, clip, 5, 0, 8, 4, 3);
        self.component.put(w, clip, 5, 0, 8, 4, 4);
        let mut var4: i32 = self.component.rotate(53, 3);
        let mut var5: i32 = self.component.rotate(53, 2);
        let mut var6: i32;
        let mut var7: i32;
        for var6 in (-1)..=(2) {
            for var7 in (0)..=(8) {
                self.component.put(w, clip, 53, var4, var7, 4 + var6, var6);
                if (var6 > -1 || var7 <= 1)
                    && (var6 > 0 || var7 <= 3)
                    && (var6 > 1 || var7 <= 4 || var7 >= 6)
                {
                    self.component
                        .put(w, clip, 53, var5, var7, 4 + var6, 5 - var6);
                }
            }
        }
        self.component
            .fill(w, clip, [3, 4, 5, 3, 4, 10], 5, 5, false);
        self.component
            .fill(w, clip, [7, 4, 2, 7, 4, 10], 5, 5, false);
        self.component
            .fill(w, clip, [4, 5, 4, 4, 5, 10], 5, 5, false);
        self.component
            .fill(w, clip, [6, 5, 4, 6, 5, 10], 5, 5, false);
        self.component
            .fill(w, clip, [5, 6, 3, 5, 6, 10], 5, 5, false);
        var6 = self.component.rotate(53, 0);
        let mut var8: i32;
        for var7 in ((1)..=(4)).rev() {
            self.component.put(w, clip, 5, 0, var7, 2 + var7, 7 - var7);
            for var8 in (8 - var7)..=(10) {
                self.component.put(w, clip, 53, var6, var7, 2 + var7, var8);
            }
        }
        var7 = self.component.rotate(53, 1);
        self.component.put(w, clip, 5, 0, 6, 6, 3);
        self.component.put(w, clip, 5, 0, 7, 5, 4);
        self.component.put(w, clip, 53, var7, 6, 6, 4);
        let mut var9: i32;
        for var8 in (6)..=(8) {
            for var9 in (5)..=(10) {
                self.component.put(w, clip, 53, var7, var8, 12 - var8, var9);
            }
        }
        self.component.put(w, clip, 17, 0, 0, 2, 1);
        self.component.put(w, clip, 17, 0, 0, 2, 4);
        self.component.put(w, clip, 102, 0, 0, 2, 2);
        self.component.put(w, clip, 102, 0, 0, 2, 3);
        self.component.put(w, clip, 17, 0, 4, 2, 0);
        self.component.put(w, clip, 102, 0, 5, 2, 0);
        self.component.put(w, clip, 17, 0, 6, 2, 0);
        self.component.put(w, clip, 17, 0, 8, 2, 1);
        self.component.put(w, clip, 102, 0, 8, 2, 2);
        self.component.put(w, clip, 102, 0, 8, 2, 3);
        self.component.put(w, clip, 17, 0, 8, 2, 4);
        self.component.put(w, clip, 5, 0, 8, 2, 5);
        self.component.put(w, clip, 17, 0, 8, 2, 6);
        self.component.put(w, clip, 102, 0, 8, 2, 7);
        self.component.put(w, clip, 102, 0, 8, 2, 8);
        self.component.put(w, clip, 17, 0, 8, 2, 9);
        self.component.put(w, clip, 17, 0, 2, 2, 6);
        self.component.put(w, clip, 102, 0, 2, 2, 7);
        self.component.put(w, clip, 102, 0, 2, 2, 8);
        self.component.put(w, clip, 17, 0, 2, 2, 9);
        self.component.put(w, clip, 17, 0, 4, 4, 10);
        self.component.put(w, clip, 102, 0, 5, 4, 10);
        self.component.put(w, clip, 17, 0, 6, 4, 10);
        self.component.put(w, clip, 5, 0, 5, 5, 10);
        self.component.put(w, clip, 0, 0, 2, 1, 0);
        self.component.put(w, clip, 0, 0, 2, 2, 0);
        self.component.put(w, clip, 50, 0, 2, 3, 1);
        self.component
            .door(w, clip, 2, 1, 0, self.component.rotate(64, 1));
        self.component
            .fill(w, clip, [1, 0, -1, 3, 2, -1], 0, 0, false);
        if self.component.get(w, clip, 2, 0, -1) == 0 && self.component.get(w, clip, 2, -1, -1) != 0
        {
            self.component
                .put(w, clip, 67, self.component.rotate(67, 3), 2, 0, -1);
        }
        for var8 in (0)..(5) {
            for var9 in (0)..(9) {
                self.component.clear_up(w, clip, var9, 7, var8);
                self.component.fill_down(w, clip, 4, 0, var9, -1, var8);
            }
        }
        for var8 in (5)..(11) {
            for var9 in (2)..(9) {
                self.component.clear_up(w, clip, var9, 7, var8);
                self.component.fill_down(w, clip, 4, 0, var9, -1, var8);
            }
        }
        self.spawn_villagers10(w,clip,4,1,2,2);
            return true;
    }
    pub(super) fn generate_9(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        if self.height < 0 {
            self.height = average_height(w, clip, self.component.bounds);
            if self.height < 0 {
                return true;
            }
            self.component
                .bounds
                .offset(0, self.height - self.component.bounds.max_y + 4 - 1, 0);
        }
        self.component
            .fill(w, clip, [0, 0, 0, 2, 3, 1], 0, 0, false);
        self.component.put(w, clip, 85, 0, 1, 0, 0);
        self.component.put(w, clip, 85, 0, 1, 1, 0);
        self.component.put(w, clip, 85, 0, 1, 2, 0);
        self.component.put(w, clip, 35, 15, 1, 3, 0);
        self.component.put(w, clip, 50, 15, 0, 3, 0);
        self.component.put(w, clip, 50, 15, 1, 3, 1);
        self.component.put(w, clip, 50, 15, 2, 3, 0);
        self.component.put(w, clip, 50, 15, 1, 3, -1);
        return true;
    }
    pub(super) fn generate_10(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        if self.height < 0 {
            self.height = average_height(w, clip, self.component.bounds);
            if self.height < 0 {
                return true;
            }
            self.component
                .bounds
                .offset(0, self.height - self.component.bounds.max_y + 3, 0);
        }
        if self.flag {}
        self.component
            .fill(w, clip, [1, 0, 1, 4, 12, 4], 4, 8, false);
        self.component.put(w, clip, 0, 0, 2, 12, 2);
        self.component.put(w, clip, 0, 0, 3, 12, 2);
        self.component.put(w, clip, 0, 0, 2, 12, 3);
        self.component.put(w, clip, 0, 0, 3, 12, 3);
        self.component.put(w, clip, 85, 0, 1, 13, 1);
        self.component.put(w, clip, 85, 0, 1, 14, 1);
        self.component.put(w, clip, 85, 0, 4, 13, 1);
        self.component.put(w, clip, 85, 0, 4, 14, 1);
        self.component.put(w, clip, 85, 0, 1, 13, 4);
        self.component.put(w, clip, 85, 0, 1, 14, 4);
        self.component.put(w, clip, 85, 0, 4, 13, 4);
        self.component.put(w, clip, 85, 0, 4, 14, 4);
        self.component
            .fill(w, clip, [1, 15, 1, 4, 15, 4], 4, 4, false);
        for var4 in (0)..=(5) {
            for var5 in (0)..=(5) {
                if var5 == 0 || var5 == 5 || var4 == 0 || var4 == 5 {
                    self.component.put(w, clip, 13, 0, var5, 11, var4);
                    self.component.clear_up(w, clip, var5, 12, var4);
                }
            }
        }
        return true;
    }
    pub(super) fn generate_11(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        for var4 in (self.component.bounds.min_x)..=(self.component.bounds.max_x) {
            for var5 in (self.component.bounds.min_z)..=(self.component.bounds.max_z) {
                if clip.contains(var4, 64, var5) {
                    let mut var6: i32 = top_solid(w, var4, var5) - 1;
                    w.set_block(var4, var6, var5, 13 as u8);
                }
            }
        }
        return true;
    }
}
