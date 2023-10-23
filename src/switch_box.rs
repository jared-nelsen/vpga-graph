use uuid::Uuid;

pub struct SwitchBox {
    pub pin_count: i32,
    pub pins: Vec<Uuid>,
}