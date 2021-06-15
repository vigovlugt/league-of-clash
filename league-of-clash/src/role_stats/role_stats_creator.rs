use league_of_clash_shared::role::Role;

use crate::matches::Match;

use super::role_stats::RoleStats;

pub struct RoleStatsCreator {
    pub role: Role,
    pub total_games: i64,
    pub total_wins: i64,
}

impl RoleStatsCreator {
    pub fn new(role: Role) -> Self {
        Self {
            role,
            total_games: 0,
            total_wins: 0,
        }
    }

    pub fn add_game(&mut self, game: &Match) {
        assert_eq!(self.role, game.role);

        self.total_games += 1;

        if game.win {
            self.total_wins += 1;
        }
    }

    pub fn get(self) -> RoleStats {
        RoleStats::new(self.role, self.total_wins, self.total_games)
    }
}
