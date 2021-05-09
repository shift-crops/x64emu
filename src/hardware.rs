pub mod processor;
pub mod memory;

use std::sync::{Arc, RwLock};

pub struct Hardware {
    pub core: processor::Processor,
    pub mem: Arc<RwLock<memory::Memory>>,
}

impl Hardware {
    pub fn new(size: usize) -> Self {
        Self {
            core: processor::Processor::new(),
            mem: Arc::new(RwLock::new(memory::Memory::new(size))),
        }
    }
}