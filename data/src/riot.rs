use serde::Deserialize;
use std::collections::HashMap;

use crate::{champion::champion::Champion, constants::DDRAGON_API};

// pub async fn get_version() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
//     let res = reqwest::get(format!("{}/api/versions.json", DDRAGON_API))
//         .await?
//         .json::<Vec<String>>()
//         .await?;

//     Ok(res[0].clone())
// }

#[derive(Deserialize)]
struct ChampionResponse {
    data: HashMap<String, Champion>,
}

pub async fn get_champions(
    version: &str,
) -> Result<Vec<Champion>, Box<dyn std::error::Error + Send + Sync>> {
    let res = reqwest::get(format!(
        "{}cdn/{}.1/data/en_US/champion.json",
        DDRAGON_API, version
    ))
    .await?
    .json::<ChampionResponse>()
    .await?;

    Ok(res.data.into_iter().map(|(_, value)| value).collect())
}
