use packed_struct::prelude::*;

#[derive(Debug, Default)]
pub(super) struct CRT {
    pub ccir:  CRTCtrlIndex,
    pub htr:   u8,
    pub hdeer: u8,
    pub hbsr:  u8,
    pub hber:  HorBlnkEnd,
    pub hssr:  u8,
    pub hser:  HorSyncEnd,
    pub vtr:   u8,
    pub ofr:   Overflow,
    pub prsr:  PresetRowScan,
    pub mslr:  MaxScanLine,
    pub tcsr:  TextCurStart,
    pub tcer:  TextCurEnd,
    pub sahr:  u8,
    pub salr:  u8,
    pub tclhr: u8,
    pub tcllr: u8,
    pub vssr:  u8,
    pub vser:  VertSyncEnd,
    pub vdeer: u8,
    pub or:    u8,
    pub ulr:   UnderLocate,
    pub vbsr:  u8,
    pub vber:  u8,
    pub cmr:   CRTMode,
    pub lcr:   u8,
    pub latch: u8,
}

impl CRT {
    pub fn read(&self) -> u8 {
        let data = match self.ccir.idx {
            0x00 => [self.htr],
            0x01 => [self.hdeer],
            0x02 => [self.hbsr],
            0x03 => self.hber.pack().unwrap(),
            0x04 => [self.hssr],
            0x05 => self.hser.pack().unwrap(),
            0x06 => [self.vtr],
            0x07 => self.ofr.pack().unwrap(),
            0x08 => self.prsr.pack().unwrap(),
            0x09 => self.mslr.pack().unwrap(),
            0x0a => self.tcsr.pack().unwrap(),
            0x0b => self.tcer.pack().unwrap(),
            0x0c => [self.sahr],
            0x0d => [self.salr],
            0x0e => [self.tclhr],
            0x0f => [self.tcllr],
            0x10 => [self.vssr],
            0x11 => self.vser.pack().unwrap(),
            0x12 => [self.vdeer],
            0x13 => [self.or],
            0x14 => self.ulr.pack().unwrap(),
            0x15 => [self.vbsr],
            0x16 => [self.vber],
            0x17 => self.cmr.pack().unwrap(),
            0x18 => [self.lcr],
            0x22 => [self.latch],
            _ => [0],
        };
        u8::from_be_bytes(data)
    }

    pub fn write(&mut self, v: u8) -> () {
        let data = &v.to_be_bytes();
        match self.ccir.idx {
            0x00 => self.htr = v,
            0x01 => self.hdeer = v,
            0x02 => self.hbsr = v,
            0x03 => self.hber = HorBlnkEnd::unpack(data).unwrap(),
            0x04 => self.hssr = v,
            0x05 => self.hser = HorSyncEnd::unpack(data).unwrap(),
            0x06 => self.vtr = v,
            0x07 => self.ofr = Overflow::unpack(data).unwrap(),
            0x08 => self.prsr = PresetRowScan::unpack(data).unwrap(),
            0x09 => self.mslr = MaxScanLine::unpack(data).unwrap(),
            0x0a => self.tcsr = TextCurStart::unpack(data).unwrap(),
            0x0b => self.tcer = TextCurEnd::unpack(data).unwrap(),
            0x0c => self.sahr = v,
            0x0d => self.salr = v,
            0x0e => self.tclhr = v,
            0x0f => self.tcllr = v,
            0x10 => self.vssr = v,
            0x11 => self.vser = VertSyncEnd::unpack(data).unwrap(),
            0x12 => self.vdeer = v,
            0x13 => self.or = v,
            0x14 => self.ulr = UnderLocate::unpack(data).unwrap(),
            0x15 => self.vbsr = v,
            0x16 => self.vber = v,
            0x17 => self.cmr = CRTMode::unpack(data).unwrap(),
            0x18 => self.lcr = v,
            _ => {},
        }
    }
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct CRTCtrlIndex {
    #[packed_field(bits="0:6")] idx: u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct HorBlnkEnd {
    #[packed_field(bits="0:4")] bl_end:    u8,
    #[packed_field(bits="5:6")] skew_ctrl: u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct HorSyncEnd {
    #[packed_field(bits="0:4")] end:    u8,
    #[packed_field(bits="5:6")] delay:  u8,
    #[packed_field(bits="7")]   bl_end: u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct Overflow {
    #[packed_field(bits="0")]   vt_total8:    u8,
    #[packed_field(bits="1")]   vt_disp_ena8: u8,
    #[packed_field(bits="2")]   vt_sync_str8: u8,
    #[packed_field(bits="3")]   vt_bl_str8:   u8,
    #[packed_field(bits="4")]   line_cmp8:    u8,
    #[packed_field(bits="5")]   vt_total9:    u8,
    #[packed_field(bits="6")]   vt_disp_ena9: u8,
    #[packed_field(bits="7")]   vt_sync_str9: u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct PresetRowScan {
    #[packed_field(bits="0:4")] scan_count: u8,
    #[packed_field(bits="5:6")] byte_pan:   u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct MaxScanLine {
    #[packed_field(bits="0:4")] scan_count: u8,
    #[packed_field(bits="5")]   vt_bl_str9: u8,
    #[packed_field(bits="6")]   line_cmp9:  u8,
    #[packed_field(bits="7")]   dbl_scan:   u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct TextCurStart {
    #[packed_field(bits="0:4")] cur_str: u8,
    #[packed_field(bits="5")]   cur_off: u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct TextCurEnd {
    #[packed_field(bits="0:4")] cur_end:  u8,
    #[packed_field(bits="5:6")] cur_skew: u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct VertSyncEnd {
    #[packed_field(bits="0:3")] end:      u8,
    #[packed_field(bits="4")]   int_clr:  u8,
    #[packed_field(bits="5")]   int_ena:  u8,
    #[packed_field(bits="7")]   prot_reg: u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct UnderLocate {
    #[packed_field(bits="0:4")] location: u8,
    #[packed_field(bits="5")]   count:    u8,
    #[packed_field(bits="6")]   dword:    u8,
}

#[derive(Debug, Default, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct CRTMode {
    #[packed_field(bits="0")]   compat:     u8,
    #[packed_field(bits="1")]   row_ctrl:   u8,
    #[packed_field(bits="2")]   hor_sel:    u8,
    #[packed_field(bits="3")]   count:      u8,
    #[packed_field(bits="5")]   addr_wrap:  u8,
    #[packed_field(bits="6")]   wb_mode:    u8,
    #[packed_field(bits="7")]   ctrl_reset: u8,
}
