pub mod ip;
pub mod general;
pub mod rflags;
pub mod segment;
pub mod control;
pub mod descriptor;

use std::convert::TryFrom;
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
        Self {
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
            println!("{} : 0x{:016x}", gpreg_name[i], self.gpregs.get(GpReg64::try_from(i).unwrap()));
        }
        println!("{:?}", self.rflags);

        for i in 0..sgreg_name.len() {
            println!("{} : {:?},  {:?}", sgreg_name[i], self.sgregs.selector(SgReg::try_from(i).unwrap()), self.sgregs.cache(SgReg::try_from(i).unwrap()));
        }

        println!("");
    }
}