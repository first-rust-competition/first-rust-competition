//! An interface for the RoboRIO's built-in FPGA.

use wpilib_sys::{bindings::HAL_GetFPGATime, hal_call, hal_call::HalError, HalResult};

pub struct FPGA;

impl FPGA {
    /// Read the microsecond-resolution timer on the FPGA.
    ///
    /// Returns The current time in microseconds according to the FPGA (since FPGA
    ///         reset).
    #[inline(always)]
    pub fn fpga_time() -> HalResult<u64> {
        hal_call!(HAL_GetFPGATime())
    }
}
