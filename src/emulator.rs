mod access;
mod instruction;

use crate::hardware::Hardware;
use crate::hardware::processor::general::*;
use crate::emulator::access::Access;
use crate::emulator::instruction::Instruction;

pub struct Emulator {
    ac: Access,
}

impl Emulator {
    pub fn new(hw: Hardware) -> Self {
        Emulator {
            ac: Access::new(hw),
        }
    }

    pub fn run(&mut self) -> () {
        /*
        let rr_core = Rc::new(RefCell::new(core));
        let ac = access::Access::new(Rc::clone(&rr_core), Rc::clone(&self.rr_mem));
        */

        let mut inst = Instruction::new();

        let mut x = self.ac.mem.as_mut_ptr(0x1030).unwrap() as *mut u64;
        self.ac.core.gpregs_mut().set(GpReg64::RSP, 0x1020);
        self.ac.push64(0xdeadbeef);
        self.ac.push64(0xcafebabe);
        unsafe {
            *x = 0x11223344;
            x = (x as usize + 8) as *mut u64;
            *x = 0x55667788;
        }
        self.ac.core.gpregs_mut().set(GpReg64::RSP, 0x1030);
        println!("{:x}", self.ac.pop64());
        self.ac.core.dump();

        self.ac.mem.dump(self.ac.core.gpregs().get(GpReg64::RSP) as usize, 0x40);

        inst.fetch(&mut self.ac);
        inst.exec(&mut self.ac);
    }
}