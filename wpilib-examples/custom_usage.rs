// Copyright 2018 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Required to use the resource_type! and resource_instance! macros.
#![feature(concat_idents)]

#[macro_use]
extern crate wpilib_sys;

// Pull in the usage reporting constants themselves.
use wpilib_sys::bindings::*;
use wpilib_sys::usage::*;

struct UsageReported {}

impl UsageReported {
    pub fn new() -> Self {
        report_usage(
            resource_type!(Language),
            resource_instance!(Language, CPlusPlus),
        );
        report_usage_context(666, 0, 123);
        report_usage_extras(9998, 9998, 9997, b"FEATURE");
        Self {}
    }
}

fn main() {
    let _usage = UsageReported::new();
    println!("Usage reporting is visible!");
}
