mod memory;
mod segment;

use crate::hardware;
use crate::hardware::processor::segment::*;
use crate::emulator::EmuException;

#[derive(Clone, Copy)]
pub enum CpuMode { Real, Protected, LongCompat16, LongCompat32, Long64 }

pub struct Access {
    pub mode: CpuMode,
    pub core: hardware::processor::Processor,
    pub mem: hardware::memory::Memory,
}

impl Access {
    pub fn new(hw: hardware::Hardware) -> Self {
        Self {
            mode: CpuMode::Real,
            core: hw.core,
            mem: hw.mem,
        }
    }

    fn update_cpumode(&mut self) -> Result<(), EmuException> {
        let efer = &self.core.msr.efer;
        let cs = &self.core.sgregs.cache(SgReg::CS);

        self.mode = match (efer.LMA, cs.L, cs.D) {
            (0, _, 0) => { CpuMode::Real },
            (0, _, 1) => { CpuMode::Protected },
            (1, 0, 0) => { CpuMode::LongCompat16 },
            (1, 0, 1) => { CpuMode::LongCompat32 },
            (1, 1, 0) => { CpuMode::Long64 },
            _ => { return Err(EmuException::UnexpectedError); },
        };

        Ok(())
    }

    pub fn dump(&self) -> () {
        use crate::hardware::processor::general::*;

        self.core.dump();
        self.mem.dump(self.core.ip.get_rip() as usize -0x10 , 0x20);
        self.mem.dump(self.core.gpregs.get(GpReg64::RSP) as usize, 0x40);
    }
}


#[cfg(test)]
#[test]
pub fn access_test() {
    let hw = hardware::Hardware::new(0, 0x1000);

    let mut ac = Access::new(hw);
    ac.set_data32((SgReg::DS, 0x10), 0xdeadbeef).unwrap();
    assert_eq!(ac.get_data8((SgReg::DS, 0x10)).unwrap(), 0xef);

    ac.set_data32((SgReg::DS, 0x1010), 0xdeadbeef).unwrap();
    assert_eq!(ac.get_data8((SgReg::DS, 0x1010)).unwrap(), 0);
}