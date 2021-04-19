use crate::emulator::access::register::*;
use crate::emulator::instruction::exec;
use crate::emulator::instruction::opcode::*;

// macro_rules! get_al { ($exec:expr) => { $exec.ac.core.gpregs().get(GpReg8::AL) } }
// macro_rules! set_al { ($exec:expr, $val:expr) => { $exec.ac.core.gpregs_mut().set(GpReg8::AL, $val) } }

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
    setcmnop!(0x8e, mov_sgr_rm16, OpFlags::MODRM);
    setcmnop!(0x90, nop,           OpFlags::NONE);
    setcmnop!(0xa0, mov_al_moffs8, OpFlags::MOFFS);
    setcmnop!(0xa2, mov_moffs8_al, OpFlags::MOFFS);
    setcmnop!(0xa8, test_al_imm8,  OpFlags::IMM8);
    for i in 0..8 {
        setcmnop!(0xb0+i, mov_opr8_imm8, OpFlags::IMM8);
    }
    setcmnop!(0xc6, mov_rm8_imm8,  OpFlags::MODRM | OpFlags::IMM8);
    /*
    setcmnop!(0xcc, int3,          OpFlags::NONE);
    setcmnop!(0xcd, int_imm8,      OpFlags::IMM8);
    setcmnop!(0xe4, in_al_imm8,    OpFlags::IMM8);
    setcmnop!(0xe6, out_imm8_al,   OpFlags::IMM8);
    */
    setcmnop!(0xeb, jmp_imm8,      OpFlags::IMM8);
    /*
    setcmnop!(0xec, in_al_dx,      OpFlags::NONE);
    setcmnop!(0xee, out_dx_al,     OpFlags::NONE);
    setcmnop!(0xfa, cli,           OpFlags::NONE);
    setcmnop!(0xfb, sti,           OpFlags::NONE);
    setcmnop!(0xfc, cld,           OpFlags::NONE);
    setcmnop!(0xfd, std,           OpFlags::NONE);
    setcmnop!(0xf4, hlt,           OpFlags::NONE);

    setcmnop!(0x0f20, mov_r32_cr,  OpFlags::MODRM);
    setcmnop!(0x0f22, mov_cr_r32,  OpFlags::MODRM);
    */
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
}

add_dst_src!(u8, rm8, r8);
add_dst_src!(u8, r8, rm8);
add_dst_src!(u8, al, imm8);

or_dst_src!(u8, rm8, r8);
or_dst_src!(u8, r8, rm8);
or_dst_src!(u8, al, imm8);

adc_dst_src!(u8, rm8, r8);
adc_dst_src!(u8, r8, rm8);
adc_dst_src!(u8, al, imm8);

sbb_dst_src!(u8, rm8, r8);
sbb_dst_src!(u8, r8, rm8);
sbb_dst_src!(u8, al, imm8);

and_dst_src!(u8, rm8, r8);
and_dst_src!(u8, r8, rm8);
and_dst_src!(u8, al, imm8);

sub_dst_src!(u8, rm8, r8);
sub_dst_src!(u8, r8, rm8);
sub_dst_src!(u8, al, imm8);

xor_dst_src!(u8, rm8, r8);
xor_dst_src!(u8, r8, rm8);
xor_dst_src!(u8, al, imm8);

cmp_dst_src!(u8, rm8, r8);
cmp_dst_src!(u8, r8, rm8);
cmp_dst_src!(u8, al, imm8);

jcc_rel!(i8, o, imm8);
jcc_rel!(i8, b, imm8);
jcc_rel!(i8, z, imm8);
jcc_rel!(i8, be, imm8);
jcc_rel!(i8, s, imm8);
jcc_rel!(i8, p, imm8);
jcc_rel!(i8, l, imm8);
jcc_rel!(i8, le, imm8);

test_dst_src!(u8, rm8, r8);

xchg_dst_src!(u8, r8, rm8);

mov_dst_src!(u8, rm8, r8);
mov_dst_src!(u8, r8, rm8);

mov_dst_src!(u16, sgr, rm16);

fn nop (_exec: &mut exec::Exec) -> Result<(), EmuException> { Ok(()) }

mov_dst_src!(u8, al, moffs8);
mov_dst_src!(u8, moffs8, al);

test_dst_src!(u8, al, imm8);

mov_dst_src!(u8, opr8, imm8);

mov_dst_src!(u8, rm8, imm8);

jmp_rel!(i8, imm8);

setcc_dst!(u8, o, rm8);
setcc_dst!(u8, b, rm8);
setcc_dst!(u8, z, rm8);
setcc_dst!(u8, be, rm8);
setcc_dst!(u8, s, rm8);
setcc_dst!(u8, p, rm8);
setcc_dst!(u8, l, rm8);
setcc_dst!(u8, le, rm8);

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

add_dst_src!(u8, rm8, imm8);
or_dst_src!(u8, rm8, imm8);
adc_dst_src!(u8, rm8, imm8);
sbb_dst_src!(u8, rm8, imm8);
and_dst_src!(u8, rm8, imm8);
sub_dst_src!(u8, rm8, imm8);
xor_dst_src!(u8, rm8, imm8);
cmp_dst_src!(u8, rm8, imm8);