//! Bit-exact port of Minecraft Beta 1.7.3 overworld and Nether generation.
//!
//! Correctness is defined by the Java oracle in `tools/oracle`, which runs Mojang's
//! original bytecode; see `src/bin/compare.rs` and `tools/verify.sh`.
//!
//! - `overworld`: per-chunk terrain (`provideChunk`), order-independent.
//! - `nether`: Nether terrain/caves and population through a dimension-aware `World`.
//! - `world` + `populate` + `features`: decoration, which in the original depends on
//!   chunk load order and lighting timing; `scenario` replays vanilla's orders.

pub mod biome;
pub mod potion_helper;
pub mod enchantment;
mod stack_tags;
pub mod item10;
pub mod end;
pub mod decorator10;
pub mod direction10;
pub mod saved_data10;
pub mod hashmap10;
pub mod profiler;
pub mod threaded_io10;
pub mod genlayer;
mod intcache;
pub mod block;
pub mod caves;
pub mod container;
pub mod convert;
pub mod entity;
pub mod decoration_verify;
pub mod features;
pub mod features181;
pub mod terrain_verify;
pub mod world3_verify;
pub mod weighted;
pub mod structures;
pub mod sky;
pub mod jmath;
pub mod math;
pub mod maps;
pub mod nbt;
pub mod nether;
pub mod noise;
pub mod overworld;
pub mod populate;
pub mod random;
pub mod scenario;
pub mod stats_table;
pub mod sign;
pub mod storage;
pub mod storage181;
pub mod world;

pub use biome::{Biome, BiomeSource};
pub use overworld::{OverworldGenerator, RawChunk};
pub use random::JavaRandom;
pub use world::World;

pub mod region_cache181;
pub mod save_format_old181;
pub mod legacy_chunk181;
