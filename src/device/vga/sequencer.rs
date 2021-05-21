use packed_struct::prelude::*;

#[derive(Debug, Default)]
pub(super) struct Sequencer {
    pub sir: SeqIndex,
    cmr: ClkMode,
    pub pmr: PlaneMask,
    cfr: CharFont,
    pub mmr: MemMode,
    hccr: u8,
}

impl Sequencer {
    pub fn get(&self) -> u8 {
        match self.sir.idx {
            1 => self.cmr.pack().unwrap()[0],
            2 => self.pmr.pack().unwrap()[0],
            3 => self.cfr.pack().unwrap()[0],
            4 => self.mmr.pack().unwrap()[0],
            7 => self.hccr,
            _ => 0,
        }
    }

    pub fn set(&mut self, v: u8) -> () {
        let data = &v.to_be_bytes();
        match self.sir.idx {
            1 => self.cmr = ClkMode::unpack(data).unwrap(),
            2 => self.pmr = PlaneMask::unpack(data).unwrap(),
            3 => self.cfr = CharFont::unpack(data).unwrap(),
            4 => self.mmr = MemMode::unpack(data).unwrap(),
            7 => self.hccr = v,
            _ => {},
        }
    }
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct SeqIndex {
    #[packed_field(bits="0:3")] idx: u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct ClkMode {
    #[packed_field(bits="0")]   dot_clk:  u8,
    #[packed_field(bits="2")]   sft_ld:   u8,
    #[packed_field(bits="3")]   dclk_div: u8,
    #[packed_field(bits="4")]   sft4:     u8,
    #[packed_field(bits="5")]   scr_off:  u8,
}

#[derive(Debug, Default, Clone, Copy, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct PlaneMask {
    #[packed_field(bits="0")]   pl0:  bool,
    #[packed_field(bits="1")]   pl1:  bool,
    #[packed_field(bits="2")]   pl2:  bool,
    #[packed_field(bits="3")]   pl3:  bool,
}

impl From<PlaneMask> for super::PlaneFlag {
    fn from(mask: PlaneMask) -> Self {
        Self::from_bits_truncate(mask.pack().unwrap()[0])
    }
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct CharFont {
    #[packed_field(bits="0:1")] b_low:  u8,
    #[packed_field(bits="2:3")] a_low:  u8,
    #[packed_field(bits="4")]   b_high: u8,
    #[packed_field(bits="5")]   a_high: u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct MemMode {
    #[packed_field(bits="1")]   pub ext_mem:  bool,
    #[packed_field(bits="2")]   pub oe_dis:   bool,
    #[packed_field(bits="3")]   pub chain4:   bool,
}