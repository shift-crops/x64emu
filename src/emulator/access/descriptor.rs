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
    pub fn get_type(&self) -> Option<DescType> {
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

pub enum DescType { System(SysDescType), Segment(SegDescType) }
#[derive(TryFromPrimitive)] #[repr(u8)]
pub enum SysDescType { TSSAvl=1, LDT=2, TSSBsy=3, Call=4, Task=5, Intr=6, Trap=7 }
pub enum SegDescType { Data(DataDescFlag), Code(CodeDescFlag) }

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

impl super::Access {
    pub fn obtain_descriptor(&self, sel: u16, gonly: bool) -> Result<Desc, EmuException> {
        let core = &self.core;

        let mut selector: SgDescSelector = Default::default();
        selector.from_u16(sel);

        let dt_index = (selector.IDX as u32) << 3;
        let mut desc: [u8;16] = [0;16];

        if dt_index > 0 {
            let (dt_base, dt_limit) = if !gonly && selector.TI == 1 { &core.dtregs.ldtr.cache } else { &core.dtregs.gdtr }.get();
            if dt_index > dt_limit { return Err(EmuException::CPUException(CPUException::GP)) }
            self.read_data_l(desc.as_mut_ptr() as *mut _, dt_base + dt_index as u64, desc.len())?;
            desc.reverse();
        }
        Ok(Desc::unpack(&desc).unwrap_or(Default::default()))
    }

    pub fn set_segment_real(&mut self, reg: SgReg, sel: u16) -> Result<(), EmuException> {
        self.get_sgselector_mut(reg)?.from_u16(sel);
        let sg = self.get_sgcache_mut(reg)?;
        sg.base = (sel as u64) << 4;
        sg.limit = 0xffff;
        sg.L = 0;
        sg.DB = 0;

        Ok(())
    }

    pub fn set_segment_protected(&mut self, reg: SgReg, sel: u16, desc: Desc) -> Result<(), EmuException> {
        let cpl  = self.get_sgselector(SgReg::CS)?.RPL;
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

        self.get_sgselector_mut(reg)?.from_u16(sel);
        *self.get_sgcache_mut(reg)? = SgDescCache::from(desc);

        Ok(())
    }
}