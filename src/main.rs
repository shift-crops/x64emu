mod emulator;
mod interface;

fn main() {
    let mut emu = emulator::Emulator::new(2*0x1000*0x1000);

    emu.cpu.test();
    emu.cpu.dump();
}
