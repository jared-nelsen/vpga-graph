use uuid::Uuid;

pub struct Connection {
    pub id: String,
    pub source_pin: Uuid,
    pub target_pin: Uuid,
    pub state: i32,
}

impl Connection {
    
    pub fn new(id: String, source_pin: Uuid, target_pin: Uuid) -> Self {
        Connection { id, source_pin, target_pin, state: 0 }
    }

    pub fn generate_connection_id(source_pin: &Uuid, target_pin: &Uuid) -> String {
        source_pin.to_string() + &target_pin.to_string()
    }

    pub fn is_live(&self) -> bool {
        self.state == 1
    }
    
}