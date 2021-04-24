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


    pub fn push_u16(&mut self, v: u16) -> Result<(), EmuException> {
        self.ac.update_gpreg(GpReg16::SP, -2)?;
        let sp = self.ac.get_gpreg(GpReg16::SP)? as u64;
        self.ac.set_data16((SgReg::SS, sp), v)
    }

    pub fn pop_u16(&mut self) -> Result<u16, EmuException> {
        let sp = self.ac.get_gpreg(GpReg16::SP)? as u64;
        self.ac.update_gpreg(GpReg16::SP, 2)?;
        self.ac.get_data16((SgReg::SS, sp))
    }

    pub fn push_u32(&mut self, v: u32) -> Result<(), EmuException> {
        self.ac.update_gpreg(GpReg32::ESP, -4)?;
        let esp = self.ac.get_gpreg(GpReg32::ESP)? as u64;
        self.ac.set_data32((SgReg::SS, esp), v)
    }

    pub fn pop_u32(&mut self) -> Result<u32, EmuException> {
        let esp = self.ac.get_gpreg(GpReg32::ESP)? as u64;
        self.ac.update_gpreg(GpReg32::ESP, 4)?;
        self.ac.get_data32((SgReg::SS, esp))
    }

    pub fn push_u64(&mut self, v: u64) -> Result<(), EmuException> {
        self.ac.update_gpreg(GpReg64::RSP, -8)?;
        let rsp = self.ac.get_gpreg(GpReg64::RSP)?;
        self.ac.set_data64((SgReg::SS, rsp), v)
    }

    pub fn pop_u64(&mut self) -> Result<u64, EmuException> {
        let rsp = self.ac.get_gpreg(GpReg64::RSP)?;
        self.ac.update_gpreg(GpReg64::RSP, 8)?;
        self.ac.get_data64((SgReg::SS, rsp))
    }

    pub fn get_sreg(&mut self) -> Result<u16, EmuException> {
        Ok(self.ac.get_sgselector(SgReg::try_from(self.idata.modrm.reg as usize).unwrap())?.to_u16())
    }

    pub fn set_sreg(&mut self, v: u16) -> Result<(), EmuException> {
        let sreg = SgReg::try_from(self.idata.modrm.reg as usize).unwrap();
        self.mov_to_sreg(sreg, v)
    }
}