#![allow(non_snake_case)]
use packed_struct::prelude::*;
use num_enum::FromPrimitive;

#[derive(FromPrimitive)] #[repr(usize)]
pub enum SgReg { #[num_enum(default)] ES, CS, SS, DS, FS, GS } 

const SGREGS_COUNT: usize = 6;

#[derive(Debug, Default, Clone, Copy, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="2", endian="msb")]
pub struct SgDescSelector {
    #[packed_field(bits="0:1")]  pub RPL: u8,
    #[packed_field(bits="2")]    pub TI:  u8,
    #[packed_field(bits="3:15")] pub IDX: u16,
}

#[derive(Debug, Default, Clone, Copy, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="8", endian="msb")]
pub struct SgDescCache {
    #[packed_field(bits="0:31")]  pub Base:  u32,
    #[packed_field(bits="32:51")] pub Limit: u32,
    #[packed_field(bits="52:55")] pub Type:  u8,
    #[packed_field(bits="56")]    pub S:     u8,
    #[packed_field(bits="57:58")] pub DPL:   u8,
    #[packed_field(bits="59")]    pub P:     u8,
    #[packed_field(bits="60")]    pub AVL:   u8,
    #[packed_field(bits="62")]    pub DB:    u8,
    #[packed_field(bits="63")]    pub G:     u8,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SgRegUnit {
    pub selector: SgDescSelector,
    pub cache: SgDescCache,
}

impl SgRegUnit {
    pub fn new() -> SgRegUnit {
        SgRegUnit::default()
    }

    /*
    pub fn to_u16(&self) -> u16 {
        u16::from_be_bytes(self.pack().unwrap())
    }

    pub fn from_u16(v: u16) -> Self {
        SgRegUnit::unpack(&v.to_be_bytes()).unwrap()
    }
    */
}

pub struct SgRegisters {
    pub regs: [SgRegUnit; SGREGS_COUNT],
}

impl SgRegisters {
    pub fn new() -> Self {
        let sgr = SgRegUnit::new();
        SgRegisters {regs: [sgr; SGREGS_COUNT]}
    }
}
