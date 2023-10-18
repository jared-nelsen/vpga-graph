use ordered_hash_map::OrderedHashMap;
use uuid::Uuid;

use crate::pin::Pin;

pub struct OutputBlock {
    pub width: i32,
    pub pins: OrderedHashMap<Uuid, Pin>
}