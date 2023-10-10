use uuid::Uuid;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct Connection {
    source: Uuid,
    target: Uuid,
}

struct Pin {
    on: bool,
}

struct InputBlock {
    width: i32,
    pins: Vec<Pin>
}

struct OutputBlock {
    width: i32,
    pins: Vec<Pin>
}

struct VPGA {
    input_block: InputBlock,
    contact_pool: HashMap<Uuid, Pin>,
    output_block: OutputBlock,
}