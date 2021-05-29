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

    pub fn get_ah(&self) -> Result<u8, EmuException> {
        self.ac.get_gpreg(GpReg8::AH)
    }

    pub fn set_ah(&mut self, v: u8) -> Result<(), EmuException> {
        self.ac.set_gpreg(GpReg8::AH, v)
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

    pub fn get_rax(&self) -> Result<u64, EmuException> {
        self.ac.get_gpreg(GpReg64::RAX)
    }

    pub fn set_rax(&mut self, v: u64) -> Result<(), EmuException> {
        self.ac.set_gpreg(GpReg64::RAX, v)
    }

    pub fn get_cl(&self) -> Result<u8, EmuException> {
        self.ac.get_gpreg(GpReg8::CL)
    }

    pub fn get_cx(&self) -> Result<u16, EmuException> {
        self.ac.get_gpreg(GpReg16::CX)
    }

    pub fn set_cx(&mut self, v: u16) -> Result<(), EmuException> {
        self.ac.set_gpreg(GpReg16::CX, v)
    }

    pub fn get_ecx(&self) -> Result<u32, EmuException> {
        self.ac.get_gpreg(GpReg32::ECX)
    }

    pub fn set_ecx(&mut self, v: u32) -> Result<(), EmuException> {
        self.ac.set_gpreg(GpReg32::ECX, v)
    }

    pub fn get_rcx(&self) -> Result<u64, EmuException> {
        self.ac.get_gpreg(GpReg64::RCX)
    }

    pub fn set_rcx(&mut self, v: u64) -> Result<(), EmuException> {
        self.ac.set_gpreg(GpReg64::RCX, v)
    }

    pub fn get_dx(&self) -> Result<u16, EmuException> {
        self.ac.get_gpreg(GpReg16::DX)
    }

    pub fn set_dx(&mut self, v: u16) -> Result<(), EmuException> {
        self.ac.set_gpreg(GpReg16::DX, v)
    }

    pub fn get_edx(&self) -> Result<u32, EmuException> {
        self.ac.get_gpreg(GpReg32::EDX)
    }

    pub fn set_edx(&mut self, v: u32) -> Result<(), EmuException> {
        self.ac.set_gpreg(GpReg32::EDX, v)
    }

    pub fn get_rdx(&self) -> Result<u64, EmuException> {
        self.ac.get_gpreg(GpReg64::RDX)
    }

    pub fn set_rdx(&mut self, v: u64) -> Result<(), EmuException> {
        self.ac.set_gpreg(GpReg64::RDX, v)
    }

    pub fn get_sreg(&mut self) -> Result<u16, EmuException> {
        Ok(self.ac.get_sgreg(SgReg::try_from(self.idata.modrm.reg as usize).unwrap())?.0)
    }

    pub fn set_sreg(&mut self, v: u16) -> Result<(), EmuException> {
        let sreg = SgReg::try_from(self.idata.modrm.reg as usize).unwrap();
        self.mov_to_sreg(sreg, v)
    }

    pub fn get_one(&self) -> Result<u8, EmuException> {
        Ok(1)
    }
}