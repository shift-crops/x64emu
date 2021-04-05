mod emulator;
mod hardware;
mod interface;

use crate::hardware::Hardware;
use crate::emulator::Emulator;

fn main() {
    let mut hw = Hardware::new();
    hw.init_memory(0x1000*0x1000);

    let mut emu = Emulator::new(hw);
    emu.load_binary();
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