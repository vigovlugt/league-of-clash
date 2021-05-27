use crate::matches::Match;

pub mod match_history;
pub mod profile;

pub async fn get_match_history_for_player(
    summoner_name: &str,
    region: &str,
    season_id: i64,
) -> Result<Vec<Match>, Box<dyn std::error::Error>> {
    let ranked_data = profile::get(summoner_name, region, season_id).await?;
    let num_games = ranked_data.losses.unwrap() + ranked_data.wins.unwrap();

    let games = match_history::get(summoner_name, region, season_id, num_games).await;

    Ok(games)
}
