use std::collections::VecDeque;
use crate::emulator::*;
use crate::emulator::access::register::*;

#[derive(Debug)]
pub enum Event {
    Hardware(u8),
    Software(u8),
}

#[derive(Debug, Default)] #[repr(C)]
struct IVT {
    offset: u16,
    segment: u16,
}

#[derive(Default)]
pub struct Interrupt(VecDeque<Event>);

impl Interrupt {
    pub fn enqueue(&mut self, e: Event) -> () {
        self.0.push_front(e);
    }

    pub fn handle(&mut self, ac: &mut access::Access) -> Result<(), EmuException> {
        if let Some(e) = self.0.pop_back(){
            let idtr = &ac.core.dtregs.idtr;

            let (n, hw) = match e {
                Event::Hardware(n) => (n, true),
                Event::Software(n) => (n, false),
            };

            match ac.mode {
                access::CpuMode::Real => {
                    let ivt_ofs = (n as u32) << 2;

                    if ivt_ofs > idtr.limit { return Err(EmuException::CPUException(CPUException::GP)); }

                    let ivt: IVT = Default::default();
                    ac.read_data_p(&ivt as *const _ as *mut _, idtr.base + ivt_ofs as u64, std::mem::size_of::<IVT>())?;

                    self.save_regs(ac)?;
                    ac.set_segment_real(SgReg::CS, ivt.segment)?;
                    ac.set_ip(ivt.offset)?;
                },
                access::CpuMode::Protected | access::CpuMode::Long => {
                    return Err(EmuException::NotImplementedOpcode);
                },
            }
        }
        Ok(())
    }

    fn save_regs(&mut self, ac: &mut access::Access) -> Result<(), EmuException> {
        match ac.size.op {
            access::AcsSize::BIT16 => {
                ac.push_u16(ac.get_rflags()? as u16)?;
                ac.push_u16(ac.get_sgselector(SgReg::CS)?.to_u16())?;
                ac.push_u16(ac.get_ip()?)?;
            },
            access::AcsSize::BIT32 => {
            },
            access::AcsSize::BIT64 => {
            },
        }
        Ok(())
    }
}