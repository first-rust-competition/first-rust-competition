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

use super::bindings::*;
use std::{borrow::Cow, error::Error, ffi::CStr, fmt};

#[derive(Copy, Clone)]
pub struct HalError(pub i32);

impl HalError {
    /// Get the HAL error message associated with this error code.
    /// In traditional WPILib, this would be printed the the driver
    /// station whenever an error occured. The resulting string may
    /// not be valid UTF-8.
    pub fn message(&self) -> Cow<str> {
        let const_char_ptr = unsafe { HAL_GetErrorMessage(self.0) };
        let c_str = unsafe { CStr::from_ptr(const_char_ptr) };
        c_str.to_string_lossy()
    }
}

impl fmt::Debug for HalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "HalError {{ {} }}", self.message())
    }
}

impl fmt::Display for HalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: \"{}\"!", self.message())
    }
}

impl Error for HalError {
    fn description(&self) -> &str {
        "Error in the HAL"
    }
}

impl From<i32> for HalError {
    fn from(code: i32) -> Self {
        HalError(code)
    }
}

pub type HalResult<T> = Result<T, HalError>;

/// Represents the result of a function call that could error,
/// but even if it does, the result is still usable. Unfortunately,
/// the way the WPILib HAL handles things, this comes up a lot.
/// Like `Result`, `HalMaybe` must be used.
#[must_use]
pub struct HalMaybe<T> {
    ret: T,
    err: Option<HalError>,
}

impl<T> HalMaybe<T> {
    pub fn new(ret: T, err: Option<HalError>) -> HalMaybe<T> {
        HalMaybe { ret, err }
    }

    /// Ignore the possible error, and consume the `HalMaybe` into
    /// its return value.
    pub fn ok(self) -> T {
        self.ret
    }

    /// Returns true if there is an error
    pub fn has_err(&self) -> bool {
        match self.err {
            Some(_) => true,
            None => false,
        }
    }

    /// Access the potential error.
    pub fn err(&self) -> Option<HalError> {
        self.err
    }

    /// Convert the `HalMaybe` into a `Result`, discarding the return
    /// value if there is an error. This is useful for accessing
    /// the many methods on `Result`, including `?` error raising.
    pub fn into_res(self) -> Result<T, HalError> {
        if let Some(x) = self.err {
            Err(x)
        } else {
            Ok(self.ret)
        }
    }
}

/// Wraps a C/C++ HAL function call that looks like `T foo(arg1, arg2, arg3, ... , int32_t* status)
/// and turns that status into a `HALResult<T>`, with a non-zero status code returning in
/// the `Err` variant.
#[macro_export]
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
#[macro_export]
macro_rules! maybe_hal_call {
    ($function:ident($($arg:expr),*)) => {{
        let mut status = 0;
        let result = unsafe { $function($(
            $arg,
        )* &mut status as *mut i32) };
        HalMaybe::new(
            result,
            if status == 0 {
                None
            } else {
                Some(HalError::from(status))
            }
        )
    }};
    ($namespace:path, $function:ident($($arg:expr),*)) => {{
        let mut status = 0;
        let result = unsafe { $namespace::$function($(
            $arg,
        )* &mut status as *mut i32) };
        HalMaybe::new(
            result,
            if status == 0 {
                None
            } else {
                Some(HalError::from(status))
            }
        )
    }};
}
