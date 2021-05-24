mod general;
mod sequencer;
mod graphics;
mod attribute;
mod dac;
mod crt;

use std::{thread, time};
use std::sync::{Arc, Mutex, RwLock};
use packed_struct::prelude::*;

enum GraphicMode { TEXT, GRAPHIC, GRAPHIC_SHIFT, GRAPHIC_256 }
enum MemAcsMode  { ODD_EVEN, SEQUENCE, CHAIN4 }

bitflags! { struct PlaneFlag: u8 {
    const PL0  = 0b00000001;
    const PL1  = 0b00000010;
    const PL2  = 0b00000100;
    const PL3  = 0b00001000;
    const EVEN = Self::PL0.bits | Self::PL2.bits;
    const ODD  = Self::PL1.bits | Self::PL3.bits;
} }

impl PlaneFlag {
    fn select_one(&self) -> Option<u8> {
        for i in 0..4 {
            if self.check(i) { return Some(i); }
        }
        None
    }

    fn check(&self, n: u8) -> bool {
        if n > 3 { return false; }
        self.bits() & (1<<n) != 0
    }
}

#[derive(Debug, Default, Clone, Copy, PackedStruct)]
#[packed_struct(bit_numbering="lsb0", size_bytes="1")]
pub struct PlaneSelect {
    #[packed_field(bits="0")]   pl0:  bool,
    #[packed_field(bits="1")]   pl1:  bool,
    #[packed_field(bits="2")]   pl2:  bool,
    #[packed_field(bits="3")]   pl3:  bool,
}

impl From<PlaneSelect> for PlaneFlag {
    fn from(sel: PlaneSelect) -> Self {
        Self::from_bits_truncate(sel.pack().unwrap()[0])
    }
}

pub struct VGA {
    gmode: GraphicMode,
    mmode: MemAcsMode,
    plane: [[u8; 0x10000]; 4],
    gr: general::General,
    seq: sequencer::Sequencer,
    gc: graphics::GraphicCtrl,
    atr: attribute::Attribute,
    dac: dac::DAConv,
    crt: crt::CRT,
}

impl VGA {
    pub fn new(image: Arc<Mutex<Vec<[u8; 3]>>>) -> (Reg, Vram) {
        let mut crt: crt::CRT = Default::default();
        crt.hdeer = 40;
        crt.vdeer = 25;

        let vga = Arc::new(RwLock::new(
            Self {
                gmode: GraphicMode::TEXT,
                mmode: MemAcsMode::ODD_EVEN,
                plane: [[0; 0x10000]; 4],
                gr:  Default::default(),
                seq: Default::default(),
                gc:  Default::default(),
                atr: Default::default(),
                dac: Default::default(),
                crt,
            }
        ));

        let _vga = vga.clone();
        thread::spawn(move || {
            let mut frame_count: u8 = 0;
            loop {
                thread::sleep(time::Duration::from_millis(100));
                _vga.read().unwrap().refresh(&mut image.lock().unwrap(), frame_count);
                frame_count = frame_count.wrapping_add(1);
            }
        });

        (Reg(vga.clone()), Vram(vga.clone()))
    }

    fn refresh(&self, buf: &mut Vec<[u8; 3]>, fc: u8) -> () {
        for i in 0..buf.len() {
            let attr_idx = match self.gmode {
                GraphicMode::TEXT => {
                    let (cur_frq, chr_frq) = ((fc/3) % 2 == 0, (fc/6) % 2 == 0);

                    let c_height  = self.crt.char_height();
                    let blink = self.atr.text_blink();

                    let (x, y) = self.crt.pixel_to_pos(i as u32);
                    let idx    = self.crt.pos_to_chridx(x, y);
                    let (chr, attr) = (self.plane[0][idx as usize], self.plane[1][idx as usize]);

                    let (cx, cy) = (x as u8 % 8, y as u8 % c_height);
                    let chr_bit = if !(blink && (attr & 0x80 != 0) && chr_frq) {
                        let map_ofs = self.seq.charmap_offset(attr&8 != 0) as usize;
                        let chr_ofs = 0x20*chr as usize + cy as usize;
                        (self.plane[2][map_ofs + chr_ofs] >> cx) & 1 != 0
                    } else { false };

                    let cur_bit = if let Some((h_rng, skew)) = self.crt.get_cursor(idx) {
                        cur_frq && (skew..=6).contains(&cx) && h_rng.contains(&cy)
                    } else { false };

                    match (chr_bit | cur_bit, blink) {
                        (true, _)      => attr & 7,
                        (false, true)  => (attr >> 4) & 7,
                        (false, false) => attr >> 4,
                    }
                },
                GraphicMode::GRAPHIC => {
                    let idx = i/8;
                    let bit = 7-(i%8);

                    let mut attr_idx = 0;
                    for j in 0..4 {
                        attr_idx |= ((self.plane[j][idx] >> bit) & 1) << j;
                    }
                    attr_idx
                },
                GraphicMode::GRAPHIC_SHIFT => {
                    let num = (i/4)%2;
                    let idx = i/8;
                    let bit = 6-((i%4)*2);

                    (((self.plane[num+2][idx] >> bit) & 3) << 2) | ((self.plane[num][idx] >> bit) & 3)
                },
                GraphicMode::GRAPHIC_256 => {
                    self.plane[i%4][i/4]
                },
            };

            let dac_idx = if let GraphicMode::GRAPHIC_256 = self.gmode { attr_idx } else { self.atr.dac_index(attr_idx) };
            buf[i] = self.dac.get_palette(dac_idx);
        }
    }

    fn read_plane(&self, n: u8, ofs: u16) -> u8 {
        self.plane[n as usize][ofs as usize]
    }

    fn read_planes(&mut self, pf: PlaneFlag, ofs: u16) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        for i in 0..4 {
            if pf.check(i) {
                data.push(self.read_plane(i, ofs));
            }
        }
        data
    }

    fn write_plane(&mut self, n: u8, ofs: u16, v: u8) -> () {
        self.plane[n as usize][ofs as usize] = v;
    }

    fn write_planes(&mut self, pf: PlaneFlag, ofs: u16, v: u8) -> () {
        for i in 0..4 {
            if pf.check(i) {
                self.write_plane(i, ofs, v);
            }
        }
    }

    fn plane_offset(&self, mut ofs: u32) -> Option<(PlaneFlag, u16)> {
        if !self.seq.mmr.ext_mem {
            ofs &= (1<<16)-1;
        }

        match (self.gc.mr.map_mode, ofs) {
            (0, 0..=0x1ffff)|(1, 0..=0xffff) => {},
            (2, 0x10000..=0x17fff) => ofs -= 0x10000,
            (3, 0x18000..=0x1ffff) => ofs -= 0x18000,
            _ => { return None; }
        }

        let pf = match self.mmode {
            MemAcsMode::ODD_EVEN => {
                let pf = if ofs % 2 == 0 { PlaneFlag::EVEN } else { PlaneFlag::ODD };
                if self.gc.mr.oe_decode {
                    ofs >>= 1;
                    if self.gc.mr.map_mode == 1 && !self.gr.msr.pg_sel {
                        ofs |= 1<<15;
                    }
                }
                pf
            },
            MemAcsMode::SEQUENCE => PlaneFlag::all(),
            MemAcsMode::CHAIN4   => {
                let sel = (ofs as u8) %4;
                ofs /= 4;
                PlaneFlag::from_bits_truncate(sel)
            },
        };

        if ofs < 1<<16 { Some((pf & self.seq.pmr.into(), ofs as u16)) } else { None }
    }
}

pub struct Reg(Arc<RwLock<VGA>>);

impl super::PortIO for Reg {
    fn in8(&self, addr: u16) -> u8 {
        let mut vga = self.0.write().unwrap();
        match (addr, vga.gr.msr.io_sel) {
            (0x3b4, 0)|(0x3d4, 1) => vga.crt.ccir.pack().unwrap()[0],
            (0x3b5, 0)|(0x3d5, 1) => vga.crt.get(),

            (0x3c2, _)            => vga.gr.st0.pack().unwrap()[0],
            (0x3ba, 0)|(0x3da, 1) => vga.gr.st1.pack().unwrap()[0],
            (0x3ca, _)            => vga.gr.fcr.pack().unwrap()[0],
            (0x3cc, _)            => vga.gr.msr.pack().unwrap()[0],

            (0x3c0, _)            => vga.atr.air.pack().unwrap()[0],
            (0x3c1, _)            => vga.atr.get(),

            (0x3c4, _)            => vga.seq.sir.pack().unwrap()[0],
            (0x3c5, _)            => vga.seq.get(),

            (0x3c6, _)            => vga.dac.pdmr,
            (0x3c7, _)            => vga.dac.dsr.pack().unwrap()[0],
            (0x3c9, _)            => vga.dac.read_palette(),

            (0x3ce, _)            => vga.gc.gcir.pack().unwrap()[0],
            (0x3cf, _)            => vga.gc.get(),

            _                     => 0,
        }
    }

    fn out8(&mut self, addr: u16, val: u8) -> () {
        let mut vga = self.0.write().unwrap();
        let datum = &[val];
        match (addr, vga.gr.msr.io_sel) {
            (0x3b4, 0)|(0x3d4, 1) => vga.crt.ccir = crt::CRTCtrlIndex::unpack(datum).unwrap(),
            (0x3b5, 0)|(0x3d5, 1) => vga.crt.set(val),

            (0x3ba, 0)|(0x3da, 1) => vga.gr.fcr = general::FeatureCtrl::unpack(datum).unwrap(),
            (0x3c2, _)            => vga.gr.msr = general::MiscOutput::unpack(datum).unwrap(),

            (0x3c0, _)            => {
                if vga.atr.port_data {
                    vga.atr.set(val);
                } else {
                    vga.atr.air = attribute::AttrCtrlIndex::unpack(datum).unwrap();
                }
                vga.atr.port_data ^= true;
            },

            (0x3c4, _)            => vga.seq.sir = sequencer::SeqIndex::unpack(datum).unwrap(),
            (0x3c5, _)            => {
                vga.seq.set(val);
                vga.mmode = vga.seq.memory_mode();
            },

            (0x3c6, _)            => vga.dac.pdmr = val,
            (0x3c7, _)            => vga.dac.set_read_idx(val),
            (0x3c8, _)            => vga.dac.set_write_idx(val),
            (0x3c9, _)            => vga.dac.write_palette(val),

            (0x3ce, _)            => vga.gc.gcir = graphics::GraphCtrlIndex::unpack(datum).unwrap(),
            (0x3cf, _)            => {
                vga.gc.set(val);
                vga.gmode = vga.gc.graphic_mode();
            },

            _                     => {}
        }
    }
}

pub struct Vram(Arc<RwLock<VGA>>);

impl super::MemoryIO for Vram {
    fn read8(&self, ofs: u64) -> u8 {
        let mut vga = self.0.write().unwrap();
        if !vga.gr.msr.mem_ena { return 0; }

        let mut datum = 0;
        if let Some((mut pf, ofs)) = vga.plane_offset(ofs as u32) {
            datum = match vga.gc.gmr.read {
                0 => {
                    if let MemAcsMode::SEQUENCE = vga.mmode {
                        pf &= PlaneFlag::from_bits_truncate(1 << vga.gc.rpsr.sel);
                    }

                    if let Some(n) = pf.select_one() {
                        vga.read_plane(n, ofs)
                    } else { 0 }
                },
                1 => {
                    let cmp = !vga.gc.ccr.pack().unwrap()[0];
                    let mut plane_data = vga.read_planes(pf, ofs);

                    for v in plane_data.iter_mut() {
                        *v = *v^cmp;
                    }
                    plane_data.iter().fold(0xf, |res, v| res & v ) & vga.gc.icr.pack().unwrap()[0]
                },
                _ => panic!("Unknown read mode: {}", vga.gc.gmr.read),
            };
            vga.crt.latch = datum;
        }
        datum
    }

    fn write8(&mut self, ofs: u64, val: u8) -> () {
        let mut vga = self.0.write().unwrap();
        if !vga.gr.msr.mem_ena { return; }

        if let Some((pf, ofs)) = vga.plane_offset(ofs as u32) {
            let latch = vga.crt.latch;
            match vga.gc.gmr.write {
                0 => {
                    let rot = vga.gc.rotate(val);
                    for i in 0..4 {
                        if !pf.check(i) { continue; }

                        let v = vga.gc.set_reset(i, Some(rot));
                        let v = vga.gc.calc_latch(v, latch);
                        let v = vga.gc.mask_latch(v, latch);
                        vga.write_plane(i, ofs, v);
                    }
                },
                1 => vga.write_planes(pf, ofs, latch),
                2 => {
                    for i in 0..4 {
                        if !pf.check(i) { continue; }

                        let v = if (val >> i) & 1 == 0 { 0 } else { 0xff };
                        let v = vga.gc.calc_latch(v, latch);
                        let v = vga.gc.mask_latch(v, latch);
                        vga.write_plane(i, ofs, v);
                    }
                },
                3 => {
                    let rot = vga.gc.rotate(val);
                    let mask = vga.gc.mask_latch(rot, 0);
                    for i in 0..4 {
                        if !pf.check(i) { continue; }

                        let sr = vga.gc.set_reset(i, None);
                        vga.write_plane(i, ofs, (sr & mask) | (latch & !mask));
                    }
                },
                _ => panic!("Unknown write mode: {}", vga.gc.gmr.write),
            }
        }
    }
}