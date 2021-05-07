#![allow(non_snake_case)]

use std::convert::TryFrom;
use num_enum::TryFromPrimitive;
use packed_struct::prelude::*;
use crate::emulator::*;
use crate::emulator::access::register::*;
use crate::hardware::processor::segment::{SgDescSelector, SgDescCache};
use crate::hardware::processor::descriptor::{DescTbl, DescTblSel};

#[derive(TryFromPrimitive)] #[repr(u8)]
pub enum SysTypes { TSSAvl=1, LDT=2, TSSBsy=3, Call=4, Task=5, Intr=6, Trap=7 }

#[derive(Debug)]
pub enum DescType { System(SysDescType), Segment(SegDescType) }
#[derive(Debug)]
pub enum SysDescType { TSS(TSSDesc), LDT(LDTDesc), Call(CallGateDesc), Task(TaskGateDesc), Intr(IntrTrapGateDesc), Trap(IntrTrapGateDesc) }
#[derive(Debug)]
pub enum SegDescType { Data(SegDesc), Code(SegDesc) }

#[derive(Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="16", endian="msb")]
pub struct Desc {
    #[packed_field(bits="40:43")]  pub Type:    u8,
    #[packed_field(bits="44")]     pub S:       u8,
}

#[derive(Default, PackedStruct, Debug)]
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
            base  : ((desc.base_h as u8 as u64) << 24) + desc.base_l as u64,
            limit : ((desc.limit_h as u32) << 16) + desc.limit_l as u32,
            Type  : desc.Type,
            DPL   : desc.DPL,
            P     : desc.P,
            AVL   : desc.AVL,
            L     : desc.L,
            DB    : desc.DB,
            G     : desc.G,
        }
    }
}

#[derive(Default, PackedStruct, Debug)]
#[packed_struct(bit_numbering="lsb0", size_bytes="16", endian="msb")]
pub struct TSSDesc {
    #[packed_field(bits="0:15")]   pub limit_l: u16,
    #[packed_field(bits="16:39")]  pub base_l:  u32,
    #[packed_field(bits="41")]     pub B:       u8,
    #[packed_field(bits="43")]     pub D:       u8,
    #[packed_field(bits="45:46")]  pub DPL:     u8,
    #[packed_field(bits="47")]     pub P:       u8,
    #[packed_field(bits="48:51")]  pub limit_h: u8,
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

#[derive(Default, PackedStruct, Debug)]
#[packed_struct(bit_numbering="lsb0", size_bytes="16", endian="msb")]
pub struct LDTDesc {
    #[packed_field(bits="0:15")]   pub limit_l: u16,
    #[packed_field(bits="16:39")]  pub base_l:  u32,
    #[packed_field(bits="47")]     pub P:       u8,
    #[packed_field(bits="48:51")]  pub limit_h: u8,
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

#[derive(Default, PackedStruct, Debug)]
#[packed_struct(bit_numbering="lsb0", size_bytes="16", endian="msb")]
pub struct CallGateDesc {
    #[packed_field(bits="0:15")]   pub offset_l:u16,
    #[packed_field(bits="16:31")]  pub selector:u16,
    #[packed_field(bits="32:39")]  pub pc:      u8,
    #[packed_field(bits="45:46")]  pub DPL:     u8,
    #[packed_field(bits="47")]     pub P:       u8,
    #[packed_field(bits="48:63")]  pub offset_h:u16,
}

#[derive(Default, PackedStruct, Debug)]
#[packed_struct(bit_numbering="lsb0", size_bytes="16", endian="msb")]
pub struct TaskGateDesc {
    #[packed_field(bits="16:31")]  pub tss_sel: u16,
    #[packed_field(bits="45:46")]  pub DPL:     u8,
    #[packed_field(bits="47")]     pub P:       u8,
}

#[derive(Default, PackedStruct, Debug)]
#[packed_struct(bit_numbering="lsb0", size_bytes="16", endian="msb")]
pub struct IntrTrapGateDesc {
    #[packed_field(bits="0:15")]   pub offset_l:u16,
    #[packed_field(bits="16:31")]  pub selector:u16,
    #[packed_field(bits="43")]     pub D:       u8,
    #[packed_field(bits="45:46")]  pub DPL:     u8,
    #[packed_field(bits="47")]     pub P:       u8,
    #[packed_field(bits="48:63")]  pub offset_h:u16,
}

#[derive(Default)]
pub struct IVT {
    pub offset: u16,
    pub segment: u16,
}

pub enum TSMode { Jmp, CallInt, Iret }

#[derive(Default, Debug)]
#[repr(C)]
pub struct TSS16 {
    pub prev_task: u16,
    pub sp0: u16,
    pub ss0: u16,
    pub sp1: u16,
    pub ss1: u16,
    pub sp2: u16,
    pub ss2: u16,
    pub ip: u16,
    pub flags: u16,
    pub ax: u16,
    pub cx: u16,
    pub dx: u16,
    pub bx: u16,
    pub sp: u16,
    pub bp: u16,
    pub si: u16,
    pub di: u16,
    pub es: u16,
    pub cs: u16,
    pub ss: u16,
    pub ds: u16,
    pub ldtr: u16,
}

#[derive(Default, Debug)]
#[repr(C)]
pub struct TSS32 {
    pub prev_task: u16,
    pub esp0: u32,
    pub ss0: u16,
    pub esp1: u32,
    pub ss1: u16,
    pub esp2: u32,
    pub ss2: u16,
    pub cr3: u32,
    pub eip: u32,
    pub eflags: u32,
    pub eax: u32,
    pub ecx: u32,
    pub edx: u32,
    pub ebx: u32,
    pub esp: u32,
    pub ebp: u32,
    pub esi: u32,
    pub edi: u32,
    pub es: u16,
    _r0: u16,
    pub cs: u16,
    _r1: u16,
    pub ss: u16,
    _r2: u16,
    pub ds: u16,
    _r3: u16,
    pub fs: u16,
    _r4: u16,
    pub gs: u16,
    _r5: u16,
    pub ldtr: u16,
    _r6: u16,
    pub T: u8,
    pub io_base: u16,
}

const TSS16_SIZE: usize = std::mem::size_of::<TSS16>();
const TSS32_SIZE: usize = std::mem::size_of::<TSS32>();

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
            debug!("{:x?}", sysdsc);
            return Some(DescType::System(sysdsc));
        }
    } else {         // segment
        let sg = SegDesc::unpack(&raw).unwrap();
        let segdsc = if desc.Type & 8 == 0 { SegDescType::Data(sg) } else { SegDescType::Code(sg) };
        debug!("{:x?}", segdsc);
        return Some(DescType::Segment(segdsc));
    }
    None
}

impl super::Access {
    pub fn get_cpl(&self) -> Result<u8, EmuException> {
        Ok(self.core.sgregs.get(SgReg::CS).selector.RPL)
    }

    pub fn obtain_gl_desc(&self, sel: u16) -> Result<Option<DescType>, EmuException> {
        let core = &self.core;

        let selector = SgDescSelector::new(sel);
        let dt_index = (selector.IDX as u32) << 3;
        if dt_index == 0 { return Ok(None); }

        let (dt_base, dt_limit) = if selector.TI == 1 { &core.dtregs.ldtr.cache } else { &core.dtregs.gdtr }.get();
        if dt_index > dt_limit { return Err(EmuException::CPUException(CPUException::GP)) }
        self.obtain_descriptor(dt_base + dt_index as u64)
    }

    pub fn obtain_g_desc(&self, sel: u16) -> Result<Option<DescType>, EmuException> {
        let selector = SgDescSelector::new(sel);
        let dt_index = (selector.IDX as u32) << 3;
        if dt_index == 0 { return Ok(None); }

        let (dt_base, dt_limit) = self.core.dtregs.gdtr.get();
        if dt_index > dt_limit { return Err(EmuException::CPUException(CPUException::GP)) }
        self.obtain_descriptor(dt_base + dt_index as u64)
    }

    pub fn obtain_i_desc(&self, idx: u8) -> Result<Option<DescType>, EmuException> {
        let dt_index = (idx as u32) << 3;
        let (dt_base, dt_limit) = self.core.dtregs.idtr.get();
        if dt_index > dt_limit { return Err(EmuException::CPUException(CPUException::GP)) }
        self.obtain_descriptor(dt_base + dt_index as u64)
    }

    fn obtain_descriptor(&self, desc_addr: u64) -> Result<Option<DescType>, EmuException> {
        let mut raw: [u8;16] = [0;16];
        let dsize = if let (access::CpuMode::Long, access::AcsSize::BIT64) = (&self.mode, &self.oasz.ad) { 16 } else { 8 };
        self.read_l(raw.as_mut_ptr() as *mut _, desc_addr, dsize)?;
        raw.reverse();

        Ok(classify_descriptor(&raw))
    }

    fn install_g_desc(&mut self, sel: u16, desc: DescType) -> Result<(), EmuException> {
        let selector = SgDescSelector::new(sel);
        let dt_index = (selector.IDX as u32) << 3;

        let (dt_base, dt_limit) = self.core.dtregs.gdtr.get();
        if dt_index > dt_limit { return Err(EmuException::CPUException(CPUException::GP)) }
        self.install_descriptor(dt_base + dt_index as u64, desc)
    }

    fn install_descriptor(&mut self, desc_addr: u64, desc: DescType) -> Result<(), EmuException> {
        let mut raw = match desc {
            DescType::System(sysdsc) => {
                match sysdsc {
                    SysDescType::TSS(d) => TSSDesc::pack(&d),
                    SysDescType::LDT(d) => LDTDesc::pack(&d),
                    SysDescType::Call(d) => CallGateDesc::pack(&d),
                    SysDescType::Task(d) => TaskGateDesc::pack(&d),
                    SysDescType::Intr(d) | SysDescType::Trap(d) => IntrTrapGateDesc::pack(&d),
                }
            },
            DescType::Segment(SegDescType::Code(d)) | DescType::Segment(SegDescType::Data(d)) => SegDesc::pack(&d),
        }.unwrap();
        raw.reverse();
        self.write_l(desc_addr, raw.as_ptr() as *const _, std::mem::size_of_val(&raw))?;

        Ok(())
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
                let desc = match self.obtain_gl_desc(sel)? {
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

        match self.obtain_gl_desc(desc.selector)? {
            Some(DescType::Segment(SegDescType::Code(cdesc))) => Ok((desc.selector, cdesc)),
            _ => Err(EmuException::CPUException(CPUException::GP)),
        }
    }

    pub fn select_taskgate(&mut self, desc: TaskGateDesc) -> Result<TSSDesc, EmuException> {
        if desc.P == 0 { return Err(EmuException::CPUException(CPUException::NP)); }
        if SgDescSelector::new(desc.tss_sel).TI == 1 { return Err(EmuException::CPUException(CPUException::GP)); }

        match self.obtain_g_desc(desc.tss_sel)? {
            Some(DescType::System(SysDescType::TSS(tssdesc))) => Ok(tssdesc),
            _ => Err(EmuException::CPUException(CPUException::GP)),
        }
    }

    pub fn select_intrtrapgate(&mut self, desc: IntrTrapGateDesc) -> Result<(u16, SegDesc), EmuException> {
        if desc.P == 0 { return Err(EmuException::CPUException(CPUException::NP)); }

        match self.obtain_gl_desc(desc.selector)? {
            Some(DescType::Segment(SegDescType::Code(cdesc))) => Ok((desc.selector, cdesc)),
            _ => Err(EmuException::CPUException(CPUException::GP)),
        }
    }


    pub fn save_regs(&mut self, size: access::AcsSize, new_pl: Option<u8>) -> Result<(), EmuException> {
        let cs_sel = self.get_sgreg(SgReg::CS)?.0;

        if let Some(pl) = new_pl {
            let tssd = self.core.dtregs.tr.cache;
            let old_rsp = self.get_gpreg(GpReg64::RSP)?;
            let old_ss = self.get_sgreg(SgReg::SS)?.0;

            match size {
                access::AcsSize::BIT16 => {
                    if (tssd.limit as usize) < TSS16_SIZE-1 { return Err(EmuException::CPUException(CPUException::TS)); }

                    let mut tss: TSS16 = Default::default();
                    self.read_l(&mut tss as *mut TSS16 as *mut _, tssd.base, TSS16_SIZE)?;

                    let (ss, sp) = match pl {
                        0 => { (tss.ss0, tss.sp0) },
                        1 => { (tss.ss1, tss.sp1) },
                        2 => { (tss.ss2, tss.sp2) },
                        _ => { return Err(EmuException::UnexpectedError); }
                    };
                    self.load_segment(SgReg::SS, ss)?;
                    self.set_gpreg(GpReg16::SP, sp)?;

                    self.push_u16(old_ss)?;
                    self.push_u16(old_rsp as u16)?;
                },
                access::AcsSize::BIT32 => {
                    if (tssd.limit as usize) < TSS32_SIZE-1 { return Err(EmuException::CPUException(CPUException::TS)); }

                    let mut tss: TSS32 = Default::default();
                    self.read_l(&mut tss as *mut TSS32 as *mut _, tssd.base, TSS32_SIZE)?;

                    let (ss, esp) = match pl {
                        0 => { (tss.ss0, tss.esp0) },
                        1 => { (tss.ss1, tss.esp1) },
                        2 => { (tss.ss2, tss.esp2) },
                        _ => { return Err(EmuException::UnexpectedError); }
                    };
                    self.load_segment(SgReg::SS, ss)?;
                    self.set_gpreg(GpReg32::ESP, esp)?;

                    self.push_u32(old_ss as u32)?;
                    self.push_u32(old_rsp as u32)?;
                },
                access::AcsSize::BIT64 => {
                    self.push_u64(old_rsp)?;
                    return Err(EmuException::NotImplementedFunction);
                },
            }
        }

        match (&self.mode, size) {
            (access::CpuMode::Real, access::AcsSize::BIT16) | (access::CpuMode::Protected, access::AcsSize::BIT16) => {
                self.push_u16(self.get_rflags()? as u16)?;
                self.push_u16(cs_sel)?;
                self.push_u16(self.get_ip()?)?;
            },
            (access::CpuMode::Protected, access::AcsSize::BIT32) => {
                self.push_u32(self.get_rflags()? as u32)?;
                self.push_u32(cs_sel as u32)?;
                self.push_u32(self.get_ip()?)?;
            },
            (access::CpuMode::Long, access::AcsSize::BIT64) => {
                self.push_u64(self.get_rflags()?)?;
                self.push_u64(cs_sel as u64)?;
                self.push_u64(self.get_ip()?)?;
            },
            _ => { return Err(EmuException::CPUException(CPUException::GP)); },
        }
        Ok(())
    }

    pub fn switch_task(&mut self, mode: TSMode, new_sel: u16, desc: TSSDesc) -> Result<(), EmuException> {
        let old_sel  = self.core.dtregs.tr.selector;
        let old_tssd = self.core.dtregs.tr.cache;

        if desc.P == 0 { return Err(EmuException::CPUException(CPUException::NP)); }
        match (&mode, desc.B) {
            (TSMode::Jmp, 1) | (TSMode::CallInt, 1) => { return Err(EmuException::CPUException(CPUException::GP)) },
            (TSMode::Iret, 0)                       => { return Err(EmuException::CPUException(CPUException::TS)) },
            _ => {},
        }
        if let TSMode::Iret = &mode { self.core.rflags.set_nesttask(false); }

        let d = desc.D;
        let new_tssd = DescTbl::from(desc);

        debug!("TaskSwitch");
        match (&self.mode, d) {
            (access::CpuMode::Protected, 0) => {
                // 16bit tss
                if (new_tssd.limit as usize) < TSS16_SIZE-1 { return Err(EmuException::CPUException(CPUException::TS)); }

                let mut tss: TSS16 = Default::default();
                self.read_l(&mut tss as *mut TSS16 as *mut _, old_tssd.base, TSS16_SIZE)?;
                tss.prev_task = old_sel;
                tss.ip     = self.get_ip()?;
                tss.flags  = self.get_rflags()? as u16;
                tss.ax     = self.get_gpreg(GpReg16::AX)?;
                tss.cx     = self.get_gpreg(GpReg16::CX)?;
                tss.dx     = self.get_gpreg(GpReg16::DX)?;
                tss.bx     = self.get_gpreg(GpReg16::BX)?;
                tss.sp     = self.get_gpreg(GpReg16::SP)?;
                tss.bp     = self.get_gpreg(GpReg16::BP)?;
                tss.si     = self.get_gpreg(GpReg16::SI)?;
                tss.di     = self.get_gpreg(GpReg16::DI)?;
                tss.es     = self.get_sgreg(SgReg::ES)?.0;
                tss.cs     = self.get_sgreg(SgReg::CS)?.0;
                tss.ss     = self.get_sgreg(SgReg::SS)?.0;
                tss.ds     = self.get_sgreg(SgReg::DS)?.0;
                tss.ldtr   = self.get_ldtr()?;
                debug!("From: {:x?}", tss);
                self.write_l(old_tssd.base, &tss as *const TSS16 as *const _, TSS16_SIZE)?;

                self.read_l(&mut tss as *mut TSS16 as *mut _, new_tssd.base, TSS16_SIZE)?;
                debug!("To: {:x?}", tss);
                self.set_ip(tss.ip)?;
                self.set_rflags(tss.flags as u64)?;
                self.set_gpreg(GpReg16::AX, tss.ax)?;
                self.set_gpreg(GpReg16::CX, tss.cx)?;
                self.set_gpreg(GpReg16::DX, tss.dx)?;
                self.set_gpreg(GpReg16::BX, tss.bx)?;
                self.set_gpreg(GpReg16::SP, tss.sp)?;
                self.set_gpreg(GpReg16::BP, tss.bp)?;
                self.set_gpreg(GpReg16::SI, tss.si)?;
                self.set_gpreg(GpReg16::DI, tss.di)?;
                self.load_segment(SgReg::ES, tss.es)?;
                self.load_segment(SgReg::CS, tss.cs)?;
                self.load_segment(SgReg::SS, tss.ss)?;
                self.load_segment(SgReg::DS, tss.ds)?;
                self.set_ldtr(tss.ldtr)?;

                if let TSMode::CallInt = &mode {
                    tss.prev_task = old_sel;
                    self.write_l(new_tssd.base, &tss as *const TSS16 as *const _, TSS16_SIZE)?;
                }
            },
            (access::CpuMode::Protected, 1) => {
                // 32bit tss
                if (new_tssd.limit as usize) < TSS32_SIZE-1 { return Err(EmuException::CPUException(CPUException::TS)); }

                let mut tss: TSS32 = Default::default();
                self.read_l(&mut tss as *mut TSS32 as *mut _, old_tssd.base, TSS32_SIZE)?;
                tss.prev_task = old_sel;
                tss.cr3    = self.get_creg(3)?;
                tss.eip    = self.get_ip()?;
                tss.eflags = self.get_rflags()? as u32;
                tss.eax    = self.get_gpreg(GpReg32::EAX)?;
                tss.ecx    = self.get_gpreg(GpReg32::ECX)?;
                tss.edx    = self.get_gpreg(GpReg32::EDX)?;
                tss.ebx    = self.get_gpreg(GpReg32::EBX)?;
                tss.esp    = self.get_gpreg(GpReg32::ESP)?;
                tss.ebp    = self.get_gpreg(GpReg32::EBP)?;
                tss.esi    = self.get_gpreg(GpReg32::ESI)?;
                tss.edi    = self.get_gpreg(GpReg32::EDI)?;
                tss.es     = self.get_sgreg(SgReg::ES)?.0;
                tss.cs     = self.get_sgreg(SgReg::CS)?.0;
                tss.ss     = self.get_sgreg(SgReg::SS)?.0;
                tss.ds     = self.get_sgreg(SgReg::DS)?.0;
                tss.fs     = self.get_sgreg(SgReg::FS)?.0;
                tss.gs     = self.get_sgreg(SgReg::GS)?.0;
                tss.ldtr   = self.get_ldtr()?;
                debug!("From: {:x?}", tss);
                self.write_l(old_tssd.base, &tss as *const TSS32 as *const _, TSS32_SIZE)?;

                self.read_l(&mut tss as *mut TSS32 as *mut _, new_tssd.base, TSS32_SIZE)?;
                debug!("To: {:x?}", tss);
                self.set_creg(3, tss.cr3)?;
                self.set_ip(tss.eip)?;
                self.set_rflags(tss.eflags as u64)?;
                self.set_gpreg(GpReg32::EAX, tss.eax)?;
                self.set_gpreg(GpReg32::ECX, tss.ecx)?;
                self.set_gpreg(GpReg32::EDX, tss.edx)?;
                self.set_gpreg(GpReg32::EBX, tss.ebx)?;
                self.set_gpreg(GpReg32::ESP, tss.esp)?;
                self.set_gpreg(GpReg32::EBP, tss.ebp)?;
                self.set_gpreg(GpReg32::ESI, tss.esi)?;
                self.set_gpreg(GpReg32::EDI, tss.edi)?;
                self.load_segment(SgReg::ES, tss.es)?;
                self.load_segment(SgReg::CS, tss.cs)?;
                self.load_segment(SgReg::SS, tss.ss)?;
                self.load_segment(SgReg::DS, tss.ds)?;
                self.load_segment(SgReg::FS, tss.fs)?;
                self.load_segment(SgReg::GS, tss.gs)?;
                self.set_ldtr(tss.ldtr)?;

                if let TSMode::CallInt = &mode {
                    tss.prev_task = old_sel;
                    self.write_l(new_tssd.base, &tss as *const TSS32 as *const _, TSS32_SIZE)?;
                }
            },
            (access::CpuMode::Long, 1) => {
                // 64bit tss
                return Err(EmuException::NotImplementedFunction);
            },
            (access::CpuMode::Long, 0) => { return Err(EmuException::CPUException(CPUException::TS)); },
            _ => { return Err(EmuException::UnexpectedError); },
        }

        match &mode {
            TSMode::Jmp => {
                self.set_busy_tssdesc(new_sel, true)?;
                self.set_busy_tssdesc(old_sel, false)?;
            },
            TSMode::CallInt => {
                self.core.rflags.set_nesttask(true);
                self.set_busy_tssdesc(new_sel, true)?;
            },
            TSMode::Iret => {
                self.set_busy_tssdesc(old_sel, false)?;
            },
        }

        self.core.dtregs.tr = DescTblSel { selector: new_sel, cache: new_tssd, };
        self.core.cregs.0.TS = 1;
        Ok(())
    }

    pub fn restore_task(&mut self) -> Result<(), EmuException> {
        let old_tssd = self.core.dtregs.tr.cache;

        let mut prev_task: u16 = 0;
        self.read_l(&mut prev_task as *mut u16 as *mut _, old_tssd.base, std::mem::size_of_val(&prev_task))?;

        if let Some(DescType::System(SysDescType::TSS(tssdesc))) = self.obtain_g_desc(prev_task)? {
            self.switch_task(TSMode::Iret, prev_task, tssdesc)
        } else {
            Err(EmuException::CPUException(CPUException::TS))
        }
    }

    fn set_busy_tssdesc(&mut self, sel: u16, busy: bool) -> Result<(), EmuException> {
        if let Some(DescType::System(SysDescType::TSS(mut tssdesc))) = self.obtain_g_desc(sel)? {
            tssdesc.B = busy as u8;
            self.install_g_desc(sel, DescType::System(SysDescType::TSS(tssdesc)))
        } else {
            Err(EmuException::UnexpectedError)
        }
    }
}