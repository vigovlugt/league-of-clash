pub mod champion_stats;
pub mod champion_stats_creator;

extern crate chrono;

use self::champion_stats::ChampionStats;
use self::champion_stats_creator::ChampionStatsCreator;
use crate::matches::Match;
use crate::ugg;
use itertools::Itertools;

pub fn get_stats_by_champion(matches: Vec<Match>, days: i64, pow: f64) -> Vec<ChampionStats> {
    let mut champion_stats = Vec::new();

    for (champion_id, group) in &matches.into_iter().into_group_map_by(|m| m.champion_id) {
        champion_stats.push(get_champion_stats(*champion_id, group, days, pow))
    }

    champion_stats.sort_by(|a, b| b.games.partial_cmp(&a.games).unwrap());

    champion_stats
}

fn get_champion_stats(champion_id: i64, group: &Vec<Match>, days: i64, pow: f64) -> ChampionStats {
    let mut creator = ChampionStatsCreator::new(champion_id, days, pow);

    for game in group.into_iter() {
        creator.add_game(game);
    }

    creator.get()
}

pub async fn get_champion_stats_for_player(
    summoner_name: &str,
    region: &str,
    season_id: i64,
) -> Result<(String, Vec<ChampionStats>), Box<dyn std::error::Error>> {
    let matches = ugg::get_match_history_for_player(summoner_name, region, season_id).await?;

    let stats = get_stats_by_champion(matches, 365, 0.0);

    Ok((summoner_name.to_owned(), stats))
}
