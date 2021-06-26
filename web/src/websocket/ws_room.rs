use std::collections::HashSet;

use serde_json::Value;
use uuid::Uuid;

pub struct WsRoom {
    pub sockets: HashSet<Uuid>,
    pub current_state: Value,
}

impl Default for WsRoom {
    fn default() -> Self {
        Self {
            sockets: HashSet::new(),
            current_state: Value::Null,
        }
    }
}

impl WsRoom {
    // pub fn send_message(&self, server: &WsServer, msg: &str) {
    //     self.sockets
    //         .iter()
    //         .for_each(|&socket_id| server.send_message(&socket_id, msg));
    // }

    pub fn on_connect(&mut self, socket_id: Uuid) {
        self.sockets.insert(socket_id);
    }

    pub fn on_disconnect(&mut self, socket_id: &Uuid) {
        self.sockets.remove(socket_id);
    }
}
