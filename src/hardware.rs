pub mod processor;
pub mod memory;
pub mod device;

pub struct Hardware {
    pub core: processor::Processor,
    pub mem: memory::Memory,
}

impl Hardware {
    pub fn new() -> Self {
        Self {
            core: processor::Processor::new(),
            mem: memory::Memory::new(),
        }
    }

    pub fn init_memory(&mut self, size: usize) -> () {
        self.mem.set_size(size);
    }
}