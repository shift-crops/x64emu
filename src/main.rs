#![allow(dead_code)]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate log;
extern crate env_logger as logger;
extern crate getopts;

mod hardware;
mod device;
mod emulator;
mod interface;

use std::{env, process};
use getopts::Options;
use gdbstub::{Connection, GdbStub};
use interface::gdbserver;

#[derive(Debug)]
struct Args {
    input: Vec<String>,
    gdbport: Option<u16>,
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} IMGFILE [options]", program);
    print!("{}", opts.usage(&brief));
    process::exit(0);
}

fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("s", "gdb", "set gdb tcp port", "1234");
    opts.optflag("h", "help", "print this help menu");

    let matches = opts.parse(&args[1..])
    .unwrap_or_else(|f| panic!("{}", f.to_string()));

    if matches.opt_present("h") {
        print_usage(&program, &opts);
    }

    /*
    if matches.free.is_empty() {
        print_usage(&program, &opts);
    }
    */

    Args {
        input: matches.free.clone(),
        gdbport: matches.opt_get("s").unwrap(),
    }
}

fn main() {
    logger::init();
    let args = parse_args();

    let hw  = hardware::Hardware::new(0x400*0x400);
    let dev = device::Device::new();
    let mut emu = emulator::Emulator::new(hw, dev);

    emu.map_binary(0xffff0, include_bytes!("bios/crt0.bin")).expect("Failed to map");
    emu.map_binary(0xf0000, include_bytes!("bios/bios.bin")).expect("Failed to map");

    let img = if args.input.len() > 0 { args.input[0].clone() } else { "/tmp/test".to_string() };
    emu.load_binfile(0x7c00, img).expect("Failed to load binary");

    if let Some(p) = args.gdbport {
        let conn: Box<dyn Connection<Error = std::io::Error>> = Box::new(gdbserver::wait_for_tcp(p).expect("wait error"));
        let mut debugger = GdbStub::new(conn);

        debugger.run(&mut emu).expect("debugger error");
    } else {
        emu.run();
    }
}