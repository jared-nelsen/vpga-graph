pub struct VPGASpec {
    pub input_block_count: i32,
    pub input_block_widths: Vec<i32>,
    pub output_block_count: i32,
    pub output_block_widths: Vec<i32>,
    pub lut_count: i32,
    pub lut_widths: Vec<i32>,
    pub switch_box_count: i32,
    pub switch_box_pin_count: i32,
}

impl VPGASpec {

    pub fn default(&self) -> Self {
        VPGASpec { 
            input_block_count: 1, 
            input_block_widths: vec![4], 
            output_block_count: 1, 
            output_block_widths: vec![4], 
            lut_count: 4, 
            lut_widths: vec![4], 
            switch_box_count: 1, 
            switch_box_pin_count: 8,
        }
    }
    
}