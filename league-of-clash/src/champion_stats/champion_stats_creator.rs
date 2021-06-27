use std::collections::HashMap;

use league_of_clash_shared::stats::Stats;

use crate::{
    champion_score::{weighted_calculator::WeightedCalculator, wr_calculator::WrCalculator},
    matches::Match,
};

use super::champion_stats::ChampionStats;

pub struct ChampionStatsCreator {
    pub total_games: i64,
    pub total_wins: i64,
    pub total_ps_games: i64,

    pub total_recent_games: i64,

    pub total: ChampionStats,
    pub score_calculator: WeightedCalculator,
    pub score2_calculator: WrCalculator,
}

impl ChampionStatsCreator {
    pub fn new(champion_id: i64) -> Self {
        ChampionStatsCreator {
            total_wins: 0,
            total_games: 0,
            total_ps_games: 0,
            total: ChampionStats::new(
                champion_id,
                0f64,
                0f64,
                0f64,
                0f64,
                0f64,
                0f64,
                0,
                0,
                0.0,
                0.0,
                false,
                HashMap::new(),
            ),
            score_calculator: WeightedCalculator::default(),
            score2_calculator: WrCalculator::default(),
            total_recent_games: 0,
        }
    }

    pub fn add_game(&mut self, game: &Match) {
        assert_eq!(self.total.champion_id, game.champion_id);

        let stats_by_role = self
            .total
            .stats_by_role
            .entry(game.role)
            .or_insert(Stats::default());

        if game.get_days_ago() <= 30 {
            self.total_recent_games += 1;
        }

        self.total_games += 1;
        stats_by_role.games += 1;

        if game.win {
            self.total_wins += 1;
            stats_by_role.wins += 1;
        }

        self.total.kills += game.kills as f64;
        self.total.deaths += game.deaths as f64;
        self.total.assists += game.assists as f64;
        self.total.kill_participation += game.kill_participation as f64;

        if game.can_use_carry_scores() {
            self.total.ps_hard_carry += game.ps_hard_carry as f64;
            self.total.ps_team_play += game.ps_team_play as f64;
            self.total_ps_games += 1;
        }

        self.score_calculator.add_game(game);
        self.score2_calculator.add_game(game);
    }

    pub fn get(self) -> ChampionStats {
        ChampionStats::new(
            self.total.champion_id,
            self.total.kills as f64 / self.total_games as f64,
            self.total.deaths as f64 / self.total_games as f64,
            self.total.assists as f64 / self.total_games as f64,
            self.total.kill_participation as f64 / self.total_games as f64,
            if self.total_ps_games != 0 {
                self.total.ps_hard_carry as f64 / self.total_ps_games as f64
            } else {
                0f64
            },
            if self.total_ps_games != 0 {
                self.total.ps_team_play as f64 / self.total_ps_games as f64
            } else {
                0f64
            },
            self.total_wins,
            self.total_games,
            self.score_calculator.get_score(),
            self.score2_calculator.get_score(),
            self.total_recent_games > 0,
            self.total.stats_by_role,
        )
    }
}
