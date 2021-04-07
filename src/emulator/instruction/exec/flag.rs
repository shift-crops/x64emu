use crate::emulator::instruction::InstrArg;

pub fn update_rflags_add<T: Into<u64> + num::traits::ops::overflowing::OverflowingAdd + Sized + Copy>(arg: &mut InstrArg, v1: T, v2: T) -> () {
    let (ur, cf) = v1.overflowing_add(&v2);
    let ur = ur.into();
    let size = std::mem::size_of::<T>()*8;
    let (sr, of) = ((v1.into() >> (size-8)) as i8).overflowing_add((v2.into() >> (size-8)) as i8);

    let rf = arg.ac.core.rflags_mut();
    rf.set_carry(cf);
    rf.set_parity(check_parity(ur as u8));
    rf.set_zero(ur == 0);
    rf.set_sign(sr.is_negative());
    rf.set_overflow(of);
}

pub fn update_rflags_sub<T: Into<u64> + num::traits::ops::overflowing::OverflowingSub + Sized + Copy>(arg: &mut InstrArg, v1: T, v2: T) -> () {
    let (ur, cf) = v1.overflowing_sub(&v2);
    let ur = ur.into();
    let size = std::mem::size_of::<T>()*8;
    let (sr, of) = ((v1.into() >> (size-8)) as i8).overflowing_sub((v2.into() >> (size-8)) as i8);

    let rf = arg.ac.core.rflags_mut();
    rf.set_carry(cf);
    rf.set_parity(check_parity(ur as u8));
    rf.set_zero(ur == 0);
    rf.set_sign(sr.is_negative());
    rf.set_overflow(of);
}

pub fn update_rflags_mul<T: Into<u64> + num::traits::WrappingMul + Sized + Copy>(arg: &mut InstrArg, v1: T, v2: T) -> () {
    let ur = v1.wrapping_mul(&v2);
    let size = std::mem::size_of::<T>()*8;
    let of = (ur.into() >> (size-1)) != 0;

    let rf = arg.ac.core.rflags_mut();
    rf.set_carry(of);
    rf.set_overflow(of);
}

pub fn update_rflags_or<T: Into<u64> + std::ops::BitOr<Output = T> + Sized + Copy>(arg: &mut InstrArg, v1: T, v2: T) -> () {
    let ur = (v1 | v2).into();
    let size = std::mem::size_of::<T>()*8;

    let rf = arg.ac.core.rflags_mut();
    rf.set_carry(false);
    rf.set_parity(check_parity(ur as u8));
    rf.set_zero(ur == 0);
    rf.set_sign((ur >> (size-1)) != 0);
    rf.set_overflow(false);
}

pub fn update_rflags_and<T: Into<u64> + std::ops::BitAnd<Output = T> + Sized + Copy>(arg: &mut InstrArg, v1: T, v2: T) -> () {
    let ur = (v1 & v2).into();
    let size = std::mem::size_of::<T>()*8;

    let rf = arg.ac.core.rflags_mut();
    rf.set_carry(false);
    rf.set_parity(check_parity(ur as u8));
    rf.set_zero(ur == 0);
    rf.set_sign((ur >> (size-1)) != 0);
    rf.set_overflow(false);
}

pub fn update_rflags_xor<T: Into<u64> + std::ops::BitXor<Output = T> + Sized + Copy>(arg: &mut InstrArg, v1: T, v2: T) -> () {
    let ur = (v1 ^ v2).into();
    let size = std::mem::size_of::<T>()*8;

    let rf = arg.ac.core.rflags_mut();
    rf.set_carry(false);
    rf.set_parity(check_parity(ur as u8));
    rf.set_zero(ur == 0);
    rf.set_sign((ur >> (size-1)) != 0);
    rf.set_overflow(false);
}

fn check_parity(v: u8) -> bool {
    let mut pf = true;
    for i in 0..8 {
        pf ^= (v>>i) & 1 != 0;
    }
    pf
}