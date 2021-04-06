use crate::emulator::instruction;
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

    fn exec(&self, arg: &mut instruction::InstrArg) -> () { (self.0[arg.idata.opcd as usize].func)(arg); } 
    fn flag(&self, opcode: u16) -> OpFlags { self.0[opcode as usize].flag }
}
