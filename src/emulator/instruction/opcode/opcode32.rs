use crate::emulator::access;
use crate::emulator::instruction::parse;
use crate::hardware::processor::general::*;
use crate::emulator::instruction::opcode::*;

pub struct Opcode32 (pub super::OpcodeArr);
impl Opcode32 {
    pub fn new(op: super::OpcodeArr) -> Self {
        Opcode32 (op)
    }
}

impl super::OpcodeTrait for Opcode32 {
    fn init_opcode(&mut self) -> (){
    }

    fn exec(&self, ac: &mut access::Access, idata: &parse::InstrData) -> () {
        (self.0[idata.opcode as usize].func)(ac, &idata);
    }

    fn flag(&self, opcode: u16) -> OpFlags {
        self.0[opcode as usize].flag
    }
}
