use crate::emulator::*;
use crate::emulator::access::register::*;
use crate::emulator::instruction::opcode::*;

pub struct Opcode64 (pub super::OpcodeArr);
impl Opcode64 {
    pub fn new(op: super::OpcodeArr) -> Self {
        Self (op)
    }
}

impl super::OpcodeTrait for Opcode64 {
    fn init_opcode(&mut self) -> () {
        macro_rules! setop {
            ($n:expr, $fnc:ident, $flg:expr) => { self.0[$n & 0x1ff] = OpcodeType{func:Self::$fnc, flag:$flg} }
        }

        // 0x00 : add_rm8_r8
        setop!(0x01, add_rm64_r64,  OpFlags::MODRM);
        // 0x02 : add_r8_rm8
        setop!(0x03, add_r64_rm64,  OpFlags::MODRM);
        // 0x04 : add_al_imm8
        setop!(0x05, add_rax_imm64, OpFlags::IMM64);
        // 0x08 : or_rm8_r8
        setop!(0x09, or_rm64_r64,   OpFlags::MODRM);
        // 0x0a : or_r8_rm8
        setop!(0x0b, or_r64_rm64,   OpFlags::MODRM);
        // 0x0c : or_al_imm8
        setop!(0x0d, or_rax_imm64,  OpFlags::IMM64);

        // 0x10 : adc_rm8_r8
        setop!(0x11, adc_rm64_r64,  OpFlags::MODRM);
        // 0x12 : adc_r8_rm8
        setop!(0x13, adc_r64_rm64,  OpFlags::MODRM);
        // 0x14 : adc_al_imm8
        setop!(0x15, adc_rax_imm64, OpFlags::IMM64);

        // 0x18 : sbb_rm8_r8
        setop!(0x19, sbb_rm64_r64,  OpFlags::MODRM);
        // 0x1a : sbb_r8_rm8
        setop!(0x1b, sbb_r64_rm64,  OpFlags::MODRM);
        // 0x1c : sbb_al_imm8
        setop!(0x1d, sbb_rax_imm64, OpFlags::IMM64);

        // 0x20 : and_rm8_r8
        setop!(0x21, and_rm64_r64,  OpFlags::MODRM);
        // 0x22 : and_r8_rm8
        setop!(0x23, and_r64_rm64,  OpFlags::MODRM);
        // 0x24 : and_al_imm8
        setop!(0x25, and_rax_imm64, OpFlags::IMM64);
        // 0x28 : sub_rm8_r8
        setop!(0x29, sub_rm64_r64,  OpFlags::MODRM);
        // 0x2a : sub_r8_rm8
        setop!(0x2b, sub_r64_rm64,  OpFlags::MODRM);
        // 0x2c : sub_al_imm8
        setop!(0x2d, sub_rax_imm64, OpFlags::IMM64);

        // 0x30 : xor_rm8_r8
        setop!(0x31, xor_rm64_r64,  OpFlags::MODRM);
        // 0x32 : xor_r8_rm8
        setop!(0x33, xor_r64_rm64,  OpFlags::MODRM);
        // 0x34 : xor_al_imm8
        setop!(0x35, xor_rax_imm64, OpFlags::IMM64);
        // 0x38 : cmp_rm8_r8
        setop!(0x39, cmp_rm64_r64,  OpFlags::MODRM);
        // 0x3a : cmp_r8_rm8
        setop!(0x3b, cmp_r64_rm64,  OpFlags::MODRM);
        // 0x3c : cmp_al_imm8
        setop!(0x3d, cmp_rax_imm64, OpFlags::IMM64);

        for i in 0..8 {
            setop!(0x40+i, inc_opr64,   OpFlags::NONE);
            setop!(0x48+i, dec_opr64,   OpFlags::NONE);
            setop!(0x50+i, push_opr64,  OpFlags::NONE);
            setop!(0x58+i, pop_opr64,   OpFlags::NONE);
        }

        // 0x60 : invalid
        // 0x61 : invalid

        setop!(0x68, push_imm64,            OpFlags::IMM64);
        setop!(0x69, imul_r64_rm64_imm64,   OpFlags::MODRM | OpFlags::IMM64);
        setop!(0x6a, push_imm8,             OpFlags::IMM8);
        setop!(0x6b, imul_r64_rm64_imm8,    OpFlags::MODRM | OpFlags::IMM8);

        // 0x70-0x7f : jcc

        // 0x84 : test_rm8_r8
        setop!(0x85, test_rm64_r64,     OpFlags::MODRM);
        // 0x86 : xchg_r8_rm8
        setop!(0x87, xchg_r64_rm64,     OpFlags::MODRM);
        // 0x88 : mov_rm8_r8
        setop!(0x89, mov_rm64_r64,      OpFlags::MODRM);
        // 0x8a : mov_r8_rm8
        setop!(0x8b, mov_r64_rm64,      OpFlags::MODRM);
        setop!(0x8c, mov_rm64_sreg,     OpFlags::MODRM);
        setop!(0x8d, lea_r64_m64,       OpFlags::MODRM);
        // 0x8e : mov_sreg_rm64

        // 0x90 : nop

        for i in 0..8 {
            setop!(0x90+i, xchg_rax_opr64,     OpFlags::NONE);
        }
        setop!(0x98, cdqe,              OpFlags::NONE);
        setop!(0x99, cqo,               OpFlags::NONE);
        // 0x9a : invalid

        setop!(0x9c, pushf,             OpFlags::NONE);
        setop!(0x9d, popf,              OpFlags::NONE);

        // 0xa0 : mov_al_moffs8
        setop!(0xa1, mov_rax_moffs64,   OpFlags::MOFFS);
        // 0xa2 : mov_moffs8_al
        setop!(0xa3, mov_moffs64_rax,   OpFlags::MOFFS);
        setop!(0xa4, movs_m8,           OpFlags::NONE);
        setop!(0xa5, movs_m64,          OpFlags::NONE);
        setop!(0xa6, cmps_m8,           OpFlags::NONE);
        setop!(0xa7, cmps_m64,          OpFlags::NONE);
        // 0xa8 : test_al_imm8
        setop!(0xa9, test_rax_imm64,    OpFlags::IMM64);
        setop!(0xaa, stos_m8,           OpFlags::NONE);
        setop!(0xab, stos_m64,          OpFlags::NONE);
        setop!(0xac, lods_m8,           OpFlags::NONE);
        setop!(0xad, lods_m64,          OpFlags::NONE);
        setop!(0xae, scas_m8,           OpFlags::NONE);
        setop!(0xaf, scas_m64,          OpFlags::NONE);

        // 0xb0-0xb7 : mov_r8_imm
        for i in 0..8 {
            setop!(0xb8+i, mov_opr64_imm64,   OpFlags::IMM64);
        }

        setop!(0xc3, ret,               OpFlags::NONE);

        setop!(0xc7, mov_rm64_imm64,    OpFlags::MODRM | OpFlags::IMM64);

        setop!(0xc9, leave,             OpFlags::NONE);

        setop!(0xcb, retf,              OpFlags::NONE);
        // 0xcc : int3
        // 0xcd : int_imm8

        setop!(0xcf, iret,              OpFlags::NONE);

        // 0xe4 : in_al_imm8
        setop!(0xe5, in_eax_imm8,       OpFlags::IMM8);
        // 0xe6 : out_imm8_al
        setop!(0xe7, out_imm8_eax,      OpFlags::IMM8);
        setop!(0xe8, call_imm64,        OpFlags::IMM64);
        setop!(0xe9, jmp_imm64,         OpFlags::IMM64);
        // 0xea : invalid
        // 0xeb : jmp_imm8
        // 0xec : in_al_dx
        setop!(0xed, in_eax_dx,         OpFlags::NONE);
        // 0xee : out_dx_al
        setop!(0xef, out_dx_eax,        OpFlags::NONE);

        setop!(0x0f80, jo_imm64,        OpFlags::IMM64);
        setop!(0x0f81, jno_imm64,       OpFlags::IMM64);
        setop!(0x0f82, jb_imm64,        OpFlags::IMM64);
        setop!(0x0f83, jnb_imm64,       OpFlags::IMM64);
        setop!(0x0f84, jz_imm64,        OpFlags::IMM64);
        setop!(0x0f85, jnz_imm64,       OpFlags::IMM64);
        setop!(0x0f86, jbe_imm64,       OpFlags::IMM64);
        setop!(0x0f87, jnbe_imm64,      OpFlags::IMM64);
        setop!(0x0f88, js_imm64,        OpFlags::IMM64);
        setop!(0x0f89, jns_imm64,       OpFlags::IMM64);
        setop!(0x0f8a, jp_imm64,        OpFlags::IMM64);
        setop!(0x0f8b, jnp_imm64,       OpFlags::IMM64);
        setop!(0x0f8c, jl_imm64,        OpFlags::IMM64);
        setop!(0x0f8d, jnl_imm64,       OpFlags::IMM64);
        setop!(0x0f8e, jle_imm64,       OpFlags::IMM64);
        setop!(0x0f8f, jnle_imm64,      OpFlags::IMM64);

        setop!(0x0faf, imul_r64_rm64,   OpFlags::MODRM);

        setop!(0x0fb6, movzx_r64_rm8,   OpFlags::MODRM);
        setop!(0x0fb7, movzx_r64_rm64,  OpFlags::MODRM);

        setop!(0x0fbe, movsx_r64_rm8,   OpFlags::MODRM);
        setop!(0x0fbf, movsx_r64_rm64,  OpFlags::MODRM);

        // 0x80 : code_80
        setop!(0x81, code_81, OpFlags::MODRM | OpFlags::IMM64);
        // 0x82 : invalid
        setop!(0x83, code_83, OpFlags::MODRM | OpFlags::IMM8);
        // 0xc0 : code_c0
        setop!(0xc1, code_c1, OpFlags::MODRM | OpFlags::IMM8);
        // 0xd2 : code_d2
        setop!(0xd3, code_d3, OpFlags::MODRM);
        // 0xf6 : code_f6
        setop!(0xf7, code_f7, OpFlags::MODRM | OpFlags::IMM64);
        // 0xfe : code_fe
        setop!(0xff, code_ff, OpFlags::MODRM);
        // 0x0f00 : code_0f00
        setop!(0x0f01, code_0f01, OpFlags::MODRM);
    }

    fn exec(&self, exec: &mut exec::Exec) -> Result<(), EmuException> {
        (self.0[exec.idata.opcode as usize].func)(exec)
    }
    fn flag(&self, opcode: u16) -> OpFlags { self.0[opcode as usize].flag }
}

impl Opcode64 {
    add_dst_src!(64, rm64, r64);
    add_dst_src!(64, r64, rm64);
    add_dst_src!(64, rax, imm64);

    or_dst_src!(64, rm64, r64);
    or_dst_src!(64, r64, rm64);
    or_dst_src!(64, rax, imm64);

    adc_dst_src!(64, rm64, r64);
    adc_dst_src!(64, r64, rm64);
    adc_dst_src!(64, rax, imm64);

    sbb_dst_src!(64, rm64, r64);
    sbb_dst_src!(64, r64, rm64);
    sbb_dst_src!(64, rax, imm64);

    and_dst_src!(64, rm64, r64);
    and_dst_src!(64, r64, rm64);
    and_dst_src!(64, rax, imm64);

    sub_dst_src!(64, rm64, r64);
    sub_dst_src!(64, r64, rm64);
    sub_dst_src!(64, rax, imm64);

    xor_dst_src!(64, rm64, r64);
    xor_dst_src!(64, r64, rm64);
    xor_dst_src!(64, rax, imm64);

    cmp_dst_src!(64, rm64, r64);
    cmp_dst_src!(64, r64, rm64);
    cmp_dst_src!(64, rax, imm64);

    inc_dst!(opr64);
    dec_dst!(opr64);
    push_src!(64, opr64);
    pop_dst!(64, opr64);

    push_src!(64, imm8);
    imul_dst_src1_src2!(64, r64, rm64, imm64);
    push_src!(64, imm64);
    imul_dst_src1_src2!(64, r64, rm64, imm8);

    test_dst_src!(64, rm64, r64);
    xchg_dst_src!(64, r64, rm64);
    mov_dst_src!(64, rm64, r64);
    mov_dst_src!(64, r64, rm64);
    mov_dst_src!(64, rm64, sreg);
    lea_dst_src!(64, r64, m64);

    xchg_dst_src!(64, rax, opr64);

    fn cdqe(exec: &mut exec::Exec) -> Result<(), EmuException> {
        let eax = exec.ac.get_gpreg(GpReg32::EAX)? as i32;
        exec.ac.set_gpreg(GpReg64::RAX, eax as u64)
    }

    fn cqo(exec: &mut exec::Exec) -> Result<(), EmuException> {
        let rax = exec.ac.get_gpreg(GpReg64::RAX)? as i64;
        exec.ac.set_gpreg(GpReg64::RDX, if rax < 0 { u64::MAX } else { 0 })
    }

    callf_abs!(64, ptr16, imm64);

    pushf!(64);
    popf!(64);

    mov_dst_src!(64, rax, moffs64);
    mov_dst_src!(64, moffs64, rax);
    movs_dst_src!(64, 8);
    movs_dst_src!(64, 64);
    cmps_src_dst!(64, 8);
    cmps_src_dst!(64, 64);

    test_dst_src!(64, rax, imm64);
    stos_dst_src!(64, 8);
    stos_dst_src!(64, 64);
    lods_dst_src!(64, 8);
    lods_dst_src!(64, 64);
    scas_src_dst!(64, 8);
    scas_src_dst!(64, 64);

    mov_dst_src!(64, opr64, imm64);

    ret!(64);

    mov_dst_src!(64, rm64, imm64);

    fn leave(exec: &mut exec::Exec) -> Result<(), EmuException> {
        let rbp = exec.ac.get_gpreg(GpReg64::RBP)?;
        exec.ac.set_gpreg(GpReg64::RSP, rbp)?;
        let new_rbp = exec.ac.pop_u64()?;
        debug!("leave: rsp <- 0x{:016x}, rbp <- 0x{:016x}", rbp, new_rbp);
        exec.ac.set_gpreg(GpReg64::RBP, new_rbp)
    }

    retf!(64);

    iret!(64);

    in_reg_port!(32, eax, imm8);
    out_port_reg!(32, imm8, eax);
    call_rel!(64, imm64);
    jmp_rel!(64, imm64);
    jmpf_abs!(64, ptr16, imm64);
    in_reg_port!(32, eax, dx);
    out_port_reg!(32, dx, eax);

    jcc_rel!(64, o, imm64);
    jcc_rel!(64, b, imm64);
    jcc_rel!(64, z, imm64);
    jcc_rel!(64, be, imm64);
    jcc_rel!(64, s, imm64);
    jcc_rel!(64, p, imm64);
    jcc_rel!(64, l, imm64);
    jcc_rel!(64, le, imm64);

    imul_dst_src!(64, r64, rm64);

    movzx_dst_src!(64, r64, 8, rm8);
    movzx_dst_src!(64, r64, 64, rm64);
    movsx_dst_src!(64, r64, 8, rm8);
    movsx_dst_src!(64, r64, 64, rm64);

    fn code_81(exec: &mut exec::Exec) -> Result<(), EmuException> {
        match exec.idata.modrm.reg as u8 {
            0 => Opcode64::add_rm64_imm64(exec)?,
            1 => Opcode64::or_rm64_imm64(exec)?,
            2 => Opcode64::adc_rm64_imm64(exec)?,
            3 => Opcode64::sbb_rm64_imm64(exec)?,
            4 => Opcode64::and_rm64_imm64(exec)?,
            5 => Opcode64::sub_rm64_imm64(exec)?,
            6 => Opcode64::xor_rm64_imm64(exec)?,
            7 => Opcode64::cmp_rm64_imm64(exec)?,
            _ => { return Err(EmuException::UnexpectedError); },
        }
        Ok(())
    }

    add_dst_src!(64, rm64, imm64);
    or_dst_src!(64, rm64, imm64);
    adc_dst_src!(64, rm64, imm64);
    sbb_dst_src!(64, rm64, imm64);
    and_dst_src!(64, rm64, imm64);
    sub_dst_src!(64, rm64, imm64);
    xor_dst_src!(64, rm64, imm64);
    cmp_dst_src!(64, rm64, imm64);

    fn code_82(exec: &mut exec::Exec) -> Result<(), EmuException> {
        super::common::code_82(exec)
    }

    fn code_83(exec: &mut exec::Exec) -> Result<(), EmuException> {
        match exec.idata.modrm.reg as u8 {
            0 => Opcode64::add_rm64_imm8(exec)?,
            1 => Opcode64::or_rm64_imm8(exec)?,
            2 => Opcode64::adc_rm64_imm8(exec)?,
            3 => Opcode64::sbb_rm64_imm8(exec)?,
            4 => Opcode64::and_rm64_imm8(exec)?,
            5 => Opcode64::sub_rm64_imm8(exec)?,
            6 => Opcode64::xor_rm64_imm8(exec)?,
            7 => Opcode64::cmp_rm64_imm8(exec)?,
            _ => { return Err(EmuException::UnexpectedError); },
        }
        Ok(())
    }

    add_dst_src!(64, rm64, imm8);
    or_dst_src!(64, rm64, imm8);
    adc_dst_src!(64, rm64, imm8);
    sbb_dst_src!(64, rm64, imm8);
    and_dst_src!(64, rm64, imm8);
    sub_dst_src!(64, rm64, imm8);
    xor_dst_src!(64, rm64, imm8);
    cmp_dst_src!(64, rm64, imm8);

    fn code_c1(exec: &mut exec::Exec) -> Result<(), EmuException> {
        match exec.idata.modrm.reg as u8 {
            /*
            0 => Opcode64::rol_rm64_imm8(exec)?,
            1 => Opcode64::ror_rm64_imm8(exec)?,
            2 => Opcode64::rcl_rm64_imm8(exec)?,
            3 => Opcode64::rcr_rm64_imm8(exec)?,
            */
            4 => Opcode64::shl_rm64_imm8(exec)?,
            5 => Opcode64::shr_rm64_imm8(exec)?,
            6 => Opcode64::sal_rm64_imm8(exec)?,
            7 => Opcode64::sar_rm64_imm8(exec)?,
            _ => { return Err(EmuException::UnexpectedError); },
        }
        Ok(())
    }

    /*
    rol_dst_src!(64, rm64, imm8);
    ror_dst_src!(64, rm64, imm8);
    rcl_dst_src!(64, rm64, imm8);
    rcr_dst_src!(64, rm64, imm8);
    */
    shl_dst_src!(64, rm64, imm8);
    shr_dst_src!(64, rm64, imm8);
    sal_dst_src!(64, rm64, imm8);
    sar_dst_src!(64, rm64, imm8);

    fn code_d3(exec: &mut exec::Exec) -> Result<(), EmuException> {
        match exec.idata.modrm.reg as u8 {
            /*
            0 => Opcode64::rol_rm64_cl(exec)?,
            1 => Opcode64::ror_rm64_cl(exec)?,
            2 => Opcode64::rcl_rm64_cl(exec)?,
            3 => Opcode64::rcr_rm64_cl(exec)?,
            */
            4 => Opcode64::shl_rm64_cl(exec)?,
            5 => Opcode64::shr_rm64_cl(exec)?,
            6 => Opcode64::sal_rm64_cl(exec)?,
            7 => Opcode64::sar_rm64_cl(exec)?,
            _ => { return Err(EmuException::UnexpectedError); },
        }
        Ok(())
    }

    /*
    rol_dst_src!(64, rm64, cl);
    ror_dst_src!(64, rm64, cl);
    rcl_dst_src!(64, rm64, cl);
    rcr_dst_src!(64, rm64, cl);
    */
    shl_dst_src!(64, rm64, cl);
    shr_dst_src!(64, rm64, cl);
    sal_dst_src!(64, rm64, cl);
    sar_dst_src!(64, rm64, cl);

    fn code_f7(exec: &mut exec::Exec) -> Result<(), EmuException> {
        let back = match exec.idata.modrm.reg as u8 {
            0 => { Opcode64::test_rm64_imm64(exec)?; 0},
            2 => { Opcode64::not_rm64(exec)?; -8},
            3 => { Opcode64::neg_rm64(exec)?; -8},
            4 => { Opcode64::mul_rdx_rax_rm64(exec)?; -8},
            5 => { Opcode64::imul_rdx_rax_rm64(exec)?; -8},
            6 => { Opcode64::div_rax_rdx_rm64(exec)?; -8},
            7 => { Opcode64::idiv_rax_rdx_rm64(exec)?; -8},
            _ => { return Err(EmuException::UnexpectedError); },
        };
        exec.ac.update_ip(back)
    }

    test_dst_src!(64, rm64, imm64);
    not_dst!(64, rm64);
    neg_dst!(64, rm64);
    mul_high_low_src!(64, rdx, rax, rm64);
    imul_high_low_src!(64, rdx, rax, rm64);
    div_quot_rem_src!(64, rax, rdx, rm64);
    idiv_quot_rem_src!(64, rax, rdx, rm64);

    fn code_ff(exec: &mut exec::Exec) -> Result<(), EmuException> {
        match exec.idata.modrm.reg as u8 {
            0 => Opcode64::inc_rm64(exec)?,
            1 => Opcode64::dec_rm64(exec)?,
            _ => { return Err(EmuException::UnexpectedError); },
        }
        Ok(())
    }

    inc_dst!(rm64);
    dec_dst!(rm64);

    fn code_0f01(exec: &mut exec::Exec) -> Result<(), EmuException> {
        match exec.idata.modrm.reg as u8 {
            2 => Opcode64::lgdt_m16_64(exec)?,
            3 => Opcode64::lidt_m16_64(exec)?,
            _ => { return Err(EmuException::NotImplementedOpcode); },
        }
        Ok(())
    }

    fn lgdt_m16_64(exec: &mut exec::Exec) -> Result<(), EmuException> {
        let (sg, adr) = exec.get_m()?;

        if exec.ac.get_cpl()? > 0 {
            return Err(EmuException::CPUException(CPUException::GP(None)));
        }

        let limit = exec.ac.get_data16((sg,adr))?;
        let base  = exec.ac.get_data64((sg,adr+2))?;
        debug!("lgdt: base = {:016x}, limit = {:04x}", base, limit);
        exec.ac.set_gdtr(base, limit)
    }

    fn lidt_m16_64(exec: &mut exec::Exec) -> Result<(), EmuException> {
        let (sg, adr) = exec.get_m()?;

        if exec.ac.get_cpl()? > 0 {
            return Err(EmuException::CPUException(CPUException::GP(None)));
        }

        let limit = exec.ac.get_data16((sg,adr))?;
        let base  = exec.ac.get_data64((sg,adr+2))?;
        debug!("lidt: base = {:016x}, limit = {:04x}", base, limit);
        exec.ac.set_idtr(base, limit)
    }
}
