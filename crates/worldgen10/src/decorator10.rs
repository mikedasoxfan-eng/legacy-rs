//! BiomeDecorator's per-biome settings and decorating lifecycle in Release 1.0.
#[derive(Clone, Copy, Debug)]
pub struct DecoratorSettings {
    pub trees: i32,
    pub flowers: i32,
    pub grass: i32,
    pub dead_bush: i32,
    pub mushrooms: i32,
    pub reeds: i32,
    pub cacti: i32,
    pub waterlilies: i32,
    pub big_mushrooms: i32,
    pub sand: i32,
    pub sand2: i32,
    pub clay: i32,
    pub generate_fluid: bool,
}
impl Default for DecoratorSettings {
    fn default() -> Self {
        Self { trees: 0, flowers: 2, grass: 1, dead_bush: 0, mushrooms: 0,
            reeds: 0, cacti: 0, waterlilies: 0, big_mushrooms: 0,
            sand: 1, sand2: 3, clay: 1, generate_fluid: true }
    }
}
impl DecoratorSettings {
    pub fn for_biome(b: crate::Biome) -> Self {
        use crate::Biome::*;
        let mut d = Self::default();
        match b {
            Desert => { d.trees = -999; d.dead_bush = 2; d.reeds = 50; d.cacti = 10; }
            Plains => { d.trees = -999; d.flowers = 4; d.grass = 10; }
            Forest => { d.trees = 10; d.grass = 2; }
            Taiga => d.trees = 10,
            Swampland => { d.trees = 2; d.flowers = -999; d.dead_bush = 1;
                d.mushrooms = 8; d.reeds = 10; d.waterlilies = 4; }
            MushroomIsland | MushroomIslandShore => { d.trees = -100; d.flowers = -100;
                d.grass = -100; d.mushrooms = 1; d.big_mushrooms = 1; }
            _ => {}
        }
        d
    }
}
// A singleplayer generator executes on one thread. Keep singleton lifecycle
// state separate from the world's RNG and from independent test threads.
thread_local! { static ACTIVE: std::cell::RefCell<[bool; 16]> = const { std::cell::RefCell::new([false; 16]) }; }
pub(crate) fn begin(b: crate::Biome) {
    ACTIVE.with(|a| {
        let mut a = a.borrow_mut();
        assert!(!a[b as usize], "Already decorating!!");
        a[b as usize] = true;
    });
}
pub(crate) fn finish(b: crate::Biome) { ACTIVE.with(|a| a.borrow_mut()[b as usize] = false); }
