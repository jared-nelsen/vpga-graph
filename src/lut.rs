use std::collections::HashMap;
use uuid::Uuid;

use crate::pin::Pin;

#[derive(Debug, Clone)]
pub struct LUT {
    pub width: i32,
    pub encoding_width: i32,
    pub encoding: Vec<i32>,
    pub input_pins: Vec<Uuid>,
    pub output_pin: Uuid,
}

impl LUT {

    pub fn new_n(count: i32, width: i32) -> Vec<Self> {
        let encoding_width = i32::pow(width, 2); // TODO Repeated computation
        let mut new_luts = Vec::new();
        for _i in 0..count {
            let mut encoding = Vec::new();
            for _j  in 0..encoding_width {
                encoding.push(0);
            }
            let mut input_pins = Vec::new();
            for _i in 0..width {
                input_pins.push(Uuid::new_v4());
            }
            let output_pin = Uuid::new_v4();
            let new_lut  = LUT { width, encoding_width, encoding, input_pins, output_pin };
            new_luts.push(new_lut);
        }
        new_luts
    }

    pub fn get_pins(self) -> Vec<Uuid> {
        let mut pins = Vec::new();
        pins.extend(self.input_pins);
        pins.push(self.output_pin);
        pins
    }

    fn mux(&mut self, i0: i32, i1: i32, sel: i32) -> i32 {
        if sel == 0 { i0 } else { i1 }
    }
    
    fn process_layer(&mut self, select: i32, encoding: Vec<i32>) -> Vec<i32> {
        let mux_count = encoding.len() / 2;
        let mut output = Vec::with_capacity(mux_count);
        for mux_index in 0..mux_count {
            let mux_val = self.mux(encoding[mux_index * 2], encoding[mux_index * 2 + 1], select);
            output.push(mux_val);
        }
        output
    }
    
    pub fn operate(&mut self, pin_map: &mut HashMap<Uuid, Pin>) {
        // Load the inputs
        let mut inputs = Vec::new();
        for pin_id in &self.input_pins {
            match pin_map.get(pin_id) {
                Some(pin) => inputs.push(pin.state),
                None => {
                    println!("Pin not found in LUT operation: {:?}", pin_id);
                    inputs.push(0);
                },
            }
        }
    
        // Process the layers
        let mut encoding = self.encoding.clone();
        for select_line in inputs {
            encoding = self.process_layer(select_line, encoding);
        }
        assert_eq!(encoding.len(), 1, "LUT operation did not reduce to a single output value");
    
        // Set the output pin
        if let Some(output_pin) = pin_map.get_mut(&self.output_pin) {
            output_pin.set_state(encoding[0]);
        } else {
            println!("Output pin {:?} not found in LUT operation", self.output_pin);
        }
    }

}