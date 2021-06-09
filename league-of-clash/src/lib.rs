#[cfg(not(target_arch = "wasm32"))]
#[macro_use]
extern crate log;

pub mod bans;
pub mod champion_score;
pub mod champion_stats;
pub mod matches;
pub mod player_stats;
pub mod team;

pub mod utils;

#[cfg(not(target_arch = "wasm32"))]
pub mod ugg;

#[cfg(target_arch = "wasm32")]
pub mod web;
