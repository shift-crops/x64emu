use crate::emulator::instruction::exec;
use crate::emulator::instruction::opcode::*;
use crate::emulator::instruction::exec::IpAccess;

pub struct Opcode64 (pub super::OpcodeArr);
impl Opcode64 {
    pub fn new(op: super::OpcodeArr) -> Self {
        Self (op)
    }
}

impl super::OpcodeTrait for Opcode64 {
    fn init_opcode(&mut self) -> () {
    }

    fn exec(&self, exec: &mut exec::Exec) -> Result<(), EmuException> {
        exec.update_ip(exec.idata.len as i64)?;
        (self.0[exec.idata.opcode as usize].func)(exec)?;
        Ok(())
    }
    fn flag(&self, opcode: u16) -> OpFlags { self.0[opcode as usize].flag }
}