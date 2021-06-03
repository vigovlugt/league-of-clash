pub mod champion_stats;
pub mod champion_stats_creator;

extern crate chrono;

use std::cmp::Ordering;

use self::champion_stats::ChampionStats;
use self::champion_stats_creator::ChampionStatsCreator;
use crate::matches::Match;
use crate::ugg;
use itertools::Itertools;

pub fn get_stats_by_champion(matches: Vec<Match>) -> Vec<ChampionStats> {
    let mut champion_stats = Vec::new();

    for (champion_id, group) in &matches.into_iter().into_group_map_by(|m| m.champion_id) {
        champion_stats.push(get_champion_stats(*champion_id, group))
    }

    champion_stats.sort_by(|a, b| {
        let score_order = b.score.partial_cmp(&a.score).unwrap();
        if score_order != Ordering::Equal {
            return score_order;
        }

        b.games.cmp(&a.games)
    });

    champion_stats
}

fn get_champion_stats(champion_id: i64, group: &Vec<Match>) -> ChampionStats {
    let mut creator = ChampionStatsCreator::new(champion_id);
    for game in group.into_iter() {
        creator.add_game(game);
    }

    creator.get()
}

pub async fn get_champion_stats_for_player(
    summoner_name: &str,
    region: &str,
    season_id: i64,
) -> Result<(String, Vec<ChampionStats>), Box<dyn std::error::Error + Send + Sync>> {
    let matches = ugg::get_match_history_for_player(summoner_name, region, season_id).await?;

    let stats = get_stats_by_champion(matches);

    Ok((summoner_name.to_owned(), stats))
}
