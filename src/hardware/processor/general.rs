use num_enum::FromPrimitive;

#[derive(FromPrimitive)] #[repr(usize)]
pub enum GpReg64 { #[num_enum(default)] RAX, RCX, RDX, RBX, RSP, RBP, RSI, RDI }
#[derive(FromPrimitive)] #[repr(usize)]
pub enum GpReg32 { #[num_enum(default)] EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI }
#[derive(FromPrimitive)] #[repr(usize)]
pub enum GpReg16 { #[num_enum(default)] AX, CX, DX, BX, SP, BP, SI, DI }
#[derive(FromPrimitive)] #[repr(usize)]
pub enum GpReg8h { #[num_enum(default)] AH, CH, DH, BH }
#[derive(FromPrimitive)] #[repr(usize)]
pub enum GpReg8l { #[num_enum(default)] AL, CL, DL, BL, SPL, BPL, SIL, DIL }

const GPREGS_COUNT: usize = 8;

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
        GpRegUnit{reg64: 0}
    }
}

#[derive(Clone, Copy)]
pub struct GpRegisters {
    regs: [GpRegUnit; GPREGS_COUNT],
}

impl GpRegisters {
    pub fn new() -> Self {
        GpRegisters {regs: [GpRegUnit::new(); GPREGS_COUNT]}
    }

    fn get64(&self, r: usize) -> u64 { unsafe { self.regs[r].reg64 } }
    fn get32(&self, r: usize) -> u32 { unsafe { self.regs[r].reg32 } }
    fn get16(&self, r: usize) -> u16 { unsafe { self.regs[r].reg16 } }
    fn get8h(&self, r: usize) -> u8 { unsafe { self.regs[r].reg8[1] } }
    fn get8l(&self, r: usize) -> u8 { unsafe { self.regs[r].reg8[0] } }

    fn set64(&mut self, r: usize, v: u64) -> () { self.regs[r].reg64 = v; }
    fn set32(&mut self, r: usize, v: u32) -> () { self.regs[r].reg32 = v; }
    fn set16(&mut self, r: usize, v: u16) -> () { self.regs[r].reg16 = v; }
    fn set8h(&mut self, r: usize, v: u8) -> () { unsafe { self.regs[r].reg8[1] = v; } }
    fn set8l(&mut self, r: usize, v: u8) -> () { unsafe { self.regs[r].reg8[0] = v; } }

    fn update64(&mut self, r: usize, v: i64) -> () { unsafe { self.regs[r].reg64 = (self.regs[r].reg64 as i64 + v) as u64; } }
    fn update32(&mut self, r: usize, v: i32) -> () { unsafe { self.regs[r].reg32 = (self.regs[r].reg32 as i32 + v) as u32; } }
    fn update16(&mut self, r: usize, v: i16) -> () { unsafe { self.regs[r].reg16 = (self.regs[r].reg16 as i16 + v) as u16; } }
    fn update8h(&mut self, r: usize, v: i8) -> () { unsafe { self.regs[r].reg8[1] = (self.regs[r].reg8[1] as i8 + v) as u8; } }
    fn update8l(&mut self, r: usize, v: i8) -> () { unsafe { self.regs[r].reg8[0] = (self.regs[r].reg8[0] as i8 + v) as u8; } }
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

impl RegAccess<GpReg8h, u8, i8> for GpRegisters {
    fn get(&self, r: GpReg8h) -> u8 { self.get8h(r as usize) }
    fn set(&mut self, r: GpReg8h, v: u8) -> () { self.set8h(r as usize, v); }
    fn update(&mut self, r: GpReg8h, v: i8) -> () { self.update8h(r as usize, v); }
}

impl RegAccess<GpReg8l, u8, i8> for GpRegisters {
    fn get(&self, r: GpReg8l) -> u8 { self.get8l(r as usize) }
    fn set(&mut self, r: GpReg8l, v: u8) -> () { self.set8l(r as usize, v); }
    fn update(&mut self, r: GpReg8l, v: i8) -> () { self.update8l(r as usize, v); }
}