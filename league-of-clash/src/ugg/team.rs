use std::collections::HashMap;

use crate::{player_stats::PlayerStats, team::Team};

use super::get_player_stats;

impl Team {
    pub async fn get_player_stats(
        &self,
        season_id: i64,
    ) -> Result<HashMap<String, PlayerStats>, Box<dyn std::error::Error + Send + Sync>> {
        let futures = self
            .players
            .iter()
            .map(|p| get_player_stats_key_value(p, &self.region, season_id))
            .collect::<Vec<_>>();

        let results = futures::future::join_all(futures).await;

        Ok(results.into_iter().map(|x| x.unwrap()).collect())
    }
}

async fn get_player_stats_key_value(
    p: &str,
    region: &str,
    season_id: i64,
) -> Result<(String, PlayerStats), Box<dyn std::error::Error + Send + Sync>> {
    Ok((
        p.to_string(),
        get_player_stats(p, &region, season_id).await?,
    ))
}
