use num_enum::TryFromPrimitive;

#[derive(TryFromPrimitive, Clone, Copy)] #[repr(usize)]
pub enum GpReg64 { RAX, RCX, RDX, RBX, RSP, RBP, RSI, RDI, R8, R9, R10, R11, R12, R13, R14, R15, END }
#[derive(TryFromPrimitive, Clone, Copy)] #[repr(usize)]
pub enum GpReg32 { EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI, R8D, R9D, R10D, R11D, R12D, R13D, R14D, R15D }
#[derive(TryFromPrimitive, Clone, Copy)] #[repr(usize)]
pub enum GpReg16 { AX, CX, DX, BX, SP, BP, SI, DI, R8W, R9W, R10W, R11W, R12W, R13W, R14W, R15W }
#[derive(TryFromPrimitive, Clone, Copy)] #[repr(usize)]
pub enum GpReg8h { AH, CH, DH, BH }
#[derive(TryFromPrimitive, Clone, Copy)] #[repr(usize)]
pub enum GpReg8l { AL, CL, DL, BL, SPL, BPL, SIL, DIL, R8B, R9B, R10B, R11B, R12B, R13B, R14B, R15B }

const GPREGS_COUNT: usize = GpReg64::END as usize;

#[repr(C)]
#[derive(Clone, Copy)]
union GpRegUnit {
    reg64: u64,
    reg16: u16,
    reg8: [u8; 2],
}

impl GpRegUnit {
    fn new() -> Self {
        Self {reg64: 0}
    }
}

#[derive(Clone, Copy)]
pub struct GpRegisters([GpRegUnit; GPREGS_COUNT]);

impl GpRegisters {
    pub fn new() -> Self {
        Self ([GpRegUnit::new(); GPREGS_COUNT])
    }

    pub fn get64(&self, r: GpReg64) -> u64 { unsafe { self.0[r as usize].reg64 } }
    pub fn get32(&self, r: GpReg32) -> u32 { unsafe { self.0[r as usize].reg64 as u32 } }
    pub fn get16(&self, r: GpReg16) -> u16 { unsafe { self.0[r as usize].reg16 } }
    pub fn get8h(&self, r: GpReg8h) -> u8 { unsafe { self.0[r as usize].reg8[1] } }
    pub fn get8l(&self, r: GpReg8l) -> u8 { unsafe { self.0[r as usize].reg8[0] } }

    pub fn set64(&mut self, r: GpReg64, v: u64) -> () { self.0[r as usize].reg64 = v; }
    pub fn set32(&mut self, r: GpReg32, v: u32) -> () { self.0[r as usize].reg64 = v as u64; }
    pub fn set16(&mut self, r: GpReg16, v: u16) -> () { self.0[r as usize].reg16 = v; }
    pub fn set8h(&mut self, r: GpReg8h, v: u8) -> () { unsafe { self.0[r as usize].reg8[1] = v; } }
    pub fn set8l(&mut self, r: GpReg8l, v: u8) -> () { unsafe { self.0[r as usize].reg8[0] = v; } }

    pub fn update64(&mut self, r: GpReg64, v: i64) -> () { unsafe { self.0[r as usize].reg64 = self.0[r as usize].reg64.wrapping_add(v as u64); } }
    pub fn update32(&mut self, r: GpReg32, v: i32) -> () { unsafe { self.0[r as usize].reg64 = (self.0[r as usize].reg64 as u32).wrapping_add(v as u32) as u64; } }
    pub fn update16(&mut self, r: GpReg16, v: i16) -> () { unsafe { self.0[r as usize].reg16 = self.0[r as usize].reg16.wrapping_add(v as u16); } }
    pub fn update8h(&mut self, r: GpReg8h, v: i8) -> () { unsafe { self.0[r as usize].reg8[1] = self.0[r as usize].reg8[1].wrapping_add(v as u8); } }
    pub fn update8l(&mut self, r: GpReg8l, v: i8) -> () { unsafe { self.0[r as usize].reg8[0] = self.0[r as usize].reg8[0].wrapping_add(v as u8); } }
}

#[cfg(test)]
#[test]
fn gpreg_test() {
    let mut reg = GpRegisters::new();

    reg.set64(GpReg64::RAX, 0xdeadbeefcafebabe);
    reg.set16(GpReg16::AX, 0x1122);
    reg.set8h(GpReg8h::AH, 0x00);
    reg.update64(GpReg64::RAX, -0x10);
    assert_eq!(reg.get64(GpReg64::RAX), 0xdeadbeefcafe0012);
    reg.update32(GpReg32::EAX, 0x10000000);
    assert_eq!(reg.get64(GpReg64::RAX), 0xdafe0012);

    reg.set32(GpReg32::EAX, 0x11223344);

    reg.set32(GpReg32::EDI, 0xc0bebeef);
    reg.set8l(GpReg8l::DIL, 0xff);
    assert_eq!(reg.get64(GpReg64::RDI), 0xc0bebeff);
}

#[cfg(test)]
#[test]
#[should_panic]
fn gpreg_test_panic() {
    use std::convert::TryFrom;
    let reg = GpRegisters::new();
    reg.get64(GpReg64::try_from(20).unwrap());
}