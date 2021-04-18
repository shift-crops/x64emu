#![allow(non_snake_case)]

use std::convert::TryFrom;
use num_enum::TryFromPrimitive;
use packed_struct::prelude::*;
use crate::emulator::*;
use crate::hardware::processor::*;

#[derive(Debug, Default, Clone, Copy, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="8", endian="msb")]
pub struct Desc {
    #[packed_field(bits="40:43")]  pub Type:    u8,
    #[packed_field(bits="44")]     pub S:       u8,
}

enum DescType { System(SysDescType), Segment(SegDescType) }
#[derive(TryFromPrimitive)] #[repr(u8)]
enum SysDescType { TSSAvl=1, LDT=2, TSSBsy=3, Call=4, Task=5, Intr=6, Trap=7 }
enum SegDescType { Data, Code }

impl Desc {
    fn get_type(&self) -> Option<DescType> {
        if self.S == 0 { // system
            if let Ok(t) = SysDescType::try_from(self.Type&7) {
                return Some(DescType::System(t));
            }
        } else {         // segment
            return Some(DescType::Segment(if self.Type & 8 == 0 { SegDescType::Data } else { SegDescType::Code }));
        }
        None
    }
}

#[derive(Debug, Default, Clone, Copy, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="16", endian="msb")]
pub struct SysDesc {
    #[packed_field(bits="0:15")]   pub limit_l: u16,
    #[packed_field(bits="16:39")]  pub base_l:  u32,
    #[packed_field(bits="40:43")]  pub Type:    u8,
    #[packed_field(bits="44")]     pub S:       ReservedZero<packed_bits::Bits1>,
    #[packed_field(bits="45:46")]  pub DPL:     u8,
    #[packed_field(bits="47")]     pub P:       u8,
    #[packed_field(bits="48:51")]  pub limit_h: u8,
    #[packed_field(bits="55")]     pub G:       u8,
    #[packed_field(bits="56:95")]  pub base_h:  u64,
    #[packed_field(bits="104:111")] _r104: ReservedZero<packed_bits::Bits8>,
}

#[derive(Debug, Default, Clone, Copy, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="8", endian="msb")]
pub struct SegDesc {
    #[packed_field(bits="0:15")]   pub limit_l: u16,
    #[packed_field(bits="16:39")]  pub base_l:  u32,
    #[packed_field(bits="40:43")]  pub Type:    u8,
    #[packed_field(bits="44")]     pub S:       ReservedOne<packed_bits::Bits1>,
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
bitflags! { pub struct CodeDescFlag: u8 {
    const A   = 0b00000001;
    const R   = 0b00000010;
    const C   = 0b00000100;
} }

impl<'a> super::Exec<'a> {
    pub fn check_codeseg(&self, sel: u16) -> Result<bool, EmuException> {
        let ty = self.get_desctype(sel).unwrap();
        let cd = if let Some(DescType::Segment(SegDescType::Code)) = ty { true } else { false };
        Ok(cd)
    }

    fn get_desctype(&self, sel: u16) -> Result<Option<DescType>, EmuException> {
        let core = &self.ac.core;
        let (dt_base, _) = if sel>>2 == 1 { &core.dtregs.ldtr.cache } else { &core.dtregs.gdtr }.get();
        let dt_index = sel & 0xfff8;

        let desc: Desc = Default::default();
        self.ac.read_data_l(&desc as *const _ as *mut _, dt_base + dt_index as u64, std::mem::size_of::<Desc>())?;
        Ok(desc.get_type())
    }

    fn set_gdtr(&mut self, base: u64, limit: u16) -> Result<(), EmuException> {
        let gdtr = &mut self.ac.core.dtregs.gdtr;
        gdtr.base = base;
        gdtr.limit = limit as u32;
        Ok(())
    }

    fn set_idtr(&mut self, base: u64, limit: u16) -> Result<(), EmuException> {
        let idtr = &mut self.ac.core.dtregs.idtr;
        idtr.base = base;
        idtr.limit = limit as u32;
        Ok(())
    }

    fn get_ldtr(&self) -> Result<u16, EmuException> {
        Ok(self.ac.core.dtregs.ldtr.selector)
    }

    fn set_ldtr(&mut self, sel: u16) -> Result<(), EmuException> {
        let (gdt_base, gdt_limit)  = self.ac.core.dtregs.gdtr.get();

        if sel > gdt_limit as u16 { return Err(EmuException::CPUException(CPUException::GP)) }

        let ldt: SysDesc = Default::default();
        self.ac.read_data_l(&ldt as *const _ as *mut _, gdt_base + sel as u64, std::mem::size_of::<SysDesc>())?;

        let ldtr = &mut self.ac.core.dtregs.ldtr;
        ldtr.cache.base  = (ldt.base_h << 24) + ldt.base_l as u64; 
        ldtr.cache.limit = ((ldt.limit_h as u32) << 16) + ldt.limit_l as u32; 
        ldtr.selector    = sel;

        Ok(())
    }

    fn get_tr(&self) -> Result<u16, EmuException> {
        Ok(self.ac.core.dtregs.tr.selector)
    }

    fn set_tr(&mut self, sel: u16) -> Result<(), EmuException> {
        let (gdt_base, gdt_limit)  = self.ac.core.dtregs.gdtr.get();

        if sel > gdt_limit as u16{ return Err(EmuException::CPUException(CPUException::GP)) }

        let tss: SysDesc = Default::default();
        self.ac.read_data_l(&tss as *const _ as *mut _, gdt_base + sel as u64, std::mem::size_of::<SysDesc>())?;

        let tr = &mut self.ac.core.dtregs.tr;
        tr.cache.base  = (tss.base_h << 24) + tss.base_l as u64; 
        tr.cache.limit = ((tss.limit_h as u32) << 16) + tss.limit_l as u32; 
        tr.selector    = sel;

        Ok(())
    }

    pub fn get_segment(&self, reg: segment::SgReg) -> Result<u16, EmuException> {
        Ok(self.ac.core.sgregs.get(reg).selector.to_u16())
    }

    pub fn set_segment(&mut self, reg: segment::SgReg, sel: u16) -> Result<(), EmuException> {
        let core = &mut self.ac.core;
        let sgreg = core.sgregs.get_mut(reg);

        sgreg.selector.from_u16(sel);

        match self.ac.mode {
            access::CpuMode::Real => {
                sgreg.cache.base = (sel as u64) << 4;
            }
            access::CpuMode::Long | access::CpuMode::Protected => {
                let (dt_base, dt_limit) = if sgreg.selector.TI == 1 { &core.dtregs.ldtr.cache } else { &core.dtregs.gdtr }.get();
                let dt_index = (sgreg.selector.IDX as u32) << 3;

                if (reg == segment::SgReg::CS || reg == segment::SgReg::CS) && dt_index == 0 { return Err(EmuException::CPUException(CPUException::GP)) }
                if dt_index > dt_limit { return Err(EmuException::CPUException(CPUException::GP)) }

                let seg: SegDesc = Default::default();
                self.ac.read_data_l(&seg as *const _ as *mut _, dt_base + dt_index as u64, std::mem::size_of::<SegDesc>())?;

                let cache = &mut self.ac.core.sgregs.get_mut(reg).cache;
                cache.base  = ((seg.base_h as u64) << 24) + seg.base_l as u64; 
                cache.limit = ((seg.limit_h as u32) << 16) + seg.limit_l as u32; 
                cache.Type  = seg.Type;
                cache.DPL   = seg.DPL;
                cache.P     = seg.P;
                cache.AVL   = seg.AVL;
                cache.L     = seg.L;
                cache.DB    = seg.DB;
                cache.G     = seg.G;
            }
        }
        Ok(())
    }
}