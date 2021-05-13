use std::{thread, time};
use std::sync::{Arc, Mutex, RwLock};
use crate::hardware::memory;

pub struct Core {
    base_addr: usize,
    mem: Arc<RwLock<memory::Memory>>,
    image: Arc<Mutex<Vec<[u8; 3]>>>,
}

pub struct VGA(Arc<RwLock<Core>>);
impl VGA {
    pub fn new(mem: Arc<RwLock<memory::Memory>>, image: Arc<Mutex<Vec<[u8; 3]>>>) -> Self {
        Self(Arc::new(RwLock::new(
            Core {
                base_addr: 0xa0000,
                mem,
                image,
            }
        )))
    }

    pub fn run(&self) {
        let core = Arc::clone(&self.0);
        std::thread::spawn(move || {
            let mut vram = [0u8; 0x20000];
            loop {
                thread::sleep(time::Duration::from_millis(40));
                let vc = core.read().unwrap();

                vc.mem.read().unwrap().read_data(vram.as_mut_ptr() as *mut _, vc.base_addr, std::mem::size_of_val(&vram)).unwrap();
                let mut buf = vc.image.lock().unwrap();

                for i in 0..320*200 {
                    let c = vram[i];
                    buf[i] = [c, c, c];
                }
            }
        });
    }
}

impl super::PortIO for VGA {
    fn in8(&self, _addr: u16) -> u8 {
        0
    }

    fn out8(&mut self, _addr: u16, _data: u8) -> () {
    }
}