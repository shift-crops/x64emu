mod hardware;
mod instruction;

pub struct Emulator {
    hw: hardware::Hardware,
}

impl Emulator {
    pub fn new(ncore: usize, size: usize) -> Self {
        let mut emu = Emulator {
            hw: hardware::Hardware::new(),
        };

        emu.hw.init_core(ncore);
        emu.hw.init_memory(size);

        emu
    }

    pub fn test(&mut self) -> () {
        self.hw.test();
    }
}