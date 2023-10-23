pub struct VPGASpec {
    pub input_block_width: i8,
    pub output_block_width: i8,
    pub lut_count: i8,
    pub lut_width: i8,
    pub switch_box_pin_count: i8,
}

impl VPGASpec {

    pub fn default(&self) -> Self {
        VPGASpec {
            input_block_width: 8, 
            output_block_width: 8, 
            lut_count: 4, 
            lut_width: 4,  
            switch_box_pin_count: 8,
        }
    }
    
}