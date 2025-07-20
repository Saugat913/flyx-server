use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, mpsc::UnboundedSender};

//this is dirty so to do refactor it
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

#[derive(Debug, Clone)]
pub struct Room {
    sender: Arc<Mutex<Option<UnboundedSender<SignalingMessage>>>>,
    receivers: Arc<Mutex<HashMap<String, UnboundedSender<SignalingMessage>>>>,
}

impl Room {
    pub fn new() -> Room {
        Self {
            sender: Arc::new(Mutex::new(None)),
            receivers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn is_sender_available(&self) -> bool {
        let sender_lock = self.sender.lock().await;
        sender_lock.is_none()
    }

    pub async fn set_sender(&self, sender: MessageSender) {
        let mut sender_lock = self.sender.lock().await;
        *sender_lock = Some(sender);
    }

    pub async fn add_receiver(&self, client_id: String, sender: MessageSender) {
        let mut receivers_lock = self.receivers.lock().await;
        receivers_lock.insert(client_id, sender);
    }

    pub async fn remove_receiver(&self, client_id: &str) {
        let mut receivers_lock = self.receivers.lock().await;
        receivers_lock.remove(client_id);
    }

    pub async fn remove_sender(&self) {
        let mut sender_lock = self.sender.lock().await;
        *sender_lock = None;
    }

    pub async fn broadcast_to_receivers(&self, message: SignalingMessage) -> Result<(), String> {
        let receivers_lock = self.receivers.lock().await;
        for (client_id, channel) in receivers_lock.iter() {
            if let Err(_) = channel.send(message.clone()) {
                println!("Failed to send message to receiver: {}", client_id);
            }
        }
        Ok(())
    }

    pub async fn send_to_receiver(
        &self,
        client_id: &str,
        message: SignalingMessage,
    ) -> Result<(), String> {
        let receivers_lock = self.receivers.lock().await;
        if let Some(receiver_channel) = receivers_lock.get(client_id) {
            receiver_channel.send(message).map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err("No receiver available".to_string())
        }
    }
    pub async fn send_to_sender(&self, message: SignalingMessage) -> Result<(), String> {
        let sender_lock = self.sender.lock().await;
        if let Some(sender) = sender_lock.as_ref() {
            sender.send(message).map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err("No sender available".to_string())
        }
    }
}
