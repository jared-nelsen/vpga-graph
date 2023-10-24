use uuid::Uuid;

pub struct Pin {
    pub id: Uuid,
    pub neighbors: Vec<Uuid>,
    pub state: i8,
}

impl Pin {

    pub fn new(id: Uuid) -> Self {
        Pin { id, neighbors: Vec::new(), state: 0 }
    }

    pub fn reset(&mut self) {
        self.state = 0;
    }
    
}