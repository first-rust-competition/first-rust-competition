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

#![macro_use]
use super::bindings::HAL_Report;
use std::ffi::CStr;
use std::ptr;

pub use super::bindings::HALUsageReporting_tInstances as instances;
pub use super::bindings::HALUsageReporting_tResourceType as resource_types;

/// Wraps the ugly type rust-bindgen generates for usage reporting types.
pub type UsageResourceType = resource_types::Type;

/// Wraps the ugly type rust-bindgen generates for usage reporting instances.
pub type UsageResourceInstance = instances::Type;

/// A utility macro for referencing rust-bindgen's generated names for usage types.
///
/// Preserved for backwards compatibility. Users are recommended to
/// reference `resource_types` directly.
///
/// ```
/// assert_eq!(resource_types::DigitalOutput, resource_type!(DigitalOutput));
/// ```
#[macro_export]
macro_rules! resource_type {
    ($resource_name:ident) => {
        $crate::bindings::HALUsageReporting_tResourceType::$resource_name
    };
}

/// A utility macro for referencing rust-bindgen's generated names for usage instances.
///
/// ```
/// assert_eq!(instances::kLanguage_CPlusPlus, resource_instance!(Language, CPlusPlus));
/// ```
///
/// This currently requires the `concat_idents` feature.
#[macro_export]
macro_rules! resource_instance {
    ($resource_name:ident, $instance_name:ident) => {{
        use $crate::bindings::HALUsageReporting_tInstances::*;
        concat_idents!(k, $resource_name, _, $instance_name)
    }};
}

/// Report the usage of a specific resource type with an `instance` value attached.
///
/// This is provided as a utility for library developers.
pub fn report_usage(resource: UsageResourceType, instance: UsageResourceInstance) -> i64 {
    report_usage_context(resource, instance, 0)
}

/// Report usage of a resource with additional context.
///
/// This is provided as a utility for library developers.
pub fn report_usage_context(
    resource: UsageResourceType,
    instance: UsageResourceInstance,
    context: i32,
) -> i64 {
    unsafe { HAL_Report(resource as i32, instance as i32, context, ptr::null()) }
}

/// This is provided as a utility for library developers.
/// Designed to be used with null-terminated byte string literals like `b"message\0"`
///
/// # Panics
/// If the underlying byte slice is not null-terminated, the function will panic
pub fn report_usage_extras<F: AsRef<[u8]>>(
    resource: UsageResourceType,
    instance: UsageResourceInstance,
    context: i32,
    feature: F,
) -> i64 {
    // local binding just to be safe with lifetimes, see https://doc.rust-lang.org/std/ffi/struct.CStr.html#method.as_ptr
    let cstr = CStr::from_bytes_with_nul(feature.as_ref())
        .expect("report_usage_extras features must be null-terminated!");
    unsafe { HAL_Report(resource as i32, instance as i32, context, cstr.as_ptr()) }
}
