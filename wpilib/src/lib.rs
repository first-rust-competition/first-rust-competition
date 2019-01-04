// Copyright 2018 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate lazy_static;
extern crate wpilib_sys;

mod sensor_util;

mod analog_input;
pub use self::analog_input::AnalogInput;
mod pdp;
pub use self::pdp::PowerDistributionPanel;
pub mod dio;
pub mod ds;
pub mod encoder;
pub mod pneumatics;
pub mod pwm;
mod robot_base;
pub use self::robot_base::*;
pub mod serial;
pub mod spi;

pub use wpilib_sys::hal_call::{HalError, HalMaybe, HalResult};
