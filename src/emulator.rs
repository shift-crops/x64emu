mod processor;
mod memory;
mod device;

pub struct Emulator {
    cpu: processor::Processor,
    mem: Vec<u8>,
}

impl Emulator {
    pub fn new(size: usize) -> Emulator {
        Emulator {
            cpu: processor::Processor::new(),
            mem: vec![0; size],
        }
    }
 
    pub fn dump(&self) {
        println!("mem size: {:x}", self.mem.len());
        processor::Processor::dump(&self.cpu);
    }
}