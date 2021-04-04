use crate::emulator::access::Access;

const MAX_OPCODE: usize = 0x100;

pub struct Instruction {
    inst: [fn(&mut Access); MAX_OPCODE],
    opcode: u8,
}

impl Instruction {
    pub fn new() -> Self {
        Instruction {
            inst : [Instruction::undefined; MAX_OPCODE],
            opcode: Default::default(),
        }
    }

    pub fn fetch(&mut self, ac: &mut Access) -> () {
        self.opcode = ac.get_code8(0);
        ac.update_rip(1);
    }

    pub fn exec(&self, ac: &mut Access) -> () {
       self.inst[self.opcode as usize](ac);
    }

    pub fn undefined(_ac: &mut Access) -> () {
        panic!("Undefined Opcode");
    }
}