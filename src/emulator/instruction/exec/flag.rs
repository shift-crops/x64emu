use crate::emulator::instruction::InstrArg;

pub fn update_rflags_add<T: Into<u64> + num::traits::ops::overflowing::OverflowingAdd + Sized + Copy>(arg: &mut InstrArg, v1: T, v2: T) -> () {
    let (result, cf) = v1.overflowing_add(&v2);
    let result = result.into();
    let sz = std::mem::size_of::<T>()*8 - 1;
    let (s1, s2, sr) = ((v1.into() >> sz) != 0, (v2.into() >> sz) != 0, (result >> sz) != 0);

    let rf = &mut arg.ac.core.rflags;
    rf.set_carry(cf);
    rf.set_parity(check_parity(result as u8));
    rf.set_zero(result == 0);
    rf.set_sign(sr);
    rf.set_overflow(!(s1^s2) && s1^sr);
}

pub fn update_rflags_adc<T: Into<u64> + num::traits::ops::overflowing::OverflowingAdd + Sized + Copy>(arg: &mut InstrArg, v1: T, v2: T, v3: T) -> () {
    let (result, cf1) = v1.overflowing_add(&v2);
    let (result, cf2) = result.overflowing_add(&v3);
    let result = result.into();
    let sz = std::mem::size_of::<T>()*8 - 1;
    let (s1, s2, sr) = ((v1.into() >> sz) != 0, (v2.into() >> sz) != 0, (result >> sz) != 0);

    let rf = &mut arg.ac.core.rflags;
    rf.set_carry(cf1 || cf2);
    rf.set_parity(check_parity(result as u8));
    rf.set_zero(result == 0);
    rf.set_sign(sr);
    rf.set_overflow(!(s1^s2) && s1^sr);
}

pub fn update_rflags_sub<T: Into<u64> + num::traits::ops::overflowing::OverflowingSub + Sized + Copy>(arg: &mut InstrArg, v1: T, v2: T) -> () {
    let (result, cf) = v1.overflowing_sub(&v2);
    let result = result.into();
    let sz = std::mem::size_of::<T>()*8 - 1;
    let (s1, s2, sr) = ((v1.into() >> sz) != 0, (v2.into() >> sz) != 0, (result >> sz) != 0);

    let rf = &mut arg.ac.core.rflags;
    rf.set_carry(cf);
    rf.set_parity(check_parity(result as u8));
    rf.set_zero(result == 0);
    rf.set_sign(sr);
    rf.set_overflow(s1^s2 && s1^sr);
}

pub fn update_rflags_sbb<T: Into<u64> + num::traits::ops::overflowing::OverflowingSub + Sized + Copy>(arg: &mut InstrArg, v1: T, v2: T, v3: T) -> () {
    let (result, cf1) = v1.overflowing_sub(&v2);
    let (result, cf2) = result.overflowing_sub(&v3);
    let result = result.into();
    let sz = std::mem::size_of::<T>()*8 - 1;
    let (s1, s2, sr) = ((v1.into() >> sz) != 0, (v2.into() >> sz) != 0, (result >> sz) != 0);

    let rf = &mut arg.ac.core.rflags;
    rf.set_carry(cf1 || cf2);
    rf.set_parity(check_parity(result as u8));
    rf.set_zero(result == 0);
    rf.set_sign(sr);
    rf.set_overflow(s1^s2 && s1^sr);
}

pub fn update_rflags_mul<T: Into<u64> + num::traits::WrappingMul + Sized + Copy>(arg: &mut InstrArg, v1: T, v2: T) -> () {
    let ur = v1.wrapping_mul(&v2);
    let sz = std::mem::size_of::<T>()*8 - 1;
    let of = (ur.into() >> sz) != 0;

    let rf = &mut arg.ac.core.rflags;
    rf.set_carry(of);
    rf.set_overflow(of);
}

pub fn update_rflags_or<T: Into<u64> + std::ops::BitOr<Output = T> + Sized + Copy>(arg: &mut InstrArg, v1: T, v2: T) -> () {
    let ur = (v1 | v2).into();
    let sz = std::mem::size_of::<T>()*8 - 1;

    let rf = &mut arg.ac.core.rflags;
    rf.set_carry(false);
    rf.set_parity(check_parity(ur as u8));
    rf.set_zero(ur == 0);
    rf.set_sign((ur >> sz) != 0);
    rf.set_overflow(false);
}

pub fn update_rflags_and<T: Into<u64> + std::ops::BitAnd<Output = T> + Sized + Copy>(arg: &mut InstrArg, v1: T, v2: T) -> () {
    let ur = (v1 & v2).into();
    let sz = std::mem::size_of::<T>()*8 - 1;

    let rf = &mut arg.ac.core.rflags;
    rf.set_carry(false);
    rf.set_parity(check_parity(ur as u8));
    rf.set_zero(ur == 0);
    rf.set_sign((ur >> sz) != 0);
    rf.set_overflow(false);
}

pub fn update_rflags_xor<T: Into<u64> + std::ops::BitXor<Output = T> + Sized + Copy>(arg: &mut InstrArg, v1: T, v2: T) -> () {
    let ur = (v1 ^ v2).into();
    let sz = std::mem::size_of::<T>()*8 - 1;

    let rf = &mut arg.ac.core.rflags;
    rf.set_carry(false);
    rf.set_parity(check_parity(ur as u8));
    rf.set_zero(ur == 0);
    rf.set_sign((ur >> sz) != 0);
    rf.set_overflow(false);
}

pub fn check_rflags_o(arg: &InstrArg) -> bool {
    let rf = arg.ac.core.rflags;
    rf.is_overflow()
}

pub fn check_rflags_b(arg: &InstrArg) -> bool {
    let rf = arg.ac.core.rflags;
    rf.is_carry()
}

pub fn check_rflags_z(arg: &InstrArg) -> bool {
    let rf = arg.ac.core.rflags;
    rf.is_zero()
}

pub fn check_rflags_be(arg: &InstrArg) -> bool {
    let rf = arg.ac.core.rflags;
    rf.is_carry() || rf.is_zero()
}

pub fn check_rflags_s(arg: &InstrArg) -> bool {
    let rf = arg.ac.core.rflags;
    rf.is_sign()
}

pub fn check_rflags_p(arg: &InstrArg) -> bool {
    let rf = arg.ac.core.rflags;
    rf.is_parity()
}

pub fn check_rflags_l(arg: &InstrArg) -> bool {
    let rf = arg.ac.core.rflags;
    rf.is_sign() ^ rf.is_overflow()
}

pub fn check_rflags_le(arg: &InstrArg) -> bool {
    let rf = arg.ac.core.rflags;
    rf.is_zero() || (rf.is_sign() ^ rf.is_overflow())
}

fn check_parity(v: u8) -> bool {
    let mut pf = true;
    for i in 0..8 {
        pf ^= (v>>i) & 1 != 0;
    }
    pf
}