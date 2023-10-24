use uuid::Uuid;

pub struct SwitchBox {
    pub pin_count: i32,
    pub pins: Vec<Uuid>,
}

impl SwitchBox {

    pub fn new(pin_count: i32) -> Self {
        let mut pins = Vec::new();
        for _i in 0..pin_count {
            pins.push(Uuid::new_v4());
        }
        SwitchBox { pin_count, pins }        
    }

    pub fn get_pins(&self) -> Vec<Uuid> {
        self.pins.clone()
    }

}