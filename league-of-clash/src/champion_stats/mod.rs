pub mod champion_stats;
pub mod champion_stats_creator;

extern crate chrono;

use std::collections::HashMap;

use self::champion_stats::ChampionStats;
use self::champion_stats_creator::ChampionStatsCreator;
use crate::matches::Match;
use itertools::Itertools;

pub fn get_stats_by_champion(matches: &Vec<Match>) -> HashMap<i64, ChampionStats> {
    let mut champion_stats = Vec::new();

    for (champion_id, group) in matches.iter().into_group_map_by(|m| m.champion_id) {
        champion_stats.push((champion_id, get_champion_stats(champion_id, group)))
    }

    champion_stats.into_iter().collect()
}

fn get_champion_stats(champion_id: i64, group: Vec<&Match>) -> ChampionStats {
    let mut creator = ChampionStatsCreator::new(champion_id);
    for game in group.into_iter() {
        creator.add_game(game);
    }

    creator.get()
}
