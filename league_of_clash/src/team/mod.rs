use std::collections::HashMap;

use crate::{
    champion_stats::{self, champion_stats::ChampionStats},
    utils,
};

pub struct Team {
    pub players: Vec<String>,
    pub region: String,
}

impl Team {
    pub fn new(players: Vec<String>, region: String) -> Self {
        Team { players, region }
    }

    pub async fn get_champion_stats(
        &self,
    ) -> Result<HashMap<String, Vec<ChampionStats>>, Box<dyn std::error::Error>> {
        let season_id = utils::get_current_season();

        let futures = self
            .players
            .iter()
            .map(|p| champion_stats::get_champion_stats_for_player(p, &self.region, season_id))
            .collect::<Vec<_>>();

        let results = futures::future::join_all(futures).await;

        let mut stats_by_player = HashMap::new();

        for res in results {
            let (summoner_name, matches) =
                res.expect("Something has gone wrong while getting matches.");

            stats_by_player.insert(summoner_name, matches);
        }

        Ok(stats_by_player)
    }
}
