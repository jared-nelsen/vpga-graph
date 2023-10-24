use uuid::Uuid;

pub struct Pin {
    pub id: Uuid,
    pub neighbors: Vec<Uuid>,
    pub state: i32,
}

impl Pin {

    pub fn new(id: Uuid) -> Self {
        Pin { id, neighbors: Vec::new(), state: 0 }
    }

    pub fn set_state(&mut self, new_state: i32) {
        self.state = new_state;
    }

    pub fn reset(&mut self) {
        self.state = 0;
    }
    
}