use std::{cell::RefCell, collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

use super::{vote::Vote, NominationID};

#[derive(Debug, Clone)]
pub struct RoomClient {
    pub id: String,
    pub addr: String,
    pub name: String,
    pub join_time: i64,
}

pub type Room = HashMap<String, Vote>;

#[derive(Debug, Clone)]
pub struct Rooms(Arc<Mutex<RefCell<Room>>>);

impl Rooms {
    pub fn new() -> Self {
        Rooms(Arc::new(Mutex::new(RefCell::new(HashMap::new()))))
    }
    pub async fn add_client(&mut self, room_id: String, client: RoomClient) {
        if let Some(room) = self.0.lock().await.borrow_mut().get_mut(&room_id) {
            room.clients.insert(client.id.clone(), client);
            return;
        }
        self.0.lock().await.borrow_mut().insert(
            room_id.clone(),
            Vote {
                room_id,
                clients: HashMap::from([(client.id.clone(), client)]),
                ..Default::default()
            },
        );
    }

    pub async fn remove(&mut self, room_id: String) {
        self.0.lock().await.borrow_mut().remove(&room_id);
    }

    pub async fn remove_client(&mut self, room_id: String, client_id: String) {
        if let Some(room) = self.0.lock().await.borrow_mut().get_mut(&room_id) {
            room.clients.remove(&client_id);
        }
    }

    pub async fn get_client(&self, room_id: String, client_id: String) -> Option<RoomClient> {
        if let Some(room) = self.0.lock().await.borrow().get(&room_id) {
            if let Some(client) = room.clients.get(&client_id) {
                return Some(client.clone());
            }
            return None;
        }
        None
    }

    pub async fn add_nomination(&self, room_id: String, nomination_id: NominationID) {
        if let Some(room) = self.0.lock().await.borrow_mut().get_mut(&room_id) {
            room.list.push(nomination_id);
        }
    }

    pub async fn contains_nomination(
        &self,
        room_id: String,
        nomination_ids: Vec<NominationID>,
    ) -> bool {
        if let Some(room) = self.0.lock().await.borrow_mut().get_mut(&room_id) {
            return nomination_ids.iter().all(|x| room.list.contains(x));
        }

        false
    }
}

impl Default for Rooms {
    fn default() -> Self {
        Self::new()
    }
}
