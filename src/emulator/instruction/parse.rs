use packed_struct::prelude::*;
use crate::emulator::access;
use crate::emulator::instruction::opcode;

#[derive(Debug, Default)]
pub struct InstrData {
    //pre_segment: Option<segment::SgReg>,
    //pre_repeat: Option<Rep>,
    //segment: Option<segment::SgReg>,

    //prefix: u16,
    pub rex: Rex,
    pub opcd: u16,
    pub modrm: ModRM,
    pub sib: Sib,
    pub disp: i32,
    pub imm: i64,
    pub ptr16: i16,
    pub moffs: u32,
}

enum Rep { NONE, REPZ, REPNZ }

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
    #[packed_field(bits="0..2")] pub rm:  u8,
    #[packed_field(bits="3..5")] pub reg:  u8,
    #[packed_field(bits="6..7")] pub mod_:  u8,
}

#[derive(Debug, Default, Clone, Copy, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct Sib {  
    #[packed_field(bits="0..2")] pub base:  u8,
    #[packed_field(bits="3..5")] pub index:  u8,
    #[packed_field(bits="6..7")] pub scale:  u8,
}

impl InstrData {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn parse(&mut self, ac: &mut access::Access, op: &opcode::Opcode) -> () {
        self.parse_legacy_prefix(ac);
        // self.parse_rex_prefix(ac); 64 bit mode

        self.parse_opcode(ac);

        let flag = op.get().flag(self.opcd);
        if flag.contains(opcode::OpFlags::MODRM) {
            self.parse_modrm(ac);
        }

        if flag.contains(opcode::OpFlags::IMM32) {
            self.imm = ac.get_code32(0) as i64;
            ac.update_rip(4);
        }
        else if flag.contains(opcode::OpFlags::IMM16) {
            self.imm = ac.get_code16(0) as i64;
            ac.update_rip(2);
        } 
        else if flag.contains(opcode::OpFlags::IMM8) {
            self.imm = ac.get_code8(0) as i64;
            ac.update_rip(1);
        } 

        if flag.contains(opcode::OpFlags::PTR16) {
            self.ptr16 = ac.get_code16(0) as i16;
            ac.update_rip(2);
        }

        if flag.contains(opcode::OpFlags::MOFFSX) {
            if 16 == 32 {
                self.moffs = ac.get_code32(0);
                ac.update_rip(4);
            }
            else {
                self.moffs = ac.get_code16(0) as u32;
                ac.update_rip(2);
            }
        }
    }
}

impl InstrData {
    fn parse_legacy_prefix(&self, ac: &mut access::Access) -> () {
        for _ in 0..4 {
            match ac.get_code8(0) {
                0x26 => {},
                0x2e => {},
                0x36 => {},
                0x3e => {},
                0x64 => {},
                0x65 => {},
                0x66 => {},
                0x67 => {},
                0xf2 => {},
                0xf3 => {},
                _ => break,
            }
            ac.update_rip(1);
        }
    }

    fn parse_rex_prefix(&mut self, ac: &mut access::Access) -> () {
        let code = ac.get_code8(0);
        if code < 0x40 || code > 0x4f { return; }

        self.rex = Rex::unpack(&code.to_be_bytes()).unwrap();
        ac.update_rip(1);
        debug!("{:} ", self.rex);
    }

    fn parse_opcode(&mut self, ac: &mut access::Access) -> () {
        self.opcd = ac.get_code8(0) as u16;
        ac.update_rip(1);

        if self.opcd == 0x0f {
            self.opcd = (1<<8) + ac.get_code8(0) as u16;
        }
    }

    fn parse_modrm(&mut self, ac: &mut access::Access) -> () {
        let code = ac.get_code8(0);
        self.modrm = ModRM::unpack(&code.to_be_bytes()).unwrap();
        debug!("{:} ", self.modrm);
        ac.update_rip(1);

        let (mod_, rm) = (self.modrm.mod_, self.modrm.rm);
        if 16 == 32 {
            if mod_ != 3 && rm == 4 {
                self.sib = Sib::unpack(&ac.get_code8(0).to_be_bytes()).unwrap();
                ac.update_rip(1);
                debug!("{:} ", self.sib);
            }

            if mod_ == 2 || (mod_ == 0 && rm == 5) || (mod_ == 0 && self.sib.base == 5) {
                self.disp = ac.get_code32(0) as i32;
                ac.update_rip(4);
            }
            else if mod_ == 1 {
                self.disp = ac.get_code8(0) as i32;
                ac.update_rip(1);
            }
            debug!("{:} ", self.disp);
        }
        else {
            if mod_ == 2 || (mod_ == 0 && rm == 6) {
                self.disp = ac.get_code16(0) as i32;
                ac.update_rip(2);
            }
            else if mod_ == 1 {
                self.disp = ac.get_code8(0) as i32;
                ac.update_rip(1);
            }
            debug!("{:} ", self.disp);
        }
    }
}