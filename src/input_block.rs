use std::collections::HashMap;
use uuid::Uuid;

use crate::pin::Pin;

#[derive(Debug)]
pub struct InputBlock {
    pub width: i32,
    pub pins: Vec<Uuid>,
}

impl InputBlock {

    pub fn new(width: i32) -> Self {
        let mut pins = Vec::new();
        for _i in 0..width {
            pins.push(Uuid::new_v4());
        }
        InputBlock { width, pins }
    }

    pub fn get_pins(&self) -> Vec<Uuid> {
        self.pins.clone()
    }

    pub fn load_input_to_pins(&self, input: &Vec<i32>, pin_map: &mut HashMap<Uuid, Pin>) {
        let mut input_index = 0;
        for pin_id in &self.pins {
            if let Some(pin) = pin_map.get_mut(pin_id) {
                pin.set_state(input[input_index]);
            }
            input_index += 1;
        }
    }

}