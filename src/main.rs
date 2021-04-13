#![allow(dead_code)]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate log;
extern crate env_logger as logger;

mod emulator;
mod hardware;
mod interface;

fn main() {
    logger::init();
    let hw = hardware::Hardware::new(0x7c00 /* 0xfff0 */, 0x1000*0x20);

    let mut emu = emulator::Emulator::new(hw);
    emu.load_binary("/tmp/test".to_string(), 0x7c00).expect("Failed to load binary");
    loop {
        emu.step();
    }
}