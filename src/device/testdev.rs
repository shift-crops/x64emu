use std::thread;
use std::time::Duration;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use crate::hardware::memory;

pub struct TestDev {
    mem: Arc<RwLock<memory::Memory>>,
    pub pdev: TestDevP,
    pub mdev: TestDevM,
}

pub struct TestDevP {
    irq: super::IReq,
    reg: u8,
    handle: Option<thread::JoinHandle<()>>,
    running: Arc<AtomicBool>,
}

impl TestDevP {
    pub fn new(irq: super::IReq) -> Self {
        Self {
            irq,
            reg: 0,
            handle:None,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    fn start_timer(&mut self) {
        let interval = self.reg as u64;
        let tx = super::IReq::clone(&self.irq);
        let running = Arc::clone(&self.running);
        running.store(true, Ordering::Relaxed);

        self.handle = Some(thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_secs(interval));
                tx.send_irq();
            }
        }));
    }

    fn stop_timer(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(h) = self.handle.take() {
            h.join().unwrap();
        }
    }
}

impl super::PortIO for TestDevP {
    fn in8(&self, _addr: u16) -> u8 {
        self.reg
    }

    fn out8(&mut self, _addr: u16, data: u8) -> () {
        self.reg = data;

        if data > 0 {
            self.start_timer();
        } else {
            self.stop_timer();
        }
    }
}

pub struct TestDevM([u8; 0x100]);

impl super::MemoryIO for TestDevM {
    fn read8(&self, ofs: u64) -> u8 {
        self.0[ofs as usize]
    }

    fn write8(&mut self, ofs: u64, data: u8) -> () {
        self.0[ofs as usize] = data;
    }
}

impl TestDev {
    pub fn new(irq: super::IReq, mem: Arc<RwLock<memory::Memory>>) -> Self {
        Self {
            mem,
            pdev: TestDevP::new(irq),
            mdev: TestDevM([0; 0x100]),
        }
    }
}