use ordered_hash_map::OrderedHashMap;
use uuid::Uuid;

fn main() {
    println!("Hello, world!");
}

struct Connection {
    source_pin_id: Uuid,
    target_pin_id: Uuid,
}

struct Pin {
    id: Uuid,
    connections: Vec<Connection>,
    on: bool,
}

struct InputBlock {
    width: i32,
    pins: OrderedHashMap<Uuid, Pin> // Consider making this a vector
}

struct OutputBlock {
    width: i32,
    pins: OrderedHashMap<Uuid, Pin>
}

struct SwitchBox {
    count: i32,
    pins: OrderedHashMap<Uuid, Pin>,
}

struct LUT {
    width: i32,
    input_pins: OrderedHashMap<Uuid, Pin>,
    output_pin: Pin,
}

struct VPGASpec {
    input_block_count: i32,
    input_block_widths: Vec<i32>,
    output_block_count: i32,
    output_block_widths: Vec<i32>,
    lut_count: i32,
    lut_widths: Vec<i32>,
    switch_box_count: i32,
    switch_box_pin_count: i32,
}

impl VPGASpec {

    pub fn default(&self) -> Self {
        VPGASpec { 
            input_block_count: 1, 
            input_block_widths: vec![4], 
            output_block_count: 1, 
            output_block_widths: vec![4], 
            lut_count: 4, 
            lut_widths: vec![4], 
            switch_box_count: 1, 
            switch_box_pin_count: 8,
        }
    }
    
}

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
        
}