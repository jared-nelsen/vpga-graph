use uuid::Uuid;

pub struct InputBlock {
    pub width: i8,
    pub pins: Vec<Uuid>,
}

impl InputBlock {

    pub fn new(width: i8) -> Self {
        let mut pins = Vec::new();
        for i in 0..width {
            pins.push(Uuid::new_v4());
        }
        InputBlock { width, pins }
    }

    pub fn get_pins(&self) -> Vec<Uuid> {
        self.pins.clone()
    }

}