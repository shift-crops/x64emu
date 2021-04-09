mod basic;
mod flag;
mod reg_mem;

use super::parse;
use crate::emulator::access;

pub struct Exec<'a> {
    pub ac: &'a mut access::Access,
    pub idata: &'a parse::InstrData,
}

impl<'a> Exec<'a> {
    pub fn new(ac: &'a mut access::Access, idata: &'a parse::InstrData) -> Self {
        Self {ac, idata, }
    }
}

#[cfg(test)]
#[test]
pub fn exec_test() {
    use crate::hardware;
    use crate::hardware::processor::general::*;

    let mut hw = hardware::Hardware::new();
    hw.init_memory(0x1000);

    let mut ac = super::access::Access::new(hw);
    let idata: parse::InstrData = Default::default();

    let mut exe = Exec::new(&mut ac, &idata);
    exe.ac.core.gpregs.set(GpReg64::RSP, 0xf20);
    exe.push64(0xdeadbeef);
    exe.push64(0xcafebabe);
    assert_eq!(exe.pop64(), 0xcafebabe);
    assert_eq!(exe.pop64(), 0xdeadbeef);

    let mut x = exe.ac.mem.as_mut_ptr(0xf20).unwrap() as *mut u64;
    unsafe {
        *x = 0x11223344;
        x = (x as usize + 8) as *mut u64;
        *x = 0x55667788;
    }
    assert_eq!(exe.pop64(), 0x11223344);
    assert_eq!(exe.pop64(), 0x55667788);
}
