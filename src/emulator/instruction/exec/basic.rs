use std::convert::TryFrom;
use crate::hardware::processor::general::*;
use crate::hardware::processor::segment::*;
use crate::emulator::EmuException;

impl<'a> super::Exec<'a> {
    pub fn get_al(&self) -> Result<u8, EmuException> {
        Ok(self.ac.core.gpregs.get(GpReg8::AL))
    }

    pub fn set_al(&mut self, v: u8) -> Result<(), EmuException> {
        self.ac.core.gpregs.set(GpReg8::AL, v);
        Ok(())
    }

    pub fn get_ax(&self) -> Result<u16, EmuException> {
        Ok(self.ac.core.gpregs.get(GpReg16::AX))
    }

    pub fn set_ax(&mut self, v: u16) -> Result<(), EmuException> {
        self.ac.core.gpregs.set(GpReg16::AX, v);
        Ok(())
    }

    pub fn get_opr8(&self) -> Result<u8, EmuException> {
        let opr = (self.idata.opcode&0x7) as usize;
        let r = GpReg8::try_from(opr).unwrap();
        Ok(self.ac.core.gpregs.get(r))
    }

    pub fn set_opr8(&mut self, v: u8) -> Result<(), EmuException> {
        let opr = (self.idata.opcode&0x7) as usize;
        let r = GpReg8::try_from(opr).unwrap();
        self.ac.core.gpregs.set(r, v);
        Ok(())
    }

    pub fn get_opr16(&self) -> Result<u16, EmuException> {
        let opr = (self.idata.opcode&0x7) as usize;
        let r = GpReg16::try_from(opr).unwrap();
        Ok(self.ac.core.gpregs.get(r))
    }

    pub fn set_opr16(&mut self, v: u16) -> Result<(), EmuException> {
        let opr = (self.idata.opcode&0x7) as usize;
        let r = GpReg16::try_from(opr).unwrap();
        self.ac.core.gpregs.set(r, v);
        Ok(())
    }

    pub fn get_opr32(&self) -> Result<u32, EmuException> {
        let opr = (self.idata.opcode&0x7) as usize;
        let r = GpReg32::try_from(opr).unwrap();
        Ok(self.ac.core.gpregs.get(r))
    }

    pub fn set_opr32(&mut self, v: u32) -> Result<(), EmuException> {
        let opr = (self.idata.opcode&0x7) as usize;
        let r = GpReg32::try_from(opr).unwrap();
        self.ac.core.gpregs.set(r, v);
        Ok(())
    }

    pub fn get_opr64(&self) -> Result<u64, EmuException> {
        let opr = (self.idata.opcode&0x7) as usize;
        let r = GpReg64::try_from(opr).unwrap();
        Ok(self.ac.core.gpregs.get(r))
    }

    pub fn set_opr64(&mut self, v: u64) -> Result<(), EmuException> {
        let opr = (self.idata.opcode&0x7) as usize;
        let r = GpReg64::try_from(opr).unwrap();
        self.ac.core.gpregs.set(r, v);
        Ok(())
    }

    pub fn push_u16(&mut self, v: u16) -> Result<(), EmuException> {
        self.ac.core.gpregs.update(GpReg16::SP, -2);
        let sp = self.ac.core.gpregs.get(GpReg16::SP) as u64;
        self.ac.set_data16((SgReg::SS, sp), v)?;
        Ok(())
    }

    pub fn pop_u16(&mut self) -> Result<u16, EmuException> {
        let sp = self.ac.core.gpregs.get(GpReg16::SP) as u64;
        self.ac.core.gpregs.update(GpReg16::SP, 2);
        Ok(self.ac.get_data16((SgReg::SS, sp))?)
    }

    pub fn push_u32(&mut self, v: u32) -> Result<(), EmuException> {
        self.ac.core.gpregs.update(GpReg32::ESP, -4);
        let esp = self.ac.core.gpregs.get(GpReg32::ESP) as u64;
        self.ac.set_data32((SgReg::SS, esp), v)?;
        Ok(())
    }

    pub fn pop_u32(&mut self) -> Result<u32, EmuException> {
        let esp = self.ac.core.gpregs.get(GpReg32::ESP) as u64;
        self.ac.core.gpregs.update(GpReg32::ESP, 4);
        Ok(self.ac.get_data32((SgReg::SS, esp))?)
    }

    pub fn push_u64(&mut self, v: u64) -> Result<(), EmuException> {
        self.ac.core.gpregs.update(GpReg64::RSP, -8);
        let rsp = self.ac.core.gpregs.get(GpReg64::RSP);
        self.ac.set_data64((SgReg::SS, rsp), v)?;
        Ok(())
    }

    pub fn pop_u64(&mut self) -> Result<u64, EmuException> {
        let rsp = self.ac.core.gpregs.get(GpReg64::RSP);
        self.ac.core.gpregs.update(GpReg64::RSP, 8);
        Ok(self.ac.get_data64((SgReg::SS, rsp))?)
    }
}