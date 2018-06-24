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

use hal::*;
use std::time::Duration;

/// Return the FPGA Version number.
///
/// For now, expect this to be competition year.
#[inline(always)]
pub fn get_fpga_version() -> HalResult<i32> {
    hal_call!(HAL_GetFPGAVersion())
}

/// Return the FPGA Revision number.
///
/// The format of the revision is 3 numbers. The 12 most significant bits are the
/// Major Revision. The next 8 bits are the Minor Revision. The 12 least
/// significant bits are the Build Number.
#[inline(always)]
pub fn get_fpga_revision() -> HalResult<i64> {
    hal_call!(HAL_GetFPGARevision())
}

/// Read the microsecond-resolution timer on the FPGA.
///
/// Returns The current time in microseconds according to the FPGA (since FPGA
///         reset).
#[inline(always)]
pub fn fpga_time() -> HalResult<u64> {
    hal_call!(HAL_GetFPGATime())
}

/// Read the microsecond-resolution timer
/// on the FPGA as a `std::time::Duration`.
pub fn fpga_time_duration() -> HalResult<Duration> {
    let usec = fpga_time()?;
    let sec: u64 = usec / 1_000_000;
    let nsec: u32 = (usec % 1_000_000) as u32 * 1000;
    Ok(Duration::new(sec, nsec))
}
/// Get the state of the "USER" button on the roboRIO.
///
/// True if the button is currently pressed.
#[inline(always)]
pub fn user_button() -> HalResult<bool> {
    Ok(hal_call!(HAL_GetFPGAButton())? != 0)
}
