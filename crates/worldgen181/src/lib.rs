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
pub mod block;
pub mod caves;
pub mod container;
pub mod convert;
pub mod entity;
pub mod decoration_verify;
pub mod features;
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
pub mod world;

pub use biome::{Biome, BiomeSource};
pub use overworld::{OverworldGenerator, RawChunk};
pub use random::JavaRandom;
pub use world::World;
