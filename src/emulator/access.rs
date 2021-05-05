pub mod register;
mod memory;
mod msr;
pub mod descriptor;
mod port;

use std::sync::{Arc, RwLock};
use crate::hardware;
use crate::device;

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
    pub oasz: OpAdSize,
    pub stsz: AcsSize,
    pub core: hardware::processor::Processor,
    pub mem: Arc<RwLock<hardware::memory::Memory>>,
    pub dev: device::Device,
    a20gate: bool,
}

impl Access {
    pub fn new(hw: hardware::Hardware, dev: device::Device) -> Self {
        Self {
            mode: CpuMode::Real,
            oasz: Default::default(),
            stsz: Default::default(),
            core: hw.core,
            mem: hw.mem,
            dev,
            a20gate: false,
        }
    }

    pub fn check_irq(&self, block: bool) -> Option<u8> {
        if self.core.rflags.is_interrupt() {
            self.dev.get_interrupt_req(block)
        } else {
            None
        }
    }

    pub fn dump(&self) -> () {
        println!("CPU Mode: {:?} mode\n", self.mode);
        self.core.dump();

        let unit = match self.oasz.ad {
            AcsSize::BIT16 => hardware::memory::MemDumpSize::Word,
            AcsSize::BIT32 => hardware::memory::MemDumpSize::DWord,
            AcsSize::BIT64 => hardware::memory::MemDumpSize::QWord
        };
        self.dump_code(unit);
        self.dump_stack(unit);
    }
}