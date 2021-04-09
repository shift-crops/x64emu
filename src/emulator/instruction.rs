mod parse;
mod opcode;
mod exec;

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

    pub fn fetch_exec(&mut self, ac: &mut access::Access) -> () {
        self.idata.parse(ac, &self.opcode);

        let mut exe = exec::Exec::new(ac, &self.idata);
        let op = self.opcode.get();
        op.exec(&mut exe);
    }
}