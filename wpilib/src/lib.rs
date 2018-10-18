// This file is part of "first-rust-competition", which is free software: you
// can redistribute it and/or modify it under the terms of the GNU General
// Public License version 3 as published by the Free Software Foundation. See
// <https://www.gnu.org/licenses/> for a copy.

#![feature(concat_idents)]
#![feature(tool_lints)]

#[macro_use]
extern crate lazy_static;
extern crate wpilib_sys;

mod pdp;
mod robot_base;
mod sensor_util;

pub mod dio;
pub mod ds;
pub mod joystick;
pub mod pneumatics;
pub mod serial;
pub mod spi;
pub mod time;

pub use self::dio::{DigitalInput, DigitalOutput};
pub use self::pdp::*;
pub use self::robot_base::*;

pub use wpilib_sys::hal_call::{HalError, HalMaybe, HalResult};
