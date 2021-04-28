#![allow(non_snake_case)]

use std::convert::TryFrom;
use num_enum::TryFromPrimitive;
use packed_struct::prelude::*;
use crate::emulator::*;
use crate::emulator::access::register::*;
use crate::hardware::processor::segment::{SgDescSelector, SgDescCache};
use crate::hardware::processor::descriptor::DescTbl;

#[derive(TryFromPrimitive)] #[repr(u8)]
pub enum SysTypes { TSSAvl=1, LDT=2, TSSBsy=3, Call=4, Task=5, Intr=6, Trap=7 }

pub enum DescType { System(SysDescType), Segment(SegDescType) }
pub enum SysDescType { TSS(TSSDesc), LDT(LDTDesc), Call(CallGateDesc), Task(TaskGateDesc), Intr(IntrTrapGateDesc), Trap(IntrTrapGateDesc) }
pub enum SegDescType { Data(SegDesc), Code(SegDesc) }

#[derive(Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="16", endian="msb")]
pub struct Desc {
    #[packed_field(bits="40:43")]  pub Type:    u8,
    #[packed_field(bits="44")]     pub S:       u8,
}

#[derive(Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="16", endian="msb")]
pub struct SegDesc {
    #[packed_field(bits="0:15")]   pub limit_l: u16,
    #[packed_field(bits="16:39")]  pub base_l:  u32,
    #[packed_field(bits="40:43")]  pub Type:    u8,
    #[packed_field(bits="45:46")]  pub DPL:     u8,
    #[packed_field(bits="47")]     pub P:       u8,
    #[packed_field(bits="48:51")]  pub limit_h: u8,
    #[packed_field(bits="52")]     pub AVL:     u8,
    #[packed_field(bits="53")]     pub L:       u8,
    #[packed_field(bits="54")]     pub DB:      u8,
    #[packed_field(bits="55")]     pub G:       u8,
    #[packed_field(bits="56:63")]  pub base_h:  u8,
}

bitflags! { pub struct DataDescFlag: u8 {
    const A   = 0b00000001;
    const W   = 0b00000010;
    const E   = 0b00000100;
} }
impl From<&SegDesc> for DataDescFlag {
    fn from(desc: &SegDesc) -> Self { Self { bits: desc.Type } }
}

bitflags! { pub struct CodeDescFlag: u8 {
    const A   = 0b00000001;
    const R   = 0b00000010;
    const C   = 0b00000100;
} }
impl From<&SegDesc> for CodeDescFlag {
    fn from(desc: &SegDesc) -> Self { Self { bits: desc.Type } }
}

impl From<SegDesc> for SgDescCache {
    fn from(desc: SegDesc) -> Self {
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

#[derive(Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="16", endian="msb")]
pub struct TSSDesc {
    #[packed_field(bits="0:15")]   pub limit_l: u16,
    #[packed_field(bits="16:39")]  pub base_l:  u32,
    #[packed_field(bits="41")]     pub B:       u8,
    #[packed_field(bits="45:46")]  pub DPL:     u8,
    #[packed_field(bits="47")]     pub P:       u8,
    #[packed_field(bits="48:51")]  pub limit_h: u8,
    #[packed_field(bits="55")]     pub G:       u8,
    #[packed_field(bits="56:95")]  pub base_h:  u64,
}

impl From<TSSDesc> for DescTbl {
    fn from(desc: TSSDesc) -> Self {
        Self {
            base  : (desc.base_h << 24) + desc.base_l as u64,
            limit : ((desc.limit_h as u32) << 16) + desc.limit_l as u32,
        }
    }
}

#[derive(Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="16", endian="msb")]
pub struct LDTDesc {
    #[packed_field(bits="0:15")]   pub limit_l: u16,
    #[packed_field(bits="16:39")]  pub base_l:  u32,
    #[packed_field(bits="45:46")]  pub DPL:     u8,
    #[packed_field(bits="47")]     pub P:       u8,
    #[packed_field(bits="48:51")]  pub limit_h: u8,
    #[packed_field(bits="55")]     pub G:       u8,
    #[packed_field(bits="56:95")]  pub base_h:  u64,
}

impl From<LDTDesc> for DescTbl {
    fn from(desc: LDTDesc) -> Self {
        Self {
            base  : (desc.base_h << 24) + desc.base_l as u64,
            limit : ((desc.limit_h as u32) << 16) + desc.limit_l as u32,
        }
    }
}

#[derive(Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="16", endian="msb")]
pub struct CallGateDesc {
    #[packed_field(bits="0:15")]   pub offset_l:u16,
    #[packed_field(bits="16:31")]  pub selector:u16,
    #[packed_field(bits="32:39")]  pub pc:      u8,
    #[packed_field(bits="45:46")]  pub DPL:     u8,
    #[packed_field(bits="47")]     pub P:       u8,
    #[packed_field(bits="48:63")]  pub offset_h:u16,
}

#[derive(Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="16", endian="msb")]
pub struct TaskGateDesc {
    #[packed_field(bits="16:31")]  pub tss_sel: u16,
    #[packed_field(bits="45:46")]  pub DPL:     u8,
    #[packed_field(bits="47")]     pub P:       u8,
}

#[derive(Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="16", endian="msb")]
pub struct IntrTrapGateDesc {
    #[packed_field(bits="0:15")]   pub offset_l:u16,
    #[packed_field(bits="16:31")]  pub selector:u16,
    #[packed_field(bits="43")]     pub D:       u8,
    #[packed_field(bits="45:46")]  pub DPL:     u8,
    #[packed_field(bits="47")]     pub P:       u8,
    #[packed_field(bits="48:63")]  pub offset_h:u16,
}

fn classify_descriptor(raw: &[u8; 16]) -> Option<DescType> {
    let desc = Desc::unpack(&raw).unwrap_or(Default::default());

    if desc.S == 0 { // system
        if let Ok(t) = SysTypes::try_from(desc.Type&7) {
            let sysdsc = match t {
                SysTypes::TSSAvl | SysTypes::TSSBsy => SysDescType::TSS(TSSDesc::unpack(&raw).unwrap()),
                SysTypes::LDT =>  SysDescType::LDT(LDTDesc::unpack(&raw).unwrap()),
                SysTypes::Call => SysDescType::Call(CallGateDesc::unpack(&raw).unwrap()),
                SysTypes::Task => SysDescType::Task(TaskGateDesc::unpack(&raw).unwrap()),
                SysTypes::Intr => SysDescType::Intr(IntrTrapGateDesc::unpack(&raw).unwrap()),
                SysTypes::Trap => SysDescType::Trap(IntrTrapGateDesc::unpack(&raw).unwrap()),
            };
            return Some(DescType::System(sysdsc));
        }
    } else {         // segment
        let sg = SegDesc::unpack(&raw).unwrap();
        let segdsc = if desc.Type & 8 == 0 { SegDescType::Data(sg) } else { SegDescType::Code(sg) };
        return Some(DescType::Segment(segdsc));
    }
    None
}

impl super::Access {
    pub fn get_cpl(&self) -> Result<u8, EmuException> {
        Ok(self.get_sgselector(SgReg::CS)?.RPL)
    }

    pub fn obtain_gl_descriptor(&self, sel: u16) -> Result<Option<DescType>, EmuException> {
        let core = &self.core;

        let selector = SgDescSelector::new(sel);
        let dt_index = (selector.IDX as u32) << 3;
        if dt_index == 0 { return Ok(None); }

        let (dt_base, dt_limit) = if selector.TI == 1 { &core.dtregs.ldtr.cache } else { &core.dtregs.gdtr }.get();
        if dt_index > dt_limit { return Err(EmuException::CPUException(CPUException::GP)) }
        self.obtain_desc(dt_base + dt_index as u64)
    }

    pub fn obtain_g_descriptor(&self, sel: u16) -> Result<Option<DescType>, EmuException> {
        let selector = SgDescSelector::new(sel);
        let dt_index = (selector.IDX as u32) << 3;
        if dt_index == 0 { return Ok(None); }

        let (dt_base, dt_limit) = self.core.dtregs.gdtr.get();
        if dt_index > dt_limit { return Err(EmuException::CPUException(CPUException::GP)) }
        self.obtain_desc(dt_base + dt_index as u64)
    }

    pub fn obtain_i_descriptor(&self, idx: u8) -> Result<Option<DescType>, EmuException> {
        let dt_index = (idx as u32) << 3;
        let (dt_base, dt_limit) = self.core.dtregs.idtr.get();
        if dt_index > dt_limit { return Err(EmuException::CPUException(CPUException::GP)) }
        self.obtain_desc(dt_base + dt_index as u64)
    }

    fn obtain_desc(&self, desc_addr: u64) -> Result<Option<DescType>, EmuException> {
        let mut raw: [u8;16] = [0;16];
        self.read_data_l(raw.as_mut_ptr() as *mut _, desc_addr, std::mem::size_of_val(&raw))?;
        raw.reverse();

        Ok(classify_descriptor(&raw))
    }

    pub fn load_segment(&mut self, reg: SgReg, sel: u16) -> Result<(), EmuException> {
        let cache = match self.mode {
            access::CpuMode::Real => {
                let mut cache: SgDescCache = Default::default();
                cache.base = (sel as u64) << 4;
                cache.limit = 0xffff;
                cache
            },
            access::CpuMode::Protected | access::CpuMode::Long => {
                let rpl = (sel&3) as u8;
                let desc = match self.obtain_gl_descriptor(sel)? {
                    Some(DescType::System(_)) => { return Err(EmuException::CPUException(CPUException::GP)) },
                    Some(DescType::Segment(segdsc)) => Some(segdsc),
                    None => None,
                };
                self.select_segdesc(reg, rpl, desc)?
            },
        };
        self.set_sgreg(reg, sel, cache)
    }

    pub fn select_segdesc(&mut self, reg: SgReg, rpl: u8, desc: Option<SegDescType>) -> Result<SgDescCache, EmuException> {
        let cpl  = self.get_cpl()?;

        let segdesc = match (reg, desc) {
            (SgReg::CS, None) | (SgReg::CS, Some(SegDescType::Data(_))) |
            (SgReg::SS, None) | (SgReg::SS, Some(SegDescType::Code(_))) => {
                return Err(EmuException::CPUException(CPUException::GP));
            },
            (SgReg::CS, Some(SegDescType::Code(cdesc))) => {
                if cdesc.P == 0 { return Err(EmuException::CPUException(CPUException::NP)); }
                cdesc
            },
            (SgReg::SS, Some(SegDescType::Data(ddesc))) => {
                if cpl != rpl || cpl != ddesc.DPL || !DataDescFlag::from(&ddesc).contains(DataDescFlag::W) {
                    return Err(EmuException::CPUException(CPUException::GP));
                } else if ddesc.P == 0 {
                    return Err(EmuException::CPUException(CPUException::SS));
                }
                ddesc
            },
            (_, Some(SegDescType::Data(ddesc))) => {
                if ddesc.P == 0 { return Err(EmuException::CPUException(CPUException::NP)); }
                ddesc
            },
            (_, Some(SegDescType::Code(cdesc))) => {
                if !CodeDescFlag::from(&cdesc).contains(CodeDescFlag::R) {
                    return Err(EmuException::CPUException(CPUException::GP));
                }
                if cdesc.P == 0 { return Err(EmuException::CPUException(CPUException::NP)); }
                cdesc
            },
            (_, None) => Default::default(),
        };

        Ok(SgDescCache::from(segdesc))
    }

    pub fn select_callgate(&mut self, desc: CallGateDesc) -> Result<(u16, SegDesc), EmuException> {
        if desc.P == 0 { return Err(EmuException::CPUException(CPUException::NP)); }

        match self.obtain_gl_descriptor(desc.selector)? {
            Some(DescType::Segment(SegDescType::Code(cdesc))) => Ok((desc.selector, cdesc)),
            _ => Err(EmuException::CPUException(CPUException::GP)),
        }
    }

    pub fn select_taskgate(&mut self, desc: TaskGateDesc) -> Result<TSSDesc, EmuException> {
        if desc.P == 0 { return Err(EmuException::CPUException(CPUException::NP)); }
        if SgDescSelector::new(desc.tss_sel).TI == 1 { return Err(EmuException::CPUException(CPUException::GP)); }

        match self.obtain_g_descriptor(desc.tss_sel)? {
            Some(DescType::System(SysDescType::TSS(tssdesc))) => Ok(tssdesc),
            _ => Err(EmuException::CPUException(CPUException::GP)),
        }
    }

    pub fn select_intrtrapgate(&mut self, desc: IntrTrapGateDesc) -> Result<(u16, SegDesc), EmuException> {
        if desc.P == 0 { return Err(EmuException::CPUException(CPUException::NP)); }

        match self.obtain_gl_descriptor(desc.selector)? {
            Some(DescType::Segment(SegDescType::Code(cdesc))) => Ok((desc.selector, cdesc)),
            _ => Err(EmuException::CPUException(CPUException::GP)),
        }
    }

    pub fn switch_task(&mut self, desc: TSSDesc) -> Result<(), EmuException> {
        if desc.P == 0 { return Err(EmuException::CPUException(CPUException::NP)); }
        if desc.B == 1 { return Err(EmuException::CPUException(CPUException::GP)); }
        Err(EmuException::NotImplementedFunction)
    }
}