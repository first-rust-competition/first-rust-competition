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

This file is part of "first-rust-competition", which is free software: you can
redistribute it and/or modify it under the terms of the GNU General Public
License version 3 as published by the Free Software Foundation. See
<https://www.gnu.org/licenses/> for a copy.
*/

#![macro_use]
use super::bindings::nUsageReporting_tInstances;
use super::bindings::nUsageReporting_tResourceType;
use super::bindings::HAL_Report;
use std::os::raw;
use std::ptr;

/// Wraps the ugly type rust-bindgen generates for usage reporting types.
pub type UsageResourceType = nUsageReporting_tResourceType;

/// Wraps the ugly type rust-bindgen generates for usage reporting instances.
pub type UsageResourceInstance = nUsageReporting_tInstances;

/// A utility macro for referencing rust-bindgen's generated names for usage types.
/// Currently, the identifier for a digital output is
/// `nUsageReporting_tResourceType_kResourceType_DigitalOutput`.
/// This is equivalent to `resource_type!(DigitalOutput)`.
macro_rules! resource_type {
    ($resource_name:ident) => {
        concat_idents!(nUsageReporting_tResourceType_kResourceType_, $resource_name)
    };
}

/// A utility macro for referencing rust-bindgen's generated names for usage instances.
/// Currently, the identifier for the C++ language is
/// `nUsageReporting_tInstances_kLanguage_CPlusPlus`.
/// This is equivalent to `resource_instance!(Language, CPlusPLus)`.
#[allow(unused_macros)] // it'll probably be used at some point.
                        // TODO(Lytigas) examine what kind of values this points to
macro_rules! resource_instance {
    ($resource_name:ident, $instance_name:ident) => {
        concat_idents!(
            nUsageReporting_tInstances_k,
            $resource_name,
            _,
            $instance_name
        )
    };
}

/// Report the usage of a specific resource type with an `instance` value attached.
///
/// This is provided as a utility for library developers.
pub fn report_usage(resource: UsageResourceType, instance: UsageResourceInstance) {
    unsafe {
        HAL_Report(resource as i32, instance as i32, 0, ptr::null());
    }
}

/// This is provided as a utility for library developers.
/// Pass `ptr::null()` for `feature` to exclude it.
pub unsafe fn report_usage_extras(
    resource: UsageResourceType,
    instance: UsageResourceInstance,
    context: i32,
    feature: *const raw::c_char,
) {
    HAL_Report(resource as i32, instance as i32, context, feature);
}
