use uuid::Uuid;
use ordered_hash_map::OrderedHashMap;

use crate::pin::Pin;
use crate::vpga_spec::VPGASpec;
use crate::input_block::InputBlock;
use crate::output_block::OutputBlock;
use crate::lut::LUT;
use crate::switch_box::SwitchBox;
use crate::connection::Connection;

struct VPGA {
    spec: VPGASpec,
    input_blocks: OrderedHashMap<Uuid, InputBlock>,
    output_blocks: OrderedHashMap<Uuid, OutputBlock>,
    luts: OrderedHashMap<Uuid, LUT>,
    switch_boxes: OrderedHashMap<Uuid, SwitchBox>,
}

impl VPGA {

    pub fn default(self) -> Self {
        let default_spec = VPGASpec::default(&self.spec);
    
        // Initialize input blocks
        let mut input_blocks = OrderedHashMap::new();
        for &width in default_spec.input_block_widths.iter() {
            let mut pins = OrderedHashMap::new();
            for _ in 0..width {
                let pin = Pin {
                    id: Uuid::new_v4(),
                    connections: Vec::new(),
                    on: false,
                };
                pins.insert(pin.id, pin);
            }
            let block = InputBlock {
                width,
                pins,
            };
            input_blocks.insert(Uuid::new_v4(), block);
        }
    
        // Initialize output blocks
        let mut output_blocks = OrderedHashMap::new();
        for &width in default_spec.output_block_widths.iter() {
            let mut pins = OrderedHashMap::new();
            for _ in 0..width {
                let pin = Pin {
                    id: Uuid::new_v4(),
                    connections: Vec::new(),
                    on: false,
                };
                pins.insert(pin.id, pin);
            }
            let block = OutputBlock {
                width,
                pins,
            };
            output_blocks.insert(Uuid::new_v4(), block);
        }
    
        // Initialize LUTs
        let mut luts = OrderedHashMap::new();
        for &width in default_spec.lut_widths.iter() {
            let mut input_pins = OrderedHashMap::new();
            for _ in 0..width {
                let pin = Pin {
                    id: Uuid::new_v4(),
                    connections: Vec::new(),
                    on: false,
                };
                input_pins.insert(pin.id, pin);
            }
            let output_pin = Pin {
                id: Uuid::new_v4(),
                connections: Vec::new(),
                on: false,
            };
            let lut = LUT {
                width,
                input_pins,
                output_pin,
            };
            luts.insert(Uuid::new_v4(), lut);
        }
    
        // Initialize switch boxes
        let mut switch_boxes = OrderedHashMap::new();
        for _ in 0..default_spec.switch_box_count {
            let mut pins = OrderedHashMap::new();
            for _ in 0..default_spec.switch_box_pin_count {
                let pin = Pin {
                    id: Uuid::new_v4(),
                    connections: Vec::new(),
                    on: false,
                };
                pins.insert(pin.id, pin);
            }
            let box_ = SwitchBox {
                count: default_spec.switch_box_pin_count,
                pins,
            };
            switch_boxes.insert(Uuid::new_v4(), box_);
        }
    
        VPGA {
            spec: default_spec,
            input_blocks, 
            output_blocks, 
            luts,
            switch_boxes,
        }
    }

    pub fn connect_vpga(&mut self) {
        
        fn connect_pins(pins: &mut OrderedHashMap<Uuid, Pin>, other_pins: &OrderedHashMap<Uuid, &Pin>) {
            for (_, pin) in pins.iter_mut() {
                for (other_pin_id, _) in other_pins.iter() {
                    if &pin.id != other_pin_id {
                        let connection = Connection {
                            source_pin_id: pin.id,
                            target_pin_id: *other_pin_id,
                        };
                        pin.connections.push(connection);
                    }
                }
            }
        }

        fn connect_single_pin(pin: &mut Pin, other_pins: &OrderedHashMap<Uuid, &Pin>) {
            for (other_pin_id, _) in other_pins.iter() {
                if &pin.id != other_pin_id {
                    let connection = Connection {
                        source_pin_id: pin.id,
                        target_pin_id: *other_pin_id,
                    };
                    pin.connections.push(connection);
                }
            }
        }

        let all_pins = OrderedHashMap::new();
        self.flatten_pins(all_pins);

        // Connect input block pins
        let input_block_ids: Vec<Uuid> = self.input_blocks.keys().cloned().collect();
        for block_id in &input_block_ids {
            let mut input_block = self.input_blocks.remove(block_id).unwrap();
            connect_pins(&mut input_block.pins, &all_pins);
            self.input_blocks.insert(*block_id, input_block);
        }

        // Connect output block pins
        let output_block_ids: Vec<Uuid> = self.output_blocks.keys().cloned().collect();
        for block_id in &output_block_ids {
            let mut output_block = self.output_blocks.remove(block_id).unwrap();
            connect_pins(&mut output_block.pins, &all_pins);
            self.output_blocks.insert(*block_id, output_block);
        }

        // Connect LUT pins
        let lut_ids: Vec<Uuid> = self.luts.keys().cloned().collect();
        for lut_id in &lut_ids {
            let mut lut = self.luts.remove(lut_id).unwrap();
            connect_pins(&mut lut.input_pins, &all_pins);
            connect_single_pin(&mut lut.output_pin, &all_pins);
            self.luts.insert(*lut_id, lut);
        }

        // Connect switch box pins
        let switch_box_ids: Vec<Uuid> = self.switch_boxes.keys().cloned().collect();
        for box_id in &switch_box_ids {
            let mut switch_box = self.switch_boxes.remove(box_id).unwrap();
            connect_pins(&mut switch_box.pins, &all_pins);
            self.switch_boxes.insert(*box_id, switch_box);
        }
    }

    fn flatten_pins(&self, mut all_pins: OrderedHashMap<Uuid, &Pin>) {

        for (_, input_block) in self.input_blocks.iter() {
            for (id, pin) in &input_block.pins {
                all_pins.insert(*id, pin);
            }
        }
        
        for (_, output_block) in self.output_blocks.iter() {
            for (id, pin) in &output_block.pins {
                all_pins.insert(*id, pin);
            }
        }

        for (_, lut) in self.luts.iter() {
            for (id, pin) in &lut.input_pins {
                all_pins.insert(*id, pin);
            }
            all_pins.insert(lut.output_pin.id, &lut.output_pin);
        }

        for (_, switch_box) in self.switch_boxes.iter() {
            for (id, pin) in &switch_box.pins {
                all_pins.insert(*id, pin);
            }
        }
    }
        
}