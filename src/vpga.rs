use std::collections::HashMap;

use uuid::Uuid;

use crate::pin::Pin;
use crate::connection::Connection;
use crate::vpga_spec::VPGASpec;
use crate::input_block::InputBlock;
use crate::output_block::OutputBlock;
use crate::switch_box::SwitchBox;
use crate::lut::LUT;
use crate::data::Data;

pub struct VPGA {
    spec: VPGASpec,
    input_block: InputBlock,
    output_block: OutputBlock,
    switch_box: SwitchBox,
    luts: Vec<LUT>,
    pin_map: HashMap<Uuid, Pin>,
    connection_map: HashMap<String, Connection>
}

impl VPGA {

    pub fn new(spec: VPGASpec) -> Self {
        let input_block = InputBlock::new(spec.input_block_width);
        let output_block = OutputBlock::new(spec.output_block_width);
        let switch_box = SwitchBox::new(spec.switch_box_pin_count);
        let luts = LUT::new_n(spec.lut_count, spec.lut_width);
        let all_pins = Self::get_all_pins(&input_block, &output_block, &switch_box, luts.clone());
        let pin_map = Self::generate_pin_map(&all_pins);
        let connection_map = Self::generate_connection_map(&all_pins);
        VPGA { spec, input_block, output_block, switch_box, luts, pin_map, connection_map }
    }

    fn reset(&mut self) {
        for pin in self.pin_map.values_mut() {
            pin.reset();
        }   
    }

    fn get_all_pins(input_block: &InputBlock, output_block: &OutputBlock, switch_box: &SwitchBox, luts: Vec<LUT>) -> Vec<Uuid> {
        let mut all_pins = Vec::new();
        all_pins.extend(input_block.get_pins());
        all_pins.extend(output_block.get_pins());
        all_pins.extend(switch_box.get_pins());
        for lut in luts {
            all_pins.extend(lut.get_pins())
        }
        all_pins
    }

    fn generate_pin_map(all_pins: &Vec<Uuid>) -> HashMap<Uuid, Pin> {
        let mut pin_map = HashMap::new();
        for pin_id in all_pins {
            pin_map.insert(pin_id.clone(), Pin::new(pin_id.clone()));
        }
        pin_map
    }

    fn generate_connection_map(all_pins: &Vec<Uuid>) -> HashMap<String, Connection> {
        let mut connection_map = HashMap::new();
        for source_pin in all_pins {
            for target_pin in all_pins {
                if source_pin != target_pin {
                    let new_connection_id = Connection::generate_connection_id(source_pin, target_pin);
                    let new_connection = Connection::new(new_connection_id.clone(), *source_pin, *target_pin);
                    connection_map.insert(new_connection_id, new_connection);
                }
            }
        }
        connection_map
    }

    fn operate(&self) {

    }

    pub fn evaluate(&mut self, data: Data) -> i32 {
        self.reset();
        let mut error = 0;
        for i in 0..data.sr_count {
            self.input_block.load_input_to_pins(&data.stimuli, &mut self.pin_map);
            self.operate();
            let output = self.output_block.get_output_from_pins(&mut self.pin_map);
            error += data.diff_output(i, output);
        }
        error
    }
        
}