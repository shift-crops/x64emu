use crate::hardware::processor::general::*;
use crate::hardware::processor::segment::*;

pub fn set_r32(arg: &mut super::InstrArg, v: u32) {
    arg.ac.core.gpregs_mut().set(GpReg32::from(arg.idata.modrm.rm as usize), v);
}

fn calc_modrm(arg: &super::InstrArg) -> (SgReg, u64) {
    let modrm = arg.idata.modrm;
    assert_ne!(modrm.rm, 3);

    let mut addr: u64 = 0;
    let mut segment = SgReg::DS;

    if 16 == 32 {
        match modrm.mod_ {
            1...2 => { addr += arg.idata.disp as u64; },
            _ => {},
        }

        if modrm.rm == 4 {
            let (sg, ad) = calc_sib(arg);
            if let Some(x) = sg { segment = x; }
            addr += ad as u64;
        }
        else if modrm.rm == 5 && modrm.mod_ == 0 {
            addr += arg.idata.disp as u64;
        }
        else {
            segment = if modrm.rm == 5 { SgReg::SS } else { SgReg::DS };
            addr += arg.ac.core.gpregs().get(GpReg32::from(modrm.rm as usize)) as u64;
        }
    }
    else {
        match modrm.mod_ {
            1...2 => { addr += arg.idata.disp as u64; },
            _ => {},
        }

        match modrm.rm {
            0|1|7 => { addr += arg.ac.core.gpregs().get(GpReg16::BX) as u64; },
            2|3|6 => {
                if modrm.mod_ == 0 && modrm.rm == 6 {
                    addr += arg.idata.disp as u64;
                }
                else {
                    addr += arg.ac.core.gpregs().get(GpReg16::BP) as u64;
                    segment = SgReg::SS;
                }
            },
            _ => {},
        }

        if modrm.rm < 6 {
            addr += arg.ac.core.gpregs().get( if modrm.rm%2 == 1 {GpReg16::DI} else {GpReg16::SI} ) as u64;
        }
    }

    (segment, addr)
}

fn calc_sib(arg: &super::InstrArg) -> (Option<SgReg>, u64) {
    let (modrm,sib) = (arg.idata.modrm, arg.idata.sib);

    let bs: u64;
    let mut segment = Option::None;
    if sib.base == 5 && modrm.mod_ == 0 {
        bs = arg.idata.disp as u64;
    }
    else if sib.base == 4 {
        assert_ne!(sib.scale, 0);
        segment = Some(SgReg::SS);
        bs = 0;
    }
    else {
        segment = Some(if modrm.rm == 5 { SgReg::SS } else { SgReg::DS });
        bs = arg.ac.core.gpregs().get(GpReg32::from(sib.base as usize)) as u64;
    }

    (segment, bs + arg.ac.core.gpregs().get(GpReg32::from(sib.index as usize)) as u64 * (1<<sib.scale))
}