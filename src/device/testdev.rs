use std::thread;
use std::time::Duration;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::Sender;
use crate::hardware::memory;

pub struct TestDev {
    irq: Sender<u8>,
    mem: Arc<RwLock<memory::Memory>>,
}

impl super::PortIO for TestDev {
}

impl super::MemoryIO for TestDev {
}

impl TestDev {
    pub fn new(irq: Sender<u8>, mem: Arc<RwLock<memory::Memory>>) -> Self {
        let tx = Sender::clone(&irq);
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(5));
                tx.send(0).unwrap();
            }
        });
        Self { irq, mem, }
    }
}