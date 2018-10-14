// This file is part of "first-rust-competition", which is free software: you
// can redistribute it and/or modify it under the terms of the GNU General
// Public License version 3 as published by the Free Software Foundation. See
// <https://www.gnu.org/licenses/> for a copy.

// Code in this module should minimize `unsafe` calls.
// Repeated patterns should be abstraced into the `hal` module
// and the items in the that module should be used as much as possible.

pub mod dio;
pub use self::dio::{DigitalInput, DigitalOutput};

mod robot_base;
pub use self::robot_base::*;

pub mod ds;

pub mod time;

pub mod joystick;

pub mod pneumatics;

mod pdp;
pub use self::pdp::*;

pub mod serial;

pub mod spi;

mod sensor_util;
