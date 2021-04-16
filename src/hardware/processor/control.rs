#![allow(non_snake_case)]
use packed_struct::prelude::*;

#[derive(Debug, Default, Clone, Copy, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="4")]
pub struct CR0 {
    #[packed_field(bits="0")]  PE: u8,
    #[packed_field(bits="1")]  MP: u8,
    #[packed_field(bits="2")]  EM: u8,
    #[packed_field(bits="3")]  TS: u8,
    #[packed_field(bits="4")]  ET: u8,
    #[packed_field(bits="5")]  NE: u8,
    #[packed_field(bits="16")] WP: u8,
    #[packed_field(bits="18")] AM: u8,
    #[packed_field(bits="29")] NW: u8,
    #[packed_field(bits="30")] CD: u8,
    #[packed_field(bits="31")] PG: u8,
}

pub struct CR2 (u64);

#[derive(Debug, Default, Clone, Copy, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="4", endian="msb")]
pub struct CR3 {
    #[packed_field(bits="3")]  PWT: u8,
    #[packed_field(bits="4")]  PCD: u8,
    #[packed_field(bits="12:31")]  PageDirBase: u32,
}

#[derive(Debug, Default, Clone, Copy, PackedStruct)]
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
