mod access;
mod instruction;

use super::hardware;

pub struct Emulator {
    pub ac: access::Access,
    inst: instruction::Instruction,
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
            breakpoints: Vec::new(),
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

    pub fn step(&mut self) -> Option<Event> {
        self.inst.fetch_exec(&mut self.ac);

        if self.breakpoints.contains(&(self.ac.core.rip.get() as u32)) {
            return Some(Event::Break);
        }
        None
    }

    pub fn run(&mut self) -> () {
        loop {
            self.inst.fetch_exec(&mut self.ac);
        }
    }
}