mod access;
mod instruction;

use std::error;
use thiserror::Error;
use super::hardware;

#[derive(Debug, Error)]
pub enum EmuException {
    #[error("Undefined Opecode")]
    UndefinedOpcode,
    #[error("Not Implemented Opecode")]
    NotImplementedOpcode,
    #[error("CPU Exception {0:?}")]
    CPUException(CPUException),
    #[error("Unexpected Error")]
    UnexpectedError,
}

#[derive(Debug)]
pub enum CPUException {
    DE = 0, DB = 1, BP = 3, OF = 4, BR = 5, UD = 6, NM = 7, DF = 8, TS = 10, NP = 11, SS = 12, GP = 13, PF = 14, MF = 16, AC = 17, MC = 18, XF = 19, VE = 20, SX = 30
}

#[derive(Clone, Copy)]
pub enum CpuMode { Real, Protected, Long }

pub struct Emulator {
    ac: access::Access,
    inst: instruction::Instruction,
    mode: CpuMode,
}

impl Emulator {
    pub fn new(hw: hardware::Hardware) -> Self {
        Self {
            ac: access::Access::new(hw),
            inst: instruction::Instruction::new(),
            mode: CpuMode::Real,
        }
    }

    pub fn step(&mut self) -> () {
        if let Err(err) = self.inst.fetch_exec(&mut self.ac, self.mode) {
            match err {
                _ => { panic!("{}", err); }
            }
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
}