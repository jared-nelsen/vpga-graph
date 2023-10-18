use uuid::Uuid;

use crate::connection::Connection;

pub struct Pin {
    pub id: Uuid,
    pub connections: Vec<Connection>,
    pub on: bool,
}