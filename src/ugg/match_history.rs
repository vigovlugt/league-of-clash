use graphql_client::*;

type UnixTimestamp = i64;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/ugg_schema.json",
    query_path = "graphql/match_history_query.graphql",
    response_derives = "Debug"
)]
pub struct FetchMatchSummaries;

pub async fn get(
    summoner_name: &str,
    games: i64,
) -> Vec<fetch_match_summaries::PlayerMatchHistoryMatchSummaries> {
    info!("Getting UGG match history: {}", summoner_name);

    let end_page = (games as f64 / 20f64).ceil() as i64;

    let match_futures = (1..=end_page)
        .collect::<Vec<i64>>()
        .into_iter()
        .map(|n| get_page(summoner_name, n))
        .collect::<Vec<_>>();

    let results = futures::future::join_all(match_futures).await;

    results
        .into_iter()
        .map(|x| x.unwrap())
        .flatten()
        .collect::<Vec<_>>()
}

async fn get_page(
    summoner_name: &str,
    page: i64,
) -> Result<Vec<fetch_match_summaries::PlayerMatchHistoryMatchSummaries>, Box<dyn std::error::Error>>
{
    let variables = fetch_match_summaries::Variables {
        summoner_name: summoner_name.to_string(),
        page,
    };

    let query = FetchMatchSummaries::build_query(variables);

    let client = reqwest::Client::new();
    let res = client.post("https://u.gg/api").json(&query).send().await?;

    let response_body: Response<fetch_match_summaries::ResponseData> = res.json().await?;

    Ok(response_body
        .data
        .unwrap()
        .fetch_player_match_summaries
        .unwrap()
        .player_match_history
        .match_summaries
        .unwrap()
        .into_iter()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>())
}
