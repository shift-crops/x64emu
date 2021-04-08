#![allow(non_snake_case)]
use packed_struct::prelude::*;
use num_enum::FromPrimitive;

#[derive(Clone, Copy, FromPrimitive)] #[repr(usize)]
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

impl SgDescSelector {
    pub fn to_u16(&self) -> u16 {
        u16::from_be_bytes(self.pack().unwrap())
    }

    pub fn from_u16(&mut self, v: u16) -> () {
        *self = SgDescSelector::unpack(&v.to_be_bytes()).unwrap();
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SgRegUnit {
    selector: SgDescSelector,
    cache: SgDescCache,
}

impl SgRegUnit {
    pub fn new() -> Self {
        SgRegUnit::default()
    }
}

#[derive(Clone, Copy)]
pub struct SgRegisters ([SgRegUnit; SGREGS_COUNT]);

impl SgRegisters {
    pub fn new() -> Self {
        SgRegisters ([SgRegUnit::new(); SGREGS_COUNT])
    }

    pub fn selector(&self, r: SgReg) -> &SgDescSelector { &self.0[r as usize].selector }
    pub fn cache(&self, r: SgReg) -> &SgDescCache { &self.0[r as usize].cache }

    pub fn selector_mut(&mut self, r: SgReg) -> &mut SgDescSelector { &mut self.0[r as usize].selector }
    pub fn cache_mut(&mut self, r: SgReg) -> &mut SgDescCache { &mut self.0[r as usize].cache }
}
