use crate::hardware::processor::general::*;
use crate::hardware::processor::segment::*;
use crate::emulator::instruction::InstrArg;

macro_rules! get_gpreg { ($arg:expr, $type:ty, $reg:expr) => { $arg.ac.core.gpregs().get(<$type>::from($reg as usize)) } }
macro_rules! set_gpreg { ($arg:expr, $type:ty, $reg:expr, $val:expr) => { $arg.ac.core.gpregs_mut().set(<$type>::from($reg as usize), $val); } }

pub fn get_al(arg: &InstrArg) -> u8 {
    arg.ac.core.gpregs().get(GpReg8::AL)
}

pub fn set_al(arg: &mut InstrArg, v: u8) -> () {
    arg.ac.core.gpregs_mut().set(GpReg8::AL, v);
}

pub fn get_imm8(arg: &InstrArg) -> u8 {
    arg.idata.imm as u8
}

pub fn get_rm32(arg: &InstrArg) -> u32 {
    let modrm = arg.idata.modrm;
    if modrm.mod_ == 3 { get_gpreg!(arg, GpReg32, modrm.rm) }
    else { arg.ac.get_data32(addr_modrm(arg)) }
}

pub fn set_rm32(arg: &mut InstrArg, v: u32) -> () {
    let modrm = arg.idata.modrm;
    if modrm.mod_ == 3 { set_gpreg!(arg, GpReg32, modrm.rm, v); }
    else { arg.ac.set_data32(addr_modrm(arg), v); }
}

pub fn get_r32(arg: &InstrArg) -> u32 {
    get_gpreg!(arg, GpReg32, arg.idata.modrm.reg)
}

pub fn set_r32(arg: &mut InstrArg, v: u32) -> () {
    set_gpreg!(arg, GpReg32, arg.idata.modrm.reg, v);
}

pub fn get_moffs32(arg: &InstrArg) -> u32 {
    arg.ac.get_data32((SgReg::DS, arg.idata.moffs))
}

pub fn set_moffs32(arg: &mut InstrArg, v: u32) -> () {
    arg.ac.set_data32((SgReg::DS, arg.idata.moffs), v);
}

pub fn get_rm16(arg: &InstrArg) -> u16 {
    let modrm = arg.idata.modrm;
    if modrm.mod_ == 3 { get_gpreg!(arg, GpReg16, modrm.rm) }
    else { arg.ac.get_data16(addr_modrm(arg)) }
}

pub fn set_rm16(arg: &mut InstrArg, v: u16) -> () {
    let modrm = arg.idata.modrm;
    if modrm.mod_ == 3 { set_gpreg!(arg, GpReg16, modrm.rm, v); }
    else { arg.ac.set_data16(addr_modrm(arg), v); }
}

pub fn get_r16(arg: &InstrArg) -> u16 {
    get_gpreg!(arg, GpReg16, arg.idata.modrm.reg)
}

pub fn set_r16(arg: &mut InstrArg, v: u16) -> () {
    set_gpreg!(arg, GpReg16, arg.idata.modrm.reg, v);
}

pub fn get_moffs16(arg: &InstrArg) -> u16 {
    arg.ac.get_data16((SgReg::DS, arg.idata.moffs))
}

pub fn set_moffs16(arg: &mut InstrArg, v: u16) -> () {
    arg.ac.set_data16((SgReg::DS, arg.idata.moffs), v);
}

pub fn get_rm8(arg: &InstrArg) -> u8 {
    let modrm = arg.idata.modrm;
    if modrm.mod_ == 3 { get_gpreg!(arg, GpReg8, modrm.rm) }
    else { arg.ac.get_data8(addr_modrm(arg)) }
}

pub fn set_rm8(arg: &mut InstrArg, v: u8) -> () {
    let modrm = arg.idata.modrm;
    if modrm.mod_ == 3 { set_gpreg!(arg, GpReg8, modrm.rm, v); }
    else { arg.ac.set_data8(addr_modrm(arg), v); }
}

pub fn get_r8(arg: &InstrArg) -> u8 {
    get_gpreg!(arg, GpReg8, arg.idata.modrm.reg)
}

pub fn set_r8(arg: &mut InstrArg, v: u8) -> () {
    set_gpreg!(arg, GpReg8, arg.idata.modrm.reg, v);
}

pub fn get_moffs8(arg: &InstrArg) -> u8 {
    arg.ac.get_data8((SgReg::DS, arg.idata.moffs))
}

pub fn set_moffs8(arg: &mut InstrArg, v: u8) -> () {
    arg.ac.set_data8((SgReg::DS, arg.idata.moffs), v);
}

pub fn get_m(arg: &InstrArg) -> u64 {
    addr_modrm(arg).1
}

fn addr_modrm(arg: &InstrArg) -> (SgReg, u64) {
    let modrm = arg.idata.modrm;
    assert_ne!(modrm.rm, 3);

    let mut addr: u64 = 0;
    let mut segment = SgReg::DS;

    if 32 == 32 {
        match modrm.mod_ {
            1|2 => addr += arg.idata.disp as u64,
            _ => {},
        }

        if modrm.rm == 4 {
            let (sg, ad) = addr_sib(arg);
            if let Some(x) = sg { segment = x; }
            addr += ad as u64;
        } else if modrm.rm == 5 && modrm.mod_ == 0 {
            addr += arg.idata.disp as u64;
        } else {
            segment = if modrm.rm == 5 { SgReg::SS } else { SgReg::DS };
            addr += arg.ac.core.gpregs().get(GpReg32::from(modrm.rm as usize)) as u64;
        }
    } else {
        match modrm.mod_ {
            1|2 => addr += arg.idata.disp as u64,
            _ => {},
        }

        match modrm.rm {
            0|1|7 => addr += arg.ac.core.gpregs().get(GpReg16::BX) as u64,
            2|3|6 => {
                if modrm.mod_ == 0 && modrm.rm == 6 {
                    addr += arg.idata.disp as u64;
                } else {
                    addr += arg.ac.core.gpregs().get(GpReg16::BP) as u64;
                    segment = SgReg::SS;
                }
            },
            _ => { panic!("ha??"); },
        }

        if modrm.rm < 6 {
            addr += arg.ac.core.gpregs().get( if modrm.rm%2 == 1 {GpReg16::DI} else {GpReg16::SI} ) as u64;
        }
    }

    if let Some(x) = arg.idata.pre_segment { segment = x };
    (segment, addr)
}

fn addr_sib(arg: &InstrArg) -> (Option<SgReg>, u64) {
    let (modrm,sib) = (arg.idata.modrm, arg.idata.sib);

    let bs: u64;
    let mut segment = Default::default();
    if sib.base == 5 && modrm.mod_ == 0 {
        bs = arg.idata.disp as u64;
    } else if sib.base == 4 {
        assert_eq!(sib.scale, 0);
        segment = Some(SgReg::SS);
        bs = 0;
    } else {
        segment = Some(if modrm.rm == 5 { SgReg::SS } else { SgReg::DS });
        bs = arg.ac.core.gpregs().get(GpReg32::from(sib.base as usize)) as u64;
    }

    (segment, bs + arg.ac.core.gpregs().get(GpReg32::from(sib.index as usize)) as u64 * (1<<sib.scale))
}