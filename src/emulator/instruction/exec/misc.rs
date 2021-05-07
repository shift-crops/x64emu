use std::convert::TryFrom;
use crate::emulator::*;
use crate::emulator::access::register::*;

impl<'a> super::Exec<'a> {
    pub fn cr_to_r32(&mut self) -> Result<(), EmuException> {
        let cr = self.ac.get_creg(self.idata.modrm.reg as usize)?;
        self.ac.set_gpreg(GpReg32::try_from(self.idata.modrm.rm as usize).unwrap(), cr)
    }

    pub fn cr_from_r32(&mut self) -> Result<(), EmuException> {
        let r = self.idata.modrm.reg as usize;
        let v = self.ac.get_gpreg(GpReg32::try_from(self.idata.modrm.rm as usize).unwrap())?;
        self.ac.set_creg(r, v)?;
        if r == 0 { self.update_cpumode()?; }
        Ok(())
    }
}