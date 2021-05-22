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
        let data = &[v];
        match self.sir.idx {
            1 => self.cmr = ClkMode::unpack(data).unwrap(),
            2 => self.pmr = PlaneMask::unpack(data).unwrap(),
            3 => self.cfr = CharFont::unpack(data).unwrap(),
            4 => self.mmr = MemMode::unpack(data).unwrap(),
            7 => self.hccr = v,
            _ => {},
        }
    }

    pub fn get_memmode(&self) -> super::MemAcsMode {
        match (self.mmr.chain4, self.mmr.oe_dis) {
            (false, false) => super::MemAcsMode::ODD_EVEN,
            (false, true)  => super::MemAcsMode::SEQUENCE,
            (true, _)      => super::MemAcsMode::CHAIN4,
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

type PlaneMask = super::PlaneSelect;

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
    #[packed_field(bits="2")]   oe_dis:   bool,
    #[packed_field(bits="3")]   chain4:   bool,
}