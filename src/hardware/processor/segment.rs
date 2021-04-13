#![allow(non_snake_case)]
use packed_struct::prelude::*;
use num_enum::TryFromPrimitive;

#[derive(Clone, Copy, TryFromPrimitive)] #[repr(usize)]
pub enum SgReg { ES, CS, SS, DS, FS, GS, END }

const SGREGS_COUNT: usize = SgReg::END as usize;

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

#[derive(Clone, Copy)]
pub struct SgRegisters ([SgRegUnit; SGREGS_COUNT]);

impl SgRegisters {
    pub fn new() -> Self {
        Self ([Default::default(); SGREGS_COUNT])
    }

    pub fn selector(&self, r: SgReg) -> &SgDescSelector { &self.0[r as usize].selector }
    pub fn cache(&self, r: SgReg) -> &SgDescCache { &self.0[r as usize].cache }

    pub fn selector_mut(&mut self, r: SgReg) -> &mut SgDescSelector { &mut self.0[r as usize].selector }
    pub fn cache_mut(&mut self, r: SgReg) -> &mut SgDescCache { &mut self.0[r as usize].cache }
}

#[cfg(test)]
#[test]
pub fn sgreg_test() {
    let mut reg = SgRegisters::new();

    reg.selector_mut(SgReg::ES).from_u16(0x2e);
    let es = reg.selector(SgReg::ES);
    assert_eq!(es.IDX, 5);
    assert_eq!(es.TI, 1);
    assert_eq!(es.RPL, 2);
}

#[cfg(test)]
#[test]
#[should_panic]
pub fn sgreg_test_panic() {
    use std::convert::TryFrom;

    let reg = SgRegisters::new();
    reg.selector(SgReg::try_from(10).unwrap());
}