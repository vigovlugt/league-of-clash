use graphql_client::*;
use reqwest;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/ugg_schema.json",
    query_path = "graphql/profile_ranks_query.graphql",
    response_derives = "Debug"
)]
pub struct FetchProfileRanks;

pub async fn get(
    summoner_name: &str,
    region: &str,
    season: i64,
) -> Result<Option<fetch_profile_ranks::PlayerRankFieldsRankScores>, Box<dyn std::error::Error>> {
    info!("Getting UGG profile for: {}", summoner_name);

    let variables = fetch_profile_ranks::Variables {
        summoner_name: summoner_name.to_owned(),
        region_id: region.to_owned(),
        season_id: season,
    };
    let query = FetchProfileRanks::build_query(variables);

    let client = reqwest::Client::new();
    let res = client.post("https://u.gg/api").json(&query).send().await?;

    let response_body: Response<fetch_profile_ranks::ResponseData> = res.json().await?;

    let player_rank_fields = response_body
        .data
        .unwrap()
        .fetch_profile_ranks
        .unwrap()
        .player_rank_fields;

    let solo_duo_data = player_rank_fields
        .rank_scores
        .unwrap()
        .into_iter()
        .find(|x| x.as_ref().unwrap().queue_type.as_ref().unwrap() == "ranked_solo_5x5");

    Ok(if let Some(Some(data)) = solo_duo_data {
        Some(data)
    } else {
        None
    })
}
