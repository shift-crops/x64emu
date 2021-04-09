use crate::emulator::instruction::opcode::*;
use crate::hardware::processor::general::*;

pub struct Opcode16 (pub super::OpcodeArr);
impl Opcode16 {
    pub fn new(op: super::OpcodeArr) -> Self {
        Self (op)
    }
}

impl super::OpcodeTrait for Opcode16 {
    fn init_opcode(&mut self) -> (){
        macro_rules! setop {
            ($n:expr, $fnc:ident, $flg:expr) => { self.0[$n & 0x1ff] = OpcodeType{func:Self::$fnc, flag:$flg} }
        }

    }

    fn exec(&self, exec: &mut exec::Exec) -> () { (self.0[exec.idata.opcd as usize].func)(exec); exec.update_rip(exec.idata.oplen as i64); }
    fn flag(&self, opcode: u16) -> OpFlags { self.0[opcode as usize].flag }
}

