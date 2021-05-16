mod parse;
mod opcode;
mod exec;

use super::access;
use crate::emulator::{EmuException, CPUException};

pub(super) struct Instruction(opcode::Opcode);

impl Instruction {
    pub fn new() -> Self {
        Self (opcode::Opcode::new())
    }

    pub fn fetch_exec(&mut self, ac: &mut access::Access) -> Result<(), EmuException> {
        let mut parse: parse::ParseInstr = Default::default();

        parse.parse_prefix(ac)?;
        let size = Instruction::opad_size(&ac.oasz, &parse.prefix);

        let op = self.0.get(size.op);
        parse.parse_opcode(ac)?;
        parse.parse_oprand(ac, op.flag(parse.instr.opcode), size.ad)?;

        ac.update_ip(parse.instr.len as i64)?;
        op.exec(&mut exec::Exec::new(ac, &parse))?;
        if ac.core.rflags.is_trap() { Err(EmuException::CPUException(CPUException::DB)) } else { Ok(()) }
    }

    pub fn opad_size(size: &access::OpAdSize, pdata: &parse::PrefixData) -> access::OpAdSize {
        let (mut op, mut ad) = (size.op, size.ad);

        if let Some(parse::Rex { w: 1, .. }) = pdata.rex {
            op = access::AcsSize::BIT64;
        }
        if pdata.size.contains(parse::OverrideSize::OP) {
            op = match op {
                access::AcsSize::BIT16 => access::AcsSize::BIT32,
                access::AcsSize::BIT32 => access::AcsSize::BIT16,
                access::AcsSize::BIT64 => access::AcsSize::BIT64,
            };
        }
        if pdata.size.contains(parse::OverrideSize::AD) {
            ad = match ad {
                access::AcsSize::BIT16 => access::AcsSize::BIT32,
                access::AcsSize::BIT32 => access::AcsSize::BIT16,
                access::AcsSize::BIT64 => access::AcsSize::BIT32,
            };
        }

        access::OpAdSize { op, ad }
    }
}