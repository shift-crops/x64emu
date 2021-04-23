use crate::emulator;
use crate::hardware::processor::general::*;
use crate::hardware::processor::segment::*;
use libc::c_void;
use gdbstub::arch;
use gdbstub::arch::x86::reg::id::X86_64CoreRegId;
use gdbstub::target;
use gdbstub::target::ext::base::singlethread::{ResumeAction, SingleThreadOps, StopReason};
use gdbstub::target::ext::breakpoints::WatchKind;
use gdbstub::target::{Target, TargetResult};
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
    type Arch = arch::x86::X86_64_SSE;
    type Error = &'static str;

    fn base_ops(&mut self) -> target::ext::base::BaseOps<Self::Arch, Self::Error> {
        target::ext::base::BaseOps::SingleThread(self)
    }
}

impl SingleThreadOps for emulator::Emulator {
    fn resume(&mut self, action: ResumeAction, check_gdb_interrupt: &mut dyn FnMut() -> bool,) -> Result<StopReason<u64>, Self::Error> {
        match action {
            ResumeAction::Step => match self.step() {
                Some(e) => e,
                None => return Ok(StopReason::DoneStep),
            },
            ResumeAction::Continue => {
                let mut cycles = 0;
                loop {
                    if let Some(event) = self.step() {
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

    fn read_registers(&mut self, regs: &mut arch::x86::reg::X86_64CoreRegs) -> TargetResult<(), Self> {
        let core = &self.ac.core;
        regs.regs[0] = core.gpregs.get(GpReg64::RAX);
        regs.regs[1] = core.gpregs.get(GpReg64::RBX);
        regs.regs[2] = core.gpregs.get(GpReg64::RCX);
        regs.regs[3] = core.gpregs.get(GpReg64::RDX);
        regs.regs[4] = core.gpregs.get(GpReg64::RSI);
        regs.regs[5] = core.gpregs.get(GpReg64::RDI);
        regs.regs[6] = core.gpregs.get(GpReg64::RBP);
        regs.regs[7] = core.gpregs.get(GpReg64::RSP);
        for i in 8..16 {
            regs.regs[i] = core.gpregs.get(GpReg64::from(i as usize));
        }
        regs.rip = core.rip.get();
        regs.eflags = core.rflags.to_u64() as u32;
        regs.segments[0] = core.sgregs.selector(SgReg::CS).to_u16() as u32;
        regs.segments[1] = core.sgregs.selector(SgReg::SS).to_u16() as u32;
        regs.segments[2] = core.sgregs.selector(SgReg::DS).to_u16() as u32;
        regs.segments[3] = core.sgregs.selector(SgReg::ES).to_u16() as u32;
        regs.segments[4] = core.sgregs.selector(SgReg::FS).to_u16() as u32;
        regs.segments[5] = core.sgregs.selector(SgReg::GS).to_u16() as u32;

        Ok(())
    }

    fn write_registers(&mut self, regs: &arch::x86::reg::X86_64CoreRegs) -> TargetResult<(), Self> {
        let core = &mut self.ac.core;
        core.gpregs.set(GpReg64::RAX, regs.regs[0]);
        core.gpregs.set(GpReg64::RBX, regs.regs[1]);
        core.gpregs.set(GpReg64::RCX, regs.regs[2]);
        core.gpregs.set(GpReg64::RDX, regs.regs[3]);
        core.gpregs.set(GpReg64::RSI, regs.regs[4]);
        core.gpregs.set(GpReg64::RDI, regs.regs[5]);
        core.gpregs.set(GpReg64::RBP, regs.regs[6]);
        core.gpregs.set(GpReg64::RSP, regs.regs[7]);
        for i in 8..16 {
            core.gpregs.set(GpReg64::from(i as usize), regs.regs[i]);
        }
        core.rip.set(regs.rip);
        core.rflags.from_u64(regs.eflags as u64);

        core.sgregs.selector_mut(SgReg::CS).from_u16(regs.segments[0] as u16);
        core.sgregs.selector_mut(SgReg::SS).from_u16(regs.segments[1] as u16);
        core.sgregs.selector_mut(SgReg::DS).from_u16(regs.segments[2] as u16);
        core.sgregs.selector_mut(SgReg::ES).from_u16(regs.segments[3] as u16);
        core.sgregs.selector_mut(SgReg::FS).from_u16(regs.segments[4] as u16);
        core.sgregs.selector_mut(SgReg::GS).from_u16(regs.segments[5] as u16);

        Ok(())
    }

    fn read_register( &mut self, reg_id: arch::x86::reg::id::X86_64CoreRegId, dst: &mut [u8],) -> TargetResult<(), Self> {
        /*
        let core = &self.ac.core;
        let reg_val = match reg_id {
            X86_64CoreRegId::Rip => Some(core.rip.get()),
            X86_64CoreRegId::Gpr(i) => Some(core.gpregs.get(GpReg64::from(i as usize))),
            _ => None,
        };

        if let Some(w) = reg_val {
            dst.copy_from_slice(&w.to_le_bytes());
            Ok(())
        } else {
            Err(().into())
        }
        */
        Ok(())
    }

    fn write_register( &mut self, reg_id: arch::x86::reg::id::X86_64CoreRegId, val: &[u8],) -> TargetResult<(), Self> {
        Ok(())
    }

    fn read_addrs(&mut self, start_addr: u64, data: &mut [u8]) -> TargetResult<(), Self> {
        self.ac.mem.read_data(data.as_mut_ptr() as *mut c_void, start_addr as usize, data.len()).expect("read_data error");
        Ok(())
    }

    fn write_addrs(&mut self, start_addr: u64, data: &[u8]) -> TargetResult<(), Self> {
        self.ac.mem.write_data(start_addr as usize, data.as_ptr() as *const c_void, data.len()).expect("write_data error");
        Ok(())
    }
}