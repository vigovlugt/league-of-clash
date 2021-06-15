use std::collections::HashMap;

use league_of_clash_shared::{rank::Rank, region::Region, role::Role};
use regex::Regex;
use reqwest;
use serde::de::DeserializeOwned;

use crate::constants::UGG_API;

pub async fn get_ugg_version() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let html = reqwest::get("https://u.gg/lol/tier-list")
        .await?
        .text()
        .await?;

    let regex = Regex::new("Patch (\\d+\\.\\d+)").unwrap();

    Ok(regex
        .captures(&html)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string())
}

pub type UggResponse<T> = HashMap<Region, HashMap<Rank, HashMap<Role, T>>>;
pub async fn get_ugg_response<T: DeserializeOwned>(
    ugg_version: &str,
    data_type: &str,
    champion_id: i64,
) -> Result<UggResponse<T>, Box<dyn std::error::Error + Send + Sync>> {
    let res = reqwest::get(format!(
        "{}lol/1.1/{}/{}/ranked_solo_5x5/{}/1.4.0.json",
        UGG_API, data_type, ugg_version, champion_id
    ))
    .await?
    .json::<UggResponse<T>>()
    .await?;

    Ok(res)
}
