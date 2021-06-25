use actix::prelude::{Actor, Context, Handler, Recipient};
use std::collections::HashMap;
use uuid::Uuid;

use super::{
    ws_events::{ConnectEvent, DisconnectEvent, MessageEvent, WsMessage},
    ws_room::WsRoom,
};

type Socket = Recipient<WsMessage>;

pub struct WsServer {
    sessions: HashMap<Uuid, Socket>,
    rooms: HashMap<Uuid, WsRoom>,
}

impl Actor for WsServer {
    type Context = Context<Self>;
}

impl Default for WsServer {
    fn default() -> WsServer {
        WsServer {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
        }
    }
}

impl WsServer {
    pub fn send_message(&self, receiver: &Uuid, message: &str) {
        if let Some(socket) = self.sessions.get(receiver) {
            let _ = socket.do_send(WsMessage(message.to_owned()));
        }
    }
}

impl Handler<DisconnectEvent> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: DisconnectEvent, _: &mut Context<Self>) {
        if self.sessions.remove(&msg.socket_id).is_some() {
            let room_mut = self.rooms.get_mut(&msg.room_id).unwrap();

            if room_mut.sockets.len() > 1 {
                room_mut.on_disconnect(&msg.socket_id);
            } else {
                self.rooms.remove(&msg.room_id);
            }
        }
    }
}

impl Handler<ConnectEvent> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: ConnectEvent, _: &mut Context<Self>) -> Self::Result {
        self.sessions.insert(msg.socket_id, msg.addr);

        let room_mut = self
            .rooms
            .entry(msg.room_id)
            .or_insert_with(WsRoom::default);

        room_mut.on_connect(msg.socket_id);
    }
}

impl Handler<MessageEvent> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: MessageEvent, _: &mut Context<Self>) -> Self::Result {
        let room = self.rooms.get(&msg.room_id).unwrap();

        room.on_message(&self, msg.socket_id, msg.msg);
    }
}
