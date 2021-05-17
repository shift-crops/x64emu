extern crate x64emu;

use x64emu::*;

#[test]
fn switch_mode() -> () {
    let hw  = hardware::Hardware::new(0x400*0x400);
    let (dev, _)  = device::Device::new();
    let mut emu = emulator::Emulator::new(hw, dev);

    emu.map_binary(0xffff0, include_bytes!("../src/bios/crt0.bin")).expect("Failed to map");
    emu.map_binary(0xf0000, include_bytes!("../src/bios/bios.bin")).expect("Failed to map");

    emu.map_binary(0x7c00, include_bytes!("kernel/ipl.bin")).expect("Failed to map");
    emu.load_binfile(0x7e00, "tests/kernel/mode/kernel.bin".to_string()).expect("Failed to load binary");

    while let None = emu.step(true) {}
    emu.dump();
    assert_eq!(emu.ac.core.cregs.0.PE, 0);      // Real Mode

    emu.wake();
    while let None = emu.step(true) {}
    emu.dump();
    assert_eq!(emu.ac.core.cregs.0.PE, 1);      // Protected Mode

    emu.wake();
    while let None = emu.step(true) {}
    emu.dump();
    assert_eq!(emu.ac.core.msr.efer.LME, 1);    // Long Mode
}

#[test]
fn interrupt_task() -> () {
    let hw  = hardware::Hardware::new(0x400*0x400);
    let (dev, (irq, _))  = device::Device::new();
    let mut emu = emulator::Emulator::new(hw, dev);

    emu.map_binary(0xffff0, include_bytes!("../src/bios/crt0.bin")).expect("Failed to map");
    emu.map_binary(0xf0000, include_bytes!("../src/bios/bios.bin")).expect("Failed to map");

    emu.map_binary(0x7c00, include_bytes!("kernel/ipl.bin")).expect("Failed to map");
    emu.load_binfile(0x7e00, "tests/kernel/task/kernel.bin".to_string()).expect("Failed to load binary");

    irq.send(0x20).unwrap();

    while let None = emu.step(true) {}
    assert_eq!(emu.ac.mem.read().unwrap().read32(0x1000), 0xdeadbeef);
}