pub mod ip;
pub mod general;
pub mod rflags;
pub mod segment;
pub mod control;
pub mod descriptor;
pub mod model_specific;

use std::convert::TryFrom;
use general::*;
use segment::*;

pub struct Processor {
    pub ip: ip::InstructionPointer,
    pub gpregs: general::GpRegisters,
    pub rflags: rflags::RFlags,
    pub cregs:  control::CRegisters,
    pub sgregs: segment::SgRegisters,
    pub dtregs: descriptor::DTRegisters, 
    pub msr: model_specific::ModelSpecific,
}

impl Processor {
    pub fn new(rst_vct: u64) -> Self {
        Self {
            ip: ip::InstructionPointer::new(rst_vct),
            gpregs: general::GpRegisters::new(),
            rflags: Default::default(),
            cregs:  Default::default(),
            sgregs: segment::SgRegisters::new(),
            dtregs: Default::default(),
            msr: Default::default(),
        }
    }

    pub fn dump(&self) -> () {
        println!("Registers Dump");

        let efer = &self.msr.efer;
        let cs = &self.sgregs.get(SgReg::CS).cache;

        match (efer.LMA, cs.L, cs.DB) {
            (1, 0, 0) | (0, _, 0) => {  // 16 bit
                println!("IP : 0x{:04x}", self.ip.get_ip());

                let gpreg_name = ["AX", "CX", "DX", "BX", "SP", "BP", "SI", "DI"];
                for i in 0..gpreg_name.len() {
                    println!("{} : 0x{:04x}", gpreg_name[i], self.gpregs.get16(GpReg16::try_from(i).unwrap()));
                }
            },
            (1, 0, 1) | (0, _, 1) => {  // 32 bit
                println!("EIP : 0x{:08x}", self.ip.get_eip());

                let gpreg_name = ["EAX", "ECX", "EDX", "EBX", "ESP", "EBP", "ESI", "EDI"];
                for i in 0..gpreg_name.len() {
                    println!("{} : 0x{:08x}", gpreg_name[i], self.gpregs.get32(GpReg32::try_from(i).unwrap()));
                }
            },
            (1, 1, 0) => {              // 64 bit
                println!("RIP : 0x{:016x}", self.ip.get_rip());

                let gpreg_name = ["RAX", "RCX", "RDX", "RBX", "RSP", "RBP", "RSI", "RDI", "R8 ", "R9 ", "R10", "R11", "R12", "R13", "R14", "R15"];
                for i in 0..gpreg_name.len() {
                    println!("{} : 0x{:016x}", gpreg_name[i], self.gpregs.get64(GpReg64::try_from(i).unwrap()));
                }
            },
            _ => { },
        }
        println!("{:?}\n", self.rflags);

        let sgreg_name = ["ES", "CS", "SS", "DS", "FS", "GS"];
        for i in 0..sgreg_name.len() {
            let sgreg = self.sgregs.get(SgReg::try_from(i).unwrap());
            println!("{} : {:?},  {:x?}", sgreg_name[i], sgreg.selector, sgreg.cache);
        }
        println!("");

        println!("GDTR : {:x?}", self.dtregs.gdtr);
        println!("IDTR : {:x?}", self.dtregs.idtr);
        println!("LDTR : {:x?}", self.dtregs.ldtr);
        println!("TR   : {:x?}", self.dtregs.tr);

        println!("");
    }
}