#![feature(concat_idents)]

#[macro_use]
extern crate lazy_static;

mod hal;
mod wpilib;
pub use wpilib::*;
