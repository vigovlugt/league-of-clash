use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub enum MessageType {
    #[serde(rename = "SYNC_STATE")]
    SyncState,
}

#[derive(Serialize, Deserialize)]
pub struct WsMessage {
    pub r#type: MessageType,
    pub data: Value,
}
