use crate::emulator::access;
use crate::emulator::instruction::parse;
use crate::emulator::instruction::opcode::*;

pub fn init_cmn_opcode(op: &mut super::OpcodeArr){
    macro_rules! setop {
        ($n:expr, $fnc:ident, $flg:expr) => { op[$n & 0x1ff] = OpcodeType{func:$fnc, flag:$flg} }
    }

    setop!(0x90, nop, OpFlags::NONE);
}

fn nop (_ac: &mut access::Access, _idata: &parse::InstrData){}