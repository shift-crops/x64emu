mod general;
mod rflags;
mod segment;
mod descriptor;

pub struct Processor {
    gpregs: [general::GpRegister; general::GPREGS_COUNT],
    rflags: u64,
    rip: u64,
}

impl Processor {
    pub fn new() -> Processor {
        let gpr = general::GpRegister::new();
        Processor{
            gpregs: [gpr; general::GPREGS_COUNT],
            rflags: 0,
            rip: 0xfff0
        }
    }
 
    pub fn dump(&self) {
        println!("rip : {:x}", self.rip);
        println!("rflags : {:x}", self.rflags);
        unsafe {
            println!("rax : {:x}", self.gpregs[general::GpReg64::RAX as usize].r);
        }
    }
}