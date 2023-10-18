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
    pins: OrderedHashMap<Uuid, Pin>
}

struct OutputBlock {
    width: i32,
    pins: OrderedHashMap<Uuid, Pin>
}

struct PinField {
    count: i32,
    pins: OrderedHashMap<Uuid, Pin>,
}

struct LUT {
    width: i32,
    input_pins: OrderedHashMap<Uuid, Pin>,
    output_pin: Pin,
}

struct VPGA {
    input_blocks: OrderedHashMap<Uuid, InputBlock>,
    output_blocks: OrderedHashMap<Uuid, OutputBlock>,
    luts: OrderedHashMap<Uuid, LUT>,
    pin_fields: OrderedHashMap<Uuid, PinField>,
    graph: Vec<Pin>,
}

// Algorithm:
// 1. Load input blocks with input in order
// 2. Iterate over each Input Block and for each pin in it start a breadth first update of neighbor pins
// 3. When breadth first update is done then iterate over the LUTs, take their updated input from the pins associated with them,
//    and perform a breadth first update from their output pin (ingore updating input pins)
// 4. When all of the breadth first updates are done from all LUTs then collect the output vector from the output pins
// 5. And use it to evaluate