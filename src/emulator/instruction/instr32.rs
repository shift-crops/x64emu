use crate::emulator::access;
use crate::emulator::instruction::parse;
use crate::hardware::processor::general::*;

pub fn init_instr(arr: &mut super::InstArr){
    arr[0x90] = hoge;
}

fn hoge (ac: &mut access::Access, pi: parse::ParseInstr) {
    ac.core.gpregs_mut().set(GpReg64::RAX, 0xdeadbeef);
    ac.pop64();
    println!("hoge!");
}