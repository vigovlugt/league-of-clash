use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::champion_stats::ChampionStats;

#[derive(Serialize, Deserialize)]
pub struct Dataset {
    pub champion_stats: HashMap<i64, ChampionStats>,
    pub games: i64,
}

impl Dataset {
    pub fn from_string(string: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(string)?)
    }
}
