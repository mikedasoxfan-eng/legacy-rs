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
pub const ACHIEVEMENTS: [Achievement; 27] = [
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
    a(16, "snipeSkeleton", 7, 0, 261, Some(13), true),
    a(17, "diamonds", -1, 5, 264, Some(5), false),
    a(18, "portal", -1, 7, 49, Some(17), false),
    a(19, "ghast", -4, 8, 370, Some(18), true),
    a(20, "blazeRod", 0, 9, 369, Some(18), false),
    a(21, "potion", 2, 8, 373, Some(20), false),
    a(22, "theEnd", 3, 10, 381, Some(20), true),
    a(23, "theEnd2", 4, 13, 122, Some(22), true),
    a(24, "enchantments", -4, 4, 116, Some(17), false),
    a(25, "overkill", -4, 1, 276, Some(24), true),
    a(26, "bookcase", -3, 6, 47, Some(24), false),
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
    /// `field_25102_a`: everything, shown by the screens.
    total: HashMap<i32, i32>,
    /// `field_25101_b`: what goes to the stats file.
    unsent: HashMap<i32, i32>,
    /// `field_27189_c`: set by the first change and never cleared.
    dirty: bool,
    file: Option<super::stats_file::Syncher>,
}

impl Stats {
    /// The client's `StatFileWriter`: stats from `<dir>/stats/stats_<user>_unsent.dat`.
    pub fn open(dir: &std::path::Path, user: &str, map_txt: &str) -> Stats {
        let (file, loaded) = super::stats_file::Syncher::open(dir, user, map_txt);
        let mut s = Stats { file: Some(file), ..Default::default() };
        // func_27179_a
        if let Some(map) = loaded {
            s.dirty = true;
            for (id, n) in map {
                Self::increment(&mut s.unsent, id, n);
                Self::increment(&mut s.total, id, n);
            }
        }
        s
    }

    pub fn get(&self, id: i32) -> i32 {
        self.total.get(&id).copied().unwrap_or(0)
    }

    /// `readStat`
    pub fn add(&mut self, id: i32, n: i32) {
        Self::increment(&mut self.unsent, id, n);
        Self::increment(&mut self.total, id, n);
        self.dirty = true;
    }

    fn increment(map: &mut HashMap<i32, i32>, id: i32, n: i32) {
        let value = map.entry(id).or_insert(0);
        *value = value.wrapping_add(n);
    }

    /// `syncStats`: write the file now (screen changes, world changes, quitting).
    pub fn sync(&mut self) {
        if let Some(f) = &self.file {
            f.sync(&self.unsent);
        }
    }

    /// `func_27178_d`, once per game tick: a periodic write, then the syncher's tick.
    pub fn tick(&mut self) {
        let Some(f) = &mut self.file else { return };
        if self.dirty && f.ready() {
            f.send(&self.unsent);
        }
        // func_27180_b: the sent file's stats plus what is still unsent.
        let (unsent, sent) = f.tick();
        if let Some(map) = unsent {
            self.dirty = true;
            for (id, n) in map { Self::increment(&mut self.unsent, id, n); }
        }
        if let Some(sent) = sent {
            for (id, n) in sent {
                let unsent = self.unsent.get(&id).copied().unwrap_or(0);
                self.total.insert(id, n.wrapping_add(unsent));
            }
        }
    }

    /// `hasAchievementUnlocked`
    pub fn unlocked(&self, i: usize) -> bool {
        self.total.contains_key(&ACHIEVEMENTS[i].id)
    }

    /// `func_27181_b`: no parent, or the parent is unlocked.
    pub fn can_unlock(&self, i: usize) -> bool {
        ACHIEVEMENTS[i].parent.is_none_or(|p| self.unlocked(p))
    }

    pub(crate) fn wait_idle(&self) {
        if let Some(file) = &self.file { file.wait_idle(); }
    }

    pub(crate) fn unsent_snapshot(&self) -> &HashMap<i32, i32> { &self.unsent }
    pub(crate) fn total_snapshot(&self) -> &HashMap<i32, i32> { &self.total }
    pub(crate) fn fixture_mailboxes(&mut self, value: i32) {
        if let Some(file) = &mut self.file {
            file.fixture_mailboxes(HashMap::from([(1000,value),(5242880,0)]),HashMap::from([(1000,-11),(5242880,0)]));
        }
    }
    pub(crate) fn ready(&self) -> bool { self.file.as_ref().is_some_and(|f| f.ready()) }
}

/// `StatList.func_27361_a(id) != null`: the stat ids the original registers.
pub fn is_registered(id: i32) -> bool {
    use super::stats_table as t;
    t::GENERAL.iter().any(|g| g.0 == id)
        || [&t::MINE[..], &t::CRAFT[..], &t::USE[..], &t::BREAK[..]].iter().any(|tb| tb.iter().any(|e| e.1 == id))
        || ACHIEVEMENTS.iter().any(|a| a.id == id)
}
