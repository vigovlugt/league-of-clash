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
pub mod ws_room;
pub mod ws_server;

pub async fn websocket_handler(
    req: HttpRequest,
    stream: web::Payload,
    data: Data<Addr<WsServer>>,
) -> Result<HttpResponse, Error> {
    let match_info = req.match_info();

    let room_id = match_info.get("room_id").unwrap();

    let connection = WsConnection::new(
        uuid::Uuid::parse_str(room_id).unwrap(),
        data.get_ref().clone(),
    );

    let response = ws::start(connection, &req, stream);

    response
}
