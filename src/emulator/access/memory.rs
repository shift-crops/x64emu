use libc::c_void;
use crate::emulator::*;
use crate::hardware::processor::CpuMode;
use crate::hardware::processor::segment::*;

#[derive(Clone, Copy)]
enum MemAccessMode { Read, Write, Exec }
enum MemAccessSize { Byte, Word, DWord, QWord }

impl super::Access {
    pub fn get_data8(&self, target: (SgReg, u64)) -> Result<u8, EmuException> { Ok(self.read_mem_v(target.0, target.1, MemAccessSize::Byte)? as u8) }
    pub fn get_data16(&self, target: (SgReg, u64)) -> Result<u16, EmuException> { Ok(self.read_mem_v(target.0, target.1, MemAccessSize::Word)? as u16) }
    pub fn get_data32(&self, target: (SgReg, u64)) -> Result<u32, EmuException> { Ok(self.read_mem_v(target.0, target.1, MemAccessSize::DWord)? as u32) }
    pub fn get_data64(&self, target: (SgReg, u64)) -> Result<u64, EmuException> { Ok(self.read_mem_v(target.0, target.1, MemAccessSize::QWord)?) }

    pub fn set_data8(&mut self, target: (SgReg, u64), v: u8) -> Result<(), EmuException> { self.write_mem_v(target.0, target.1, v as u64, MemAccessSize::Byte)?; Ok(()) }
    pub fn set_data16(&mut self, target: (SgReg, u64), v: u16) -> Result<(), EmuException> { self.write_mem_v(target.0, target.1, v as u64, MemAccessSize::Word)?; Ok(()) }
    pub fn set_data32(&mut self, target: (SgReg, u64), v: u32) -> Result<(), EmuException> { self.write_mem_v(target.0, target.1, v as u64, MemAccessSize::DWord)?; Ok(()) }
    pub fn set_data64(&mut self, target: (SgReg, u64), v: u64) -> Result<(), EmuException> { self.write_mem_v(target.0, target.1, v, MemAccessSize::QWord)?; Ok(()) }

    pub fn get_code8(&self, index: u64) -> Result<u8, EmuException> { Ok(self.fetch_mem_v(index, MemAccessSize::Byte)? as u8) }
    pub fn get_code16(&self, index: u64) -> Result<u16, EmuException> { Ok(self.fetch_mem_v(index, MemAccessSize::Word)? as u16) }
    pub fn get_code32(&self, index: u64) -> Result<u32, EmuException> { Ok(self.fetch_mem_v(index, MemAccessSize::DWord)? as u32) }
    pub fn get_code64(&self, index: u64) -> Result<u64, EmuException> { Ok(self.fetch_mem_v(index, MemAccessSize::QWord)?) }

    pub fn read_data_p(&self, dst: *mut c_void, src_addr: u64, len: usize) -> Result<usize, EmuException> {
        if let Ok(n) = self.mem.read_data(dst, src_addr as usize, len) { return Ok(n); }
        Err(EmuException::UnexpectedError)
    }

    pub fn write_data_p(&mut self, dst_addr: u64, src: *const c_void, len: usize) -> Result<usize, EmuException> {
        if let Ok(n) = self.mem.write_data(dst_addr as usize, src, len) { return Ok(n); }
        Err(EmuException::UnexpectedError)
    }

    pub fn read_data_l(&self, dst: *mut c_void, src_addr: u64, len: usize) -> Result<usize, EmuException> {
        self.read_data_p(dst, self.trans_l2p(MemAccessMode::Read, src_addr)?, len)
    }

    pub fn write_data_l(&mut self, dst_addr: u64, src: *const c_void, len: usize) -> Result<usize, EmuException> {
        self.write_data_p(self.trans_l2p(MemAccessMode::Write, dst_addr)?, src, len)
    }

    pub fn dump_code(&self) -> () {
        let addr = self.trans_v2p(MemAccessMode::Read, SgReg::CS, self.core.ip.get_rip()).unwrap();
        self.mem.dump(addr as usize -0x10, 0x20);
    }

    pub fn dump_stack(&self) -> () {
        use crate::hardware::processor::general::*;
        let addr = self.trans_v2p(MemAccessMode::Read, SgReg::SS, self.core.gpregs.get(GpReg64::RSP)).unwrap();
        self.mem.dump(addr as usize, 0x40);
    }
}

impl super::Access {
    fn read_mem_v(&self, sg: SgReg, vaddr: u64, size: MemAccessSize) -> Result<u64, EmuException> {
        let paddr = self.trans_v2p(MemAccessMode::Read, sg, vaddr)? as usize;
        let v = match size {
            MemAccessSize::Byte  => self.mem.read8(paddr) as u64,
            MemAccessSize::Word  => self.mem.read16(paddr) as u64,
            MemAccessSize::DWord => self.mem.read32(paddr) as u64,
            MemAccessSize::QWord => self.mem.read64(paddr),
        };
        Ok(v)
    }

    fn write_mem_v(&mut self, sg: SgReg, vaddr: u64, v: u64, size: MemAccessSize) -> Result<(), EmuException> {
        let paddr = self.trans_v2p(MemAccessMode::Write, sg, vaddr)? as usize;
        match size {
            MemAccessSize::Byte  => { self.mem.write8(paddr, v as u8) },
            MemAccessSize::Word  => { self.mem.write16(paddr, v as u16) },
            MemAccessSize::DWord => { self.mem.write32(paddr, v as u32) },
            MemAccessSize::QWord => { self.mem.write64(paddr, v) },
        }
        Ok(())
    }

    fn fetch_mem_v(&self, index: u64, size: MemAccessSize) -> Result<u64, EmuException> {
        let paddr = self.trans_v2p(MemAccessMode::Exec, SgReg::CS, self.core.ip.get_rip() + index)? as usize;
        let v = match size {
            MemAccessSize::Byte  => self.mem.read8(paddr) as u64,
            MemAccessSize::Word  => self.mem.read16(paddr) as u64,
            MemAccessSize::DWord => self.mem.read32(paddr) as u64,
            MemAccessSize::QWord => self.mem.read64(paddr),
        };
        Ok(v)
    }

    fn trans_v2p(&self, acsmode: MemAccessMode, sg: SgReg, vaddr: u64) -> Result<u64, EmuException> {
        let laddr = self.trans_v2l(acsmode, sg, vaddr)?;
        self.trans_l2p(acsmode, laddr)
    }

    fn trans_v2l(&self, _acsmode: MemAccessMode, sg: SgReg, vaddr: u64) -> Result<u64, EmuException> {
        let cache = self.core.sgregs.get(sg).cache;

        let base  = if self.core.mode == CpuMode::Long64 && !(sg == SgReg::FS || sg == SgReg::GS) { 0 } else { cache.base };
        let limit = (cache.limit as u64) << (if cache.G == 1 { 12 } else { 0 });

        match (self.core.mode, sg) {
            (CpuMode::Long64, SgReg::CS) | (CpuMode::LongCompat32, _) | (CpuMode::Protected, _) => {
                /*
                if cache.Type & 0 {

                } else {

                }
                */

                if self.core.mode != CpuMode::Long64 && vaddr > limit { return Err(EmuException::CPUException(CPUException::GP)); }
            },
            _ => {}
        }

        Ok(base + vaddr)
    }

    fn trans_l2p(&self, _acsmode: MemAccessMode, laddr: u64) -> Result<u64, EmuException> {
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