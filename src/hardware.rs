pub mod processor;
pub mod memory;

pub struct Hardware {
    pub core: processor::Processor,
    pub mem: memory::Memory,
}

impl Hardware {
    pub fn new(size: usize) -> Self {
        Self {
            core: processor::Processor::new(),
            mem: memory::Memory::new(size),
        }
    }
}