use crate::emulator::access::register::*;
use crate::emulator::instruction::exec;
use crate::emulator::instruction::opcode::*;

pub struct Opcode32 (pub super::OpcodeArr);
impl Opcode32 {
    pub fn new(op: super::OpcodeArr) -> Self {
        Self (op)
    }
}

impl super::OpcodeTrait for Opcode32 {
    fn init_opcode(&mut self) -> () {
    }

    fn exec(&self, exec: &mut exec::Exec) -> Result<(), EmuException> {
        exec.ac.update_ip(exec.idata.len as i32)?;
        (self.0[exec.idata.opcode as usize].func)(exec)?;
        Ok(())
    }
    fn flag(&self, opcode: u16) -> OpFlags { self.0[opcode as usize].flag }
}
