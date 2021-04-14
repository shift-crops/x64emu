use crate::hardware::processor::segment::*;
use crate::emulator::EmuException;

#[derive(Clone, Copy)]
enum MemAccessMode { Read, Write, Exec }

impl super::Access {
    pub fn get_data64(&self, target: (SgReg, u64)) -> Result<u64, EmuException> { self.read_seg_mem64(target.0, target.1) }
    pub fn get_data32(&self, target: (SgReg, u64)) -> Result<u32, EmuException> { self.read_seg_mem32(target.0, target.1) }
    pub fn get_data16(&self, target: (SgReg, u64)) -> Result<u16, EmuException> { self.read_seg_mem16(target.0, target.1) }
    pub fn get_data8(&self, target: (SgReg, u64)) -> Result<u8, EmuException> { self.read_seg_mem8(target.0, target.1) }

    pub fn set_data64(&mut self, target: (SgReg, u64), v: u64) -> Result<(), EmuException> { self.write_seg_mem64(target.0, target.1, v)?; Ok(()) }
    pub fn set_data32(&mut self, target: (SgReg, u64), v: u32) -> Result<(), EmuException> { self.write_seg_mem32(target.0, target.1, v)?; Ok(()) }
    pub fn set_data16(&mut self, target: (SgReg, u64), v: u16) -> Result<(), EmuException> { self.write_seg_mem16(target.0, target.1, v)?; Ok(()) }
    pub fn set_data8(&mut self, target: (SgReg, u64), v: u8) -> Result<(), EmuException> { self.write_seg_mem8(target.0, target.1, v)?; Ok(()) }

    pub fn get_code64(&self, index: u64) -> Result<u64, EmuException> { self.fetch_seg_mem64(SgReg::CS, self.core.ip.get_rip() + index) }
    pub fn get_code32(&self, index: u64) -> Result<u32, EmuException> { self.fetch_seg_mem32(SgReg::CS, self.core.ip.get_rip() + index) }
    pub fn get_code16(&self, index: u64) -> Result<u16, EmuException> { self.fetch_seg_mem16(SgReg::CS, self.core.ip.get_rip() + index) }
    pub fn get_code8(&self, index: u64) -> Result<u8, EmuException> { self.fetch_seg_mem8(SgReg::CS, self.core.ip.get_rip() + index) }

    fn read_seg_mem64(&self, sg: SgReg, vaddr: u64) -> Result<u64, EmuException> {
        let paddr = self.trans_v2p(MemAccessMode::Read, sg, vaddr)?;
        Ok(self.mem.read64(paddr as usize))
    }

    fn read_seg_mem32(&self, sg: SgReg, vaddr: u64) -> Result<u32, EmuException> {
        let paddr = self.trans_v2p(MemAccessMode::Read, sg, vaddr)?;
        Ok(self.mem.read32(paddr as usize))
    }

    fn read_seg_mem16(&self, sg: SgReg, vaddr: u64) -> Result<u16, EmuException> {
        let paddr = self.trans_v2p(MemAccessMode::Read, sg, vaddr)?;
        Ok(self.mem.read16(paddr as usize))
    }

    fn read_seg_mem8(&self, sg: SgReg, vaddr: u64) -> Result<u8, EmuException> {
        let paddr = self.trans_v2p(MemAccessMode::Read, sg, vaddr)?;
        Ok(self.mem.read8(paddr as usize))
    }

    fn write_seg_mem64(&mut self, sg: SgReg, vaddr: u64, v: u64) -> Result<(), EmuException> {
        let paddr = self.trans_v2p(MemAccessMode::Write, sg, vaddr)?;
        self.mem.write64(paddr as usize, v);
        Ok(())
    }

    fn write_seg_mem32(&mut self, sg: SgReg, vaddr: u64, v: u32) -> Result<(), EmuException> {
        let paddr = self.trans_v2p(MemAccessMode::Write, sg, vaddr)?;
        self.mem.write32(paddr as usize, v);
        Ok(())
    }

    fn write_seg_mem16(&mut self, sg: SgReg, vaddr: u64, v: u16) -> Result<(), EmuException> {
        let paddr = self.trans_v2p(MemAccessMode::Write, sg, vaddr)?;
        self.mem.write16(paddr as usize, v);
        Ok(())
    }

    fn write_seg_mem8(&mut self, sg: SgReg, vaddr: u64, v: u8) -> Result<(), EmuException> {
        let paddr = self.trans_v2p(MemAccessMode::Write, sg, vaddr)?;
        self.mem.write8(paddr as usize, v);
        Ok(())
    }

    fn fetch_seg_mem64(&self, sg: SgReg, vaddr: u64) -> Result<u64, EmuException> {
        let paddr = self.trans_v2p(MemAccessMode::Exec, sg, vaddr)?;
        Ok(self.mem.read64(paddr as usize))
    }

    fn fetch_seg_mem32(&self, sg: SgReg, vaddr: u64) -> Result<u32, EmuException> {
        let paddr = self.trans_v2p(MemAccessMode::Exec, sg, vaddr)?;
        Ok(self.mem.read32(paddr as usize))
    }

    fn fetch_seg_mem16(&self, sg: SgReg, vaddr: u64) -> Result<u16, EmuException> {
        let paddr = self.trans_v2p(MemAccessMode::Exec, sg, vaddr)?;
        Ok(self.mem.read16(paddr as usize))
    }

    fn fetch_seg_mem8(&self, sg: SgReg, vaddr: u64) -> Result<u8, EmuException> {
        let paddr = self.trans_v2p(MemAccessMode::Exec, sg, vaddr)?;
        Ok(self.mem.read8(paddr as usize))
    }

    fn trans_v2p(&self, mode: MemAccessMode, sg: SgReg, vaddr: u64) -> Result<u64, EmuException> {
        let laddr = self.trans_v2l(mode, sg, vaddr)?;
        self.trans_l2p(mode, laddr)
    }

    fn trans_v2l(&self, _mode: MemAccessMode, sg: SgReg, vaddr: u64) -> Result<u64, EmuException> {
        Ok(self.core.sgregs.cache(sg).Base as u64 + vaddr)
    }

    fn trans_l2p(&self, _mode: MemAccessMode, laddr: u64) -> Result<u64, EmuException> {
        Ok(laddr)
    }
}