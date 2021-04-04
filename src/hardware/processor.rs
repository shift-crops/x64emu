pub mod ip;
pub mod general;
pub mod rflags;
pub mod segment;
pub mod descriptor;

use general::*;
use segment::*;

#[derive(Clone)]
pub struct Processor {
    rip: ip::InstructionPointer,
    gpregs: general::GpRegisters,
    sgregs: segment::SgRegisters,
    rflags: rflags::RFlags,
}

impl Processor {
    pub fn new() -> Self {
        Processor{
            rip: ip::InstructionPointer::new(0xfff0),
            gpregs: general::GpRegisters::new(),
            sgregs: segment::SgRegisters::new(),
            rflags: rflags::RFlags::new(),
        }
    }

    pub fn rip(&self) -> ip::InstructionPointer { self.rip }
    pub fn gpregs(&self) -> general::GpRegisters { self.gpregs }
    pub fn sgregs(&self) -> segment::SgRegisters { self.sgregs }
    pub fn rflags(&self) -> rflags::RFlags { self.rflags }

    pub fn rip_mut(&mut self) -> &mut ip::InstructionPointer { &mut self.rip }
    pub fn gpregs_mut(&mut self) -> &mut general::GpRegisters { &mut self.gpregs }
    pub fn sgregs_mut(&mut self) -> &mut segment::SgRegisters { &mut self.sgregs }
    pub fn rflags_mut(&mut self) -> &mut rflags::RFlags { &mut self.rflags }

    pub fn dump(&self) -> () {
        let gpreg_name = ["RAX", "RCX", "RDX", "RBX", "RSP", "RBP", "RSI", "RDI"];
        let sgreg_name = ["ES", "CS", "SS", "DS", "FS", "GS"];
        let dtreg_name = ["GDTR", "IDTR", "LDTR", " TR "];

        println!("Registers Dump");
        println!("RIP : 0x{:016x}", self.rip.get());
        for i in 0..gpreg_name.len() {
            println!("{} : 0x{:016x}", gpreg_name[i], self.gpregs.get(GpReg64::from(i)));
        }
        println!("{:?}", self.rflags);

        for i in 0..sgreg_name.len() {
            println!("{} : {:?}", sgreg_name[i], self.sgregs.get(SgReg::from(i)));
        }

        println!("");
    }
 
    pub fn test(&mut self) -> () {
        self.gpregs.set(GpReg64::from(0), 0xdeadbeefcafebabe);
        self.gpregs.set(GpReg32::EAX, 0x11223344);
        self.gpregs.set(GpReg8l::AL, 0x00);
        self.gpregs.update(GpReg64::RAX, -1);

        self.gpregs.set(GpReg8l::DIL, 0xff);

        self.rflags.from_u64(0xdeadbeef);
        self.sgregs.set_sel(SgReg::ES, 0x114);
    }
}
