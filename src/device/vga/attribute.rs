use packed_struct::prelude::*;

#[derive(Debug, Default)]
pub(super) struct Attribute {
    pub port_data: bool,
    pub air:  AttrCtrlIndex,
    pr:   [Palette; 0x10],
    mcr:  ModeCtrl,
    mper: MemPlaneEnable,
    hppr: HorPixelPan,
    csr:  ColorSel,
}

impl Attribute {
    pub fn get(&self) -> u8 {
        match self.air.idx {
            i @ 0x00..=0x0f => self.pr[i as usize].pack().unwrap()[0],
            0x10 => self.mcr.pack().unwrap()[0],
            0x12 => self.mper.pack().unwrap()[0],
            0x13 => self.hppr.pack().unwrap()[0],
            0x14 => self.csr.pack().unwrap()[0],
            _ => 0,
        }
    }

    pub fn set(&mut self, v: u8) -> () {
        let data = &[v];
        match self.air.idx {
            i @ 0x00..=0x0f => self.pr[i as usize] = Palette::unpack(data).unwrap(),
            0x10 => self.mcr  = ModeCtrl::unpack(data).unwrap(),
            0x12 => self.mper = MemPlaneEnable::unpack(data).unwrap(),
            0x13 => self.hppr = HorPixelPan::unpack(data).unwrap(),
            0x14 => self.csr  = ColorSel::unpack(data).unwrap(),
            _ => {},
        }
    }

    pub fn dac_index(&self, idx: u8) -> u8 {
        let plt = &self.pr[(idx&0xf) as usize];

        let mut dac_idx = plt.p03;
        dac_idx += (if self.mcr.p45_sel { self.csr.alt_p45 } else { plt.p45 }) << 4;
        dac_idx += self.csr.p67 << 6;

        dac_idx
    }

    pub fn text_blink(&self) -> bool {
        !self.mcr.graph_text & self.mcr.ebsb_sel
    }
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct AttrCtrlIndex {
    #[packed_field(bits="0:4")] idx: u8,
    #[packed_field(bits="5")]   video_ena: u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct Palette {
    #[packed_field(bits="0:3")] p03: u8,
    #[packed_field(bits="4:5")] p45: u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct ModeCtrl {
    #[packed_field(bits="0")]   graph_text: bool,
    #[packed_field(bits="1")]   disp_sel:   u8,
    #[packed_field(bits="2")]   lgcc_ena:   u8,
    #[packed_field(bits="3")]   ebsb_sel:   bool,
    #[packed_field(bits="5")]   pixel_cmpt: u8,
    #[packed_field(bits="6")]   pwc_sel:    u8,
    #[packed_field(bits="7")]   p45_sel:    bool,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct MemPlaneEnable {
    #[packed_field(bits="0")]   pl0:   u8,
    #[packed_field(bits="1")]   pl1:   u8,
    #[packed_field(bits="2")]   pl2:   u8,
    #[packed_field(bits="3")]   pl3:   u8,
    #[packed_field(bits="4:5")] vstat: u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct HorPixelPan {
    #[packed_field(bits="0:3")] shift: u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct ColorSel {
    #[packed_field(bits="0:1")] alt_p45: u8,
    #[packed_field(bits="2:3")] p67: u8,
}