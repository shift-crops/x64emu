mod regmem;
mod basic;
mod flag;
mod desc;
mod string;
mod misc;

use super::parse;
use crate::emulator::*;

pub struct Exec<'a> {
    pub(super) ac: &'a mut access::Access,
    pub(super) idata: &'a parse::InstrData,
    pdata: &'a parse::PrefixData,
}

impl<'a> Exec<'a> {
    pub(super) fn new(ac: &'a mut access::Access, parse: &'a parse::ParseInstr) -> Self {
        Self { ac, idata: &parse.instr, pdata: &parse.prefix, }
    }
}

#[cfg(test)]
#[test]
fn exec_test() {
    use crate::hardware;
    use crate::device;
    use crate::emulator::access::register::*;

    let hw = hardware::Hardware::new(0x1000);
    let (dev, _) = device::Device::new();
    let mut ac = super::access::Access::new(hw, dev);
    let parse: parse::ParseInstr = Default::default();

    let exe = Exec::new(&mut ac, &parse);
    exe.ac.set_gpreg(GpReg64::RSP, 0xf20).unwrap();
    exe.ac.push_u64(0xdeadbeef).unwrap();
    exe.ac.push_u64(0xcafebabe).unwrap();
    assert_eq!(exe.ac.pop_u64().unwrap(), 0xcafebabe);
    assert_eq!(exe.ac.pop_u64().unwrap(), 0xdeadbeef);

    let mut x = exe.ac.mem.write().unwrap().as_mut_ptr(0xf20).unwrap() as *mut u64;
    unsafe {
        *x = 0x11223344;
        x = (x as usize + 8) as *mut u64;
        *x = 0x55667788;
    }
    assert_eq!(exe.ac.pop_u64().unwrap(), 0x11223344);
    assert_eq!(exe.ac.pop_u64().unwrap(), 0x55667788);
}
