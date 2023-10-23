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
    luts: Vec<LUT>,
}

impl VPGA {

    pub fn default(self, default_spec: VPGASpec) -> Self {
        VPGA {
            spec: default_spec,
            input_block: InputBlock::new(default_spec.input_block_width), 
            output_block: OutputBlock::new(default_spec.output_block_width),
            switch_box: SwitchBox::new(default_spec.switch_box_pin_count),
            luts: LUT::new_n(default_spec.lut_count, default_spec.lut_width),
        }
    }
        
}