/* PORTIONS OF THIS FILE WERE ORIGINALLY DISTRIBUTED WITH THE FOLLOWING LICENSE

"""
MIT License
Copyright (c) 2017 Rust for Robotics Developers
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"""

Copyright 2018 First Rust Competition Developers.
Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
option. This file may not be copied, modified, or distributed
except according to those terms.
*/

//! Lightweight wrappers around usage reporting.

use super::bindings::HAL_Report;
use std::ffi::CStr;
use std::ptr;

#[allow(non_upper_case_globals)]
pub mod instances;
#[allow(non_upper_case_globals)]
pub mod resource_types;

/// Report the usage of a specific resource type with an `instance` value attached.
///
/// This is provided as a utility for library developers.
pub fn report(resource: resource_types::Type, instance: instances::Type) -> i64 {
    report_context(resource, instance, 0)
}

/// Report usage of a resource with additional context.
///
/// This is provided as a utility for library developers.
pub fn report_context(
    resource: resource_types::Type,
    instance: instances::Type,
    context: i32,
) -> i64 {
    unsafe { HAL_Report(resource, instance, context, ptr::null()) }
}

/// Report usage of a resource with context and a feature string.
///
/// This is provided as a utility for library developers.
pub fn report_feature(
    resource: resource_types::Type,
    instance: instances::Type,
    context: i32,
    feature: impl AsRef<CStr>,
) -> i64 {
    unsafe { HAL_Report(resource, instance, context, feature.as_ref().as_ptr()) }
}
