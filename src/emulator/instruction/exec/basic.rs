use std::convert::TryFrom;
use crate::emulator::EmuException;
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

    pub fn get_opr8(&self) -> Result<u8, EmuException> {
        let opr = (self.idata.opcode&0x7) as usize;
        let r = GpReg8::try_from(opr).unwrap();
        self.ac.get_gpreg(r)
    }

    pub fn set_opr8(&mut self, v: u8) -> Result<(), EmuException> {
        let opr = (self.idata.opcode&0x7) as usize;
        let r = GpReg8::try_from(opr).unwrap();
        self.ac.set_gpreg(r, v)
    }

    pub fn get_opr16(&self) -> Result<u16, EmuException> {
        let opr = (self.idata.opcode&0x7) as usize;
        let r = GpReg16::try_from(opr).unwrap();
        self.ac.get_gpreg(r)
    }

    pub fn set_opr16(&mut self, v: u16) -> Result<(), EmuException> {
        let opr = (self.idata.opcode&0x7) as usize;
        let r = GpReg16::try_from(opr).unwrap();
        self.ac.set_gpreg(r, v)
    }

    pub fn get_opr32(&self) -> Result<u32, EmuException> {
        let opr = (self.idata.opcode&0x7) as usize;
        let r = GpReg32::try_from(opr).unwrap();
        self.ac.get_gpreg(r)
    }

    pub fn set_opr32(&mut self, v: u32) -> Result<(), EmuException> {
        let opr = (self.idata.opcode&0x7) as usize;
        let r = GpReg32::try_from(opr).unwrap();
        self.ac.set_gpreg(r, v)
    }

    pub fn get_opr64(&self) -> Result<u64, EmuException> {
        let opr = (self.idata.opcode&0x7) as usize;
        let r = GpReg64::try_from(opr).unwrap();
        self.ac.get_gpreg(r)
    }

    pub fn set_opr64(&mut self, v: u64) -> Result<(), EmuException> {
        let opr = (self.idata.opcode&0x7) as usize;
        let r = GpReg64::try_from(opr).unwrap();
        self.ac.set_gpreg(r, v)
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

    pub fn set_sgr(&mut self, v: u16) -> Result<(), EmuException> {
        let sreg = SgReg::try_from(self.idata.modrm.reg as usize).unwrap();
        self.set_segment(sreg, v)?;
        if sreg == SgReg::CS { self.update_opadsize()?; }
        Ok(())
    }

    pub fn get_sgr(&mut self) -> Result<u16, EmuException> {
        self.get_segment(SgReg::try_from(self.idata.modrm.reg as usize).unwrap())
    }
}