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
use super::bindings::HALUsageReporting_tInstances;
use super::bindings::HALUsageReporting_tResourceType;
use super::bindings::HAL_Report;
use std::ffi::CStr;
use std::ptr;

/// Wraps the ugly type rust-bindgen generates for usage reporting types.
pub type UsageResourceType = HALUsageReporting_tResourceType::Type;

/// Wraps the ugly type rust-bindgen generates for usage reporting instances.
pub type UsageResourceInstance = HALUsageReporting_tInstances::Type;

/// A utility macro for referencing rust-bindgen's generated names for usage types.
///
/// ```
/// assert_eq!(
///   HALUsageReporting_tResourceType::kResourceType_DigitalOutput,
///   resource_type!(DigitalOutput)
/// );
/// ```
///
/// This currently requires the `concat_idents` feature.
#[macro_export]
macro_rules! resource_type {
    ($resource_name:ident) => {{
        use $crate::bindings::HALUsageReporting_tResourceType::*;
        concat_idents!(kResourceType_, $resource_name)
    }};
}

/// A utility macro for referencing rust-bindgen's generated names for usage instances.
///
/// ```
/// assert_eq!(
///   HALUsageReporting_tInstances::kLanguage_CPlusPlus,
///   resource_instance!(Language, CPlusPlus)
/// );
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

/// Report usage of an additional feature.
///
/// This is provided as a utility for library developers.
pub fn report_usage_extras(
    resource: UsageResourceType,
    instance: UsageResourceInstance,
    context: i32,
    feature: CStr,
) -> i64 {
    unsafe { HAL_Report(resource as i32, instance as i32, context, feature.as_ptr()) }
}
