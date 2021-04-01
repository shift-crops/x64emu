pub enum GpReg64 { RAX, RCX, RDX, RBX, RSP, RBP, RSI, RDI }
enum GpReg32 { EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI }
enum GpReg16 { AX, CX, DX, BX, SP, BP, SI, DI }
enum GpReg8h { AH, CH, DH, BH }
enum GpReg8l { AL, CL, DL, BL, SPL, BPL, SIL, DIL }

pub const GPREGS_COUNT: usize = 8;

#[repr(C)]
#[derive(Clone, Copy)]
pub union GpRegister {
    pub r: u64,
    e: u32,
    x: u16,
    hl: [u8; 2],
}

impl GpRegister {
    pub fn new() -> GpRegister {
        GpRegister{r: 0}
    }
}