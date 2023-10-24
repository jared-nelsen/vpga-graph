
use crate::vpga_spec::VPGASpec;
use crate::vpga::{VPGA, self};
use crate::data::Data;

pub struct Simulation {
    sr_count: i32,
    vpga_spec: VPGASpec,
    vpga: VPGA,
    data: Data,
}

impl Simulation {

    pub fn new() -> Self {
        let sr_count = 10;
        let vpga_spec = VPGASpec::default();
        let vpga = VPGA::new(vpga_spec);
        let data = Data::random(
            sr_count, 
            vpga_spec.input_block_width, 
            vpga_spec.output_block_width,
        );
        Simulation { sr_count, vpga_spec, vpga, data }
    }
    
}