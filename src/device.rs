mod testtimer;
mod testdma;

use core::ops::Range;
use std::thread;
use std::sync::{Arc, RwLock};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use super::hardware::memory;

pub struct Device {
    io_req_tx: Sender<IORequest>,
    io_res_rx: Receiver<IOResult>,
    irq_rx: Receiver<u8>,
    memio_range: Vec<Range<u64>>
}

enum IOReqType { PortIO(u16), MemIO(u64) }
enum IOReqRW { Read(usize), Write(Vec<u8>) }

pub struct IORequest {
    ty: IOReqType,
    rw: IOReqRW,
}

pub struct IOResult {
    data: Option<Vec<u8>>,
}

pub trait PortIO {
    fn in8(&self, addr: u16) -> u8;
    fn out8(&mut self, addr: u16, data: u8) -> ();

    fn in_io(&self, addr: u16, len: usize) -> Vec<u8> {
        let mut data = vec![0; len];
        for i in 0..len {
            data[i] = self.in8(addr+i as u16);
        }
        data
    }

    fn out_io(&mut self, addr: u16, data: Vec<u8>) -> () {
        for i in 0..data.len() {
            self.out8(addr+i as u16, data[i]);
        }
    }
}

pub trait MemoryIO {
    fn read8(&self, ofs: u64) -> u8;
    fn write8(&mut self, ofs: u64, data: u8) -> ();

    fn read_io(&self, ofs: u64, len: usize) -> Vec<u8> {
        let mut data = vec![0; len];
        for i in 0..len {
            data[i] = self.read8(ofs+i as u64);
        }
        data
    }

    fn write_io(&mut self, ofs: u64, data: Vec<u8>) -> () {
        for i in 0..data.len() {
            self.write8(ofs+i as u64, data[i]);
        }
    }
}

#[derive(Clone)]
pub struct IReq {
    irq_tx: Sender<u8>,
    irq_no: u8
}

impl IReq {
    pub fn new(tx: &Sender<u8>, n: u8) -> Self {
        Self {
            irq_tx: mpsc::Sender::clone(tx),
            irq_no: n,
        }
    }

    pub fn send_irq(&self) -> () {
        self.irq_tx.send(self.irq_no).unwrap();
    }
}

type PortIOMap<'a> = Vec<(Range<u16>, &'a mut dyn PortIO)>;
type MemoryIOMap<'a> = Vec<(Range<u64>, &'a mut dyn MemoryIO)>;

impl Device {
    pub fn new(mem: Arc<RwLock<memory::Memory>>) -> Self {
        let (req_tx, req_rx): (Sender<IORequest>, Receiver<IORequest>) = mpsc::channel();
        let (res_tx, res_rx): (Sender<IOResult>, Receiver<IOResult>) = mpsc::channel();
        let (irq_tx, irq_rx): (Sender<u8>, Receiver<u8>) = mpsc::channel();

        let mut memio_range: Vec<Range<u64>> = Vec::new();
        memio_range.push(0x1000..0x1000+0x100);

        thread::spawn(move || {
            Self::init_handler(mem, req_rx, res_tx, irq_tx);
        });

        Self {
            io_req_tx: req_tx,
            io_res_rx: res_rx,
            irq_rx: irq_rx,
            memio_range,
        }
    }

    fn init_handler(mem: Arc<RwLock<memory::Memory>>, req_rx: Receiver<IORequest>, res_tx: Sender<IOResult>, irq_tx: Sender<u8>) -> () {
        let mut port_io_map: PortIOMap = Vec::new();
        let mut memory_io_map: MemoryIOMap = Vec::new();

        let (mut tst_dma_ctl, mut tst_dma_adr) = testdma::TestDMA::new(IReq::new(&irq_tx, 1), Arc::clone(&mem));
        let mut tst_timer = testtimer::TestTimer::new(IReq::new(&irq_tx, 2));

        port_io_map.push((0x10..0x10+1, &mut tst_dma_ctl));
        port_io_map.push((0x20..0x20+1, &mut tst_timer));

        memory_io_map.push((0x1000..0x1000+0x10, &mut tst_dma_adr));

        loop {
            let req = req_rx.recv().unwrap();
            let res = match req.ty {
                IOReqType::PortIO(addr) => {
                    let mut dev: Option<&mut dyn PortIO> = None;
                    for (r, d) in port_io_map.iter_mut() {
                        if r.contains(&addr) {
                            dev = Some(*d);
                            break;
                        }
                    }

                    match (req.rw, dev) {
                        (IOReqRW::Read(size), Some(d)) => {
                            Some(d.in_io(addr, size))
                        },
                        (IOReqRW::Write(ref data), Some(d)) => {
                            d.out_io(addr, data.to_vec());
                            None
                        },
                        (_, None) => None,
                    }
                },
                IOReqType::MemIO(addr) => {
                    let mut dev: Option<&mut dyn MemoryIO> = None;
                    let mut ofs = 0;
                    for (r, d) in memory_io_map.iter_mut() {
                        if r.contains(&addr) {
                            dev = Some(*d);
                            ofs = addr - r.start;
                            break;
                        }
                    }

                    match (req.rw, dev) {
                        (IOReqRW::Read(size), Some(d)) => {
                            Some(d.read_io(ofs, size))
                        },
                        (IOReqRW::Write(ref data), Some(d)) => {
                            d.write_io(ofs, data.to_vec());
                            None
                        },
                        (_, None) => None,
                    }
                },
            };
            res_tx.send(IOResult{ data: res }).unwrap();
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

    pub fn in_portio(&self, addr: u16, dst: &mut [u8]) -> () {
        let req = IORequest {
            ty: IOReqType::PortIO(addr),
            rw: IOReqRW::Read(dst.len()),
        };
        self.io_req_tx.send(req).unwrap();

        if let Some(data) = self.io_res_rx.recv().unwrap().data {
            dst.copy_from_slice(&data);
        }
    }

    pub fn out_portio(&self, addr: u16, src: &[u8]) -> () {
        let req = IORequest {
            ty: IOReqType::PortIO(addr),
            rw: IOReqRW::Write(src.to_vec()),
        };
        self.io_req_tx.send(req).unwrap();
        self.io_res_rx.recv().unwrap();
    }

    pub fn read_memio(&self, addr: u64, dst: &mut [u8]) -> () {
        let req = IORequest {
            ty: IOReqType::MemIO(addr),
            rw: IOReqRW::Read(dst.len()),
        };
        self.io_req_tx.send(req).unwrap();

        if let Some(data) = self.io_res_rx.recv().unwrap().data {
            dst.copy_from_slice(&data);
        }
    }

    pub fn write_memio(&self, addr: u64, src: &[u8]) -> () {
        let req = IORequest {
            ty: IOReqType::MemIO(addr),
            rw: IOReqRW::Write(src.to_vec()),
        };
        self.io_req_tx.send(req).unwrap();
        self.io_res_rx.recv().unwrap();
    }

    pub fn check_memio(&self, addr: u64, length: u64) -> bool {
        for r in self.memio_range.iter() {
            if r.start <= addr && addr+length-1 < r.end {
                return true;
            }
        }
        false
    }

}