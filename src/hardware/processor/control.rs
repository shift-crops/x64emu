#![allow(non_snake_case)]
use packed_struct::prelude::*;

#[derive(Debug, Default)]
pub struct CRegisters (pub CR0, u64, pub CR2, pub CR3, pub CR4);
impl CRegisters {
    pub fn get(&self, v: usize) -> Option<&dyn CRAccess> {
        match v {
            0 => Some(&self.0),
            2 => Some(&self.2),
            3 => Some(&self.3),
            4 => Some(&self.4),
            _ => None,
        }
    }

    pub fn get_mut(&mut self, v: usize) -> Option<&mut dyn CRAccess> {
        match v {
            0 => Some(&mut self.0),
            2 => Some(&mut self.2),
            3 => Some(&mut self.3),
            4 => Some(&mut self.4),
            _ => None,
        }
    }
}

pub trait CRAccess {
    fn to_u32(&self) -> u32;
    fn from_u32(&mut self, v: u32) -> ();
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="4")]
pub struct CR0 {
    #[packed_field(bits="0")]  pub PE: u8,
    #[packed_field(bits="1")]  pub MP: u8,
    #[packed_field(bits="2")]  pub EM: u8,
    #[packed_field(bits="3")]  pub TS: u8,
    #[packed_field(bits="4")]  pub ET: u8,
    #[packed_field(bits="5")]  pub NE: u8,
    #[packed_field(bits="16")] pub WP: u8,
    #[packed_field(bits="18")] pub AM: u8,
    #[packed_field(bits="29")] pub NW: u8,
    #[packed_field(bits="30")] pub CD: u8,
    #[packed_field(bits="31")] pub PG: u8,
}
impl CRAccess for CR0 {
    fn to_u32(&self) -> u32 { u32::from_be_bytes(self.pack().unwrap()) }
    fn from_u32(&mut self, v: u32) -> () { *self = CR0::unpack(&v.to_be_bytes()).unwrap(); }
}

#[derive(Debug, Default)]
pub struct CR2(u32);
impl CRAccess for CR2 {
    fn to_u32(&self) -> u32 { self.0 }
    fn from_u32(&mut self, v: u32) -> () { self.0 = v; }
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="4", endian="msb")]
pub struct CR3 {
    #[packed_field(bits="3")]  PWT: u8,
    #[packed_field(bits="4")]  PCD: u8,
    #[packed_field(bits="12:31")]  PageDirBase: u32,
}
impl CRAccess for CR3 {
    fn to_u32(&self) -> u32 { u32::from_be_bytes(self.pack().unwrap()) }
    fn from_u32(&mut self, v: u32) -> () { *self = CR3::unpack(&v.to_be_bytes()).unwrap(); }
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="4")]
pub struct CR4 {
    #[packed_field(bits="0")]  VME: u8,
    #[packed_field(bits="1")]  PVI: u8,
    #[packed_field(bits="2")]  TSD: u8,
    #[packed_field(bits="3")]  DE:  u8,
    #[packed_field(bits="4")]  PSE: u8,
    #[packed_field(bits="5")]  PAE: u8,
    #[packed_field(bits="6")]  MCE: u8,
    #[packed_field(bits="7")]  PGE: u8,
    #[packed_field(bits="8")]  PCE: u8,
    #[packed_field(bits="9")]  OSFXSR: u8,
    #[packed_field(bits="10")] OSXMMEXCPT: u8,
}
impl CRAccess for CR4 {
    fn to_u32(&self) -> u32 { u32::from_be_bytes(self.pack().unwrap()) }
    fn from_u32(&mut self, v: u32) -> () { *self = CR4::unpack(&v.to_be_bytes()).unwrap(); }
}

#[cfg(test)]
#[test]
pub fn cr_test() {
    let mut cr: CRegisters = Default::default();

    cr.0.from_u32(0x50001);
    assert_eq!(cr.0.PE, 1);
    assert_eq!(cr.0.WP, 1);
    assert_eq!(cr.0.AM, 1);
    assert_eq!(cr.0.PG, 0);

    cr.3.PWT = 1;
    cr.3.PageDirBase = 0xdead;
    assert_eq!(cr.3.to_u32(), 0xdead008);
}