use std::env;

mod websocket;

use actix::Actor;
use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use env_logger;
use league_of_clash::{team::Team, utils};
use websocket::ws_server::WsServer;

async fn champion_stats_team(req: HttpRequest) -> impl Responder {
    let match_info = req.match_info();

    let region = match_info.get("region").unwrap();

    let players = match_info
        .get("team")
        .unwrap()
        .split("+")
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();

    let team = Team::new(players, region.to_string());

    let stats = team
        .get_player_stats(utils::get_current_season())
        .await
        .unwrap();

    web::Json(stats)
}

async fn healthcheck() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let ws_manager = WsServer::default().start();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Cors::default().allow_any_origin())
            .data(ws_manager.clone())
            .route("/healthcheck", web::get().to(healthcheck))
            .route(
                "/api/team/{region}/{team}",
                web::get().to(champion_stats_team),
            )
            .route(
                "/ws/room/{region}/{allyTeam}/{enemyTeam}",
                web::get().to(websocket::websocket_handler),
            )
    })
    .bind((
        "0.0.0.0",
        env::var("PORT")
            .unwrap_or(String::from("8080"))
            .parse::<u16>()
            .unwrap(),
    ))?
    .run()
    .await
}
