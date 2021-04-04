use crate::emulator::access;
use crate::emulator::instruction::parse;
use crate::hardware::processor::general::*;

pub fn init_instr(arr: &mut super::InstArr){
    arr[0x90] = nop;
}

fn nop (ac: &mut access::Access, pi: parse::ParseInstr){}