#![allow(non_snake_case)]
use packed_struct::prelude::*;
use num_enum::TryFromPrimitive;

#[derive(Clone, Copy, PartialEq, TryFromPrimitive)] #[repr(usize)]
pub enum SgReg { ES, CS, SS, DS, FS, GS, KernelGS, END }

const SGREGS_COUNT: usize = SgReg::END as usize;

#[derive(Debug, Default, Clone, Copy, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="2", endian="msb")]
pub struct SgDescSelector {
    #[packed_field(bits="0:1")]  pub RPL: u8,
    #[packed_field(bits="2")]    pub TI:  u8,
    #[packed_field(bits="3:15")] pub IDX: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct SgDescCache {
    pub base:  u64,
    pub limit: u32,
    pub Type:  u8,
    pub DPL:   u8,
    pub P:     u8,
    pub AVL:   u8,
    pub L:     u8,
    pub DB:    u8,
    pub G:     u8,
}
impl Default for SgDescCache {
    fn default() -> Self {
        Self{ base:0, limit:0xffff, Type:0, DPL:0, P:0, AVL:0, L:0, DB:0, G:0 }
    }
}

impl SgDescSelector {
    pub fn to_u16(&self) -> u16 {
        u16::from_be_bytes(self.pack().unwrap())
    }

    pub fn from_u16(&mut self, v: u16) -> () {
        *self = SgDescSelector::unpack(&v.to_be_bytes()).unwrap();
    }
}

impl super::model_specific::MSRAccess for SgDescCache {
    fn get(&self) -> u64 { self.base }
    fn set(&mut self, v: u64) -> () { self.base = v; }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SgRegUnit {
    pub selector: SgDescSelector,
    pub cache: SgDescCache,
}

pub struct SgRegisters ([SgRegUnit; SGREGS_COUNT]);

impl SgRegisters {
    pub fn new() -> Self {
        Self ([Default::default(); SGREGS_COUNT])
    }

    pub fn get(&self, r: SgReg) -> &SgRegUnit { &self.0[r as usize] }
    pub fn get_mut(&mut self, r: SgReg) -> &mut SgRegUnit { &mut self.0[r as usize] }
}

#[cfg(test)]
#[test]
pub fn sgreg_test() {
    let mut reg = SgRegisters::new();

    let mut sel = reg.get_mut(SgReg::ES).selector;
    sel.from_u16(0x2e);
    assert_eq!(sel.IDX, 5);
    assert_eq!(sel.TI, 1);
    assert_eq!(sel.RPL, 2);
}

#[cfg(test)]
#[test]
#[should_panic]
pub fn sgreg_test_panic() {
    use std::convert::TryFrom;

    let reg = SgRegisters::new();
    reg.get(SgReg::try_from(10).unwrap());
}