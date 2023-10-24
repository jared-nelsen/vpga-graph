use uuid::Uuid;

use crate::input_block::InputBlock;

#[derive(Clone)]
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

    pub fn turn_on(&mut self) {
        self.set_state(1);
    }

    pub fn turn_off(&mut self) {
        self.set_state(0);
    }

    pub fn is_on(&self) -> bool {
        self.state == 1
    }

    pub fn is_input_pin(&self, input_block: &InputBlock) -> bool {
        input_block.pins.contains(&self.id)
    }

    pub fn reset(&mut self) {
        self.state = 0;
    }
    
}