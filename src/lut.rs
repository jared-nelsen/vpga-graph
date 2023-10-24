use uuid::Uuid;

#[derive(Clone)]
pub struct LUT {
    pub width: i8,
    pub input_pins: Vec<Uuid>,
    pub output_pin: Uuid,
}

impl LUT {

    pub fn new_n(count: i8, width: i8) -> Vec<Self> {
        let mut new_luts = Vec::new();
        for _i in 0..count {
            let mut input_pins = Vec::new();
            for _i in 0..width {
                input_pins.push(Uuid::new_v4());
            }
            let output_pin = Uuid::new_v4();
            let new_lut  = LUT { width, input_pins, output_pin };
            new_luts.push(new_lut);
        }
        new_luts
    }

    pub fn get_pins(self) -> Vec<Uuid> {
        let mut pins = Vec::new();
        pins.extend(self.input_pins);
        pins.push(self.output_pin);
        pins
    }

}