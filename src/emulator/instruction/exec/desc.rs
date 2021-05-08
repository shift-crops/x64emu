use crate::emulator::*;
use crate::emulator::access::register::*;
use crate::emulator::access::descriptor::*;

macro_rules! jmp_far {
    ( $type:ty ) => { paste::item! {
        pub fn [<jmp_far_ $type>](&mut self, sel: u16, abs: $type) -> Result<(), EmuException> {
            match self.ac.mode {
                access::CpuMode::Real => {
                    self.ac.load_segment(SgReg::CS, sel)?;
                    self.ac.set_ip(abs)?;
                },
                access::CpuMode::Protected | access::CpuMode::Long => {
                    let cpl  = self.ac.get_cpl()?;
                    let rpl = (sel & 3) as u8;

                    match self.ac.obtain_gl_desc(sel)? {
                        Some(DescType::Segment(SegDescType::Code(desc))) => {
                            if CodeDescFlag::from(&desc).contains(CodeDescFlag::C) {
                                if desc.DPL > cpl {
                                    return Err(EmuException::CPUException(CPUException::GP));
                                }
                            } else if rpl > cpl || desc.DPL != cpl {
                                return Err(EmuException::CPUException(CPUException::GP));
                            }
                            if self.ac.oasz.op != access::AcsSize::BIT64 && abs as u32 > ((desc.limit_h as u32) << 16) + desc.limit_l as u32 {
                                return Err(EmuException::CPUException(CPUException::GP));
                            }

                            let cache = self.ac.select_segdesc(SgReg::CS, cpl, Some(SegDescType::Code(desc)))?;
                            self.ac.set_sgreg(SgReg::CS, (sel & 0xfff8) | cpl as u16, cache)?;
                            self.ac.set_ip(abs)?;
                        },
                        Some(DescType::System(SysDescType::Call(gate))) => {
                            if gate.DPL < cpl || gate.DPL < rpl { return Err(EmuException::CPUException(CPUException::GP)); }
                            let ip = ((gate.offset_h as u32) << 16) + gate.offset_l as u32;
                            let (sel, desc) = self.ac.select_callgate(gate)?;

                            if CodeDescFlag::from(&desc).contains(CodeDescFlag::C) {
                                if desc.DPL > cpl { return Err(EmuException::CPUException(CPUException::GP)); }
                            } else if desc.DPL != cpl {
                                return Err(EmuException::CPUException(CPUException::GP));
                            }

                            if self.ac.oasz.op != access::AcsSize::BIT64 && abs as u32 > ((desc.limit_h as u32) << 16) + desc.limit_l as u32 {
                                return Err(EmuException::CPUException(CPUException::GP));
                            }

                            let cache = self.ac.select_segdesc(SgReg::CS, cpl, Some(SegDescType::Code(desc)))?;
                            self.ac.set_sgreg(SgReg::CS, (sel & 0xfff8) | cpl as u16, cache)?;
                            self.ac.set_ip(ip)?;
                        },
                        Some(DescType::System(SysDescType::Task(gate))) => {
                            if gate.DPL < cpl || gate.DPL < rpl { return Err(EmuException::CPUException(CPUException::GP)); }
                            let tss_sel = gate.tss_sel;
                            let desc = self.ac.select_taskgate(gate)?;
                            self.ac.switch_task(TSMode::Jmp, tss_sel, desc)?;
                        },
                        Some(DescType::System(SysDescType::TSS(desc))) => {
                            if desc.DPL < cpl || desc.DPL < rpl { return Err(EmuException::CPUException(CPUException::GP)); }
                            self.ac.switch_task(TSMode::Jmp, sel, desc)?;
                        },
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
            let cs = self.ac.get_sgreg(SgReg::CS)?;
            let cs_sel = cs.0 as $type;
            let ip: $type = self.ac.get_ip()?;

            match self.ac.mode {
                access::CpuMode::Real => {
                    if abs as u32 > cs.1.limit {
                        return Err(EmuException::CPUException(CPUException::GP));
                    }

                    self.ac.load_segment(SgReg::CS, sel)?;
                    self.ac.set_ip(abs)?;
                    self.ac.[<push_ $type>](cs_sel)?;
                    self.ac.[<push_ $type>](ip)?;
                },
                access::CpuMode::Protected | access::CpuMode::Long => {
                    let cpl  = self.ac.get_cpl()?;
                    let rpl = (sel & 3) as u8;

                    match self.ac.obtain_gl_desc(sel)? {
                        Some(DescType::Segment(SegDescType::Code(cdesc))) => {
                            if CodeDescFlag::from(&cdesc).contains(CodeDescFlag::C) {
                                if cdesc.DPL > cpl {
                                    return Err(EmuException::CPUException(CPUException::GP));
                                }
                            } else if rpl > cpl || cdesc.DPL != cpl {
                                return Err(EmuException::CPUException(CPUException::GP));
                            }
                            if self.ac.oasz.op != access::AcsSize::BIT64 && abs as u32 > ((cdesc.limit_h as u32) << 16) + cdesc.limit_l as u32 {
                                return Err(EmuException::CPUException(CPUException::GP));
                            }

                            let cache = self.ac.select_segdesc(SgReg::CS, cpl, Some(SegDescType::Code(cdesc)))?;

                            self.ac.[<push_ $type>](cs_sel)?;
                            self.ac.[<push_ $type>](ip)?;
                            self.ac.set_sgreg(SgReg::CS, (sel & 0xfff8) | cpl as u16, cache)?;
                            self.ac.set_ip(abs)?;
                        },
                        Some(DescType::System(SysDescType::Call(gate))) => {
                            if gate.DPL < cpl || gate.DPL < rpl { return Err(EmuException::CPUException(CPUException::GP)); }
                            let new_ip = ((gate.offset_h as u32) << 16) + gate.offset_l as u32;
                            let (sel, desc) = self.ac.select_callgate(gate)?;

                            if desc.DPL > cpl { return Err(EmuException::CPUException(CPUException::GP)); }

                            if !CodeDescFlag::from(&desc).contains(CodeDescFlag::C) && desc.DPL < cpl {
                                return Err(EmuException::NotImplementedFunction);
                            } else {
                                if self.ac.oasz.op != access::AcsSize::BIT64 && new_ip as u32 > ((desc.limit_h as u32) << 16) + desc.limit_l as u32 {
                                    return Err(EmuException::CPUException(CPUException::GP));
                                }

                                let cache = self.ac.select_segdesc(SgReg::CS, cpl, Some(SegDescType::Code(desc)))?;

                                self.ac.[<push_ $type>](cs_sel)?;
                                self.ac.[<push_ $type>](ip)?;
                                self.ac.set_sgreg(SgReg::CS, (sel & 0xfff8) | cpl as u16, cache)?;
                                self.ac.set_ip(ip)?;
                            }
                        },
                        Some(DescType::System(SysDescType::Task(gate))) => {
                            if gate.DPL < cpl || gate.DPL < rpl { return Err(EmuException::CPUException(CPUException::GP)); }
                            let tss_sel = gate.tss_sel;
                            let desc = self.ac.select_taskgate(gate)?;
                            self.ac.switch_task(TSMode::CallInt, tss_sel, desc)?;
                        },
                        Some(DescType::System(SysDescType::TSS(desc))) => {
                            if desc.DPL < cpl || desc.DPL < rpl { return Err(EmuException::CPUException(CPUException::GP)); }
                            self.ac.switch_task(TSMode::CallInt, sel, desc)?;
                        },
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

macro_rules! ret_far {
    ( $type:ty ) => { paste::item! {
        pub fn [<ret_far_ $type>](&mut self) -> Result<(), EmuException> {
            let new_ip = self.ac.[<pop_ $type>]()?;
            let new_cs = self.ac.[<pop_ $type>]()? as u16;

            match self.ac.mode {
                access::CpuMode::Real => {
                    self.ac.load_segment(SgReg::CS, new_cs)?;
                },
                access::CpuMode::Protected | access::CpuMode::Long => {
                    let cpl = self.ac.get_cpl()?;
                    let rpl = (new_cs & 3) as u8;

                    if rpl < cpl {
                        return Err(EmuException::CPUException(CPUException::GP));
                    }

                    if let Some(DescType::Segment(SegDescType::Code(cdesc))) = self.ac.obtain_gl_desc(new_cs)? {
                        if CodeDescFlag::from(&cdesc).contains(CodeDescFlag::C) && cdesc.DPL > rpl {
                            return Err(EmuException::CPUException(CPUException::GP));
                        }

                        if self.ac.oasz.op != access::AcsSize::BIT64 && new_ip as u32 > ((cdesc.limit_h as u32) << 16) + cdesc.limit_l as u32 {
                            return Err(EmuException::CPUException(CPUException::GP));
                        }

                        let cache = self.ac.select_segdesc(SgReg::CS, rpl, Some(SegDescType::Code(cdesc)))?;
                        self.ac.set_sgreg(SgReg::CS, new_cs, cache)?;
                    } else {
                        return Err(EmuException::CPUException(CPUException::GP));
                    }

                    if rpl > cpl {
                        let new_sp = self.ac.[<pop_ $type>]()?;
                        let new_ss = self.ac.[<pop_ $type>]()? as u16;
                        self.ac.set_gpreg(GpReg64::RSP, new_sp as u64)?;
                        self.ac.load_segment(SgReg::SS, new_ss)?;

                        for r in vec!(SgReg::ES, SgReg::FS, SgReg::GS, SgReg::DS).iter() {
                            if rpl > self.ac.get_sgreg(*r)?.1.DPL {
                                self.ac.set_sgreg(*r, 0, Default::default())?;
                            }
                        }
                    }
                },
            }

            self.ac.set_ip(new_ip)?;
            self.update_opadsize()
        }
    } };
}

macro_rules! int_ret {
    ( $type:ty ) => { paste::item! {
        pub fn [<int_ret_ $type>](&mut self) -> Result<(), EmuException> {
            let old_flag = self.ac.core.rflags;

            let new_ip   = self.ac.[<pop_ $type>]()?;
            let new_cs   = self.ac.[<pop_ $type>]()? as u16;
            let new_flag = self.ac.[<pop_ $type>]()? as u64;
            self.ac.core.rflags.from_u64(new_flag);

            match self.ac.mode {
                access::CpuMode::Real => {
                    self.ac.load_segment(SgReg::CS, new_cs)?;
                },
                access::CpuMode::Protected | access::CpuMode::Long => {
                    let cpl = self.ac.get_cpl()?;
                    let rpl = (new_cs & 3) as u8;

                    if old_flag.is_nesttask() {
                        return self.ac.restore_task();
                    }

                    if rpl < cpl {
                        return Err(EmuException::CPUException(CPUException::GP));
                    }

                    if let Some(DescType::Segment(SegDescType::Code(cdesc))) = self.ac.obtain_gl_desc(new_cs)? {
                        if CodeDescFlag::from(&cdesc).contains(CodeDescFlag::C) && cdesc.DPL > rpl {
                            return Err(EmuException::CPUException(CPUException::GP));
                        }

                        if self.ac.oasz.op != access::AcsSize::BIT64 && new_ip as u32 > ((cdesc.limit_h as u32) << 16) + cdesc.limit_l as u32 {
                            return Err(EmuException::CPUException(CPUException::GP));
                        }

                        let cache = self.ac.select_segdesc(SgReg::CS, rpl, Some(SegDescType::Code(cdesc)))?;
                        self.ac.set_sgreg(SgReg::CS, new_cs, cache)?;
                    } else {
                        return Err(EmuException::CPUException(CPUException::GP));
                    }

                    if rpl > cpl {
                        let new_sp = self.ac.[<pop_ $type>]()?;
                        let new_ss = self.ac.[<pop_ $type>]()? as u16;
                        self.ac.set_gpreg(GpReg64::RSP, new_sp as u64)?;
                        self.ac.load_segment(SgReg::SS, new_ss)?;

                        for r in vec!(SgReg::ES, SgReg::FS, SgReg::GS, SgReg::DS).iter() {
                            if rpl > self.ac.get_sgreg(*r)?.1.DPL {
                                self.ac.set_sgreg(*r, 0, Default::default())?;
                            }
                        }
                    }
                },
            }

            self.ac.set_ip(new_ip)?;
            self.update_opadsize()
        }
    } };
}

impl<'a> super::Exec<'a> {
    jmp_far!(u16);
    jmp_far!(u32);
    jmp_far!(u64);

    call_far!(u16);
    call_far!(u32);
    call_far!(u64);

    ret_far!(u16);
    ret_far!(u32);
    ret_far!(u64);

    int_ret!(u16);
    int_ret!(u32);
    int_ret!(u64);

    pub fn mov_to_sreg(&mut self, reg: SgReg, sel: u16) -> Result<(), EmuException> {
        if reg == SgReg::CS {
            return Err(EmuException::CPUException(CPUException::UD));
        }
        self.ac.load_segment(reg, sel)?;

        if reg == SgReg::SS {
            self.update_stacksize()?;
        }
        Ok(())
    }
}