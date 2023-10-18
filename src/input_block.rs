use ordered_hash_map::OrderedHashMap;
use uuid::Uuid;

use crate::pin::Pin;

pub struct InputBlock {
    pub width: i32,
    pub pins: OrderedHashMap<Uuid, Pin> // Consider making this a vector
}