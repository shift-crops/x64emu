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
pub enum SysDescType { TSS(TSSDesc), LDT(LDTDesc), Call(CallGateDesc), Task(TaskGateDesc), Intr(IntrGateDesc), Trap(TrapGateDesc) }
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
    #[packed_field(bits="16:31")]  pub selector:u16,
    #[packed_field(bits="45:46")]  pub DPL:     u8,
    #[packed_field(bits="47")]     pub P:       u8,
}

#[derive(Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="16", endian="msb")]
pub struct IntrGateDesc {
    #[packed_field(bits="0:15")]   pub offset_l:u16,
    #[packed_field(bits="16:31")]  pub selector:u16,
    #[packed_field(bits="43")]     pub D:       u8,
    #[packed_field(bits="45:46")]  pub DPL:     u8,
    #[packed_field(bits="47")]     pub P:       u8,
    #[packed_field(bits="48:63")]  pub offset_h:u16,
}

#[derive(Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="16", endian="msb")]
pub struct TrapGateDesc {
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
                SysTypes::Intr => SysDescType::Intr(IntrGateDesc::unpack(&raw).unwrap()),
                SysTypes::Trap => SysDescType::Trap(TrapGateDesc::unpack(&raw).unwrap()),
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
    pub fn obtain_gl_descriptor(&self, sel: u16) -> Result<Option<DescType>, EmuException> {
        let core = &self.core;

        let mut selector: SgDescSelector = Default::default();
        selector.from_u16(sel);

        let dt_index = (selector.IDX as u32) << 3;
        if dt_index == 0 { return Ok(None); }

        let (dt_base, dt_limit) = if selector.TI == 1 { &core.dtregs.ldtr.cache } else { &core.dtregs.gdtr }.get();
        if dt_index > dt_limit { return Err(EmuException::CPUException(CPUException::GP)) }
        self.obtain_desc(dt_base, dt_index as u64)
    }

    pub fn obtain_g_descriptor(&self, sel: u16) -> Result<Option<DescType>, EmuException> {
        let mut selector: SgDescSelector = Default::default();
        selector.from_u16(sel);

        let dt_index = (selector.IDX as u32) << 3;
        if dt_index == 0 { return Ok(None); }

        let (dt_base, dt_limit) = self.core.dtregs.gdtr.get();
        if dt_index > dt_limit { return Err(EmuException::CPUException(CPUException::GP)) }
        self.obtain_desc(dt_base, dt_index as u64)
    }

    fn obtain_desc(&self, base: u64, index: u64) -> Result<Option<DescType>, EmuException> {
        let mut raw: [u8;16] = [0;16];
        self.read_data_l(raw.as_mut_ptr() as *mut _, base + index, std::mem::size_of_val(&raw))?;
        raw.reverse();

        Ok(classify_descriptor(&raw))
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

    pub fn set_segment_protected(&mut self, reg: SgReg, sel: u16) -> Result<(), EmuException> {
        let cpl  = self.get_sgselector(SgReg::CS)?.RPL;
        let rpl = (sel & 3) as u8;

        let segdesc = match (reg, self.obtain_gl_descriptor(sel)?) {
            (_, Some(DescType::System(_)))  |
            (SgReg::CS, None) | (SgReg::CS, Some(DescType::Segment(SegDescType::Data(_)))) |
            (SgReg::SS, None) | (SgReg::SS, Some(DescType::Segment(SegDescType::Code(_)))) => {
                return Err(EmuException::CPUException(CPUException::GP));
            },
            (SgReg::CS, Some(DescType::Segment(SegDescType::Code(cd)))) => {
                if cd.P == 0 { return Err(EmuException::CPUException(CPUException::NP)); }
                cd
            },
            (SgReg::SS, Some(DescType::Segment(SegDescType::Data(dd)))) => {
                if rpl != cpl || dd.DPL != cpl || !DataDescFlag::from(&dd).contains(DataDescFlag::W) { return Err(EmuException::CPUException(CPUException::GP)); }
                if dd.P == 0 { return Err(EmuException::CPUException(CPUException::SS)); }
                dd
            },
            (_, Some(DescType::Segment(SegDescType::Data(dd)))) => {
                if dd.P == 0 { return Err(EmuException::CPUException(CPUException::NP)); }
                dd
           },
            (_, Some(DescType::Segment(SegDescType::Code(cd)))) => {
                if !CodeDescFlag::from(&cd).contains(CodeDescFlag::R) {
                    return Err(EmuException::CPUException(CPUException::GP));
                }
                if cd.P == 0 { return Err(EmuException::CPUException(CPUException::NP)); }
                cd
            },
            (_, None) => Default::default(),
        };

        self.get_sgselector_mut(reg)?.from_u16(sel);
        *self.get_sgcache_mut(reg)? = SgDescCache::from(segdesc);

        Ok(())
    }
}