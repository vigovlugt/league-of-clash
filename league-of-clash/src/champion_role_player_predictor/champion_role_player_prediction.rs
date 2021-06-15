use league_of_clash_shared::role::Role;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChampionRolePlayerPrediction {
    pub champion_id: i64,
    pub role: Role,
    pub summoner_name: String,
    pub probability: f64,
}

impl ChampionRolePlayerPrediction {
    pub fn new(champion_id: i64, role: Role, summoner_name: String, probability: f64) -> Self {
        ChampionRolePlayerPrediction {
            champion_id,
            role,
            summoner_name,
            probability,
        }
    }
}
