use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Matchup {
    pub champion_id: i64,
    pub wins: i64,
    pub games: i64,
}
