use std::convert::TryFrom;
use crate::emulator::*;
use crate::emulator::access::register::*;
use crate::emulator::instruction::opcode::*;

pub struct Opcode16 (pub super::OpcodeArr);
impl Opcode16 {
    pub fn new(op: super::OpcodeArr) -> Self {
        Self (op)
    }
}

impl super::OpcodeTrait for Opcode16 {
    fn init_opcode(&mut self) -> () {
        macro_rules! setop {
            ($n:expr, $fnc:ident, $flg:expr) => { self.0[$n & 0x1ff] = OpcodeType{func:Self::$fnc, flag:$flg} }
        }

        // 0x00 : add_rm8_r8
        setop!(0x01, add_rm16_r16,  OpFlags::MODRM);
        // 0x02 : add_r8_rm8
        setop!(0x03, add_r16_rm16,  OpFlags::MODRM);
        // 0x04 : add_al_imm8
        setop!(0x05, add_ax_imm16,  OpFlags::IMM16);
        // 0x08 : or_rm8_r8
        setop!(0x09, or_rm16_r16,   OpFlags::MODRM);
        // 0x0a : or_r8_rm8
        setop!(0x0b, or_r16_rm16,   OpFlags::MODRM);
        // 0x0c : or_al_imm8
        setop!(0x0d, or_ax_imm16,   OpFlags::IMM16);

        // 0x10 : adc_rm8_r8
        setop!(0x11, adc_rm16_r16,  OpFlags::MODRM);
        // 0x12 : adc_r8_rm8
        setop!(0x13, adc_r16_rm16,  OpFlags::MODRM);
        // 0x14 : adc_al_imm8
        setop!(0x15, adc_ax_imm16,  OpFlags::IMM16);

        // 0x18 : sbb_rm8_r8
        setop!(0x19, sbb_rm16_r16,  OpFlags::MODRM);
        // 0x1a : sbb_r8_rm8
        setop!(0x1b, sbb_r16_rm16,  OpFlags::MODRM);
        // 0x1c : sbb_al_imm8
        setop!(0x1d, sbb_ax_imm16,  OpFlags::IMM16);

        // 0x20 : and_rm8_r8
        setop!(0x21, and_rm16_r16,  OpFlags::MODRM);
        // 0x22 : and_r8_rm8
        setop!(0x23, and_r16_rm16,  OpFlags::MODRM);
        // 0x24 : and_al_imm8
        setop!(0x25, and_ax_imm16,  OpFlags::IMM16);
        // 0x28 : sub_rm8_r8
        setop!(0x29, sub_rm16_r16,  OpFlags::MODRM);
        // 0x2a : sub_r8_rm8
        setop!(0x2b, sub_r16_rm16,  OpFlags::MODRM);
        // 0x2c : sub_al_imm8
        setop!(0x2d, sub_ax_imm16,  OpFlags::IMM16);

        // 0x30 : xor_rm8_r8
        setop!(0x31, xor_rm16_r16,  OpFlags::MODRM);
        // 0x32 : xor_r8_rm8
        setop!(0x33, xor_r16_rm16,  OpFlags::MODRM);
        // 0x34 : xor_al_imm8
        setop!(0x35, xor_ax_imm16,  OpFlags::IMM16);
        // 0x38 : cmp_rm8_r8
        setop!(0x39, cmp_rm16_r16,  OpFlags::MODRM);
        // 0x3a : cmp_r8_rm8
        setop!(0x3b, cmp_r16_rm16,  OpFlags::MODRM);
        // 0x3c : cmp_al_imm8
        setop!(0x3d, cmp_ax_imm16,  OpFlags::IMM16);

        for i in 0..8 {
            setop!(0x40+i, inc_opr16,   OpFlags::NONE);
            setop!(0x48+i, dec_opr16,   OpFlags::NONE);
            setop!(0x50+i, push_opr16,  OpFlags::NONE);
            setop!(0x58+i, pop_opr16,   OpFlags::NONE);
        }

        setop!(0x60, pusha,             OpFlags::NONE);
        setop!(0x61, popa,              OpFlags::NONE);

        setop!(0x68, push_imm16,            OpFlags::IMM16);
        setop!(0x69, imul_r16_rm16_imm16,   OpFlags::MODRM | OpFlags::IMM16);
        setop!(0x6a, push_imm8,             OpFlags::IMM8);
        setop!(0x6b, imul_r16_rm16_imm8,    OpFlags::MODRM | OpFlags::IMM8);

        // 0x70-0x7f : jcc

        // 0x84 : test_rm8_r8
        setop!(0x85, test_rm16_r16,     OpFlags::MODRM);
        // 0x86 : xchg_r8_rm8
        setop!(0x87, xchg_r16_rm16,     OpFlags::MODRM);
        // 0x88 : mov_rm8_r8
        setop!(0x89, mov_rm16_r16,      OpFlags::MODRM);
        // 0x8a : mov_r8_rm8
        setop!(0x8b, mov_r16_rm16,      OpFlags::MODRM);
        setop!(0x8c, mov_rm16_sreg,     OpFlags::MODRM);
        setop!(0x8d, lea_r16_m16,       OpFlags::MODRM);
        // 0x8e : mov_sreg_rm16

        // 0x90 : nop

        for i in 0..8 {
            setop!(0x90+i, xchg_ax_opr16,     OpFlags::NONE);
        }
        setop!(0x98, cbw,               OpFlags::NONE);
        setop!(0x99, cwd,               OpFlags::NONE);
        setop!(0x9a, callf_ptr16_imm16, OpFlags::PTR16 | OpFlags::IMM16);
            
        setop!(0x9c, pushf,             OpFlags::NONE);
        setop!(0x9d, popf,              OpFlags::NONE);
            
        // 0xa0 : mov_al_moffs8
        setop!(0xa1, mov_ax_moffs16,    OpFlags::MOFFS);
        // 0xa2 : mov_moffs8_al
        setop!(0xa3, mov_moffs16_ax,    OpFlags::MOFFS);
        setop!(0xa4, movs_m8,           OpFlags::NONE);
        setop!(0xa5, movs_m16,          OpFlags::NONE);
        setop!(0xa6, cmps_m8,           OpFlags::NONE);
        setop!(0xa7, cmps_m16,          OpFlags::NONE);
        // 0xa8 : test_al_imm8
        setop!(0xa9, test_ax_imm16,     OpFlags::IMM16);
        setop!(0xaa, stos_m8,           OpFlags::NONE);
        setop!(0xab, stos_m16,          OpFlags::NONE);
        setop!(0xac, lods_m8,           OpFlags::NONE);
        setop!(0xad, lods_m16,          OpFlags::NONE);
        setop!(0xae, scas_m8,           OpFlags::NONE);
        setop!(0xaf, scas_m16,          OpFlags::NONE);
            
        // 0xb0-0xb7 : mov_r8_imm
        for i in 0..8 {
            setop!(0xb8+i, mov_opr16_imm16,   OpFlags::IMM16);
        }
            
        setop!(0xc3, ret,               OpFlags::NONE);
            
        setop!(0xc7, mov_rm16_imm16,    OpFlags::MODRM | OpFlags::IMM16);
            
        setop!(0xc9, leave,             OpFlags::NONE);
            
        setop!(0xcb, retf,              OpFlags::NONE);
        // 0xcc : int3
        // 0xcd : int_imm8
            
        setop!(0xcf, iret,              OpFlags::NONE);
            
        // 0xe4 : in_al_imm8
        setop!(0xe5, in_ax_imm8,        OpFlags::IMM8);
        // 0xe6 : out_imm8_al
        setop!(0xe7, out_imm8_ax,       OpFlags::IMM8);
        setop!(0xe8, call_imm16,        OpFlags::IMM16);
        setop!(0xe9, jmp_imm16,         OpFlags::IMM16);
        setop!(0xea, jmpf_ptr16_imm16,  OpFlags::PTR16 | OpFlags::IMM16);
        // 0xeb : jmp_imm8
        // 0xec : in_al_dx
        setop!(0xed, in_ax_dx,          OpFlags::NONE);
        // 0xee : out_dx_al
        setop!(0xef, out_dx_ax,         OpFlags::NONE);
            
        setop!(0x0f80, jo_imm16,        OpFlags::IMM16);
        setop!(0x0f81, jno_imm16,       OpFlags::IMM16);
        setop!(0x0f82, jb_imm16,        OpFlags::IMM16);
        setop!(0x0f83, jnb_imm16,       OpFlags::IMM16);
        setop!(0x0f84, jz_imm16,        OpFlags::IMM16);
        setop!(0x0f85, jnz_imm16,       OpFlags::IMM16);
        setop!(0x0f86, jbe_imm16,       OpFlags::IMM16);
        setop!(0x0f87, jnbe_imm16,      OpFlags::IMM16);
        setop!(0x0f88, js_imm16,        OpFlags::IMM16);
        setop!(0x0f89, jns_imm16,       OpFlags::IMM16);
        setop!(0x0f8a, jp_imm16,        OpFlags::IMM16);
        setop!(0x0f8b, jnp_imm16,       OpFlags::IMM16);
        setop!(0x0f8c, jl_imm16,        OpFlags::IMM16);
        setop!(0x0f8d, jnl_imm16,       OpFlags::IMM16);
        setop!(0x0f8e, jle_imm16,       OpFlags::IMM16);
        setop!(0x0f8f, jnle_imm16,      OpFlags::IMM16);
            
        setop!(0x0faf, imul_r16_rm16,   OpFlags::MODRM);

        setop!(0x0fb6, movzx_r16_rm8,   OpFlags::MODRM);
        setop!(0x0fb7, movzx_r16_rm16,  OpFlags::MODRM);
            
        setop!(0x0fbe, movsx_r16_rm8,   OpFlags::MODRM);
        setop!(0x0fbf, movsx_r16_rm16,  OpFlags::MODRM);

        // 0x80 : code_80
        setop!(0x81, code_81, OpFlags::MODRM | OpFlags::IMM16);
        // 0x82 : code_82
        setop!(0x83, code_83, OpFlags::MODRM | OpFlags::IMM8);
        /*
        // 0xc0 : code_c0
        setop!(0xc1, code_c1, OpFlags::MODRM | OpFlags::IMM8);
        setop!(0xd3, code_d3, OpFlags::MODRM);
        setop!(0xf7, code_f7, OpFlags::MODRM);
        setop!(0xff, code_ff, OpFlags::MODRM);
        */
        // 0x0f00 : code_0f00
        setop!(0x0f01, code_0f01, OpFlags::MODRM);
    }

    fn exec(&self, exec: &mut exec::Exec) -> Result<(), EmuException> {
        exec.ac.update_ip(exec.idata.len as i16)?;
        (self.0[exec.idata.opcode as usize].func)(exec)
    }
    fn flag(&self, opcode: u16) -> OpFlags { self.0[opcode as usize].flag }
}

impl Opcode16 {
    add_dst_src!(16, rm16, r16);
    add_dst_src!(16, r16, rm16);
    add_dst_src!(16, ax, imm16);

    or_dst_src!(16, rm16, r16);
    or_dst_src!(16, r16, rm16);
    or_dst_src!(16, ax, imm16);

    adc_dst_src!(16, rm16, r16);
    adc_dst_src!(16, r16, rm16);
    adc_dst_src!(16, ax, imm16);

    sbb_dst_src!(16, rm16, r16);
    sbb_dst_src!(16, r16, rm16);
    sbb_dst_src!(16, ax, imm16);

    and_dst_src!(16, rm16, r16);
    and_dst_src!(16, r16, rm16);
    and_dst_src!(16, ax, imm16);

    sub_dst_src!(16, rm16, r16);
    sub_dst_src!(16, r16, rm16);
    sub_dst_src!(16, ax, imm16);

    xor_dst_src!(16, rm16, r16);
    xor_dst_src!(16, r16, rm16);
    xor_dst_src!(16, ax, imm16);

    cmp_dst_src!(16, rm16, r16);
    cmp_dst_src!(16, r16, rm16);
    cmp_dst_src!(16, ax, imm16);

    inc_opr!(16);
    dec_opr!(16);
    push_src!(16, opr16);
    pop_dst!(16, opr16);

    fn pusha(exec: &mut exec::Exec) -> Result<(), EmuException> {
        debug!("pusha");
        let sp = exec.ac.get_gpreg(GpReg16::SP)?;
        for i in 0..4 {
            exec.ac.push_u16(exec.ac.get_gpreg(GpReg16::try_from(i).unwrap())?)?;
        }
        exec.ac.push_u16(sp)?;
        for i in 5..8 {
            exec.ac.push_u16(exec.ac.get_gpreg(GpReg16::try_from(i).unwrap())?)?;
        }
        Ok(())
    }

    fn popa(exec: &mut exec::Exec) -> Result<(), EmuException> {
        debug!("popa");
        for i in (5..8).rev() {
            let v = exec.ac.pop_u16()?;
            exec.ac.set_gpreg(GpReg16::try_from(i).unwrap(), v)?;
        }
        let sp = exec.ac.pop_u16()?;
        for i in (0..4).rev() {
            let v = exec.ac.pop_u16()?;
            exec.ac.set_gpreg(GpReg16::try_from(i).unwrap(), v)?;
        }
        exec.ac.set_gpreg(GpReg16::SP, sp)
    }

    push_src!(16, imm8);
    imul_dst_src1_src2!(16, r16, rm16, imm16);
    push_src!(16, imm16);
    imul_dst_src1_src2!(16, r16, rm16, imm8);

    test_dst_src!(16, rm16, r16);
    xchg_dst_src!(16, r16, rm16);
    mov_dst_src!(16, rm16, r16);
    mov_dst_src!(16, r16, rm16);
    mov_dst_src!(16, rm16, sreg);
    lea_dst_src!(16, r16, m16);

    xchg_dst_src!(16, ax, opr16);

    fn cbw(exec: &mut exec::Exec) -> Result<(), EmuException> {
        let al = exec.ac.get_gpreg(GpReg8::AL)? as i8;
        exec.ac.set_gpreg(GpReg16::AX, al as u16)
    }

    fn cwd(exec: &mut exec::Exec) -> Result<(), EmuException> {
        let ax = exec.ac.get_gpreg(GpReg16::AX)? as i16;
        exec.ac.set_gpreg(GpReg16::DX, if ax < 0 { u16::MAX } else { 0 })
    }

    callf_abs!(16, ptr16, imm16);

    pushf!(16);
    popf!(16);

    mov_dst_src!(16, ax, moffs16);
    mov_dst_src!(16, moffs16, ax);
    movs_dst_src!(16, 8);
    movs_dst_src!(16, 16);
    cmps_src_dst!(16, 8);
    cmps_src_dst!(16, 16);

    test_dst_src!(16, ax, imm16);
    stos_dst_src!(16, 8);
    stos_dst_src!(16, 16);
    lods_dst_src!(16, 8);
    lods_dst_src!(16, 16);
    scas_src_dst!(16, 8);
    scas_src_dst!(16, 16);

    mov_dst_src!(16, opr16, imm16);

    ret!(16);

    mov_dst_src!(16, rm16, imm16);

    fn leave(exec: &mut exec::Exec) -> Result<(), EmuException> {
        let bp = exec.ac.get_gpreg(GpReg16::BP)?;
        exec.ac.set_gpreg(GpReg16::SP, bp)?;
        let new_bp = exec.ac.pop_u16()?;
        debug!("leave: sp <- 0x{:04x}, bp <- 0x{:04x}", bp, new_bp);
        exec.ac.set_gpreg(GpReg16::BP, new_bp)
    }

    retf!(16);

    iret!(16);

    in_reg_port!(16, ax, imm8);
    out_port_reg!(16, imm8, ax);
    call_rel!(16, imm16);
    jmp_rel!(16, imm16);
    jmpf_abs!(16, ptr16, imm16);
    in_reg_port!(16, ax, dx);
    out_port_reg!(16, dx, ax);

    jcc_rel!(16, o, imm16);
    jcc_rel!(16, b, imm16);
    jcc_rel!(16, z, imm16);
    jcc_rel!(16, be, imm16);
    jcc_rel!(16, s, imm16);
    jcc_rel!(16, p, imm16);
    jcc_rel!(16, l, imm16);
    jcc_rel!(16, le, imm16);

    imul_dst_src!(16, r16, rm16);

    movzx_dst_src!(16, r16, 8, rm8);
    movzx_dst_src!(16, r16, 16, rm16);
    movsx_dst_src!(16, r16, 8, rm8);
    movsx_dst_src!(16, r16, 16, rm16);

    fn code_81(exec: &mut exec::Exec) -> Result<(), EmuException> {
        match exec.idata.modrm.reg as u16 {
            0 => Opcode16::add_rm16_imm16(exec)?,
            1 => Opcode16::or_rm16_imm16(exec)?,
            2 => Opcode16::adc_rm16_imm16(exec)?,
            3 => Opcode16::sbb_rm16_imm16(exec)?,
            4 => Opcode16::and_rm16_imm16(exec)?,
            5 => Opcode16::sub_rm16_imm16(exec)?,
            6 => Opcode16::xor_rm16_imm16(exec)?,
            7 => Opcode16::cmp_rm16_imm16(exec)?,
            _ => { return Err(EmuException::UnexpectedError); },
        }
        Ok(())
    }

    add_dst_src!(16, rm16, imm16);
    or_dst_src!(16, rm16, imm16);
    adc_dst_src!(16, rm16, imm16);
    sbb_dst_src!(16, rm16, imm16);
    and_dst_src!(16, rm16, imm16);
    sub_dst_src!(16, rm16, imm16);
    xor_dst_src!(16, rm16, imm16);
    cmp_dst_src!(16, rm16, imm16);

    fn code_83(exec: &mut exec::Exec) -> Result<(), EmuException> {
        match exec.idata.modrm.reg as u16 {
            0 => Opcode16::add_rm16_imm8(exec)?,
            1 => Opcode16::or_rm16_imm8(exec)?,
            2 => Opcode16::adc_rm16_imm8(exec)?,
            3 => Opcode16::sbb_rm16_imm8(exec)?,
            4 => Opcode16::and_rm16_imm8(exec)?,
            5 => Opcode16::sub_rm16_imm8(exec)?,
            6 => Opcode16::xor_rm16_imm8(exec)?,
            7 => Opcode16::cmp_rm16_imm8(exec)?,
            _ => { return Err(EmuException::UnexpectedError); },
        }
        Ok(())
    }

    add_dst_src!(16, rm16, imm8);
    or_dst_src!(16, rm16, imm8);
    adc_dst_src!(16, rm16, imm8);
    sbb_dst_src!(16, rm16, imm8);
    and_dst_src!(16, rm16, imm8);
    sub_dst_src!(16, rm16, imm8);
    xor_dst_src!(16, rm16, imm8);
    cmp_dst_src!(16, rm16, imm8);

    fn code_0f01(exec: &mut exec::Exec) -> Result<(), EmuException> {
        match exec.idata.modrm.reg as u16 {
            2 => Opcode16::lgdt_m16_24(exec)?,
            3 => Opcode16::lidt_m16_24(exec)?,
            _ => { return Err(EmuException::NotImplementedOpcode); },
        }
        Ok(())
    }

    fn lgdt_m16_24(exec: &mut exec::Exec) -> Result<(), EmuException> {
        let (sg, adr) = exec.get_m()?;

        if exec.ac.get_cpl()? > 0 {
            return Err(EmuException::CPUException(CPUException::GP));
        }

        let limit = exec.ac.get_data16((sg,adr))?;
        let base  = exec.ac.get_data32((sg,adr+2))? & ((1<<24)-1);
        debug!("lgdt: base = {:04x}, limit = {:02x}", base, limit);
        exec.ac.set_gdtr(base as u64, limit)
    }

    fn lidt_m16_24(exec: &mut exec::Exec) -> Result<(), EmuException> {
        let (sg, adr) = exec.get_m()?;

        if exec.ac.get_cpl()? > 0 {
            return Err(EmuException::CPUException(CPUException::GP));
        }

        let limit = exec.ac.get_data16((sg,adr))?;
        let base  = exec.ac.get_data32((sg,adr+2))? & ((1<<24)-1);
        debug!("lidt: base = {:04x}, limit = {:02x}", base, limit);
        exec.ac.set_idtr(base as u64, limit)
    }
}
