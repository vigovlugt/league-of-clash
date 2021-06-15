use league_of_clash_shared::role::Role;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleStats {
    pub role: Role,
    pub wins: i64,
    pub games: i64,
}

impl RoleStats {
    pub fn new(role: Role, wins: i64, games: i64) -> RoleStats {
        RoleStats { role, wins, games }
    }
}
