use std::convert::TryFrom;
use crate::emulator::*;
use crate::emulator::access::register::*;
use crate::emulator::instruction::opcode::*;

pub struct Opcode32 (pub super::OpcodeArr);
impl Opcode32 {
    pub fn new(op: super::OpcodeArr) -> Self {
        Self (op)
    }
}

impl super::OpcodeTrait for Opcode32 {
    fn init_opcode(&mut self) -> () {
        macro_rules! setop {
            ($n:expr, $fnc:ident, $flg:expr) => { self.0[$n & 0x1ff] = OpcodeType{func:Self::$fnc, flag:$flg} }
        }

        // 0x00 : add_rm8_r8
        setop!(0x01, add_rm32_r32,  OpFlags::MODRM);
        // 0x02 : add_r8_rm8
        setop!(0x03, add_r32_rm32,  OpFlags::MODRM);
        // 0x04 : add_al_imm8
        setop!(0x05, add_eax_imm32, OpFlags::IMM32);
        // 0x08 : or_rm8_r8
        setop!(0x09, or_rm32_r32,   OpFlags::MODRM);
        // 0x0a : or_r8_rm8
        setop!(0x0b, or_r32_rm32,   OpFlags::MODRM);
        // 0x0c : or_al_imm8
        setop!(0x0d, or_eax_imm32,  OpFlags::IMM32);

        // 0x10 : adc_rm8_r8
        setop!(0x11, adc_rm32_r32,  OpFlags::MODRM);
        // 0x12 : adc_r8_rm8
        setop!(0x13, adc_r32_rm32,  OpFlags::MODRM);
        // 0x14 : adc_al_imm8
        setop!(0x15, adc_eax_imm32, OpFlags::IMM32);

        // 0x18 : sbb_rm8_r8
        setop!(0x19, sbb_rm32_r32,  OpFlags::MODRM);
        // 0x1a : sbb_r8_rm8
        setop!(0x1b, sbb_r32_rm32,  OpFlags::MODRM);
        // 0x1c : sbb_al_imm8
        setop!(0x1d, sbb_eax_imm32, OpFlags::IMM32);

        // 0x20 : and_rm8_r8
        setop!(0x21, and_rm32_r32,  OpFlags::MODRM);
        // 0x22 : and_r8_rm8
        setop!(0x23, and_r32_rm32,  OpFlags::MODRM);
        // 0x24 : and_al_imm8
        setop!(0x25, and_eax_imm32, OpFlags::IMM32);
        // 0x28 : sub_rm8_r8
        setop!(0x29, sub_rm32_r32,  OpFlags::MODRM);
        // 0x2a : sub_r8_rm8
        setop!(0x2b, sub_r32_rm32,  OpFlags::MODRM);
        // 0x2c : sub_al_imm8
        setop!(0x2d, sub_eax_imm32, OpFlags::IMM32);

        // 0x30 : xor_rm8_r8
        setop!(0x31, xor_rm32_r32,  OpFlags::MODRM);
        // 0x32 : xor_r8_rm8
        setop!(0x33, xor_r32_rm32,  OpFlags::MODRM);
        // 0x34 : xor_al_imm8
        setop!(0x35, xor_eax_imm32, OpFlags::IMM32);
        // 0x38 : cmp_rm8_r8
        setop!(0x39, cmp_rm32_r32,  OpFlags::MODRM);
        // 0x3a : cmp_r8_rm8
        setop!(0x3b, cmp_r32_rm32,  OpFlags::MODRM);
        // 0x3c : cmp_al_imm8
        setop!(0x3d, cmp_eax_imm32, OpFlags::IMM32);

        for i in 0..8 {
            setop!(0x40+i, inc_opr32,   OpFlags::NONE);
            setop!(0x48+i, dec_opr32,   OpFlags::NONE);
            setop!(0x50+i, push_opr32,  OpFlags::NONE);
            setop!(0x58+i, pop_opr32,   OpFlags::NONE);
        }

        setop!(0x60, pushad,            OpFlags::NONE);
        setop!(0x61, popad,             OpFlags::NONE);

        setop!(0x68, push_imm32,            OpFlags::IMM32);
        setop!(0x69, imul_r32_rm32_imm32,   OpFlags::MODRM | OpFlags::IMM32);
        setop!(0x6a, push_imm8,             OpFlags::IMM8);
        setop!(0x6b, imul_r32_rm32_imm8,    OpFlags::MODRM | OpFlags::IMM8);

        // 0x70-0x7f : jcc

        // 0x84 : test_rm8_r8
        setop!(0x85, test_rm32_r32,     OpFlags::MODRM);
        // 0x86 : xchg_r8_rm8
        setop!(0x87, xchg_r32_rm32,     OpFlags::MODRM);
        // 0x88 : mov_rm8_r8
        setop!(0x89, mov_rm32_r32,      OpFlags::MODRM);
        // 0x8a : mov_r8_rm8
        setop!(0x8b, mov_r32_rm32,      OpFlags::MODRM);
        setop!(0x8c, mov_rm32_sreg,     OpFlags::MODRM);
        setop!(0x8d, lea_r32_m32,       OpFlags::MODRM);
        // 0x8e : mov_sreg_rm32

        // 0x90 : nop

        for i in 0..8 {
            setop!(0x90+i, xchg_eax_opr32,     OpFlags::NONE);
        }
        setop!(0x98, cwde,              OpFlags::NONE);
        setop!(0x99, cdq,               OpFlags::NONE);
        setop!(0x9a, callf_ptr16_imm32, OpFlags::PTR16 | OpFlags::IMM32);

        setop!(0x9c, pushf,             OpFlags::NONE);
        setop!(0x9d, popf,              OpFlags::NONE);

        // 0xa0 : mov_al_moffs8
        setop!(0xa1, mov_eax_moffs32,   OpFlags::MOFFS);
        // 0xa2 : mov_moffs8_al
        setop!(0xa3, mov_moffs32_eax,   OpFlags::MOFFS);
        setop!(0xa4, movs_m8,           OpFlags::NONE);
        setop!(0xa5, movs_m32,          OpFlags::NONE);
        setop!(0xa6, cmps_m8,           OpFlags::NONE);
        setop!(0xa7, cmps_m32,          OpFlags::NONE);
        // 0xa8 : test_al_imm8
        setop!(0xa9, test_eax_imm32,    OpFlags::IMM32);
        setop!(0xaa, stos_m8,           OpFlags::NONE);
        setop!(0xab, stos_m32,          OpFlags::NONE);
        setop!(0xac, lods_m8,           OpFlags::NONE);
        setop!(0xad, lods_m32,          OpFlags::NONE);
        setop!(0xae, scas_m8,           OpFlags::NONE);
        setop!(0xaf, scas_m32,          OpFlags::NONE);

        // 0xb0-0xb7 : mov_r8_imm
        for i in 0..8 {
            setop!(0xb8+i, mov_opr32_imm32,   OpFlags::IMM32);
        }

        setop!(0xc3, ret,               OpFlags::NONE);

        setop!(0xc7, mov_rm32_imm32,    OpFlags::MODRM | OpFlags::IMM32);

        setop!(0xc9, leave,             OpFlags::NONE);

        setop!(0xcb, retf,              OpFlags::NONE);
        // 0xcc : int3
        // 0xcd : int_imm8

        setop!(0xcf, iret,              OpFlags::NONE);

        // 0xe4 : in_al_imm8
        setop!(0xe5, in_eax_imm8,       OpFlags::IMM8);
        // 0xe6 : out_imm8_al
        setop!(0xe7, out_imm8_eax,      OpFlags::IMM8);
        setop!(0xe8, call_rel_imm32,    OpFlags::IMM32);
        setop!(0xe9, jmp_rel_imm32,     OpFlags::IMM32);
        setop!(0xea, jmpf_ptr16_imm32,  OpFlags::PTR16 | OpFlags::IMM32);
        // 0xeb : jmp_imm8
        // 0xec : in_al_dx
        setop!(0xed, in_eax_dx,         OpFlags::NONE);
        // 0xee : out_dx_al
        setop!(0xef, out_dx_eax,        OpFlags::NONE);

        setop!(0x0f80, jo_imm32,        OpFlags::IMM32);
        setop!(0x0f81, jno_imm32,       OpFlags::IMM32);
        setop!(0x0f82, jb_imm32,        OpFlags::IMM32);
        setop!(0x0f83, jnb_imm32,       OpFlags::IMM32);
        setop!(0x0f84, jz_imm32,        OpFlags::IMM32);
        setop!(0x0f85, jnz_imm32,       OpFlags::IMM32);
        setop!(0x0f86, jbe_imm32,       OpFlags::IMM32);
        setop!(0x0f87, jnbe_imm32,      OpFlags::IMM32);
        setop!(0x0f88, js_imm32,        OpFlags::IMM32);
        setop!(0x0f89, jns_imm32,       OpFlags::IMM32);
        setop!(0x0f8a, jp_imm32,        OpFlags::IMM32);
        setop!(0x0f8b, jnp_imm32,       OpFlags::IMM32);
        setop!(0x0f8c, jl_imm32,        OpFlags::IMM32);
        setop!(0x0f8d, jnl_imm32,       OpFlags::IMM32);
        setop!(0x0f8e, jle_imm32,       OpFlags::IMM32);
        setop!(0x0f8f, jnle_imm32,      OpFlags::IMM32);

        setop!(0x0faf, imul_r32_rm32,   OpFlags::MODRM);

        setop!(0x0fb6, movzx_r32_rm8,   OpFlags::MODRM);
        setop!(0x0fb7, movzx_r32_rm32,  OpFlags::MODRM);

        setop!(0x0fbe, movsx_r32_rm8,   OpFlags::MODRM);
        setop!(0x0fbf, movsx_r32_rm32,  OpFlags::MODRM);

        // 0x80 : code_80
        setop!(0x81, code_81, OpFlags::MODRM | OpFlags::IMM32);
        setop!(0x82, code_82, OpFlags::MODRM | OpFlags::IMM8);
        setop!(0x83, code_83, OpFlags::MODRM | OpFlags::IMM8);
        // 0xc0 : code_c0
        setop!(0xc1, code_c1, OpFlags::MODRM | OpFlags::IMM8);
        // 0xd2 : code_d2
        setop!(0xd3, code_d3, OpFlags::MODRM);
        // 0xf6 : code_f6
        setop!(0xf7, code_f7, OpFlags::MODRM | OpFlags::IMM32);
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

impl Opcode32 {
    add_dst_src!(32, rm32, r32);
    add_dst_src!(32, r32, rm32);
    add_dst_src!(32, eax, imm32);

    or_dst_src!(32, rm32, r32);
    or_dst_src!(32, r32, rm32);
    or_dst_src!(32, eax, imm32);

    adc_dst_src!(32, rm32, r32);
    adc_dst_src!(32, r32, rm32);
    adc_dst_src!(32, eax, imm32);

    sbb_dst_src!(32, rm32, r32);
    sbb_dst_src!(32, r32, rm32);
    sbb_dst_src!(32, eax, imm32);

    and_dst_src!(32, rm32, r32);
    and_dst_src!(32, r32, rm32);
    and_dst_src!(32, eax, imm32);

    sub_dst_src!(32, rm32, r32);
    sub_dst_src!(32, r32, rm32);
    sub_dst_src!(32, eax, imm32);

    xor_dst_src!(32, rm32, r32);
    xor_dst_src!(32, r32, rm32);
    xor_dst_src!(32, eax, imm32);

    cmp_dst_src!(32, rm32, r32);
    cmp_dst_src!(32, r32, rm32);
    cmp_dst_src!(32, eax, imm32);

    inc_dst!(opr32);
    dec_dst!(opr32);
    push_src!(32, opr32);
    pop_dst!(32, opr32);

    fn pushad(exec: &mut exec::Exec) -> Result<(), EmuException> {
        debug!("pushad");
        let sp = exec.ac.get_gpreg(GpReg32::ESP)?;
        for i in 0..4 {
            exec.ac.push_u32(exec.ac.get_gpreg(GpReg32::try_from(i).unwrap())?)?;
        }
        exec.ac.push_u32(sp)?;
        for i in 5..8 {
            exec.ac.push_u32(exec.ac.get_gpreg(GpReg32::try_from(i).unwrap())?)?;
        }
        Ok(())
    }

    fn popad(exec: &mut exec::Exec) -> Result<(), EmuException> {
        debug!("popad");
        for i in (5..8).rev() {
            let v = exec.ac.pop_u32()?;
            exec.ac.set_gpreg(GpReg32::try_from(i).unwrap(), v)?;
        }
        let sp = exec.ac.pop_u32()?;
        for i in (0..4).rev() {
            let v = exec.ac.pop_u32()?;
            exec.ac.set_gpreg(GpReg32::try_from(i).unwrap(), v)?;
        }
        exec.ac.set_gpreg(GpReg32::ESP, sp)
    }

    push_src!(32, imm8);
    imul_dst_src1_src2!(32, r32, rm32, imm32);
    push_src!(32, imm32);
    imul_dst_src1_src2!(32, r32, rm32, imm8);

    test_dst_src!(32, rm32, r32);
    xchg_dst_src!(32, r32, rm32);
    mov_dst_src!(32, rm32, r32);
    mov_dst_src!(32, r32, rm32);
    mov_dst_src!(32, rm32, sreg);
    lea_dst_src!(32, r32, m32);

    xchg_dst_src!(32, eax, opr32);

    fn cwde(exec: &mut exec::Exec) -> Result<(), EmuException> {
        let ax = exec.ac.get_gpreg(GpReg16::AX)? as i16;
        exec.ac.set_gpreg(GpReg32::EAX, ax as u32)
    }

    fn cdq(exec: &mut exec::Exec) -> Result<(), EmuException> {
        let eax = exec.ac.get_gpreg(GpReg32::EAX)? as i32;
        exec.ac.set_gpreg(GpReg32::EDX, if eax < 0 { u32::MAX } else { 0 })
    }

    callf_abs!(32, ptr16, imm32);

    pushf!(32);
    popf!(32);

    mov_dst_src!(32, eax, moffs32);
    mov_dst_src!(32, moffs32, eax);
    movs_dst_src!(32, 8);
    movs_dst_src!(32, 32);
    cmps_src_dst!(32, 8);
    cmps_src_dst!(32, 32);

    test_dst_src!(32, eax, imm32);
    stos_dst_src!(32, 8);
    stos_dst_src!(32, 32);
    lods_dst_src!(32, 8);
    lods_dst_src!(32, 32);
    scas_src_dst!(32, 8);
    scas_src_dst!(32, 32);

    mov_dst_src!(32, opr32, imm32);

    ret!(32);

    mov_dst_src!(32, rm32, imm32);

    fn leave(exec: &mut exec::Exec) -> Result<(), EmuException> {
        let ebp = exec.ac.get_gpreg(GpReg32::EBP)?;
        exec.ac.set_gpreg(GpReg32::ESP, ebp)?;
        let new_ebp = exec.ac.pop_u32()?;
        debug!("leave: esp <- 0x{:08x}, ebp <- 0x{:08x}", ebp, new_ebp);
        exec.ac.set_gpreg(GpReg32::EBP, new_ebp)
    }

    retf!(32);

    iret!(32);

    in_reg_port!(32, eax, imm8);
    out_port_reg!(32, imm8, eax);
    call_rel!(32, imm32);
    jmp_rel!(32, imm32);
    jmpf_abs!(32, ptr16, imm32);
    in_reg_port!(32, eax, dx);
    out_port_reg!(32, dx, eax);

    jcc_rel!(32, o, imm32);
    jcc_rel!(32, b, imm32);
    jcc_rel!(32, z, imm32);
    jcc_rel!(32, be, imm32);
    jcc_rel!(32, s, imm32);
    jcc_rel!(32, p, imm32);
    jcc_rel!(32, l, imm32);
    jcc_rel!(32, le, imm32);

    imul_dst_src!(32, r32, rm32);

    movzx_dst_src!(32, r32, 8, rm8);
    movzx_dst_src!(32, r32, 32, rm32);
    movsx_dst_src!(32, r32, 8, rm8);
    movsx_dst_src!(32, r32, 32, rm32);

    fn code_81(exec: &mut exec::Exec) -> Result<(), EmuException> {
        match exec.idata.modrm.reg as u8 {
            0 => Opcode32::add_rm32_imm32(exec)?,
            1 => Opcode32::or_rm32_imm32(exec)?,
            2 => Opcode32::adc_rm32_imm32(exec)?,
            3 => Opcode32::sbb_rm32_imm32(exec)?,
            4 => Opcode32::and_rm32_imm32(exec)?,
            5 => Opcode32::sub_rm32_imm32(exec)?,
            6 => Opcode32::xor_rm32_imm32(exec)?,
            7 => Opcode32::cmp_rm32_imm32(exec)?,
            _ => { return Err(EmuException::UnexpectedError); },
        }
        Ok(())
    }

    add_dst_src!(32, rm32, imm32);
    or_dst_src!(32, rm32, imm32);
    adc_dst_src!(32, rm32, imm32);
    sbb_dst_src!(32, rm32, imm32);
    and_dst_src!(32, rm32, imm32);
    sub_dst_src!(32, rm32, imm32);
    xor_dst_src!(32, rm32, imm32);
    cmp_dst_src!(32, rm32, imm32);

    fn code_82(exec: &mut exec::Exec) -> Result<(), EmuException> {
        super::common::code_82(exec)
    }

    fn code_83(exec: &mut exec::Exec) -> Result<(), EmuException> {
        match exec.idata.modrm.reg as u8 {
            0 => Opcode32::add_rm32_imm8(exec)?,
            1 => Opcode32::or_rm32_imm8(exec)?,
            2 => Opcode32::adc_rm32_imm8(exec)?,
            3 => Opcode32::sbb_rm32_imm8(exec)?,
            4 => Opcode32::and_rm32_imm8(exec)?,
            5 => Opcode32::sub_rm32_imm8(exec)?,
            6 => Opcode32::xor_rm32_imm8(exec)?,
            7 => Opcode32::cmp_rm32_imm8(exec)?,
            _ => { return Err(EmuException::UnexpectedError); },
        }
        Ok(())
    }

    add_dst_src!(32, rm32, imm8);
    or_dst_src!(32, rm32, imm8);
    adc_dst_src!(32, rm32, imm8);
    sbb_dst_src!(32, rm32, imm8);
    and_dst_src!(32, rm32, imm8);
    sub_dst_src!(32, rm32, imm8);
    xor_dst_src!(32, rm32, imm8);
    cmp_dst_src!(32, rm32, imm8);

    fn code_c1(exec: &mut exec::Exec) -> Result<(), EmuException> {
        match exec.idata.modrm.reg as u8 {
            /*
            0 => Opcode32::rol_rm32_imm8(exec)?,
            1 => Opcode32::ror_rm32_imm8(exec)?,
            2 => Opcode32::rcl_rm32_imm8(exec)?,
            3 => Opcode32::rcr_rm32_imm8(exec)?,
            */
            4 => Opcode32::shl_rm32_imm8(exec)?,
            5 => Opcode32::shr_rm32_imm8(exec)?,
            6 => Opcode32::sal_rm32_imm8(exec)?,
            7 => Opcode32::sar_rm32_imm8(exec)?,
            _ => { return Err(EmuException::UnexpectedError); },
        }
        Ok(())
    }

    /*
    rol_dst_src!(32, rm32, imm8);
    ror_dst_src!(32, rm32, imm8);
    rcl_dst_src!(32, rm32, imm8);
    rcr_dst_src!(32, rm32, imm8);
    */
    shl_dst_src!(32, rm32, imm8);
    shr_dst_src!(32, rm32, imm8);
    sal_dst_src!(32, rm32, imm8);
    sar_dst_src!(32, rm32, imm8);

    fn code_d3(exec: &mut exec::Exec) -> Result<(), EmuException> {
        match exec.idata.modrm.reg as u8 {
            /*
            0 => Opcode32::rol_rm32_cl(exec)?,
            1 => Opcode32::ror_rm32_cl(exec)?,
            2 => Opcode32::rcl_rm32_cl(exec)?,
            3 => Opcode32::rcr_rm32_cl(exec)?,
            */
            4 => Opcode32::shl_rm32_cl(exec)?,
            5 => Opcode32::shr_rm32_cl(exec)?,
            6 => Opcode32::sal_rm32_cl(exec)?,
            7 => Opcode32::sar_rm32_cl(exec)?,
            _ => { return Err(EmuException::UnexpectedError); },
        }
        Ok(())
    }

    /*
    rol_dst_src!(32, rm32, cl);
    ror_dst_src!(32, rm32, cl);
    rcl_dst_src!(32, rm32, cl);
    rcr_dst_src!(32, rm32, cl);
    */
    shl_dst_src!(32, rm32, cl);
    shr_dst_src!(32, rm32, cl);
    sal_dst_src!(32, rm32, cl);
    sar_dst_src!(32, rm32, cl);

    fn code_f7(exec: &mut exec::Exec) -> Result<(), EmuException> {
        let back = match exec.idata.modrm.reg as u8 {
            0 => { Opcode32::test_rm32_imm32(exec)?; 0},
            2 => { Opcode32::not_rm32(exec)?; -4},
            3 => { Opcode32::neg_rm32(exec)?; -4},
            4 => { Opcode32::mul_edx_eax_rm32(exec)?; -4},
            5 => { Opcode32::imul_edx_eax_rm32(exec)?; -4},
            6 => { Opcode32::div_eax_edx_rm32(exec)?; -4},
            7 => { Opcode32::idiv_eax_edx_rm32(exec)?; -4},
            _ => { return Err(EmuException::UnexpectedError); },
        };
        exec.ac.update_ip(back)
    }

    test_dst_src!(32, rm32, imm32);
    not_dst!(32, rm32);
    neg_dst!(32, rm32);
    mul_high_low_src!(32, edx, eax, rm32);
    imul_high_low_src!(32, edx, eax, rm32);
    div_quot_rem_src!(32, eax, edx, rm32);
    idiv_quot_rem_src!(32, eax, edx, rm32);

    fn code_ff(exec: &mut exec::Exec) -> Result<(), EmuException> {
        match exec.idata.modrm.reg as u8 {
            0 => Opcode32::inc_rm32(exec)?,
            1 => Opcode32::dec_rm32(exec)?,
            2 => Opcode32::call_abs_rm32(exec)?,
            3 => Opcode32::callf_m16_32(exec)?,
            4 => Opcode32::jmp_abs_rm32(exec)?,
            5 => Opcode32::jmpf_m16_32(exec)?,
            6 => Opcode32::push_rm32(exec)?,
            _ => { return Err(EmuException::UnexpectedError); },
        }
        Ok(())
    }

    inc_dst!(rm32);
    dec_dst!(rm32);
    call_abs!(32, rm32);
    jmp_abs!(32, rm32);
    push_src!(32, rm32);

    fn callf_m16_32(exec: &mut exec::Exec) -> Result<(), EmuException> {
        let (sg, adr) = exec.get_m()?;

        let sel = exec.ac.get_data16((sg,adr))?;
        let abs = exec.ac.get_data32((sg,adr+2))?;
        debug!("callf: {:04x}:{:08x}", sel, abs);
        exec.call_far_u32(sel, abs)
    }

    fn jmpf_m16_32(exec: &mut exec::Exec) -> Result<(), EmuException> {
        let (sg, adr) = exec.get_m()?;

        let sel = exec.ac.get_data16((sg,adr))?;
        let abs = exec.ac.get_data32((sg,adr+2))?;
        debug!("jmpf: {:04x}:{:08x}", sel, abs);
        exec.jmp_far_u32(sel, abs)
    }

    fn code_0f01(exec: &mut exec::Exec) -> Result<(), EmuException> {
        match exec.idata.modrm.reg as u8 {
            2 => Opcode32::lgdt_m16_32(exec)?,
            3 => Opcode32::lidt_m16_32(exec)?,
            _ => { return Err(EmuException::NotImplementedOpcode); },
        }
        Ok(())
    }

    fn lgdt_m16_32(exec: &mut exec::Exec) -> Result<(), EmuException> {
        let (sg, adr) = exec.get_m()?;

        if exec.ac.get_cpl()? > 0 {
            return Err(EmuException::CPUException(CPUException::GP(None)));
        }

        let limit = exec.ac.get_data16((sg,adr))?;
        let base  = exec.ac.get_data32((sg,adr+2))?;
        debug!("lgdt: base = {:08x}, limit = {:04x}", base, limit);
        exec.ac.set_gdtr(base as u64, limit)
    }

    fn lidt_m16_32(exec: &mut exec::Exec) -> Result<(), EmuException> {
        let (sg, adr) = exec.get_m()?;

        if exec.ac.get_cpl()? > 0 {
            return Err(EmuException::CPUException(CPUException::GP(None)));
        }

        let limit = exec.ac.get_data16((sg,adr))?;
        let base  = exec.ac.get_data32((sg,adr+2))?;
        debug!("lidt: base = {:08x}, limit = {:04x}", base, limit);
        exec.ac.set_idtr(base as u64, limit)
    }
}
