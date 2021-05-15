use crate::emulator::EmuException;

impl<'a> super::Exec<'a> {
    pub fn update_rflags_add<T: Into<u64> + num::traits::ops::overflowing::OverflowingAdd + Copy>(&mut self, v1: T, v2: T) -> Result<(), EmuException> {
        let (result, cf) = v1.overflowing_add(&v2);
        let (s1, s2, sr) = (Self::check_msb(v1), Self::check_msb(v2), Self::check_msb(result));
        let result = result.into();

        let rf = &mut self.ac.core.rflags;
        rf.set_carry(cf);
        rf.set_parity(Self::check_parity(result as u8));
        rf.set_zero(result == 0);
        rf.set_sign(sr);
        rf.set_overflow(!(s1^s2) && s1^sr);
        Ok(())
    }

    pub fn update_rflags_adc<T: Into<u64> + num::traits::ops::overflowing::OverflowingAdd + Copy>(&mut self, v1: T, v2: T, v3: T) -> Result<(), EmuException> {
        let (result, cf1) = v1.overflowing_add(&v2);
        let (result, cf2) = result.overflowing_add(&v3);
        let (s1, s2, sr) = (Self::check_msb(v1), Self::check_msb(v2), Self::check_msb(result));
        let result = result.into();

        let rf = &mut self.ac.core.rflags;
        rf.set_carry(cf1 || cf2);
        rf.set_parity(Self::check_parity(result as u8));
        rf.set_zero(result == 0);
        rf.set_sign(sr);
        rf.set_overflow(!(s1^s2) && s1^sr);
        Ok(())
    }

    pub fn update_rflags_sub<T: Into<u64> + num::traits::ops::overflowing::OverflowingSub + Copy>(&mut self, v1: T, v2: T) -> Result<(), EmuException> {
        let (result, cf) = v1.overflowing_sub(&v2);
        let (s1, s2, sr) = (Self::check_msb(v1), Self::check_msb(v2), Self::check_msb(result));
        let result = result.into();

        let rf = &mut self.ac.core.rflags;
        rf.set_carry(cf);
        rf.set_parity(Self::check_parity(result as u8));
        rf.set_zero(result == 0);
        rf.set_sign(sr);
        rf.set_overflow(s1^s2 && s1^sr);
        Ok(())
    }

    pub fn update_rflags_sbb<T: Into<u64> + num::traits::ops::overflowing::OverflowingSub + Copy>(&mut self, v1: T, v2: T, v3: T) -> Result<(), EmuException> {
        let (result, cf1) = v1.overflowing_sub(&v2);
        let (result, cf2) = result.overflowing_sub(&v3);
        let (s1, s2, sr) = (Self::check_msb(v1), Self::check_msb(v2), Self::check_msb(result));
        let result = result.into();

        let rf = &mut self.ac.core.rflags;
        rf.set_carry(cf1 || cf2);
        rf.set_parity(Self::check_parity(result as u8));
        rf.set_zero(result == 0);
        rf.set_sign(sr);
        rf.set_overflow(s1^s2 && s1^sr);
        Ok(())
    }

    pub fn update_rflags_mul<T: Into<u64> + num::traits::WrappingMul + Copy>(&mut self, v1: T, v2: T) -> Result<(), EmuException> {
        let result = v1.wrapping_mul(&v2);
        let of = Self::check_msb(result);

        let rf = &mut self.ac.core.rflags;
        rf.set_carry(of);
        rf.set_overflow(of);
        Ok(())
    }

    pub fn update_rflags_imul<T: Into<i64> + num::traits::ops::overflowing::OverflowingMul + Copy>(&mut self, v1: T, v2: T) -> Result<(), EmuException> {
        let of = !v1.overflowing_mul(&v2).1;

        let rf = &mut self.ac.core.rflags;
        rf.set_carry(of);
        rf.set_overflow(of);
        Ok(())
    }

    pub fn update_rflags_or<T: Into<u64> + std::ops::BitOr<Output = T> + Copy>(&mut self, v1: T, v2: T) -> Result<(), EmuException> {
        let result = v1 | v2;
        let sr = Self::check_msb(result);
        let result = result.into();

        let rf = &mut self.ac.core.rflags;
        rf.set_carry(false);
        rf.set_parity(Self::check_parity(result as u8));
        rf.set_zero(result == 0);
        rf.set_sign(sr);
        rf.set_overflow(false);
        Ok(())
    }

    pub fn update_rflags_and<T: Into<u64> + std::ops::BitAnd<Output = T> + Copy>(&mut self, v1: T, v2: T) -> Result<(), EmuException> {
        let result = v1 & v2;
        let sr = Self::check_msb(result);
        let result = result.into();

        let rf = &mut self.ac.core.rflags;
        rf.set_carry(false);
        rf.set_parity(Self::check_parity(result as u8));
        rf.set_zero(result == 0);
        rf.set_sign(sr);
        rf.set_overflow(false);
        Ok(())
    }

    pub fn update_rflags_xor<T: Into<u64> + std::ops::BitXor<Output = T> + Copy>(&mut self, v1: T, v2: T) -> Result<(), EmuException> {
        let result = v1 ^ v2;
        let sr = Self::check_msb(result);
        let result = result.into();

        let rf = &mut self.ac.core.rflags;
        rf.set_carry(false);
        rf.set_parity(Self::check_parity(result as u8));
        rf.set_zero(result == 0);
        rf.set_sign(sr);
        rf.set_overflow(false);
        Ok(())
    }

    pub fn update_rflags_shl<T: Into<u64> + num::traits::WrappingShl + Copy>(&mut self, v: T, c: u32) -> Result<(), EmuException> {
        if c == 0 { return Ok(()); }

        let result = v.wrapping_shl(c);
        let sf = Self::check_msb(result);
        let cf = Self::check_msb(v.wrapping_shl(c-1));
        let result = result.into();

        let rf = &mut self.ac.core.rflags;
        rf.set_carry(cf);
        rf.set_parity(Self::check_parity(result as u8));
        rf.set_zero(result == 0);
        rf.set_sign(sf);
        if c == 1 {
            rf.set_overflow(sf ^ cf);
        }
        Ok(())
    }

    pub fn update_rflags_shr<T: Into<u64> + num::traits::WrappingShr + Copy>(&mut self, v: T, c: u32) -> Result<(), EmuException> {
        if c == 0 { return Ok(()); }

        let result = v.wrapping_shr(c);
        let sf = Self::check_msb(result);
        let cf = Self::check_lsb(v.wrapping_shr(c-1));
        let result = result.into();

        let rf = &mut self.ac.core.rflags;
        rf.set_carry(cf);
        rf.set_parity(Self::check_parity(result as u8));
        rf.set_zero(result == 0);
        rf.set_sign(sf);
        if c == 1 {
            rf.set_overflow(Self::check_msb(v));
        }
        Ok(())
    }

    pub fn update_rflags_sar<T: Into<i64> + num::traits::WrappingShr + Copy>(&mut self, v: T, c: u32) -> Result<(), EmuException> {
        if c == 0 { return Ok(()); }

        let result = v.wrapping_shr(c).into();
        let sf = result < 0;
        let cf = v.wrapping_shr(c-1).into() & 1 != 0;

        let rf = &mut self.ac.core.rflags;
        rf.set_carry(cf);
        rf.set_parity(Self::check_parity(result as u8));
        rf.set_zero(result == 0);
        rf.set_sign(sf);
        if c == 1 {
            rf.set_overflow(false);
        }
        Ok(())
    }

    pub fn check_rflags_o(&self) -> Result<bool, EmuException> {
        let rf = self.ac.core.rflags;
        Ok(rf.is_overflow())
    }

    pub fn check_rflags_b(&self) -> Result<bool, EmuException> {
        let rf = self.ac.core.rflags;
        Ok(rf.is_carry())
    }

    pub fn check_rflags_z(&self) -> Result<bool, EmuException> {
        let rf = self.ac.core.rflags;
        Ok(rf.is_zero())
    }

    pub fn check_rflags_be(&self) -> Result<bool, EmuException> {
        let rf = self.ac.core.rflags;
        Ok(rf.is_carry() || rf.is_zero())
    }

    pub fn check_rflags_s(&self) -> Result<bool, EmuException> {
        let rf = self.ac.core.rflags;
        Ok(rf.is_sign())
    }

    pub fn check_rflags_p(&self) -> Result<bool, EmuException> {
        let rf = self.ac.core.rflags;
        Ok(rf.is_parity())
    }

    pub fn check_rflags_l(&self) -> Result<bool, EmuException> {
        let rf = self.ac.core.rflags;
        Ok(rf.is_sign() ^ rf.is_overflow())
    }

    pub fn check_rflags_le(&self) -> Result<bool, EmuException> {
        let rf = self.ac.core.rflags;
        Ok(rf.is_zero() || (rf.is_sign() ^ rf.is_overflow()))
    }

    fn check_parity(mut v: u8) -> bool {
        let mut c = 0;
        while v > 0 {
            c += v & 1;
            v >>= 1;
        }
        c % 2 == 0
    }

    fn check_msb<T: Into<u64> + Copy>(v: T) -> bool {
        (v.into() >> (std::mem::size_of::<T>()*8 - 1)) != 0
    }

    fn check_lsb<T: Into<u64> + Copy>(v: T) -> bool {
        v.into() & 1 != 0
    }
}