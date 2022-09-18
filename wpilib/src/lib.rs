// Copyright 2018 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod analog_input;
pub use self::analog_input::AnalogInput;
mod pdp;
pub use self::pdp::PowerDistributionPanel;
pub mod can;
pub use self::can::Can;
pub mod dio;
pub mod ds;
pub mod encoder;
pub mod i2c;
pub mod notifier;
pub mod observe;
pub mod pneumatics;
pub mod pwm;
pub mod relay;
mod robot;
pub use self::robot::*;
mod robot_base;
pub use self::robot_base::*;
pub mod serial;
pub mod spi;

pub use wpilib_sys::hal_call::{HalError, HalMaybe, HalResult};
