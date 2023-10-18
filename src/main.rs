use ordered_hash_map::OrderedHashMap;
use uuid::Uuid;

fn main() {
    println!("Hello, world!");
}

struct Connection {
    source: Uuid,
    target: Uuid,
}

struct Pin {
    id: Uuid,
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

struct LUT {
    width: i32,
    input_pins: OrderedHashMap<Uuid, Pin>,
    output_pin: Pin,
}

struct VPGA {
    input_blocks: OrderedHashMap<Uuid, InputBlock>,
    output_blocks: OrderedHashMap<Uuid, OutputBlock>,
    luts: OrderedHashMap<Uuid, LUT>,
    contact_pool: OrderedHashMap<Uuid, Pin>,
}