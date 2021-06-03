use crate::matches::Match;

#[cfg(target_arch = "wasm32")]
pub const UGG_API: &str = "https://cors-anywhere.herokuapp.com/https://u.gg/api";
#[cfg(not(target_arch = "wasm32"))]
pub const UGG_API: &str = "https://u.gg/api";

pub mod match_history;
pub mod profile;

pub async fn get_match_history_for_player(
    summoner_name: &str,
    region: &str,
    season_id: i64,
) -> Result<Vec<Match>, Box<dyn std::error::Error + Send + Sync>> {
    let ranked_data = profile::get(summoner_name, region, season_id).await?;
    if ranked_data.is_none() {
        return Ok(Vec::new());
    }

    let ranked_data = ranked_data.unwrap();

    let num_games = ranked_data.losses.unwrap() + ranked_data.wins.unwrap();

    let games = match_history::get(summoner_name, region, season_id, num_games).await;

    Ok(games)
}
