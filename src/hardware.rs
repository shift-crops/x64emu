pub mod processor;
pub mod memory;
pub mod device;

pub struct Hardware {
    pub core: processor::Processor,
    pub mem: memory::Memory,
}

impl Hardware {
    pub fn new(rst_vct: u64, size: usize) -> Self {
        Self {
            core: processor::Processor::new(rst_vct),
            mem: memory::Memory::new(size),
        }
    }
}