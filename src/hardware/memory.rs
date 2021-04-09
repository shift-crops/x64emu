extern crate libc;
use libc::c_void;

pub struct Memory {
    mem: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Self { mem: Vec::new() }
    }

    pub fn set_size(&mut self, size: usize) -> () {
        self.mem = vec![0; (size+0xfff) & !0xfff];
    }

    pub fn read64(&self, addr: usize) -> u64 { if let Some(slice) = self.mem.get(addr..addr+8) { unsafe{ return *(slice.as_ptr() as *const u64); } } 0 }
    pub fn read32(&self, addr: usize) -> u32 { if let Some(slice) = self.mem.get(addr..addr+4) { unsafe{ return *(slice.as_ptr() as *const u32); } } 0 }
    pub fn read16(&self, addr: usize) -> u16 { if let Some(slice) = self.mem.get(addr..addr+2) { unsafe{ return *(slice.as_ptr() as *const u16); } } 0 }
    pub fn read8(&self, addr: usize) -> u8 { if let Some(slice) = self.mem.get(addr) { return *slice; } 0 }

    pub fn write64(&mut self, addr: usize, v: u64) -> () { if let Some(slice) = self.mem.get_mut(addr..addr+8) { unsafe { *(slice.as_mut_ptr() as *mut u64) = v; } } }
    pub fn write32(&mut self, addr: usize, v: u32) -> () { if let Some(slice) = self.mem.get_mut(addr..addr+4) { unsafe { *(slice.as_mut_ptr() as *mut u32) = v; } } }
    pub fn write16(&mut self, addr: usize, v: u16) -> () { if let Some(slice) = self.mem.get_mut(addr..addr+2) { unsafe { *(slice.as_mut_ptr() as *mut u16) = v; } } }
    pub fn write8(&mut self, addr: usize, v: u8) -> () { if let Some(slice) = self.mem.get_mut(addr) { *slice = v; } }

    pub fn read_data(&self, dst: *mut c_void, src_addr: usize, len: usize) -> Result<usize, &'static str> {
        unsafe{
            if let Some(slice) = self.mem.get(src_addr..src_addr+len) {
                    libc::memcpy(dst, slice.as_ptr() as *const c_void, len);
                    Ok(len)
            } else {
                    libc::memset(dst, 0, len);
                    Err("src: Out of range")
            }
        }
    }

    pub fn write_data(&mut self, dst_addr: usize, src: *const c_void, len: usize) -> Result<usize, &'static str> {
        if let Some(slice) = self.mem.get_mut(dst_addr..dst_addr+len) {
            unsafe{ libc::memcpy(slice.as_mut_ptr() as *mut c_void, src, len); }
            Ok(len)
        } else { Err("dst: Out of range") }
    }

    pub fn as_ptr(&self, addr: usize) -> Result<*const u8, &'static str> {
        if let Some(slice) = self.mem.get(addr..) {
            Ok(slice.as_ptr())
        } else { Err("as_ptr: Out of range") }
    }

    pub fn as_mut_ptr(&mut self, addr: usize) -> Result<*mut u8, &'static str> {
        if let Some(slice) = self.mem.get_mut(addr..) {
            Ok(slice.as_mut_ptr())
        } else { Err("as_mut_ptr: Out of range") }
    }

    pub fn dump(&self, addr: usize, len: usize) -> () {
        let addr = addr & !0xf;
        let n  = (len+0xf) / 0x10;

        println!("Memory Dump");
        for i in 0..n {
            println!("{:016x}: 0x{:016x} 0x{:016x}", addr+0x10*i, self.read64(addr+0x10*i), self.read64(addr+0x10*i+8));
        }
        println!("");
    }
}

#[cfg(test)]
#[test]
fn mem_test(){
    let mut mem = Memory::new();
    mem.set_size(0x1000);
    mem.write16(0x100, 0xbabe);
    mem.write16(0x102, 0xcafe);
    mem.write32(0x104, 0xdeadbeef);
    assert_eq!(mem.read64(0x100), 0xdeadbeefcafebabe);

    let mut x = mem.as_mut_ptr(0x200).unwrap() as *mut u32;
    unsafe {
        *x = 0x55667788;
        x = (x as usize + 4) as *mut u32;
        *x = 0x11223344;
    }
    assert_eq!(mem.read64(0x200), 0x1122334455667788);

    mem.write64(0x1100, 0xdeadbeef);
    assert_eq!(mem.read64(0x1100), 0x0);
}

#[cfg(test)]
#[test]
#[should_panic]
fn mem_test_panic(){
    let mut mem = Memory::new();
    mem.set_size(0x1000);

    mem.as_ptr(0x1100).unwrap();
}