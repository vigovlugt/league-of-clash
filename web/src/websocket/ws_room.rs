use std::collections::HashSet;

use uuid::Uuid;

use super::ws_server::WsServer;

pub struct WsRoom {
    pub sockets: HashSet<Uuid>,
}

impl Default for WsRoom {
    fn default() -> Self {
        Self {
            sockets: HashSet::new(),
        }
    }
}

impl WsRoom {
    // pub fn send_message(&self, server: &WsServer, msg: &str) {
    //     self.sockets
    //         .iter()
    //         .for_each(|&socket_id| server.send_message(&socket_id, msg));
    // }

    pub fn broadcast(&self, server: &WsServer, sender: &Uuid, msg: &str) {
        self.sockets.iter().for_each(|socket_id| {
            if socket_id == sender {
                return;
            }

            server.send_message(&socket_id, msg)
        });
    }

    pub fn on_connect(&mut self, socket_id: Uuid) {
        self.sockets.insert(socket_id);
    }

    pub fn on_disconnect(&mut self, socket_id: &Uuid) {
        self.sockets.remove(socket_id);
    }

    pub fn on_message(&self, server: &WsServer, sender: Uuid, msg: String) {
        self.broadcast(server, &sender, &msg)
    }
}
