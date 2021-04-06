mod common;
mod opcode16;
mod opcode32;
mod opcode64;

bitflags! {
    pub struct OpFlags: u8 {
        const NONE  = 0b00000000;
        const MODRM = 0b00000001;
        const IMM   = 0b00000010;
        const PTR   = 0b00000100;
        const MOFFS = 0b00001000;
        const SZ64  = 0b00010000;
        const SZ32  = 0b00100000;
        const SZ16  = 0b01000000;
        const SZ8   = 0b10000000;
        const IMM32     = Self::IMM.bits | Self::SZ32.bits;
        const IMM16     = Self::IMM.bits | Self::SZ16.bits;
        const IMM8      = Self::IMM.bits | Self::SZ8.bits;
        const PTR16     = Self::PTR.bits | Self::SZ16.bits;
        const MOFFSX   = Self::MOFFS.bits | Self::SZ32.bits | Self::SZ16.bits;
        const MOFFS8   = Self::MOFFS.bits | Self::SZ8.bits;
    }
}

#[derive(Clone, Copy)]
pub struct OpcodeType {
    func: fn(&mut super::InstrArg),
    flag: OpFlags,
}
impl Default for OpcodeType {
    fn default() -> Self {
        Self { func:undefined, flag:OpFlags::NONE }
    }
}

const MAX_OPCODE: usize = 0x200;
type OpcodeArr = [OpcodeType; MAX_OPCODE];

pub struct Opcode {
    op16: opcode16::Opcode16,
    op32: opcode32::Opcode32,
    op64: opcode64::Opcode64,
}

impl Opcode {
    pub fn new() -> Self {
        let mut opa: OpcodeArr = [ Default::default(); MAX_OPCODE];
        common::init_cmn_opcode(&mut opa);

        let mut op = Opcode {
            op16: opcode16::Opcode16::new(opa),
            op32: opcode32::Opcode32::new(opa),
            op64: opcode64::Opcode64::new(opa),
        };
        op.op16.init_opcode();
        op.op32.init_opcode();
        op.op64.init_opcode();
        op
    }

    pub fn get(&self) -> &dyn OpcodeTrait {
        &self.op16
    }
}

pub trait OpcodeTrait {
    fn init_opcode(&mut self) -> ();
    fn exec(&self, arg: &mut super::InstrArg) -> ();
    fn flag(&self, opcode: u16) -> OpFlags;
}

fn undefined(arg: &mut super::InstrArg) -> () {
    arg.ac.dump();
    panic!("Undefined Opcode");
}