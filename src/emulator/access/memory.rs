use libc::c_void;
use crate::emulator::*;
use super::register::*;

#[derive(Clone, Copy)]
enum MemAccessMode { Read, Write, Exec, Monitor }
enum MemAccessSize { Byte, Word, DWord, QWord }

impl super::Access {
    pub fn get_data8(&self, target: (SgReg, u64)) -> Result<u8, EmuException> { Ok(self.read_vaddr(target.0, target.1, MemAccessSize::Byte)? as u8) }
    pub fn get_data16(&self, target: (SgReg, u64)) -> Result<u16, EmuException> { Ok(self.read_vaddr(target.0, target.1, MemAccessSize::Word)? as u16) }
    pub fn get_data32(&self, target: (SgReg, u64)) -> Result<u32, EmuException> { Ok(self.read_vaddr(target.0, target.1, MemAccessSize::DWord)? as u32) }
    pub fn get_data64(&self, target: (SgReg, u64)) -> Result<u64, EmuException> { Ok(self.read_vaddr(target.0, target.1, MemAccessSize::QWord)?) }

    pub fn set_data8(&mut self, target: (SgReg, u64), v: u8) -> Result<(), EmuException> { self.write_vaddr(target.0, target.1, v as u64, MemAccessSize::Byte)?; Ok(()) }
    pub fn set_data16(&mut self, target: (SgReg, u64), v: u16) -> Result<(), EmuException> { self.write_vaddr(target.0, target.1, v as u64, MemAccessSize::Word)?; Ok(()) }
    pub fn set_data32(&mut self, target: (SgReg, u64), v: u32) -> Result<(), EmuException> { self.write_vaddr(target.0, target.1, v as u64, MemAccessSize::DWord)?; Ok(()) }
    pub fn set_data64(&mut self, target: (SgReg, u64), v: u64) -> Result<(), EmuException> { self.write_vaddr(target.0, target.1, v, MemAccessSize::QWord)?; Ok(()) }

    pub fn get_code8(&self, index: u64) -> Result<u8, EmuException> { Ok(self.fetch_vaddr(index, MemAccessSize::Byte)? as u8) }
    pub fn get_code16(&self, index: u64) -> Result<u16, EmuException> { Ok(self.fetch_vaddr(index, MemAccessSize::Word)? as u16) }
    pub fn get_code32(&self, index: u64) -> Result<u32, EmuException> { Ok(self.fetch_vaddr(index, MemAccessSize::DWord)? as u32) }
    pub fn get_code64(&self, index: u64) -> Result<u64, EmuException> { Ok(self.fetch_vaddr(index, MemAccessSize::QWord)?) }

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

    pub fn read_data_v(&self, seg: SgReg, dst: *mut c_void, src_addr: u64, len: usize) -> Result<usize, EmuException> {
        self.read_data_p(dst, self.trans_v2p(MemAccessMode::Read, seg, src_addr)?, len)
    }

    pub fn write_data_v(&mut self, seg: SgReg, dst_addr: u64, src: *const c_void, len: usize) -> Result<usize, EmuException> {
        self.write_data_p(self.trans_v2p(MemAccessMode::Write, seg, dst_addr)?, src, len)
    }

    pub fn addr_v2p(&mut self, seg: SgReg, vaddr: u64) -> Result<u64, EmuException> {
        self.trans_v2p(MemAccessMode::Monitor, seg, vaddr)
    }

    pub fn dump_code(&self) -> () {
        let addr = self.trans_v2p(MemAccessMode::Read, SgReg::CS, self.get_ip().unwrap()).unwrap();
        self.mem.dump(addr as usize -0x10, 0x20);
    }

    pub fn dump_stack(&self) -> () {
        let addr = self.trans_v2p(MemAccessMode::Read, SgReg::SS, self.get_gpreg(GpReg64::RSP).unwrap()).unwrap();
        self.mem.dump(addr as usize, 0x40);
    }
}

impl super::Access {
    fn stack_update(&mut self, size: i8) -> Result<u64, EmuException> {
        let sp = match self.stsz {
            access::AcsSize::BIT16 => {
                self.update_gpreg(GpReg16::SP, size as i16)?;
                self.get_gpreg(GpReg16::SP)? as u64
            },
            access::AcsSize::BIT32 => {
                self.update_gpreg(GpReg32::ESP, size as i32)?;
                self.get_gpreg(GpReg32::ESP)? as u64
            },
            access::AcsSize::BIT64 => {
                self.update_gpreg(GpReg64::RSP, size as i64)?;
                self.get_gpreg(GpReg64::RSP)?
            },
        };
        Ok(sp)
    }

    fn read_vaddr(&self, sg: SgReg, vaddr: u64, size: MemAccessSize) -> Result<u64, EmuException> {
        let paddr = self.trans_v2p(MemAccessMode::Read, sg, vaddr)? as usize;
        let v = match size {
            MemAccessSize::Byte  => self.mem.read8(paddr) as u64,
            MemAccessSize::Word  => self.mem.read16(paddr) as u64,
            MemAccessSize::DWord => self.mem.read32(paddr) as u64,
            MemAccessSize::QWord => self.mem.read64(paddr),
        };
        Ok(v)
    }

    fn write_vaddr(&mut self, sg: SgReg, vaddr: u64, v: u64, size: MemAccessSize) -> Result<(), EmuException> {
        let paddr = self.trans_v2p(MemAccessMode::Write, sg, vaddr)? as usize;
        match size {
            MemAccessSize::Byte  => self.mem.write8(paddr, v as u8),
            MemAccessSize::Word  => self.mem.write16(paddr, v as u16),
            MemAccessSize::DWord => self.mem.write32(paddr, v as u32),
            MemAccessSize::QWord => self.mem.write64(paddr, v),
        }
        Ok(())
    }

    fn fetch_vaddr(&self, index: u64, size: MemAccessSize) -> Result<u64, EmuException> {
        let ip: u64 = self.get_ip()?;
        let paddr = self.trans_v2p(MemAccessMode::Exec, SgReg::CS, ip + index)? as usize;
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
        let paddr = self.trans_l2p(acsmode, laddr)?;

        Ok( if self.a20gate { paddr } else { paddr & (1<<20)-1 } )
    }

    fn trans_v2l(&self, _acsmode: MemAccessMode, sg: SgReg, vaddr: u64) -> Result<u64, EmuException> {
        let long64 = self.mode == access::CpuMode::Long && self.size.ad == access::AcsSize::BIT64;

        let cache = self.get_sgcache(sg)?;
        let base  = if long64 && !(sg == SgReg::FS || sg == SgReg::GS) { 0 } else { cache.base };
        let limit = (cache.limit as u64) << (if cache.G == 1 { 12 } else { 0 });

        match (&self.mode, &self.size.ad, sg) {
            (access::CpuMode::Long, access::AcsSize::BIT64, SgReg::CS) | (access::CpuMode::Protected, _, _) => {
                /*
                if cache.Type & 0 {

                } else {

                }
                */

                if !long64 && vaddr > limit { return Err(EmuException::CPUException(CPUException::GP)); }
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
    let hw = hardware::Hardware::new(0x1000);

    let mut ac = access::Access::new(hw);
    ac.set_data32((SgReg::DS, 0x10), 0xdeadbeef).unwrap();
    assert_eq!(ac.get_data8((SgReg::DS, 0x10)).unwrap(), 0xef);

    ac.set_data32((SgReg::DS, 0x1010), 0xdeadbeef).unwrap();
    assert_eq!(ac.get_data8((SgReg::DS, 0x1010)).unwrap(), 0);
}