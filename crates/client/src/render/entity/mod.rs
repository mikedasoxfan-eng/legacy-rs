//! Literal b1.7.3 mob models and `RenderLiving`, independent of simulation.

pub mod bridge;
pub mod check;
pub mod effects;
pub mod equipped;
pub mod fixed;
pub mod geometry;
pub mod gui_item;
pub mod inventory;
pub mod item_table;
pub mod living;
pub mod models;
pub mod object_check;
pub mod objects;
pub mod pipeline;
pub mod player;
pub mod shot;
pub mod special;

#[cfg(test)]
mod tests;
