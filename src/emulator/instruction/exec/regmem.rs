use std::convert::TryFrom;
use crate::emulator::access;
use crate::emulator::access::register::*;
use crate::emulator::EmuException;

macro_rules! get_gpreg { ($self:expr, $type:ty, $reg:expr) => { $self.ac.get_gpreg(<$type>::try_from($reg as usize).unwrap()) } }
macro_rules! set_gpreg { ($self:expr, $type:ty, $reg:expr, $val:expr) => { $self.ac.set_gpreg(<$type>::try_from($reg as usize).unwrap(), $val); } }

impl<'a> super::Exec<'a> {
    pub fn get_r8(&self) -> Result<u8, EmuException> {
        if let Some(rex) = self.pdata.rex {
            get_gpreg!(self, GpReg8w, (rex.r << 3) + self.idata.modrm.reg)
        } else {
            get_gpreg!(self, GpReg8, self.idata.modrm.reg)
        }
    }

    pub fn set_r8(&mut self, v: u8) -> Result<(), EmuException> {
        if let Some(rex) = self.pdata.rex {
            set_gpreg!(self, GpReg8w, (rex.r << 3) + self.idata.modrm.reg, v)
        } else {
            set_gpreg!(self, GpReg8, self.idata.modrm.reg, v)
        }
    }

    pub fn get_rm8(&self) -> Result<u8, EmuException> {
        let modrm = self.idata.modrm;
        match (modrm.mod_, self.pdata.rex) {
            (3, Some(rex)) => get_gpreg!(self, GpReg8w, (rex.b << 3) + modrm.rm),
            (3, None) => get_gpreg!(self, GpReg8, modrm.rm),
            _ => self.ac.get_data8(Self::addr_modrm(self)?),
        }
    }

    pub fn set_rm8(&mut self, v: u8) -> Result<(), EmuException> {
        let modrm = self.idata.modrm;
        match (modrm.mod_, self.pdata.rex) {
            (3, Some(rex)) => set_gpreg!(self, GpReg8w, (rex.b << 3) + modrm.rm, v),
            (3, None) => set_gpreg!(self, GpReg8, modrm.rm, v),
            _ => self.ac.set_data8(Self::addr_modrm(self)?, v),
        }
    }

    pub fn get_opr8(&self) -> Result<u8, EmuException> {
        if let Some(rex) = self.pdata.rex {
            get_gpreg!(self, GpReg8w, (rex.b << 3) as u16 + (self.idata.opcode&0x7))
        } else {
            get_gpreg!(self, GpReg8, self.idata.opcode&0x7)
        }
    }

    pub fn set_opr8(&mut self, v: u8) -> Result<(), EmuException> {
        if let Some(rex) = self.pdata.rex {
            set_gpreg!(self, GpReg8w, (rex.b << 3) as u16 + (self.idata.opcode&0x7), v)
        } else {
            set_gpreg!(self, GpReg8, self.idata.opcode&0x7, v)
        }
    }

    pub fn get_r16(&self) -> Result<u16, EmuException> {
        let r = if let Some(rex) = self.pdata.rex { rex.r << 3 } else { 0 };
        get_gpreg!(self, GpReg16, r + self.idata.modrm.reg)
    }

    pub fn set_r16(&mut self, v: u16) -> Result<(), EmuException> {
        let r = if let Some(rex) = self.pdata.rex { rex.r << 3 } else { 0 };
        set_gpreg!(self, GpReg16, r + self.idata.modrm.reg, v)
    }

    pub fn get_rm16(&self) -> Result<u16, EmuException> {
        let modrm = self.idata.modrm;
        if modrm.mod_ == 3 {
            let b = if let Some(rex) = self.pdata.rex { rex.b << 3 } else { 0 };
            get_gpreg!(self, GpReg16, b + modrm.rm)
        } else {
            self.ac.get_data16(Self::addr_modrm(self)?)
        }
    }

    pub fn set_rm16(&mut self, v: u16) -> Result<(), EmuException> {
        let modrm = self.idata.modrm;
        if modrm.mod_ == 3 {
            let b = if let Some(rex) = self.pdata.rex { rex.b << 3 } else { 0 };
            set_gpreg!(self, GpReg16, b + modrm.rm, v)
        } else {
            self.ac.set_data16(Self::addr_modrm(self)?, v)
        }
    }

    pub fn get_opr16(&self) -> Result<u16, EmuException> {
        let b = if let Some(rex) = self.pdata.rex { rex.b << 3 } else { 0 };
        get_gpreg!(self, GpReg16, b as u16 + (self.idata.opcode&0x7))
    }

    pub fn set_opr16(&mut self, v: u16) -> Result<(), EmuException> {
        let b = if let Some(rex) = self.pdata.rex { rex.b << 3 } else { 0 };
        set_gpreg!(self, GpReg16, b as u16 + (self.idata.opcode&0x7), v)
    }

    pub fn get_r32(&self) -> Result<u32, EmuException> {
        let r = if let Some(rex) = self.pdata.rex { rex.r << 3 } else { 0 };
        get_gpreg!(self, GpReg32, r + self.idata.modrm.reg)
    }

    pub fn set_r32(&mut self, v: u32) -> Result<(), EmuException> {
        let r = if let Some(rex) = self.pdata.rex { rex.r << 3 } else { 0 };
        set_gpreg!(self, GpReg32, r + self.idata.modrm.reg, v)
    }

    pub fn get_rm32(&self) -> Result<u32, EmuException> {
        let modrm = self.idata.modrm;
        if modrm.mod_ == 3 {
            let b = if let Some(rex) = self.pdata.rex { rex.b << 3 } else { 0 };
            get_gpreg!(self, GpReg32, b + modrm.rm)
        } else {
            self.ac.get_data32(Self::addr_modrm(self)?)
        }
    }

    pub fn set_rm32(&mut self, v: u32) -> Result<(), EmuException> {
        let modrm = self.idata.modrm;
        if modrm.mod_ == 3 {
            let b = if let Some(rex) = self.pdata.rex { rex.b << 3 } else { 0 };
            set_gpreg!(self, GpReg32, b + modrm.rm, v)
        } else {
            self.ac.set_data32(Self::addr_modrm(self)?, v)
        }
    }

    pub fn get_opr32(&self) -> Result<u32, EmuException> {
        let b = if let Some(rex) = self.pdata.rex { rex.b << 3 } else { 0 };
        get_gpreg!(self, GpReg32, b as u16 + (self.idata.opcode&0x7))
    }

    pub fn set_opr32(&mut self, v: u32) -> Result<(), EmuException> {
        let b = if let Some(rex) = self.pdata.rex { rex.b << 3 } else { 0 };
        set_gpreg!(self, GpReg32, b as u16 + (self.idata.opcode&0x7), v)
    }

    pub fn get_r64(&self) -> Result<u64, EmuException> {
        let r = if let Some(rex) = self.pdata.rex { rex.r << 3 } else { 0 };
        get_gpreg!(self, GpReg64, r + self.idata.modrm.reg)
    }

    pub fn set_r64(&mut self, v: u64) -> Result<(), EmuException> {
        let r = if let Some(rex) = self.pdata.rex { rex.r << 3 } else { 0 };
        set_gpreg!(self, GpReg64, r + self.idata.modrm.reg, v)
    }

    pub fn get_rm64(&self) -> Result<u64, EmuException> {
        let modrm = self.idata.modrm;
        if modrm.mod_ == 3 {
            let b = if let Some(rex) = self.pdata.rex { rex.b << 3 } else { 0 };
            get_gpreg!(self, GpReg64, b + modrm.rm)
        } else {
            self.ac.get_data64(Self::addr_modrm(self)?)
        }
    }

    pub fn set_rm64(&mut self, v: u64) -> Result<(), EmuException> {
        let modrm = self.idata.modrm;
        if modrm.mod_ == 3 {
            let b = if let Some(rex) = self.pdata.rex { rex.b << 3 } else { 0 };
            set_gpreg!(self, GpReg64, b + modrm.rm, v)
        } else {
            self.ac.set_data64(Self::addr_modrm(self)?, v)
        }
    }

    pub fn get_opr64(&self) -> Result<u64, EmuException> {
        let b = if let Some(rex) = self.pdata.rex { rex.b << 3 } else { 0 };
        get_gpreg!(self, GpReg64, b as u16 + (self.idata.opcode&0x7))
    }

    pub fn set_opr64(&mut self, v: u64) -> Result<(), EmuException> {
        let b = if let Some(rex) = self.pdata.rex { rex.b << 3 } else { 0 };
        set_gpreg!(self, GpReg64, b as u16 + (self.idata.opcode&0x7), v)
    }

    pub fn get_imm8(&self) -> Result<u8, EmuException> {
        if let Some(v) = self.idata.imm { Ok(v as u8) } else { Err(EmuException::UnexpectedError) }
    }

    pub fn get_imm16(&self) -> Result<u16, EmuException> {
        if let Some(v) = self.idata.imm { Ok(v as u16) } else { Err(EmuException::UnexpectedError) }
    }

    pub fn get_imm32(&self) -> Result<u32, EmuException> {
        if let Some(v) = self.idata.imm { Ok(v as u32) } else { Err(EmuException::UnexpectedError) }
    }

    pub fn get_imm64(&self) -> Result<u64, EmuException> {
        self.idata.imm.ok_or(EmuException::UnexpectedError)
    }

    pub fn get_ptr16(&self) -> Result<u16, EmuException> {
        self.idata.ptr16.ok_or(EmuException::UnexpectedError)
    }

    pub fn get_moffs8(&self) -> Result<u8, EmuException> {
        self.ac.get_data8((SgReg::DS, self.idata.moffs.ok_or(EmuException::UnexpectedError)?))
    }

    pub fn set_moffs8(&mut self, v: u8) -> Result<(), EmuException> {
        self.ac.set_data8((SgReg::DS, self.idata.moffs.ok_or(EmuException::UnexpectedError)?), v)
    }

    pub fn get_moffs16(&self) -> Result<u16, EmuException> {
        self.ac.get_data16((SgReg::DS, self.idata.moffs.ok_or(EmuException::UnexpectedError)?))
    }

    pub fn set_moffs16(&mut self, v: u16) -> Result<(), EmuException> {
        self.ac.set_data16((SgReg::DS, self.idata.moffs.ok_or(EmuException::UnexpectedError)?), v)
    }

    pub fn get_moffs32(&self) -> Result<u32, EmuException> {
        self.ac.get_data32((SgReg::DS, self.idata.moffs.ok_or(EmuException::UnexpectedError)?))
    }

    pub fn set_moffs32(&mut self, v: u32) -> Result<(), EmuException> {
        self.ac.set_data32((SgReg::DS, self.idata.moffs.ok_or(EmuException::UnexpectedError)?), v)
    }

    pub fn get_moffs64(&self) -> Result<u64, EmuException> {
        self.ac.get_data64((SgReg::DS, self.idata.moffs.ok_or(EmuException::UnexpectedError)?))
    }

    pub fn set_moffs64(&mut self, v: u64) -> Result<(), EmuException> {
        self.ac.set_data64((SgReg::DS, self.idata.moffs.ok_or(EmuException::UnexpectedError)?), v)
    }

    pub fn get_m(&self) -> Result<(SgReg, u64), EmuException> {
        Ok(Self::addr_modrm(self)?)
    }

    fn addr_modrm(&self) -> Result<(SgReg, u64), EmuException> {
        let modrm = self.idata.modrm;
        let (mod_, rm) = (modrm.mod_, modrm.rm);

        assert_ne!(mod_, 3);

        let mut addr: u64 = 0;
        let mut segment: Option<SgReg> = None;

        match self.idata.adsize {
            access::AcsSize::BIT16 => {
                addr += match mod_ {
                    1|2 => self.idata.disp as u64,
                    _ => 0,
                };

                addr += match (rm, mod_) {
                    (6, 0)                   => self.idata.disp as u64,
                    (0, _) | (1, _) | (7, _) => self.ac.get_gpreg(GpReg16::BX)? as u64,
                    (2, _) | (3, _) | (6, _) => {
                        segment = Some(SgReg::SS);
                        self.ac.get_gpreg(GpReg16::BP)? as u64
                    }
                    _ => 0,
                };

                addr += match rm {
                    0|2|4 => self.ac.get_gpreg(GpReg16::SI)? as u64,
                    1|3|5 => self.ac.get_gpreg(GpReg16::DI)? as u64,
                    _ => { 0 },
                };
            },
            access::AcsSize::BIT32 => {
                addr += match mod_ {
                    1|2 => self.idata.disp as u64,
                    _ => 0,
                };

                let sgad = match (rm, mod_) {
                    (4, _) => self.addr_sib()?,
                    (5, 0) => (None, self.idata.disp as u64),
                    (5, _) => (Some(SgReg::SS), self.ac.get_gpreg(GpReg32::EBP)? as u64),
                    _      => (None, get_gpreg!(self, GpReg32, rm as usize)? as u64),
                };
                segment = sgad.0;
                addr += sgad.1 as u32 as u64;
            },
            access::AcsSize::BIT64 => {
                let b = if let Some(rex) = self.pdata.rex { rex.b << 3 } else { 0 };

                addr += match mod_ {
                    1|2 => self.idata.disp as u64,
                    _ => 0,
                };

                let sgad = match (rm, mod_, b) {
                    (4, _, _) => self.addr_sib()?,
                    (5, 0, _) => {
                        let ip: u64 = self.ac.get_ip()?;
                        (Some(SgReg::CS), ip + self.idata.disp as u64)
                    },
                    (5, _, 0) => (Some(SgReg::SS), self.ac.get_gpreg(GpReg64::RBP)?),
                    _         => (None, get_gpreg!(self, GpReg64, (b + rm) as usize)?),
                };
                segment = sgad.0;
                addr += sgad.1;
            },
        }

        Ok((self.pdata.segment.or(segment).unwrap_or(SgReg::DS), addr))
    }

    fn addr_sib(&self) -> Result<(Option<SgReg>, u64), EmuException> {
        let (modrm, sib) = (self.idata.modrm, self.idata.sib);
        let (b, x) = if let Some(rex) = self.pdata.rex { (rex.b << 3, rex.x << 3) } else { (0, 0) };

        let (seg, base) = match (sib.base, modrm.mod_, b) {
            (5, 0, _) => (None, self.idata.disp as u64),
            (4..=5, _, 0) => (Some(SgReg::SS), get_gpreg!(self, GpReg64, sib.base)?),
            _ => (None, get_gpreg!(self, GpReg64, b + sib.base)?),
        };
        let idx = if sib.index == 4 { 0 } else { get_gpreg!(self, GpReg64, x + sib.index)? };

        Ok((seg, base + idx * (1<<sib.scale)))
    }
}