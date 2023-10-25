use std::collections::HashMap;

use uuid::Uuid;

use crate::pin::Pin;

#[derive(Debug)]
pub struct OutputBlock {
    pub width: i32,
    pub pins: Vec<Uuid>,
}

impl OutputBlock {

    pub fn new(width: i32) -> Self {
        let mut pins = Vec::new();
        for _i in 0..width {
            pins.push(Uuid::new_v4());
        }
        OutputBlock { width, pins }        
    }

    pub fn get_pins(&self) -> Vec<Uuid> {
        self.pins.clone()
    }

    pub fn get_output_from_pins(&self, pin_map: &mut HashMap<Uuid, Pin>) -> Vec<i32> {
        let mut output = Vec::new();
        for pin_id in self.pins.clone() {
            if let Some(pin) = pin_map.get(&pin_id) {
                output.push(pin.state);
            }
        }
        output
    }

}