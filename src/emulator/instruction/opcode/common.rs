use crate::emulator::instruction;
use crate::emulator::instruction::opcode::*;
use crate::emulator::instruction::exec::regmem::*;
use crate::emulator::instruction::exec::flag::*;
use crate::hardware::processor::general::*;

macro_rules! get_al { ($arg:expr) => { $arg.ac.core.gpregs().get(GpReg8::AL) } }
macro_rules! set_al { ($arg:expr, $val:expr) => { $arg.ac.core.gpregs_mut().set(GpReg8::AL, $val) } }

pub fn init_cmn_opcode(op: &mut super::OpcodeArr){
    macro_rules! setop {
        ($n:expr, $fnc:ident, $flg:expr) => { op[$n & 0x1ff] = OpcodeType{func:$fnc, flag:$flg} }
    }

    setop!(0x00, add_rm8_r8,    OpFlags::MODRM);
    setop!(0x02, add_r8_rm8,    OpFlags::MODRM);
    setop!(0x04, add_al_imm8,   OpFlags::IMM8);
    setop!(0x08, or_rm8_r8,     OpFlags::MODRM);
    setop!(0x0a, or_r8_rm8,     OpFlags::MODRM);
    setop!(0x0c, or_al_imm8,    OpFlags::IMM8);
    setop!(0x20, and_rm8_r8,    OpFlags::MODRM);
    setop!(0x22, and_r8_rm8,    OpFlags::MODRM);
    setop!(0x24, and_al_imm8,   OpFlags::IMM8);
    setop!(0x28, sub_rm8_r8,    OpFlags::MODRM);
    setop!(0x2a, sub_r8_rm8,    OpFlags::MODRM);
    setop!(0x2c, sub_al_imm8,   OpFlags::IMM8);
    setop!(0x30, xor_rm8_r8,    OpFlags::MODRM);
    setop!(0x32, xor_r8_rm8,    OpFlags::MODRM);
    setop!(0x34, xor_al_imm8,   OpFlags::IMM8);
    setop!(0x38, cmp_rm8_r8,    OpFlags::MODRM);
    setop!(0x3a, cmp_r8_rm8,    OpFlags::MODRM);
    setop!(0x3c, cmp_al_imm8,   OpFlags::IMM8);
    /*
    setop!(0x70, jo_rel8,       OpFlags::IMM8);
    setop!(0x71, jno_rel8,      OpFlags::IMM8);
    setop!(0x72, jb_rel8,       OpFlags::IMM8);
    setop!(0x73, jnb_rel8,      OpFlags::IMM8);
    setop!(0x74, jz_rel8,       OpFlags::IMM8);
    setop!(0x75, jnz_rel8,      OpFlags::IMM8);
    setop!(0x76, jbe_rel8,      OpFlags::IMM8);
    setop!(0x77, ja_rel8,       OpFlags::IMM8);
    setop!(0x78, js_rel8,       OpFlags::IMM8);
    setop!(0x79, jns_rel8,      OpFlags::IMM8);
    setop!(0x7a, jp_rel8,       OpFlags::IMM8);
    setop!(0x7b, jnp_rel8,      OpFlags::IMM8);
    setop!(0x7c, jl_rel8,       OpFlags::IMM8);
    setop!(0x7d, jnl_rel8,      OpFlags::IMM8);
    setop!(0x7e, jle_rel8,      OpFlags::IMM8);
    setop!(0x7f, jnle_rel8,     OpFlags::IMM8);
    setop!(0x84, test_rm8_r8,   OpFlags::MODRM);
    setop!(0x86, xchg_r8_rm8,   OpFlags::MODRM);
    */
    setop!(0x88, mov_rm8_r8,    OpFlags::MODRM);
    setop!(0x8a, mov_r8_rm8,    OpFlags::MODRM);
    //setop!(0x8e, mov_sreg_rm16, OpFlags::MODRM);
    setop!(0x90, nop,           OpFlags::NONE);
    setop!(0xa0, mov_al_moffs8, OpFlags::MOFFS);
    setop!(0xa2, mov_moffs8_al, OpFlags::MOFFS);
    setop!(0xa8, test_al_imm8,  OpFlags::IMM8);
    for i in 0..8 {
        setop!(0xb0+i, mov_r8_imm8, OpFlags::IMM8);
    }
    /*
    setop!(0xc6, mov_rm8_imm8,  OpFlags::MODRM | OpFlags::IMM8);
    setop!(0xcb, retf,          OpFlags::NONE);
    setop!(0xcc, int3,          OpFlags::NONE);
    setop!(0xcd, int_imm8,      OpFlags::IMM8);
    setop!(0xcf, iret,          OpFlags::NONE);
    setop!(0xe4, in_al_imm8,    OpFlags::IMM8);
    setop!(0xe6, out_imm8_al,   OpFlags::IMM8);
    setop!(0xeb, jmp,           OpFlags::IMM8);
    setop!(0xec, in_al_dx,      OpFlags::NONE);
    setop!(0xee, out_dx_al,     OpFlags::NONE);
    setop!(0xfa, cli,           OpFlags::NONE);
    setop!(0xfb, sti,           OpFlags::NONE);
    setop!(0xfc, cld,           OpFlags::NONE);
    setop!(0xfd, std,           OpFlags::NONE);
    setop!(0xf4, hlt,           OpFlags::NONE);

    setop!(0x0f20, mov_r32_crn, OpFlags::MODRM);
    setop!(0x0f22, mov_crn_r32, OpFlags::MODRM);
    setop!(0x0f90, seto_rm8,    OpFlags::MODRM);
    setop!(0x0f91, setno_rm8,   OpFlags::MODRM);
    setop!(0x0f92, setb_rm8,    OpFlags::MODRM);
    setop!(0x0f93, setnb_rm8,   OpFlags::MODRM);
    setop!(0x0f94, setz_rm8,    OpFlags::MODRM);
    setop!(0x0f95, setnz_rm8,   OpFlags::MODRM);
    setop!(0x0f96, setbe_rm8,   OpFlags::MODRM);
    setop!(0x0f97, seta_rm8,    OpFlags::MODRM);
    setop!(0x0f98, sets_rm8,    OpFlags::MODRM);
    setop!(0x0f99, setns_rm8,   OpFlags::MODRM);
    setop!(0x0f9a, setp_rm8,    OpFlags::MODRM);
    setop!(0x0f9b, setnp_rm8,   OpFlags::MODRM);
    setop!(0x0f9c, setl_rm8,    OpFlags::MODRM);
    setop!(0x0f9d, setnl_rm8,   OpFlags::MODRM);
    setop!(0x0f9e, setle_rm8,   OpFlags::MODRM);
    setop!(0x0f9f, setnle_rm8,  OpFlags::MODRM);

    setop!(0x80, code_80,       OpFlags::MODRM | OpFlags::IMM8);
    setop!(0x82, code_82,       OpFlags::MODRM | OpFlags::IMM8);
    setop!(0xc0, code_c0,       OpFlags::MODRM | OpFlags::IMM8);
    setop!(0xf6, code_f6,       OpFlags::MODRM);
    */
}

fn add_rm8_r8 (arg: &mut instruction::InstrArg){
    let rm8: u8 = get_rm8(arg);
    let r8:  u8 = get_r8(arg);
    debug!("add_rm8_r8: rm8 = 0x{:02x}, r8 = 0x{:02x}", rm8, r8);
    set_rm8(arg, rm8.wrapping_add(r8));
    update_rflags_add(arg, rm8, r8);
}

fn add_r8_rm8 (arg: &mut instruction::InstrArg){
    let r8:  u8 = get_r8(arg);
    let rm8: u8 = get_rm8(arg);
    debug!("add_r8_rm8: r8 = 0x{:02x}, rm8 = 0x{:02x}", r8, rm8);
    set_r8(arg, r8.wrapping_add(rm8));
    update_rflags_add(arg, r8, rm8);
}

fn add_al_imm8 (arg: &mut instruction::InstrArg){
    let al:   u8 = get_al!(arg);
    let imm8: u8 = arg.idata.imm as u8;
    debug!("add_al_imm8: al = 0x{:02x}, imm8 = 0x{:02x}", al, imm8);
    set_al!(arg, al.wrapping_add(imm8));
    update_rflags_add(arg, al, imm8);
}
 
fn or_rm8_r8 (arg: &mut instruction::InstrArg){
    let rm8: u8 = get_rm8(arg);
    let r8:  u8 = get_r8(arg);
    debug!("or_rm8_r8: rm8 = 0x{:02x}, r8 = 0x{:02x}", rm8, r8);
    set_rm8(arg, rm8 | r8);
    update_rflags_or(arg, rm8, r8);
}

fn or_r8_rm8 (arg: &mut instruction::InstrArg){
    let r8:  u8 = get_r8(arg);
    let rm8: u8 = get_rm8(arg);
    debug!("or_r8_rm8: r8 = 0x{:02x}, rm8 = 0x{:02x}", r8, rm8);
    set_r8(arg, r8 | rm8);
    update_rflags_or(arg, r8, rm8);
}

fn or_al_imm8 (arg: &mut instruction::InstrArg){
    let al:   u8 = get_al!(arg);
    let imm8: u8 = arg.idata.imm as u8;
    debug!("or_al_imm8: al = 0x{:02x}, imm8 = 0x{:02x}", al, imm8);
    set_al!(arg, al | imm8);
    update_rflags_or(arg, al, imm8);
}
 
fn and_rm8_r8 (arg: &mut instruction::InstrArg){
    let rm8: u8 = get_rm8(arg);
    let r8:  u8 = get_r8(arg);
    debug!("and_rm8_r8: rm8 = 0x{:02x}, r8 = 0x{:02x}", rm8, r8);
    set_rm8(arg, rm8 & r8);
    update_rflags_and(arg, rm8, r8);
}

fn and_r8_rm8 (arg: &mut instruction::InstrArg){
    let r8:  u8 = get_r8(arg);
    let rm8: u8 = get_rm8(arg);
    debug!("and_r8_rm8: r8 = 0x{:02x}, rm8 = 0x{:02x}", r8, rm8);
    set_r8(arg, r8 & rm8);
    update_rflags_and(arg, r8, rm8);
}

fn and_al_imm8 (arg: &mut instruction::InstrArg){
    let al:   u8 = get_al!(arg);
    let imm8: u8 = arg.idata.imm as u8;
    debug!("and_al_imm8: al = 0x{:02x}, imm8 = 0x{:02x}", al, imm8);
    set_al!(arg, al & imm8);
    update_rflags_and(arg, al, imm8);
}

fn sub_rm8_r8 (arg: &mut instruction::InstrArg){
    let rm8: u8 = get_rm8(arg);
    let r8:  u8 = get_r8(arg);
    debug!("sub_rm8_r8: rm8 = 0x{:02x}, r8 = 0x{:02x}", rm8, r8);
    set_rm8(arg, rm8.wrapping_sub(r8));
    update_rflags_sub(arg, rm8, r8);
}

fn sub_r8_rm8 (arg: &mut instruction::InstrArg){
    let r8:  u8 = get_r8(arg);
    let rm8: u8 = get_rm8(arg);
    debug!("sub_r8_rm8: r8 = 0x{:02x}, rm8 = 0x{:02x}", r8, rm8);
    set_r8(arg, r8.wrapping_sub(rm8));
    update_rflags_sub(arg, r8, rm8);
}

fn sub_al_imm8 (arg: &mut instruction::InstrArg){
    let al:   u8 = get_al!(arg);
    let imm8: u8 = arg.idata.imm as u8;
    debug!("sub_al_imm8: al = 0x{:02x}, imm8 = 0x{:02x}", al, imm8);
    set_al!(arg, al.wrapping_sub(imm8));
    update_rflags_sub(arg, al, imm8);
}
 
fn xor_rm8_r8 (arg: &mut instruction::InstrArg){
    let rm8: u8 = get_rm8(arg);
    let r8:  u8 = get_r8(arg);
    debug!("xor_rm8_r8: rm8 = 0x{:02x}, r8 = 0x{:02x}", rm8, r8);
    set_rm8(arg, rm8 ^ r8);
    update_rflags_xor(arg, rm8, r8);
}

fn xor_r8_rm8 (arg: &mut instruction::InstrArg){
    let r8:  u8 = get_r8(arg);
    let rm8: u8 = get_rm8(arg);
    debug!("xor_r8_rm8: r8 = 0x{:02x}, rm8 = 0x{:02x}", r8, rm8);
    set_r8(arg, r8 ^ rm8);
    update_rflags_xor(arg, r8, rm8);
}

fn xor_al_imm8 (arg: &mut instruction::InstrArg){
    let al:   u8 = get_al!(arg);
    let imm8: u8 = arg.idata.imm as u8;
    debug!("xor_al_imm8: al = 0x{:02x}, imm8 = 0x{:02x}", al, imm8);
    set_al!(arg, al ^ imm8);
    update_rflags_xor(arg, al, imm8);
}

fn cmp_rm8_r8 (arg: &mut instruction::InstrArg){
    let rm8: u8 = get_rm8(arg);
    let r8:  u8 = get_r8(arg);
    debug!("cmp_rm8_r8: rm8 = 0x{:02x}, r8 = 0x{:02x}", rm8, r8);
    update_rflags_sub(arg, rm8, r8);
}

fn cmp_r8_rm8 (arg: &mut instruction::InstrArg){
    let r8:  u8 = get_r8(arg);
    let rm8: u8 = get_rm8(arg);
    debug!("cmp_r8_rm8: r8 = 0x{:02x}, rm8 = 0x{:02x}", r8, rm8);
    update_rflags_sub(arg, r8, rm8);
}

fn cmp_al_imm8 (arg: &mut instruction::InstrArg){
    let al:   u8 = get_al!(arg);
    let imm8: u8 = arg.idata.imm as u8;
    debug!("cmp_al_imm8: al = 0x{:02x}, imm8 = 0x{:02x}", al, imm8);
    update_rflags_sub(arg, al, imm8);
}
 
fn mov_rm8_r8 (arg: &mut instruction::InstrArg){
    let r8: u8 = get_r8(arg);
    debug!("mov_rm8_r8: r8 = 0x{:02x}", r8);
    set_rm8(arg, r8);
}

fn mov_r8_rm8 (arg: &mut instruction::InstrArg){
    let rm8: u8 = get_rm8(arg);
    debug!("mov_r8_rm8: rm8 = 0x{:02x}", rm8);
    set_r8(arg, rm8);
}

fn nop (_arg: &mut instruction::InstrArg){}

fn mov_al_moffs8 (arg: &mut instruction::InstrArg){
    let moffs8: u8 = arg.idata.moffs as u8;
    debug!("mov_al_moffs8: imm8 = 0x{:02x}", moffs8);
    set_al!(arg, moffs8);
}

fn mov_moffs8_al (arg: &mut instruction::InstrArg){
    let al: u8 = get_al!(arg);
    debug!("mov_moffs8_al: al = 0x{:02x}", al);
    set_moffs8(arg, al);
}

fn test_al_imm8 (arg: &mut instruction::InstrArg){
    let al:   u8 = get_al!(arg);
    let imm8: u8 = arg.idata.imm as u8;
    debug!("test_al_imm8: al = 0x{:02x}, imm8 = 0x{:02x}", al, imm8);
    update_rflags_and(arg, al, imm8);
}

fn mov_r8_imm8 (arg: &mut instruction::InstrArg){
    let imm8: u8 = arg.idata.imm as u8;
    debug!("mov_r8_imm8: rm8 = 0x{:02x}", imm8);
    arg.ac.core.gpregs_mut().set(GpReg8::from((arg.idata.opcd&0x7) as usize), imm8);
}
