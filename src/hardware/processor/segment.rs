#![allow(non_snake_case)]
use packed_struct::prelude::*;
use num_enum::FromPrimitive;

#[derive(FromPrimitive)] #[repr(usize)]
pub enum SgReg { #[num_enum(default)] ES, CS, SS, DS, FS, GS } 

const SGREGS_COUNT: usize = 6;

#[derive(Debug, Default, Clone, Copy, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="2", endian="msb")]
pub struct SgDescSelector {
    #[packed_field(bits="0:1")]  RPL: u8,
    #[packed_field(bits="2")]    TI:  u8,
    #[packed_field(bits="3:15")] IDX: u16,
}

#[derive(Debug, Default, Clone, Copy, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="8", endian="msb")]
pub struct SgDescCache {
    #[packed_field(bits="0:31")]  Base:  u32,
    #[packed_field(bits="32:51")] Limit: u32,
    #[packed_field(bits="52:55")] Type:  u8,
    #[packed_field(bits="56")]    S:     u8,
    #[packed_field(bits="57:58")] DPL:   u8,
    #[packed_field(bits="59")]    P:     u8,
    #[packed_field(bits="60")]    AVL:   u8,
    #[packed_field(bits="62")]    DB:    u8,
    #[packed_field(bits="63")]    G:     u8,
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
pub struct SgRegisters {
    regs: [SgRegUnit; SGREGS_COUNT],
}

impl SgRegisters {
    pub fn new() -> Self {
        SgRegisters {regs: [SgRegUnit::new(); SGREGS_COUNT]}
    }

    pub fn get_sel(&self, r: SgReg) -> u16 { self.regs[r as usize].selector.to_u16() }
    pub fn set_sel(&mut self, r: SgReg, v: u16) -> () { self.regs[r as usize].selector.from_u16(v); }

    pub fn get(&self, r: SgReg) -> SgRegUnit { self.regs[r as usize] }
}
