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
    let mut hw = hardware::Hardware::new();
    hw.init_memory(0x1000*0x20);

    let mut emu = emulator::Emulator::new(hw);
    emu.load_binary("/tmp/test".to_string(), 0xfff0).expect("Failed to load binary");
    emu.run();
}