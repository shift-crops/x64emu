use crate::hardware;
use crate::hardware::memory::Memory;
use crate::hardware::processor::*;
use crate::hardware::processor::general::*;

pub struct Access {
    pub core: Processor,
    pub mem: Memory,
}

impl Access {
    pub fn new(hw: hardware::Hardware) -> Self {
        Access { core: hw.core, mem: hw.mem, }
    }

    pub fn update_rip(&mut self, v: i64) -> () { self.core.rip_mut().update(v); }

    pub fn get_data64(&self, addr: usize) -> u64 { self.mem.read64(addr) } 
    pub fn get_data32(&self, addr: usize) -> u32 { self.mem.read32(addr) } 
    pub fn get_data16(&self, addr: usize) -> u16 { self.mem.read16(addr) } 
    pub fn get_data8(&self, addr: usize) -> u8 { self.mem.read8(addr) } 

    pub fn set_data64(&mut self, addr: usize, v: u64) -> () { self.mem.write64(addr, v); } 
    pub fn set_data32(&mut self, addr: usize, v: u32) -> () { self.mem.write32(addr, v); } 
    pub fn set_data16(&mut self, addr: usize, v: u16) -> () { self.mem.write16(addr, v); } 
    pub fn set_data8(&mut self, addr: usize, v: u8) -> () { self.mem.write8(addr, v); } 

    pub fn get_code64(&self, index: usize) -> u64 { self.mem.read64(self.core.rip().get() as usize + index) } 
    pub fn get_code32(&self, index: usize) -> u32 { self.mem.read32(self.core.rip().get() as usize + index) } 
    pub fn get_code16(&self, index: usize) -> u16 { self.mem.read16(self.core.rip().get() as usize + index) } 
    pub fn get_code8(&self, index: usize) -> u8 { self.mem.read8(self.core.rip().get()  as usize + index) }

    pub fn push64(&mut self, v: u64) -> () {
        self.core.gpregs_mut().update(GpReg64::RSP, -8);
        self.mem.write64((self.core.gpregs().get(GpReg64::RSP)) as usize, v);
    }

    pub fn pop64(&mut self) -> u64 {
        self.core.gpregs_mut().update(GpReg64::RSP, 8);
        self.mem.read64((self.core.gpregs().get(GpReg64::RSP)-8) as usize)
    }
}