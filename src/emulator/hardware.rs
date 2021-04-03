mod processor;
mod memory;
mod device;

pub struct Hardware {
    core: Vec<processor::Processor>,
    mem: memory::Memory,
}

impl Hardware {
    pub fn new() -> Self {
        Hardware {
            core: Vec::new(),
            mem: memory::Memory::new(),
        }
    }

    pub fn init_core(&mut self, ncore: usize) -> () {
        self.core = vec![processor::Processor::new(); ncore];
    }

    pub fn init_memory(&mut self, size: usize) -> () {
        self.mem.set_size(size);
    }

    pub fn test(&mut self) -> () {
        self.mem.write32(0x1000, 0xdeadbeef);
        self.mem.write32(0x1004, 0xcafebabe);
        self.mem.dump(0x1000, 0x100);

        for c in &mut self.core {
            c.test();
            c.dump();
        }
    }
}