// This file is part of "first-rust-competition", which is free software: you
// can redistribute it and/or modify it under the terms of the GNU General
// Public License version 3 as published by the Free Software Foundation. See
// <https://www.gnu.org/licenses/> for a copy.

#![feature(concat_idents)]
#![macro_use]
extern crate lazy_static;
extern crate wpilib_sys;

mod robot_base;
mod pdp;
mod sensor_util;

pub mod dio;
pub mod ds;
pub mod time;
pub mod joystick;
pub mod pneumatics;
pub mod serial;
pub mod spi;

pub use self::dio::{DigitalInput, DigitalOutput};
pub use self::robot_base::*;
pub use self::pdp::*;

pub use wpilib_sys::bindings::*;
pub use wpilib_sys::hal_call::{HalMaybe, HalError, HalResult};
pub use wpilib_sys::usage::{report_usage, report_usage_extras};