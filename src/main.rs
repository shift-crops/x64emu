mod emulator;
mod hardware;
mod interface;

use crate::hardware::Hardware;
use crate::emulator::Emulator;

fn main() {
    let mut hw = Hardware::new();
    hw.init_memory(0x1000*2);
    //hw.test();

    let mut emu = Emulator::new(hw);
    emu.run();
}
