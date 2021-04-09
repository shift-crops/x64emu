use crate::emulator::instruction::exec;
use crate::emulator::instruction::opcode::*;
use crate::hardware::processor::general::*;

// macro_rules! get_al { ($exec:expr) => { $exec.ac.core.gpregs().get(GpReg8::AL) } }
// macro_rules! set_al { ($exec:expr, $val:expr) => { $exec.ac.core.gpregs_mut().set(GpReg8::AL, $val) } }

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
    setop!(0x10, adc_rm8_r8,    OpFlags::MODRM);
    setop!(0x12, adc_r8_rm8,    OpFlags::MODRM);
    setop!(0x14, adc_al_imm8,   OpFlags::IMM8);
    setop!(0x18, sbb_rm8_r8,    OpFlags::MODRM);
    setop!(0x1a, sbb_r8_rm8,    OpFlags::MODRM);
    setop!(0x1c, sbb_al_imm8,   OpFlags::IMM8);
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
    setop!(0x70, jo_imm8,       OpFlags::IMM8);
    setop!(0x71, jno_imm8,      OpFlags::IMM8);
    setop!(0x72, jb_imm8,       OpFlags::IMM8);
    setop!(0x73, jnb_imm8,      OpFlags::IMM8);
    setop!(0x74, jz_imm8,       OpFlags::IMM8);
    setop!(0x75, jnz_imm8,      OpFlags::IMM8);
    setop!(0x76, jbe_imm8,      OpFlags::IMM8);
    setop!(0x77, jnbe_imm8,     OpFlags::IMM8);
    setop!(0x78, js_imm8,       OpFlags::IMM8);
    setop!(0x79, jns_imm8,      OpFlags::IMM8);
    setop!(0x7a, jp_imm8,       OpFlags::IMM8);
    setop!(0x7b, jnp_imm8,      OpFlags::IMM8);
    setop!(0x7c, jl_imm8,       OpFlags::IMM8);
    setop!(0x7d, jnl_imm8,      OpFlags::IMM8);
    setop!(0x7e, jle_imm8,      OpFlags::IMM8);
    setop!(0x7f, jnle_imm8,     OpFlags::IMM8);
    setop!(0x84, test_rm8_r8,   OpFlags::MODRM);
    setop!(0x86, xchg_r8_rm8,   OpFlags::MODRM);
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
    setop!(0xc6, mov_rm8_imm8,  OpFlags::MODRM | OpFlags::IMM8);
    /*
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
    */
    setop!(0x0f90, seto_rm8,    OpFlags::MODRM);
    setop!(0x0f91, setno_rm8,   OpFlags::MODRM);
    setop!(0x0f92, setb_rm8,    OpFlags::MODRM);
    setop!(0x0f93, setnb_rm8,   OpFlags::MODRM);
    setop!(0x0f94, setz_rm8,    OpFlags::MODRM);
    setop!(0x0f95, setnz_rm8,   OpFlags::MODRM);
    setop!(0x0f96, setbe_rm8,   OpFlags::MODRM);
    setop!(0x0f97, setnbe_rm8,  OpFlags::MODRM);
    setop!(0x0f98, sets_rm8,    OpFlags::MODRM);
    setop!(0x0f99, setns_rm8,   OpFlags::MODRM);
    setop!(0x0f9a, setp_rm8,    OpFlags::MODRM);
    setop!(0x0f9b, setnp_rm8,   OpFlags::MODRM);
    setop!(0x0f9c, setl_rm8,    OpFlags::MODRM);
    setop!(0x0f9d, setnl_rm8,   OpFlags::MODRM);
    setop!(0x0f9e, setle_rm8,   OpFlags::MODRM);
    setop!(0x0f9f, setnle_rm8,  OpFlags::MODRM);

    setop!(0x80, code_80,       OpFlags::MODRM | OpFlags::IMM8);
    //setop!(0xc0, code_c0,       OpFlags::MODRM | OpFlags::IMM8);
    //setop!(0xf6, code_f6,       OpFlags::MODRM);
}

macro_rules! add_dst_src {
    ( $dst:ident, $src:ident ) => {
        paste::item! {
            fn [<add_ $dst _ $src>](exec: &mut exec::Exec) {
                let dst: u8 = exec.[<get_ $dst>]();
                let src: u8 = exec.[<get_ $src>]();

                debug!("add: {:02x}, {:02x}", dst, src);
                exec.[<set_ $dst>](dst.wrapping_add(src));
                exec.update_rflags_add(dst, src);
            }
        }
    };
}

macro_rules! or_dst_src {
    ( $dst:ident, $src:ident ) => {
        paste::item! {
            fn [<or_ $dst _ $src>](exec: &mut exec::Exec) {
                let dst: u8 = exec.[<get_ $dst>]();
                let src: u8 = exec.[<get_ $src>]();

                debug!("or: {:02x}, {:02x}", dst, src);
                exec.[<set_ $dst>](dst | src);
                exec.update_rflags_or(dst, src);
            }
        }
    };
}

macro_rules! adc_dst_src {
    ( $dst:ident, $src:ident ) => {
        paste::item! {
            fn [<adc_ $dst _ $src>](exec: &mut exec::Exec) {
                let dst: u8 = exec.[<get_ $dst>]();
                let src: u8 = exec.[<get_ $src>]();
                let cf:  u8 = exec.ac.core.rflags.is_carry() as u8;

                debug!("adc: {:02x}, {:02x}", dst, src);
                exec.[<set_ $dst>](dst.wrapping_add(src).wrapping_add(cf));
                exec.update_rflags_adc(dst, src, cf);
            }
        }
    };
}

macro_rules! sbb_dst_src {
    ( $dst:ident, $src:ident ) => {
        paste::item! {
            fn [<sbb_ $dst _ $src>](exec: &mut exec::Exec) {
                let dst: u8 = exec.[<get_ $dst>]();
                let src: u8 = exec.[<get_ $src>]();
                let cf:  u8 = exec.ac.core.rflags.is_carry() as u8;

                debug!("sbb: {:02x}, {:02x}", dst, src);
                exec.[<set_ $dst>](dst.wrapping_sub(src).wrapping_sub(cf));
                exec.update_rflags_sbb(dst, src, cf);
            }
        }
    };
}

macro_rules! and_dst_src {
    ( $dst:ident, $src:ident ) => {
        paste::item! {
            fn [<and_ $dst _ $src>](exec: &mut exec::Exec) {
                let dst: u8 = exec.[<get_ $dst>]();
                let src: u8 = exec.[<get_ $src>]();

                debug!("and: {:02x}, {:02x}", dst, src);
                exec.[<set_ $dst>](dst & src);
                exec.update_rflags_and(dst, src);
            }
        }
    };
}

macro_rules! sub_dst_src {
    ( $dst:ident, $src:ident ) => {
        paste::item! {
            fn [<sub_ $dst _ $src>](exec: &mut exec::Exec) {
                let dst: u8 = exec.[<get_ $dst>]();
                let src: u8 = exec.[<get_ $src>]();

                debug!("sub: {:02x}, {:02x}", dst, src);
                exec.[<set_ $dst>](dst.wrapping_sub(src));
                exec.update_rflags_sub(dst, src);
            }
        }
    };
}

macro_rules! xor_dst_src {
    ( $dst:ident, $src:ident ) => {
        paste::item! {
            fn [<xor_ $dst _ $src>](exec: &mut exec::Exec) {
                let dst: u8 = exec.[<get_ $dst>]();
                let src: u8 = exec.[<get_ $src>]();

                debug!("xor: {:02x}, {:02x}", dst, src);
                exec.[<set_ $dst>](dst ^ src);
                exec.update_rflags_xor(dst, src);
            }
        }
    };
}

macro_rules! cmp_dst_src {
    ( $dst:ident, $src:ident ) => {
        paste::item! {
            fn [<cmp_ $dst _ $src>](exec: &mut exec::Exec) {
                let dst: u8 = exec.[<get_ $dst>]();
                let src: u8 = exec.[<get_ $src>]();
                debug!("cmp: {:02x}, {:02x}", dst, src);
                exec.update_rflags_sub(dst, src);
            }
        }
    };
}

macro_rules! jcc_imm8 {
    ( $cc:ident ) => {
        paste::item! {
            fn [<j $cc _imm8>](exec: &mut exec::Exec) {
                if(exec.[<check_rflags_ $cc>]()){
                    let imm8: i8 = exec.get_imm8() as i8;
                    debug!("jmp: {}", imm8);
                    exec.update_rip(imm8 as i64);
                }
            }

            fn [<jn $cc _imm8>](exec: &mut exec::Exec) {
                if(!exec.[<check_rflags_ $cc>]()){
                    let imm8: i8 = exec.get_imm8() as i8;
                    debug!("jmp: {}", imm8);
                    exec.update_rip(imm8 as i64);
                }
            }
        }
    };
}

macro_rules! test_dst_src {
    ( $dst:ident, $src:ident ) => {
        paste::item! {
            fn [<test_ $dst _ $src>](exec: &mut exec::Exec) {
                let dst: u8 = exec.[<get_ $dst>]();
                let src: u8 = exec.[<get_ $src>]();
                debug!("test: {:02x}, {:02x}", dst, src);
                exec.update_rflags_and(dst, src);
            }
        }
    };
}

macro_rules! mov_dst_src {
    ( $dst:ident, $src:ident ) => {
        paste::item! {
            fn [<mov_ $dst _ $src>](exec: &mut exec::Exec) {
                let src: u8 = exec.[<get_ $src>]();
                debug!("mov: {:02x}", src);
                exec.[<set_ $dst>](src);
            }
        }
    };
}

macro_rules! setcc_rm8 {
    ( $cc:ident ) => {
        paste::item! {
            fn [<set $cc _rm8>](exec: &mut exec::Exec) {
                let flag: bool = exec.[<check_rflags_ $cc>]();
                exec.set_rm8(flag as u8);
            }

            fn [<setn $cc _rm8>](exec: &mut exec::Exec) {
                let flag: bool = exec.[<check_rflags_ $cc>]();
                exec.set_rm8(!flag as u8);
            }
        }
    };
}

add_dst_src!(rm8, r8);
add_dst_src!(r8, rm8);
add_dst_src!(al, imm8);

or_dst_src!(rm8, r8);
or_dst_src!(r8, rm8);
or_dst_src!(al, imm8);

adc_dst_src!(rm8, r8);
adc_dst_src!(r8, rm8);
adc_dst_src!(al, imm8);

sbb_dst_src!(rm8, r8);
sbb_dst_src!(r8, rm8);
sbb_dst_src!(al, imm8);

and_dst_src!(rm8, r8);
and_dst_src!(r8, rm8);
and_dst_src!(al, imm8);

sub_dst_src!(rm8, r8);
sub_dst_src!(r8, rm8);
sub_dst_src!(al, imm8);

xor_dst_src!(rm8, r8);
xor_dst_src!(r8, rm8);
xor_dst_src!(al, imm8);

cmp_dst_src!(rm8, r8);
cmp_dst_src!(r8, rm8);
cmp_dst_src!(al, imm8);

jcc_imm8!(o);
jcc_imm8!(b);
jcc_imm8!(z);
jcc_imm8!(be);
jcc_imm8!(s);
jcc_imm8!(p);
jcc_imm8!(l);
jcc_imm8!(le);

test_dst_src!(rm8, r8);

fn xchg_r8_rm8(exec: &mut exec::Exec) {
    let r8:  u8 = exec.get_r8();
    let rm8: u8 = exec.get_rm8();
    debug!("xchg_r8_rm8: r8 = 0x{:02x}, rm8 = 0x{:02x}", r8, rm8);
    exec.set_r8(rm8);
    exec.set_rm8(r8);
}

mov_dst_src!(rm8, r8);
mov_dst_src!(r8, rm8);

fn nop (_exec: &mut exec::Exec){}

mov_dst_src!(al, moffs8);
mov_dst_src!(moffs8, al);

test_dst_src!(al, imm8);

fn mov_r8_imm8(exec: &mut exec::Exec) {
    let imm8: u8 = exec.get_imm8();
    debug!("mov_r8_imm8: imm8 = 0x{:02x}", imm8);
    exec.ac.core.gpregs.set(GpReg8::from((exec.idata.opcd&0x7) as usize), imm8);
}

mov_dst_src!(rm8, imm8);

setcc_rm8!(o);
setcc_rm8!(b);
setcc_rm8!(z);
setcc_rm8!(be);
setcc_rm8!(s);
setcc_rm8!(p);
setcc_rm8!(l);
setcc_rm8!(le);

fn code_80(exec: &mut exec::Exec) {
    match exec.idata.modrm.reg as u8 {
        0 => add_rm8_imm8(exec),
        1 => or_rm8_imm8(exec),
        2 => adc_rm8_imm8(exec),
        3 => sbb_rm8_imm8(exec),
        4 => and_rm8_imm8(exec),
        5 => sub_rm8_imm8(exec),
        6 => xor_rm8_imm8(exec),
        7 => cmp_rm8_imm8(exec),
        _ => { panic!("ha??"); },
    }
}

add_dst_src!(rm8, imm8);
or_dst_src!(rm8, imm8);
adc_dst_src!(rm8, imm8);
sbb_dst_src!(rm8, imm8);
and_dst_src!(rm8, imm8);
sub_dst_src!(rm8, imm8);
xor_dst_src!(rm8, imm8);
cmp_dst_src!(rm8, imm8);