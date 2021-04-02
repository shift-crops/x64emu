mod processor;
mod memory;
mod device;

pub struct Emulator {
    pub cpu: processor::Processor,
    mem: Vec<u8>,
}

impl Emulator {
    pub fn new(size: usize) -> Emulator {
        Emulator {
            cpu: processor::Processor::new(),
            mem: vec![0; size],
        }
    }
}