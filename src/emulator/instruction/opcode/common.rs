use crate::emulator::access;
use crate::emulator::instruction::parse;

pub fn init_cmn_opcode(opa: &mut super::OpcodeArr){
    opa[0x90] = nop;
}

fn nop (_ac: &mut access::Access, _idata: &parse::InstrData){}