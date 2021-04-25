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
        let sp = self.stack_addr(-2)?;
        self.ac.set_data16((SgReg::SS, sp), v)
    }

    pub fn pop_u16(&mut self) -> Result<u16, EmuException> {
        let sp = self.stack_addr(2)?;
        self.ac.get_data16((SgReg::SS, sp-2))
    }

    pub fn push_u32(&mut self, v: u32) -> Result<(), EmuException> {
        let esp = self.stack_addr(-4)?;
        self.ac.set_data32((SgReg::SS, esp), v)
    }

    pub fn pop_u32(&mut self) -> Result<u32, EmuException> {
        let esp = self.stack_addr(4)?;
        self.ac.get_data32((SgReg::SS, esp-4))
    }

    pub fn push_u64(&mut self, v: u64) -> Result<(), EmuException> {
        let rsp = self.stack_addr(-8)?;
        self.ac.set_data64((SgReg::SS, rsp), v)
    }

    pub fn pop_u64(&mut self) -> Result<u64, EmuException> {
        let rsp = self.stack_addr(8)?;
        self.ac.get_data64((SgReg::SS, rsp-8))
    }

    pub fn get_sreg(&mut self) -> Result<u16, EmuException> {
        Ok(self.ac.get_sgselector(SgReg::try_from(self.idata.modrm.reg as usize).unwrap())?.to_u16())
    }

    pub fn set_sreg(&mut self, v: u16) -> Result<(), EmuException> {
        let sreg = SgReg::try_from(self.idata.modrm.reg as usize).unwrap();
        self.mov_to_sreg(sreg, v)
    }

    fn stack_addr(&mut self, size: i8) -> Result<u64, EmuException> {
        let sp = match self.ac.stsz {
            access::AcsSize::BIT16 => {
                self.ac.update_gpreg(GpReg16::SP, size as i16)?;
                self.ac.get_gpreg(GpReg16::SP)? as u64
            },
            access::AcsSize::BIT32 => {
                self.ac.update_gpreg(GpReg32::ESP, size as i32)?;
                self.ac.get_gpreg(GpReg32::ESP)? as u64
            },
            access::AcsSize::BIT64 => {
                self.ac.update_gpreg(GpReg64::RSP, size as i64)?;
                self.ac.get_gpreg(GpReg64::RSP)?
            },
        };
        Ok(sp)
    }

}