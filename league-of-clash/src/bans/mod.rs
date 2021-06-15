use std::collections::HashMap;

use crate::player_stats::PlayerStats;

use self::{ban_creator::BanCreator, bans::Bans};

pub mod ban;
pub mod ban_creator;
pub mod ban_set;
pub mod bans;

pub fn get_bans(player_stats: &HashMap<String, PlayerStats>) -> Vec<Bans> {
    let mut ban_creator = BanCreator::new();

    for (summoner_name, stats) in player_stats {
        ban_creator.add_summoner_stats(&summoner_name, &stats.get_sorted_champion_stats())
    }

    ban_creator.get_best_bans()
}
