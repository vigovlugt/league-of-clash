use crate::{matches::Match, utils};

#[derive(Default)]
pub struct ChampionScoreTracker {
    pub matches: i64,
    pub wins: i64,

    pub ps_hard_carry: i64,
    pub ps_team_play: i64,
    pub ps_games: i64,

    pub config: ChampionScoreConfig,
}

pub struct ChampionScoreConfig {
    pub max_days: i64,
}

impl ChampionScoreConfig {
    pub fn new(max_days: i64) -> ChampionScoreConfig {
        Self { max_days }
    }
}

impl Default for ChampionScoreConfig {
    fn default() -> Self {
        Self::new(90)
    }
}

impl ChampionScoreTracker {
    pub fn add_game(&mut self, game: &Match) {
        // let days_ago = game.get_days_ago();

        // if days_ago > self.config.max_days {
        //     return;
        // }

        // TODO use matches from normal?
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
        if self.matches < 5 {
            return 0.0;
        }

        let (lower, _) = utils::stats::get_wilson_interval(self.wins as f64, self.matches as f64);

        lower
    }
}
