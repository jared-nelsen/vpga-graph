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
    luts: LUT,
    switch_boxes: SwitchBox,
}

impl VPGA {

    pub fn default(self) -> Self {
        let default_spec = VPGASpec::default(&self.spec);
    
        VPGA {
            spec: default_spec,
            input_block: input_blocks, 
            output_blocks, 
            luts,
            switch_boxes,
        }
    }
        
}