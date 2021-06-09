use actix_web::{middleware, web, App, HttpRequest, HttpServer, Responder};
use env_logger;
use league_of_clash::{team::Team, utils};

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
        .get_champion_stats(utils::get_current_season())
        .await
        .unwrap();

    web::Json(stats)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default()).route(
            "/api/team/{region}/{team}",
            web::get().to(champion_stats_team),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
