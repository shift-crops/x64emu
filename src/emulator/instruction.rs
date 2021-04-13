mod parse;
mod opcode;
mod exec;

use super::access;
use crate::emulator::EmuException;

#[derive(Clone, Copy)]
pub enum OpAdSize { BIT16, BIT32, BIT64 }
impl Default for OpAdSize {
    fn default() -> Self {
        OpAdSize::BIT16
    }
}

pub struct Instruction(opcode::Opcode);

impl Instruction {
    pub fn new() -> Self {
        Self (opcode::Opcode::new())
    }

    pub fn fetch_exec(&mut self, ac: &mut access::Access) -> Result<(), EmuException> {
        let mut parse: parse::ParseInstr = Default::default();

        parse.parse_prefix(ac)?;
        let (opsize, adsize) = Instruction::opad_size(ac.mode, &parse.prefix);

        let op = self.0.get(opsize);
        parse.parse_opcode(ac)?;
        parse.parse_oprand(ac, op.flag(parse.instr.opcode), adsize)?;

        op.exec(&mut exec::Exec::new(ac, &parse.instr, parse.prefix.segment, parse.prefix.repeat))?;

        Ok(())
    }

    pub fn opad_size(mode: access::CpuMode, pdata: &parse::PrefixData) -> (OpAdSize, OpAdSize) {
        let (mut ops, mut ads) = match mode {
            access::CpuMode::Real => {
                (OpAdSize::BIT16, OpAdSize::BIT16)
            },
            access::CpuMode::Protected => {
                (OpAdSize::BIT32, OpAdSize::BIT32)
            },
            access::CpuMode::Long => {
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