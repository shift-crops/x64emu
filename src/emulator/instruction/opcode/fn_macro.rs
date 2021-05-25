macro_rules! i { ($size:expr) => { paste::item! { [<i $size>] } }}
macro_rules! u { ($size:expr) => { paste::item! { [<u $size>] } }}

macro_rules! add_dst_src {
    ( $size:expr, $dst:ident, $src:ident ) => { paste::item! {
        fn [<add_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let dst = exec.[<get_ $dst>]()? as u!($size);
            let src = exec.[<get_ $src>]()? as u!($size);

            debug!("add: {:02x}, {:02x}", dst, src);
            exec.update_rflags_add(dst, src)?;
            exec.[<set_ $dst>](dst.wrapping_add(src))
        }
    } };
}

macro_rules! or_dst_src {
    ( $size:expr, $dst:ident, $src:ident ) => { paste::item! {
        fn [<or_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let dst = exec.[<get_ $dst>]()? as u!($size);
            let src = exec.[<get_ $src>]()? as u!($size);

            debug!("or: {:02x}, {:02x}", dst, src);
            exec.update_rflags_or(dst, src)?;
            exec.[<set_ $dst>](dst | src)
        }
    } };
}

macro_rules! adc_dst_src {
    ( $size:expr, $dst:ident, $src:ident ) => { paste::item! {
        fn [<adc_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let dst = exec.[<get_ $dst>]()? as u!($size);
            let src = exec.[<get_ $src>]()? as u!($size);
            let cf  = exec.check_rflags_b()? as u!($size);

            debug!("adc: {:02x}, {:02x}", dst, src);
            exec.update_rflags_adc(dst, src, cf)?;
            exec.[<set_ $dst>](dst.wrapping_add(src).wrapping_add(cf))
        }
    } };
}

macro_rules! sbb_dst_src {
    ( $size:expr, $dst:ident, $src:ident ) => { paste::item! {
        fn [<sbb_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let dst = exec.[<get_ $dst>]()? as u!($size);
            let src = exec.[<get_ $src>]()? as u!($size);
            let cf  = exec.check_rflags_b()? as u!($size);

            debug!("sbb: {:02x}, {:02x}", dst, src);
            exec.update_rflags_sbb(dst, src, cf)?;
            exec.[<set_ $dst>](dst.wrapping_sub(src).wrapping_sub(cf))
        }
    } };
}

macro_rules! and_dst_src {
    ( $size:expr, $dst:ident, $src:ident ) => { paste::item! {
        fn [<and_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let dst = exec.[<get_ $dst>]()? as u!($size);
            let src = exec.[<get_ $src>]()? as u!($size);

            debug!("and: {:02x}, {:02x}", dst, src);
            exec.update_rflags_and(dst, src)?;
            exec.[<set_ $dst>](dst & src)
        }
    } };
}

macro_rules! sub_dst_src {
    ( $size:expr, $dst:ident, $src:ident ) => { paste::item! {
        fn [<sub_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let dst = exec.[<get_ $dst>]()? as u!($size);
            let src = exec.[<get_ $src>]()? as u!($size);

            debug!("sub: {:02x}, {:02x}", dst, src);
            exec.update_rflags_sub(dst, src)?;
            exec.[<set_ $dst>](dst.wrapping_sub(src))
        }
    } };
}

macro_rules! xor_dst_src {
    ( $size:expr, $dst:ident, $src:ident ) => { paste::item! {
        fn [<xor_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let dst = exec.[<get_ $dst>]()? as u!($size);
            let src = exec.[<get_ $src>]()? as u!($size);

            debug!("xor: {:02x}, {:02x}", dst, src);
            exec.update_rflags_xor(dst, src)?;
            exec.[<set_ $dst>](dst ^ src)
        }
    } };
}

macro_rules! cmp_dst_src {
    ( $size:expr, $dst:ident, $src:ident ) => { paste::item! {
        fn [<cmp_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let dst = exec.[<get_ $dst>]()? as u!($size);
            let src = exec.[<get_ $src>]()? as u!($size);
            debug!("cmp: {:02x}, {:02x}", dst, src);
            exec.update_rflags_sub(dst, src)
        }
    } };
}

macro_rules! inc_dst {
    ( $dst:ident ) => { paste::item! {
        fn [<inc_ $dst>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let v = exec.[<get_ $dst>]()?;
            exec.[<set_ $dst>](v.wrapping_add(1))
        }
    } };
}

macro_rules! dec_dst {
    ( $dst:ident ) => { paste::item! {
        fn [<dec_ $dst>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let v = exec.[<get_ $dst>]()?;
            exec.[<set_ $dst>](v.wrapping_sub(1))
        }
    } };
}

macro_rules! push_src {
    ( $size:expr, $src:ident ) => { paste::item! {
        fn [<push_ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let v = exec.[<get_ $src>]()? as u!($size);
            debug!("push: {:02x}", v);
            exec.ac.[<push_u $size>](v)
        }
    } };
}

macro_rules! pop_dst {
    ( $size:expr, $dst:ident ) => { paste::item! {
        fn [<pop_ $dst>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let v = exec.ac.[<pop_u $size>]()? as u!($size);
            debug!("pop: {:02x}", v);
            exec.[<set_ $dst>](v)
        }
    } };
}

macro_rules! imul_dst_src1_src2 {
    ( $size:expr, $dst:ident, $src1:ident, $src2:ident ) => { paste::item! {
        fn [<imul_ $dst _ $src1 _ $src2>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let src1 = exec.[<get_ $src1>]()? as i!($size);
            let src2 = exec.[<get_ $src2>]()? as i!($size);
            debug!("imul: {:02x}, {:02x}", src1, src2);
            exec.update_rflags_imul(src1, src2)?;
            exec.[<set_ $dst>](src1.wrapping_mul(src2) as u!($size))
        }
    } };
}

macro_rules! jcc_rel {
    ( $size:expr, $cc:ident, $rel:ident ) => { paste::item! {
        fn [<j $cc _ $rel>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            if(exec.[<check_rflags_ $cc>]()?){
                let rel = exec.[<get_ $rel>]()? as i!($size);
                debug!("jmp: {}", rel);
                exec.ac.update_ip(rel as i64)?;
            }
            Ok(())
        }

        fn [<jn $cc _ $rel>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            if(!exec.[<check_rflags_ $cc>]()?){
                let rel = exec.[<get_ $rel>]()? as i!($size);
                debug!("jmp: {}", rel);
                exec.ac.update_ip(rel as i64)?;
            }
            Ok(())
        }
    } };
}

macro_rules! test_dst_src {
    ( $size:expr, $dst:ident, $src:ident ) => { paste::item! {
        fn [<test_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let dst = exec.[<get_ $dst>]()? as u!($size);
            let src = exec.[<get_ $src>]()? as u!($size);
            debug!("test: {:02x}, {:02x}", dst, src);
            exec.update_rflags_and(dst, src)
        }
    } };
}

macro_rules! xchg_dst_src {
    ( $size:expr, $dst:ident, $src:ident ) => { paste::item! {
        fn [<xchg_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let dst = exec.[<get_ $dst>]()? as u!($size);
            let src = exec.[<get_ $src>]()? as u!($size);

            debug!("xchg: {:02x}, {:02x}", dst, src);
            exec.[<set_ $dst>](src)?;
            exec.[<set_ $src>](dst)?;
            Ok(())
        }
    } };
}

macro_rules! mov_dst_src {
    ( $size:expr, $dst:ident, $src:ident ) => { paste::item! {
        fn [<mov_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let src = exec.[<get_ $src>]()? as u!($size);
            debug!("mov: {:02x}", src);
            exec.[<set_ $dst>](src)
        }
    } };
}

macro_rules! lea_dst_src {
    ( $size:expr, $dst:ident, $src:ident ) => { paste::item! {
        fn [<lea_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let src = exec.get_m()?.1 as u!($size);
            debug!("lea: {:02x}", src);
            exec.[<set_ $dst>](src)
        }
    } };
}

macro_rules! callf_abs {
    ( $size:expr, $sel:ident, $abs:ident ) => { paste::item! {
        fn [<callf_ $sel _ $abs>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let sel: u16 = exec.[<get_ $sel>]()?;
            let abs = exec.[<get_ $abs>]()? as u!($size);
            debug!("callf: {:04x}:{:04x}", sel, abs);
            exec.[<call_far_u $size>](sel, abs)
        }
    } };
}

macro_rules! pushf {
    ( $size:expr ) => { paste::item! {
        fn pushf(exec: &mut exec::Exec) -> Result<(), EmuException> {
            let flag = exec.ac.get_rflags()? as u!($size);
            debug!("pushf: {:08x}", flag);
            exec.ac.[<push_u $size>](flag)
        }
    } };
}

macro_rules! popf {
    ( $size:expr ) => { paste::item! {
        fn popf(exec: &mut exec::Exec) -> Result<(), EmuException> {
            let flag = exec.ac.[<pop_u $size>]()?;
            debug!("popf: {:08x}", flag);
            exec.ac.set_rflags(flag as u64)
        }
    } };
}

macro_rules! movs_dst_src {
    ( $opsize:expr, $adsize:expr ) => { paste::item! {
        fn [<movs_m $adsize>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            exec.[<move_str $adsize>]()?;
            exec.[<repeat_ $opsize>]()
        }
    } };
}

macro_rules! cmps_src_dst {
    ( $opsize:expr, $adsize:expr ) => { paste::item! {
        fn [<cmps_m $adsize>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            exec.[<cmp_str $adsize>]()?;
            exec.[<repeat_ $opsize>]()
        }
    } };
}

macro_rules! stos_dst_src {
    ( $opsize:expr, $adsize:expr ) => { paste::item! {
        fn [<stos_m $adsize>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            exec.[<store_str $adsize>]()?;
            exec.[<repeat_ $opsize>]()
        }
    } };
}

macro_rules! lods_dst_src {
    ( $opsize:expr, $adsize:expr ) => { paste::item! {
        fn [<lods_m $adsize>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            exec.[<load_str $adsize>]()?;
            exec.[<repeat_ $opsize>]()
        }
    } };
}

macro_rules! scas_src_dst {
    ( $opsize:expr, $adsize:expr ) => { paste::item! {
        fn [<scas_m $adsize>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            exec.[<scan_str $adsize>]()?;
            exec.[<repeat_ $opsize>]()
        }
    } };
}

macro_rules! ret {
    ( $size:expr ) => { paste::item! {
        fn ret(exec: &mut exec::Exec) -> Result<(), EmuException> {
            let ret = exec.ac.[<pop_u $size>]()? as u64;
            debug!("ret: {:04x}", ret);
            exec.ac.set_ip(ret)
        }
    } };
}

macro_rules! retf {
    ( $size:expr ) => { paste::item! {
        fn retf(exec: &mut exec::Exec) -> Result<(), EmuException> {
            exec.[<ret_far_u $size>]()
        }
    } };
}

macro_rules! iret {
    ( $size:expr ) => { paste::item! {
        fn iret(exec: &mut exec::Exec) -> Result<(), EmuException> {
            exec.[<int_ret_u $size>]()
        }
    } };
}

macro_rules! in_reg_port {
    ( $size:expr, $reg:ident, $port:ident ) => { paste::item! {
        fn [<in_ $reg _ $port>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let port = exec.[<get_ $port>]()? as u16;
            let v = exec.ac.[<in_ $size>](port)?;
            debug!("in: {:04x}", port);
            exec.[<set_ $reg>](v)
        }
    } };
}

macro_rules! out_port_reg {
    ( $size:expr, $port:ident, $reg:ident ) => { paste::item! {
        fn [<out_ $port _ $reg>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let port = exec.[<get_ $port>]()? as u16;
            let v = exec.[<get_ $reg>]()?;
            debug!("out: {:04x}", port);
            exec.ac.[<out_ $size>](port, v)
        }
    } };
}

macro_rules! call_rel {
    ( $size:expr, $rel:ident ) => { paste::item! {
        fn [<call_ $rel>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let offs = exec.[<get_ $rel>]()? as i64;
            let rip = exec.ac.get_ip()?;
            debug!("call: 0x{:04x}", rip as i64 + offs);
            exec.ac.[<push_u $size>](rip as u!($size))?;
            exec.ac.update_ip(offs)
        }
    } };
}

macro_rules! jmp_rel {
    ( $size:expr, $rel:ident ) => { paste::item! {
        fn [<jmp_ $rel>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let rel = exec.[<get_ $rel>]()? as i!($size);
            debug!("jmp: {:04x}", rel);
            exec.ac.update_ip(rel as i64)
        }
    } };
}

macro_rules! jmpf_abs {
    ( $size:expr, $sel:ident, $abs:ident ) => { paste::item! {
        fn [<jmpf_ $sel _ $abs>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let sel: u16 = exec.[<get_ $sel>]()?;
            let abs = exec.[<get_ $abs>]()? as u!($size);
            debug!("jmpf: {:04x}:{:04x}", sel, abs);
            exec.[<jmp_far_u $size>](sel, abs)
        }
    } };
}

macro_rules! setcc_dst {
    ( $size:expr, $cc:ident, $dst:ident ) => { paste::item! {
        fn [<set $cc _ $dst>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let flag: bool = exec.[<check_rflags_ $cc>]()?;
            exec.[<set_ $dst>](flag as u!($size))
        }

        fn [<setn $cc _ $dst>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let flag: bool = exec.[<check_rflags_ $cc>]()?;
            exec.[<set_ $dst>](!flag as u!($size))
        }
    } };
}

macro_rules! imul_dst_src {
    ( $size:expr, $dst:ident, $src:ident ) => { paste::item! {
        fn [<imul_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let dst = exec.[<get_ $dst>]()? as u!($size);
            let src = exec.[<get_ $src>]()? as u!($size);
            debug!("imul: {:02x}, {:02x}", dst, src);
            exec.update_rflags_mul(dst, src)?;
            exec.[<set_ $dst>](dst.wrapping_mul(src))
        }
    } };
}

macro_rules! movzx_dst_src {
    ( $dsize:expr, $dst:ident, $ssize:expr, $src:ident ) => { paste::item! {
        fn [<movzx_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let src: u!($ssize) = exec.[<get_ $src>]()? as u!($ssize);
            debug!("movzx: {:02x}", src);
            exec.[<set_ $dst>](src as u!($dsize))
        }
    } };
}

macro_rules! movsx_dst_src {
    ( $dsize:expr, $dst:ident, $ssize:expr, $src:ident ) => { paste::item! {
        fn [<movsx_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let src: i!($ssize) = exec.[<get_ $src>]()? as i!($ssize);
            debug!("movzsx: {:02x}", src);
            exec.[<set_ $dst>](src as u!($dsize))
        }
    } };
}

macro_rules! shl_dst_src {
    ( $size:expr, $dst:ident, $src:ident ) => { paste::item! {
        fn [<shl_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let dst = exec.[<get_ $dst>]()? as u!($size);
            let src: u32 = exec.[<get_ $src>]()? as u32;
            debug!("shl: {:02x}, {:02x}", dst, src);
            exec.update_rflags_shl(dst, src)?;
            exec.[<set_ $dst>](dst.wrapping_shl(src))
        }
    } };
}

macro_rules! shr_dst_src {
    ( $size:expr, $dst:ident, $src:ident ) => { paste::item! {
        fn [<shr_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let dst = exec.[<get_ $dst>]()? as u!($size);
            let src: u32 = exec.[<get_ $src>]()? as u32;
            debug!("shr: {:02x}, {:02x}", dst, src);
            exec.update_rflags_shr(dst, src)?;
            exec.[<set_ $dst>](dst.wrapping_shr(src))
        }
    } };
}

macro_rules! sal_dst_src {
    ( $size:expr, $dst:ident, $src:ident ) => { paste::item! {
        fn [<sal_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let dst = exec.[<get_ $dst>]()? as i!($size);
            let src: u32 = exec.[<get_ $src>]()? as u32;
            debug!("sal: {:02x}, {:02x}", dst, src);
            exec.update_rflags_shl(dst as u!($size), src)?;
            exec.[<set_ $dst>](dst.wrapping_shl(src) as u!($size))
        }
    } };
}

macro_rules! sar_dst_src {
    ( $size:expr, $dst:ident, $src:ident ) => { paste::item! {
        fn [<sar_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let dst = exec.[<get_ $dst>]()? as i!($size);
            let src: u32 = exec.[<get_ $src>]()? as u32;
            debug!("sar: {:02x}, {:02x}", dst, src);
            exec.update_rflags_sar(dst, src)?;
            exec.[<set_ $dst>](dst.wrapping_shr(src) as u!($size))
        }
    } };
}

macro_rules! not_dst {
    ( $size:expr, $dst:ident ) => { paste::item! {
        fn [<not_ $dst>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let v = exec.[<get_ $dst>]()? as u!($size);
            debug!("not: {:02x}", v);
            exec.[<set_ $dst>](!v)
        }
    } };
}

macro_rules! neg_dst {
    ( $size:expr, $dst:ident ) => { paste::item! {
        fn [<neg_ $dst>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let v = exec.[<get_ $dst>]()? as u!($size);
            let z = 0 as i!($size);
            debug!("neg: {:02x}", v);
            exec.update_rflags_sub(0, v)?;
            exec.[<set_ $dst>](z.wrapping_sub(v as i!($size)) as u!($size))
        }
    } };
}

macro_rules! mul_high_low_src {
    ( $size:expr, $high:ident, $low:ident, $src:ident ) => { paste::item! {
        fn [<mul_ $high _ $low _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let src1h = exec.[<get_ $high>]()? as u!($size);
            let src1l = exec.[<get_ $low>]()? as u!($size);
            let src1 = ((src1h as u128) << $size) + src1l as u128;
            let src2 = exec.[<get_ $src>]()? as u!($size);

            debug!("imul: {:02x}, {:02x}", src1, src2);
            exec.update_rflags_mul(src1h, src2)?;
            let v = src1.wrapping_mul(src2 as u128);
            exec.[<set_ $high>]((v >> $size) as u!($size))?;
            exec.[<set_ $low>](v  as u!($size))?;
            Ok(())
        }
    } };
}

macro_rules! imul_high_low_src {
    ( $size:expr, $high:ident, $low:ident, $src:ident ) => { paste::item! {
        fn [<imul_ $high _ $low _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let src1h = exec.[<get_ $high>]()? as i!($size);
            let src1l = exec.[<get_ $low>]()? as i!($size);
            let src1 = ((src1h as i128) << $size) + src1l as i128;
            let src2 = exec.[<get_ $src>]()? as i!($size);

            debug!("imul: {:02x}, {:02x}", src1, src2);
            exec.update_rflags_imul(src1h, src2)?;
            let v = src1.wrapping_mul(src2 as i128);
            exec.[<set_ $high>]((v >> $size) as u!($size))?;
            exec.[<set_ $low>](v  as u!($size))?;
            Ok(())
        }
    } };
}

macro_rules! div_quot_rem_src {
    ( $size:expr, $quot:ident, $rem:ident, $src:ident ) => { paste::item! {
        fn [<div_ $quot _ $rem _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let src1 = ((exec.[<get_ $rem>]()? as u128) << $size) + exec.[<get_ $quot>]()? as u128;
            let src2 = exec.[<get_ $src>]()? as u128;
            if src2 == 0 { return Err(EmuException::CPUException(CPUException::DE)); }

            debug!("div: {:02x}, {:02x}", src1, src2);
            exec.[<set_ $quot>](src1.wrapping_div(src2) as u!($size))?;
            exec.[<set_ $rem>]((src1 % src2) as u!($size))?;
            Ok(())
        }
    } };
}

macro_rules! idiv_quot_rem_src {
    ( $size:expr, $quot:ident, $rem:ident, $src:ident ) => { paste::item! {
        fn [<idiv_ $quot _ $rem _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let src1 = ((exec.[<get_ $rem>]()? as i128) << $size) + exec.[<get_ $quot>]()? as i128;
            let src2 = exec.[<get_ $src>]()? as i128;
            if src2 == 0 { return Err(EmuException::CPUException(CPUException::DE)); }

            debug!("idiv: {:02x}, {:02x}", src1, src2);
            exec.[<set_ $quot>](src1.wrapping_div(src2) as u!($size))?;
            exec.[<set_ $rem>]((src1 % src2) as u!($size))?;
            Ok(())
        }
    } };
}