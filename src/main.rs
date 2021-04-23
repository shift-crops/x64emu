#![allow(dead_code)]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate log;
extern crate env_logger as logger;

mod emulator;
mod hardware;
mod interface;

use std::env;
use interface::gdbserver;
use gdbstub::{Connection, GdbStub};

fn main() {
    logger::init();
    let args: Vec<String> = env::args().collect();

    let hw = hardware::Hardware::new(0x400*0x400);
    let mut emu = emulator::Emulator::new(hw);

    emu.map_binary(0xffff0, include_bytes!("bios/crt0.bin")).expect("Failed to map");
    emu.map_binary(0xf0000, include_bytes!("bios/bios.bin")).expect("Failed to map");

    let img = if args.len() > 1 { args[1].clone() } else { "/tmp/test".to_string() };
    emu.load_binfile(0x7c00, img).expect("Failed to load binary");

    let connection: Box<dyn Connection<Error = std::io::Error>> = Box::new(gdbserver::wait_for_tcp(9001).expect("wait error"));
    let mut debugger = GdbStub::new(connection);

    debugger.run(&mut emu).expect("debugger error");
    loop {
        emu.step();
    }
}