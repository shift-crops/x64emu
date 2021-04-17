#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use packed_struct::prelude::*;

#[derive(Default)]
pub struct ModelSpecific {
    pub efer: IA32_EFER,
    pub apic: IA32_APIC_BASE,
    pub star: STAR,
    pub lstar: LSTAR,
    pub cstar: CSTAR,
    pub fmask: FMASK,
}

pub trait MSRAccess {
    fn get(&self) -> u64;
    fn set(&mut self, v: u64) -> ();
}

#[derive(Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="8")]
pub struct IA32_EFER {
    #[packed_field(bits="0")]  pub SCE: u8,
    #[packed_field(bits="1:7")]   _r01: ReservedZero<packed_bits::Bits7>,
    #[packed_field(bits="8")]  pub LME: u8,
    #[packed_field(bits="9")]     _r09: ReservedZero<packed_bits::Bits1>,
    #[packed_field(bits="10")] pub LMA: u8,
    #[packed_field(bits="11")] pub NXE: u8,
    #[packed_field(bits="12:63")] _r11: ReservedZero<packed_bits::Bits51>,
}
impl MSRAccess for IA32_EFER {
    fn get(&self) -> u64 { u64::from_be_bytes(self.pack().unwrap()) }
    fn set(&mut self, v: u64) -> () { *self = Self::unpack(&v.to_be_bytes()).unwrap(); }
}

#[derive(Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="8", endian="msb")]
pub struct IA32_APIC_BASE {
    #[packed_field(bits="8")]  pub BSP: u8,
    #[packed_field(bits="11")] pub G: u8,
    #[packed_field(bits="12:35")] pub Base: u32,
}
impl MSRAccess for IA32_APIC_BASE {
    fn get(&self) -> u64 { u64::from_be_bytes(self.pack().unwrap()) }
    fn set(&mut self, v: u64) -> () { *self = Self::unpack(&v.to_be_bytes()).unwrap(); }
}

#[derive(Default)]
pub struct STAR {
    pub ip: u32,
    pub cs: u16,
    pub l_csss: u16,
}
impl MSRAccess for STAR {
    fn get(&self) -> u64 { ((self.l_csss as u64) << 48) + ((self.cs as u64) << 32) + self.ip as u64 }
    fn set(&mut self, v: u64) -> () {
        self.ip = v as u32;
        self.cs = (v >> 32) as u16;
        self.l_csss = (v >> 48) as u16;
    }
}

#[derive(Default)]
pub struct LSTAR (u64);
impl MSRAccess for LSTAR {
    fn get(&self) -> u64 { self.0 }
    fn set(&mut self, v: u64) -> () { self.0 = v; }
}

#[derive(Default)]
pub struct CSTAR(u64);
impl MSRAccess for CSTAR {
    fn get(&self) -> u64 { self.0 }
    fn set(&mut self, v: u64) -> () { self.0 = v; }
}

#[derive(Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="8", endian="msb")]
pub struct FMASK {
    #[packed_field(bits="0:31")]  pub mask: u32,
}
impl MSRAccess for FMASK {
    fn get(&self) -> u64 { u64::from_be_bytes(self.pack().unwrap()) }
    fn set(&mut self, v: u64) -> () { *self = Self::unpack(&v.to_be_bytes()).unwrap(); }
}

#[cfg(test)]
#[test]
pub fn msr_test() {
    let mut msr: ModelSpecific = Default::default();

    msr.efer.set(0x401);
    assert_eq!(msr.efer.LMA, 1);
    assert_eq!(msr.efer.LME, 0);
    assert_eq!(msr.efer.SCE, 1);

    msr.apic.set(0xdead100);
    assert_eq!(msr.apic.BSP, 1);
    assert_eq!(msr.apic.G, 0);
    assert_eq!(msr.apic.Base, 0xdead);


    msr.star.set(0x11223344deadbeef);
    assert_eq!(msr.star.ip, 0xdeadbeef);
    assert_eq!(msr.star.cs, 0x3344);

    msr.fmask.set(0xdeadbeef);
    assert_eq!(msr.fmask.mask, 0xdeadbeef);
}