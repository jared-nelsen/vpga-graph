use uuid::Uuid;

pub struct Pin {
    pub id: Uuid,
    pub neighbors: Vec<Uuid>,
    pub state: i8,
}