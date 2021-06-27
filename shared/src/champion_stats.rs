use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{duo::Duo, matchup::Matchup, stats::Stats};

use super::role::Role;

#[derive(Serialize, Deserialize)]
pub struct ChampionStats {
    pub champion_id: i64,

    pub wins: i64,
    pub games: i64,

    pub stats_by_role: HashMap<Role, Stats>,
    pub matchups_by_role: HashMap<Role, HashMap<i64, Matchup>>,
    pub duos_by_role: HashMap<Role, HashMap<i64, Duo>>,
}
