mod access;
mod instruction;
mod interrupt;

use std::error;
use thiserror::Error;
use interrupt::IntrEvent;
use super::hardware;
use super::device;
use crate::hardware::processor::control::*;

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
    DE, DB,     BP, OF, BR, UD, NM,
    DF,     TS, NP, SS(Option<u16>), GP(Option<u16>), PF(u64),
    MF, AC, MC, XF, VE,
    SX
}

impl From<&CPUException> for u8 {
    fn from(e: &CPUException) -> u8 {
        match e {
            CPUException::DE => 0,     CPUException::DB => 1,                                CPUException::BP => 3,
            CPUException::OF => 4,     CPUException::BR => 5,     CPUException::UD => 6,     CPUException::NM => 7,
            CPUException::DF => 8,                                CPUException::TS => 10,    CPUException::NP => 11,
            CPUException::SS(_) => 12, CPUException::GP(_) => 13, CPUException::PF(_) => 14,
            CPUException::MF => 16,    CPUException::AC => 17,    CPUException::MC => 18,    CPUException::XF => 19,
            CPUException::VE => 20,
            CPUException::SX => 30,
        }
    }
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

    pub fn wake(&mut self) -> () {
        self.halt = false;
    }

    pub fn step(&mut self, debugged: bool) -> Option<Event> {
        if !self.halt {
            debug!("IP : 0x{:016x}", self.ac.core.ip.get_rip());
            match self.inst.fetch_exec(&mut self.ac) {
                Err(EmuException::Interrupt(i))    => self.intrpt.enqueue_top(IntrEvent::Software(i)),
                Err(EmuException::CPUException(e)) => {
                    match e {
                        CPUException::BP => self.ac.dump(),
                        CPUException::PF(laddr) => self.ac.core.cregs.2.from_u64(laddr),
                        _ => panic!("CPUException : {:?}", e),
                    }
                    debug!("CPUException : {:?}", e);
                    //self.intrpt.enqueue_top(IntrEvent::Hardware(u8::from(&e)));
                },
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
            Err(EmuException::CPUException(e)) => self.intrpt.enqueue_top(IntrEvent::Hardware(u8::from(&e))),
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

    pub fn dump(&self) -> () {
        self.ac.dump();
    }
}