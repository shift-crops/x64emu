use crate::emulator::*;
use crate::hardware::processor::segment::*;

#[derive(Clone, Copy)]
enum MemAccessMode { Read, Write, Exec }
enum MemAccessSize { Byte, Word, DWord, QWord }

impl super::Access {
    pub fn get_data8(&self, target: (SgReg, u64)) -> Result<u8, EmuException> { Ok(self.read_seg_mem(target.0, target.1, MemAccessSize::Byte)? as u8) }
    pub fn get_data16(&self, target: (SgReg, u64)) -> Result<u16, EmuException> { Ok(self.read_seg_mem(target.0, target.1, MemAccessSize::Word)? as u16) }
    pub fn get_data32(&self, target: (SgReg, u64)) -> Result<u32, EmuException> { Ok(self.read_seg_mem(target.0, target.1, MemAccessSize::DWord)? as u32) }
    pub fn get_data64(&self, target: (SgReg, u64)) -> Result<u64, EmuException> { Ok(self.read_seg_mem(target.0, target.1, MemAccessSize::QWord)?) }

    pub fn set_data8(&mut self, target: (SgReg, u64), v: u8) -> Result<(), EmuException> { self.write_seg_mem(target.0, target.1, v as u64, MemAccessSize::Byte)?; Ok(()) }
    pub fn set_data16(&mut self, target: (SgReg, u64), v: u16) -> Result<(), EmuException> { self.write_seg_mem(target.0, target.1, v as u64, MemAccessSize::Word)?; Ok(()) }
    pub fn set_data32(&mut self, target: (SgReg, u64), v: u32) -> Result<(), EmuException> { self.write_seg_mem(target.0, target.1, v as u64, MemAccessSize::DWord)?; Ok(()) }
    pub fn set_data64(&mut self, target: (SgReg, u64), v: u64) -> Result<(), EmuException> { self.write_seg_mem(target.0, target.1, v, MemAccessSize::QWord)?; Ok(()) }

    pub fn get_code8(&self, index: u64) -> Result<u8, EmuException> { Ok(self.fetch_seg_mem(index, MemAccessSize::Byte)? as u8) }
    pub fn get_code16(&self, index: u64) -> Result<u16, EmuException> { Ok(self.fetch_seg_mem(index, MemAccessSize::Word)? as u16) }
    pub fn get_code32(&self, index: u64) -> Result<u32, EmuException> { Ok(self.fetch_seg_mem(index, MemAccessSize::DWord)? as u32) }
    pub fn get_code64(&self, index: u64) -> Result<u64, EmuException> { Ok(self.fetch_seg_mem(index, MemAccessSize::QWord)?) }

    fn read_seg_mem(&self, sg: SgReg, vaddr: u64, size: MemAccessSize) -> Result<u64, EmuException> {
        let paddr = self.trans_v2p(MemAccessMode::Read, sg, vaddr)? as usize;
        let v = match size {
            MemAccessSize::Byte  => self.mem.read8(paddr) as u64,
            MemAccessSize::Word  => self.mem.read16(paddr) as u64,
            MemAccessSize::DWord => self.mem.read32(paddr) as u64,
            MemAccessSize::QWord => self.mem.read64(paddr),
        };
        Ok(v)
    }

    fn write_seg_mem(&mut self, sg: SgReg, vaddr: u64, v: u64, size: MemAccessSize) -> Result<(), EmuException> {
        let paddr = self.trans_v2p(MemAccessMode::Write, sg, vaddr)? as usize;
        match size {
            MemAccessSize::Byte  => { self.mem.write8(paddr, v as u8) },
            MemAccessSize::Word  => { self.mem.write16(paddr, v as u16) },
            MemAccessSize::DWord => { self.mem.write32(paddr, v as u32) },
            MemAccessSize::QWord => { self.mem.write64(paddr, v) },
        }
        Ok(())
    }

    fn fetch_seg_mem(&self, index: u64, size: MemAccessSize) -> Result<u64, EmuException> {
        let paddr = self.trans_v2p(MemAccessMode::Exec, SgReg::CS, self.core.ip.get_rip() + index)? as usize;
        let v = match size {
            MemAccessSize::Byte  => self.mem.read8(paddr) as u64,
            MemAccessSize::Word  => self.mem.read16(paddr) as u64,
            MemAccessSize::DWord => self.mem.read32(paddr) as u64,
            MemAccessSize::QWord => self.mem.read64(paddr),
        };
        Ok(v)
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

#[cfg(test)]
#[test]
pub fn access_mem_test() {
    let hw = hardware::Hardware::new(0, 0x1000);

    let mut ac = access::Access::new(hw);
    ac.set_data32((SgReg::DS, 0x10), 0xdeadbeef).unwrap();
    assert_eq!(ac.get_data8((SgReg::DS, 0x10)).unwrap(), 0xef);

    ac.set_data32((SgReg::DS, 0x1010), 0xdeadbeef).unwrap();
    assert_eq!(ac.get_data8((SgReg::DS, 0x1010)).unwrap(), 0);
}