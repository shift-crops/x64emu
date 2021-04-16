mod basic;
mod flag;
mod reg_mem;
mod desc;

use super::parse;
use crate::emulator::*;
use crate::emulator::EmuException;
use crate::hardware::processor;
use crate::hardware::processor::segment;
use crate::hardware::processor::segment::*;

pub struct Exec<'a> {
    pub ac: &'a mut access::Access,
    pub idata: &'a parse::InstrData,
    segment: Option<segment::SgReg>,
    rep: Option<parse::Rep>
}

impl<'a> Exec<'a> {
    pub fn new(ac: &'a mut access::Access, idata: &'a parse::InstrData, segment: Option<segment::SgReg>, rep: Option<parse::Rep>) -> Self {
        Self { ac, idata, segment, rep, }
    }

    fn update_cpumode(&mut self) -> Result<(), EmuException> {
        let core = &mut self.ac.core;
        let efer = &core.msr.efer;
        let cs = &core.sgregs.get(SgReg::CS).cache;

        core.mode = match (efer.LMA, cs.L, cs.DB) {
            (0, _, 0) => { processor::CpuMode::Real },
            (0, _, 1) => { processor::CpuMode::Protected },
            (1, 0, 0) => { processor::CpuMode::LongCompat16 },
            (1, 0, 1) => { processor::CpuMode::LongCompat32 },
            (1, 1, 0) => { processor::CpuMode::Long64 },
            _ => { return Err(EmuException::CPUException(CPUException::GP)); },
        };

        Ok(())
    }
}

pub trait IpAccess<T, U> {
    fn get_ip(&self) -> Result<T, EmuException>;
    fn set_ip(&mut self, v: T) -> Result<(), EmuException>;
    fn update_ip(&mut self, v: U) -> Result<(), EmuException>;
}

impl<'a> IpAccess<u16, i16> for Exec<'a> {
    fn get_ip(&self) -> Result<u16, EmuException> { Ok(self.ac.core.ip.get_ip()) }
    fn set_ip(&mut self, v: u16) -> Result<(), EmuException> { self.ac.core.ip.set_ip(v); Ok(()) }
    fn update_ip(&mut self, v: i16) -> Result<(), EmuException> { self.ac.core.ip.update_ip(v); Ok(()) }
}

impl<'a> IpAccess<u32, i32> for Exec<'a> {
    fn get_ip(&self) -> Result<u32, EmuException> { Ok(self.ac.core.ip.get_eip()) }
    fn set_ip(&mut self, v: u32) -> Result<(), EmuException> { self.ac.core.ip.set_eip(v); Ok(()) }
    fn update_ip(&mut self, v: i32) -> Result<(), EmuException> { self.ac.core.ip.update_eip(v); Ok(()) }
}

impl<'a> IpAccess<u64, i64> for Exec<'a> {
    fn get_ip(&self) -> Result<u64, EmuException> { Ok(self.ac.core.ip.get_rip()) }
    fn set_ip(&mut self, v: u64) -> Result<(), EmuException> { self.ac.core.ip.set_rip(v); Ok(()) }
    fn update_ip(&mut self, v: i64) -> Result<(), EmuException> { self.ac.core.ip.update_rip(v); Ok(()) }
}

#[cfg(test)]
#[test]
pub fn exec_test() {
    use crate::hardware;
    use crate::hardware::processor::general::*;

    let hw = hardware::Hardware::new(0, 0x1000);

    let mut ac = super::access::Access::new(hw);
    let idata: parse::InstrData = Default::default();

    let mut exe = Exec::new(&mut ac, &idata, None, None);
    exe.ac.core.gpregs.set(GpReg64::RSP, 0xf20);
    exe.push_u64(0xdeadbeef).unwrap();
    exe.push_u64(0xcafebabe).unwrap();
    assert_eq!(exe.pop_u64().unwrap(), 0xcafebabe);
    assert_eq!(exe.pop_u64().unwrap(), 0xdeadbeef);

    let mut x = exe.ac.mem.as_mut_ptr(0xf20).unwrap() as *mut u64;
    unsafe {
        *x = 0x11223344;
        x = (x as usize + 8) as *mut u64;
        *x = 0x55667788;
    }
    assert_eq!(exe.pop_u64().unwrap(), 0x11223344);
    assert_eq!(exe.pop_u64().unwrap(), 0x55667788);
}
