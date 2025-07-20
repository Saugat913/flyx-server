use crate::models::Room;
use rand::{Rng, distr::Alphanumeric};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct AppState {
    pub rooms: Arc<Mutex<HashMap<String, Room>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            rooms: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn generate_room_id() -> String {
        rand::rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect()
    }

    async fn generate_unique_id(&self) -> String {
        loop {
            let id = Self::generate_room_id(); // or generate_numeric_room_id()
            if !self.rooms.lock().await.contains_key(&id) {
                self.rooms.lock().await.insert(id.clone(), Room::new());
                return id;
            }
        }
    }
    pub async fn create_new_room(&self) -> String {
        self.generate_unique_id().await
    }
}
