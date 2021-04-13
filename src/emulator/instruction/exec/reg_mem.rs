use std::convert::TryFrom;
use crate::emulator::instruction::OpAdSize;
use crate::emulator::EmuException;
use crate::hardware::processor::general::*;
use crate::hardware::processor::segment::*;

macro_rules! get_gpreg { ($self:expr, $type:ty, $reg:expr) => { $self.ac.core.gpregs.get(<$type>::try_from($reg as usize).unwrap()) } }
macro_rules! set_gpreg { ($self:expr, $type:ty, $reg:expr, $val:expr) => { $self.ac.core.gpregs.set(<$type>::try_from($reg as usize).unwrap(), $val); } }

impl<'a> super::Exec<'a> {
    pub fn get_imm8(&self) -> Result<u8, EmuException> {
        Ok(self.idata.imm as u8)
    }

    pub fn get_imm16(&self) -> Result<u16, EmuException> {
        Ok(self.idata.imm as u16)
    }

    pub fn get_rm32(&self) -> Result<u32, EmuException> {
        let modrm = self.idata.modrm;
        let v = if modrm.mod_ == 3 { 
            get_gpreg!(self, GpReg32, modrm.rm)
        } else {
            self.ac.get_data32(Self::addr_modrm(self)?)?
        };
        Ok(v)
    }

    pub fn set_rm32(&mut self, v: u32) -> Result<(), EmuException> {
        let modrm = self.idata.modrm;
        if modrm.mod_ == 3 {
            set_gpreg!(self, GpReg32, modrm.rm, v);
        } else {
            self.ac.set_data32(Self::addr_modrm(self)?, v)?;
        }
        Ok(())
    }

    pub fn get_r32(&self) -> Result<u32, EmuException> {
        Ok(get_gpreg!(self, GpReg32, self.idata.modrm.reg))
    }

    pub fn set_r32(&mut self, v: u32) -> Result<(), EmuException> {
        set_gpreg!(self, GpReg32, self.idata.modrm.reg, v);
        Ok(())
    }

    pub fn get_moffs32(&self) -> Result<u32, EmuException> {
        Ok(self.ac.get_data32((SgReg::DS, self.idata.moffs))?)
    }

    pub fn set_moffs32(&mut self, v: u32) -> Result<(), EmuException> {
        self.ac.set_data32((SgReg::DS, self.idata.moffs), v)?;
        Ok(())
    }

    pub fn get_rm16(&self) -> Result<u16, EmuException> {
        let modrm = self.idata.modrm;
        let v = if modrm.mod_ == 3 {
            get_gpreg!(self, GpReg16, modrm.rm)
        } else {
            self.ac.get_data16(Self::addr_modrm(self)?)?
        };
        Ok(v)
    }

    pub fn set_rm16(&mut self, v: u16) -> Result<(), EmuException> {
        let modrm = self.idata.modrm;
        if modrm.mod_ == 3 {
            set_gpreg!(self, GpReg16, modrm.rm, v);
        } else {
            self.ac.set_data16(Self::addr_modrm(self)?, v)?;
        }
        Ok(())
    }

    pub fn get_r16(&self) -> Result<u16, EmuException> {
        Ok(get_gpreg!(self, GpReg16, self.idata.modrm.reg))
    }

    pub fn set_r16(&mut self, v: u16) -> Result<(), EmuException> {
        set_gpreg!(self, GpReg16, self.idata.modrm.reg, v);
        Ok(())
    }

    pub fn get_moffs16(&self) -> Result<u16, EmuException> {
        Ok(self.ac.get_data16((SgReg::DS, self.idata.moffs))?)
    }

    pub fn set_moffs16(&mut self, v: u16) -> Result<(), EmuException> {
        self.ac.set_data16((SgReg::DS, self.idata.moffs), v)?;
        Ok(())
    }

    pub fn get_rm8(&self) -> Result<u8, EmuException> {
        let modrm = self.idata.modrm;
        let v = if modrm.mod_ == 3 {
            get_gpreg!(self, GpReg8, modrm.rm)
        } else {
            self.ac.get_data8(Self::addr_modrm(self)?)?
        };
        Ok(v)
    }

    pub fn set_rm8(&mut self, v: u8) -> Result<(), EmuException> {
        let modrm = self.idata.modrm;
        if modrm.mod_ == 3 {
            set_gpreg!(self, GpReg8, modrm.rm, v);
        } else {
            self.ac.set_data8(Self::addr_modrm(self)?, v)?;
        }
        Ok(())
    }

    pub fn get_r8(&self) -> Result<u8, EmuException> {
        Ok(get_gpreg!(self, GpReg8, self.idata.modrm.reg))
    }

    pub fn set_r8(&mut self, v: u8) -> Result<(), EmuException> {
        set_gpreg!(self, GpReg8, self.idata.modrm.reg, v);
        Ok(())
    }

    pub fn get_moffs8(&self) -> Result<u8, EmuException> {
        Ok(self.ac.get_data8((SgReg::DS, self.idata.moffs))?)
    }

    pub fn set_moffs8(&mut self, v: u8) -> Result<(), EmuException> {
        self.ac.set_data8((SgReg::DS, self.idata.moffs), v)?;
        Ok(())
    }

    pub fn get_m(&self) -> Result<u64, EmuException> {
        Ok(Self::addr_modrm(self)?.1)
    }

    fn addr_modrm(&self) -> Result<(SgReg, u64), EmuException> {
        let modrm = self.idata.modrm;
        assert_ne!(modrm.rm, 3);

        let mut addr: u64 = 0;
        let mut segment = SgReg::DS;

        match self.idata.adsize {
            OpAdSize::BIT16 => {
                match modrm.mod_ {
                    1|2 => addr += self.idata.disp as u64,
                    _ => {},
                }

                match modrm.rm {
                    0|1|7 => addr += self.ac.core.gpregs.get(GpReg16::BX) as u64,
                    2|3|6 => {
                        if modrm.mod_ == 0 && modrm.rm == 6 {
                            addr += self.idata.disp as u64;
                        } else {
                            addr += self.ac.core.gpregs.get(GpReg16::BP) as u64;
                            segment = SgReg::SS;
                        }
                    },
                    _ => {},
                }

                if modrm.rm < 6 {
                    addr += self.ac.core.gpregs.get( if modrm.rm%2 == 1 {GpReg16::DI} else {GpReg16::SI} ) as u64;
                }
            },
            OpAdSize::BIT32 => {
                match modrm.mod_ {
                    1|2 => addr += self.idata.disp as u64,
                    _ => {},
                }

                if modrm.rm == 4 {
                    let (sg, ad) = Self::addr_sib(self)?;
                    if let Some(x) = sg { segment = x; }
                    addr += ad as u64;
                } else if modrm.rm == 5 && modrm.mod_ == 0 {
                    addr += self.idata.disp as u64;
                } else {
                    segment = if modrm.rm == 5 { SgReg::SS } else { SgReg::DS };
                    addr += self.ac.core.gpregs.get(GpReg32::try_from(modrm.rm as usize).unwrap()) as u64;
                }
            },
            OpAdSize::BIT64 => {},
        }

        if let Some(x) = self.segment { segment = x };
        Ok((segment, addr))
    }

    fn addr_sib(&self) -> Result<(Option<SgReg>, u64), EmuException> {
        let (modrm,sib) = (self.idata.modrm, self.idata.sib);

        let bs: u64;
        let mut segment = Default::default();
        if sib.base == 5 && modrm.mod_ == 0 {
            bs = self.idata.disp as u64;
        } else if sib.base == 4 {
            assert_eq!(sib.scale, 0);
            segment = Some(SgReg::SS);
            bs = 0;
        } else {
            segment = Some(if modrm.rm == 5 { SgReg::SS } else { SgReg::DS });
            bs = self.ac.core.gpregs.get(GpReg32::try_from(sib.base as usize).unwrap()) as u64;
        }

        Ok((segment, bs + self.ac.core.gpregs.get(GpReg32::try_from(sib.index as usize).unwrap()) as u64 * (1<<sib.scale)))
    }
}