// Copyright 2018 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
