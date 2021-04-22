macro_rules! add_dst_src {
    ( $type:ty, $dst:ident, $src:ident ) => { paste::item! {
        fn [<add_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let dst: $type = exec.[<get_ $dst>]()? as $type;
            let src: $type = exec.[<get_ $src>]()? as $type;

            debug!("add: {:02x}, {:02x}", dst, src);
            exec.update_rflags_add(dst, src)?;
            exec.[<set_ $dst>](dst.wrapping_add(src))
        }
    } };
}

macro_rules! or_dst_src {
    ( $type:ty, $dst:ident, $src:ident ) => { paste::item! {
        fn [<or_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let dst: $type = exec.[<get_ $dst>]()? as $type;
            let src: $type = exec.[<get_ $src>]()? as $type;

            debug!("or: {:02x}, {:02x}", dst, src);
            exec.update_rflags_or(dst, src)?;
            exec.[<set_ $dst>](dst | src)
        }
    } };
}

macro_rules! adc_dst_src {
    ( $type:ty, $dst:ident, $src:ident ) => { paste::item! {
        fn [<adc_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let dst: $type = exec.[<get_ $dst>]()? as $type;
            let src: $type = exec.[<get_ $src>]()? as $type;
            let cf:  $type = exec.check_rflags_b()? as $type;

            debug!("adc: {:02x}, {:02x}", dst, src);
            exec.update_rflags_adc(dst, src, cf)?;
            exec.[<set_ $dst>](dst.wrapping_add(src).wrapping_add(cf))
        }
    } };
}

macro_rules! sbb_dst_src {
    ( $type:ty, $dst:ident, $src:ident ) => { paste::item! {
        fn [<sbb_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let dst: $type = exec.[<get_ $dst>]()? as $type;
            let src: $type = exec.[<get_ $src>]()? as $type;
            let cf:  $type = exec.check_rflags_b()? as $type;

            debug!("sbb: {:02x}, {:02x}", dst, src);
            exec.update_rflags_sbb(dst, src, cf)?;
            exec.[<set_ $dst>](dst.wrapping_sub(src).wrapping_sub(cf))
        }
    } };
}

macro_rules! and_dst_src {
    ( $type:ty, $dst:ident, $src:ident ) => { paste::item! {
        fn [<and_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let dst: $type = exec.[<get_ $dst>]()? as $type;
            let src: $type = exec.[<get_ $src>]()? as $type;

            debug!("and: {:02x}, {:02x}", dst, src);
            exec.update_rflags_and(dst, src)?;
            exec.[<set_ $dst>](dst & src)
        }
    } };
}

macro_rules! sub_dst_src {
    ( $type:ty, $dst:ident, $src:ident ) => { paste::item! {
        fn [<sub_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let dst: $type = exec.[<get_ $dst>]()? as $type;
            let src: $type = exec.[<get_ $src>]()? as $type;

            debug!("sub: {:02x}, {:02x}", dst, src);
            exec.update_rflags_sub(dst, src)?;
            exec.[<set_ $dst>](dst.wrapping_sub(src))
        }
    } };
}

macro_rules! xor_dst_src {
    ( $type:ty, $dst:ident, $src:ident ) => { paste::item! {
        fn [<xor_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let dst: $type = exec.[<get_ $dst>]()? as $type;
            let src: $type = exec.[<get_ $src>]()? as $type;

            debug!("xor: {:02x}, {:02x}", dst, src);
            exec.update_rflags_xor(dst, src)?;
            exec.[<set_ $dst>](dst ^ src)
        }
    } };
}

macro_rules! cmp_dst_src {
    ( $type:ty, $dst:ident, $src:ident ) => { paste::item! {
        fn [<cmp_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let dst: $type = exec.[<get_ $dst>]()? as $type;
            let src: $type = exec.[<get_ $src>]()? as $type;
            debug!("cmp: {:02x}, {:02x}", dst, src);
            exec.update_rflags_sub(dst, src)
        }
    } };
}

macro_rules! push_src {
    ( $type:ty, $src:ident ) => { paste::item! {
        fn [<push_ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let v: $type = exec.[<get_ $src>]()? as $type;
            debug!("push: {:02x}", v);
            exec.[<push_ $type>](v)
        }
    } };
}

macro_rules! pop_dst {
    ( $type:ty, $dst:ident ) => { paste::item! {
        fn [<pop_ $dst>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let v: $type = exec.[<pop_ $type>]()? as $type;
            debug!("pop: {:02x}", v);
            exec.[<set_ $dst>](v)
        }
    } };
}

macro_rules! imul_dst_src1_src2 {
    ( $type:ty, $dst:ident, $src1:ident, $src2:ident ) => { paste::item! {
        fn [<imul_ $dst _ $src1 _ $src2>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let src1: $type = exec.[<get_ $src1>]()? as $type;
            let src2: $type = exec.[<get_ $src2>]()? as $type;
            debug!("imul: {:02x}, {:02x}", src1, src2);
            exec.update_rflags_mul(src1, src2)?;
            exec.[<set_ $dst>](src1.wrapping_mul(src2))
        }
    } };
}

macro_rules! jcc_rel {
    ( $type:ty, $cc:ident, $rel:ident ) => { paste::item! {
        fn [<j $cc _ $rel>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            if(exec.[<check_rflags_ $cc>]()?){
                let rel: $type = exec.[<get_ $rel>]()? as $type;
                debug!("jmp: {}", rel);
                exec.ac.update_ip(rel as i64)?;
            }
            Ok(())
        }

        fn [<jn $cc _ $rel>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            if(!exec.[<check_rflags_ $cc>]()?){
                let rel: $type = exec.[<get_ $rel>]()? as $type;
                debug!("jmp: {}", rel);
                exec.ac.update_ip(rel as i64)?;
            }
            Ok(())
        }
    } };
}

macro_rules! test_dst_src {
    ( $type:ty, $dst:ident, $src:ident ) => { paste::item! {
        fn [<test_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let dst: $type = exec.[<get_ $dst>]()? as $type;
            let src: $type = exec.[<get_ $src>]()? as $type;
            debug!("test: {:02x}, {:02x}", dst, src);
            exec.update_rflags_and(dst, src)
        }
    } };
}

macro_rules! xchg_dst_src {
    ( $type:ty, $dst:ident, $src:ident ) => { paste::item! {
        fn [<xchg_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let dst: $type = exec.[<get_ $dst>]()? as $type;
            let src: $type = exec.[<get_ $src>]()? as $type;

            debug!("xchg: {:02x}, {:02x}", dst, src);
            exec.[<set_ $dst>](src)?;
            exec.[<set_ $src>](dst)?;
            Ok(())
        }
    } };
}

macro_rules! mov_dst_src {
    ( $type:ty, $dst:ident, $src:ident ) => { paste::item! {
        fn [<mov_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let src: $type = exec.[<get_ $src>]()? as $type;
            debug!("mov: {:02x}", src);
            exec.[<set_ $dst>](src)
        }
    } };
}

macro_rules! lea_dst_src {
    ( $type:ty, $dst:ident, $src:ident ) => { paste::item! {
        fn [<lea_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let src: $type = exec.get_m()? as $type;
            debug!("lea: {:02x}", src);
            exec.[<set_ $dst>](src)
        }
    } };
}

macro_rules! callf_abs {
    ( $type:ty, $sel:ident, $abs:ident ) => { paste::item! {
        fn [<callf_ $sel _ $abs>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let sel: u16   = exec.[<get_ $sel>]()?;
            let abs: $type = exec.[<get_ $abs>]()? as $type;
            debug!("callf: {:04x}:{:04x}", sel, abs);
            exec.[<callf_ $type>](sel, abs)
        }
    } };
}

macro_rules! pushf {
    ( $type:ty ) => { paste::item! {
        fn pushf(exec: &mut exec::Exec) -> Result<(), EmuException> {
            let flag = exec.ac.get_rflags()? as $type;
            debug!("pushf: {:08x}", flag);
            exec.[<push_ $type>](flag)
        }
    } };
}

macro_rules! popf {
    ( $type:ty ) => { paste::item! {
        fn popf(exec: &mut exec::Exec) -> Result<(), EmuException> {
            let flag = exec.[<pop_ $type>]()?;
            debug!("popf: {:08x}", flag);
            exec.ac.set_rflags(flag as u64)
        }
    } };
}

macro_rules! ret {
    ( $type:ty ) => { paste::item! {
        fn ret(exec: &mut exec::Exec) -> Result<(), EmuException> {
            let ret: $type = exec.[<pop_ $type>]()? as $type;
            debug!("ret: {:04x}", ret);
            exec.ac.set_ip(ret)
        }
    } };
}

macro_rules! jmp_rel {
    ( $type:ty, $rel:ident ) => { paste::item! {
        fn [<jmp_ $rel>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let rel: $type = exec.[<get_ $rel>]()? as $type;
            debug!("jmp: {:04x}", rel);
            exec.ac.update_ip(rel as i64)
        }
    } };
}

macro_rules! jmpf_abs {
    ( $type:ty, $sel:ident, $abs:ident ) => { paste::item! {
        fn [<jmpf_ $sel _ $abs>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let sel: u16   = exec.[<get_ $sel>]()?;
            let abs: $type = exec.[<get_ $abs>]()? as $type;
            debug!("jmpf: {:04x}:{:04x}", sel, abs);
            exec.[<jmpf_ $type>](sel, abs)
        }
    } };
}

macro_rules! setcc_dst {
    ( $type:ty, $cc:ident, $dst:ident ) => { paste::item! {
        fn [<set $cc _ $dst>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let flag: bool = exec.[<check_rflags_ $cc>]()?;
            exec.[<set_ $dst>](flag as $type)
        }

        fn [<setn $cc _ $dst>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let flag: bool = exec.[<check_rflags_ $cc>]()?;
            exec.[<set_ $dst>](!flag as $type)
        }
    } };
}

macro_rules! imul_dst_src {
    ( $type:ty, $dst:ident, $src:ident ) => { paste::item! {
        fn [<imul_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let dst: $type = exec.[<get_ $dst>]()? as $type;
            let src: $type = exec.[<get_ $src>]()? as $type;
            debug!("imul: {:02x}, {:02x}", dst, src);
            exec.update_rflags_mul(dst, src)?;
            exec.[<set_ $dst>](dst.wrapping_mul(src))
        }
    } };
}

macro_rules! movx_dst_src {
    ( $zs:ident, $dtype:ty, $dst:ident, $stype:ty, $src:ident ) => { paste::item! {
        fn [<mov $zs x_ $dst _ $src>](exec: &mut exec::Exec) -> Result<(), EmuException> {
            let src: $stype = exec.[<get_ $src>]()? as $stype;
            debug!("movzsx: {:02x}", src);
            exec.[<set_ $dst>](src as $dtype)
        }
    } };
}