#![allow(non_snake_case)]

use libc::c_void;
use packed_struct::prelude::*;
use crate::emulator::*;
use super::register::*;
use crate::hardware::memory::MemDumpSize;

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="8", endian="msb")]
pub struct PML5E {
    #[packed_field(bits="0")]  P:   u8,
    #[packed_field(bits="1")]  RW:  u8,
    #[packed_field(bits="2")]  US:  u8,
    #[packed_field(bits="3")]  PWT: u8,
    #[packed_field(bits="4")]  PCD: u8,
    #[packed_field(bits="5")]  A:   u8,
    #[packed_field(bits="12:39")] pml4_base: u32,
    #[packed_field(bits="63")] XD:  u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="8", endian="msb")]
pub struct PML4E {
    #[packed_field(bits="0")]  P:   u8,
    #[packed_field(bits="1")]  RW:  u8,
    #[packed_field(bits="2")]  US:  u8,
    #[packed_field(bits="3")]  PWT: u8,
    #[packed_field(bits="4")]  PCD: u8,
    #[packed_field(bits="5")]  A:   u8,
    #[packed_field(bits="12:39")] pdpt_base: u32,
    #[packed_field(bits="63")] XD:  u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="8", endian="msb")]
pub struct PDPTE {
    #[packed_field(bits="0")]  P:   u8,
    #[packed_field(bits="1")]  RW:  u8,
    #[packed_field(bits="2")]  US:  u8,
    #[packed_field(bits="3")]  PWT: u8,
    #[packed_field(bits="4")]  PCD: u8,
    #[packed_field(bits="5")]  A:   u8,
    #[packed_field(bits="6")]  D:   u8,
    #[packed_field(bits="7")]  PS:  u8,
    #[packed_field(bits="8")]  G:   u8,
    #[packed_field(bits="12:39")] pdt_base: u32,
    #[packed_field(bits="63")] XD:  u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="8", endian="msb")]
pub struct PDE {
    #[packed_field(bits="0")]  P:   u8,
    #[packed_field(bits="1")]  RW:  u8,
    #[packed_field(bits="2")]  US:  u8,
    #[packed_field(bits="3")]  PWT: u8,
    #[packed_field(bits="4")]  PCD: u8,
    #[packed_field(bits="5")]  A:   u8,
    #[packed_field(bits="6")]  D:   u8,
    #[packed_field(bits="7")]  PS:  u8,
    #[packed_field(bits="8")]  G:   u8,
    #[packed_field(bits="12:39")] pt_base: u32,
    #[packed_field(bits="63")] XD:  u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="8", endian="msb")]
pub struct PTE {
    #[packed_field(bits="0")]  P:   u8,
    #[packed_field(bits="1")]  RW:  u8,
    #[packed_field(bits="2")]  US:  u8,
    #[packed_field(bits="3")]  PWT: u8,
    #[packed_field(bits="4")]  PCD: u8,
    #[packed_field(bits="5")]  A:   u8,
    #[packed_field(bits="6")]  D:   u8,
    #[packed_field(bits="7")]  PS:  u8,
    #[packed_field(bits="8")]  G:   u8,
    #[packed_field(bits="12:39")] page_base: u32,
    #[packed_field(bits="63")] XD:  u8,
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
    bitlen: usize,
    pml5: Option<u64>,
    pml4: Option<u64>,
    pdpt: Option<u64>,
    pd:   u64,
    pt:   u64,
}
impl From<&LAddrLegacy> for PagingStructIndex {
    fn from(l: &LAddrLegacy) -> Self {
        Self { bitlen: 10, pml5: None, pml4: None, pdpt: None, pd: l.pd_ofs as u64, pt: l.pt_ofs as u64 }
    }
}
impl From<&LAddrPAE> for PagingStructIndex {
    fn from(l: &LAddrPAE) -> Self {
        Self { bitlen: 9, pml5: None, pml4: None, pdpt: Some(l.pdpt_ofs as u64), pd: l.pd_ofs as u64, pt: l.pt_ofs as u64 }
    }
}
impl From<&LAddrIa32e> for PagingStructIndex {
    fn from(l: &LAddrIa32e) -> Self {
        Self { bitlen: 9, pml5: Some(l.pml5_ofs as u64), pml4: Some(l.pml4_ofs as u64), pdpt: Some(l.pdpt_ofs as u64), pd: l.pd_ofs as u64, pt: l.pt_ofs as u64 }
    }
}

#[derive(Clone, Copy)]
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

    pub fn read_p(&self, dst: *mut c_void, src_addr: u64, len: usize) -> Result<usize, EmuException> {
        if let Ok(n) = self.mem.read().unwrap().read_data(dst, src_addr as usize, len) { return Ok(n); }
        panic!("{:?}", EmuException::UnexpectedError);
    }

    pub fn write_p(&mut self, dst_addr: u64, src: *const c_void, len: usize) -> Result<usize, EmuException> {
        if let Ok(n) = self.mem.write().unwrap().write_data(dst_addr as usize, src, len) { return Ok(n); }
        panic!("{:?}", EmuException::UnexpectedError);
    }

    pub fn read_l(&self, dst: *mut c_void, src_addr: u64, len: usize) -> Result<usize, EmuException> {
        self.read_p(dst, self.trans_l2p(MemAccessMode::Read, src_addr)?, len)
    }

    pub fn write_l(&mut self, dst_addr: u64, src: *const c_void, len: usize) -> Result<usize, EmuException> {
        self.write_p(self.trans_l2p(MemAccessMode::Write, dst_addr)?, src, len)
    }

    pub fn read_v(&self, seg: SgReg, dst: *mut c_void, src_addr: u64, len: usize) -> Result<usize, EmuException> {
        self.read_p(dst, self.trans_v2p(MemAccessMode::Read, seg, src_addr)?, len)
    }

    pub fn write_v(&mut self, seg: SgReg, dst_addr: u64, src: *const c_void, len: usize) -> Result<usize, EmuException> {
        self.write_p(self.trans_v2p(MemAccessMode::Write, seg, dst_addr)?, src, len)
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
            match size {
                MemAccessSize::Byte  => self.mem.read().unwrap().read8(paddr) as u64,
                MemAccessSize::Word  => self.mem.read().unwrap().read16(paddr) as u64,
                MemAccessSize::DWord => self.mem.read().unwrap().read32(paddr) as u64,
                MemAccessSize::QWord => self.mem.read().unwrap().read64(paddr),
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
            match size {
                MemAccessSize::Byte  => self.mem.write().unwrap().write8(paddr, v as u8),
                MemAccessSize::Word  => self.mem.write().unwrap().write16(paddr, v as u16),
                MemAccessSize::DWord => self.mem.write().unwrap().write32(paddr, v as u32),
                MemAccessSize::QWord => self.mem.write().unwrap().write64(paddr, v),
            }
        }
        Ok(())
    }

    fn get_code_size(&self, index: u64, size: MemAccessSize) -> Result<u64, EmuException> {
        let ip: u64 = self.get_ip()?;
        let paddr = self.trans_v2p(MemAccessMode::Exec, SgReg::CS, ip + index)? as usize;
        let v = match size {
            MemAccessSize::Byte  => self.mem.read().unwrap().read8(paddr) as u64,
            MemAccessSize::Word  => self.mem.read().unwrap().read16(paddr) as u64,
            MemAccessSize::DWord => self.mem.read().unwrap().read32(paddr) as u64,
            MemAccessSize::QWord => self.mem.read().unwrap().read64(paddr),
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

                if !long64 && (vaddr >> 12*cache.G) > cache.limit as u64 { return Err(EmuException::CPUException(CPUException::GP)); }
            },
            _ => {}
        }

        Ok(base + vaddr)
    }

    fn trans_l2p(&self, acs: MemAccessMode, laddr: u64) -> Result<u64, EmuException> {
        if let Some(md) = &self.pgmd {
            let (psidx, offset) = match md {
                super::PagingMode::Legacy => {
                    let l = LAddrLegacy::unpack(&(laddr as u32).to_be_bytes()).unwrap();
                    (PagingStructIndex::from(&l), l.p_ofs)
                },
                super::PagingMode::LegacyPAE => {
                    let l = LAddrPAE::unpack(&(laddr as u32).to_be_bytes()).unwrap();
                    (PagingStructIndex::from(&l), l.p_ofs)
                },
                super::PagingMode::Ia32e4Lv => {
                    let l = LAddrIa32e::unpack(&laddr.to_be_bytes()).unwrap();
                    let mut psi = PagingStructIndex::from(&l);
                    psi.pml5 = None;
                    (psi, l.p_ofs)
                },
                super::PagingMode::Ia32e5Lv => {
                    let l = LAddrIa32e::unpack(&laddr.to_be_bytes()).unwrap();
                    (PagingStructIndex::from(&l), l.p_ofs)
                },
            };

            let page_paddr = self.page_walk(acs, &md, psidx)?;
            Ok(page_paddr + offset as u64)
        } else {
            Ok(laddr)
        }
   }

    fn page_walk(&self, acs: MemAccessMode, pmd: &super::PagingMode, psidx: PagingStructIndex) -> Result<u64, EmuException> {
        let cr3 = &self.core.cregs.3;
        let table_size = match pmd {
            super::PagingMode::Legacy => 4,
            _ => 8,
        };

        let pml5e: Option<PML5E> = if let Some(idx) = psidx.pml5 {
            let pml5_base = cr3.get_pagedir_base();
            let mut raw: [u8; 8] = [0; 8];
            self.read_p(raw.as_mut_ptr() as *mut _, pml5_base + idx*8, 8)?;
            raw.reverse();
            Some(PML5E::unpack(&raw).unwrap())
        } else { None };

        let pml4e: Option<PML4E> = if let Some(idx) = psidx.pml4 {
            let pml4_base = match pml5e {
                Some(e) => {
                    (e.pml4_base as u64) << 12
                },
                None => cr3.get_pagedir_base(),
            };
            let mut raw: [u8; 8] = [0; 8];
            self.read_p(raw.as_mut_ptr() as *mut _, pml4_base + idx*8, 8)?;
            raw.reverse();
            Some(PML4E::unpack(&raw).unwrap())
        } else { None };

        let pdpte: Option<PDPTE> = if let Some(idx) = psidx.pdpt {
            let pdpt_base = match pml4e {
                Some(e) => {
                    (e.pdpt_base as u64) << 12
                },
                None => cr3.get_pagedir_base(),
            };
            let mut raw: [u8; 8] = [0; 8];
            self.read_p(raw.as_mut_ptr() as *mut _, pdpt_base + idx*table_size, table_size as usize)?;
            raw.reverse();
            Some(PDPTE::unpack(&raw).unwrap())
        } else { None };

        let pd_base = match pdpte {
            Some(e) => {
                match e.PS {
                    0 => {
                        (e.pdt_base as u64) << 12
                    },
                    1 => {
                        return Ok( (((((e.pdt_base as u64) << psidx.bitlen) + psidx.pd) << psidx.bitlen) + psidx.pt) << 12 );
                    },
                    _ => { panic!("{:?}", EmuException::UnexpectedError); },
                }
            },
            None => cr3.get_pagedir_base(),
        };
        let mut raw: [u8; 8] = [0; 8];
        self.read_p(raw.as_mut_ptr() as *mut _, pd_base + psidx.pd*table_size, table_size as usize)?;
        raw.reverse();
        let pde = PDE::unpack(&raw).unwrap();

        let pt_base = match pde.PS {
            0 => {
                (pde.pt_base as u64) << 12
            },
            1 => {
                return Ok( (((pde.pt_base as u64) << psidx.bitlen) + psidx.pt) << 12 );
            },
            _ => { panic!("{:?}", EmuException::UnexpectedError); },
        };
        let mut raw: [u8; 8] = [0; 8];
        self.read_p(raw.as_mut_ptr() as *mut _, pt_base + psidx.pt*table_size, table_size as usize)?;
        raw.reverse();
        let pte = PTE::unpack(&raw).unwrap();

        //println!("{:x?}", pte);
        Ok((pte.page_base as u64) << 12)
    }
}

#[cfg(test)]
#[test]
pub fn access_mem_test() {
    let hw = hardware::Hardware::new(0x1000);
    let dev = device::Device::new(std::sync::Arc::clone(&hw.mem));
    let mut ac = super::Access::new(hw, dev);

    ac.set_data32((SgReg::DS, 0x10), 0xdeadbeef).unwrap();
    assert_eq!(ac.get_data8((SgReg::DS, 0x10)).unwrap(), 0xef);

    ac.set_data32((SgReg::DS, 0x1010), 0xdeadbeef).unwrap();
    assert_eq!(ac.get_data8((SgReg::DS, 0x1010)).unwrap(), 0);
}