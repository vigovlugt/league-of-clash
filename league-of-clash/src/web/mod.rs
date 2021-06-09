use wasm_bindgen::prelude::*;

use crate::{bans, player_stats::PlayerStats};

extern crate console_error_panic_hook;

#[wasm_bindgen]
pub fn get_bans(player_stats: &JsValue) -> JsValue {
    console_error_panic_hook::set_once();

    let player_stats = player_stats.into_serde::<Vec<PlayerStats>>().unwrap();

    let bans = bans::get_bans(&player_stats);

    JsValue::from_serde(&bans).unwrap()
}
