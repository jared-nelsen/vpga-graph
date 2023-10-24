use uuid::Uuid;

pub struct OutputBlock {
    pub width: i32,
    pub pins: Vec<Uuid>,
}

impl OutputBlock {

    pub fn new(width: i32) -> Self {
        let mut pins = Vec::new();
        for _i in 0..width {
            pins.push(Uuid::new_v4());
        }
        OutputBlock { width, pins }        
    }

    pub fn get_pins(&self) -> Vec<Uuid> {
        self.pins.clone()
    }

}