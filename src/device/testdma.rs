use core::convert::TryInto;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, RwLock};
use crate::hardware::memory;

pub struct TestDMA {
    irq: super::IReq,
    mem: Arc<RwLock<memory::Memory>>,
    raw: [u8; 0x10],
}

impl TestDMA {
    pub fn new(irq: super::IReq, mem: Arc<RwLock<memory::Memory>>) -> (DMACtrl, DMAAddr) {
        let dma = Rc::new(RefCell::new(Self {
            irq,
            mem,
            raw: [0; 0x10],
        }));

        (DMACtrl(dma.clone()), DMAAddr(dma.clone()))
    }

    fn get_src(&self) -> u64 {
        u64::from_le_bytes(self.raw[0..8].try_into().unwrap())
    }

    fn get_dst(&self) -> u64 {
        u64::from_le_bytes(self.raw[8..16].try_into().unwrap())
    }
}

pub struct DMACtrl(Rc<RefCell<TestDMA>>);

impl super::PortIO for DMACtrl {
    fn in8(&self, _addr: u16) -> u8 {
        0
    }

    fn out8(&mut self, addr: u16, val: u8) -> () {
        if addr > 0x10 { return; }
        let size = val as usize;
        let mut tmp = vec![0; size];

        let dma = self.0.borrow_mut();
        dma.mem.read().unwrap().read_data(tmp.as_mut_ptr() as *mut _, dma.get_src() as usize, size).unwrap();
        dma.mem.write().unwrap().write_data(dma.get_dst() as usize, tmp.as_ptr() as *const _, size).unwrap();
        dma.irq.send_irq();
    }
}

pub struct DMAAddr(Rc<RefCell<TestDMA>>);

impl super::MemoryIO for DMAAddr {
    fn read8(&self, ofs: u64) -> u8 {
        self.0.borrow().raw[ofs as usize]
    }

    fn write8(&mut self, ofs: u64, val: u8) -> () {
        self.0.borrow_mut().raw[ofs as usize] = val;
    }
}