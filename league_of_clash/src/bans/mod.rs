use std::collections::HashMap;

use crate::champion_stats::champion_stats::ChampionStats;

use self::{ban_creator::BanCreator, bans::Bans};

pub mod ban;
pub mod ban_creator;
pub mod ban_set;
pub mod bans;

pub fn get_bans(team_stats: &HashMap<String, Vec<ChampionStats>>) -> Vec<Bans> {
    let mut ban_creator = BanCreator::new();

    for (summoner_name, champion_stats) in team_stats {
        ban_creator.add_summoner_stats(summoner_name, champion_stats)
    }

    ban_creator.get_best_bans()
}
