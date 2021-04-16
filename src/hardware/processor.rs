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

#[derive(Clone, Copy, PartialEq)]
pub enum CpuMode { Real, Protected, LongCompat16, LongCompat32, Long64 }

pub struct Processor {
    pub mode: CpuMode,
    pub ip: ip::InstructionPointer,
    pub gpregs: general::GpRegisters,
    pub sgregs: segment::SgRegisters,
    pub rflags: rflags::RFlags,
    pub dtregs: descriptor::DTRegisters, 
    pub msr: model_specific::ModelSpecific,
}

impl Processor {
    pub fn new(rst_vct: u64) -> Self {
        Self {
            mode: CpuMode::Real,
            ip: ip::InstructionPointer::new(rst_vct),
            gpregs: general::GpRegisters::new(),
            sgregs: segment::SgRegisters::new(),
            rflags: Default::default(),
            dtregs: Default::default(),
            msr: Default::default(),
        }
    }

    pub fn dump(&self) -> () {
        println!("Registers Dump");
        println!("IP : {:016x}", self.ip.get_rip());

        match self.mode {
            CpuMode::LongCompat16 | CpuMode::Real => {
                let gpreg_name = ["AX", "CX", "DX", "BX", "SP", "BP", "SI", "DI"];
                for i in 0..gpreg_name.len() {
                    println!("{} : 0x{:04x}", gpreg_name[i], self.gpregs.get(GpReg16::try_from(i).unwrap()));
                }
            }
            CpuMode::LongCompat32 | CpuMode::Protected => {
                let gpreg_name = ["EAX", "ECX", "EDX", "EBX", "ESP", "EBP", "ESI", "EDI"];
                for i in 0..gpreg_name.len() {
                    println!("{} : 0x{:08x}", gpreg_name[i], self.gpregs.get(GpReg32::try_from(i).unwrap()));
                }
            }
            CpuMode::Long64 => {
                let gpreg_name = ["RAX", "RCX", "RDX", "RBX", "RSP", "RBP", "RSI", "RDI", "R8 ", "R9 ", "R10", "R11", "R12", "R13", "R14", "R15"];
                for i in 0..gpreg_name.len() {
                    println!("{} : 0x{:016x}", gpreg_name[i], self.gpregs.get(GpReg64::try_from(i).unwrap()));
                }
            }
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