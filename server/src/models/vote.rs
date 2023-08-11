use std::collections::HashMap;

use super::{room::RoomClient, NominationID};

#[derive(Debug, Default)]
pub struct Vote {
    pub room_id: String,
    pub list: Vec<NominationID>,
    pub clients: HashMap<String, RoomClient>,
}
