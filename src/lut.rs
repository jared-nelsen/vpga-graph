use uuid::Uuid;

pub struct LUT {
    pub width: i32,
    pub input_pins: Vec<Uuid>,
    pub output_pin: Uuid,
}