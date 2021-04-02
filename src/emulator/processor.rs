use general::*;

mod general;
mod rflags;
mod segment;
mod descriptor;

pub struct Processor {
    rip: u64,
    gpregs: general::GpRegisters,
    sgregs: segment::SgRegisters,
    rflags: rflags::RFlags,
}

impl Processor {
    pub fn new() -> Processor {
        Processor{
            rip: 0xfff0,
            gpregs: general::GpRegisters::new(),
            sgregs: segment::SgRegisters::new(),
            rflags: rflags::RFlags::new(),
        }
    }
 
    pub fn test(&mut self){
        self.gpregs.set(GpReg64::from(0), 0xdeadbeefcafebabe);
        self.gpregs.set(GpReg32::EAX, 0x11223344);
        self.gpregs.set(GpReg8h::AH, 0x00);

        self.gpregs.set(GpReg8l::DIL, 0xff);

        self.rflags = rflags::RFlags::from_u64(0xdeadbeef);
        self.sgregs.regs[0].selector.IDX = 1145;
    }

    pub fn dump(&self) {
        let gpreg_name = ["RAX", "RCX", "RDX", "RBX", "RSP", "RBP", "RSI", "RDI"];
        let sgreg_name = ["ES", "CS", "SS", "DS", "FS", "GS"];
        let dtreg_name = ["GDTR", "IDTR", "LDTR", " TR "];

        println!("RIP :  0x{:016x}", self.rip);
        for i in 0..gpreg_name.len() {
            println!("{} :  0x{:016x}", gpreg_name[i], self.gpregs.get(GpReg64::from(i)));
        }
        println!("{:?}", self.rflags);

        for i in 0..sgreg_name.len() {
            println!("{} :  {:?}", sgreg_name[i], self.sgregs.regs[i]);
        }
    }
}