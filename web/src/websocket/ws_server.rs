use actix::prelude::{Actor, Context, Handler, Recipient};
use std::collections::HashMap;
use uuid::Uuid;

use super::{
    ws_events::{ConnectEvent, DisconnectEvent, MessageEvent, WsMessageEvent},
    ws_message::{MessageType, WsMessage},
    ws_room::WsRoom,
};

type Socket = Recipient<WsMessageEvent>;

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
    pub fn send_message_str(&self, receiver: &Uuid, message: &str) {
        if let Some(socket) = self.sessions.get(receiver) {
            let _ = socket.do_send(WsMessageEvent(message.to_owned()));
        }
    }

    pub fn send_message(&self, receiver: &Uuid, message: &WsMessage) {
        let data = serde_json::to_string(message).unwrap();

        self.send_message_str(receiver, &data);
    }

    pub fn broadcast_room(&self, room: &Uuid, sender: &Uuid, message: &WsMessage) {
        let room = self.rooms.get(room).unwrap();

        let data = serde_json::to_string(message).unwrap();

        room.sockets.iter().for_each(|socket_id| {
            if socket_id == sender {
                return;
            }

            self.send_message_str(&socket_id, &data)
        });
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

    fn handle(&mut self, event: ConnectEvent, _: &mut Context<Self>) -> Self::Result {
        self.sessions.insert(event.socket_id, event.addr);

        let room_mut = self
            .rooms
            .entry(event.room_id)
            .or_insert_with(WsRoom::default);

        room_mut.on_connect(event.socket_id);

        if !room_mut.current_state.is_null() {
            let new_message = WsMessage {
                r#type: MessageType::SyncState,
                data: room_mut.current_state.clone(),
            };

            self.send_message(&event.socket_id, &new_message)
        }
    }
}

impl Handler<MessageEvent> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: MessageEvent, _: &mut Context<Self>) -> Self::Result {
        if let Ok(message) = serde_json::from_str::<WsMessage>(&msg.msg) {
            match message.r#type {
                MessageType::SyncState => {
                    let room = self.rooms.get_mut(&msg.room_id).unwrap();
                    room.current_state = message.data;

                    let new_message = WsMessage {
                        r#type: MessageType::SyncState,
                        data: room.current_state.clone(),
                    };

                    self.broadcast_room(&msg.room_id, &msg.socket_id, &new_message)
                }
            }
        }
    }
}
