pub mod register;
mod memory;
mod msr;
pub mod descriptor;
mod port;

use std::sync::{Arc, RwLock};
use crate::hardware;
use crate::device;
use crate::emulator::{EmuException, CPUException};

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

#[derive(Debug, PartialEq)]
pub enum PagingMode { Legacy, LegacyPAE, Ia32e4Lv, Ia32e5Lv }

pub struct Access {
    pub mode: CpuMode,
    pub oasz: OpAdSize,
    pub stsz: AcsSize,
    pub pgmd: Option<PagingMode>,
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
            pgmd: None,
            core: hw.core,
            mem: hw.mem,
            dev,
            a20gate: false,
        }
    }

    pub fn update_cpumode(&mut self) -> Result<(), EmuException> {
        let efer = &self.core.msr.efer;
        let cr0 = &self.core.cregs.0;

        self.mode = match (efer.LME, cr0.PE) {
            (0, 0) => CpuMode::Real,
            (0, 1) => CpuMode::Protected,
            (1, 1) => CpuMode::Long,
            _ => return Err(EmuException::CPUException(CPUException::GP)),
        };

        Ok(())
    }

    pub fn update_opadsize(&mut self) -> Result<(), EmuException> {
        let efer = &self.core.msr.efer;
        let cs = &self.core.sgregs.get(register::SgReg::CS).cache;

        let (op, ad) = match (efer.LMA, cs.L, cs.DB) {
            (1, 0, 0) | (0, _, 0) => (AcsSize::BIT16, AcsSize::BIT16),
            (1, 0, 1) | (0, _, 1) => (AcsSize::BIT32, AcsSize::BIT32),
            (1, 1, 0)             => (AcsSize::BIT32, AcsSize::BIT64),
            _ => return Err(EmuException::CPUException(CPUException::GP)),
        };
        self.oasz = OpAdSize { op, ad };
        Ok(())
    }

    pub fn update_stacksize(&mut self) -> Result<(), EmuException> {
        let ss = &self.core.sgregs.get(register::SgReg::SS).cache;

        self.stsz = match (ss.L, ss.DB) {
            (0, 0) => AcsSize::BIT16,
            (0, 1) => AcsSize::BIT32,
            (1, 0) => AcsSize::BIT64,
            _ => return Err(EmuException::CPUException(CPUException::SS)),
        };
        Ok(())
    }

    pub fn update_pgmode(&mut self) -> Result<(), EmuException> {
        let efer = &mut self.core.msr.efer;
        let cr0 = &self.core.cregs.0;
        let cr4 = &self.core.cregs.4;

        if cr4.PAE == 0 && efer.LMA == 1 {
            return Err(EmuException::CPUException(CPUException::GP));
        }

        self.pgmd = match (&self.mode, cr0.PG, cr4.PAE, cr4.LA57) {
            (CpuMode::Real, _, _, _) | (_, 0, _, _) => None,
            (CpuMode::Protected, 1, 0, _)           => Some(PagingMode::Legacy),
            (CpuMode::Protected, 1, 1, _)           => Some(PagingMode::LegacyPAE),
            (CpuMode::Long, 1, 1, 0)                => Some(PagingMode::Ia32e4Lv),
            (CpuMode::Long, 1, 1, 1)                => Some(PagingMode::Ia32e5Lv),
            _ => return Err(EmuException::CPUException(CPUException::GP)),
        };
        efer.LMA = if let Some(PagingMode::Ia32e4Lv) | Some(PagingMode::Ia32e5Lv) = &self.pgmd { 1 } else { 0 };

        Ok(())
    }

    pub fn test_cpumode(&self, mode: CpuMode) -> bool {
        self.mode == mode
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