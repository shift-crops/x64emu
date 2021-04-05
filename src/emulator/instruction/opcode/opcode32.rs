use crate::emulator::access;
use crate::emulator::instruction::parse;
use crate::hardware::processor::general::*;

pub struct Opcode32 (pub super::OpcodeArr);
impl Opcode32 {
    pub fn new(opa: super::OpcodeArr) -> Self {
        Opcode32 (opa)
    }
}

impl super::OpcodeTrait for Opcode32 {
    fn init_opcode(&mut self) -> (){
    }

    fn exec(&self, ac: &mut access::Access, idata: &parse::InstrData) -> () {
        self.0[idata.opcode as usize](ac, &idata);
    }
}
