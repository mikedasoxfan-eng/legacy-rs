//! Bit-exact port of Minecraft Beta 1.7.3 overworld generation.
//!
//! Correctness is defined by the Java oracle in `tools/oracle`, which runs Mojang's
//! original bytecode; see `src/bin/compare.rs` and `tools/verify.sh`.
//!
//! - `overworld`: per-chunk terrain (`provideChunk`), order-independent.
//! - `world` + `populate` + `features`: decoration, which in the original depends on
//!   chunk load order and lighting timing; `scenario` replays vanilla's orders.

pub mod biome;
pub mod block;
pub mod caves;
pub mod features;
pub mod jmath;
pub mod math;
pub mod noise;
pub mod overworld;
pub mod populate;
pub mod random;
pub mod scenario;
pub mod world;

pub use biome::{Biome, BiomeSource};
pub use overworld::{OverworldGenerator, RawChunk};
pub use random::JavaRandom;
pub use world::World;
