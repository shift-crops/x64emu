use packed_struct::prelude::*;

#[derive(Debug, Default)]
pub(super) struct General {
    pub st0: InputStat0,
    pub st1: InputStat1,
    pub fsr: FeatureCtrl,
    pub msr: MiscOutput,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct InputStat0 {
    #[packed_field(bits="4")]   rgb_cmp: u8,
    #[packed_field(bits="7")]   crt_int: u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct InputStat1 {
    #[packed_field(bits="0")]   disp_ena: u8,
    #[packed_field(bits="3")]   vrt_retr: u8,
    #[packed_field(bits="4:5")] video_fb: u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct FeatureCtrl {
    #[packed_field(bits="3")]   vs_ctrl: u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct MiscOutput {
    #[packed_field(bits="0")]   io_sel:  u8,
    #[packed_field(bits="1")]   mem_ena: u8,
    #[packed_field(bits="2:3")] clk_sel: u8,
    #[packed_field(bits="5")]   pg_sel:  u8,
    #[packed_field(bits="6")]   hs_pol:  u8,
    #[packed_field(bits="7")]   vs_pol:  u8,
}
