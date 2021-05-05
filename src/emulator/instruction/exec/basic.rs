use std::convert::TryFrom;
use crate::emulator::*;
use crate::emulator::access::register::*;

impl<'a> super::Exec<'a> {
    pub fn get_al(&self) -> Result<u8, EmuException> {
        self.ac.get_gpreg(GpReg8::AL)
    }

    pub fn set_al(&mut self, v: u8) -> Result<(), EmuException> {
        self.ac.set_gpreg(GpReg8::AL, v)
    }

    pub fn get_ax(&self) -> Result<u16, EmuException> {
        self.ac.get_gpreg(GpReg16::AX)
    }

    pub fn set_ax(&mut self, v: u16) -> Result<(), EmuException> {
        self.ac.set_gpreg(GpReg16::AX, v)
    }

    pub fn get_eax(&self) -> Result<u32, EmuException> {
        self.ac.get_gpreg(GpReg32::EAX)
    }

    pub fn set_eax(&mut self, v: u32) -> Result<(), EmuException> {
        self.ac.set_gpreg(GpReg32::EAX, v)
    }

    pub fn get_dx(&self) -> Result<u16, EmuException> {
        self.ac.get_gpreg(GpReg16::DX)
    }

    pub fn get_sreg(&mut self) -> Result<u16, EmuException> {
        Ok(self.ac.get_sgselector(SgReg::try_from(self.idata.modrm.reg as usize).unwrap())?.to_u16())
    }

    pub fn set_sreg(&mut self, v: u16) -> Result<(), EmuException> {
        let sreg = SgReg::try_from(self.idata.modrm.reg as usize).unwrap();
        self.mov_to_sreg(sreg, v)
    }
}