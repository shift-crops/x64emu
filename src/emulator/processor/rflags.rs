#![allow(non_snake_case)]
use packed_struct::prelude::*;

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="8")]
pub struct RFlags {
    #[packed_field(bits="0")]  pub CF:  u8,
    //#[packed_field(bits="1")] _r01: ReservedOnes<packed_bits::Bits1>,
    #[packed_field(bits="2")]  pub PF:  u8,
    //#[packed_field(bits="3")] _r03: ReservedZero<packed_bits::Bits1>,
    #[packed_field(bits="4")]  pub AF:  u8,
    //#[packed_field(bits="5")] _r05: ReservedZero<packed_bits::Bits1>,
    #[packed_field(bits="6")]  pub ZF:  u8,
    #[packed_field(bits="7")]  pub SF:  u8,
    #[packed_field(bits="8")]  pub TF:  u8,
    #[packed_field(bits="9")]  pub IF:  u8,
    #[packed_field(bits="10")] pub DF:  u8,
    #[packed_field(bits="11")] pub OF:  u8,
    #[packed_field(bits="12:13")] pub IOPL: u8,
    #[packed_field(bits="14")] pub NT:  u8,
    //#[packed_field(bits="15")] _r15: ReservedZero<packed_bits::Bits1>,
    #[packed_field(bits="16")] pub RF:  u8,
    #[packed_field(bits="17")] pub VM:  u8,
    #[packed_field(bits="18")] pub AC:  u8,
    #[packed_field(bits="19")] pub VIF: u8,
    #[packed_field(bits="20")] pub VIP: u8,
    #[packed_field(bits="21")] pub ID:  u8,
}

impl RFlags {
    pub fn new() -> RFlags {
        RFlags::default()
    }

    pub fn to_u64(&self) -> u64 {
        u64::from_be_bytes(self.pack().unwrap())
    }

    pub fn from_u64(v: u64) -> Self {
        RFlags::unpack(&v.to_be_bytes()).unwrap()
    }
}