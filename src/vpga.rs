use std::collections::{HashMap, HashSet, VecDeque};
use uuid::Uuid;

use crate::pin::Pin;
use crate::connection::Connection;
use crate::vpga_spec::VPGASpec;
use crate::input_block::InputBlock;
use crate::output_block::OutputBlock;
use crate::switch_box::SwitchBox;
use crate::lut::LUT;
use crate::data::Data;
use crate::encoding::Encoding;

pub struct VPGA {
    _spec: VPGASpec,
    input_block: InputBlock,
    output_block: OutputBlock,
    _switch_box: SwitchBox,
    luts: Vec<LUT>,
    pin_map: HashMap<Uuid, Pin>,
    connection_map: HashMap<String, Connection>,
    pub fitness: i32
}

impl VPGA {

    pub fn new(_spec: VPGASpec) -> Self {
        let input_block = InputBlock::new(_spec.input_block_width);
        let output_block = OutputBlock::new(_spec.output_block_width);
        let _switch_box = SwitchBox::new(_spec.switch_box_pin_count);
        let luts = LUT::new_n(_spec.lut_count, _spec.lut_width);
        let all_pins = Self::get_all_pins(&input_block, &output_block, &_switch_box, luts.clone());
        let pin_map = Self::generate_pin_map(&all_pins);
        let connection_map = Self::generate_connection_map(&all_pins);
        let fitness = 999999999;
        VPGA { _spec, input_block, output_block, _switch_box, luts, pin_map, connection_map, fitness }
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

    pub fn get_encoding_length(&self) -> i32 {
        let connection_count = self.connection_map.len() as i32;
        let lut_input_pin_count = self._spec.lut_count * self._spec.lut_width;
        lut_input_pin_count + connection_count
    }

    pub fn apply_encoding_to_vpga(&mut self, encoding: &Encoding) {

    }

    pub fn evaluate(&mut self, data: &Data) {
        self.reset();
        let mut error = 0;
        for sr_index in 0..data.sr_count {
            self.input_block.load_input_to_pins(&data.stimuli, &mut self.pin_map);
            self.operate();
            let output = self.output_block.get_output_from_pins(&mut self.pin_map);
            error += data.diff_output(sr_index, output);
        }
        self.fitness = error;
    }

    fn operate(&mut self) {
        for input_pin_id in self.input_block.get_pins() {
            self.bfs_update(input_pin_id);
        }
        for mut lut in self.luts.to_vec() {
            lut.operate();
            self.bfs_update(lut.output_pin);
        }
    }

    fn bfs_update(&mut self, initial_pin_id: Uuid) {
        let mut visited_pins: HashSet<Uuid> = HashSet::new();
        let mut queue: VecDeque<Uuid> = VecDeque::new();
    
        if !self.pin_map.contains_key(&initial_pin_id) {
            println!("Initial pin {} not found", initial_pin_id);
            return;
        }
    
        visited_pins.insert(initial_pin_id);
        queue.push_back(initial_pin_id);
    
        while let Some(current_pin_id) = queue.pop_front() {
            let current_pin = &self.pin_map[&current_pin_id];
            let neighbor_ids: Vec<Uuid> = current_pin.neighbors.clone();
        
            for neighbor_pin_id in &neighbor_ids {
                if !visited_pins.contains(neighbor_pin_id) {
                    visited_pins.insert(*neighbor_pin_id);
                    queue.push_back(*neighbor_pin_id);
        
                    let connection_id = Connection::generate_connection_id(&current_pin_id, neighbor_pin_id);
        
                    let should_turn_on = if let Some(connection) = self.connection_map.get(&connection_id) {
                        self.pin_map[&connection.source_pin].is_on() && connection.is_live()
                    } else {
                        println!("No connection found for id: {}", connection_id);
                        continue;
                    };
        
                    if should_turn_on {
                        if let Some(target_pin) = self.pin_map.get_mut(&neighbor_pin_id) {
                            if !target_pin.is_input_pin(&self.input_block) {
                                target_pin.turn_on();
                            }
                        }
                    }
                }
            }
        }
    }
        
}