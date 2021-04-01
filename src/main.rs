mod emulator;
mod interface;

fn main() {
    let emu = emulator::Emulator::new(2*0x1000*0x1000);

    emu.dump();
}
