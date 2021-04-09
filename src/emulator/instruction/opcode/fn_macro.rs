macro_rules! add_dst_src {
    ( $type:ty, $dst:ident, $src:ident ) => { paste::item! {
        fn [<add_ $dst _ $src>](exec: &mut exec::Exec) {
            let dst: $type = exec.[<get_ $dst>]();
            let src: $type = exec.[<get_ $src>]();

            debug!("add: {:02x}, {:02x}", dst, src);
            exec.[<set_ $dst>](dst.wrapping_add(src));
            exec.update_rflags_add(dst, src);
        }
    } };
}

macro_rules! or_dst_src {
    ( $type:ty, $dst:ident, $src:ident ) => { paste::item! {
        fn [<or_ $dst _ $src>](exec: &mut exec::Exec) {
            let dst: $type = exec.[<get_ $dst>]();
            let src: $type = exec.[<get_ $src>]();

            debug!("or: {:02x}, {:02x}", dst, src);
            exec.[<set_ $dst>](dst | src);
            exec.update_rflags_or(dst, src);
        }
    } };
}

macro_rules! adc_dst_src {
    ( $type:ty, $dst:ident, $src:ident ) => { paste::item! {
        fn [<adc_ $dst _ $src>](exec: &mut exec::Exec) {
            let dst: $type = exec.[<get_ $dst>]();
            let src: $type = exec.[<get_ $src>]();
            let cf:  $type = exec.ac.core.rflags.is_carry() as $type;

            debug!("adc: {:02x}, {:02x}", dst, src);
            exec.[<set_ $dst>](dst.wrapping_add(src).wrapping_add(cf));
            exec.update_rflags_adc(dst, src, cf);
        }
    } };
}

macro_rules! sbb_dst_src {
    ( $type:ty, $dst:ident, $src:ident ) => { paste::item! {
        fn [<sbb_ $dst _ $src>](exec: &mut exec::Exec) {
            let dst: $type = exec.[<get_ $dst>]();
            let src: $type = exec.[<get_ $src>]();
            let cf:  $type = exec.ac.core.rflags.is_carry() as $type;

            debug!("sbb: {:02x}, {:02x}", dst, src);
            exec.[<set_ $dst>](dst.wrapping_sub(src).wrapping_sub(cf));
            exec.update_rflags_sbb(dst, src, cf);
        }
    } };
}

macro_rules! and_dst_src {
    ( $type:ty, $dst:ident, $src:ident ) => { paste::item! {
        fn [<and_ $dst _ $src>](exec: &mut exec::Exec) {
            let dst: $type = exec.[<get_ $dst>]();
            let src: $type = exec.[<get_ $src>]();

            debug!("and: {:02x}, {:02x}", dst, src);
            exec.[<set_ $dst>](dst & src);
            exec.update_rflags_and(dst, src);
        }
    } };
}

macro_rules! sub_dst_src {
    ( $type:ty, $dst:ident, $src:ident ) => { paste::item! {
        fn [<sub_ $dst _ $src>](exec: &mut exec::Exec) {
            let dst: $type = exec.[<get_ $dst>]();
            let src: $type = exec.[<get_ $src>]();

            debug!("sub: {:02x}, {:02x}", dst, src);
            exec.[<set_ $dst>](dst.wrapping_sub(src));
            exec.update_rflags_sub(dst, src);
        }
    } };
}

macro_rules! xor_dst_src {
    ( $type:ty, $dst:ident, $src:ident ) => { paste::item! {
        fn [<xor_ $dst _ $src>](exec: &mut exec::Exec) {
            let dst: $type = exec.[<get_ $dst>]();
            let src: $type = exec.[<get_ $src>]();

            debug!("xor: {:02x}, {:02x}", dst, src);
            exec.[<set_ $dst>](dst ^ src);
            exec.update_rflags_xor(dst, src);
        }
    } };
}

macro_rules! cmp_dst_src {
    ( $type:ty, $dst:ident, $src:ident ) => { paste::item! {
        fn [<cmp_ $dst _ $src>](exec: &mut exec::Exec) {
            let dst: $type = exec.[<get_ $dst>]();
            let src: $type = exec.[<get_ $src>]();
            debug!("cmp: {:02x}, {:02x}", dst, src);
            exec.update_rflags_sub(dst, src);
        }
    } };
}

macro_rules! push_src {
    ( $type:ty, $src:ident ) => { paste::item! {
        fn [<push_ $src>](exec: &mut exec::Exec) {
            let v: $type = exec.[<get_ $src>]() as $type;
            debug!("push: {:02x}", v);
            exec.[<push_ $type>](v);
        }
    } };
}

macro_rules! pop_dst {
    ( $type:ty, $dst:ident ) => { paste::item! {
        fn [<pop_ $dst>](exec: &mut exec::Exec) {
            let v: $type = exec.[<pop_ $type>]() as $type;
            debug!("pop: {:02x}", v);
            exec.[<set_ $dst>](v);
        }
    } };
}
macro_rules! imul_dst_src1_src2 {
    ( $type:ty, $dst:ident, $src1:ident, $src2:ident ) => { paste::item! {
        fn [<imul_ $dst _ $src1 _ $src2>](exec: &mut exec::Exec) {
            let src1: $type = exec.[<get_ $src1>]();
            let src2: $type = exec.[<get_ $src2>]() as $type;
            debug!("imul: {:02x}, {:02x}", src1, src2);
            exec.[<set_ $dst>](src1.wrapping_mul(src2));
            exec.update_rflags_sub(src1, src2);
        }
    } };
}

macro_rules! jcc_rel {
    ( $type:tt, $cc:ident, $rel:ident ) => {
        paste::item! {
            fn [<j $cc _ $rel>](exec: &mut exec::Exec) {
                if(exec.[<check_rflags_ $cc>]()){
                    let rel: $type = exec.[<get_ $rel>]() as $type;
                    debug!("jmp: {}", rel);
                    exec.update_rip(rel as i64);
                }
            }

            fn [<jn $cc _ $rel>](exec: &mut exec::Exec) {
                if(!exec.[<check_rflags_ $cc>]()){
                    let rel: $type = exec.[<get_ $rel>]() as $type;
                    debug!("jmp: {}", rel);
                    exec.update_rip(rel as i64);
                }
            }
        }
    };
}

macro_rules! test_dst_src {
    ( $type:ty, $dst:ident, $src:ident ) => { paste::item! {
        fn [<test_ $dst _ $src>](exec: &mut exec::Exec) {
            let dst: $type = exec.[<get_ $dst>]();
            let src: $type = exec.[<get_ $src>]();
            debug!("test: {:02x}, {:02x}", dst, src);
            exec.update_rflags_and(dst, src);
        }
    } };
}

macro_rules! xchg_dst_src {
    ( $type:ty, $dst:ident, $src:ident ) => { paste::item! {
        fn [<xchg_ $dst _ $src>](exec: &mut exec::Exec) {
            let dst: $type = exec.[<get_ $dst>]();
            let src: $type = exec.[<get_ $src>]();

            debug!("xchg: {:02x}, {:02x}", dst, src);
            exec.[<set_ $dst>](src);
            exec.[<set_ $src>](dst);
        }
    } };
}

macro_rules! mov_dst_src {
    ( $type:ty, $dst:ident, $src:ident ) => { paste::item! {
        fn [<mov_ $dst _ $src>](exec: &mut exec::Exec) {
            let src: $type = exec.[<get_ $src>]();
            debug!("mov: {:02x}", src);
            exec.[<set_ $dst>](src);
        }
    } };
}

macro_rules! lea_dst_src {
    ( $type:ty, $dst:ident, $src:ident ) => { paste::item! {
        fn [<lea_ $dst _ $src>](exec: &mut exec::Exec) {
            let src: $type = exec.get_m() as $type;
            debug!("lea: {:02x}", src);
            exec.[<set_ $dst>](src);
        }
    } };
}

macro_rules! setcc_dst {
    ( $type:tt, $cc:ident, $dst:ident ) => {
        paste::item! {
            fn [<set $cc _ $dst>](exec: &mut exec::Exec) {
                let flag: bool = exec.[<check_rflags_ $cc>]();
                exec.[<set_ $dst>](flag as $type);
            }

            fn [<setn $cc _ $dst>](exec: &mut exec::Exec) {
                let flag: bool = exec.[<check_rflags_ $cc>]();
                exec.[<set_ $dst>](!flag as $type);
            }
        }
    };
}