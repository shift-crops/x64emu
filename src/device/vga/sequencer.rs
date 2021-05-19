use packed_struct::prelude::*;

#[derive(Debug, Default)]
pub(super) struct Sequencer {
    pub sir: SeqIndex,
    pub cmr: ClkMode,
    pub pmr: PlaneMask,
    pub cfr: CharFont,
    pub mmr: MemMode,
    pub hccr: u8,
}

impl Sequencer {
    pub fn read(&self) -> u8 {
        let data = match self.sir.idx {
            1 => self.cmr.pack().unwrap(),
            2 => self.pmr.pack().unwrap(),
            3 => self.cfr.pack().unwrap(),
            4 => self.mmr.pack().unwrap(),
            7 => [self.hccr],
            _ => [0],
        };
        u8::from_be_bytes(data)
    }

    pub fn write(&mut self, v: u8) -> () {
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

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct PlaneMask {
    #[packed_field(bits="0")]   pl0:  u8,
    #[packed_field(bits="1")]   pl1:  u8,
    #[packed_field(bits="2")]   pl2:  u8,
    #[packed_field(bits="3")]   pl3:  u8,
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
    #[packed_field(bits="1")]   ext_mem:  u8,
    #[packed_field(bits="2")]   odd_even: u8,
    #[packed_field(bits="3")]   chain4:   u8,
}