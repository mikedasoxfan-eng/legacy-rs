//! `AchievementList` and the part of `StatFileWriter` the screens read: which achievements
//! are unlocked and which can be unlocked next.

use std::collections::HashMap;

pub struct Achievement {
    /// `statId` = 5242880 + index
    pub id: i32,
    /// Translation key suffix (`achievement.<key>`, `.desc`).
    pub key: &'static str,
    pub column: i32,
    pub row: i32,
    /// The item shown (id, damage).
    pub icon: (i16, i16),
    pub parent: Option<usize>,
    pub special: bool,
}

const fn a(i: i32, key: &'static str, column: i32, row: i32, icon: i16, parent: Option<usize>, special: bool) -> Achievement {
    Achievement { id: 5242880 + i, key, column, row, icon: (icon, 0), parent, special }
}

/// In registration order (`AchievementList.achievementList`).
pub const ACHIEVEMENTS: [Achievement; 16] = [
    a(0, "openInventory", 0, 0, 340, None, false),
    a(1, "mineWood", 2, 1, 17, Some(0), false),
    a(2, "buildWorkBench", 4, -1, 58, Some(1), false),
    a(3, "buildPickaxe", 4, 2, 270, Some(2), false),
    a(4, "buildFurnace", 3, 4, 62, Some(3), false),
    a(5, "acquireIron", 1, 4, 265, Some(4), false),
    a(6, "buildHoe", 2, -3, 290, Some(2), false),
    a(7, "makeBread", -1, -3, 297, Some(6), false),
    a(8, "bakeCake", 0, -5, 354, Some(6), false),
    a(9, "buildBetterPickaxe", 6, 2, 274, Some(3), false),
    a(10, "cookFish", 2, 6, 350, Some(4), false),
    a(11, "onARail", 2, 3, 66, Some(5), true),
    a(12, "buildSword", 6, -1, 268, Some(2), false),
    a(13, "killEnemy", 8, -1, 352, Some(12), false),
    a(14, "killCow", 7, -3, 334, Some(12), false),
    a(15, "flyPig", 8, -4, 329, Some(14), true),
];

/// `AchievementList.min/maxDisplayColumn/Row` after registration.
pub fn display_bounds() -> (i32, i32, i32, i32) {
    let mut b = (0, 0, 0, 0);
    for x in &ACHIEVEMENTS {
        b.0 = b.0.min(x.column);
        b.1 = b.1.min(x.row);
        b.2 = b.2.max(x.column);
        b.3 = b.3.max(x.row);
    }
    b
}

/// `StatFileWriter`'s counters (stat id -> value).
#[derive(Default, Clone, Debug)]
pub struct Stats {
    values: HashMap<i32, i32>,
}

impl Stats {
    pub fn get(&self, id: i32) -> i32 {
        self.values.get(&id).copied().unwrap_or(0)
    }

    pub fn add(&mut self, id: i32, n: i32) {
        *self.values.entry(id).or_insert(0) += n;
    }

    /// `hasAchievementUnlocked`
    pub fn unlocked(&self, i: usize) -> bool {
        self.values.contains_key(&ACHIEVEMENTS[i].id)
    }

    /// `func_27181_b`: no parent, or the parent is unlocked.
    pub fn can_unlock(&self, i: usize) -> bool {
        ACHIEVEMENTS[i].parent.is_none_or(|p| self.unlocked(p))
    }
}
