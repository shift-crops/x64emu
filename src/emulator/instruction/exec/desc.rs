use crate::emulator::*;
use crate::emulator::access::register::*;
use crate::emulator::access::descriptor::*;
use crate::hardware::processor::descriptor::DescTbl;

macro_rules! ret_far {
    ( $type:ty ) => { paste::item! {
        pub fn [<ret_far_ $type>](&mut self) -> Result<(), EmuException> {
            let new_ip = self.ac.[<pop_ $type>]()?;
            let new_cs = self.ac.[<pop_ $type>]()? as u16;

            match self.ac.mode {
                access::CpuMode::Real => {
                    if new_ip as u32 > self.ac.get_sgcache(SgReg::CS)?.limit {
                        return Err(EmuException::CPUException(CPUException::GP));
                    }
                    self.ac.set_segment_real(SgReg::CS, new_cs)?;
                },
                access::CpuMode::Protected | access::CpuMode::Long => {
                    let cpl = self.get_cpl()?;
                    let rpl = (new_cs & 3) as u8;

                    if new_cs == 0 || rpl < cpl {
                        return Err(EmuException::CPUException(CPUException::GP));
                    }

                    let desc = self.ac.obtain_descriptor(new_cs, false)?;
                    if let Some(DescType::Segment(SegDescType::Code(f))) = desc.get_type() {
                        if f.contains(CodeDescFlag::C) && desc.DPL > rpl {
                            return Err(EmuException::CPUException(CPUException::GP));
                        }
                    } else {
                        return Err(EmuException::CPUException(CPUException::GP));
                    }

                    if self.ac.size.op != access::AcsSize::BIT64 && new_ip as u32 > ((desc.limit_h as u32) << 16) + desc.limit_l as u32 {
                        return Err(EmuException::CPUException(CPUException::GP));
                    }
                    self.ac.set_segment_protected(SgReg::CS, new_cs, desc)?;

                    if rpl > cpl {
                        let new_sp = self.ac.[<pop_ $type>]()?;
                        let new_ss = self.ac.[<pop_ $type>]()? as u16;
                        self.ac.set_gpreg(GpReg64::RSP, new_sp as u64)?;
                        self.ac.set_segment_protected(SgReg::SS, new_ss, self.ac.obtain_descriptor(new_ss, false)?)?;
                    }
                    for r in vec!(SgReg::ES, SgReg::FS, SgReg::GS, SgReg::DS ).iter() {
                        if rpl > self.ac.get_sgcache(*r)?.DPL {
                            self.ac.set_segment_protected(*r, 0, self.ac.obtain_descriptor(0, false)?)?;
                        }
                    }
                },
            }

            self.ac.set_ip(new_ip)?;
            self.update_opadsize()
        }
    } };
}

macro_rules! jmp_far {
    ( $type:ty ) => { paste::item! {
        pub fn [<jmp_far_ $type>](&mut self, sel: u16, abs: $type) -> Result<(), EmuException> {
            match self.ac.mode {
                access::CpuMode::Real => {
                    if abs as u32 > self.ac.get_sgcache(SgReg::CS)?.limit {
                        return Err(EmuException::CPUException(CPUException::GP));
                    }

                    self.ac.set_segment_real(SgReg::CS, sel)?;
                    self.ac.set_ip(abs)?;
                },
                access::CpuMode::Protected | access::CpuMode::Long => {
                    let cpl  = self.get_cpl()?;
                    let rpl = (sel & 3) as u8;
                    let desc = self.ac.obtain_descriptor(sel, false)?;

                    match desc.get_type() {
                        Some(DescType::Segment(SegDescType::Code(f))) => {
                            if f.contains(CodeDescFlag::C) {
                                if desc.DPL > cpl {
                                    return Err(EmuException::CPUException(CPUException::GP));
                                }
                            } else if rpl > cpl || desc.DPL != cpl {
                                return Err(EmuException::CPUException(CPUException::GP));
                            }
                            if self.ac.size.op != access::AcsSize::BIT64 && abs as u32 > ((desc.limit_h as u32) << 16) + desc.limit_l as u32 {
                                return Err(EmuException::CPUException(CPUException::GP));
                            }
                            self.ac.set_segment_protected(SgReg::CS, (sel & 0xff8) | cpl as u16, desc)?;
                            self.ac.set_ip(abs)?;
                        },
                        Some(DescType::System(SysDescType::Call)) => { return Err(EmuException::NotImplementedOpcode); },
                        Some(DescType::System(SysDescType::Task)) => { return Err(EmuException::NotImplementedOpcode); },
                        Some(DescType::System(SysDescType::TSSAvl)) => { return Err(EmuException::NotImplementedOpcode); },
                        _ => {
                            return Err(EmuException::CPUException(CPUException::GP));
                        },
                    }
                },
            }
            self.update_opadsize()
        }
    } };
}

macro_rules! call_far {
    ( $type:ty ) => { paste::item! {
        pub fn [<call_far_ $type>](&mut self, sel: u16, abs: $type) -> Result<(), EmuException> {
            match self.ac.mode {
                access::CpuMode::Real => {
                    if abs as u32 > self.ac.get_sgcache(SgReg::CS)?.limit {
                        return Err(EmuException::CPUException(CPUException::GP));
                    }

                    self.ac.[<push_ $type>](self.ac.get_sgselector(SgReg::CS)?.to_u16() as $type)?;
                    self.ac.[<push_ $type>](self.ac.get_ip()?)?;
                    self.ac.set_segment_real(SgReg::CS, sel)?;
                    self.ac.set_ip(abs)?;
                },
                access::CpuMode::Protected | access::CpuMode::Long => {
                    let cpl  = self.get_cpl()?;
                    let rpl = (sel & 3) as u8;
                    let desc = self.ac.obtain_descriptor(sel, false)?;

                    match desc.get_type() {
                        Some(DescType::Segment(SegDescType::Code(f))) => {
                            if f.contains(CodeDescFlag::C) {
                                if desc.DPL > cpl {
                                    return Err(EmuException::CPUException(CPUException::GP));
                                }
                            } else if rpl > cpl || desc.DPL != cpl {
                                return Err(EmuException::CPUException(CPUException::GP));
                            }
                            if self.ac.size.op != access::AcsSize::BIT64 && abs as u32 > ((desc.limit_h as u32) << 16) + desc.limit_l as u32 {
                                return Err(EmuException::CPUException(CPUException::GP));
                            }
                            self.ac.[<push_ $type>](self.ac.get_sgselector(SgReg::CS)?.to_u16() as $type)?;
                            self.ac.[<push_ $type>](self.ac.get_ip()?)?;
                            self.ac.set_segment_protected(SgReg::CS, (sel & 0xff8) | cpl as u16, desc)?;
                            self.ac.set_ip(abs)?;
                        },
                        Some(DescType::System(SysDescType::Call)) => { return Err(EmuException::NotImplementedOpcode); },
                        Some(DescType::System(SysDescType::Task)) => { return Err(EmuException::NotImplementedOpcode); },
                        Some(DescType::System(SysDescType::TSSAvl)) => { return Err(EmuException::NotImplementedOpcode); },
                        _ => {
                            return Err(EmuException::CPUException(CPUException::GP));
                        },
                    }
                },
            }
            self.update_opadsize()
        }
    } };
}

impl<'a> super::Exec<'a> {
    ret_far!(u16);
    ret_far!(u32);
    ret_far!(u64);

    jmp_far!(u16);
    jmp_far!(u32);
    jmp_far!(u64);

    call_far!(u16);
    call_far!(u32);
    call_far!(u64);

    pub fn mov_to_sreg(&mut self, reg: SgReg, sel: u16) -> Result<(), EmuException> {
        if reg == SgReg::CS {
            return Err(EmuException::CPUException(CPUException::UD));
        }
        match self.ac.mode {
            access::CpuMode::Real =>
                self.ac.set_segment_real(reg, sel)?,
            access::CpuMode::Protected | access::CpuMode::Long =>
                self.ac.set_segment_protected(reg, sel, self.ac.obtain_descriptor(sel, false)?)?,
        }

        if reg == SgReg::SS { self.update_stacksize()?; }
        Ok(())
    }

    pub fn set_gdtr(&mut self, base: u64, limit: u16) -> Result<(), EmuException> {
        let gdtr = &mut self.ac.core.dtregs.gdtr;
        gdtr.base = base;
        gdtr.limit = limit as u32;
        Ok(())
    }

    pub fn set_idtr(&mut self, base: u64, limit: u16) -> Result<(), EmuException> {
        let idtr = &mut self.ac.core.dtregs.idtr;
        idtr.base = base;
        idtr.limit = limit as u32;
        Ok(())
    }

    fn get_ldtr(&self) -> Result<u16, EmuException> {
        Ok(self.ac.core.dtregs.ldtr.selector)
    }

    pub fn set_ldtr(&mut self, sel: u16) -> Result<(), EmuException> {
        let desc = self.ac.obtain_descriptor(sel, true)?;
        let ldtr = &mut self.ac.core.dtregs.ldtr;
        ldtr.cache       = DescTbl::from(desc);
        ldtr.selector    = sel;

        Ok(())
    }

    fn get_tr(&self) -> Result<u16, EmuException> {
        Ok(self.ac.core.dtregs.tr.selector)
    }

    pub fn set_tr(&mut self, sel: u16) -> Result<(), EmuException> {
        let desc = self.ac.obtain_descriptor(sel, true)?;
        let tr = &mut self.ac.core.dtregs.tr;
        tr.cache    = DescTbl::from(desc);
        tr.selector = sel;

        Ok(())
    }

    pub fn get_cpl(&self) -> Result<u8, EmuException> {
        Ok(self.ac.get_sgselector(SgReg::CS)?.RPL)
    }
}