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

    pub fn test(&mut self) -> () {
        self.core.test();
        self.core.dump();

        self.mem.test();
        self.mem.dump(0x1000, 0x100);
    }
}