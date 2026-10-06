//! Literal b1.7.3 mob models and `RenderLiving`, independent of simulation.

pub mod bridge;
pub mod check;
pub mod chest;
pub mod effects;
pub mod equipped;
pub mod fixed;
pub mod geometry;
pub mod gui_item;
pub mod inventory;
pub mod item_table;
pub mod living;
pub mod models;
pub mod models_release;
pub mod models_release_check;
pub mod object_check;
pub mod objects;
pub mod pipeline;
pub mod player;
pub mod raw_living181;
pub mod shot;
pub mod special;
pub mod vehicles;

#[cfg(test)]
mod tests;

pub mod bed_model;

pub mod manager;
pub mod manager_check9;
pub mod tile_special;
pub mod tile_special_check9;
pub mod tile_manager;
pub mod tile_manager_check9;

pub mod render_release;
