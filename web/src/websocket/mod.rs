use self::ws_server::WsServer;
use crate::websocket::ws_connection::WsConnection;
use actix::Addr;
use actix_web::{
    web::{self, Data},
    Error, HttpRequest, HttpResponse,
};
use actix_web_actors::ws;

pub mod ws_connection;
pub mod ws_events;
pub mod ws_message;
pub mod ws_room;
pub mod ws_server;

pub async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
    data: Data<Addr<WsServer>>,
) -> Result<HttpResponse, Error> {
    let match_info = req.match_info();

    let region = match_info.get("region").unwrap();
    let ally_team = match_info.get("allyTeam").unwrap();
    let enemy_team = match_info.get("enemyTeam").unwrap();

    let name = region.to_owned() + "|" + ally_team + "|" + enemy_team + "|";

    let connection = WsConnection::new(
        uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_OID, name.as_bytes()),
        data.get_ref().clone(),
    );

    let response = ws::start(connection, &req, stream);

    response
}
