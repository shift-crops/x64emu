pub mod processor;
pub mod memory;
pub mod device;

use processor::general::*;

pub struct Hardware {
    pub core: processor::Processor,
    pub mem: memory::Memory,
}

impl Hardware {
    pub fn new() -> Self {
        Hardware {
            core: processor::Processor::new(),
            mem: memory::Memory::new(),
        }
    }

    pub fn init_memory(&mut self, size: usize) -> () {
        self.mem.set_size(size);
    }

    pub fn dump(&mut self) -> () {
        self.core.dump();
        self.mem.dump(self.core.rip().get() as usize -0x10 , 0x20);
        self.mem.dump(self.core.gpregs().get(GpReg64::RSP) as usize, 0x40);
    }

    pub fn test(&mut self) -> () {
        self.core.test();
        self.core.dump();

        self.mem.test();
        self.mem.dump(0x1000, 0x100);
    }
}