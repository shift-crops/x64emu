use crate::emulator::instruction::exec;
use crate::emulator::instruction::opcode::*;

pub struct Opcode64 (pub super::OpcodeArr);
impl Opcode64 {
    pub fn new(op: super::OpcodeArr) -> Self {
        Self (op)
    }
}

impl super::OpcodeTrait for Opcode64 {
    fn init_opcode(&mut self) -> (){
    }

    fn exec(&self, exec: &mut exec::Exec) -> () { (self.0[exec.idata.opcd as usize].func)(exec); exec.update_rip(exec.idata.oplen as i64); }
    fn flag(&self, opcode: u16) -> OpFlags { self.0[opcode as usize].flag }
}