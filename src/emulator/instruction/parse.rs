use thiserror::Error;
use packed_struct::prelude::*;
use super::opcode;
use crate::emulator::access;
use crate::hardware::processor::segment;

#[derive(Debug, Error)]
pub enum InstrError {
    #[error("Pack Error")]
    PackError(packed_struct::PackingError),
}

impl From<packed_struct::PackingError> for InstrError {
    fn from(err: packed_struct::PackingError) -> InstrError {
        InstrError::PackError(err)
    }
}

#[derive(Default)]
pub struct InstrData {
    pub pre_segment: Option<segment::SgReg>,
    pub pre_repeat: Option<Rep>,
    pub pre_size: OverrideSize,

    pub rex: Rex,
    pub opcd: u16,
    pub modrm: ModRM,
    pub sib: Sib,
    pub disp: u32,
    pub imm: u64,
    pub ptr16: u16,
    pub moffs: u64,

    pub oplen: u64,
}

pub struct PrefixData {

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
    #[packed_field(bits="0")] b:  u8,
    #[packed_field(bits="1")] x:  u8,
    #[packed_field(bits="2")] r:  u8,
    #[packed_field(bits="3")] w:  u8,
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

impl InstrData {
    pub fn parse1(&mut self, ac: &mut access::Access) -> Result<(), InstrError> {
        self.oplen = 0;

        self.get_legacy_prefix(ac)?;
        // self.get_rex_prefix(ac)?; 64 bit mode

        self.get_opcode(ac)?;
        Ok(())
    }

    pub fn parse2(&mut self, ac: &mut access::Access, flag: &opcode::OpFlags) -> Result<(), InstrError> {
        if flag.contains(opcode::OpFlags::MODRM) {
            self.get_modrm(ac)?;
        }

        if flag.contains(opcode::OpFlags::IMM32) {
            self.imm = ac.get_code32(self.oplen) as u64;
            self.oplen += 4;
        } else if flag.contains(opcode::OpFlags::IMM16) {
            self.imm = ac.get_code16(self.oplen) as u64;
            self.oplen += 2;
        } else if flag.contains(opcode::OpFlags::IMM8) {
            self.imm = ac.get_code8(self.oplen) as u64;
            self.oplen += 1;
        }

        if flag.contains(opcode::OpFlags::PTR16) {
            self.ptr16 = ac.get_code16(self.oplen) as u16;
            self.oplen += 2;
        }

        if flag.contains(opcode::OpFlags::MOFFSX) {
            self.get_moffs(ac)?;
        }

        Ok(())
    }
}

impl InstrData {
    fn get_legacy_prefix(&mut self, ac: &mut access::Access) -> Result<(), InstrError> {
        for _ in 0..4 {
            match ac.get_code8(self.oplen) {
                0x26 => { self.pre_segment = Some(segment::SgReg::ES); },
                0x2e => { self.pre_segment = Some(segment::SgReg::CS); },
                0x36 => { self.pre_segment = Some(segment::SgReg::SS); },
                0x3e => { self.pre_segment = Some(segment::SgReg::DS); },
                0x64 => { self.pre_segment = Some(segment::SgReg::FS); },
                0x65 => { self.pre_segment = Some(segment::SgReg::GS); },
                0x66 => { self.pre_size |= OverrideSize::OP; },
                0x67 => { self.pre_size |= OverrideSize::AD; },
                0xf2 => { self.pre_repeat = Some(Rep::REPNZ) },
                0xf3 => { self.pre_repeat = Some(Rep::REPZ) },
                _ => break,
            }
            self.oplen += 1;
        }
        Ok(())
    }

    fn get_rex_prefix(&mut self, ac: &mut access::Access) -> Result<(), InstrError> {
        let code = ac.get_code8(self.oplen);
        if (code >> 4) != 4 { return Ok(()); }

        self.rex = Rex::unpack(&code.to_be_bytes())?;
        self.oplen += 1;
        debug!("{:} ", self.rex);
        Ok(())
    }

    fn get_opcode(&mut self, ac: &mut access::Access) -> Result<(), InstrError> {
        self.opcd = ac.get_code8(self.oplen) as u16;
        self.oplen += 1;

        if self.opcd == 0x0f {
            self.opcd = (1<<8) + ac.get_code8(self.oplen) as u16;
        }
        debug!("opcode: {:02x} ", self.opcd);
        Ok(())
    }

    fn get_modrm(&mut self, ac: &mut access::Access) -> Result<(), InstrError> {
        let code = ac.get_code8(self.oplen);
        self.modrm = ModRM::unpack(&code.to_be_bytes())?;
        debug!("{:?} ", self.modrm);
        self.oplen += 1;

        let (mod_, rm) = (self.modrm.mod_, self.modrm.rm);
        if 16 == 32 {
            if mod_ != 3 && rm == 4 {
                self.sib = Sib::unpack(&ac.get_code8(self.oplen).to_be_bytes())?;
                self.oplen += 1;
                debug!("{:?} ", self.sib);
            }

            if mod_ == 2 || (mod_ == 0 && rm == 5) || (mod_ == 0 && self.sib.base == 5) {
                self.disp = ac.get_code32(self.oplen) as u32;
                self.oplen += 4;
            } else if mod_ == 1 {
                self.disp = ac.get_code8(self.oplen) as u32;
                self.oplen += 1;
            }
            debug!("disp: {:02x} ", self.disp);
        } else {
            if mod_ == 2 || (mod_ == 0 && rm == 6) {
                self.disp = ac.get_code16(self.oplen) as u32;
                self.oplen += 2;
            } else if mod_ == 1 {
                self.disp = ac.get_code8(self.oplen) as u32;
                self.oplen += 1;
            }
            debug!("disp: {:02x} ", self.disp);
        }
        Ok(())
    }

    fn get_moffs(&mut self, ac: &mut access::Access) -> Result<(), InstrError> {
        if 16 == 32 {
            self.moffs = ac.get_code32(self.oplen) as u64;
            self.oplen += 4;
        } else {
            self.moffs = ac.get_code16(self.oplen) as u64;
            self.oplen += 2;
        }
        Ok(())
    }
}