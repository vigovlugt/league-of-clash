use std::collections::HashMap;

use league_of_clash_shared::stats::Stats;

use crate::{champion_score::champion_score_tracker::ChampionScoreTracker, matches::Match};

use super::champion_stats::ChampionStats;

pub struct ChampionStatsCreator {
    pub total_games: i64,
    pub total_wins: i64,
    pub total_ps_games: i64,

    pub total: ChampionStats,
    pub score_tracker: ChampionScoreTracker,
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
                HashMap::new(),
            ),
            score_tracker: ChampionScoreTracker::default(),
        }
    }

    pub fn add_game(&mut self, game: &Match) {
        assert_eq!(self.total.champion_id, game.champion_id);

        let stats_by_role = self
            .total
            .stats_by_role
            .entry(game.role)
            .or_insert(Stats::default());

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

        self.score_tracker.add_game(game);
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
            self.score_tracker.get_score(),
            self.total.stats_by_role,
        )
    }
}
