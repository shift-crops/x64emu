mod access;
mod instruction;

use crate::hardware::Hardware;
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

    pub fn load_binary(&mut self) -> () {
        for i in 0..2 {
            self.ac.set_data64(0xfff0+i*8, 0x9090909090909090);
        }
    }

    pub fn run(&mut self) -> () {
        let mut inst = Instruction::new();

        loop {
            inst.fetch_exec(&mut self.ac);
        }
    }

    #[cfg(test)]
    pub fn test(&mut self) -> () {
        use crate::hardware::processor::general::*;

        self.ac.core.gpregs_mut().set(GpReg64::RSP, 0xf20);
        self.ac.push64(0xdeadbeef);
        self.ac.push64(0xcafebabe);
        assert_eq!(self.ac.pop64(), 0xcafebabe);
        assert_eq!(self.ac.pop64(), 0xdeadbeef);

        let mut x = self.ac.mem.as_mut_ptr(0xf20).unwrap() as *mut u64;
        unsafe {
            *x = 0x11223344;
            x = (x as usize + 8) as *mut u64;
            *x = 0x55667788;
        }
        assert_eq!(self.ac.pop64(), 0x11223344);
        assert_eq!(self.ac.pop64(), 0x55667788);

        self.ac.core.dump();
        self.ac.mem.dump(self.ac.core.gpregs().get(GpReg64::RSP) as usize, 0x40);
    }
}