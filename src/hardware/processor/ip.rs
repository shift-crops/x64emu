#[repr(C)]
#[derive(Clone, Copy)]
pub union InstructionPointer {
    rip: u64,
    eip: u32,
    ip: u16,
}

impl InstructionPointer {
    pub fn new(rv: u64) -> Self {
        Self { rip: rv, }
    }

    pub fn get(&self) -> u64 { unsafe { self.rip } }
    pub fn set(&mut self, v: u64) -> () { self.rip = v; }
    pub fn update(&mut self, v: i64) -> () { unsafe { self.rip = (self.rip as i64 + v) as u64; } }
}
