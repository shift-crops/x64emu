use std::collections::BTreeMap;
use libc::c_void;
use packed_struct::prelude::*;
use crate::emulator::*;
use super::register::*;
use crate::hardware::memory::MemDumpSize;

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="8", endian="msb")]
pub struct PML5E {
    #[packed_field(bits="0")]  P:   bool,
    #[packed_field(bits="1")]  RW:  bool,
    #[packed_field(bits="2")]  US:  bool,
    #[packed_field(bits="3")]  PWT: bool,
    #[packed_field(bits="4")]  PCD: bool,
    #[packed_field(bits="5")]  A:   bool,
    #[packed_field(bits="12:39")] pml4_base: u32,
    #[packed_field(bits="63")] XD:  bool,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="8", endian="msb")]
pub struct PML4E {
    #[packed_field(bits="0")]  P:   bool,
    #[packed_field(bits="1")]  RW:  bool,
    #[packed_field(bits="2")]  US:  bool,
    #[packed_field(bits="3")]  PWT: bool,
    #[packed_field(bits="4")]  PCD: bool,
    #[packed_field(bits="5")]  A:   bool,
    #[packed_field(bits="12:39")] pdpt_base: u32,
    #[packed_field(bits="63")] XD:  bool,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="8", endian="msb")]
pub struct PDPTE {
    #[packed_field(bits="0")]  P:   bool,
    #[packed_field(bits="1")]  RW:  bool,
    #[packed_field(bits="2")]  US:  bool,
    #[packed_field(bits="3")]  PWT: bool,
    #[packed_field(bits="4")]  PCD: bool,
    #[packed_field(bits="5")]  A:   bool,
    #[packed_field(bits="6")]  D:   bool,
    #[packed_field(bits="7")]  PS:  bool,
    #[packed_field(bits="8")]  G:   bool,
    #[packed_field(bits="12:39")] pdt_base: u32,
    #[packed_field(bits="63")] XD:  bool,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="8", endian="msb")]
pub struct PDE {
    #[packed_field(bits="0")]  P:   bool,
    #[packed_field(bits="1")]  RW:  bool,
    #[packed_field(bits="2")]  US:  bool,
    #[packed_field(bits="3")]  PWT: bool,
    #[packed_field(bits="4")]  PCD: bool,
    #[packed_field(bits="5")]  A:   bool,
    #[packed_field(bits="6")]  D:   bool,
    #[packed_field(bits="7")]  PS:  bool,
    #[packed_field(bits="8")]  G:   bool,
    #[packed_field(bits="12:39")] pt_base: u32,
    #[packed_field(bits="63")] XD:  bool,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="8", endian="msb")]
pub struct PTE {
    #[packed_field(bits="0")]  P:   bool,
    #[packed_field(bits="1")]  RW:  bool,
    #[packed_field(bits="2")]  US:  bool,
    #[packed_field(bits="3")]  PWT: bool,
    #[packed_field(bits="4")]  PCD: bool,
    #[packed_field(bits="5")]  A:   bool,
    #[packed_field(bits="6")]  D:   bool,
    #[packed_field(bits="7")]  PS:  bool,
    #[packed_field(bits="8")]  G:   bool,
    #[packed_field(bits="12:39")] page_base: u32,
    #[packed_field(bits="63")] XD:  bool,
}

#[derive(Debug, Default, Clone, Copy)]
struct PageCache {
    RW:  bool,
    US:  bool,
    PWT: bool,
    PCD: bool,
    G:   bool,
    base: u64,
    XD:  bool,
}
impl From<&PDPTE> for PageCache {
    fn from(e: &PDPTE) -> Self {
        Self { RW: e.RW, US: e.US, PWT: e.PWT, PCD: e.PCD, G: e.G, base: (e.pdt_base as u64) << 12, XD: e.XD, }
    }
}
impl From<&PDE> for PageCache {
    fn from(e: &PDE) -> Self {
        Self { RW: e.RW, US: e.US, PWT: e.PWT, PCD: e.PCD, G: e.G, base: (e.pt_base as u64) << 12, XD: e.XD, }
    }
}
impl From<&PTE> for PageCache {
    fn from(e: &PTE) -> Self {
        Self { RW: e.RW, US: e.US, PWT: e.PWT, PCD: e.PCD, G: e.G, base: (e.page_base as u64) << 12, XD: e.XD, }
    }
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="4", endian="msb")]
pub struct LAddrLegacy {
    #[packed_field(bits="0:11")]  p_ofs:    u16,
    #[packed_field(bits="12:21")] pt_ofs:   u16,
    #[packed_field(bits="22:31")] pd_ofs:   u16,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="4", endian="msb")]
pub struct LAddrPAE {
    #[packed_field(bits="0:11")]  p_ofs:    u16,
    #[packed_field(bits="12:20")] pt_ofs:   u16,
    #[packed_field(bits="21:29")] pd_ofs:   u16,
    #[packed_field(bits="30:31")] pdpt_ofs: u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="8", endian="msb")]
pub struct LAddrIa32e {
    #[packed_field(bits="0:11")]  p_ofs:    u16,
    #[packed_field(bits="12:20")] pt_ofs:   u16,
    #[packed_field(bits="21:29")] pd_ofs:   u16,
    #[packed_field(bits="30:38")] pdpt_ofs: u16,
    #[packed_field(bits="39:47")] pml4_ofs: u16,
    #[packed_field(bits="48:56")] pml5_ofs: u16,
}

#[derive(Debug, Default)]
struct PagingStructIndex {
    legacy: bool,
    pml5: Option<u64>,
    pml4: Option<u64>,
    pdpt: Option<u64>,
    pd:   u64,
    pt:   u64,
}
impl From<&LAddrLegacy> for PagingStructIndex {
    fn from(l: &LAddrLegacy) -> Self {
        Self { legacy: true, pml5: None, pml4: None, pdpt: None, pd: l.pd_ofs as u64, pt: l.pt_ofs as u64 }
    }
}
impl From<&LAddrPAE> for PagingStructIndex {
    fn from(l: &LAddrPAE) -> Self {
        Self { legacy: false, pml5: None, pml4: None, pdpt: Some(l.pdpt_ofs as u64), pd: l.pd_ofs as u64, pt: l.pt_ofs as u64 }
    }
}
impl From<&LAddrIa32e> for PagingStructIndex {
    fn from(l: &LAddrIa32e) -> Self {
        Self { legacy: false, pml5: Some(l.pml5_ofs as u64), pml4: Some(l.pml4_ofs as u64), pdpt: Some(l.pdpt_ofs as u64), pd: l.pd_ofs as u64, pt: l.pt_ofs as u64 }
    }
}

#[derive(Debug, Clone, Copy)]
enum PageType {
    Page1GB(PageCache), Page4MB(PageCache), Page2MB(PageCache), Page4KB(PageCache)
}

#[derive(Default)]
pub(super) struct TLB {
    p1gb: BTreeMap<u64, PageCache>,
    p4mb: BTreeMap<u64, PageCache>,
    p2mb: BTreeMap<u64, PageCache>,
    p4kb: BTreeMap<u64, PageCache>,
}

impl TLB {
    pub fn flush(&mut self) -> () {
        self.p1gb.clear();
        self.p4mb.clear();
        self.p2mb.clear();
        self.p4kb.clear();
    }

    fn add_cache(&mut self, vpn: u64, cache: PageType) -> () {
        match cache {
            PageType::Page1GB(tbl) if !tbl.PCD => { self.p1gb.insert(vpn >> 18, tbl); },
            PageType::Page4MB(tbl) if !tbl.PCD => { self.p4mb.insert(vpn >> 10, tbl); },
            PageType::Page2MB(tbl) if !tbl.PCD => { self.p2mb.insert(vpn >> 9, tbl); },
            PageType::Page4KB(tbl) if !tbl.PCD => { self.p4kb.insert(vpn, tbl); },
            _ => {},
        }
    }

    fn find_cache(&self, vpn: u64) -> Option<PageType> {
        if let Some(tbl) = self.p1gb.get(&(vpn >> 18)) {
            Some(PageType::Page1GB(*tbl))
        } else if let Some(tbl) = self.p4mb.get(&(vpn >> 10)){
            Some(PageType::Page4MB(*tbl))
        } else if let Some(tbl) = self.p2mb.get(&(vpn >> 9)){
            Some(PageType::Page2MB(*tbl))
        } else if let Some(tbl) = self.p4kb.get(&vpn){
            Some(PageType::Page4KB(*tbl))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum MemAccessMode { Read, Write, Exec, Monitor }
#[derive(Clone, Copy)]
enum MemAccessSize { Byte = 1, Word = 2, DWord = 4, QWord = 8 }

impl super::Access {
    pub fn get_data8(&self, target: (SgReg, u64)) -> Result<u8, EmuException> { Ok(self.get_data_size(target.0, target.1, MemAccessSize::Byte)? as u8) }
    pub fn get_data16(&self, target: (SgReg, u64)) -> Result<u16, EmuException> { Ok(self.get_data_size(target.0, target.1, MemAccessSize::Word)? as u16) }
    pub fn get_data32(&self, target: (SgReg, u64)) -> Result<u32, EmuException> { Ok(self.get_data_size(target.0, target.1, MemAccessSize::DWord)? as u32) }
    pub fn get_data64(&self, target: (SgReg, u64)) -> Result<u64, EmuException> { Ok(self.get_data_size(target.0, target.1, MemAccessSize::QWord)?) }

    pub fn set_data8(&mut self, target: (SgReg, u64), v: u8) -> Result<(), EmuException> { self.set_data_size(target.0, target.1, v as u64, MemAccessSize::Byte)?; Ok(()) }
    pub fn set_data16(&mut self, target: (SgReg, u64), v: u16) -> Result<(), EmuException> { self.set_data_size(target.0, target.1, v as u64, MemAccessSize::Word)?; Ok(()) }
    pub fn set_data32(&mut self, target: (SgReg, u64), v: u32) -> Result<(), EmuException> { self.set_data_size(target.0, target.1, v as u64, MemAccessSize::DWord)?; Ok(()) }
    pub fn set_data64(&mut self, target: (SgReg, u64), v: u64) -> Result<(), EmuException> { self.set_data_size(target.0, target.1, v, MemAccessSize::QWord)?; Ok(()) }

    pub fn get_code8(&self, index: u64) -> Result<u8, EmuException> { Ok(self.get_code_size(index, MemAccessSize::Byte)? as u8) }
    pub fn get_code16(&self, index: u64) -> Result<u16, EmuException> { Ok(self.get_code_size(index, MemAccessSize::Word)? as u16) }
    pub fn get_code32(&self, index: u64) -> Result<u32, EmuException> { Ok(self.get_code_size(index, MemAccessSize::DWord)? as u32) }
    pub fn get_code64(&self, index: u64) -> Result<u64, EmuException> { Ok(self.get_code_size(index, MemAccessSize::QWord)?) }

    pub fn push_u16(&mut self, v: u16) -> Result<(), EmuException> {
        let sp = self.stack_update(-2)?;
        self.set_data16((SgReg::SS, sp), v)
    }

    pub fn pop_u16(&mut self) -> Result<u16, EmuException> {
        let sp = self.stack_update(2)?;
        self.get_data16((SgReg::SS, sp-2))
    }

    pub fn push_u32(&mut self, v: u32) -> Result<(), EmuException> {
        let esp = self.stack_update(-4)?;
        self.set_data32((SgReg::SS, esp), v)
    }

    pub fn pop_u32(&mut self) -> Result<u32, EmuException> {
        let esp = self.stack_update(4)?;
        self.get_data32((SgReg::SS, esp-4))
    }

    pub fn push_u64(&mut self, v: u64) -> Result<(), EmuException> {
        let rsp = self.stack_update(-8)?;
        self.set_data64((SgReg::SS, rsp), v)
    }

    pub fn pop_u64(&mut self) -> Result<u64, EmuException> {
        let rsp = self.stack_update(8)?;
        self.get_data64((SgReg::SS, rsp-8))
    }

    pub fn read_p(&self, dst: *mut c_void, src_addr: u64, len: usize) -> usize {
        if let Ok(n) = self.mem.read().unwrap().read_data(dst, src_addr as usize, len) { n } else { 0 }
    }

    pub fn write_p(&mut self, dst_addr: u64, src: *const c_void, len: usize) -> usize {
        if let Ok(n) = self.mem.write().unwrap().write_data(dst_addr as usize, src, len) { n } else { 0 }
    }

    pub fn read_l(&self, dst: *mut c_void, src_addr: u64, len: usize) -> Result<usize, EmuException> {
        Ok(self.read_p(dst, self.trans_l2p(MemAccessMode::Read, src_addr)?, len))
    }

    pub fn write_l(&mut self, dst_addr: u64, src: *const c_void, len: usize) -> Result<usize, EmuException> {
        Ok(self.write_p(self.trans_l2p(MemAccessMode::Write, dst_addr)?, src, len))
    }

    pub fn read_v(&self, seg: SgReg, dst: *mut c_void, src_addr: u64, len: usize) -> Result<usize, EmuException> {
        Ok(self.read_p(dst, self.trans_v2p(MemAccessMode::Read, seg, src_addr)?, len))
    }

    pub fn write_v(&mut self, seg: SgReg, dst_addr: u64, src: *const c_void, len: usize) -> Result<usize, EmuException> {
        Ok(self.write_p(self.trans_v2p(MemAccessMode::Write, seg, dst_addr)?, src, len))
    }

    pub fn addr_v2p(&mut self, seg: SgReg, vaddr: u64) -> Result<u64, EmuException> {
        self.trans_v2p(MemAccessMode::Monitor, seg, vaddr)
    }

    pub fn dump_code(&self, unit: MemDumpSize) -> () {
        let addr = self.trans_v2p(MemAccessMode::Read, SgReg::CS, self.get_ip().unwrap()).unwrap();
        self.mem.read().unwrap().dump(addr as usize -0x10, 0x20, unit);
    }

    pub fn dump_stack(&self, unit: MemDumpSize) -> () {
        let addr = self.trans_v2p(MemAccessMode::Read, SgReg::SS, self.get_gpreg(GpReg64::RSP).unwrap()).unwrap();
        self.mem.read().unwrap().dump(addr as usize, 0x40, unit);
    }
}

impl super::Access {
    fn stack_update(&mut self, size: i8) -> Result<u64, EmuException> {
        let sp = match self.stsz {
            super::AcsSize::BIT16 => {
                self.update_gpreg(GpReg16::SP, size as i16)?;
                self.get_gpreg(GpReg16::SP)? as u64
            },
            super::AcsSize::BIT32 => {
                self.update_gpreg(GpReg32::ESP, size as i32)?;
                self.get_gpreg(GpReg32::ESP)? as u64
            },
            super::AcsSize::BIT64 => {
                self.update_gpreg(GpReg64::RSP, size as i64)?;
                self.get_gpreg(GpReg64::RSP)?
            },
        };
        Ok(sp)
    }

    fn get_data_size(&self, sg: SgReg, vaddr: u64, size: MemAccessSize) -> Result<u64, EmuException> {
        let paddr = self.trans_v2p(MemAccessMode::Read, sg, vaddr)?;
        let v = if self.dev.check_memio(paddr, size as u64 - 1) {
            let mut data = vec![0; size as usize];
            self.dev.read_memio(paddr, &mut data);
            let mut dst = [0; 8];
            dst[..size as usize].copy_from_slice(&data);
            u64::from_le_bytes(dst)
        } else {
            let paddr = paddr as usize;
            let mem = self.mem.read().unwrap();
            match size {
                MemAccessSize::Byte  => mem.read8(paddr) as u64,
                MemAccessSize::Word  => mem.read16(paddr) as u64,
                MemAccessSize::DWord => mem.read32(paddr) as u64,
                MemAccessSize::QWord => mem.read64(paddr),
            }
        };
        Ok(v)
    }

    fn set_data_size(&mut self, sg: SgReg, vaddr: u64, v: u64, size: MemAccessSize) -> Result<(), EmuException> {
        let paddr = self.trans_v2p(MemAccessMode::Write, sg, vaddr)?;
        if self.dev.check_memio(paddr, size as u64 - 1) {
            self.dev.write_memio(paddr, &v.to_le_bytes()[..size as usize]);
        } else {
            let paddr = paddr as usize;
            let mut mem = self.mem.write().unwrap();
            match size {
                MemAccessSize::Byte  => mem.write8(paddr, v as u8),
                MemAccessSize::Word  => mem.write16(paddr, v as u16),
                MemAccessSize::DWord => mem.write32(paddr, v as u32),
                MemAccessSize::QWord => mem.write64(paddr, v),
            }
        }
        Ok(())
    }

    fn get_code_size(&self, index: u64, size: MemAccessSize) -> Result<u64, EmuException> {
        let ip: u64 = self.get_ip()?;
        let paddr = self.trans_v2p(MemAccessMode::Exec, SgReg::CS, ip + index)? as usize;
        let mem = self.mem.read().unwrap();
        let v = match size {
            MemAccessSize::Byte  => mem.read8(paddr) as u64,
            MemAccessSize::Word  => mem.read16(paddr) as u64,
            MemAccessSize::DWord => mem.read32(paddr) as u64,
            MemAccessSize::QWord => mem.read64(paddr),
        };
        Ok(v)
    }

    fn trans_v2p(&self, acsmode: MemAccessMode, sg: SgReg, vaddr: u64) -> Result<u64, EmuException> {
        let laddr = self.trans_v2l(acsmode, sg, vaddr)?;
        let paddr = self.trans_l2p(acsmode, laddr)?;

        Ok( if self.a20gate { paddr } else { paddr & (1<<20)-1 } )
    }

    fn trans_v2l(&self, _acsmode: MemAccessMode, sg: SgReg, vaddr: u64) -> Result<u64, EmuException> {
        let long64 = self.mode == super::CpuMode::Long && self.oasz.ad == super::AcsSize::BIT64;

        let cache = self.get_sgreg(sg)?.1;
        let base  = if long64 && !(sg == SgReg::FS || sg == SgReg::GS) { 0 } else { cache.base };

        match (&self.mode, &self.oasz.ad, sg) {
            (super::CpuMode::Long, super::AcsSize::BIT64, SgReg::CS) | (super::CpuMode::Protected, _, _) => {
                /*
                if cache.Type & 0 {

                } else {

                }
                */

                if !long64 && (vaddr >> 12*cache.G) > cache.limit as u64 { return Err(EmuException::CPUException(CPUException::GP(None))); }
            },
            _ => {}
        }

        Ok(base + vaddr)
    }

    fn trans_l2p(&self, acs: MemAccessMode, laddr: u64) -> Result<u64, EmuException> {
        let paddr = if let Some(md) = &self.pgmd {
            let vpn = laddr >> 12;
            let mut tlb = self.tlb.borrow_mut();

            let ptype = if let Some(ty) = tlb.find_cache(vpn) {
                match ty {
                    PageType::Page1GB(e) | PageType::Page4MB(e) | PageType::Page2MB(e) | PageType::Page4KB(e) => {
                        if (acs == MemAccessMode::Write && !e.RW) || (self.get_cpl()? > 2 && !e.US ){
                            return Err(EmuException::CPUException(CPUException::PF(laddr)));
                        }
                    },
                }
                ty
            } else {
                let psidx = match md {
                    super::PagingMode::Legacy => {
                        PagingStructIndex::from(&LAddrLegacy::unpack(&(laddr as u32).to_be_bytes()).unwrap())
                    },
                    super::PagingMode::LegacyPAE => {
                        PagingStructIndex::from(&LAddrPAE::unpack(&(laddr as u32).to_be_bytes()).unwrap())
                    },
                    super::PagingMode::Ia32e4Lv => {
                        if laddr >> 48 != 0 {
                            return Err(EmuException::CPUException(CPUException::GP(None)));
                        }
                        let l = LAddrIa32e::unpack(&laddr.to_be_bytes()).unwrap();
                        let mut psi = PagingStructIndex::from(&l);
                        psi.pml5 = None;
                        psi
                    },
                    super::PagingMode::Ia32e5Lv => {
                        if laddr >> 57 != 0 {
                            return Err(EmuException::CPUException(CPUException::GP(None)));
                        }
                        PagingStructIndex::from(&LAddrIa32e::unpack(&laddr.to_be_bytes()).unwrap())
                    },
                };

                if let Some(p) = self.page_walk(acs, &md, psidx) {
                    debug!("{:x?}", p);
                    tlb.add_cache(vpn, p);
                    p
                } else {
                    return Err(EmuException::CPUException(CPUException::PF(laddr)));
                }
            };

            match ptype {
                PageType::Page1GB(tbl) => { tbl.base + (laddr & ((1<<30)-1)) },
                PageType::Page4MB(tbl) => { tbl.base + (laddr & ((1<<22)-1)) },
                PageType::Page2MB(tbl) => { tbl.base + (laddr & ((1<<21)-1)) },
                PageType::Page4KB(tbl) => { tbl.base + (laddr & ((1<<12)-1)) },
            }
        } else { laddr };
        Ok(paddr)
   }

    fn page_walk(&self, acs: MemAccessMode, pmd: &super::PagingMode, psidx: PagingStructIndex) -> Option<PageType> {
        let cpl = self.get_cpl().ok()?;
        let cr3 = &self.core.cregs.3;
        let mem = self.mem.read().unwrap();

        let pml5e: Option<PML5E> = if let Some(idx) = psidx.pml5 {
            let pml5_base = cr3.get_pagedir_base() as usize;
            Some(PML5E::unpack(&mem.read64(pml5_base + (idx as usize)*8).to_be_bytes()).unwrap())
        } else { None };

        let pml4e: Option<PML4E> = if let Some(idx) = psidx.pml4 {
            let pml4_base = if let Some(e) = pml5e {
                if !e.P || (acs == MemAccessMode::Write && !e.RW) || (cpl > 2 && !e.US) { return None; }
                (e.pml4_base as usize) << 12
            } else { cr3.get_pagedir_base() as usize };
            Some(PML4E::unpack(&mem.read64(pml4_base + (idx as usize)*8).to_be_bytes()).unwrap())
        } else { None };

        let pdpte: Option<PDPTE> = if let Some(idx) = psidx.pdpt {
            let pdpt_base = if let Some(e) = pml4e {
                if !e.P || (acs == MemAccessMode::Write && !e.RW) || (cpl > 2 && !e.US) { return None; }
                (e.pdpt_base as usize) << 12
            } else { cr3.get_pagedir_base() as usize };
            let raw = if let super::PagingMode::Legacy = pmd { mem.read32(pdpt_base + (idx as usize)*4) as u64 } else { mem.read64(pdpt_base + (idx as usize)*8) };
            Some(PDPTE::unpack(&raw.to_be_bytes()).unwrap())
        } else { None };

        let pd_base = if let Some(mut e) = pdpte {
            if e.PS {
                e.pdt_base &= !((1<<18)-1);
                return Some(PageType::Page1GB(PageCache::from(&e)));
            } else {
                if !e.P || (acs == MemAccessMode::Write && !e.RW) || (cpl > 2 && !e.US) { return None; }
                (e.pdt_base as usize) << 12
            }
        } else { cr3.get_pagedir_base() as usize };
        let raw = if let super::PagingMode::Legacy = pmd { mem.read32(pd_base + (psidx.pd as usize)*4) as u64 } else { mem.read64(pd_base + (psidx.pd as usize)*8) };
        let mut pde = PDE::unpack(&raw.to_be_bytes()).unwrap();

        let pt_base = match (psidx.legacy, self.core.cregs.4.PSE, pde.PS) {
            (true, 1, true) => {
                pde.pt_base &= !((1<<10)-1);
                return Some(PageType::Page4MB(PageCache::from(&pde)));
            },
            (false, _, true) => {
                pde.pt_base &= !((1<<9)-1);
                return Some(PageType::Page2MB(PageCache::from(&pde)));
            },
            _ => {
                if !pde.P || (acs == MemAccessMode::Write && !pde.RW) || (cpl > 2 && !pde.US) { return None; }
                (pde.pt_base as usize) << 12
            },
        };
        let raw = if let super::PagingMode::Legacy = pmd { mem.read32(pt_base + (psidx.pt as usize)*4) as u64 } else { mem.read64(pt_base + (psidx.pt as usize)*8) };
        let pte = PTE::unpack(&raw.to_be_bytes()).unwrap();

        if !pte.P || (acs == MemAccessMode::Write && !pte.RW) || (cpl > 2 && !pte.US) { return None; }

        Some(PageType::Page4KB(PageCache::from(&pte)))
    }
}

#[cfg(test)]
#[test]
fn laddr_test() {
    let laddr_32: u32 = 0x44332211;
    let laddr_64: u64 = 0x77665544332211;

    let legacy = LAddrLegacy::unpack(&laddr_32.to_be_bytes()).unwrap();
    assert_eq!(legacy.p_ofs, 0x211);
    assert_eq!(legacy.pt_ofs, 0x332);
    assert_eq!(legacy.pd_ofs, 0x110);

    let pae = LAddrPAE::unpack(&laddr_32.to_be_bytes()).unwrap();
    assert_eq!(pae.p_ofs, 0x211);
    assert_eq!(pae.pt_ofs, 0x132);
    assert_eq!(pae.pd_ofs, 0x21);
    assert_eq!(pae.pdpt_ofs, 0x1);

    let ia32e = LAddrIa32e::unpack(&laddr_64.to_be_bytes()).unwrap();
    assert_eq!(ia32e.p_ofs, 0x211);
    assert_eq!(ia32e.pt_ofs, 0x132);
    assert_eq!(ia32e.pd_ofs, 0x21);
    assert_eq!(ia32e.pdpt_ofs, 0x155);
    assert_eq!(ia32e.pml4_ofs, 0xcc);
    assert_eq!(ia32e.pml5_ofs, 0x77);
}

#[cfg(test)]
#[test]
fn page_table_test() {
    let paddr: u64 = 0xdeadbee000;
    let mut raw = paddr.to_le_bytes();
    raw[0] |= 0x01;
    raw[1] |= 0x01;
    raw[7] |= 0x80;
    raw.reverse();

    let pte = PTE::unpack(&raw).unwrap();
    assert_eq!(pte.P, true);
    assert_eq!(pte.G, true);
    assert_eq!(pte.page_base, 0xdeadbee);
    assert_eq!(pte.XD, true);
}

#[cfg(test)]
#[test]
fn page_walk_legacy_test() {
    let hw = hardware::Hardware::new(0x2000);
    let dev = device::Device::new(std::sync::Arc::clone(&hw.mem));
    let mut ac = super::Access::new(hw, dev);

    ac.pgmd = Some(super::PagingMode::Legacy);
    ac.core.cregs.3.from_u64(0);
    ac.core.cregs.4.PSE = 1;
    {
        let mut mem = ac.mem.write().unwrap();

        let mut pde: PDE = Default::default();
        pde.P = true;
        pde.pt_base = 0x1;
        mem.write32(0 + 4*0x32b, u64::from_be_bytes(pde.pack().unwrap()) as u32);

        let mut pde: PDE = Default::default();
        pde.P = true;
        pde.PS = true;
        pde.pt_base = 0x2fc00;
        mem.write32(0 + 4*0x37a, u64::from_be_bytes(pde.pack().unwrap()) as u32);

        let mut pte: PTE = Default::default();
        pte.P = true;
        pte.page_base = 0x1ffff;
        mem.write32(0x1000 + 4*0x3eb, u64::from_be_bytes(pte.pack().unwrap()) as u32);
    }

    assert_eq!(ac.trans_l2p(MemAccessMode::Read, 0xcafebabe).unwrap(), 0x1ffffabe);
    assert_eq!(ac.trans_l2p(MemAccessMode::Read, 0xdeadbeef).unwrap(), 0x2fedbeef);
}

#[cfg(test)]
#[test]
fn page_walk_pae_test() {
    let hw = hardware::Hardware::new(0x3000);
    let dev = device::Device::new(std::sync::Arc::clone(&hw.mem));
    let mut ac = super::Access::new(hw, dev);

    ac.pgmd = Some(super::PagingMode::LegacyPAE);
    ac.core.cregs.3.from_u64(0);
    {
        let mut mem = ac.mem.write().unwrap();

        let mut pdpte: PDPTE = Default::default();
        pdpte.P = true;
        pdpte.pdt_base = 0x1;
        mem.write64(0 + 8*0x3, u64::from_be_bytes(pdpte.pack().unwrap()));

        let mut pde: PDE = Default::default();
        pde.P = true;
        pde.pt_base = 0x2;
        mem.write64(0x1000 + 8*0x57, u64::from_be_bytes(pde.pack().unwrap()));

        let mut pde: PDE = Default::default();
        pde.P = true;
        pde.PS = true;
        pde.pt_base = 0x2fffc00;
        mem.write64(0x1000 + 8*0xf5, u64::from_be_bytes(pde.pack().unwrap()));

        let mut pte: PTE = Default::default();
        pte.P = true;
        pte.page_base = 0x1ffffff;
        mem.write64(0x2000 + 8*0x1eb, u64::from_be_bytes(pte.pack().unwrap()));
    }

    assert_eq!(ac.trans_l2p(MemAccessMode::Read, 0xcafebabe).unwrap(), 0x1ffffffabe);
    assert_eq!(ac.trans_l2p(MemAccessMode::Read, 0xdeadbeef).unwrap(), 0x2fffcdbeef);
}

#[cfg(test)]
#[test]
fn page_walk_ia32e4l_test() {
    let hw = hardware::Hardware::new(0x5000);
    let dev = device::Device::new(std::sync::Arc::clone(&hw.mem));
    let mut ac = super::Access::new(hw, dev);

    ac.pgmd = Some(super::PagingMode::Ia32e4Lv);
    ac.core.cregs.3.from_u64(0);
    {
        let mut mem = ac.mem.write().unwrap();

        let mut pdpte: PML4E = Default::default();
        pdpte.P = true;
        pdpte.pdpt_base = 0x1;
        mem.write64(0 + 8*0xff, u64::from_be_bytes(pdpte.pack().unwrap()));

        let mut pdpte: PML4E = Default::default();
        pdpte.P = true;
        pdpte.pdpt_base = 0x2;
        mem.write64(0 + 8*0x100, u64::from_be_bytes(pdpte.pack().unwrap()));

        let mut pdpte: PDPTE = Default::default();
        pdpte.P = true;
        pdpte.pdt_base = 0x3;
        mem.write64(0x1000 + 8*0x1c3, u64::from_be_bytes(pdpte.pack().unwrap()));

        let mut pdpte: PDPTE = Default::default();
        pdpte.P = true;
        pdpte.PS = true;
        pdpte.pdt_base = 0x3fc0000;
        mem.write64(0x2000 + 8*0x3, u64::from_be_bytes(pdpte.pack().unwrap()));

        let mut pde: PDE = Default::default();
        pde.P = true;
        pde.pt_base = 0x4;
        mem.write64(0x3000 + 8*0x57, u64::from_be_bytes(pde.pack().unwrap()));

        let mut pde: PDE = Default::default();
        pde.P = true;
        pde.PS = true;
        pde.pt_base = 0x2fffc00;
        mem.write64(0x3000 + 8*0xf5, u64::from_be_bytes(pde.pack().unwrap()));

        let mut pte: PTE = Default::default();
        pte.P = true;
        pte.page_base = 0x1ffffff;
        mem.write64(0x4000 + 8*0x1eb, u64::from_be_bytes(pte.pack().unwrap()));
    }

    assert_eq!(ac.trans_l2p(MemAccessMode::Read, 0x7ff0cafebabe).unwrap(), 0x1ffffffabe);
    assert_eq!(ac.trans_l2p(MemAccessMode::Read, 0x7ff0deadbeef).unwrap(), 0x2fffcdbeef);
    assert_eq!(ac.trans_l2p(MemAccessMode::Read, 0x8000feedc0de).unwrap(), 0x3ffeedc0de);
}

#[cfg(test)]
#[test]
fn page_walk_ia32e5l_test() {
    let hw = hardware::Hardware::new(0x6000);
    let dev = device::Device::new(std::sync::Arc::clone(&hw.mem));
    let mut ac = super::Access::new(hw, dev);

    ac.pgmd = Some(super::PagingMode::Ia32e5Lv);
    ac.core.cregs.3.from_u64(0);
    {
        let mut mem = ac.mem.write().unwrap();

        let mut pdpte: PML5E = Default::default();
        pdpte.P = true;
        pdpte.pml4_base = 0x1;
        mem.write64(0 + 8*0x100, u64::from_be_bytes(pdpte.pack().unwrap()));

        let mut pdpte: PML4E = Default::default();
        pdpte.P = true;
        pdpte.pdpt_base = 0x2;
        mem.write64(0x1000 + 8*0xff, u64::from_be_bytes(pdpte.pack().unwrap()));

        let mut pdpte: PML4E = Default::default();
        pdpte.P = true;
        pdpte.pdpt_base = 0x3;
        mem.write64(0x1000 + 8*0x100, u64::from_be_bytes(pdpte.pack().unwrap()));

        let mut pdpte: PDPTE = Default::default();
        pdpte.P = true;
        pdpte.pdt_base = 0x4;
        mem.write64(0x2000 + 8*0x1c3, u64::from_be_bytes(pdpte.pack().unwrap()));

        let mut pdpte: PDPTE = Default::default();
        pdpte.P = true;
        pdpte.PS = true;
        pdpte.pdt_base = 0x3fc0000;
        mem.write64(0x3000 + 8*0x3, u64::from_be_bytes(pdpte.pack().unwrap()));

        let mut pde: PDE = Default::default();
        pde.P = true;
        pde.pt_base = 0x5;
        mem.write64(0x4000 + 8*0x57, u64::from_be_bytes(pde.pack().unwrap()));

        let mut pde: PDE = Default::default();
        pde.P = true;
        pde.PS = true;
        pde.pt_base = 0x2fffc00;
        mem.write64(0x4000 + 8*0xf5, u64::from_be_bytes(pde.pack().unwrap()));

        let mut pte: PTE = Default::default();
        pte.P = true;
        pte.page_base = 0x1ffffff;
        mem.write64(0x5000 + 8*0x1eb, u64::from_be_bytes(pte.pack().unwrap()));
    }

    assert_eq!(ac.trans_l2p(MemAccessMode::Read, 0x1007ff0cafebabe).unwrap(), 0x1ffffffabe);
    assert_eq!(ac.trans_l2p(MemAccessMode::Read, 0x1007ff0deadbeef).unwrap(), 0x2fffcdbeef);
    assert_eq!(ac.trans_l2p(MemAccessMode::Read, 0x1008000feedc0de).unwrap(), 0x3ffeedc0de);
}

#[cfg(test)]
#[test]
#[should_panic]
fn page_walk_test_panic() {
    let hw = hardware::Hardware::new(0x1000);
    let dev = device::Device::new(std::sync::Arc::clone(&hw.mem));
    let mut ac = super::Access::new(hw, dev);

    ac.pgmd = Some(super::PagingMode::Legacy);
    ac.core.cregs.3.from_u64(0);

    ac.trans_l2p(MemAccessMode::Read, 0xdeadbeef).unwrap();
}

#[cfg(test)]
#[test]
fn real_mem_test() {
    let hw = hardware::Hardware::new(0x1000);
    let dev = device::Device::new(std::sync::Arc::clone(&hw.mem));
    let mut ac = super::Access::new(hw, dev);

    ac.set_data32((SgReg::DS, 0x10), 0xdeadbeef).unwrap();
    assert_eq!(ac.get_data8((SgReg::DS, 0x10)).unwrap(), 0xef);

    ac.set_data32((SgReg::DS, 0x1010), 0xdeadbeef).unwrap();
    assert_eq!(ac.get_data8((SgReg::DS, 0x1010)).unwrap(), 0);
}