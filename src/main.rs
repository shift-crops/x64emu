#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate log;
extern crate env_logger as logger;

mod emulator;
mod hardware;
mod interface;

use crate::hardware::Hardware;
use crate::emulator::Emulator;

fn main() {
    logger::init();
    let mut hw = Hardware::new();
    hw.init_memory(0x1000*0x10);

    let mut emu = Emulator::new(hw);
    emu.load_binary("/tmp/test".to_string(), 0xfff0).expect("Failed to load binary");
    emu.run();
}

#[cfg(test)]
#[test]
fn x64emu_test(){
    let mut hw = Hardware::new();
    hw.init_memory(0x1000);
    hw.test();

    let mut emu = Emulator::new(hw);
    emu.test();
}