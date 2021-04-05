mod common;
mod opcode16;
mod opcode32;
mod opcode64;

use crate::emulator::access;
use crate::emulator::instruction::parse;

const MAX_OPCODE: usize = 0x200;
type OpcodeArr = [fn(&mut access::Access, &parse::InstrData); MAX_OPCODE];

pub struct Opcode {
    op16: opcode16::Opcode16,
    op32: opcode32::Opcode32,
    op64: opcode64::Opcode64,
}

impl Opcode {
    pub fn new() -> Self {
        let mut opa: OpcodeArr = [undefined; MAX_OPCODE];
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
    fn exec(&self, ac: &mut access::Access, idata: &parse::InstrData) -> ();
}

fn undefined(ac: &mut access::Access, _idata: &parse::InstrData) -> () {
    ac.dump();
    panic!("Undefined Opcode");
}