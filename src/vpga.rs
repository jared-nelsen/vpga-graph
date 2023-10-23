use std::collections::HashMap;

use uuid::Uuid;

use crate::vpga_spec::VPGASpec;
use crate::input_block::InputBlock;
use crate::output_block::OutputBlock;
use crate::switch_box::SwitchBox;
use crate::lut::LUT;
use crate::pin::Pin;
use crate::connection::Connection;

struct VPGA {
    spec: VPGASpec,
    input_block: InputBlock,
    output_block: OutputBlock,
    switch_box: SwitchBox,
    luts: Vec<LUT>,
    pin_map: HashMap<Uuid, Pin>,
    connection_map: HashMap<String, Connection>
}

impl VPGA {

    pub fn default(self, spec: VPGASpec) -> Self {
        let input_block = InputBlock::new(spec.input_block_width);
        let output_block = OutputBlock::new(spec.output_block_width);
        let switch_box = SwitchBox::new(spec.switch_box_pin_count);
        let luts = LUT::new_n(spec.lut_count, spec.lut_width);
        let pin_map = self.generate_pin_map();
        let connection_map = self.generate_connection_map();
        VPGA { spec, input_block, output_block, switch_box, luts, pin_map, connection_map }
    }

    fn generate_pin_map(&self) -> HashMap<Uuid, Pin> {
        HashMap::new()
    }

    fn generate_connection_map(&self) -> HashMap<String, Connection> {
        HashMap::new()
    }
        
}