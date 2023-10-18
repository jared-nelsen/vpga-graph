use uuid::Uuid;
use ordered_hash_map::OrderedHashMap;

use crate::pin::Pin;

pub struct SwitchBox {
    pub count: i32,
    pub pins: OrderedHashMap<Uuid, Pin>,
}