use std::convert::TryFrom;
use num_enum::TryFromPrimitive;
use super::descriptor::*;
use crate::emulator::*;
use crate::hardware::processor::general;
use crate::hardware::processor::segment;
use crate::hardware::processor::descriptor::DescTbl;

pub type GpReg64 = general::GpReg64;
pub type GpReg32 = general::GpReg32;
pub type GpReg16 = general::GpReg16;

#[derive(TryFromPrimitive, Clone, Copy)] #[repr(usize)]
pub enum GpReg8 { AL, CL, DL, BL, AH, CH, DH, BH }
pub type GpReg8w = general::GpReg8l;

pub type SgReg = segment::SgReg;

pub trait GpRegAccess<T, U, V> {
    fn get_gpreg(&self, r: T) -> Result<U, EmuException>;
    fn set_gpreg(&mut self, r: T, v: U) -> Result<(), EmuException>;
    fn update_gpreg(&mut self, r: T, v: V) -> Result<(), EmuException>;
}

impl GpRegAccess<GpReg64, u64, i64> for super::Access {
    fn get_gpreg(&self, r: GpReg64) -> Result<u64, EmuException> { Ok(self.core.gpregs.get64(r)) }
    fn set_gpreg(&mut self, r: GpReg64, v: u64) -> Result<(), EmuException> { self.core.gpregs.set64(r, v); Ok(()) }
    fn update_gpreg(&mut self, r: GpReg64, v: i64) -> Result<(), EmuException> { self.core.gpregs.update64(r, v); Ok(()) }
}

impl GpRegAccess<GpReg32, u32, i32> for super::Access {
    fn get_gpreg(&self, r: GpReg32) -> Result<u32, EmuException> { Ok(self.core.gpregs.get32(r)) }
    fn set_gpreg(&mut self, r: GpReg32, v: u32) -> Result<(), EmuException> { self.core.gpregs.set32(r, v); Ok(()) }
    fn update_gpreg(&mut self, r: GpReg32, v: i32) -> Result<(), EmuException> { self.core.gpregs.update32(r, v); Ok(()) }
}

impl GpRegAccess<GpReg16, u16, i16> for super::Access {
    fn get_gpreg(&self, r: GpReg16) -> Result<u16, EmuException> { Ok(self.core.gpregs.get16(r)) }
    fn set_gpreg(&mut self, r: GpReg16, v: u16) -> Result<(), EmuException> { self.core.gpregs.set16(r, v); Ok(()) }
    fn update_gpreg(&mut self, r: GpReg16, v: i16) -> Result<(), EmuException> { self.core.gpregs.update16(r, v); Ok(()) }
}

impl GpRegAccess<GpReg8, u8, i8> for super::Access {
    fn get_gpreg(&self, r: GpReg8) -> Result<u8, EmuException> {
        let r = r as usize;
        Ok(if r < 4 { self.core.gpregs.get8l(general::GpReg8l::try_from(r).unwrap()) } else { self.core.gpregs.get8h(general::GpReg8h::try_from(r%4).unwrap()) })
    }
    fn set_gpreg(&mut self, r: GpReg8, v: u8) -> Result<(), EmuException> {
        let r = r as usize;
        if r < 4 { self.core.gpregs.set8l(general::GpReg8l::try_from(r).unwrap(), v) } else { self.core.gpregs.set8h(general::GpReg8h::try_from(r%4).unwrap(), v) };
        Ok(()) 
    }
    fn update_gpreg(&mut self, r: GpReg8, v: i8) -> Result<(), EmuException> {
        let r = r as usize;
        if r < 4 { self.core.gpregs.update8l(general::GpReg8l::try_from(r).unwrap(), v) } else { self.core.gpregs.update8h(general::GpReg8h::try_from(r%4).unwrap(), v) };
        Ok(()) 
    }
}

impl GpRegAccess<GpReg8w, u8, i8> for super::Access {
    fn get_gpreg(&self, r: GpReg8w) -> Result<u8, EmuException> { Ok(self.core.gpregs.get8l(r)) }
    fn set_gpreg(&mut self, r: GpReg8w, v: u8) -> Result<(), EmuException> { self.core.gpregs.set8l(r, v); Ok(()) }
    fn update_gpreg(&mut self, r: GpReg8w, v: i8) -> Result<(), EmuException> { self.core.gpregs.update8l(r, v); Ok(()) }
}

pub trait IpAccess<T, U> {
    fn get_ip(&self) -> Result<T, EmuException>;
    fn set_ip(&mut self, v: T) -> Result<(), EmuException>;
    fn update_ip(&mut self, v: U) -> Result<(), EmuException>;
}

impl IpAccess<u16, i16> for super::Access {
    fn get_ip(&self) -> Result<u16, EmuException> { Ok(self.core.ip.get_ip()) }
    fn set_ip(&mut self, v: u16) -> Result<(), EmuException> { self.core.ip.set_ip(v); Ok(()) }
    fn update_ip(&mut self, v: i16) -> Result<(), EmuException> { self.core.ip.update_ip(v); Ok(()) }
}

impl IpAccess<u32, i32> for super::Access {
    fn get_ip(&self) -> Result<u32, EmuException> { Ok(self.core.ip.get_eip()) }
    fn set_ip(&mut self, v: u32) -> Result<(), EmuException> { self.core.ip.set_eip(v); Ok(()) }
    fn update_ip(&mut self, v: i32) -> Result<(), EmuException> { self.core.ip.update_eip(v); Ok(()) }
}

impl IpAccess<u64, i64> for super::Access {
    fn get_ip(&self) -> Result<u64, EmuException> { Ok(self.core.ip.get_rip()) }
    fn set_ip(&mut self, v: u64) -> Result<(), EmuException> { self.core.ip.set_rip(v); Ok(()) }
    fn update_ip(&mut self, v: i64) -> Result<(), EmuException> { self.core.ip.update_rip(v); Ok(()) }
}

impl super::Access {
    pub fn get_rflags(&self) -> Result<u64, EmuException> { Ok(self.core.rflags.to_u64()) }
    pub fn set_rflags(&mut self, v: u64) -> Result<(), EmuException> { self.core.rflags.from_u64(v); Ok(()) }

    pub fn get_creg(&self, r: usize) -> Result<u32, EmuException> {
        if let Some(cr) = self.core.cregs.get(r) { Ok(cr.to_u32()) } else { Err(EmuException::NotImplementedFunction) }
    }

    pub fn set_creg(&mut self, r: usize, v: u32) -> Result<(), EmuException> {
        if let Some(cr) = self.core.cregs.get_mut(r) { cr.from_u32(v); Ok(()) } else { Err(EmuException::UnexpectedError) }
    }

    pub fn get_sgreg(&self, r: SgReg) -> Result<(u16, segment::SgDescCache), EmuException> {
        let sg = self.core.sgregs.get(r);
        Ok((sg.selector.to_u16(), sg.cache))
    }

    pub fn set_sgreg(&mut self, r: SgReg, sel: u16, desc: segment::SgDescCache) -> Result<(), EmuException> {
        let sg = self.core.sgregs.get_mut(r);
        sg.selector.from_u16(sel);
        sg.cache = desc;
        Ok(())
    }

    pub fn get_gdtr(&self) -> Result<(u64, u32), EmuException> { let gdtr = &self.core.dtregs.gdtr; Ok((gdtr.base, gdtr.limit)) }
    pub fn get_idtr(&self) -> Result<(u64, u32), EmuException> { let idtr = &self.core.dtregs.idtr; Ok((idtr.base, idtr.limit)) }
    pub fn get_ldtr(&self) -> Result<u16, EmuException> { Ok(self.core.dtregs.ldtr.selector) }
    pub fn get_tr(&self) -> Result<u16, EmuException> { Ok(self.core.dtregs.tr.selector) }

    pub fn set_gdtr(&mut self, base: u64, limit: u16) -> Result<(), EmuException> {
        let gdtr = &mut self.core.dtregs.gdtr;
        gdtr.base = base;
        gdtr.limit = limit as u32;
        Ok(())
    }

    pub fn set_idtr(&mut self, base: u64, limit: u16) -> Result<(), EmuException> {
        let idtr = &mut self.core.dtregs.idtr;
        idtr.base = base;
        idtr.limit = limit as u32;
        Ok(())
    }

    pub fn set_ldtr(&mut self, sel: u16) -> Result<(), EmuException> {
        if let Some(DescType::System(SysDescType::LDT(ldtd))) = self.obtain_g_desc(sel)? {
            if self.get_cpl()? > 0 { return Err(EmuException::CPUException(CPUException::GP)); }
            if ldtd.P == 0 { return Err(EmuException::CPUException(CPUException::NP)); }

            let ldtr = &mut self.core.dtregs.ldtr;
            ldtr.cache       = DescTbl::from(ldtd);
            ldtr.selector    = sel;
            Ok(())
        } else {
            Err(EmuException::CPUException(CPUException::GP))
        }
    }

    pub fn set_tr(&mut self, sel: u16) -> Result<(), EmuException> {
        if let Some(DescType::System(SysDescType::TSS(tssd))) = self.obtain_g_desc(sel)? {
            if tssd.P == 0 { return Err(EmuException::CPUException(CPUException::NP)); }

            let tr = &mut self.core.dtregs.tr;
            tr.cache       = DescTbl::from(tssd);
            tr.selector    = sel;
            Ok(())
        } else {
            Err(EmuException::CPUException(CPUException::GP))
        }
    }
}