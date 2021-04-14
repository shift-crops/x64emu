#![allow(non_snake_case)]
use packed_struct::prelude::*;
use num_enum::TryFromPrimitive;

#[derive(Clone, Copy, TryFromPrimitive)] #[repr(usize)]
pub enum SgReg { ES, CS, SS, DS, FS, GS, KernelGS, END }

const SGREGS_COUNT: usize = SgReg::END as usize;

#[derive(Debug, Default, Clone, Copy, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="2", endian="msb")]
pub struct SgDescSelector {
    #[packed_field(bits="0:1")]  pub RPL: u8,
    #[packed_field(bits="2")]    pub TI:  u8,
    #[packed_field(bits="3:15")] pub IDX: u16,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SgDescCache {
    pub Base:  u64,
    pub Limit: u32,
    pub Type:  u8,
    pub S:     u8,
    pub DPL:   u8,
    pub P:     u8,
    pub AVL:   u8,
    pub L:     u8,
    pub D:     u8,
    pub G:     u8,
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