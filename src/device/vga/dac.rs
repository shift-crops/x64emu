use packed_struct::prelude::*;

#[derive(Debug, Default)]
pub(super) struct DAC {
    pub pdmr: u8,
    pub dsr:  State,
    pub prir: u8,
    pub pwir: u8,
    pub pdr:  u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct State {
    #[packed_field(bits="0:1")] stat: u8,
}