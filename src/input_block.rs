use uuid::Uuid;

pub struct InputBlock {
    pub width: i32,
    pub pins: Vec<Uuid>,
}