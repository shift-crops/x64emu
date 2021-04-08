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

    pub fn load_binary(&mut self, path: String, addr: usize) -> Result<(), Box<dyn std::error::Error>> {
        use std::io::Read;
        use std::fs::File;
        use libc::c_void;

        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        let len = file.read_to_end(&mut buf)?;
        self.ac.mem.write_data(addr, buf.as_ptr() as *const c_void, len)?;

        Ok(())
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

        self.ac.core.gpregs.set(GpReg64::RSP, 0xf20);
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
        self.ac.mem.dump(self.ac.core.gpregs.get(GpReg64::RSP) as usize, 0x40);
    }
}