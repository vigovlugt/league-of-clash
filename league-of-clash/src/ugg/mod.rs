use crate::{
    champion_stats::get_stats_by_champion, player_stats::PlayerStats, role_stats::get_stats_by_role,
};

pub const UGG_API: &str = "https://u.gg/api";

pub mod match_history;
pub mod profile;
pub mod team;

pub async fn get_player_stats(
    summoner_name: &str,
    region: &str,
    season_id: i64,
) -> Result<PlayerStats, Box<dyn std::error::Error + Send + Sync>> {
    let mut player_stats = profile::get(summoner_name, region, season_id).await?;

    if player_stats.games == 0 {
        return Ok(player_stats);
    }

    let games = match_history::get(summoner_name, region, season_id, player_stats.games).await;

    let champion_stats = get_stats_by_champion(&games);
    let role_stats = get_stats_by_role(&games);

    player_stats.set_champion_stats(champion_stats);
    player_stats.set_role_stats(role_stats);

    Ok(player_stats)
}
