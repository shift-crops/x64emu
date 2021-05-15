use crate::emulator::*;
use crate::emulator::access::*;
use crate::emulator::access::register::*;
use crate::emulator::instruction::parse::Rep;

macro_rules! move_str {
    ( $size:expr ) => { paste::item! {
        pub fn [<move_str $size>](&mut self) -> Result<(), EmuException> {
            let step = if self.ac.core.rflags.is_direction() { -$size/8 } else { $size/8 };
            let si = self.get_si_addr(step)?;
            let di = self.get_di_addr(step)?;

            let src = self.ac.[<get_data $size>](si)?;
            self.ac.[<set_data $size>](di, src)
        }
    } };
}

macro_rules! cmp_str {
    ( $size:expr ) => { paste::item! {
        pub fn [<cmp_str $size>](&mut self) -> Result<(), EmuException> {
            let step = if self.ac.core.rflags.is_direction() { -$size/8 } else { $size/8 };
            let si = self.get_si_addr(step)?;
            let di = self.get_di_addr(step)?;

            let src = self.ac.[<get_data $size>](si)?;
            let dst = self.ac.[<get_data $size>](di)?;

            self.update_rflags_sub(src, dst)
        }
    } };
}

macro_rules! store_str {
    ( $size:expr, $reg:ident ) => { paste::item! {
        pub fn [<store_str $size>](&mut self) -> Result<(), EmuException> {
            let di = self.get_di_addr(if self.ac.core.rflags.is_direction() { -$size/8 } else { $size/8 })?;

            let src = self.[<get_ $reg>]()?;
            self.ac.[<set_data $size>](di, src)
        }
    } };
}

macro_rules! load_str {
    ( $size:expr, $reg:ident ) => { paste::item! {
        pub fn [<load_str $size>](&mut self) -> Result<(), EmuException> {
            let si = self.get_si_addr(if self.ac.core.rflags.is_direction() { -$size/8 } else { $size/8 })?;

            let src = self.ac.[<get_data $size>](si)?;
            self.[<set_ $reg>](src)
        }
    } };
}

macro_rules! scan_str {
    ( $size:expr, $reg:ident ) => { paste::item! {
        pub fn [<scan_str $size>](&mut self) -> Result<(), EmuException> {
            let di = self.get_di_addr(if self.ac.core.rflags.is_direction() { -$size/8 } else { $size/8 })?;

            let src = self.[<get_ $reg>]()?;
            let dst = self.ac.[<get_data $size>](di)?;

            self.update_rflags_sub(src, dst)
        }
    } };
}

macro_rules! repeat_reg {
    ( $size:expr, $reg:ty ) => { paste::item! {
        pub fn [<repeat_ $size>](&mut self) -> Result<(), EmuException> {
            let mut rep = false;
            if let Some(r) = &self.pdata.repeat {
                self.ac.update_gpreg($reg, -1)?;
                rep = match (self.ac.get_gpreg($reg)?, r, self.ac.core.rflags.is_zero()) {
                (0, _, _) | (_, Rep::REPZ, false) | (_, Rep::REPNZ, true) => false,
                _ => true,
                }
            }
            if rep { self.ac.update_ip(-(self.idata.len as i16))?; }
            Ok(())
        }
    } };
}

impl<'a> super::Exec<'a> {
    move_str!(8);
    move_str!(16);
    move_str!(32);
    move_str!(64);

    cmp_str!(8);
    cmp_str!(16);
    cmp_str!(32);
    cmp_str!(64);

    store_str!(8, al);
    store_str!(16, ax);
    store_str!(32, eax);
    store_str!(64, rax);

    load_str!(8, al);
    load_str!(16, ax);
    load_str!(32, eax);
    load_str!(64, rax);

    scan_str!(8, al);
    scan_str!(16, ax);
    scan_str!(32, eax);
    scan_str!(64, rax);

    fn get_si_addr(&mut self, step: i64) -> Result<(SgReg, u64), EmuException> {
        let seg = self.pdata.segment.unwrap_or(SgReg::DS);
        let addr = self.ac.get_gpreg(GpReg64::RSI)?;

        let addr = match self.idata.adsize {
            AcsSize::BIT16 => {
                self.ac.update_gpreg(GpReg16::SI, step as i16)?;
                addr as u16 as u64
            },
            AcsSize::BIT32 => {
                self.ac.update_gpreg(GpReg32::ESI, step as i32)?;
                addr as u32 as u64
            },
            AcsSize::BIT64 => {
                self.ac.update_gpreg(GpReg64::RSI, step)?;
                addr
            },
        };
        Ok((seg, addr))
    }

    fn get_di_addr(&mut self, step: i64) -> Result<(SgReg, u64), EmuException> {
        let addr = self.ac.get_gpreg(GpReg64::RDI)?;

        let addr = match self.idata.adsize {
            AcsSize::BIT16 => {
                self.ac.update_gpreg(GpReg16::DI, step as i16)?;
                addr as u16 as u64
            },
            AcsSize::BIT32 => {
                self.ac.update_gpreg(GpReg32::EDI, step as i32)?;
                addr as u32 as u64
            },
            AcsSize::BIT64 => {
                self.ac.update_gpreg(GpReg64::RDI, step)?;
                addr
            },
        };
        Ok((SgReg::ES, addr))
    }

    repeat_reg!(16, GpReg16::CX);
    repeat_reg!(32, GpReg32::ECX);
    repeat_reg!(64, GpReg64::RCX);
}