use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use league_of_clash_shared::dataset::Dataset;

use crate::{
    bans, champion_role_player_predictor::ChampionRolePlayerPredictor, draft::Draft,
    player_stats::PlayerStats, team::Team,
};

extern crate console_error_panic_hook;

#[wasm_bindgen]
pub fn get_bans(player_stats: &JsValue) -> JsValue {
    console_error_panic_hook::set_once();

    let player_stats = player_stats
        .into_serde::<HashMap<String, PlayerStats>>()
        .unwrap();

    let bans = bans::get_bans(&player_stats);

    JsValue::from_serde(&bans).unwrap()
}

#[wasm_bindgen]
pub fn create_predictor(player_stats: &JsValue, dataset_str: &str) -> JsValue {
    console_error_panic_hook::set_once();

    let player_stats = player_stats
        .into_serde::<HashMap<String, PlayerStats>>()
        .unwrap();
    let dataset = Dataset::from_string(dataset_str).unwrap();

    let predictor = ChampionRolePlayerPredictor::new(dataset, player_stats).unwrap();

    JsValue::from_serde(&predictor).unwrap()
}

#[wasm_bindgen]
pub fn get_predictions(predictor: &JsValue, team: &JsValue, draft: &JsValue) -> JsValue {
    console_error_panic_hook::set_once();

    let predictor = predictor
        .into_serde::<ChampionRolePlayerPredictor>()
        .unwrap();
    let team = team.into_serde::<Team>().unwrap();
    let draft = draft.into_serde::<Draft>().unwrap();

    let predictions = predictor.predict(&team, &draft);

    JsValue::from_serde(&predictions).unwrap()
}
