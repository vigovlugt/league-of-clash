use std::collections::HashMap;

use wasm_bindgen::prelude::*;

use crate::{
    bans,
    champion_stats::{self, champion_stats::ChampionStats},
    team::Team,
};

extern crate console_error_panic_hook;

#[wasm_bindgen]
pub fn create_team() -> JsValue {
    console_error_panic_hook::set_once();

    JsValue::from_serde(&Team::new(Vec::new(), "euw1".to_owned())).unwrap()
}

#[wasm_bindgen]
pub fn get_bans(team_stats: &JsValue) -> JsValue {
    console_error_panic_hook::set_once();

    let team_stats = team_stats
        .into_serde::<HashMap<String, Vec<ChampionStats>>>()
        .unwrap();

    let bans = bans::get_bans(&team_stats);

    JsValue::from_serde(&bans).unwrap()
}

#[wasm_bindgen]
pub async fn get_champion_stats_for_player(
    summoner_name: JsValue,
    region: JsValue,
    season_id: JsValue,
) -> Result<JsValue, JsValue> {
    console_error_panic_hook::set_once();

    let summoner_name: String = summoner_name.into_serde().unwrap();
    let region: String = region.into_serde().unwrap();
    let season_id = season_id.into_serde().unwrap();

    let (_, champion_stats) =
        champion_stats::get_champion_stats_for_player(&summoner_name, &region, season_id)
            .await
            .unwrap();

    Ok(JsValue::from_serde(&champion_stats).unwrap())
}

#[wasm_bindgen]
pub async fn get_team_champion_stats(
    team: JsValue
) -> Result<JsValue, JsValue> {
    console_error_panic_hook::set_once();
    
    let team = team
        .into_serde::<Team>()
        .unwrap();

    let team_stats = team.get_champion_stats(16).await.unwrap();

    Ok(JsValue::from_serde(&team_stats).unwrap())
}