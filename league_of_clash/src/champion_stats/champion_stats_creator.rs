use crate::matches::Match;

use super::champion_stats::ChampionStats;

pub struct ChampionStatsCreator {
    pub total_games: i64,
    pub total_wins: i64,
    pub total_ps_games: i64,

    pub total: ChampionStats,

    pub total_weight: f64,
    pub total_ps_weight: f64,

    // How old Games can be.
    pub days: i64,
    pub pow: f64,
}

impl ChampionStatsCreator {
    pub fn new(champion_id: i64, days: i64, pow: f64) -> Self {
        ChampionStatsCreator {
            total_wins: 0,
            total_games: 0,
            total_ps_games: 0,
            total: ChampionStats::new(champion_id, 0f64, 0f64, 0f64, 0f64, 0f64, 0f64, 0f64, 0, 0),
            total_weight: 0f64,
            total_ps_weight: 0f64,

            days,
            pow,
        }
    }

    pub fn add_game(&mut self, game: &Match) {
        assert_eq!(self.total.champion_id, game.champion_id);

        let days_ago = game.get_days_ago();
        if days_ago > self.days {
            return;
        }

        let weight = ((self.days - days_ago) as f64).powf(self.pow);
        self.total_weight += weight;

        self.total_games += 1;

        if game.win {
            self.total.wins += weight;
            self.total_wins += 1;
        }

        self.total.kills += game.kills as f64 * weight;
        self.total.deaths += game.deaths as f64 * weight;
        self.total.assists += game.assists as f64 * weight;
        self.total.kill_participation += game.kill_participation as f64 * weight;

        if game.can_use_carry_scores() {
            self.total.ps_hard_carry += game.ps_hard_carry as f64 * weight;
            self.total.ps_team_play += game.ps_team_play as f64 * weight;
            self.total_ps_games += 1;
            self.total_ps_weight += weight;
        }
    }

    pub fn get(self) -> ChampionStats {
        ChampionStats::new(
            self.total.champion_id,
            self.total.kills as f64 / self.total_weight,
            self.total.deaths as f64 / self.total_weight,
            self.total.assists as f64 / self.total_weight,
            self.total.kill_participation as f64 / self.total_weight,
            if self.total_ps_games != 0 {
                self.total.ps_hard_carry as f64 / self.total_ps_weight
            } else {
                0f64
            },
            if self.total_ps_games != 0 {
                self.total.ps_team_play as f64 / self.total_ps_weight
            } else {
                0f64
            },
            self.total.wins / self.total_weight * self.total_games as f64,
            self.total_games,
            self.total_wins,
        )
    }
}
