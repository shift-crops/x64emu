mod memory;
mod msr;

use crate::hardware;

pub struct Access {
    pub core: hardware::processor::Processor,
    pub mem: hardware::memory::Memory,
}

impl Access {
    pub fn new(hw: hardware::Hardware) -> Self {
        Self {
            core: hw.core,
            mem: hw.mem,
        }
    }

    pub fn dump(&self) -> () {
        self.core.dump();
        self.dump_code();
        self.dump_stack();
    }
}