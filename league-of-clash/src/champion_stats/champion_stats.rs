use std::collections::HashMap;

use league_of_clash_shared::{role::Role, stats::Stats};
use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ChampionStats {
    pub champion_id: i64,

    pub kills: f64,
    pub deaths: f64,
    pub assists: f64,
    pub kill_participation: f64,

    pub ps_hard_carry: f64,
    pub ps_team_play: f64,

    pub wins: i64,
    pub games: i64,

    pub score: f64,

    pub stats_by_role: HashMap<Role, Stats>,
}

impl ChampionStats {
    pub fn new(
        champion_id: i64,
        kills: f64,
        deaths: f64,
        assists: f64,
        kill_participation: f64,
        ps_hard_carry: f64,
        ps_team_play: f64,
        wins: i64,
        games: i64,
        score: f64,
        stats_by_role: HashMap<Role, Stats>,
    ) -> Self {
        ChampionStats {
            champion_id,
            kills,
            deaths,
            assists,
            kill_participation,
            ps_hard_carry,
            ps_team_play,
            wins,
            games,
            score,
            stats_by_role,
        }
    }

    pub fn carry_score(&self) -> f64 {
        (self.ps_hard_carry + self.ps_team_play) / 2f64
    }

    pub fn kda(&self) -> f64 {
        (self.kills + self.assists) / self.deaths
    }

    pub fn winrate(&self) -> f64 {
        self.wins as f64 / self.games as f64
    }
}
