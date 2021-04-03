mod emulator;
mod interface;

fn main() {
    let mut emu = emulator::Emulator::new(1, 0x1000*2);

    emu.test();
}
