use crate::emulator::access;
use crate::emulator::instruction::parse;
use crate::hardware::processor::general::*;
use crate::emulator::instruction::opcode::*;

pub struct Opcode64 (pub super::OpcodeArr);
impl Opcode64 {
    pub fn new(op: super::OpcodeArr) -> Self {
        Opcode64 (op)
    }
}

impl super::OpcodeTrait for Opcode64 {
    fn init_opcode(&mut self) -> (){
    }

    fn exec(&self, ac: &mut access::Access, idata: &parse::InstrData) -> () {
        (self.0[idata.opcode as usize].func)(ac, &idata);
    }

    fn flag(&self, opcode: u16) -> OpFlags {
        self.0[opcode as usize].flag
    }
}