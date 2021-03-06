use packed_struct::prelude::*;

#[derive(Debug, Default, Clone, Copy, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="8")]
pub struct RFlags {
    #[packed_field(bits="0")]  CF:  u8,
    #[packed_field(bits="1")] _r01: ReservedOnes<packed_bits::Bits1>,
    #[packed_field(bits="2")]  PF:  u8,
    //#[packed_field(bits="3")] _r03: ReservedZero<packed_bits::Bits1>,
    #[packed_field(bits="4")]  AF:  u8,
    //#[packed_field(bits="5")] _r05: ReservedZero<packed_bits::Bits1>,
    #[packed_field(bits="6")]  ZF:  u8,
    #[packed_field(bits="7")]  SF:  u8,
    #[packed_field(bits="8")]  TF:  u8,
    #[packed_field(bits="9")]  IF:  u8,
    #[packed_field(bits="10")] DF:  u8,
    #[packed_field(bits="11")] OF:  u8,
    #[packed_field(bits="12:13")] IOPL: u8,
    #[packed_field(bits="14")] NT:  u8,
    //#[packed_field(bits="15")] _r15: ReservedZero<packed_bits::Bits1>,
    #[packed_field(bits="16")] RF:  u8,
    #[packed_field(bits="17")] VM:  u8,
    #[packed_field(bits="18")] AC:  u8,
    #[packed_field(bits="19")] VIF: u8,
    #[packed_field(bits="20")] VIP: u8,
    #[packed_field(bits="21")] ID:  u8,
}

impl RFlags {
    pub fn to_u64(&self) -> u64 { u64::from_be_bytes(self.pack().unwrap()) }
    pub fn from_u64(&mut self, v: u64) -> () { *self = RFlags::unpack(&v.to_be_bytes()).unwrap(); }

    pub fn is_carry(&self) -> bool { self.CF != 0 }
    pub fn is_parity(&self) -> bool { self.PF != 0 }
    pub fn is_zero(&self) -> bool { self.ZF != 0 }
    pub fn is_sign(&self) -> bool { self.SF != 0 }
    pub fn is_trap(&self) -> bool { self.TF != 0 }
    pub fn is_interrupt(&self) -> bool { self.IF != 0 }
    pub fn is_direction(&self) -> bool { self.DF != 0 }
    pub fn is_overflow(&self) -> bool { self.OF != 0 }
    pub fn is_nesttask(&self) -> bool { self.NT != 0 }
    pub fn get_iopl(&self) -> u8 { self.IOPL }

    pub fn set_carry(&mut self, f: bool) -> () { self.CF = f as u8; }
    pub fn set_parity(&mut self, f: bool) -> () { self.PF = f as u8; }
    pub fn set_zero(&mut self, f: bool) -> () { self.ZF = f as u8; }
    pub fn set_sign(&mut self, f: bool) -> () { self.SF = f as u8; }
    pub fn set_trap(&mut self, f: bool) -> () { self.TF = f as u8; }
    pub fn set_interrupt(&mut self, f: bool) -> () { self.IF = f as u8; }
    pub fn set_direction(&mut self, f: bool) -> () { self.DF = f as u8; }
    pub fn set_overflow(&mut self, f: bool) -> () { self.OF = f as u8; }
    pub fn set_nesttask(&mut self, f: bool) -> () { self.NT = f as u8; }
    pub fn set_iopl(&mut self, pl: u8) -> () { self.IOPL = pl; }
}

#[cfg(test)]
#[test]
fn rflags_test() {
    let mut flag: RFlags = Default::default();

    flag.from_u64(0);
    assert_eq!(flag.to_u64(), 2);
    flag.set_carry(true);
    assert_eq!(flag.to_u64(), 3);
}