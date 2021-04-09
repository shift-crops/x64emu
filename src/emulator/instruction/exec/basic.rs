use crate::hardware::processor::general::*;
use crate::hardware::processor::segment::*;

impl<'a> super::Exec<'a> {
    pub fn update_rip(&mut self, v: i64) -> () {
        self.ac.core.rip.update(v);
    }

    pub fn get_al(&self) -> u8 {
        self.ac.core.gpregs.get(GpReg8::AL)
    }

    pub fn set_al(&mut self, v: u8) -> () {
        self.ac.core.gpregs.set(GpReg8::AL, v);
    }

    pub fn get_ax(&self) -> u16 {
        self.ac.core.gpregs.get(GpReg16::AX)
    }

    pub fn set_ax(&mut self, v: u16) -> () {
        self.ac.core.gpregs.set(GpReg16::AX, v);
    }

    pub fn push16(&mut self, v: u16) -> () {
        self.ac.core.gpregs.update(GpReg16::SP, -2);
        let sp = self.ac.core.gpregs.get(GpReg16::SP) as u64;
        self.ac.set_data16((SgReg::SS, sp), v);
    }

    pub fn pop16(&mut self) -> u16 {
        let sp = self.ac.core.gpregs.get(GpReg16::SP) as u64;
        self.ac.core.gpregs.update(GpReg16::SP, 2);
        self.ac.get_data16((SgReg::SS, sp))
    }

    pub fn push32(&mut self, v: u32) -> () {
        self.ac.core.gpregs.update(GpReg32::ESP, -4);
        let esp = self.ac.core.gpregs.get(GpReg32::ESP) as u64;
        self.ac.set_data32((SgReg::SS, esp), v);
    }

    pub fn pop32(&mut self) -> u32 {
        let esp = self.ac.core.gpregs.get(GpReg32::ESP) as u64;
        self.ac.core.gpregs.update(GpReg32::ESP, 4);
        self.ac.get_data32((SgReg::SS, esp))
    }

    pub fn push64(&mut self, v: u64) -> () {
        self.ac.core.gpregs.update(GpReg64::RSP, -8);
        let rsp = self.ac.core.gpregs.get(GpReg64::RSP);
        self.ac.set_data64((SgReg::SS, rsp), v);
    }

    pub fn pop64(&mut self) -> u64 {
        let rsp = self.ac.core.gpregs.get(GpReg64::RSP);
        self.ac.core.gpregs.update(GpReg64::RSP, 8);
        self.ac.get_data64((SgReg::SS, rsp))
    }
}