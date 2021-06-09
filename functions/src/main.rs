use league_of_clash::team::Team;
use league_of_clash::utils;
use serde::Deserialize;
use std::env;
use std::net::Ipv4Addr;
use warp::Filter;

#[derive(Deserialize)]
struct ChampionStatsTeamParams {
    team: String,
    region: Option<String>,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let server = warp::get()
        .and(warp::path("api"))
        .and(warp::path("championstats"))
        .and(warp::query::raw())
        .and_then(champion_stats_team);

    let port: u16 = match env::var("FUNCTIONS_CUSTOMHANDLER_PORT") {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(server).run((Ipv4Addr::UNSPECIFIED, port)).await
}

async fn champion_stats_team(params: String) -> Result<impl warp::Reply, warp::Rejection> {
    let params: ChampionStatsTeamParams = serde_qs::from_str(&params).unwrap();

    let players = params
        .team
        .split(",")
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();

    let default_region = String::from("euw1");

    log::info!("Getting champion stats for {}", players.join(", "));

    let team = Team::new(players, params.region.unwrap_or(default_region));

    let stats = team
        .get_champion_stats(utils::get_current_season())
        .await
        .unwrap();

    Ok(warp::reply::json(&stats))
}
