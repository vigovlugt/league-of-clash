use crate::player_stats::PlayerStats;

use self::{ban_creator::BanCreator, bans::Bans};

pub mod ban;
pub mod ban_creator;
pub mod ban_set;
pub mod bans;

pub fn get_bans(player_stats: &Vec<PlayerStats>) -> Vec<Bans> {
    let mut ban_creator = BanCreator::new();

    for stats in player_stats {
        ban_creator.add_summoner_stats(&stats.summoner_name, &stats.champion_stats)
    }

    ban_creator.get_best_bans()
}
