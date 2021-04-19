#![allow(non_camel_case_types)]

use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use crate::emulator::*;
use crate::hardware::processor::model_specific::*;
use super::register::*;

#[derive(TryFromPrimitive)] #[repr(u32)]
pub enum MSRAddress {
    IA32_EFER    = 0xc0000080,
    STAR         = 0xc0000081,
    CSTAR        = 0xc0000082,
    LSTAR        = 0xc0000083,
    FMASK        = 0xc0000084,
    FSBase       = 0xc0000100,
    GSBase       = 0xc0000101,
    KernelGSBase = 0xc0000102,
}

impl super::Access {
    pub fn read_msr(&self, addr: u32) -> Result<u64, EmuException> {
        if let Some(msr) = self.get_msr(addr) {
            return Ok(msr.get());
        }
        Err(EmuException::CPUException(CPUException::GP))
    }

    pub fn write_msr(&mut self, addr: u32, val: u64) -> Result<(), EmuException> {
        if let Some(msr) = self.get_mut_msr(addr) {
            msr.set(val);
            return Ok(());
        }
        Err(EmuException::CPUException(CPUException::GP))
    }

    fn get_msr(&self, addr: u32) -> Option<Box<&dyn MSRAccess>> {
        if let Ok(ad) = MSRAddress::try_from(addr) {
            let v: Box<&dyn MSRAccess> = match ad {
                MSRAddress::IA32_EFER    => Box::new(&self.core.msr.efer),
                MSRAddress::STAR         => Box::new(&self.core.msr.star),
                MSRAddress::CSTAR        => Box::new(&self.core.msr.cstar),
                MSRAddress::LSTAR        => Box::new(&self.core.msr.lstar),
                MSRAddress::FMASK        => Box::new(&self.core.msr.fmask),
                MSRAddress::FSBase       => Box::new(self.get_sgcache(SgReg::FS).unwrap()),
                MSRAddress::GSBase       => Box::new(self.get_sgcache(SgReg::GS).unwrap()),
                MSRAddress::KernelGSBase => Box::new(self.get_sgcache(SgReg::KernelGS).unwrap()),
            };
            return Some(v);
        }
        None
    }

    fn get_mut_msr(&mut self, addr: u32) -> Option<Box<&mut dyn MSRAccess>> {
        if let Ok(ad) = MSRAddress::try_from(addr) {
            let v: Box<&mut dyn MSRAccess> = match ad {
                MSRAddress::IA32_EFER    => Box::new(&mut self.core.msr.efer),
                MSRAddress::STAR         => Box::new(&mut self.core.msr.star),
                MSRAddress::CSTAR        => Box::new(&mut self.core.msr.cstar),
                MSRAddress::LSTAR        => Box::new(&mut self.core.msr.lstar),
                MSRAddress::FMASK        => Box::new(&mut self.core.msr.fmask),
                MSRAddress::FSBase       => Box::new(self.get_sgcache_mut(SgReg::FS).unwrap()),
                MSRAddress::GSBase       => Box::new(self.get_sgcache_mut(SgReg::GS).unwrap()),
                MSRAddress::KernelGSBase => Box::new(self.get_sgcache_mut(SgReg::KernelGS).unwrap()),
            };
            return Some(v);
        }
        None
    }
}

#[cfg(test)]
#[test]
pub fn access_msr_test() {
    let hw = hardware::Hardware::new(0, 0x1000);
    let mut ac = access::Access::new(hw);

    ac.core.msr.efer.LMA = 1;
    assert_eq!(ac.read_msr(MSRAddress::IA32_EFER as u32).unwrap(), 0x400);

    ac.write_msr(0xc0000100, 0xdeadbeef).unwrap();
    assert_eq!(ac.core.sgregs.get(SgReg::FS).cache.base, 0xdeadbeef);
}

#[cfg(test)]
#[test]
#[should_panic]
pub fn access_msr_test_panic() {
    let hw = hardware::Hardware::new(0, 0x1000);
    let mut ac = access::Access::new(hw);

    ac.write_msr(0xc0000103, 0xdeadbeef).unwrap();
}