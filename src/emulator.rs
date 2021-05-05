mod access;
mod instruction;
mod interrupt;

use std::error;
use thiserror::Error;
use super::hardware;
use super::device;
use interrupt::IntrEvent;

#[derive(Debug, Error)]
pub enum EmuException {
    #[error("CPU Exception {0:?}")]
    CPUException(CPUException),
    #[error("Interrupt {0:?}")]
    Interrupt(u8),
    #[error("Halt")]
    Halt,
    #[error("Undefined Opecode")]
    UndefinedOpcode,
    #[error("Not Implemented Opecode")]
    NotImplementedOpcode,
    #[error("Not Implemented Function")]
    NotImplementedFunction,
    #[error("Unexpected Error")]
    UnexpectedError,
}

#[derive(Debug)]
pub enum CPUException {
    DE = 0,  DB = 1,           BP = 3,  OF = 4,  BR = 5,  UD = 6,  NM = 7,
    DF = 8,           TS = 10, NP = 11, SS = 12, GP = 13, PF = 14,
    MF = 16, AC = 17, MC = 18, XF = 19, VE = 20,          SX = 30
}

pub struct Emulator {
    pub ac: access::Access,
    inst: instruction::Instruction,
    intrpt: interrupt::Interrupt,
    halt: bool,
    pub breakpoints: Vec<u32>,
}

#[derive(Debug)]
pub enum Event {
    Halted,
    Break,
    WatchWrite(u32),
    WatchRead(u32),
}

impl Emulator {
    pub fn new(hw: hardware::Hardware, dev: device::Device) -> Self {
        Emulator {
            ac: access::Access::new(hw, dev),
            inst: instruction::Instruction::new(),
            intrpt: Default::default(),
            halt: false,
            breakpoints: Vec::new(),
        }
    }

    pub fn run(&mut self) -> () {
        loop {
            self.step(false);
        }
    }

    pub fn step(&mut self, debugged: bool) -> Option<Event> {
        if !self.halt {
            debug!("IP : 0x{:016x}", self.ac.core.ip.get_rip());
            match self.inst.fetch_exec(&mut self.ac) {
                Err(EmuException::Interrupt(3))    => self.ac.dump(),
                Err(EmuException::Interrupt(i))    => self.intrpt.enqueue(IntrEvent::Software(i)),
                Err(EmuException::CPUException(e)) => self.intrpt.enqueue(IntrEvent::Hardware(e as u8)),
                Err(EmuException::Halt)            => self.halt = true,
                Err(err) => {
                    self.ac.dump();
                    panic!("{}", err);
                },
                _ => {},
            }
        }

        if let Some(ev) = self.ac.check_irq(self.halt && !debugged) {
            debug!("Interrupt Occured : 0x{:02x}", ev);
            self.intrpt.enqueue(IntrEvent::Hardware(ev));
            self.halt = false;
        }

        match self.intrpt.handle(&mut self.ac) {
            Err(EmuException::CPUException(e)) => self.intrpt.enqueue(IntrEvent::Hardware(e as u8)),
            Err(err) => {
                self.ac.dump();
                panic!("{}", err)
            },
            _ => {},
        }

        if debugged && self.breakpoints.contains(&(self.ac.core.ip.get_eip())) {
            Some(Event::Break)
        } else if self.halt {
            Some(Event::Halted)
        } else {
            None
        }
    }

    pub fn map_binary(&mut self, addr: usize, bin: &[u8]) -> Result<(), Box<dyn error::Error>> {
        self.ac.mem.write().unwrap().write_data(addr, bin.as_ptr() as *const _, bin.len())?;

        Ok(())
    }

    pub fn load_binfile(&mut self, addr: usize, path: String) -> Result<(), Box<dyn error::Error>> {
        use std::io::Read;
        use std::fs::File;

        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        let len = file.read_to_end(&mut buf)?;
        self.ac.mem.write().unwrap().write_data(addr, buf.as_ptr() as *const _, len)?;

        Ok(())
    }
}