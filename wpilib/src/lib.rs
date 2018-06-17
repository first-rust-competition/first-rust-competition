#![feature(concat_idents)]

#[macro_use]
extern crate lazy_static;

mod hal;
pub use hal::{HalError, HalResult};
mod wpilib;
pub use wpilib::*;
