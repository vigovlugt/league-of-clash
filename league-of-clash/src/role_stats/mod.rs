use std::collections::HashMap;

use itertools::Itertools;
use league_of_clash_shared::role::Role;

use crate::matches::Match;

use self::{role_stats::RoleStats, role_stats_creator::RoleStatsCreator};

pub mod role_stats;
pub mod role_stats_creator;

pub fn get_stats_by_role(matches: &Vec<Match>) -> HashMap<Role, RoleStats> {
    let mut champion_stats = Vec::new();

    for (role, group) in matches.iter().into_group_map_by(|m| m.role) {
        champion_stats.push((role, get_role_stats(role, group)))
    }

    champion_stats.into_iter().collect()
}

fn get_role_stats(role: Role, group: Vec<&Match>) -> RoleStats {
    let mut creator = RoleStatsCreator::new(role);
    for game in group.into_iter() {
        creator.add_game(game);
    }

    creator.get()
}
