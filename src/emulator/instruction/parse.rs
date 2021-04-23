use packed_struct::prelude::*;
use super::opcode;
use crate::emulator::access;
use crate::emulator::access::register::*;
use crate::emulator::EmuException;

#[derive(Default)]
pub struct ParseInstr {
    pub prefix: PrefixData,
    pub instr: InstrData,
}

#[derive(Default)]
pub struct PrefixData {
    pub segment: Option<SgReg>,
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
    pub adsize: access::AcsSize,

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
    pub fn parse_prefix(&mut self, ac: &mut access::Access) -> Result<(), EmuException> {
        self.get_legacy_prefix(ac)?;

        if let (access::CpuMode::Long, access::AcsSize::BIT64) = (&ac.mode, ac.size.ad) {
            self.get_rex_prefix(ac)?;
        }
        Ok(())
    }

    pub fn parse_opcode(&mut self, ac: &mut access::Access) -> Result<(), EmuException> {
        self.get_opcode(ac)?;
        Ok(())
    }

    pub fn parse_oprand(&mut self, ac: &mut access::Access, flag: opcode::OpFlags, adsize: access::AcsSize) -> Result<(), EmuException> {
        self.instr.adsize = adsize;

        if flag.contains(opcode::OpFlags::MODRM) {
            self.get_modrm(ac)?;
            self.get_sib_disp(ac)?;
        }

        if flag.contains(opcode::OpFlags::IMM) {
            let (imm, len) = match flag & opcode::OpFlags::SZBIT {
                opcode::OpFlags::SZ8  => (ac.get_code8(self.instr.len)? as u64, 1),
                opcode::OpFlags::SZ16 => (ac.get_code16(self.instr.len)? as u64, 2),
                opcode::OpFlags::SZ32 => (ac.get_code32(self.instr.len)? as u64, 4),
                opcode::OpFlags::SZ64 => (ac.get_code64(self.instr.len)? as u64, 8),
                _ => (0, 0),
            };
            self.instr.imm = imm;
            self.instr.len += len;
        }

        if flag.contains(opcode::OpFlags::PTR16) {
            self.instr.ptr16 = ac.get_code16(self.instr.len)? as u16;
            self.instr.len += 2;
        }

        if flag.contains(opcode::OpFlags::MOFFS) {
            self.get_moffs(ac)?;
        }

        Ok(())
    }
}

impl ParseInstr {
    fn get_legacy_prefix(&mut self, ac: &mut access::Access) -> Result<(), EmuException> {
        let prefix = &mut self.prefix;
        for _ in 0..4 {
            match ac.get_code8(self.instr.len)? {
                0x26 => prefix.segment = Some(SgReg::ES),
                0x2e => prefix.segment = Some(SgReg::CS),
                0x36 => prefix.segment = Some(SgReg::SS),
                0x3e => prefix.segment = Some(SgReg::DS),
                0x64 => prefix.segment = Some(SgReg::FS),
                0x65 => prefix.segment = Some(SgReg::GS),
                0x66 => prefix.size |= OverrideSize::OP,
                0x67 => prefix.size |= OverrideSize::AD,
                0xf2 => prefix.repeat = Some(Rep::REPNZ),
                0xf3 => prefix.repeat = Some(Rep::REPZ),
                _ => break,
            }
            self.instr.len += 1;
        }
        Ok(())
    }

    fn get_rex_prefix(&mut self, ac: &mut access::Access) -> Result<(), EmuException> {
        let code = ac.get_code8(self.instr.len)?;
        if (code >> 4) != 4 { return Ok(()); }

        self.prefix.rex = Rex::unpack(&code.to_be_bytes()).unwrap();
        self.instr.len += 1;
        debug!("{:} ", self.prefix.rex);
        Ok(())
    }

    fn get_opcode(&mut self, ac: &mut access::Access) -> Result<(), EmuException> {
        let mut opcode = ac.get_code8(self.instr.len)? as u16;
        self.instr.len += 1;

        if opcode == 0x0f {
            opcode = (1<<8) + ac.get_code8(self.instr.len)? as u16;
            self.instr.len += 1;
            debug!("opcode: 0f{:02x} ", opcode&0xff);
        } else {
            debug!("opcode: {:02x} ", opcode);
        }
        self.instr.opcode = opcode;
        Ok(())
    }

    fn get_modrm(&mut self, ac: &mut access::Access) -> Result<(), EmuException> {
        let code = ac.get_code8(self.instr.len)?;
        self.instr.modrm = ModRM::unpack(&code.to_be_bytes()).unwrap();
        debug!("{:?} ", self.instr.modrm);
        self.instr.len += 1;
        Ok(())
    }

    fn get_sib_disp(&mut self, ac: &mut access::Access) -> Result<(), EmuException> {
        let (mod_, rm) = (self.instr.modrm.mod_, self.instr.modrm.rm);
        match self.instr.adsize {
            access::AcsSize::BIT16 => {
                match (mod_, rm) {
                    (1, _) => {
                        self.instr.disp = ac.get_code8(self.instr.len)? as u32;
                        self.instr.len += 1;
                    },
                    (2, _) | (0, 6) => {
                        self.instr.disp = ac.get_code16(self.instr.len)? as u32;
                        self.instr.len += 2;
                    },
                    _ => {},
                }
                debug!("disp: {:04x} ", self.instr.disp);
            },
            access::AcsSize::BIT32 | access::AcsSize::BIT64 => {
                if mod_ != 3 && rm == 4 {
                    self.instr.sib = Sib::unpack(&ac.get_code8(self.instr.len)?.to_be_bytes()).unwrap();
                    self.instr.len += 1;
                    debug!("{:?} ", self.instr.sib);
                }

                match (mod_, rm, self.instr.sib.base) {
                    (1, _, _) => {
                        self.instr.disp = ac.get_code8(self.instr.len)? as u32;
                        self.instr.len += 1;
                    },
                    (2, _, _) | (0, 5, _) | (0, 4, 5)=> {
                        self.instr.disp = ac.get_code32(self.instr.len)? as u32;
                        self.instr.len += 4;
                    },
                    _ => {},
                }
                debug!("disp: {:08x} ", self.instr.disp);
            },
        }
        Ok(())
    }

    fn get_moffs(&mut self, ac: &mut access::Access) -> Result<(), EmuException> {
        match self.instr.adsize {
            access::AcsSize::BIT16 => {
                self.instr.moffs = ac.get_code16(self.instr.len)? as u64;
                self.instr.len += 2;
            },
            access::AcsSize::BIT32 => {
                self.instr.moffs = ac.get_code32(self.instr.len)? as u64;
                self.instr.len += 4;
            },
            access::AcsSize::BIT64 => {
                self.instr.moffs = ac.get_code64(self.instr.len)? as u64;
                self.instr.len += 8;
            },
        }
        Ok(())
    }
}