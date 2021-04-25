pub mod register;
mod memory;
mod msr;
pub mod descriptor;

use crate::hardware;

#[derive(Debug, PartialEq)]
pub enum CpuMode { Real, Protected, Long }

#[derive(Clone, Copy, PartialEq)]
pub enum AcsSize { BIT16, BIT32, BIT64 }
impl Default for AcsSize {
    fn default() -> Self {
        AcsSize::BIT16
    }
}

#[derive(Default)]
pub struct OpAdSize {
    pub op: AcsSize,
    pub ad: AcsSize,
}

pub struct Access {
    pub mode: CpuMode,
    pub size: OpAdSize,
    pub stsz: AcsSize,
    pub core: hardware::processor::Processor,
    pub mem: hardware::memory::Memory,
    a20gate: bool,
}

impl Access {
    pub fn new(hw: hardware::Hardware) -> Self {
        Self {
            mode: CpuMode::Real,
            size: Default::default(),
            stsz: Default::default(),
            core: hw.core,
            mem: hw.mem,
            a20gate: false,
        }
    }

    pub fn dump(&self) -> () {
        println!("CPU Mode: {:?} mode\n", self.mode);
        self.core.dump();
        self.dump_code();
        self.dump_stack();
    }
}