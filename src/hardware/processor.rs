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
            println!("{} : 0x{:016x}", gpreg_name[i], self.gpregs.get(GpReg64::from(i)));
        }
        println!("{:?}", self.rflags);

        for i in 0..sgreg_name.len() {
            println!("{} : {:?},  {:?}", sgreg_name[i], self.sgregs.selector(SgReg::from(i)), self.sgregs.cache(SgReg::from(i)));
        }

        println!("");
    }
}

#[cfg(test)]
#[test]
pub fn core_test() {
    let core = Processor::new();
    let (gpregs, rflags, sgregs) = &mut (core.gpregs, core.rflags, core.sgregs);

    gpregs.set(GpReg64::from(0), 0xdeadbeefcafebabe);
    gpregs.set(GpReg32::EAX, 0x11223344);
    gpregs.set(GpReg8::AH, 0x00);
    gpregs.update(GpReg64::RAX, -0x10);
    assert_eq!(gpregs.get(GpReg64::RAX), 0xdeadbeef11220034);

    gpregs.set(GpReg32::EDI, 0xc0bebeef);
    gpregs.set(GpReg8x::DIL, 0xff);
    assert_eq!(gpregs.get(GpReg64::RDI), 0xc0bebeff);

    rflags.from_u64(0);
    assert_eq!(rflags.to_u64(), 2);
    rflags.set_carry(true);
    assert_eq!(rflags.to_u64(), 3);

    sgregs.selector_mut(SgReg::ES).from_u16(0x2e);
    let es = sgregs.selector(SgReg::ES);
    assert_eq!(es.IDX, 5);
    assert_eq!(es.TI, 1);
    assert_eq!(es.RPL, 2);
}