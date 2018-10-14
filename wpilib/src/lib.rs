// This file is part of "first-rust-competition", which is free software: you
// can redistribute it and/or modify it under the terms of the GNU General
// Public License version 3 as published by the Free Software Foundation. See
// <https://www.gnu.org/licenses/> for a copy.

#![feature(concat_idents)]

#[macro_use]
extern crate lazy_static;

mod hal;
pub use hal::{report_usage, report_usage_extras, HalError, HalMaybe, HalResult};
mod wpilib;
pub use wpilib::*;
