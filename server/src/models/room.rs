use crate::models::NominationID;
use dashmap::DashMap;
use tokio::sync::broadcast;
use tokio::sync::broadcast::{Receiver, Sender};

#[derive(Debug, Clone)]
pub struct RoomClient {
    pub id: String,
    pub addr: String,
    pub name: String,
    pub join_time: i64,
}

pub type Room = DashMap<String, Vote>;

#[derive(Debug, Clone)]
pub struct Vote {
    pub room_id: String,
    pub list: Vec<NominationID>,
    pub clients: DashMap<String, RoomClient>,

    pub sender: Sender<String>,
}

impl Vote {
    pub fn broadcast(&mut self, message: String) {
        let _ = self.sender.send(message);
    }

    pub fn subscribe(&self) -> Receiver<String> {
        self.sender.subscribe()
    }
}

#[derive(Debug)]
pub struct Rooms {
    room: Room,
    broadcast_capacity: usize,
}

impl Rooms {
    pub fn new(broadcast_capacity: usize) -> Self {
        Self {
            room: DashMap::new(),
            broadcast_capacity,
        }
    }
    pub async fn add_client(&self, room_id: String, client: RoomClient) -> Vote {
        if let Some(room) = self.room.get_mut(&room_id) {
            room.clients.insert(client.id.clone(), client);
            return room.clone();
        }

        let (tx, _rx) = broadcast::channel(self.broadcast_capacity);
        let clients = DashMap::new();
        clients.insert(client.id.clone(), client);
        let room = Vote {
            room_id: room_id.clone(),
            clients,
            list: Vec::new(),
            sender: tx,
        };
        self.room.insert(room_id.clone(), room.clone());

        room
    }

    pub async fn remove(&self, room_id: String) {
        self.room.remove(&room_id);
    }

    pub async fn remove_client(&self, room_id: String, client_id: String) {
        if let Some(room) = self.room.get_mut(&room_id) {
            room.clients.remove(&client_id);
        }
    }

    pub async fn get_client(&self, room_id: String, client_id: String) -> Option<RoomClient> {
        if let Some(room) = self.room.get(&room_id) {
            if let Some(client) = room.clients.get(&client_id) {
                return Some(client.clone());
            }
            return None;
        }
        None
    }

    pub async fn add_nomination(&self, room_id: String, nomination_id: NominationID) {
        if let Some(mut room) = self.room.get_mut(&room_id) {
            room.list.push(nomination_id);
        }
    }

    pub async fn remove_nomination(&self, room_id: String, nomination_id: NominationID) {
        if let Some(mut room) = self.room.get_mut(&room_id) {
            room.list.retain(|id| id != &nomination_id);
        }
    }

    pub async fn contains_nomination(
        &self,
        room_id: String,
        nomination_ids: Vec<NominationID>,
    ) -> bool {
        if let Some(room) = self.room.get_mut(&room_id) {
            return nomination_ids.iter().all(|x| room.list.contains(x));
        }

        false
    }
}

impl Default for Rooms {
    fn default() -> Self {
        Self::new(100)
    }
}
