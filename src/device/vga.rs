use std::sync::{Arc, Mutex, RwLock};

enum GraphicMode { MODE_TEXT, MODE_GRAPHIC, MODE_GRAPHIC256 }

pub struct VGA {
    image: Arc<Mutex<Vec<[u8; 3]>>>,
    vram: [u8; 0x20000],
}

impl VGA {
    pub fn new(image: Arc<Mutex<Vec<[u8; 3]>>>) -> (Reg, Vram) {
        let vga = Arc::new(RwLock::new(
            Self {
                image,
                vram: [0u8; 0x20000],
            }
        ));

        (Reg(vga.clone()), Vram(vga.clone()))
    }
}

pub struct Reg(Arc<RwLock<VGA>>);

impl super::PortIO for Reg {
    fn in8(&self, _addr: u16) -> u8 {
        0
    }

    fn out8(&mut self, _addr: u16, _data: u8) -> () {
    }
}

pub struct Vram(Arc<RwLock<VGA>>);

impl super::MemoryIO for Vram {
    fn read8(&self, _ofs: u64) -> u8 {
        0
    }

    fn write8(&mut self, ofs: u64, data: u8) -> () {
        let vc = self.0.write().unwrap();
        //vc.vram[ofs as usize] = data;
        let mut buf = vc.image.lock().unwrap();
        buf[ofs as usize] = [data, data, data];
    }
}