use crate::{matches::Match, utils};

// const BASE_CARRY_SCORE: i64 = 75;

pub struct Config {
    pub max_days: i64,
}

impl Config {
    pub fn new(max_days: i64) -> Config {
        Self { max_days }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new(30)
    }
}

#[derive(Default)]
pub struct WeightedCalculator {
    pub matches: i64,
    pub wins: i64,

    pub recent_matches: i64,

    pub ps_hard_carry: i64,
    pub ps_team_play: i64,
    pub ps_games: i64,

    pub config: Config,
}

impl WeightedCalculator {
    pub fn add_game(&mut self, game: &Match) {
        let days_ago = game.get_days_ago();

        if days_ago < self.config.max_days {
            self.recent_matches += 1;
        }

        if game.win {
            self.wins += 1;
        }

        self.matches += 1;

        if game.can_use_carry_scores() {
            self.ps_hard_carry += game.ps_hard_carry;
            self.ps_team_play += game.ps_team_play;
            self.ps_games += 1;
        }
    }

    pub fn get_score(self) -> f64 {
        // let total_carry_score = (self.ps_hard_carry + self.ps_team_play) as f64 / 2.0;
        // let average_carry_score = total_carry_score as f64 / self.ps_games as f64;

        let score = self.get_winrate(self.wins, self.matches); // + (average_carry_score - BASE_CARRY_SCORE as f64) / 100.0;

        let score = f64::max(0.0, score);

        if self.recent_matches == 0 {
            return score * 0.5;
        }

        score
    }

    fn get_winrate(&self, wins: i64, matches: i64) -> f64 {
        // Add black box loss.
        let matches = matches + 1;

        let winrate = if matches <= 5 {
            let (highest, _) = utils::stats::get_wilson_interval(wins as f64, matches as f64, 2);
            highest
        } else if matches <= 15 {
            let (highest, _) = utils::stats::get_wilson_interval(wins as f64, matches as f64, 2);
            let (lower, _) = utils::stats::get_wilson_interval(wins as f64, matches as f64, 1);
            let highest_weight = (10 - (matches - 5)) as f64 / 10 as f64;
            highest_weight * highest + (1.0 - highest_weight) * lower
        } else if matches <= 25 {
            let (highest, _) = utils::stats::get_wilson_interval(wins as f64, matches as f64, 1);
            let (lower, _) = utils::stats::get_wilson_interval(wins as f64, matches as f64, 0);
            let highest_weight = (10 - (matches - 15)) as f64 / 10 as f64;
            highest_weight * highest + (1.0 - highest_weight) * lower
        } else {
            let (lower, _) = utils::stats::get_wilson_interval(wins as f64, matches as f64, 0);

            lower
        };

        winrate
    }
}
