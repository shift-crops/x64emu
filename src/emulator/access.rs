pub(super) mod register;
mod memory;
mod msr;
pub(super) mod descriptor;
mod port;

use std::cell::RefCell;
use std::sync::{Arc, RwLock};
use crate::hardware;
use crate::device;
use crate::emulator::{EmuException, CPUException};

#[derive(Debug, PartialEq)]
pub(super) enum CpuMode { Real, Protected, Long }

#[derive(Clone, Copy, PartialEq)]
pub enum AcsSize { BIT16, BIT32, BIT64 }
impl Default for AcsSize {
    fn default() -> Self {
        AcsSize::BIT16
    }
}

#[derive(Default)]
pub(super) struct OpAdSize {
    pub op: AcsSize,
    pub ad: AcsSize,
}

#[derive(Debug)]
enum PagingMode { Legacy, LegacyPAE, Ia32e4Lv, Ia32e5Lv }

pub struct Access {
    pub core: hardware::processor::Processor,
    pub mem: Arc<RwLock<hardware::memory::Memory>>,
    dev: device::Device,
    pub(super) mode: CpuMode,
    pub(super) oasz: OpAdSize,
    stsz: AcsSize,
    pgmd: Option<PagingMode>,
    tlb: RefCell<memory::TLB>,
    a20gate: bool,
}

impl Access {
    pub(super) fn new(hw: hardware::Hardware, dev: device::Device) -> Self {
        Self {
            core: hw.core,
            mem: hw.mem,
            dev,
            mode: CpuMode::Real,
            oasz: Default::default(),
            stsz: Default::default(),
            pgmd: None,
            tlb: Default::default(),
            a20gate: false,
        }
    }

    pub(super) fn update_cpumode(&mut self) -> Result<(), EmuException> {
        let efer = &self.core.msr.efer;
        let cr0 = &self.core.cregs.0;

        self.mode = match (efer.LME, cr0.PE) {
            (0, 0) => CpuMode::Real,
            (0, 1) => CpuMode::Protected,
            (1, 1) => CpuMode::Long,
            _ => return Err(EmuException::CPUException(CPUException::GP(None))),
        };

        Ok(())
    }

    pub(super) fn update_opadsize(&mut self) -> Result<(), EmuException> {
        let efer = &self.core.msr.efer;
        let cs = &self.core.sgregs.get(register::SgReg::CS).cache;

        let (op, ad) = match (efer.LMA, cs.L, cs.DB) {
            (1, 0, 0) | (0, _, 0) => (AcsSize::BIT16, AcsSize::BIT16),
            (1, 0, 1) | (0, _, 1) => (AcsSize::BIT32, AcsSize::BIT32),
            (1, 1, 0)             => (AcsSize::BIT32, AcsSize::BIT64),
            _ => return Err(EmuException::CPUException(CPUException::GP(None))),
        };
        self.oasz = OpAdSize { op, ad };
        Ok(())
    }

    pub(super) fn update_stacksize(&mut self) -> Result<(), EmuException> {
        let ss = &self.core.sgregs.get(register::SgReg::SS);

        self.stsz = match (ss.cache.L, ss.cache.DB) {
            (0, 0) => AcsSize::BIT16,
            (0, 1) => AcsSize::BIT32,
            (1, 0) => AcsSize::BIT64,
            _ => return Err(EmuException::CPUException(CPUException::SS(Some(ss.selector.to_u16())))),
        };
        Ok(())
    }

    pub(super) fn update_pgmode(&mut self) -> Result<(), EmuException> {
        let efer = &mut self.core.msr.efer;
        let cr0 = &self.core.cregs.0;
        let cr4 = &self.core.cregs.4;

        if cr4.PAE == 0 && efer.LMA == 1 {
            return Err(EmuException::CPUException(CPUException::GP(None)));
        }

        self.pgmd = match (&self.mode, cr0.PG, cr4.PAE, cr4.LA57) {
            (CpuMode::Real, _, _, _) | (_, 0, _, _) => None,
            (CpuMode::Protected, 1, 0, _)           => Some(PagingMode::Legacy),
            (CpuMode::Protected, 1, 1, _)           => Some(PagingMode::LegacyPAE),
            (CpuMode::Long, 1, 1, 0)                => Some(PagingMode::Ia32e4Lv),
            (CpuMode::Long, 1, 1, 1)                => Some(PagingMode::Ia32e5Lv),
            _ => return Err(EmuException::CPUException(CPUException::GP(None))),
        };
        efer.LMA = if let Some(PagingMode::Ia32e4Lv) | Some(PagingMode::Ia32e5Lv) = &self.pgmd { 1 } else { 0 };

        Ok(())
    }

    pub(super) fn test_cpumode(&self, mode: CpuMode) -> bool {
        self.mode == mode
    }

    pub(super) fn check_irq(&self, block: bool) -> Option<u8> {
        if self.core.rflags.is_interrupt() {
            self.dev.get_interrupt_req(block)
        } else {
            None
        }
    }

    pub(super) fn dump(&self) -> () {
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