use actix::{fut, ActorContext, ActorFuture, ContextFutureSpawner, WrapFuture};
use actix::{Actor, Addr, Running, StreamHandler};
use actix::{AsyncContext, Handler};
use actix_web_actors::ws;
use actix_web_actors::ws::Message::Text;
use std::time::{Duration, Instant};
use uuid::Uuid;

use crate::websocket::ws_events::MessageEvent;

use super::ws_events::{ConnectEvent, DisconnectEvent, WsMessageEvent};
use super::ws_server::WsServer;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(15);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

pub struct WsConnection {
    id: Uuid,
    room_id: Uuid,
    server: Addr<WsServer>,
    heartbeat: Instant,
}

impl WsConnection {
    pub fn new(room_id: Uuid, server: Addr<WsServer>) -> WsConnection {
        WsConnection {
            id: Uuid::new_v4(),
            heartbeat: Instant::now(),
            server,
            room_id,
        }
    }
}

impl Actor for WsConnection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);

        let addr = ctx.address();
        self.server
            .send(ConnectEvent {
                addr: addr.recipient(),
                room_id: self.room_id,
                socket_id: self.id,
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                if res.is_err() {
                    ctx.stop();
                }

                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.server.do_send(DisconnectEvent {
            socket_id: self.id,
            room_id: self.room_id,
        });
        Running::Stop
    }
}

impl WsConnection {
    fn heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |socket, ctx| {
            if Instant::now().duration_since(socket.heartbeat) > CLIENT_TIMEOUT {
                socket.server.do_send(DisconnectEvent {
                    socket_id: socket.id,
                    room_id: socket.room_id,
                });
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConnection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.heartbeat = Instant::now();
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(Text(s)) => self.server.do_send(MessageEvent {
                socket_id: self.id,
                msg: s,
                room_id: self.room_id,
            }),
            _ => (),
        }
    }
}

impl Handler<WsMessageEvent> for WsConnection {
    type Result = ();

    fn handle(&mut self, msg: WsMessageEvent, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}
