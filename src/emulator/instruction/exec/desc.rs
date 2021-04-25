#![allow(non_snake_case)]

use std::convert::TryFrom;
use num_enum::TryFromPrimitive;
use packed_struct::prelude::*;
use crate::emulator::*;
use crate::emulator::access::register::*;
use crate::hardware::processor::segment::{SgDescSelector, SgDescCache};
use crate::hardware::processor::descriptor::DescTbl;

#[derive(Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="16", endian="msb")]
pub struct Desc {
    #[packed_field(bits="0:15")]   pub limit_l: u16,
    #[packed_field(bits="16:39")]  pub base_l:  u32,
    #[packed_field(bits="40:43")]  pub Type:    u8,
    #[packed_field(bits="44")]     pub S:       u8,
    #[packed_field(bits="45:46")]  pub DPL:     u8,
    #[packed_field(bits="47")]     pub P:       u8,
    #[packed_field(bits="48:51")]  pub limit_h: u8,
    #[packed_field(bits="52")]     pub AVL:     u8,
    #[packed_field(bits="53")]     pub L:       u8,
    #[packed_field(bits="54")]     pub DB:      u8,
    #[packed_field(bits="55")]     pub G:       u8,
    #[packed_field(bits="56:95")]  pub base_h:  u64,
}

impl Desc {
    fn get_type(&self) -> Option<DescType> {
        if self.S == 0 { // system
            if let Ok(t) = SysDescType::try_from(self.Type&7) {
                return Some(DescType::System(t));
            }
        } else {         // segment
            let ds = if self.Type & 8 == 0 { SegDescType::Data(DataDescFlag::from(self.Type&7)) } else { SegDescType::Code(CodeDescFlag::from(self.Type&7)) };
            return Some(DescType::Segment(ds));
        }
        None
    }
}

enum DescType { System(SysDescType), Segment(SegDescType) }
#[derive(TryFromPrimitive)] #[repr(u8)]
enum SysDescType { TSSAvl=1, LDT=2, TSSBsy=3, Call=4, Task=5, Intr=6, Trap=7 }
enum SegDescType { Data(DataDescFlag), Code(CodeDescFlag) }

bitflags! { pub struct DataDescFlag: u8 {
    const A   = 0b00000001;
    const W   = 0b00000010;
    const E   = 0b00000100;
} }
impl From<u8> for DataDescFlag {
    fn from(v: u8) -> Self { Self { bits: v } }
}

bitflags! { pub struct CodeDescFlag: u8 {
    const A   = 0b00000001;
    const R   = 0b00000010;
    const C   = 0b00000100;
} }
impl From<u8> for CodeDescFlag {
    fn from(v: u8) -> Self { Self { bits: v } }
}

impl From<Desc> for SgDescCache {
    fn from(desc: Desc) -> Self {
        Self {
            base    : ((desc.base_h as u8 as u64) << 24) + desc.base_l as u64,
            limit   : ((desc.limit_h as u32) << 16) + desc.limit_l as u32,
            Type    : desc.Type,
            DPL     : desc.DPL,
            P       : desc.P,
            AVL     : desc.AVL,
            L       : desc.L,
            DB      : desc.DB,
            G       : desc.G,
        }
    }
}

impl From<Desc> for DescTbl {
    fn from(desc: Desc) -> Self {
        Self {
            base    : (desc.base_h << 24) + desc.base_l as u64,
            limit   : ((desc.limit_h as u32) << 16) + desc.limit_l as u32,
        }
    }
}

macro_rules! ret_far {
    ( $type:ty ) => { paste::item! {
        pub fn [<ret_far_ $type>](&mut self) -> Result<(), EmuException> {
            let new_ip = self.[<pop_ $type>]()?;
            let new_cs = self.[<pop_ $type>]()? as u16;

            match self.ac.mode {
                access::CpuMode::Real => {
                    if new_ip as u32 > self.ac.get_sgcache(SgReg::CS)?.limit {
                        return Err(EmuException::CPUException(CPUException::GP));
                    }
                    self.set_segment_real(SgReg::CS, new_cs)?;
                },
                access::CpuMode::Protected | access::CpuMode::Long => {
                    let cpl = self.ac.get_sgselector(SgReg::CS)?.RPL;
                    let rpl = (new_cs & 3) as u8;

                    if new_cs == 0 || rpl < cpl {
                        return Err(EmuException::CPUException(CPUException::GP));
                    }

                    let desc = self.obtain_descriptor(new_cs, false)?;
                    if let Some(DescType::Segment(SegDescType::Code(f))) = desc.get_type() {
                        if f.contains(CodeDescFlag::C) && desc.DPL > rpl {
                            return Err(EmuException::CPUException(CPUException::GP));
                        }
                    } else {
                        return Err(EmuException::CPUException(CPUException::GP));
                    }

                    if self.ac.size.op != access::AcsSize::BIT64 && new_ip as u32 > ((desc.limit_h as u32) << 16) + desc.limit_l as u32 {
                        return Err(EmuException::CPUException(CPUException::GP));
                    }
                    self.set_segment_protected(SgReg::CS, new_cs, desc)?;

                    if rpl > cpl {
                        let new_sp = self.[<pop_ $type>]()?;
                        let new_ss = self.[<pop_ $type>]()? as u16;
                        self.ac.set_gpreg(GpReg64::RSP, new_sp as u64)?;
                        self.set_segment_protected(SgReg::SS, new_ss, self.obtain_descriptor(new_ss, false)?)?;
                    }
                    for r in vec!(SgReg::ES, SgReg::FS, SgReg::GS, SgReg::DS ).iter() {
                        if rpl > self.ac.get_sgcache(*r)?.DPL {
                            self.set_segment_protected(*r, 0, self.obtain_descriptor(0, false)?)?;
                        }
                    }
                },
            }

            self.ac.set_ip(new_ip)
        }
    } };
}

macro_rules! jmp_far {
    ( $type:ty ) => { paste::item! {
        pub fn [<jmp_far_ $type>](&mut self, sel: u16, abs: $type) -> Result<(), EmuException> {
            match self.ac.mode {
                access::CpuMode::Real => {
                    if abs as u32 > self.ac.get_sgcache(SgReg::CS)?.limit {
                        return Err(EmuException::CPUException(CPUException::GP));
                    }

                    self.set_segment_real(SgReg::CS, sel)?;
                    self.ac.set_ip(abs)?;
                },
                access::CpuMode::Protected | access::CpuMode::Long => {
                    let cpl  = self.ac.get_sgselector(SgReg::CS)?.RPL;
                    let rpl = (sel & 3) as u8;
                    let desc = self.obtain_descriptor(sel, false)?;

                    match desc.get_type() {
                        Some(DescType::Segment(SegDescType::Code(f))) => {
                            if f.contains(CodeDescFlag::C) {
                                if desc.DPL > cpl {
                                    return Err(EmuException::CPUException(CPUException::GP));
                                }
                            } else if rpl > cpl || desc.DPL != cpl {
                                return Err(EmuException::CPUException(CPUException::GP));
                            }
                            if self.ac.size.op != access::AcsSize::BIT64 && abs as u32 > ((desc.limit_h as u32) << 16) + desc.limit_l as u32 {
                                return Err(EmuException::CPUException(CPUException::GP));
                            }
                            self.set_segment_protected(SgReg::CS, (sel & 0xff8) | cpl as u16, desc)?;
                            self.ac.set_ip(abs)?;
                        },
                        Some(DescType::System(SysDescType::Call)) => { return Err(EmuException::NotImplementedOpcode); },
                        Some(DescType::System(SysDescType::Task)) => { return Err(EmuException::NotImplementedOpcode); },
                        Some(DescType::System(SysDescType::TSSAvl)) => { return Err(EmuException::NotImplementedOpcode); },
                        _ => {
                            return Err(EmuException::CPUException(CPUException::GP));
                        },
                    }
                },
            }
            Ok(())
        }
    } };
}

macro_rules! call_far {
    ( $type:ty ) => { paste::item! {
        pub fn [<call_far_ $type>](&mut self, sel: u16, abs: $type) -> Result<(), EmuException> {
            match self.ac.mode {
                access::CpuMode::Real => {
                    if abs as u32 > self.ac.get_sgcache(SgReg::CS)?.limit {
                        return Err(EmuException::CPUException(CPUException::GP));
                    }

                    self.[<push_ $type>](self.ac.get_sgselector(SgReg::CS)?.to_u16() as $type)?;
                    self.[<push_ $type>](self.ac.get_ip()?)?;
                    self.set_segment_real(SgReg::CS, sel)?;
                    self.ac.set_ip(abs)?;
                },
                access::CpuMode::Protected | access::CpuMode::Long => {
                    let cpl  = self.ac.get_sgselector(SgReg::CS)?.RPL;
                    let rpl = (sel & 3) as u8;
                    let desc = self.obtain_descriptor(sel, false)?;

                    match desc.get_type() {
                        Some(DescType::Segment(SegDescType::Code(f))) => {
                            if f.contains(CodeDescFlag::C) {
                                if desc.DPL > cpl {
                                    return Err(EmuException::CPUException(CPUException::GP));
                                }
                            } else if rpl > cpl || desc.DPL != cpl {
                                return Err(EmuException::CPUException(CPUException::GP));
                            }
                            if self.ac.size.op != access::AcsSize::BIT64 && abs as u32 > ((desc.limit_h as u32) << 16) + desc.limit_l as u32 {
                                return Err(EmuException::CPUException(CPUException::GP));
                            }
                            self.[<push_ $type>](self.ac.get_sgselector(SgReg::CS)?.to_u16() as $type)?;
                            self.[<push_ $type>](self.ac.get_ip()?)?;
                            self.set_segment_protected(SgReg::CS, (sel & 0xff8) | cpl as u16, desc)?;
                            self.ac.set_ip(abs)?;
                        },
                        Some(DescType::System(SysDescType::Call)) => { return Err(EmuException::NotImplementedOpcode); },
                        Some(DescType::System(SysDescType::Task)) => { return Err(EmuException::NotImplementedOpcode); },
                        Some(DescType::System(SysDescType::TSSAvl)) => { return Err(EmuException::NotImplementedOpcode); },
                        _ => {
                            return Err(EmuException::CPUException(CPUException::GP));
                        },
                    }
                },
            }
            Ok(())
        }
    } };
}

impl<'a> super::Exec<'a> {
    ret_far!(u16);
    ret_far!(u32);
    ret_far!(u64);

    jmp_far!(u16);
    jmp_far!(u32);
    jmp_far!(u64);

    call_far!(u16);
    call_far!(u32);
    call_far!(u64);

    pub fn mov_to_sreg(&mut self, reg: SgReg, sel: u16) -> Result<(), EmuException> {
        if reg == SgReg::CS {
            return Err(EmuException::CPUException(CPUException::UD));
        }
        match self.ac.mode {
            access::CpuMode::Real =>
                self.set_segment_real(reg, sel),
            access::CpuMode::Protected | access::CpuMode::Long =>
                self.set_segment_protected(reg, sel, self.obtain_descriptor(sel, false)?),
        }
    }
}

impl<'a> super::Exec<'a> {
    fn obtain_descriptor(&self, sel: u16, gonly: bool) -> Result<Desc, EmuException> {
        let core = &self.ac.core;

        let mut selector: SgDescSelector = Default::default();
        selector.from_u16(sel);

        let dt_index = (selector.IDX as u32) << 3;
        let mut desc: [u8;16] = [0;16];

        if dt_index > 0 {
            let (dt_base, dt_limit) = if !gonly && selector.TI == 1 { &core.dtregs.ldtr.cache } else { &core.dtregs.gdtr }.get();
            if dt_index > dt_limit { return Err(EmuException::CPUException(CPUException::GP)) }
            self.ac.read_data_l(desc.as_mut_ptr() as *mut _, dt_base + dt_index as u64, desc.len())?;
            desc.reverse();
        }
        Ok(Desc::unpack(&desc).unwrap_or(Default::default()))
    }

    fn set_segment_real(&mut self, reg: SgReg, sel: u16) -> Result<(), EmuException> {
        self.ac.get_sgselector_mut(reg)?.from_u16(sel);
        let sg = self.ac.get_sgcache_mut(reg)?;
        sg.base = (sel as u64) << 4;
        sg.limit = 0xffff;
        sg.L = 0;
        sg.DB = 0;

        match reg {
            SgReg::CS => self.update_opadsize()?,
            SgReg::SS => self.update_stacksize()?,
            _ => {},
        }

        Ok(())
    }

    fn set_segment_protected(&mut self, reg: SgReg, sel: u16, desc: Desc) -> Result<(), EmuException> {
        let cpl  = self.ac.get_sgselector(SgReg::CS)?.RPL;
        let rpl = (sel & 3) as u8;
        let ty   = desc.get_type();

        match (reg, ty) {
            (_, Some(DescType::System(_))) | (SgReg::CS, None) | (SgReg::SS, None) => {
                return Err(EmuException::CPUException(CPUException::GP));
            },
            (SgReg::CS, Some(DescType::Segment(t))) => {
                match t {
                    SegDescType::Code(_) => { },
                    _ => { return Err(EmuException::CPUException(CPUException::GP)); }
                }
                if desc.P == 0 {
                    return Err(EmuException::CPUException(CPUException::NP));
                }
            },
            (SgReg::SS, Some(DescType::Segment(t))) => {
                if rpl != cpl || desc.DPL != cpl {
                    return Err(EmuException::CPUException(CPUException::GP));
                }
                match t {
                    SegDescType::Data(f) => {
                        if !f.contains(DataDescFlag::W) { return Err(EmuException::CPUException(CPUException::GP)); }
                    },
                    _ => { return Err(EmuException::CPUException(CPUException::GP)); }
                }
                if desc.P == 0 {
                    return Err(EmuException::CPUException(CPUException::SS));
                }
            },
            (_, Some(DescType::Segment(t))) => {
                match t {
                    SegDescType::Data(_) => {},
                    SegDescType::Code(f) => {
                        if !f.contains(CodeDescFlag::R) {
                            return Err(EmuException::CPUException(CPUException::GP));
                        }
                    },
                }
                if desc.P == 0 {
                    return Err(EmuException::CPUException(CPUException::NP));
                }
            },
            (_, None) => {},
        }

        self.ac.get_sgselector_mut(reg)?.from_u16(sel);
        *self.ac.get_sgcache_mut(reg)? = SgDescCache::from(desc);

        match reg {
            SgReg::CS => self.update_opadsize()?,
            SgReg::SS => self.update_stacksize()?,
            _ => {},
        }

        Ok(())
    }

    pub fn set_gdtr(&mut self, base: u64, limit: u16) -> Result<(), EmuException> {
        let gdtr = &mut self.ac.core.dtregs.gdtr;
        gdtr.base = base;
        gdtr.limit = limit as u32;
        Ok(())
    }

    pub fn set_idtr(&mut self, base: u64, limit: u16) -> Result<(), EmuException> {
        let idtr = &mut self.ac.core.dtregs.idtr;
        idtr.base = base;
        idtr.limit = limit as u32;
        Ok(())
    }

    fn get_ldtr(&self) -> Result<u16, EmuException> {
        Ok(self.ac.core.dtregs.ldtr.selector)
    }

    pub fn set_ldtr(&mut self, sel: u16) -> Result<(), EmuException> {
        let desc = self.obtain_descriptor(sel, true)?;
        let ldtr = &mut self.ac.core.dtregs.ldtr;
        ldtr.cache       = DescTbl::from(desc);
        ldtr.selector    = sel;

        Ok(())
    }

    fn get_tr(&self) -> Result<u16, EmuException> {
        Ok(self.ac.core.dtregs.tr.selector)
    }

    pub fn set_tr(&mut self, sel: u16) -> Result<(), EmuException> {
        let desc = self.obtain_descriptor(sel, true)?;
        let tr = &mut self.ac.core.dtregs.tr;
        tr.cache    = DescTbl::from(desc);
        tr.selector = sel;

        Ok(())
    }
}