mod access;
mod instruction;

use super::hardware;

pub struct Emulator {
    ac: access::Access,
}

impl Emulator {
    pub fn new(hw: hardware::Hardware) -> Self {
        Self {
            ac: access::Access::new(hw),
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
        let mut inst = instruction::Instruction::new();

        loop {
            inst.fetch_exec(&mut self.ac);
        }
    }
}