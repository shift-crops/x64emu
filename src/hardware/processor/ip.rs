#[derive(Clone, Copy)]
pub struct InstructionPointer(u64);

impl InstructionPointer {
    pub fn new(rst_vct: u64) -> Self {
        Self ( rst_vct )
    }

    pub fn get_ip(&self) -> u16 { self.0 as u16 }
    pub fn get_eip(&self) -> u32 { self.0 as u32 }
    pub fn get_rip(&self) -> u64 { self.0 }

    pub fn set_ip(&mut self, v: u16) -> () { self.0 = v as u64; }
    pub fn set_eip(&mut self, v: u32) -> () { self.0 = v as u64; }
    pub fn set_rip(&mut self, v: u64) -> () { self.0 = v; }

    pub fn update_ip(&mut self, v: i16) -> () { self.0 = (self.0 as i16).wrapping_add(v) as u64; }
    pub fn update_eip(&mut self, v: i32) -> () { self.0 = (self.0 as i32).wrapping_add(v) as u64; }
    pub fn update_rip(&mut self, v: i64) -> () { self.0 = (self.0 as i64).wrapping_add(v) as u64; }
}
