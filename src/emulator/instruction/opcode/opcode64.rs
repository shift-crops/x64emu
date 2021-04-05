use crate::emulator::access;
use crate::emulator::instruction::parse;
use crate::hardware::processor::general::*;

pub struct Opcode64 (pub super::OpcodeArr);
impl Opcode64 {
    pub fn new(opa: super::OpcodeArr) -> Self {
        Opcode64 (opa)
    }
}

impl super::OpcodeTrait for Opcode64 {
    fn init_opcode(&mut self) -> (){
    }

    fn exec(&self, ac: &mut access::Access, idata: &parse::InstrData) -> () {
        self.0[idata.opcode as usize](ac, &idata);
    }
}