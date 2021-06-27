use std::collections::HashMap;

use league_of_clash_shared::{
    champion_stats::ChampionStats, rank::Rank, region::Region, role::Role, stats::Stats,
};
use serde_json::Value;

use crate::ugg;

pub async fn get_champion_stats(
    version: &str,
    champion_id: i64,
) -> Result<ChampionStats, Box<dyn std::error::Error + Send + Sync>> {
    log::info!("Getting champion stats for: {}", champion_id);

    let res =
        ugg::get_ugg_response::<Value>(&version.replace(".", "_"), "overview", champion_id).await?;

    log::info!("Gotten champion stats for: {}", champion_id);

    let data = res
        .get(&Region::World)
        .unwrap()
        .get(&Rank::PlatinumPlus)
        .unwrap();

    let stats_by_role = data
        .into_iter()
        .map(|(role, value)| {
            return (
                *role,
                Stats {
                    wins: value[0][6][0].as_i64().unwrap(),
                    games: value[0][6][1].as_i64().unwrap(),
                },
            );
        })
        .collect::<HashMap<Role, Stats>>();

    let mut total_games = 0;
    let mut total_wins = 0;

    for stats in stats_by_role.values() {
        total_games += stats.games;
        total_wins += stats.wins;
    }

    Ok(ChampionStats {
        champion_id,
        stats_by_role,
        wins: total_wins,
        games: total_games,
        matchups_by_role: HashMap::new(),
        duos_by_role: HashMap::new(),
    })
}
