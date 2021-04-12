mod parse;
mod opcode;
mod exec;

use super::access;
use crate::emulator::EmuException;

#[derive(Clone, Copy)]
pub enum OpAdSize { BIT16, BIT32, BIT64 }

pub struct Instruction(opcode::Opcode);

impl Instruction {
    pub fn new() -> Self {
        Self (opcode::Opcode::new())
    }

    pub fn fetch_exec(&mut self, ac: &mut access::Access, mode: super::CpuMode) -> Result<(), EmuException> {
        let mut parse: parse::ParseInstr = Default::default();

        parse.parse_prefix(ac, mode)?;
        let (opsize, adsize) = Instruction::opad_size(mode, &parse.prefix);

        let op = self.0.get(opsize);
        parse.parse_instruction(ac, op, adsize)?;

        op.exec(&mut exec::Exec::new(ac, &parse.instr, adsize, parse.prefix.segment))?;

        Ok(())
    }

    pub fn opad_size(mode: super::CpuMode, pdata: &parse::PrefixData) -> (OpAdSize, OpAdSize) {
        let (mut ops, mut ads) = match mode {
            super::CpuMode::Real => {
                (OpAdSize::BIT16, OpAdSize::BIT16)
            },
            super::CpuMode::Protected => {
                (OpAdSize::BIT32, OpAdSize::BIT32)
            },
            super::CpuMode::Long => {
                (if pdata.rex.w == 1 { OpAdSize::BIT64 } else { OpAdSize::BIT32 }, OpAdSize::BIT64)
            },
        };

        if pdata.size.contains(parse::OverrideSize::OP) {
            ops = match ops {
                OpAdSize::BIT16 => OpAdSize::BIT32,
                OpAdSize::BIT32 => OpAdSize::BIT16,
                OpAdSize::BIT64 => OpAdSize::BIT64,
            };
        }
        if pdata.size.contains(parse::OverrideSize::AD) {
            ads = match ads {
                OpAdSize::BIT16 => OpAdSize::BIT32,
                OpAdSize::BIT32 => OpAdSize::BIT16,
                OpAdSize::BIT64 => OpAdSize::BIT32,
            };
        }

        (ops, ads)
    }
}