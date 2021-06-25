use actix::prelude::{Message, Recipient};
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct ConnectEvent {
    pub socket_id: Uuid,
    pub room_id: Uuid,
    pub addr: Recipient<WsMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct DisconnectEvent {
    pub socket_id: Uuid,
    pub room_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct MessageEvent {
    pub socket_id: Uuid,
    pub room_id: Uuid,
    pub msg: String,
}
