use packed_struct::prelude::*;

#[derive(Debug, Default)]
pub(super) struct GraphicCtrl {
    pub gcir: GraphCtrlIndex,
    pub spr:  SetPlane,
    pub espr: EnableSetPlane,
    pub ccr:  ColorCmp,
    pub drr:  DataRot,
    pub rpsr: ReadPlaneSel,
    pub gmr:  GraphMode,
    pub mr:   Misc,
    pub icr:  IgnoreColor,
    pub bmr:  u8,
    pub amr:  AddrMap,
    pub psr:  u8,
    pub flag: u8,
}

impl GraphicCtrl {
    pub fn read(&self) -> u8 {
        let data = match self.gcir.idx {
            0x00 => self.spr.pack().unwrap(),
            0x01 => self.espr.pack().unwrap(),
            0x02 => self.ccr.pack().unwrap(),
            0x03 => self.drr.pack().unwrap(),
            0x04 => self.rpsr.pack().unwrap(),
            0x05 => self.gmr.pack().unwrap(),
            0x06 => self.mr.pack().unwrap(),
            0x07 => self.icr.pack().unwrap(),
            0x08 => [self.bmr],
            0x10 => self.amr.pack().unwrap(),
            0x11 => [self.psr],
            0x18 => [self.flag],
            _ => [0],
        };
        u8::from_be_bytes(data)
    }

    pub fn write(&mut self, v: u8) -> () {
        let data = &v.to_be_bytes();
        match self.gcir.idx {
            0x00 => self.spr  = SetPlane::unpack(data).unwrap(),
            0x01 => self.espr = EnableSetPlane::unpack(data).unwrap(),
            0x02 => self.ccr  = ColorCmp::unpack(data).unwrap(),
            0x03 => self.drr  = DataRot::unpack(data).unwrap(),
            0x04 => self.rpsr = ReadPlaneSel::unpack(data).unwrap(),
            0x05 => self.gmr  = GraphMode::unpack(data).unwrap(),
            0x06 => self.mr   = Misc::unpack(data).unwrap(),
            0x07 => self.icr  = IgnoreColor::unpack(data).unwrap(),
            0x08 => self.bmr  = v,
            0x10 => self.amr  = AddrMap::unpack(data).unwrap(),
            0x11 => self.psr  = v,
            0x18 => self.flag = v,
            _ => {},
        }
    }
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct GraphCtrlIndex {
    #[packed_field(bits="0:4")] idx: u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct SetPlane {
    #[packed_field(bits="0")]   pl0:  u8,
    #[packed_field(bits="1")]   pl1:  u8,
    #[packed_field(bits="2")]   pl2:  u8,
    #[packed_field(bits="3")]   pl3:  u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct EnableSetPlane {
    #[packed_field(bits="0")]   pl0:  u8,
    #[packed_field(bits="1")]   pl1:  u8,
    #[packed_field(bits="2")]   pl2:  u8,
    #[packed_field(bits="3")]   pl3:  u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct ColorCmp {
    #[packed_field(bits="0")]   pl0:  u8,
    #[packed_field(bits="1")]   pl1:  u8,
    #[packed_field(bits="2")]   pl2:  u8,
    #[packed_field(bits="3")]   pl3:  u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct DataRot {
    #[packed_field(bits="0:2")] rot_count: u8,
    #[packed_field(bits="3:4")] func_sel:  u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct ReadPlaneSel {
    #[packed_field(bits="0:1")] sel: u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct GraphMode {
    #[packed_field(bits="0:1")] write:    u8,
    #[packed_field(bits="3")]   read:     u8,
    #[packed_field(bits="4")]   odd_even: u8,
    #[packed_field(bits="5:6")] sft_ctrl: u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct Misc {
    #[packed_field(bits="0")]   graph_mode: u8,
    #[packed_field(bits="1")]   odd_even:   u8,
    #[packed_field(bits="2:3")] map_mode:   u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct IgnoreColor {
    #[packed_field(bits="0")]   pl0:  u8,
    #[packed_field(bits="1")]   pl1:  u8,
    #[packed_field(bits="2")]   pl2:  u8,
    #[packed_field(bits="3")]   pl3:  u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct AddrMap {
    #[packed_field(bits="0")]   page_map: u8,
    #[packed_field(bits="1:2")] target:   u8,
    #[packed_field(bits="3")]   io_ena:   u8,
    #[packed_field(bits="4:7")] page_ext: u8,
}