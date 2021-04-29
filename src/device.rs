use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

pub struct Device {
    pub tx: Sender<u8>,
    pub rx: Receiver<u8>,
}

impl Device {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();

        /*
        let tx1 = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(5));
                tx1.send(0).unwrap();
            }
        });
        */

        Self{ tx, rx, }
    }

}