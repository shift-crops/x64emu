mod parse;
mod opcode;
mod exec;

use crate::emulator::access;

pub struct InstrArg<'a> {
    ac: &'a mut access::Access,
    idata: &'a parse::InstrData,
}

pub struct Instruction {
    idata: parse::InstrData,
    opcode: opcode::Opcode,
}

impl Instruction {
    pub fn new() -> Self {
        Instruction {
            idata: Default::default(),
            opcode: opcode::Opcode::new(),
        }
    }

    pub fn fetch_exec(&mut self, ac: &mut access::Access) -> () {
        self.idata.parse(ac, &self.opcode);

        let op = self.opcode.get();
        op.exec(&mut InstrArg{ac, idata: &self.idata});
    }
}