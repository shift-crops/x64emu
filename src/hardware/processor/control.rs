#![allow(non_snake_case)]
use packed_struct::prelude::*;

#[derive(Debug, Default)]
pub struct CRegisters (pub CR0, u64, pub u64, pub CR3, pub CR4);

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

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="4", endian="msb")]
pub struct CR3 {
    #[packed_field(bits="3")]  PWT: u8,
    #[packed_field(bits="4")]  PCD: u8,
    #[packed_field(bits="12:31")]  PageDirBase: u32,
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
