mod access;
mod instruction;

use std::error;
use super::hardware;

pub struct Emulator {
    ac: access::Access,
    inst: instruction::Instruction,
    mode: CpuMode,
}

#[derive(Clone, Copy)]
pub enum CpuMode { Real, Protected, Long }

impl Emulator {
    pub fn new(hw: hardware::Hardware) -> Self {
        Self {
            ac: access::Access::new(hw),
            inst: instruction::Instruction::new(),
            mode: CpuMode::Real,
        }
    }

    pub fn load_binary(&mut self, path: String, addr: usize) -> Result<(), Box<dyn error::Error>> {
        use std::io::Read;
        use std::fs::File;
        use libc::c_void;

        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        let len = file.read_to_end(&mut buf)?;
        self.ac.mem.write_data(addr, buf.as_ptr() as *const c_void, len)?;

        Ok(())
    }

    pub fn step(&mut self) -> () {
        self.inst.fetch_exec(&mut self.ac, self.mode).unwrap();
    }
}