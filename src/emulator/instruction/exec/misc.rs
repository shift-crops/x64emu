use std::convert::TryFrom;
use crate::emulator::*;
use crate::emulator::access::register::*;

impl<'a> super::Exec<'a> {
    pub fn cr_to_reg(&mut self) -> Result<(), EmuException> {
        let cr = self.ac.get_creg(self.idata.modrm.reg as usize)?;
        self.ac.set_gpreg(GpReg32::try_from(self.idata.modrm.rm as usize).unwrap(), cr)
    }

    pub fn cr_from_reg(&mut self) -> Result<(), EmuException> {
        let r = self.idata.modrm.reg as usize;
        let v = self.ac.get_gpreg(GpReg32::try_from(self.idata.modrm.rm as usize).unwrap())?;
        self.ac.set_creg(r, v)?;

        match r {
            0 => {
                self.ac.update_cpumode()?;
                self.ac.update_pgmode()?;
            },
            4 => {
                self.ac.update_pgmode()?;
            },
            _ => {},
        }
        Ok(())
    }

    pub fn msr_to_reg(&mut self) -> Result<(), EmuException> {
        let addr = self.ac.get_gpreg(GpReg32::ECX)?;
        let v = self.ac.read_msr(addr)?;

        self.set_edx((v >> 32) as u32)?;
        self.set_eax(v as u32)?;
        Ok(())
    }

    pub fn msr_from_reg(&mut self) -> Result<(), EmuException> {
        let addr = self.ac.get_gpreg(GpReg32::ECX)?;
        let v = ((self.get_edx()? as u64) << 32) + self.get_eax()? as u64;

        self.ac.write_msr(addr, v)
    }

}