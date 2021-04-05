use crate::emulator::access;
use crate::emulator::instruction::parse;
use crate::hardware::processor::general::*;
use crate::emulator::instruction::opcode::*;

pub struct Opcode16 (pub super::OpcodeArr);
impl Opcode16 {
    pub fn new(op: super::OpcodeArr) -> Self {
        Opcode16 (op)
    }
}

impl super::OpcodeTrait for Opcode16 {
    fn init_opcode(&mut self) -> (){
        macro_rules! setop {
            ($n:expr, $fnc:ident, $flg:expr) => { self.0[$n & 0x1ff] = OpcodeType{func:$fnc, flag:$flg} }
        }

        setop!(0x90, hoge, OpFlags::NONE);
    }

    fn exec(&self, ac: &mut access::Access, idata: &parse::InstrData) -> () {
        (self.0[idata.opcode as usize].func)(ac, &idata);
    }

    fn flag(&self, opcode: u16) -> OpFlags {
        self.0[opcode as usize].flag
    }
}

fn hoge (ac: &mut access::Access, _idata: &parse::InstrData) {
    ac.core.gpregs_mut().set(GpReg64::RAX, 0xdeadbeef);
    ac.pop64();
    println!("hoge!");
}