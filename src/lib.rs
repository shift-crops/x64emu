#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate log;

pub mod hardware;
pub mod device;
pub mod emulator;
pub mod interface;