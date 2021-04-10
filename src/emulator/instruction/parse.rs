use packed_struct::prelude::*;
use super::opcode;
use crate::emulator::access;
use crate::hardware::processor::segment;

#[derive(Default)]
pub struct InstrData {
    pub pre_segment: Option<segment::SgReg>,
    pub pre_repeat: Option<Rep>,
    pub pre_size: Option<ChSz>,

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

pub enum Rep { REPZ, REPNZ }

bitflags! {
    pub struct ChSz: u8 {
        const REG  = 0b00000001;
        const ADDR = 0b00000010;
    }
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
    pub fn parse(&mut self, ac: &mut access::Access, op: &dyn opcode::OpcodeTrait) -> () {
        self.oplen = 0;

        self.parse_legacy_prefix(ac);
        // self.parse_rex_prefix(ac); 64 bit mode

        self.parse_opcode(ac);

        let flag = op.flag(self.opcd);
        if flag.contains(opcode::OpFlags::MODRM) {
            self.parse_modrm(ac);
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
            if 32 == 32 {
                self.moffs = ac.get_code32(self.oplen) as u64;
                self.oplen += 4;
            } else {
                self.moffs = ac.get_code16(self.oplen) as u64;
                self.oplen += 2;
            }
        }
    }
}

impl InstrData {
    fn parse_legacy_prefix(&mut self, ac: &mut access::Access) -> () {
        for _ in 0..4 {
            match ac.get_code8(self.oplen) {
                0x26 => { self.pre_segment = Some(segment::SgReg::ES); },
                0x2e => { self.pre_segment = Some(segment::SgReg::CS); },
                0x36 => { self.pre_segment = Some(segment::SgReg::SS); },
                0x3e => { self.pre_segment = Some(segment::SgReg::DS); },
                0x64 => { self.pre_segment = Some(segment::SgReg::FS); },
                0x65 => { self.pre_segment = Some(segment::SgReg::GS); },
                0x66 => { self.pre_size = Some(ChSz::REG); },
                0x67 => { self.pre_size = Some(ChSz::ADDR); },
                0xf2 => { self.pre_repeat = Some(Rep::REPNZ) },
                0xf3 => { self.pre_repeat = Some(Rep::REPZ) },
                _ => break,
            }
            self.oplen += 1;
        }
    }

    fn parse_rex_prefix(&mut self, ac: &mut access::Access) -> () {
        let code = ac.get_code8(self.oplen);
        if code < 0x40 || code > 0x4f { return; }

        self.rex = Rex::unpack(&code.to_be_bytes()).unwrap();
        self.oplen += 1;
        debug!("{:} ", self.rex);
    }

    fn parse_opcode(&mut self, ac: &mut access::Access) -> () {
        self.opcd = ac.get_code8(self.oplen) as u16;
        self.oplen += 1;

        if self.opcd == 0x0f {
            self.opcd = (1<<8) + ac.get_code8(self.oplen) as u16;
        }
        debug!("opcode: {:02x} ", self.opcd);
    }

    fn parse_modrm(&mut self, ac: &mut access::Access) -> () {
        let code = ac.get_code8(self.oplen);
        self.modrm = ModRM::unpack(&code.to_be_bytes()).unwrap();
        debug!("{:?} ", self.modrm);
        self.oplen += 1;

        let (mod_, rm) = (self.modrm.mod_, self.modrm.rm);
        if 32 == 32 {
            if mod_ != 3 && rm == 4 {
                self.sib = Sib::unpack(&ac.get_code8(self.oplen).to_be_bytes()).unwrap();
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
    }
}