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
        use crate::hardware::processor::general::*;

        self.core.dump();
        self.mem.dump(self.core.ip.get_rip() as usize -0x10 , 0x20);
        self.mem.dump(self.core.gpregs.get(GpReg64::RSP) as usize, 0x40);
    }
}