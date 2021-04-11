mod parse;
mod opcode;
mod exec;

use std::error;
use super::access;

pub struct Instruction {
    idata: parse::InstrData,
    opcode: opcode::Opcode,
}

impl Instruction {
    pub fn new() -> Self {
        Self {
            idata: Default::default(),
            opcode: opcode::Opcode::new(),
        }
    }

    pub fn fetch_exec(&mut self, ac: &mut access::Access) -> Result<(), Box<dyn error::Error>> {
        self.idata.parse1(ac)?;

        let op = self.opcode.get();
        let flag = op.flag(self.idata.opcd);

        self.idata.parse2(ac, &flag)?;
        let exec = &mut exec::Exec::new(ac, &self.idata);

        op.exec(exec)?;

        Ok(())
    }
}