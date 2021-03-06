use crate::emulator;
use crate::hardware::processor::general::*;
use crate::hardware::processor::segment::*;
use libc::c_void;
use gdbstub::arch;
use gdbstub::target;
use gdbstub::arch::x86::reg::id::X86CoreRegId;
use gdbstub::target::ext::base::singlethread::{ResumeAction, SingleThreadOps, StopReason};
use gdbstub::target::{Target, TargetResult, TargetError};
use std::net::{TcpListener, TcpStream};

pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>; 
pub fn wait_for_tcp(port: u16) -> DynResult<TcpStream> {
    let sockaddr = format!("127.0.0.1:{}", port);
    eprintln!("Waiting for a GDB connection on {:?}...", sockaddr);

    let sock = TcpListener::bind(sockaddr)?;
    let (stream, addr) = sock.accept()?;
    eprintln!("Debugger connected from {}", addr);

    Ok(stream)
}

impl Target for emulator::Emulator {
    type Arch = arch::x86::X86_SSE;
    type Error = &'static str;

    fn base_ops(&mut self) -> target::ext::base::BaseOps<Self::Arch, Self::Error> {
        target::ext::base::BaseOps::SingleThread(self)
    }

    fn sw_breakpoint(&mut self) -> Option<target::ext::breakpoints::SwBreakpointOps<Self>> {
        Some(self)
    }
}

impl SingleThreadOps for emulator::Emulator {
    fn resume(&mut self, action: ResumeAction, check_gdb_interrupt: &mut dyn FnMut() -> bool,) -> Result<StopReason<u32>, Self::Error> {
        match action {
            ResumeAction::Step => match self.step(true) {
                Some(e) => e,
                None => return Ok(StopReason::DoneStep),
            },
            ResumeAction::Continue => {
                let mut cycles = 0;
                loop {
                    if let Some(event) = self.step(true) {
                        break event;
                    };

                    // check for GDB interrupt every 1024 instructions
                    cycles += 1;
                    if cycles % 1024 == 0 && check_gdb_interrupt() {
                        return Ok(StopReason::GdbInterrupt);
                    }
                }
            }
        };
        Ok(StopReason::DoneStep)
    }

    fn read_registers(&mut self, regs: &mut arch::x86::reg::X86CoreRegs) -> TargetResult<(), Self> {
        let core = &self.ac.core;
        regs.eax = core.gpregs.get32(GpReg32::EAX);
        regs.ecx = core.gpregs.get32(GpReg32::ECX);
        regs.edx = core.gpregs.get32(GpReg32::EDX);
        regs.ebx = core.gpregs.get32(GpReg32::EBX);
        regs.esi = core.gpregs.get32(GpReg32::ESI);
        regs.edi = core.gpregs.get32(GpReg32::EDI);
        regs.ebp = core.gpregs.get32(GpReg32::EBP);
        regs.esp = core.gpregs.get32(GpReg32::ESP);

        regs.eip = core.ip.get_eip();
        regs.eflags = core.rflags.to_u64() as u32;
        regs.segments[0] = core.sgregs.get(SgReg::CS).selector.to_u16() as u32;
        regs.segments[1] = core.sgregs.get(SgReg::SS).selector.to_u16() as u32;
        regs.segments[2] = core.sgregs.get(SgReg::DS).selector.to_u16() as u32;
        regs.segments[3] = core.sgregs.get(SgReg::ES).selector.to_u16() as u32;
        regs.segments[4] = core.sgregs.get(SgReg::FS).selector.to_u16() as u32;
        regs.segments[5] = core.sgregs.get(SgReg::GS).selector.to_u16() as u32;

        Ok(())
    }

    fn write_registers(&mut self, regs: &arch::x86::reg::X86CoreRegs) -> TargetResult<(), Self> {
        let core = &mut self.ac.core;
        core.gpregs.set32(GpReg32::EAX, regs.eax);
        core.gpregs.set32(GpReg32::ECX, regs.ecx);
        core.gpregs.set32(GpReg32::EDX, regs.edx);
        core.gpregs.set32(GpReg32::EBX, regs.ebx);
        core.gpregs.set32(GpReg32::ESI, regs.esi);
        core.gpregs.set32(GpReg32::EDI, regs.edi);
        core.gpregs.set32(GpReg32::EBP, regs.ebp);
        core.gpregs.set32(GpReg32::ESP, regs.esp);

        core.ip.set_eip(regs.eip);
        core.rflags.from_u64(regs.eflags as u64);

        core.sgregs.get_mut(SgReg::CS).selector.from_u16(regs.segments[0] as u16);
        core.sgregs.get_mut(SgReg::SS).selector.from_u16(regs.segments[1] as u16);
        core.sgregs.get_mut(SgReg::DS).selector.from_u16(regs.segments[2] as u16);
        core.sgregs.get_mut(SgReg::ES).selector.from_u16(regs.segments[3] as u16);
        core.sgregs.get_mut(SgReg::FS).selector.from_u16(regs.segments[4] as u16);
        core.sgregs.get_mut(SgReg::GS).selector.from_u16(regs.segments[5] as u16);

        Ok(())
    }

    fn read_register( &mut self, reg_id: arch::x86::reg::id::X86CoreRegId, dst: &mut [u8],) -> TargetResult<(), Self> {
        let core = &self.ac.core;
        let reg_val = match reg_id {
            X86CoreRegId::Eip => Some(core.ip.get_eip()),
            X86CoreRegId::Eax => Some(core.gpregs.get32(GpReg32::EAX)),
            X86CoreRegId::Ebx => Some(core.gpregs.get32(GpReg32::EBX)),
            X86CoreRegId::Ecx => Some(core.gpregs.get32(GpReg32::ECX)),
            X86CoreRegId::Edx => Some(core.gpregs.get32(GpReg32::EDX)),
            X86CoreRegId::Edi => Some(core.gpregs.get32(GpReg32::EDI)),
            X86CoreRegId::Esi => Some(core.gpregs.get32(GpReg32::ESI)),
            X86CoreRegId::Ebp => Some(core.gpregs.get32(GpReg32::EBP)),
            X86CoreRegId::Esp => Some(core.gpregs.get32(GpReg32::ESP)),
            _ => None,
        };

        if let Some(w) = reg_val {
            dst.copy_from_slice(&w.to_le_bytes());
            Ok(())
        } else {
            Err(().into())
        }
    }

    fn write_register( &mut self, reg_id: arch::x86::reg::id::X86CoreRegId, val: &[u8],) -> TargetResult<(), Self> {
        let mut dst = [0u8; 4];
        dst.clone_from_slice(val);
        let w = u32::from_le_bytes(dst);

        let core = &mut self.ac.core;
        match reg_id {
            X86CoreRegId::Eip => core.ip.set_eip(w),
            X86CoreRegId::Eax => core.gpregs.set32(GpReg32::EAX, w),
            X86CoreRegId::Ebx => core.gpregs.set32(GpReg32::EBX, w),
            X86CoreRegId::Ecx => core.gpregs.set32(GpReg32::ECX, w),
            X86CoreRegId::Edx => core.gpregs.set32(GpReg32::EDX, w),
            X86CoreRegId::Edi => core.gpregs.set32(GpReg32::EDI, w),
            X86CoreRegId::Esi => core.gpregs.set32(GpReg32::ESI, w),
            X86CoreRegId::Ebp => core.gpregs.set32(GpReg32::EBP, w),
            X86CoreRegId::Esp => core.gpregs.set32(GpReg32::ESP, w),
            _ => {},
        };
        Ok(())
    }

    fn read_addrs(&mut self, start_addr: u32, data: &mut [u8]) -> TargetResult<(), Self> {
        if let Ok(paddr) = self.ac.addr_v2p(SgReg::DS, start_addr as u64) {
            if let Ok(_) = self.ac.mem.read().unwrap().read_data(data.as_mut_ptr() as *mut c_void, paddr as usize, data.len()) {
                return Ok(());
            }
        }
        Err(TargetError::NonFatal)
    }

    fn write_addrs(&mut self, start_addr: u32, data: &[u8]) -> TargetResult<(), Self> {
        if let Ok(paddr) = self.ac.addr_v2p(SgReg::DS, start_addr as u64) {
            if let Ok(_) = self.ac.mem.write().unwrap().write_data(paddr as usize, data.as_ptr() as *const c_void, data.len()) {
                return Ok(());
            }
        }
        Err(TargetError::NonFatal)
    }
}

impl target::ext::breakpoints::SwBreakpoint for emulator::Emulator {
    fn add_sw_breakpoint(&mut self, addr: u32) -> TargetResult<bool, Self> {
        self.breakpoints.push(addr);
        Ok(true)
    }

    fn remove_sw_breakpoint(&mut self, addr: u32) -> TargetResult<bool, Self> {
        match self.breakpoints.iter().position(|x| *x == addr) {
            None => return Ok(false),
            Some(pos) => self.breakpoints.remove(pos),
        };

        Ok(true)
    }
}