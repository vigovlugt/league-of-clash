use league_of_clash_shared::{
    champion_stats::ChampionStats, dataset::Dataset, matchup::Matchup, role::Role,
};
use std::{collections::HashMap, fs::File, io::Write};

use crate::{
    champion::{champion::Champion, champion_stats, matchups},
    riot, ugg,
};

pub async fn create_dataset() -> Result<Dataset, Box<dyn std::error::Error + Sync + Send>> {
    log::info!("Creating dataset");

    let version = ugg::get_ugg_version().await?;

    log::info!("Ugg version: {}", version);

    let champions = riot::get_champions(&version).await?;

    let futures = champions
        .iter()
        .map(|c| champion_stats::get_champion_stats(&version, c.key.parse::<i64>().unwrap()));

    let mut champion_stats = futures::future::join_all(futures)
        .await
        .into_iter()
        .map(|x| {
            let stats = x.unwrap();

            (stats.champion_id, stats)
        })
        .collect::<HashMap<i64, ChampionStats>>();

    let matchup_futures = champions.iter().map(|c| get_champion_matchups(&version, c));

    let champion_matchups = futures::future::join_all(matchup_futures)
        .await
        .into_iter()
        .collect::<HashMap<i64, HashMap<Role, Vec<Matchup>>>>();

    for (champion_id, matchup_stats) in champion_matchups {
        champion_stats
            .get_mut(&champion_id)
            .unwrap()
            .matchups_by_role = matchup_stats;
    }

    let mut total_games = 0;

    for champion_stats in champion_stats.values() {
        total_games += champion_stats.games;
    }

    Ok(Dataset {
        champion_stats,
        games: total_games,
    })
}

async fn get_champion_matchups(version: &str, c: &Champion) -> (i64, HashMap<Role, Vec<Matchup>>) {
    let champion_id = c.key.parse::<i64>().unwrap();
    let matchup_data = matchups::get_champion_matchups(version, champion_id)
        .await
        .unwrap();

    (champion_id, matchup_data)
}

pub fn save_dataset(
    dataset: &Dataset,
    path: &str,
) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    let mut file = File::create(path)?;

    file.write_all(serde_json::to_string(&dataset)?.as_bytes())?;

    Ok(())
}
