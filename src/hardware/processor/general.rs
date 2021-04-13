use num_enum::TryFromPrimitive;

#[derive(TryFromPrimitive, Clone, Copy)] #[repr(usize)]
pub enum GpReg64 { RAX, RCX, RDX, RBX, RSP, RBP, RSI, RDI, R8, R9, R10, R11, R12, R13, R14, R15, END }
#[derive(TryFromPrimitive, Clone, Copy)] #[repr(usize)]
pub enum GpReg32 { EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI, R8D, R9D, R10D, R11D, R12D, R13D, R14D, R15D }
#[derive(TryFromPrimitive, Clone, Copy)] #[repr(usize)]
pub enum GpReg16 { AX, CX, DX, BX, SP, BP, SI, DI, R8W, R9W, R10W, R11W, R12W, R13W, R14W, R15W }
#[derive(TryFromPrimitive, Clone, Copy)] #[repr(usize)]
pub enum GpReg8 { AL, CL, DL, BL, AH, CH, DH, BH }
#[derive(TryFromPrimitive, Clone, Copy)] #[repr(usize)]
pub enum GpReg8w { AL, CL, DL, BL, SPL, BPL, SIL, DIL, R8B, R9B, R10B, R11B, R12B, R13B, R14B, R15B }

const GPREGS_COUNT: usize = GpReg64::END as usize;

#[repr(C)]
#[derive(Clone, Copy)]
union GpRegUnit {
    reg64: u64,
    reg32: u32,
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

    fn get64(&self, r: usize) -> u64 { unsafe { self.0[r].reg64 } }
    fn get32(&self, r: usize) -> u32 { unsafe { self.0[r].reg32 } }
    fn get16(&self, r: usize) -> u16 { unsafe { self.0[r].reg16 } }
    fn get8h(&self, r: usize) -> u8 { unsafe { self.0[r].reg8[1] } }
    fn get8l(&self, r: usize) -> u8 { unsafe { self.0[r].reg8[0] } }

    fn set64(&mut self, r: usize, v: u64) -> () { self.0[r].reg64 = v; }
    fn set32(&mut self, r: usize, v: u32) -> () { self.0[r].reg32 = v; }
    fn set16(&mut self, r: usize, v: u16) -> () { self.0[r].reg16 = v; }
    fn set8h(&mut self, r: usize, v: u8) -> () { unsafe { self.0[r].reg8[1] = v; } }
    fn set8l(&mut self, r: usize, v: u8) -> () { unsafe { self.0[r].reg8[0] = v; } }

    fn update64(&mut self, r: usize, v: i64) -> () { unsafe { self.0[r].reg64 = self.0[r].reg64.wrapping_add(v as u64); } }
    fn update32(&mut self, r: usize, v: i32) -> () { unsafe { self.0[r].reg32 = self.0[r].reg32.wrapping_add(v as u32); } }
    fn update16(&mut self, r: usize, v: i16) -> () { unsafe { self.0[r].reg16 = self.0[r].reg16.wrapping_add(v as u16); } }
    fn update8h(&mut self, r: usize, v: i8) -> () { unsafe { self.0[r].reg8[1] = self.0[r].reg8[1].wrapping_add(v as u8); } }
    fn update8l(&mut self, r: usize, v: i8) -> () { unsafe { self.0[r].reg8[0] = self.0[r].reg8[0].wrapping_add(v as u8); } }
}

pub trait RegAccess<T, U, V> {
    fn get(&self, r: T) -> U;
    fn set(&mut self, r: T, v: U) -> ();
    fn update(&mut self, r: T, v: V) -> ();
}

impl RegAccess<GpReg64, u64, i64> for GpRegisters {
    fn get(&self, r: GpReg64) -> u64 { self.get64(r as usize) }
    fn set(&mut self, r: GpReg64, v: u64) -> () { self.set64(r as usize, v); }
    fn update(&mut self, r: GpReg64, v: i64) -> () { self.update64(r as usize, v); }
}

impl RegAccess<GpReg32, u32, i32> for GpRegisters {
    fn get(&self, r: GpReg32) -> u32 { self.get32(r as usize) }
    fn set(&mut self, r: GpReg32, v: u32) -> () { self.set32(r as usize, v); }
    fn update(&mut self, r: GpReg32, v: i32) -> () { self.update32(r as usize, v); }
}

impl RegAccess<GpReg16, u16, i16> for GpRegisters {
    fn get(&self, r: GpReg16) -> u16 { self.get16(r as usize) }
    fn set(&mut self, r: GpReg16, v: u16) -> () { self.set16(r as usize, v); }
    fn update(&mut self, r: GpReg16, v: i16) -> () { self.update16(r as usize, v); }
}

impl RegAccess<GpReg8, u8, i8> for GpRegisters {
    fn get(&self, r: GpReg8) -> u8 { let r = r as usize; if r < 4 { self.get8l(r) } else { self.get8h(r%4) } }
    fn set(&mut self, r: GpReg8, v: u8) -> () { let r = r as usize; if r < 4 { self.set8l(r, v) } else { self.set8h(r%4, v) }; }
    fn update(&mut self, r: GpReg8, v: i8) -> () { let r = r as usize; if r < 4 { self.update8l(r, v) } else { self.update8h(r%4, v) }; }
}

impl RegAccess<GpReg8w, u8, i8> for GpRegisters {
    fn get(&self, r: GpReg8w) -> u8 { self.get8l(r as usize) }
    fn set(&mut self, r: GpReg8w, v: u8) -> () { self.set8l(r as usize, v); }
    fn update(&mut self, r: GpReg8w, v: i8) -> () { self.update8l(r as usize, v); }
}

#[cfg(test)]
#[test]
pub fn gpreg_test() {
    let mut reg = GpRegisters::new();

    reg.set(GpReg64::RAX, 0xdeadbeefcafebabe);
    reg.set(GpReg32::EAX, 0x11223344);
    reg.set(GpReg8::AH, 0x00);
    reg.update(GpReg64::RAX, -0x10);
    assert_eq!(reg.get(GpReg64::RAX), 0xdeadbeef11220034);

    reg.set(GpReg32::EDI, 0xc0bebeef);
    reg.set(GpReg8w::DIL, 0xff);
    assert_eq!(reg.get(GpReg64::RDI), 0xc0bebeff);
}

#[cfg(test)]
#[test]
#[should_panic]
fn gpreg_test_panic() {
    use std::convert::TryFrom;
    let reg = GpRegisters::new();
    reg.get(GpReg64::try_from(20).unwrap());
}