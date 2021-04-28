mod access;
mod instruction;
mod interrupt;

use std::error;
use thiserror::Error;
use super::hardware;

#[derive(Debug, Error)]
pub enum EmuException {
    #[error("Undefined Opecode")]
    UndefinedOpcode,
    #[error("Not Implemented Opecode")]
    NotImplementedOpcode,
    #[error("Not Implemented Function")]
    NotImplementedFunction,
    #[error("CPU Exception {0:?}")]
    CPUException(CPUException),
    #[error("Interrupt {0:?}")]
    Interrupt(interrupt::Event),
    #[error("Unexpected Error")]
    UnexpectedError,
}

#[derive(Debug)]
pub enum CPUException {
    DE = 0,  DB = 1,           BP = 3,  OF = 4,  BR = 5,  UD = 6,  NM = 7,
    DF = 8,           TS = 10, NP = 11, SS = 12, GP = 13, PF = 14,
    MF = 16, AC = 17, MC = 18, XF = 19, VE = 20,          SX = 30
}
impl From<CPUException> for interrupt::Event {
    fn from(e: CPUException) -> Self {
        Self::Hardware(e as u8)
    }
}

pub struct Emulator {
    pub ac: access::Access,
    inst: instruction::Instruction,
    intrpt: interrupt::Interrupt,
    pub breakpoints: Vec<u32>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Event {
    Halted,
    Break,
    WatchWrite(u32),
    WatchRead(u32),
}

impl Emulator {
    pub fn new(hw: hardware::Hardware) -> Self {
        Emulator {
            ac: access::Access::new(hw),
            inst: instruction::Instruction::new(),
            intrpt: Default::default(),
            breakpoints: Vec::new(),
        }
    }

    pub fn run(&mut self) -> () {
        loop {
            self.step();
        }
    }

    pub fn step(&mut self) -> Option<Event> {

        debug!("IP : 0x{:016x}", self.ac.core.ip.get_rip());
        if let Err(err) = self.inst.fetch_exec(&mut self.ac) {
            match err {
                EmuException::Interrupt(i)    => self.intrpt.enqueue(i),
                EmuException::CPUException(e) => {
                    debug!("CPUException {:?}", e);
                    self.intrpt.enqueue(e.into())
                },
                _ => {
                    self.ac.dump();
                    panic!("{}", err);
                }
            }
        }

        if let Err(err) = self.intrpt.handle(&mut self.ac) {
            match err {
                EmuException::CPUException(e) => self.intrpt.enqueue(e.into()),
                _ => {
                    self.ac.dump();
                    panic!("{}", err)
                },
            }
        }

        if self.breakpoints.contains(&(self.ac.core.ip.get_eip())) {
            return Some(Event::Break);
        }
        None
    }

    pub fn map_binary(&mut self, addr: usize, bin: &[u8]) -> Result<(), Box<dyn error::Error>> {
        self.ac.mem.write_data(addr, bin.as_ptr() as *const _, bin.len())?;

        Ok(())
    }

    pub fn load_binfile(&mut self, addr: usize, path: String) -> Result<(), Box<dyn error::Error>> {
        use std::io::Read;
        use std::fs::File;

        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        let len = file.read_to_end(&mut buf)?;
        self.ac.mem.write_data(addr, buf.as_ptr() as *const _, len)?;

        Ok(())
    }
}