/* THE ORIGINAL VERSION OF THIS FILE WAS DISTRIBUTED WITH THE FOLLOWING LICENSE

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

THE CURRENT FORM OF THIS FILE IS LICENSED UNDER THE SAME TERMS AS THE REST OF
THIS REPOSITORY. SEE THE LICENSE FILE FOR FULL TERMS.
*/

#![macro_use]

use super::bindings::*;
use std::{ffi, fmt};

#[derive(Copy, Clone)]
pub struct HalError(pub i32);

impl From<i32> for HalError {
    fn from(code: i32) -> Self {
        HalError(code)
    }
}

impl fmt::Debug for HalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let error_string = unsafe { ffi::CStr::from_ptr(HAL_GetErrorMessage(self.0)) };
        write!(f, "HalError {{ {} }}", error_string.to_str().unwrap())
    }
}

pub type HalResult<T> = Result<T, HalError>;

/// Wraps a C/C++ HAL function call that looks like `T foo(arg1, arg2, arg3, ... , int32_t* status)
/// and turns that status into a `HALResult<T>`, with a non-zero status code returning in
/// the `Err` variant.
macro_rules! hal_call {
    ($function:ident($($arg:expr),*)) => {{
        let mut status = 0;
        let result = unsafe { $function($(
            $arg,
        )* &mut status as *mut i32) };
        if status == 0 { Ok(result) } else { Err(HalError::from(status)) }
    }};
    ($namespace:path, $function:ident($($arg:expr),*)) => {{
        let mut status = 0;
        let result = unsafe { $namespace::$function($(
            $arg,
        )* &mut status as *mut i32) };
        if status == 0 { Ok(result) } else { Err(HalError::from(status)) }
    }};
}

/// Like `hal_call!`, but ignores the status code and returns the functions result anyway.
/// This sounds bad, but WPILibC does it in some places, and there isn't really a reason to
/// needlessly complicate the public interface.
macro_rules! ok_hal_call {
    ($function:ident($($arg:expr),*)) => {{
        let mut status = 0;
        unsafe { $function($(
            $arg,
        )* &mut status as *mut i32) }
    }};
    ($namespace:path, $function:ident($($arg:expr),*)) => {{
        let mut status = 0;
        unsafe { $namespace::$function($(
            $arg,
        )* &mut status as *mut i32) }
    }};
}
