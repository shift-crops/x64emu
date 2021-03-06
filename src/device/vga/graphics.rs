use packed_struct::prelude::*;

#[derive(Debug, Default)]
pub(super) struct GraphicCtrl {
    pub gcir: GraphCtrlIndex,
    spr:  SetPlane,
    espr: EnableSetPlane,
    pub ccr:  ColorCmp,
    drr:  DataRot,
    pub rpsr: ReadPlaneSel,
    pub gmr:  GraphMode,
    pub mr:   Misc,
    pub icr:  IgnoreColor,
    bmr:  u8,
    amr:  AddrMap,
    psr:  u8,
    flag: u8,
}

impl GraphicCtrl {
    pub fn get(&self) -> u8 {
        match self.gcir.idx {
            0x00 => self.spr.pack().unwrap()[0],
            0x01 => self.espr.pack().unwrap()[0],
            0x02 => self.ccr.pack().unwrap()[0],
            0x03 => self.drr.pack().unwrap()[0],
            0x04 => self.rpsr.pack().unwrap()[0],
            0x05 => self.gmr.pack().unwrap()[0],
            0x06 => self.mr.pack().unwrap()[0],
            0x07 => self.icr.pack().unwrap()[0],
            0x08 => self.bmr,
            0x10 => self.amr.pack().unwrap()[0],
            0x11 => self.psr,
            0x18 => self.flag,
            _ => 0,
        }
    }

    pub fn set(&mut self, v: u8) -> () {
        let data = &[v];
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

    pub fn graphic_mode(&self) -> super::GraphicMode {
        match (self.mr.graph_text, self.gmr.c256_mode, self.gmr.sft_ctrl) {
            (false, _, _)        => super::GraphicMode::TEXT,
            (true, false, false) => super::GraphicMode::GRAPHIC,
            (true, false, true)  => super::GraphicMode::GRAPHIC_SHIFT,
            (true, true, _)      => super::GraphicMode::GRAPHIC_256,
        }
    }

    pub fn rotate(&self, v: u8) -> u8 {
        v.rotate_right(self.drr.rot_count as u32)
    }

    pub fn set_reset(&self, n: u8, v: Option<u8>) -> u8 {
        let sr = if super::PlaneFlag::from(self.spr).check(n) { 0xff } else { 0 };

        if let Some(v) = v {
            if super::PlaneFlag::from(self.espr).check(n) {sr } else { v }
        } else { sr }
    }

    pub fn calc_latch(&self, v: u8, latch: u8) -> u8 {
        match self.drr.func_sel {
            1 => v & latch,
            2 => v | latch,
            3 => v ^ latch,
            _ => v,
        }
    }

    pub fn mask_latch(&self, v: u8, latch: u8) -> u8 {
        (v & self.bmr) | (latch & !self.bmr)
    }
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct GraphCtrlIndex {
    #[packed_field(bits="0:4")] idx: u8,
}

type SetPlane = super::PlaneSelect;
type EnableSetPlane = super::PlaneSelect;

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct ColorCmp {
    #[packed_field(bits="0")]   bit0:  u8,
    #[packed_field(bits="1")]   bit1:  u8,
    #[packed_field(bits="2")]   bit2:  u8,
    #[packed_field(bits="3")]   bit3:  u8,
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
    #[packed_field(bits="0:1")] pub sel: u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct GraphMode {
    #[packed_field(bits="0:1")] pub write: u8,
    #[packed_field(bits="3")]   pub read:  u8,
    #[packed_field(bits="4")]   oe_cga:    u8,
    #[packed_field(bits="5")]   sft_ctrl:  bool,
    #[packed_field(bits="6")]   c256_mode: bool,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct Misc {
    #[packed_field(bits="0")]   graph_text:    bool,
    #[packed_field(bits="1")]   pub oe_decode: bool,
    #[packed_field(bits="2:3")] pub map_mode:  u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct IgnoreColor {
    #[packed_field(bits="0")]   bit0:  u8,
    #[packed_field(bits="1")]   bit1:  u8,
    #[packed_field(bits="2")]   bit2:  u8,
    #[packed_field(bits="3")]   bit3:  u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct AddrMap {
    #[packed_field(bits="0")]   page_map: u8,
    #[packed_field(bits="1:2")] target:   u8,
    #[packed_field(bits="3")]   io_ena:   u8,
    #[packed_field(bits="4:7")] page_ext: u8,
}