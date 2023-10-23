use uuid::Uuid;

use crate::pin::Pin;
use crate::vpga_spec::VPGASpec;
use crate::input_block::InputBlock;
use crate::output_block::OutputBlock;
use crate::lut::LUT;
use crate::switch_box::SwitchBox;
use crate::connection::Connection;

struct VPGA {
    spec: VPGASpec,
    input_block: InputBlock,
    output_block: OutputBlock,
    switch_box: SwitchBox,
    luts: LUT,
}

impl VPGA {

    pub fn default(self) -> Self {
        let default_spec = VPGASpec::default(&self.spec);
    
        VPGA {
            spec: default_spec,
            input_block: InputBlock::new(&self, default_spec.input_block_width), 
            output_block: OutputBlock::new(&self, default_spec.output_block_width),
            switch_box: ,
            luts: ,
        }
    }
        
}