use crate::{matches::Match, utils};

#[derive(Default)]
pub struct WrCalculator {
    pub matches: i64,
    pub wins: i64,
}

impl WrCalculator {
    pub fn add_game(&mut self, game: &Match) {
        if game.win {
            self.wins += 1;
        }
        self.matches += 1;
    }

    pub fn get_score(self) -> f64 {
        if self.matches < 10 {
            return 0.0;
        }

        let (lower, _) =
            utils::stats::get_wilson_interval(self.wins as f64, self.matches as f64, 1);

        lower
    }
}
