use crate::emulator::access::register::*;
use crate::emulator::instruction::exec;
use crate::emulator::instruction::opcode::*;

pub fn init_cmn_opcode(op: &mut super::OpcodeArr){
    macro_rules! setcmnop {
        ($n:expr, $fnc:ident, $flg:expr) => { op[$n & 0x1ff] = OpcodeType{func:$fnc, flag:$flg} }
    }

    setcmnop!(0x00, add_rm8_r8,    OpFlags::MODRM);
    setcmnop!(0x02, add_r8_rm8,    OpFlags::MODRM);
    setcmnop!(0x04, add_al_imm8,   OpFlags::IMM8);
    setcmnop!(0x08, or_rm8_r8,     OpFlags::MODRM);
    setcmnop!(0x0a, or_r8_rm8,     OpFlags::MODRM);
    setcmnop!(0x0c, or_al_imm8,    OpFlags::IMM8);
    setcmnop!(0x10, adc_rm8_r8,    OpFlags::MODRM);
    setcmnop!(0x12, adc_r8_rm8,    OpFlags::MODRM);
    setcmnop!(0x14, adc_al_imm8,   OpFlags::IMM8);
    setcmnop!(0x18, sbb_rm8_r8,    OpFlags::MODRM);
    setcmnop!(0x1a, sbb_r8_rm8,    OpFlags::MODRM);
    setcmnop!(0x1c, sbb_al_imm8,   OpFlags::IMM8);
    setcmnop!(0x20, and_rm8_r8,    OpFlags::MODRM);
    setcmnop!(0x22, and_r8_rm8,    OpFlags::MODRM);
    setcmnop!(0x24, and_al_imm8,   OpFlags::IMM8);
    setcmnop!(0x28, sub_rm8_r8,    OpFlags::MODRM);
    setcmnop!(0x2a, sub_r8_rm8,    OpFlags::MODRM);
    setcmnop!(0x2c, sub_al_imm8,   OpFlags::IMM8);
    setcmnop!(0x30, xor_rm8_r8,    OpFlags::MODRM);
    setcmnop!(0x32, xor_r8_rm8,    OpFlags::MODRM);
    setcmnop!(0x34, xor_al_imm8,   OpFlags::IMM8);
    setcmnop!(0x38, cmp_rm8_r8,    OpFlags::MODRM);
    setcmnop!(0x3a, cmp_r8_rm8,    OpFlags::MODRM);
    setcmnop!(0x3c, cmp_al_imm8,   OpFlags::IMM8);
    setcmnop!(0x70, jo_imm8,       OpFlags::IMM8);
    setcmnop!(0x71, jno_imm8,      OpFlags::IMM8);
    setcmnop!(0x72, jb_imm8,       OpFlags::IMM8);
    setcmnop!(0x73, jnb_imm8,      OpFlags::IMM8);
    setcmnop!(0x74, jz_imm8,       OpFlags::IMM8);
    setcmnop!(0x75, jnz_imm8,      OpFlags::IMM8);
    setcmnop!(0x76, jbe_imm8,      OpFlags::IMM8);
    setcmnop!(0x77, jnbe_imm8,     OpFlags::IMM8);
    setcmnop!(0x78, js_imm8,       OpFlags::IMM8);
    setcmnop!(0x79, jns_imm8,      OpFlags::IMM8);
    setcmnop!(0x7a, jp_imm8,       OpFlags::IMM8);
    setcmnop!(0x7b, jnp_imm8,      OpFlags::IMM8);
    setcmnop!(0x7c, jl_imm8,       OpFlags::IMM8);
    setcmnop!(0x7d, jnl_imm8,      OpFlags::IMM8);
    setcmnop!(0x7e, jle_imm8,      OpFlags::IMM8);
    setcmnop!(0x7f, jnle_imm8,     OpFlags::IMM8);
    setcmnop!(0x84, test_rm8_r8,   OpFlags::MODRM);
    setcmnop!(0x86, xchg_r8_rm8,   OpFlags::MODRM);
    setcmnop!(0x88, mov_rm8_r8,    OpFlags::MODRM);
    setcmnop!(0x8a, mov_r8_rm8,    OpFlags::MODRM);
    setcmnop!(0x8e, mov_sreg_rm16, OpFlags::MODRM);
    setcmnop!(0x90, nop,           OpFlags::NONE);
    setcmnop!(0xa0, mov_al_moffs8, OpFlags::MOFFS);
    setcmnop!(0xa2, mov_moffs8_al, OpFlags::MOFFS);
    setcmnop!(0xa8, test_al_imm8,  OpFlags::IMM8);
    for i in 0..8 {
        setcmnop!(0xb0+i, mov_opr8_imm8, OpFlags::IMM8);
    }
    setcmnop!(0xc6, mov_rm8_imm8,  OpFlags::MODRM | OpFlags::IMM8);
    setcmnop!(0xcc, int3,          OpFlags::NONE);
    setcmnop!(0xcd, int_imm8,      OpFlags::IMM8);
    setcmnop!(0xce, into,          OpFlags::NONE);
    setcmnop!(0xe4, in_al_imm8,    OpFlags::IMM8);
    setcmnop!(0xe6, out_imm8_al,   OpFlags::IMM8);
    setcmnop!(0xeb, jmp_imm8,      OpFlags::IMM8);
    setcmnop!(0xec, in_al_dx,      OpFlags::NONE);
    setcmnop!(0xee, out_dx_al,     OpFlags::NONE);
    setcmnop!(0xfa, cli,           OpFlags::NONE);
    setcmnop!(0xfb, sti,           OpFlags::NONE);
    setcmnop!(0xfc, cld,           OpFlags::NONE);
    setcmnop!(0xfd, std,           OpFlags::NONE);
    setcmnop!(0xf4, hlt,           OpFlags::NONE);

    setcmnop!(0x0f20, mov_r32_cr,  OpFlags::MODRM);
    setcmnop!(0x0f22, mov_cr_r32,  OpFlags::MODRM);
    setcmnop!(0x0f90, seto_rm8,    OpFlags::MODRM);
    setcmnop!(0x0f91, setno_rm8,   OpFlags::MODRM);
    setcmnop!(0x0f92, setb_rm8,    OpFlags::MODRM);
    setcmnop!(0x0f93, setnb_rm8,   OpFlags::MODRM);
    setcmnop!(0x0f94, setz_rm8,    OpFlags::MODRM);
    setcmnop!(0x0f95, setnz_rm8,   OpFlags::MODRM);
    setcmnop!(0x0f96, setbe_rm8,   OpFlags::MODRM);
    setcmnop!(0x0f97, setnbe_rm8,  OpFlags::MODRM);
    setcmnop!(0x0f98, sets_rm8,    OpFlags::MODRM);
    setcmnop!(0x0f99, setns_rm8,   OpFlags::MODRM);
    setcmnop!(0x0f9a, setp_rm8,    OpFlags::MODRM);
    setcmnop!(0x0f9b, setnp_rm8,   OpFlags::MODRM);
    setcmnop!(0x0f9c, setl_rm8,    OpFlags::MODRM);
    setcmnop!(0x0f9d, setnl_rm8,   OpFlags::MODRM);
    setcmnop!(0x0f9e, setle_rm8,   OpFlags::MODRM);
    setcmnop!(0x0f9f, setnle_rm8,  OpFlags::MODRM);

    setcmnop!(0x80, code_80,       OpFlags::MODRM | OpFlags::IMM8);
    //setcmnop!(0x82, code_82,       OpFlags::MODRM | OpFlags::IMM8);
    //setcmnop!(0xc0, code_c0,       OpFlags::MODRM | OpFlags::IMM8);
    //setcmnop!(0xf6, code_f6,       OpFlags::MODRM);
    setcmnop!(0x0f00, code_0f00,   OpFlags::MODRM);
}

add_dst_src!(8, rm8, r8);
add_dst_src!(8, r8, rm8);
add_dst_src!(8, al, imm8);

or_dst_src!(8, rm8, r8);
or_dst_src!(8, r8, rm8);
or_dst_src!(8, al, imm8);

adc_dst_src!(8, rm8, r8);
adc_dst_src!(8, r8, rm8);
adc_dst_src!(8, al, imm8);

sbb_dst_src!(8, rm8, r8);
sbb_dst_src!(8, r8, rm8);
sbb_dst_src!(8, al, imm8);

and_dst_src!(8, rm8, r8);
and_dst_src!(8, r8, rm8);
and_dst_src!(8, al, imm8);

sub_dst_src!(8, rm8, r8);
sub_dst_src!(8, r8, rm8);
sub_dst_src!(8, al, imm8);

xor_dst_src!(8, rm8, r8);
xor_dst_src!(8, r8, rm8);
xor_dst_src!(8, al, imm8);

cmp_dst_src!(8, rm8, r8);
cmp_dst_src!(8, r8, rm8);
cmp_dst_src!(8, al, imm8);

jcc_rel!(8, o, imm8);
jcc_rel!(8, b, imm8);
jcc_rel!(8, z, imm8);
jcc_rel!(8, be, imm8);
jcc_rel!(8, s, imm8);
jcc_rel!(8, p, imm8);
jcc_rel!(8, l, imm8);
jcc_rel!(8, le, imm8);

test_dst_src!(8, rm8, r8);

xchg_dst_src!(8, r8, rm8);

mov_dst_src!(8, rm8, r8);
mov_dst_src!(8, r8, rm8);

mov_dst_src!(16, sreg, rm16);

fn nop(_exec: &mut exec::Exec) -> Result<(), EmuException> { Ok(()) }

mov_dst_src!(8, al, moffs8);
mov_dst_src!(8, moffs8, al);

test_dst_src!(8, al, imm8);

mov_dst_src!(8, opr8, imm8);

mov_dst_src!(8, rm8, imm8);

fn int3(_exec: &mut exec::Exec) -> Result<(), EmuException> { Err(EmuException::Interrupt(3)) }
fn int_imm8(exec: &mut exec::Exec) -> Result<(), EmuException> { Err(EmuException::Interrupt(exec.get_imm8()?)) }
fn into(_exec: &mut exec::Exec) -> Result<(), EmuException> { Err(EmuException::Interrupt(4)) }

in_reg_port!(8, al, imm8);
out_port_reg!(8, imm8, al);

jmp_rel!(8, imm8);

in_reg_port!(8, al, dx);
out_port_reg!(8, dx, al);

fn cli(exec: &mut exec::Exec) -> Result<(), EmuException> { exec.ac.core.rflags.set_interrupt(false); Ok(()) }
fn sti(exec: &mut exec::Exec) -> Result<(), EmuException> { exec.ac.core.rflags.set_interrupt(true); Ok(()) }
fn cld(exec: &mut exec::Exec) -> Result<(), EmuException> { exec.ac.core.rflags.set_direction(false); Ok(()) }
fn std(exec: &mut exec::Exec) -> Result<(), EmuException> { exec.ac.core.rflags.set_direction(true); Ok(()) }

fn hlt(_exec: &mut exec::Exec) -> Result<(), EmuException> { Err(EmuException::Halt) }

fn mov_r32_cr(exec: &mut exec::Exec) -> Result<(), EmuException> { exec.cr_to_r32() }
fn mov_cr_r32(exec: &mut exec::Exec) -> Result<(), EmuException> { exec.cr_from_r32() }

setcc_dst!(8, o, rm8);
setcc_dst!(8, b, rm8);
setcc_dst!(8, z, rm8);
setcc_dst!(8, be, rm8);
setcc_dst!(8, s, rm8);
setcc_dst!(8, p, rm8);
setcc_dst!(8, l, rm8);
setcc_dst!(8, le, rm8);

fn code_80(exec: &mut exec::Exec) -> Result<(), EmuException> {
    match exec.idata.modrm.reg as u8 {
        0 => add_rm8_imm8(exec)?,
        1 => or_rm8_imm8(exec)?,
        2 => adc_rm8_imm8(exec)?,
        3 => sbb_rm8_imm8(exec)?,
        4 => and_rm8_imm8(exec)?,
        5 => sub_rm8_imm8(exec)?,
        6 => xor_rm8_imm8(exec)?,
        7 => cmp_rm8_imm8(exec)?,
        _ => { return Err(EmuException::UnexpectedError); },
    }
    Ok(())
}

add_dst_src!(8, rm8, imm8);
or_dst_src!(8, rm8, imm8);
adc_dst_src!(8, rm8, imm8);
sbb_dst_src!(8, rm8, imm8);
and_dst_src!(8, rm8, imm8);
sub_dst_src!(8, rm8, imm8);
xor_dst_src!(8, rm8, imm8);
cmp_dst_src!(8, rm8, imm8);

fn code_0f00(exec: &mut exec::Exec) -> Result<(), EmuException> {
    match exec.idata.modrm.reg as u16 {
        2 => lldt_rm16(exec)?,
        3 => ltr_rm16(exec)?,
        _ => { return Err(EmuException::NotImplementedOpcode); },
    }
    Ok(())
}

fn lldt_rm16(exec: &mut exec::Exec) -> Result<(), EmuException> {
    let sel = exec.get_rm16()?;
    exec.set_ldtr(sel)
}

fn ltr_rm16(exec: &mut exec::Exec) -> Result<(), EmuException> {
    let sel = exec.get_rm16()?;
    exec.set_tr(sel)
}
