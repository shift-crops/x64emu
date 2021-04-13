#[repr(C)]
#[derive(Clone, Copy)]
pub union InstructionPointer {
    rip: u64,
    eip: u32,
    ip: u16,
}

impl InstructionPointer {
    pub fn new(rst_vct: u64) -> Self {
        Self { rip: rst_vct, }
    }

    pub fn get_ip(&self) -> u16 { unsafe { self.ip } }
    pub fn get_eip(&self) -> u32 { unsafe { self.eip } }
    pub fn get_rip(&self) -> u64 { unsafe { self.rip } }

    pub fn set_ip(&mut self, v: u16) -> () { self.ip = v; }
    pub fn set_eip(&mut self, v: u32) -> () { self.eip = v; }
    pub fn set_rip(&mut self, v: u64) -> () { self.rip = v; }

    pub fn update_ip(&mut self, v: i16) -> () { unsafe { self.ip = (self.ip as i16).wrapping_add(v) as u16; } }
    pub fn update_eip(&mut self, v: i32) -> () { unsafe { self.eip = (self.eip as i32).wrapping_add(v) as u32; } }
    pub fn update_rip(&mut self, v: i64) -> () { unsafe { self.rip = (self.rip as i64).wrapping_add(v) as u64; } }
}
