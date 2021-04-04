mod emulator;
mod hardware;
mod interface;

use crate::hardware::Hardware;
use crate::emulator::Emulator;

fn main() {
    let mut hw = Hardware::new();
    hw.init_memory(0x1000*0x1000);
    //hw.test();

    let mut emu = Emulator::new(hw);
    emu.load_binary();
    //emu.test();
    emu.run();
}
