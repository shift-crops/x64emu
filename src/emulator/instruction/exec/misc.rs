use std::convert::TryFrom;
use crate::emulator::*;
use crate::emulator::access::register::*;
use crate::emulator::instruction::parse::Rep;

macro_rules! repeat_reg {
    ( $name:ident, $reg:ty ) => { paste::item! {
        pub fn [<repeat_ $name>](&mut self) -> Result<bool, EmuException> {
            let mut result = false;
            if let Some(rep) = &self.pdata.repeat {
                self.ac.update_gpreg($reg, -1)?;
                result = match (self.ac.get_gpreg($reg)?, rep, self.ac.core.rflags.is_zero()) {
                (0, _, _) | (_, Rep::REPZ, false) | (_, Rep::REPNZ, true) => false,
                _ => true,
                }
            }
            Ok(result)
        }
    } };
}

impl<'a> super::Exec<'a> {
    pub fn cr_to_r32(&mut self) -> Result<(), EmuException> {
        let cr = self.ac.get_cregs(self.idata.modrm.reg as usize)?;
        self.ac.set_gpreg(GpReg32::try_from(self.idata.modrm.rm as usize).unwrap(), cr)
    }

    pub fn cr_from_r32(&mut self) -> Result<(), EmuException> {
        let r = self.idata.modrm.reg as usize;
        let v = self.ac.get_gpreg(GpReg32::try_from(self.idata.modrm.rm as usize).unwrap())?;
        self.ac.set_cregs(r, v)?;
        if r == 0 { self.update_cpumode()?; }
        Ok(())
    }

    pub fn select_segment(&self, def: SgReg) -> Result<SgReg, EmuException> {
        Ok(self.pdata.segment.unwrap_or(def))
    }

    repeat_reg!(cx,  GpReg16::CX);
    repeat_reg!(ecx, GpReg32::ECX);
    repeat_reg!(rcx, GpReg64::RCX);
}