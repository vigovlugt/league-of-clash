use std::{cmp::Ordering, collections::HashMap};

use league_of_clash_shared::role::Role;
use serde::{Deserialize, Serialize};

use crate::{champion_stats::champion_stats::ChampionStats, role_stats::role_stats::RoleStats};

#[derive(Serialize, Deserialize)]
pub struct PlayerStats {
    pub summoner_name: String,

    pub wins: i64,
    pub games: i64,

    pub rank: String,
    pub tier: String,

    pub champion_stats: HashMap<i64, ChampionStats>,
    pub role_stats: HashMap<Role, RoleStats>,
}

impl PlayerStats {
    pub fn new(summoner_name: &str, wins: i64, games: i64, rank: String, tier: String) -> Self {
        PlayerStats {
            summoner_name: summoner_name.to_string(),
            wins,
            games,
            rank,
            tier,
            champion_stats: HashMap::new(),
            role_stats: HashMap::new(),
        }
    }

    pub fn set_champion_stats(&mut self, champion_stats: HashMap<i64, ChampionStats>) {
        self.champion_stats = champion_stats;
    }

    pub fn set_role_stats(&mut self, role_stats: HashMap<Role, RoleStats>) {
        self.role_stats = role_stats;
    }

    pub fn get_sorted_champion_stats(&self) -> Vec<&ChampionStats> {
        let mut champion_stats_vec = self
            .champion_stats
            .values()
            .collect::<Vec<&ChampionStats>>();

        champion_stats_vec.sort_by(|a, b| {
            let score_order = b.score.partial_cmp(&a.score).unwrap();
            if score_order != Ordering::Equal {
                return score_order;
            }

            b.games.cmp(&a.games)
        });

        champion_stats_vec
    }
}
