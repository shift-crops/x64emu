mod regmem;
mod basic;
mod flag;
mod desc;
mod string;
mod misc;

use super::parse;
use crate::emulator::*;
use crate::emulator::EmuException;
use crate::emulator::access::register::*;

pub struct Exec<'a> {
    pub ac: &'a mut access::Access,
    pub idata: &'a parse::InstrData,
    pdata: &'a parse::PrefixData,
}

impl<'a> Exec<'a> {
    pub fn new(ac: &'a mut access::Access, parse: &'a parse::ParseInstr) -> Self {
        Self { ac, idata: &parse.instr, pdata: &parse.prefix, }
    }

    fn update_cpumode(&mut self) -> Result<(), EmuException> {
        let ac = &mut self.ac;
        let efer = &mut ac.core.msr.efer;
        let cr0 = &ac.core.cregs.0;

        ac.mode = match (efer.LME, cr0.PE, cr0.PG) {
            (0, 0, _) => access::CpuMode::Real,
            (0, 1, _) => access::CpuMode::Protected,
            (1, 1, 1) => access::CpuMode::Long,
            _ => return Err(EmuException::CPUException(CPUException::GP)),
        };
        Ok(())
    }

    fn update_opadsize(&mut self) -> Result<(), EmuException> {
        let ac = &mut self.ac;
        let efer = &ac.core.msr.efer;
        let cs = &ac.core.sgregs.get(SgReg::CS).cache;

        let (op, ad) = match (efer.LMA, cs.L, cs.DB) {
            (1, 0, 0) | (0, _, 0) => (access::AcsSize::BIT16, access::AcsSize::BIT16),
            (1, 0, 1) | (0, _, 1) => (access::AcsSize::BIT32, access::AcsSize::BIT32),
            (1, 1, 0)             => (access::AcsSize::BIT32, access::AcsSize::BIT64),
            _ => return Err(EmuException::CPUException(CPUException::GP)),
        };
        ac.size = access::OpAdSize { op, ad };
        Ok(())
    }

    fn update_stacksize(&mut self) -> Result<(), EmuException> {
        let ss = &self.ac.core.sgregs.get(SgReg::SS).cache;

        self.ac.stsz = match (ss.L, ss.DB) {
            (0, 0) => access::AcsSize::BIT16,
            (0, 1) => access::AcsSize::BIT32,
            (1, 0) => access::AcsSize::BIT64,
            _ => return Err(EmuException::CPUException(CPUException::SS)),
        };
        Ok(())
    }
}

#[cfg(test)]
#[test]
pub fn exec_test() {
    use crate::hardware;

    let hw = hardware::Hardware::new(0x1000);

    let mut ac = super::access::Access::new(hw);
    let parse: parse::ParseInstr = Default::default();

    let exe = Exec::new(&mut ac, &parse);
    exe.ac.set_gpreg(GpReg64::RSP, 0xf20).unwrap();
    exe.ac.push_u64(0xdeadbeef).unwrap();
    exe.ac.push_u64(0xcafebabe).unwrap();
    assert_eq!(exe.ac.pop_u64().unwrap(), 0xcafebabe);
    assert_eq!(exe.ac.pop_u64().unwrap(), 0xdeadbeef);

    let mut x = exe.ac.mem.as_mut_ptr(0xf20).unwrap() as *mut u64;
    unsafe {
        *x = 0x11223344;
        x = (x as usize + 8) as *mut u64;
        *x = 0x55667788;
    }
    assert_eq!(exe.ac.pop_u64().unwrap(), 0x11223344);
    assert_eq!(exe.ac.pop_u64().unwrap(), 0x55667788);
}
