use std::collections::VecDeque;
use crate::emulator::*;
use crate::emulator::access::*;
use crate::emulator::access::register::*;
use crate::emulator::access::descriptor::*;

#[derive(Debug)]
pub enum IntrEvent {
    Hardware(u8),
    Software(u8),
}

#[derive(Default)]
pub struct Interrupt(VecDeque<IntrEvent>);

impl Interrupt {
    pub fn enqueue(&mut self, e: IntrEvent) -> () {
        self.0.push_front(e);
    }

    pub fn handle(&mut self, ac: &mut Access) -> Result<(), EmuException> {
        if let Some(e) = self.0.pop_back(){
            let (n, hw) = match e {
                IntrEvent::Hardware(n) => (n, true),
                IntrEvent::Software(n) => (n, false),
            };

            interrupt_vector(ac, n, hw)?;
        }
        Ok(())
    }
}

fn interrupt_vector(ac: &mut Access, ivec: u8, hw: bool) -> Result<(), EmuException> {
    let idtr = &ac.core.dtregs.idtr;

    match ac.mode {
        CpuMode::Real => {
            let ivt_ofs = (ivec as u32) << 2;

            if ivt_ofs > idtr.limit { return Err(EmuException::CPUException(CPUException::GP)); }

            let mut ivt: [u16;2] = [0; 2]; // [offset, segment]
            ac.read_data_l(ivt.as_mut_ptr() as *mut _, idtr.base + ivt_ofs as u64, std::mem::size_of_val(&ivt))?;

            save_regs(ac, AcsSize::BIT16, false)?;
            ac.load_segment(SgReg::CS, ivt[1])?;
            ac.set_ip(ivt[0])?;
        },
        CpuMode::Protected | CpuMode::Long => {
            let cpl = ac.get_cpl()?;
            match ac.obtain_i_descriptor(ivec)? {
                Some(DescType::System(SysDescType::Intr(gate))) => {
                    let (new_ip, dpl) = (((gate.offset_h as u32) << 16) + gate.offset_l as u32, gate.DPL);
                    let gatesize = if gate.D == 0 { AcsSize::BIT16 } else if ac.mode == CpuMode::Long { AcsSize::BIT64 } else { AcsSize::BIT32 };

                    let (sel, desc) = ac.select_intrtrapgate(gate)?;
                    let rpl = (sel & 3) as u8;
                    if (cpl < rpl) || (!hw && cpl > dpl) { return Err(EmuException::CPUException(CPUException::GP)); }

                    let cache = ac.select_segdesc(SgReg::CS, rpl, Some(SegDescType::Code(desc)))?;

                    save_regs(ac, gatesize, cpl > rpl)?;
                    ac.core.rflags.set_interrupt(false);
                    ac.set_sgreg(SgReg::CS, sel, cache)?;
                    ac.set_ip(new_ip)?;
                },
                Some(DescType::System(SysDescType::Trap(gate))) => {
                    let (new_ip, dpl) = (((gate.offset_h as u32) << 16) + gate.offset_l as u32, gate.DPL);
                    let gatesize = if gate.D == 0 { AcsSize::BIT16 } else if ac.mode == CpuMode::Long { AcsSize::BIT64 } else { AcsSize::BIT32 };

                    let (sel, desc) = ac.select_intrtrapgate(gate)?;
                    let rpl = (sel & 3) as u8;
                    if (cpl < rpl) || (!hw && cpl > dpl) { return Err(EmuException::CPUException(CPUException::GP)); }

                    let cache = ac.select_segdesc(SgReg::CS, rpl, Some(SegDescType::Code(desc)))?;

                    save_regs(ac, gatesize, cpl > rpl)?;
                    ac.set_sgreg(SgReg::CS, sel, cache)?;
                    ac.set_ip(new_ip)?;
                },
                Some(DescType::System(SysDescType::Task(gate))) => {
                    if gate.DPL < cpl { return Err(EmuException::CPUException(CPUException::GP)); }
                    let desc = ac.select_taskgate(gate)?;
                    ac.switch_task(desc)?;
                },
                _ => { return Err(EmuException::CPUException(CPUException::GP)); },
            }
        },
    }
    Ok(())
}

fn save_regs(ac: &mut Access, size: AcsSize, chpl: bool) -> Result<(), EmuException> {
    let cs_sel = ac.get_sgselector(SgReg::CS)?.to_u16();

    if chpl { return Err(EmuException::NotImplementedFunction); }

    match (&ac.mode, size) {
        (CpuMode::Real, AcsSize::BIT16) | (CpuMode::Protected, AcsSize::BIT16) => {
            ac.push_u16(ac.get_rflags()? as u16)?;
            ac.push_u16(cs_sel)?;
            ac.push_u16(ac.get_ip()?)?;
        },
        (CpuMode::Protected, AcsSize::BIT32) => {
            ac.push_u32(ac.get_rflags()? as u32)?;
            ac.push_u32(cs_sel as u32)?;
            ac.push_u32(ac.get_ip()?)?;
        },
        (CpuMode::Long, AcsSize::BIT64) => {
            ac.push_u64(ac.get_rflags()?)?;
            ac.push_u64(cs_sel as u64)?;
            ac.push_u64(ac.get_ip()?)?;
        },
        _ => { return Err(EmuException::CPUException(CPUException::GP)); },
    }
    Ok(())
}
