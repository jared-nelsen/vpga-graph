use uuid::Uuid;

pub struct Connection {
    pub source_pin_id: Uuid,
    pub target_pin_id: Uuid,
}