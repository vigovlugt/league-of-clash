use serde::{Deserialize, Serialize};

use crate::champion_stats::champion_stats::ChampionStats;

#[derive(Serialize, Deserialize)]
pub struct PlayerStats {
    pub summoner_name: String,

    pub wins: i64,
    pub games: i64,

    pub rank: String,
    pub tier: String,

    pub champion_stats: Vec<ChampionStats>,
}

impl PlayerStats {
    pub fn new(summoner_name: &str, wins: i64, games: i64, rank: String, tier: String) -> Self {
        PlayerStats {
            summoner_name: summoner_name.to_string(),
            wins,
            games,
            rank,
            tier,
            champion_stats: Vec::new(),
        }
    }

    pub fn set_champion_stats(&mut self, champion_stats: Vec<ChampionStats>) {
        self.champion_stats = champion_stats;
    }
}
