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

                    if let Some(DescType::Segment(SegDescType::Code(cd))) = self.ac.obtain_gl_descriptor(new_cs)? {
                        if CodeDescFlag::from(&cd).contains(CodeDescFlag::C) && cd.DPL > rpl {
                            return Err(EmuException::CPUException(CPUException::GP));
                        }

                        if self.ac.size.op != access::AcsSize::BIT64 && new_ip as u32 > ((cd.limit_h as u32) << 16) + cd.limit_l as u32 {
                            return Err(EmuException::CPUException(CPUException::GP));
                        }
                    } else {
                        return Err(EmuException::CPUException(CPUException::GP));
                    }

                    self.ac.set_segment_protected(SgReg::CS, new_cs)?;

                    if rpl > cpl {
                        let new_sp = self.ac.[<pop_ $type>]()?;
                        let new_ss = self.ac.[<pop_ $type>]()? as u16;
                        self.ac.set_gpreg(GpReg64::RSP, new_sp as u64)?;
                        self.ac.set_segment_protected(SgReg::SS, new_ss)?;
                    }
                    for r in vec!(SgReg::ES, SgReg::FS, SgReg::GS, SgReg::DS ).iter() {
                        if rpl > self.ac.get_sgcache(*r)?.DPL {
                            self.ac.set_segment_protected(*r, 0)?;
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
                    let desc = self.ac.obtain_gl_descriptor(sel)?;

                    match desc {
                        Some(DescType::Segment(SegDescType::Code(cd))) => {
                            if CodeDescFlag::from(&cd).contains(CodeDescFlag::C) {
                                if cd.DPL > cpl {
                                    return Err(EmuException::CPUException(CPUException::GP));
                                }
                            } else if rpl > cpl || cd.DPL != cpl {
                                return Err(EmuException::CPUException(CPUException::GP));
                            }
                            if self.ac.size.op != access::AcsSize::BIT64 && abs as u32 > ((cd.limit_h as u32) << 16) + cd.limit_l as u32 {
                                return Err(EmuException::CPUException(CPUException::GP));
                            }
                            self.ac.set_segment_protected(SgReg::CS, (sel & 0xff8) | cpl as u16)?;
                            self.ac.set_ip(abs)?;
                        },
                        Some(DescType::System(SysDescType::Call(_))) => { return Err(EmuException::NotImplementedFunction); },
                        Some(DescType::System(SysDescType::Task(_))) => { return Err(EmuException::NotImplementedFunction); },
                        Some(DescType::System(SysDescType::TSS(_))) => { return Err(EmuException::NotImplementedFunction); },
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
                    let desc = self.ac.obtain_gl_descriptor(sel)?;

                    match desc {
                        Some(DescType::Segment(SegDescType::Code(cd))) => {
                            if CodeDescFlag::from(&cd).contains(CodeDescFlag::C) {
                                if cd.DPL > cpl {
                                    return Err(EmuException::CPUException(CPUException::GP));
                                }
                            } else if rpl > cpl || cd.DPL != cpl {
                                return Err(EmuException::CPUException(CPUException::GP));
                            }
                            if self.ac.size.op != access::AcsSize::BIT64 && abs as u32 > ((cd.limit_h as u32) << 16) + cd.limit_l as u32 {
                                return Err(EmuException::CPUException(CPUException::GP));
                            }
                            self.ac.[<push_ $type>](self.ac.get_sgselector(SgReg::CS)?.to_u16() as $type)?;
                            self.ac.[<push_ $type>](self.ac.get_ip()?)?;
                            self.ac.set_segment_protected(SgReg::CS, (sel & 0xff8) | cpl as u16)?;
                            self.ac.set_ip(abs)?;
                        },
                        Some(DescType::System(SysDescType::Call(_))) => { return Err(EmuException::NotImplementedFunction); },
                        Some(DescType::System(SysDescType::Task(_))) => { return Err(EmuException::NotImplementedFunction); },
                        Some(DescType::System(SysDescType::TSS(_))) => { return Err(EmuException::NotImplementedFunction); },
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
                self.ac.set_segment_protected(reg, sel)?,
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
        if let Some(DescType::System(SysDescType::LDT(ldtd))) = self.ac.obtain_g_descriptor(sel)? {
            let ldtr = &mut self.ac.core.dtregs.ldtr;
            ldtr.cache       = DescTbl::from(ldtd);
            ldtr.selector    = sel;
            Ok(())
        } else {
            Err(EmuException::CPUException(CPUException::GP))
        }
    }

    fn get_tr(&self) -> Result<u16, EmuException> {
        Ok(self.ac.core.dtregs.tr.selector)
    }

    pub fn set_tr(&mut self, sel: u16) -> Result<(), EmuException> {
        if let Some(DescType::System(SysDescType::TSS(tssd))) = self.ac.obtain_g_descriptor(sel)? {
            let tr = &mut self.ac.core.dtregs.tr;
            tr.cache       = DescTbl::from(tssd);
            tr.selector    = sel;
            Ok(())
        } else {
            Err(EmuException::CPUException(CPUException::GP))
        }
    }

    pub fn get_cpl(&self) -> Result<u8, EmuException> {
        Ok(self.ac.get_sgselector(SgReg::CS)?.RPL)
    }
}