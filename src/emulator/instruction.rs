mod parse;
mod instrcmn;
mod instr16;
mod instr32;
mod instr64;

use crate::emulator::access::Access;

const MAX_OPCODE: usize = 0x100;
type InstArr = [fn(&mut Access, parse::ParseInstr); MAX_OPCODE];

pub struct Instruction {
    inst: Box<InstArr>,
    inst16: InstArr,
    inst32: InstArr,
    inst64: InstArr,
}

impl Instruction {
    pub fn new() -> Self {
        let mut inst16: InstArr = [Instruction::undefined; MAX_OPCODE];
        instrcmn::init_instr(&mut inst16);
        let mut inst32: InstArr = inst16;
        let mut inst64: InstArr = inst16;
        instr16::init_instr(&mut inst16);
        instr32::init_instr(&mut inst32);
        instr64::init_instr(&mut inst64);

        Instruction {
            inst: Box::new(inst16),
            inst16, inst32, inst64,
        }
    }

    pub fn fetch(&mut self, ac: &mut Access) -> () {
        let mut pi = parse::ParseInstr::new();
        pi.parse(ac);

        self.inst[pi.opcode as usize](ac, pi);
    }

    pub fn undefined(ac: &mut Access, pi: parse::ParseInstr) -> () {
        ac.dump();
        panic!("Undefined Opcode");
    }
}