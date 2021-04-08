pub mod ip;
pub mod general;
pub mod rflags;
pub mod segment;
pub mod descriptor;

use general::*;
use segment::*;

#[derive(Clone)]
pub struct Processor {
    pub rip: ip::InstructionPointer,
    pub gpregs: general::GpRegisters,
    pub sgregs: segment::SgRegisters,
    pub rflags: rflags::RFlags,
}

impl Processor {
    pub fn new() -> Self {
        Processor{
            rip: ip::InstructionPointer::new(0xfff0),
            gpregs: general::GpRegisters::new(),
            sgregs: segment::SgRegisters::new(),
            rflags: Default::default(),
        }
    }

    pub fn dump(&self) -> () {
        let gpreg_name = ["RAX", "RCX", "RDX", "RBX", "RSP", "RBP", "RSI", "RDI", "R8 ", "R9 ", "R10", "R11", "R12", "R13", "R14", "R15"];
        let sgreg_name = ["ES", "CS", "SS", "DS", "FS", "GS"];
        let dtreg_name = ["GDTR", "IDTR", "LDTR", " TR "];

        println!("Registers Dump");
        println!("RIP : 0x{:016x}", self.rip.get());
        for i in 0..gpreg_name.len() {
            println!("{} : 0x{:016x}", gpreg_name[i], self.gpregs.get(GpReg64::from(i)));
        }
        println!("{:?}", self.rflags);

        for i in 0..sgreg_name.len() {
            println!("{} : {:?},  {:?}", sgreg_name[i], self.sgregs.selector(SgReg::from(i)), self.sgregs.cache(SgReg::from(i)));
        }

        println!("");
    }
 
    #[cfg(test)]
    pub fn test(&mut self) -> () {
        self.gpregs.set(GpReg64::from(0), 0xdeadbeefcafebabe);
        self.gpregs.set(GpReg32::EAX, 0x11223344);
        self.gpregs.set(GpReg8::AH, 0x00);
        self.gpregs.update(GpReg64::RAX, -0x10);
        assert_eq!(self.gpregs.get(GpReg64::RAX), 0xdeadbeef11220034);

        // self.gpregs.set(GpReg8l::DIL, 0xff);
        // assert_eq!(self.gpregs.get(GpReg64::RDI), 0xff);

        self.rflags.from_u64(0xdeadbeef);
        self.sgregs.selector_mut(SgReg::ES).from_u16(0x114);
    }
}
