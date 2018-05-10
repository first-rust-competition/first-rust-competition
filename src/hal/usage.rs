use hal::tInstances;
use hal::tResourceType;
use hal::HAL_Report;
use std::os::raw;
use std::ptr;

/// Report the usage of a specific resource type with an `instance` value attached.
pub fn report_usage(resource: tResourceType, instance: tInstances) {
    unsafe {
        HAL_Report(resource as i32, instance as i32, 0, ptr::null());
    }
}

/// Just a safe wrapper around HAL_Report
pub fn report_usage_extras(
    resource: tResourceType,
    instance: tInstances,
    context: i32,
    feature: *const raw::c_char,
) {
    unsafe {
        HAL_Report(resource as i32, instance as i32, context, feature);
    }
}
