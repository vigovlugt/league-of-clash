use graphql_client::*;
use league_of_clash_shared::role::Role;

use crate::matches::Match;

use super::UGG_API;

type UnixTimestamp = i64;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/ugg_schema.json",
    query_path = "graphql/match_history_query.graphql",
    response_derives = "Debug"
)]
struct FetchMatchSummaries;

impl Match {
    fn from_ugg_match(ugg_match: fetch_match_summaries::PlayerMatchHistoryMatchSummaries) -> Self {
        Match {
            assists: ugg_match.assists.unwrap(),
            champion_id: ugg_match.champion_id.unwrap(),
            deaths: ugg_match.deaths.unwrap(),
            kill_participation: ugg_match.kill_participation.unwrap(),
            kills: ugg_match.kills.unwrap(),
            match_creation_time: ugg_match.match_creation_time.unwrap(),
            match_duration: ugg_match.match_duration.unwrap(),
            match_id: ugg_match.match_id.unwrap(),
            summoner_name: ugg_match.summoner_name.unwrap(),
            ps_hard_carry: ugg_match.ps_hard_carry.unwrap(),
            ps_team_play: ugg_match.ps_team_play.unwrap(),
            win: ugg_match.win.unwrap(),
            role: Role::from_i64(ugg_match.role.unwrap()),
        }
    }
}

pub async fn get(summoner_name: &str, region: &str, season_id: i64, games: i64) -> Vec<Match> {
    info!("Getting UGG match history: {}", summoner_name);

    let end_page = (games as f64 / 20f64).ceil() as i64;

    let match_futures = (1..=end_page)
        .collect::<Vec<i64>>()
        .into_iter()
        .map(|n| get_page(summoner_name, region, season_id, n))
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
    region: &str,
    season_id: i64,
    page: i64,
) -> Result<Vec<Match>, Box<dyn std::error::Error + Send + Sync>> {
    let variables = fetch_match_summaries::Variables {
        summoner_name: summoner_name.to_string(),
        page,
        region_id: region.to_owned(),
        season_id,
    };

    let query = FetchMatchSummaries::build_query(variables);

    let client = reqwest::Client::new();
    let res = client.post(UGG_API).json(&query).send().await?;

    res.error_for_status_ref()?;

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
        .map(|x| Match::from_ugg_match(x.unwrap()))
        .collect::<Vec<_>>())
}
