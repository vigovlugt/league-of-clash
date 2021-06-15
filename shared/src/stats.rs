use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats {
    pub wins: i64,
    pub games: i64,
}

impl Default for Stats {
    fn default() -> Self {
        Stats { wins: 0, games: 0 }
    }
}
