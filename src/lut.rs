use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct LUT {
    pub width: i32,
    pub encoding_width: i32,
    pub encoding: Vec<i32>,
    pub input_pins: Vec<Uuid>,
    pub output_pin: Uuid,
}

impl LUT {

    pub fn new_n(count: i32, width: i32) -> Vec<Self> {
        let encoding_width = i32::pow(width, 2);
        let mut new_luts = Vec::new();
        for _i in 0..count {
            let mut encoding = Vec::new();
            for _j  in 0..encoding_width {
                encoding.push(0);
            }
            let mut input_pins = Vec::new();
            for _i in 0..width {
                input_pins.push(Uuid::new_v4());
            }
            let output_pin = Uuid::new_v4();
            let new_lut  = LUT { width, encoding_width, encoding, input_pins, output_pin };
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

    pub fn operate(&mut self) {
        // TODO
    }

}