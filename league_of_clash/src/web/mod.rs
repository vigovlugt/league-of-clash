use std::collections::HashMap;

use wasm_bindgen::prelude::*;

use crate::{
    bans::{self},
    champion_stats::champion_stats::ChampionStats,
    team::Team,
};

#[wasm_bindgen]
pub fn create_team() -> JsValue {
    JsValue::from_serde(&Team::new(Vec::new(), "euw1".to_owned())).unwrap()
}

#[wasm_bindgen]
pub fn get_bans(team_stats: &JsValue) -> JsValue {
    let team_stats = team_stats
        .into_serde::<HashMap<String, Vec<ChampionStats>>>()
        .unwrap();

    let bans = bans::get_bans(&team_stats);

    JsValue::from_serde(&bans).unwrap()
}
