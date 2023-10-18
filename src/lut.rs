use uuid::Uuid;
use ordered_hash_map::OrderedHashMap;

use crate::pin::Pin;

pub struct LUT {
    pub width: i32,
    pub input_pins: OrderedHashMap<Uuid, Pin>,
    pub output_pin: Pin,
}