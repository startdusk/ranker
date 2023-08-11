use std::{cell::RefCell, collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct RoomClient {
    pub id: String,
    pub addr: String,
    pub name: String,
    pub join_time: i64,
}

pub type Room = HashMap<String, HashMap<String, RoomClient>>;

#[derive(Debug, Clone)]
pub struct Rooms(Arc<Mutex<RefCell<Room>>>);

impl Rooms {
    pub fn new() -> Self {
        Rooms(Arc::new(Mutex::new(RefCell::new(HashMap::new()))))
    }
    pub async fn add_client(&mut self, room_id: String, client: RoomClient) {
        if let Some(room) = self.0.lock().await.borrow_mut().get_mut(&room_id) {
            room.insert(client.id.clone(), client);
            return;
        }
        self.0
            .lock()
            .await
            .borrow_mut()
            .insert(room_id, HashMap::from([(client.id.clone(), client)]));
    }

    pub async fn remove(&mut self, room_id: String) {
        self.0.lock().await.borrow_mut().remove(&room_id);
    }

    pub async fn remove_client(&mut self, room_id: String, client_id: String) {
        if let Some(room) = self.0.lock().await.borrow_mut().get_mut(&room_id) {
            room.remove(&client_id);
        }
    }

    pub async fn get_client(&self, room_id: String, client_id: String) -> Option<RoomClient> {
        if let Some(room) = self.0.lock().await.borrow().get(&room_id) {
            if let Some(client) = room.get(&client_id) {
                return Some(client.clone());
            }
            return None;
        }
        None
    }
}

impl Default for Rooms {
    fn default() -> Self {
        Self::new()
    }
}
