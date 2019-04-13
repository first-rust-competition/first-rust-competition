// Copyright 2018 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate wpilib_sys;

use wpilib_sys::usage::*;

struct UsageReported {}

impl UsageReported {
    pub fn new() -> Self {
        report(resource_types::Language, instances::kLanguage_CPlusPlus);
        report_context(666, 0, 123);
        report_extras(9998, 9998, 9997, b"FEATURE\0");
        Self {}
    }
}

fn main() {
    let _usage = UsageReported::new();
    println!("Usage reporting is visible!");
}
