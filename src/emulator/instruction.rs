mod parse;
mod opcode;

use crate::emulator::access;

pub struct Instruction {
    idata: parse::InstrData,
    opcode: opcode::Opcode,
}

impl Instruction {
    pub fn new() -> Self {
        Instruction {
            idata: parse::InstrData::new(),
            opcode: opcode::Opcode::new(),
        }
    }

    pub fn fetch_exec(&mut self, ac: &mut access::Access) -> () {
        self.idata.parse(ac, &self.opcode);

        let op = self.opcode.get();
        op.exec(ac, &self.idata);
    }
}