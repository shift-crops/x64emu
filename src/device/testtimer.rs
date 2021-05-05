use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::mpsc::Sender;

pub struct TestTimer {
    irq: super::IReq,
    reg: u8,
    sig: Option<Sender<()>>,
    handle: Option<thread::JoinHandle<()>>,
    running: Arc<AtomicBool>,
}

impl TestTimer {
    pub fn new(irq: super::IReq) -> Self {
        Self {
            irq,
            reg: 0,
            sig: None,
            handle: None,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    fn start_timer(&mut self) {
        let interval = self.reg as u64;
        let irq = super::IReq::clone(&self.irq);
        let running = Arc::clone(&self.running);
        running.store(true, Ordering::Relaxed);

        let (tx, rx) = mpsc::channel();
        self.sig = Some(tx);
        self.handle = Some(thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                match rx.recv_timeout(Duration::from_secs(interval)) {
                    Err(_) => irq.send_irq(),
                    Ok(_) => break,
                }
            }
        }));
    }

    fn stop_timer(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        if let Some(tx) = self.sig.take() {
            tx.send(()).unwrap();
        }
        if let Some(h) = self.handle.take() {
            h.join().unwrap();
        }
    }
}

impl super::PortIO for TestTimer {
    fn in8(&self, addr: u16) -> u8 {
        if addr != 0x20 { 0 } else { self.reg }
    }

    fn out8(&mut self, addr: u16, data: u8) -> () {
        if addr != 0x20 { return; }

        self.reg = data;

        self.stop_timer();
        if data > 0 {
            self.start_timer();
        }
    }
}