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
        self.0.push_back(e);
    }

    pub fn enqueue_top(&mut self, e: IntrEvent) -> () {
        self.0.push_front(e);
    }

    pub fn handle(&mut self, ac: &mut Access) -> Result<(), EmuException> {
        if let Some(e) = self.0.pop_front(){
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

            if ivt_ofs > idtr.limit { return Err(EmuException::CPUException(CPUException::GP(None))); }

            let mut ivt: IVT = Default::default();
            ac.read_l(&mut ivt as *mut IVT as *mut _, idtr.base + ivt_ofs as u64, std::mem::size_of_val(&ivt))?;

            ac.save_regs(AcsSize::BIT16, None)?;
            ac.load_segment(SgReg::CS, ivt.segment)?;
            ac.set_ip(ivt.offset)?;
        },
        CpuMode::Protected | CpuMode::Long => {
            let cpl = ac.get_cpl()?;
            match ac.obtain_i_desc(ivec)? {
                Some(DescType::System(SysDescType::Intr(gate))) => {
                    let (new_ip, dpl) = (((gate.offset_h as u32) << 16) + gate.offset_l as u32, gate.DPL);
                    let gatesize = if gate.D == 0 { AcsSize::BIT16 } else if ac.mode == CpuMode::Long { AcsSize::BIT64 } else { AcsSize::BIT32 };

                    let (sel, desc) = ac.select_intrtrapgate(gate)?;
                    let rpl = (sel & 3) as u8;
                    if (cpl < rpl) || (!hw && cpl > dpl) { return Err(EmuException::CPUException(CPUException::GP(None))); }

                    let cache = ac.select_segdesc(SgReg::CS, rpl, Some(SegDescType::Code(desc)))?;

                    ac.save_regs(gatesize, if rpl < cpl { Some(rpl) } else { None })?;
                    ac.core.rflags.set_interrupt(false);
                    ac.set_sgreg(SgReg::CS, sel, cache)?;
                    ac.set_ip(new_ip)?;
                },
                Some(DescType::System(SysDescType::Trap(gate))) => {
                    let (new_ip, dpl) = (((gate.offset_h as u32) << 16) + gate.offset_l as u32, gate.DPL);
                    let gatesize = if gate.D == 0 { AcsSize::BIT16 } else if ac.mode == CpuMode::Long { AcsSize::BIT64 } else { AcsSize::BIT32 };

                    let (sel, desc) = ac.select_intrtrapgate(gate)?;
                    let rpl = (sel & 3) as u8;
                    if (cpl < rpl) || (!hw && cpl > dpl) { return Err(EmuException::CPUException(CPUException::GP(None))); }

                    let cache = ac.select_segdesc(SgReg::CS, rpl, Some(SegDescType::Code(desc)))?;

                    ac.save_regs(gatesize, if rpl < cpl { Some(rpl) } else { None })?;
                    ac.set_sgreg(SgReg::CS, sel, cache)?;
                    ac.set_ip(new_ip)?;
                },
                Some(DescType::System(SysDescType::Task(gate))) => {
                    if gate.DPL < cpl { return Err(EmuException::CPUException(CPUException::GP(None))); }
                    let tss_sel = gate.tss_sel;
                    let desc = ac.select_taskgate(gate)?;
                    ac.switch_task(TSMode::CallInt, tss_sel, desc)?;
                },
                _ => { return Err(EmuException::CPUException(CPUException::GP(None))); },
            }
        },
    }
    Ok(())
}
