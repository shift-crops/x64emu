mod parse;
mod opcode;
mod exec;

use std::error;
use super::access;

pub enum OpAdSize { BIT16, BIT32, BIT64 }

pub struct Instruction {
    opcode: opcode::Opcode,
}

impl Instruction {
    pub fn new() -> Self {
        Self {
            opcode: opcode::Opcode::new(),
        }
    }

    pub fn fetch_exec(&mut self, ac: &mut access::Access) -> Result<(), Box<dyn error::Error>> {
        let mut parse: parse::ParseInstr = Default::default();
        parse.parse_prefix(ac)?;

        let op = self.opcode.get(OpAdSize::BIT16);
        parse.parse_instruction(ac, op)?;

        let exec = &mut exec::Exec::new(ac, &parse.instr, OpAdSize::BIT16, parse.prefix.segment);
        op.exec(exec)?;

        Ok(())
    }
}