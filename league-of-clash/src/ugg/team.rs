use crate::{player_stats::PlayerStats, team::Team};

use super::get_player_stats;

impl Team {
    pub async fn get_champion_stats(
        &self,
        season_id: i64,
    ) -> Result<Vec<PlayerStats>, Box<dyn std::error::Error + Send + Sync>> {
        let futures = self
            .players
            .iter()
            .map(|p| get_player_stats(p, &self.region, season_id))
            .collect::<Vec<_>>();

        let results = futures::future::join_all(futures).await;

        Ok(results.into_iter().map(|x| x.unwrap()).collect())
    }
}
