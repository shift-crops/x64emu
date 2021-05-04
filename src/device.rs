mod testdev;

use core::ops::Range;
use std::thread;
use std::sync::{Arc, RwLock};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::collections::HashMap;

use super::hardware::memory;

pub struct Device {
    io_req_tx: Sender<IORequest>,
    io_res_rx: Receiver<IOResult>,
    irq_rx: Receiver<u8>,
    memio_range: Vec<Range<u64>>
}

pub struct IORequest {
}

pub struct IOResult {
}

pub trait PortIO {
}

pub trait MemoryIO {
}

type PortIOMap<'a> = HashMap<u16, (u8, &'a mut dyn PortIO)>;
type MemoryIOMap<'a> = HashMap<u64, (u64, &'a mut dyn MemoryIO)>;

impl Device {
    pub fn new(mem: Arc<RwLock<memory::Memory>>) -> Self {
        let (req_tx, req_rx): (Sender<IORequest>, Receiver<IORequest>) = mpsc::channel();
        let (res_tx, res_rx): (Sender<IOResult>, Receiver<IOResult>) = mpsc::channel();
        let (irq_tx, irq_rx): (Sender<u8>, Receiver<u8>) = mpsc::channel();

        let mut port_io_map: PortIOMap = HashMap::new();
        let mut memory_io_map: MemoryIOMap = HashMap::new();
        let mut memio_range: Vec<Range<u64>> = vec!();

        let mut tstdev = testdev::TestDev::new(mpsc::Sender::clone(&irq_tx), Arc::clone(&mem));
        port_io_map.insert(0, (1, &mut tstdev));
        memory_io_map.insert(0x1000, (0x10, &mut tstdev));
        memio_range.push(0x1000..0x1000+0x10);

        thread::spawn(move || {
            loop {
                req_rx.recv().unwrap();
            }
        });

        Self {
            io_req_tx: req_tx,
            io_res_rx: res_rx,
            irq_rx: irq_rx,
            memio_range,
        }
    }

    pub fn get_interrupt_req(&self, block: bool) -> Option<u8> {
        if block {
            Some(self.irq_rx.recv().unwrap())
        } else if let Ok(n) = self.irq_rx.try_recv() {
            Some(n)
        } else {
            None
        }
    }

    pub fn check_memio(&self, addr: u64, length: u64) -> bool {
        for r in self.memio_range.iter() {
            if r.contains(&addr) && r.contains(&(addr+length)) {
                return true;
            }
        }
        false
    }

    pub fn read_memio(&self, _addr: u64, _dst: &mut [u8]) -> () {
        panic!("Not Implemented");
    }

    pub fn write_memio(&self, _addr: u64, _src: &[u8]) -> () {
        panic!("Not Implemented");
    }
}