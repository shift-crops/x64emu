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

    fn exec(&self, exec: &mut exec::Exec) -> Result<(), OpException> {
        (self.0[exec.idata.opcode as usize].func)(exec)?;
        exec.update_rip(exec.idata.len as i64)?;
        Ok(())
    }
    fn flag(&self, opcode: u16) -> OpFlags { self.0[opcode as usize].flag }
}
