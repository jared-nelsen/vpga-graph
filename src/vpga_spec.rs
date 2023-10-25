
#[derive(Debug,Clone, Copy)]
pub struct VPGASpec {
    pub input_block_width: i32,
    pub output_block_width: i32,
    pub lut_count: i32,
    pub lut_width: i32,
    pub lut_encoding_width: i32,
    pub switch_box_pin_count: i32,
}

impl VPGASpec {

    pub fn default() -> Self {
        let lut_width = 4;
        let lut_encoding_width = i32::pow(lut_width, 2);
        VPGASpec {
            input_block_width: 8, 
            output_block_width: 8, 
            lut_count: 50, 
            lut_width,  
            lut_encoding_width,
            switch_box_pin_count: 10,
        }
    }
    
}