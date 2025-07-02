use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use crate::room::Room;

pub type MessageSender = UnboundedSender<SignalingMessage>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum SignalingMessage {
    #[serde(rename = "disconnect")]
    Disconnect { client_id: String },
    #[serde(rename = "offer")]
    Offer { client_id: String, sdp: String },
    #[serde(rename = "answer")]
    Answer { client_id: String, sdp: String },
    #[serde(rename = "ice_candidate")]
    IceCandidate {
        client_id: String,
        candidate: String,
    },
    #[serde(rename = "join")]
    Join { client_id: String },
}

pub enum ClientRole {
    Sender,
    Receiver(String),
}

pub struct Context {
    client_role: ClientRole,
    room: Room,
}
