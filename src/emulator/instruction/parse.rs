use std::error;
use packed_struct::prelude::*;
use super::opcode;
use crate::emulator;
use crate::emulator::access;
use crate::hardware::processor::segment;

#[derive(Default)]
pub struct ParseInstr {
    pub prefix: PrefixData,
    pub instr: InstrData,
}

#[derive(Default)]
pub struct PrefixData {
    pub segment: Option<segment::SgReg>,
    pub repeat: Option<Rep>,
    pub size: OverrideSize,
    pub rex: Rex,
}

pub enum Rep { REPZ, REPNZ }

bitflags! {
    pub struct OverrideSize: u8 {
        const NONE = 0b00000000;
        const OP   = 0b00000001;
        const AD   = 0b00000010;
    }
}
impl Default for OverrideSize {
    fn default() -> Self { OverrideSize::NONE }
}

#[derive(Debug, Default, Clone, Copy, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct Rex {
    #[packed_field(bits="0")] pub b:  u8,
    #[packed_field(bits="1")] pub x:  u8,
    #[packed_field(bits="2")] pub r:  u8,
    #[packed_field(bits="3")] pub w:  u8,
}

#[derive(Default)]
pub struct InstrData {
    pub len: u64,

    pub opcode: u16,
    pub modrm: ModRM,
    pub sib: Sib,
    pub disp: u32,
    pub imm: u64,
    pub ptr16: u16,
    pub moffs: u64,
}

#[derive(Debug, Default, Clone, Copy, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct ModRM {
    #[packed_field(bits="0:2")] pub rm:  u8,
    #[packed_field(bits="3:5")] pub reg:  u8,
    #[packed_field(bits="6:7")] pub mod_:  u8,
}

#[derive(Debug, Default, Clone, Copy, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct Sib { 
    #[packed_field(bits="0:2")] pub base:  u8,
    #[packed_field(bits="3:5")] pub index:  u8,
    #[packed_field(bits="6:7")] pub scale:  u8,
}

impl ParseInstr {
    pub fn parse_prefix(&mut self, ac: &mut access::Access, mode: emulator::CpuMode) -> Result<(), Box<dyn error::Error>> {
        self.get_legacy_prefix(ac)?;

        if let emulator::CpuMode::Long = mode {
            self.get_rex_prefix(ac)?;
        }
        Ok(())
    }

    pub fn parse_instruction(&mut self, ac: &mut access::Access, op: &dyn opcode::OpcodeTrait, adsize: super::OpAdSize) -> Result<(), Box<dyn error::Error>> {
        self.get_opcode(ac)?;

        let flag = op.flag(self.instr.opcode);

        if flag.contains(opcode::OpFlags::MODRM) {
            self.get_modrm(ac)?;
            self.get_sib_disp(ac, adsize)?;
        }

        if flag.contains(opcode::OpFlags::IMM32) {
            self.instr.imm = ac.get_code32(self.instr.len)? as u64;
            self.instr.len += 4;
        } else if flag.contains(opcode::OpFlags::IMM16) {
            self.instr.imm = ac.get_code16(self.instr.len)? as u64;
            self.instr.len += 2;
        } else if flag.contains(opcode::OpFlags::IMM8) {
            self.instr.imm = ac.get_code8(self.instr.len)? as u64;
            self.instr.len += 1;
        }

        if flag.contains(opcode::OpFlags::PTR16) {
            self.instr.ptr16 = ac.get_code16(self.instr.len)? as u16;
            self.instr.len += 2;
        }

        if flag.contains(opcode::OpFlags::MOFFSX) {
            self.get_moffs(ac, adsize)?;
        }

        Ok(())
    }
}

impl ParseInstr {
    fn get_legacy_prefix(&mut self, ac: &mut access::Access) -> Result<(), Box<dyn error::Error>> {
        let prefix = &mut self.prefix;
        for _ in 0..4 {
            match ac.get_code8(self.instr.len)? {
                0x26 => { prefix.segment = Some(segment::SgReg::ES); },
                0x2e => { prefix.segment = Some(segment::SgReg::CS); },
                0x36 => { prefix.segment = Some(segment::SgReg::SS); },
                0x3e => { prefix.segment = Some(segment::SgReg::DS); },
                0x64 => { prefix.segment = Some(segment::SgReg::FS); },
                0x65 => { prefix.segment = Some(segment::SgReg::GS); },
                0x66 => { prefix.size |= OverrideSize::OP; },
                0x67 => { prefix.size |= OverrideSize::AD; },
                0xf2 => { prefix.repeat = Some(Rep::REPNZ) },
                0xf3 => { prefix.repeat = Some(Rep::REPZ) },
                _ => break,
            }
            self.instr.len += 1;
        }
        Ok(())
    }

    fn get_rex_prefix(&mut self, ac: &mut access::Access) -> Result<(), Box<dyn error::Error>> {
        let code = ac.get_code8(self.instr.len)?;
        if (code >> 4) != 4 { return Ok(()); }

        self.prefix.rex = Rex::unpack(&code.to_be_bytes()).unwrap();
        self.instr.len += 1;
        debug!("{:} ", self.prefix.rex);
        Ok(())
    }

    fn get_opcode(&mut self, ac: &mut access::Access) -> Result<(), Box<dyn error::Error>> {
        let opcode = &mut self.instr.opcode;
        *opcode = ac.get_code8(self.instr.len)? as u16;
        self.instr.len += 1;

        if *opcode == 0x0f {
            *opcode = (1<<8) + ac.get_code8(self.instr.len)? as u16;
        }
        debug!("opcode: {:02x} ", *opcode);
        Ok(())
    }

    fn get_modrm(&mut self, ac: &mut access::Access) -> Result<(), Box<dyn error::Error>> {
        let code = ac.get_code8(self.instr.len)?;
        self.instr.modrm = ModRM::unpack(&code.to_be_bytes()).unwrap();
        debug!("{:?} ", self.instr.modrm);
        self.instr.len += 1;
        Ok(())
    }

    fn get_sib_disp(&mut self, ac: &mut access::Access, adsize: super::OpAdSize) -> Result<(), Box<dyn error::Error>> {
        let (mod_, rm) = (self.instr.modrm.mod_, self.instr.modrm.rm);
        match adsize {
            super::OpAdSize::BIT16 => {
                if mod_ == 2 || (mod_ == 0 && rm == 6) {
                    self.instr.disp = ac.get_code16(self.instr.len)? as u32;
                    self.instr.len += 2;
                } else if mod_ == 1 {
                    self.instr.disp = ac.get_code8(self.instr.len)? as u32;
                    self.instr.len += 1;
                }
                debug!("disp: {:02x} ", self.instr.disp);
            },
            super::OpAdSize::BIT32 => {
                if mod_ != 3 && rm == 4 {
                    self.instr.sib = Sib::unpack(&ac.get_code8(self.instr.len)?.to_be_bytes()).unwrap();
                    self.instr.len += 1;
                    debug!("{:?} ", self.instr.sib);
                }

                if mod_ == 2 || (mod_ == 0 && rm == 5) || (mod_ == 0 && self.instr.sib.base == 5) {
                    self.instr.disp = ac.get_code32(self.instr.len)? as u32;
                    self.instr.len += 4;
                } else if mod_ == 1 {
                    self.instr.disp = ac.get_code8(self.instr.len)? as u32;
                    self.instr.len += 1;
                }
                debug!("disp: {:02x} ", self.instr.disp);
            },
            super::OpAdSize::BIT64 => {},
        }
        Ok(())
    }

    fn get_moffs(&mut self, ac: &mut access::Access, adsize: super::OpAdSize) -> Result<(), Box<dyn error::Error>> {
        let moffs = &mut self.instr.moffs;
        match adsize {
            super::OpAdSize::BIT16 => {
                *moffs = ac.get_code16(self.instr.len)? as u64;
                self.instr.len += 2;
            },
            super::OpAdSize::BIT32 => {
                *moffs = ac.get_code32(self.instr.len)? as u64;
                self.instr.len += 4;
            },
            super::OpAdSize::BIT64 => {},
        }
        Ok(())
    }
}