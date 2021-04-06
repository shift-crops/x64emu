use crate::hardware;
use crate::hardware::memory::Memory;
use crate::hardware::processor::*;
use crate::hardware::processor::general::*;
use crate::hardware::processor::segment::*;

pub struct Access {
    pub core: Processor,
    pub mem: Memory,
}

impl Access {
    pub fn new(hw: hardware::Hardware) -> Self {
        Access { core: hw.core, mem: hw.mem, }
    }

    pub fn update_rip(&mut self, v: i64) -> () { self.core.rip_mut().update(v); }

    pub fn get_data64(&self, sg: SgReg, addr: u64) -> u64 { self.read_seg_mem64(sg, addr) } 
    pub fn get_data32(&self, sg: SgReg, addr: u64) -> u32 { self.read_seg_mem32(sg, addr) } 
    pub fn get_data16(&self, sg: SgReg, addr: u64) -> u16 { self.read_seg_mem16(sg, addr) } 
    pub fn get_data8(&self, sg: SgReg, addr: u64) -> u8 { self.read_seg_mem8(sg, addr) } 

    pub fn set_data64(&mut self, sg: SgReg, addr: u64, v: u64) -> () { self.write_seg_mem64(sg, addr, v); } 
    pub fn set_data32(&mut self, sg: SgReg, addr: u64, v: u32) -> () { self.write_seg_mem32(sg, addr, v); } 
    pub fn set_data16(&mut self, sg: SgReg, addr: u64, v: u16) -> () { self.write_seg_mem16(sg, addr, v); } 
    pub fn set_data8(&mut self, sg: SgReg, addr: u64, v: u8) -> () { self.write_seg_mem8(sg, addr, v); } 

    pub fn get_code64(&self, index: u64) -> u64 { self.fetch_seg_mem64(SgReg::CS, self.core.rip().get() + index) } 
    pub fn get_code32(&self, index: u64) -> u32 { self.fetch_seg_mem32(SgReg::CS, self.core.rip().get() + index) } 
    pub fn get_code16(&self, index: u64) -> u16 { self.fetch_seg_mem16(SgReg::CS, self.core.rip().get() + index) } 
    pub fn get_code8(&self, index: u64) -> u8 { self.fetch_seg_mem8(SgReg::CS, self.core.rip().get() + index) }

    pub fn push64(&mut self, v: u64) -> () {
        self.core.gpregs_mut().update(GpReg64::RSP, -8);
        let rsp = self.core.gpregs().get(GpReg64::RSP);
        self.write_seg_mem64(SgReg::SS, rsp, v);
    }

    pub fn pop64(&mut self) -> u64 {
        let rsp = self.core.gpregs().get(GpReg64::RSP);
        self.core.gpregs_mut().update(GpReg64::RSP, 8);
        self.read_seg_mem64(SgReg::SS, rsp)
    }

    pub fn dump(&self) -> () {
        self.core.dump();
        self.mem.dump(self.core.rip().get() as usize -0x10 , 0x20);
        self.mem.dump(self.core.gpregs().get(GpReg64::RSP) as usize, 0x40);
    }
}

impl Access {
    fn read_seg_mem64(&self, sg: SgReg, vaddr: u64) -> u64 {
        let paddr = self.trans_v2p(sg, vaddr);
        self.mem.read64(paddr as usize)
    }

    fn read_seg_mem32(&self, sg: SgReg, vaddr: u64) -> u32 {
        let paddr = self.trans_v2p(sg, vaddr);
        self.mem.read32(paddr as usize)
    }

    fn read_seg_mem16(&self, sg: SgReg, vaddr: u64) -> u16 {
        let paddr = self.trans_v2p(sg, vaddr);
        self.mem.read16(paddr as usize)
    }

    fn read_seg_mem8(&self, sg: SgReg, vaddr: u64) -> u8 {
        let paddr = self.trans_v2p(sg, vaddr);
        self.mem.read8(paddr as usize)
    }

    fn write_seg_mem64(&mut self, sg: SgReg, vaddr: u64, v: u64) -> () {
        let paddr = self.trans_v2p(sg, vaddr);
        self.mem.write64(paddr as usize, v);
    }

    fn write_seg_mem32(&mut self, sg: SgReg, vaddr: u64, v: u32) -> () {
        let paddr = self.trans_v2p(sg, vaddr);
        self.mem.write32(paddr as usize, v);
    }

    fn write_seg_mem16(&mut self, sg: SgReg, vaddr: u64, v: u16) -> () {
        let paddr = self.trans_v2p(sg, vaddr);
        self.mem.write16(paddr as usize, v);
    }

    fn write_seg_mem8(&mut self, sg: SgReg, vaddr: u64, v: u8) -> () {
        let paddr = self.trans_v2p(sg, vaddr);
        self.mem.write8(paddr as usize, v);
    }

    fn fetch_seg_mem64(&self, sg: SgReg, vaddr: u64) -> u64 {
        let paddr = self.trans_v2p(sg, vaddr);
        self.mem.read64(paddr as usize)
    }

    fn fetch_seg_mem32(&self, sg: SgReg, vaddr: u64) -> u32 {
        let paddr = self.trans_v2p(sg, vaddr);
        self.mem.read32(paddr as usize)
    }

    fn fetch_seg_mem16(&self, sg: SgReg, vaddr: u64) -> u16 {
        let paddr = self.trans_v2p(sg, vaddr);
        self.mem.read16(paddr as usize)
    }

    fn fetch_seg_mem8(&self, sg: SgReg, vaddr: u64) -> u8 {
        let paddr = self.trans_v2p(sg, vaddr);
        self.mem.read8(paddr as usize)
    }

    fn trans_v2p(&self, sg: SgReg, vaddr: u64) -> u64 {
        let laddr = self.trans_v2l(sg, vaddr);
        self.trans_l2p(laddr)
    }

    fn trans_v2l(&self, sg: SgReg, vaddr: u64) -> u64 {
        self.core.sgregs().cache(sg).Base as u64 + vaddr
    }

    fn trans_l2p(&self, laddr: u64) -> u64 {
        laddr
    }
}