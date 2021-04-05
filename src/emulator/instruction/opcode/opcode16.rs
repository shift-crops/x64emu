use crate::emulator::access;
use crate::emulator::instruction::parse;
use crate::hardware::processor::general::*;

pub struct Opcode16 (pub super::OpcodeArr);
impl Opcode16 {
    pub fn new(opa: super::OpcodeArr) -> Self {
        Opcode16 (opa)
    }
}

impl super::OpcodeTrait for Opcode16 {
    fn init_opcode(&mut self) -> (){
        self.0[0x90] = hoge;
    }

    fn exec(&self, ac: &mut access::Access, idata: &parse::InstrData) -> () {
        self.0[idata.opcode as usize](ac, &idata);
    }
}

fn hoge (ac: &mut access::Access, _idata: &parse::InstrData) {
    ac.core.gpregs_mut().set(GpReg64::RAX, 0xdeadbeef);
    ac.pop64();
    println!("hoge!");
}