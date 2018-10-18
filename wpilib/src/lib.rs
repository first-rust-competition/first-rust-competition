// Copyright 2018 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(concat_idents)]

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
