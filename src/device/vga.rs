mod general;
mod sequencer;
mod graphics;
mod attribute;
mod dac;
mod crt;

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use packed_struct::prelude::*;

enum GraphicMode { MODE_TEXT, MODE_GRAPHIC, MODE_GRAPHIC256 }
enum MemAcsMode  { ODD_EVEN, SEQUENCE, CHAIN4 }

bitflags! { struct PlaneFlag: u8 {
    const PL0  = 0b00000001;
    const PL1  = 0b00000010;
    const PL2  = 0b00000100;
    const PL3  = 0b00001000;
    const EVEN = Self::PL0.bits | Self::PL2.bits;
    const ODD  = Self::PL1.bits | Self::PL3.bits;
} }

pub struct VGA {
    image: Arc<Mutex<Vec<[u8; 3]>>>,
    plane: [[u8; 0x10000]; 4],
    gr: general::General,
    seq: sequencer::Sequencer,
    gc: graphics::GraphicCtrl,
    atr: attribute::Attribute,
    dac: dac::DAC,
    crt: crt::CRT,
    mem_mode: MemAcsMode,
}

impl VGA {
    pub fn new(image: Arc<Mutex<Vec<[u8; 3]>>>) -> (Reg, Vram) {
        let vga = Rc::new(RefCell::new(
            Self {
                image,
                plane: [[0; 0x10000]; 4],
                gr:  Default::default(),
                seq: Default::default(),
                gc:  Default::default(),
                atr: Default::default(),
                dac: Default::default(),
                crt: Default::default(),
                mem_mode: MemAcsMode::ODD_EVEN,
            }
        ));

        (Reg(vga.clone()), Vram(vga.clone()))
    }

    fn update_memmode(&mut self) -> () {
        let mmr = &self.seq.mmr;
        self.mem_mode = match (mmr.chain4, mmr.oe_dis) {
            (false, false) => MemAcsMode::ODD_EVEN,
            (false, true)  => MemAcsMode::SEQUENCE,
            (true, _)      => MemAcsMode::CHAIN4,
        };
    }

    fn read_plane(&self, pf: PlaneFlag, ofs: u32) -> u8 {
        for i in 0..4 {
            if pf.bits() & (1<<i) != 0 {
                return self.plane[i][ofs as usize];
            }
        }
        0
    }

    fn write_plane(&mut self, pf: PlaneFlag, ofs: u32, v: u8) -> () {
        for i in 0..4 {
            if pf.bits() & (1<<i) != 0 {
                self.plane[i][ofs as usize] = v;
            }
        }
    }

    fn plane_offset(&self, mut ofs: u32) -> Option<(PlaneFlag, u32)> {
        if !self.seq.mmr.ext_mem {
            ofs &= (1<<16)-1;
        }

        let pf = match self.mem_mode {
            MemAcsMode::ODD_EVEN => if ofs % 2 == 0 { PlaneFlag::EVEN } else { PlaneFlag::ODD },
            MemAcsMode::SEQUENCE => PlaneFlag::all(),
            MemAcsMode::CHAIN4   => PlaneFlag::from_bits_truncate((ofs as u8) % 4),
        };

        match (self.gc.mr.map_mode, ofs) {
            (0, 0..=0x1ffff)|(1, 0..=0xffff) => {},
            (2, 0x10000..=0x17fff) => ofs -= 0x10000,
            (3, 0x18000..=0x1ffff) => ofs -= 0x18000,
            _ => { return None; }
        }

        if self.gc.mr.oe_decode {
            ofs >>= 1;
            if self.gc.mr.map_mode == 1 && !self.gr.msr.pg_sel {
                ofs |= 1<<15;
            }
        }

        Some((pf & self.seq.pmr.into(), ofs))
    }
}

pub struct Reg(Rc<RefCell<VGA>>);

impl super::PortIO for Reg {
    fn in8(&self, addr: u16) -> u8 {
        let mut vga = self.0.borrow_mut();
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

    fn out8(&mut self, addr: u16, v: u8) -> () {
        let mut vga = self.0.borrow_mut();
        let data = &v.to_be_bytes();
        match (addr, vga.gr.msr.io_sel) {
            (0x3b4, 0)|(0x3d4, 1) => vga.crt.ccir = crt::CRTCtrlIndex::unpack(data).unwrap(),
            (0x3b5, 0)|(0x3d5, 1) => vga.crt.set(v),

            (0x3ba, 0)|(0x3da, 1) => vga.gr.fcr = general::FeatureCtrl::unpack(data).unwrap(),
            (0x3c2, _)            => vga.gr.msr = general::MiscOutput::unpack(data).unwrap(),

            (0x3c0, _)            => {
                if vga.atr.port_data {
                    vga.atr.set(v);
                } else {
                    vga.atr.air = attribute::AttrCtrlIndex::unpack(data).unwrap();
                }
                vga.atr.port_data ^= true;
            },

            (0x3c4, _)            => vga.seq.sir = sequencer::SeqIndex::unpack(data).unwrap(),
            (0x3c5, _)            => {
                vga.seq.set(v);
                vga.update_memmode();
            },

            (0x3c6, _)            => vga.dac.pdmr = v,
            (0x3c7, _)            => vga.dac.set_read_idx(v),
            (0x3c8, _)            => vga.dac.set_write_idx(v),
            (0x3c9, _)            => vga.dac.write_palette(v),

            (0x3ce, _)            => vga.gc.gcir = graphics::GraphCtrlIndex::unpack(data).unwrap(),
            (0x3cf, _)            => vga.gc.set(v),

            _                     => {}
        }
    }
}

pub struct Vram(Rc<RefCell<VGA>>);

impl super::MemoryIO for Vram {
    fn read8(&self, ofs: u64) -> u8 {
        let mut vga = self.0.borrow_mut();
        if !vga.gr.msr.mem_ena { return 0; }

        if let Some((mut pf, ofs)) = vga.plane_offset(ofs as u32) {
            if let MemAcsMode::SEQUENCE = vga.mem_mode {
                pf &= PlaneFlag::from_bits_truncate(1 << vga.gc.rpsr.sel);
            }

            let v = vga.read_plane(pf, ofs);
            vga.crt.latch = v;
            match vga.gc.gmr.read {
                0 => v,
                _ => 0,
            }
        } else { 0 }
    }

    fn write8(&mut self, ofs: u64, data: u8) -> () {
        let mut vga = self.0.borrow_mut();
        if !vga.gr.msr.mem_ena { return; }

        if let Some((pf, ofs)) = vga.plane_offset(ofs as u32) {
            let v = match vga.gc.gmr.write {
                0 => data,
                _ => 0,
            };
            vga.write_plane(pf, ofs, v);

            // test
            let mut buf = vga.image.lock().unwrap();
            let ofs = ofs as usize;
            buf[ofs] = [vga.plane[0][ofs], vga.plane[1][ofs], vga.plane[2][ofs]];
        }
    }
}