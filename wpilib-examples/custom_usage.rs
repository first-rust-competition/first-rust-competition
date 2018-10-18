// This file is part of "first-rust-competition", which is free software: you
// can redistribute it and/or modify it under the terms of the GNU General
// Public License version 3 as published by the Free Software Foundation. See
// <https://www.gnu.org/licenses/> for a copy.

extern crate wpilib;

extern crate wpilib_sys;

use wpilib_sys::usage::*;

struct UsageReported {}

impl UsageReported {
    pub fn new() -> Self {
        report_usage(9999, 9999);
        unsafe { report_usage_extras(9998, 9998, 9997, b"FEATURE".as_ptr()) };
        Self {}
    }
}

fn main() {
    let _usage = UsageReported::new();
    println!("Usage reporting is visible!");
}
