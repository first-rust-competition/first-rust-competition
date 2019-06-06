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

use wpilib_sys::*;

/// An interface to the PDP for getting information about robot power.
#[derive(Debug)]
pub struct PowerDistributionPanel {
    handle: HAL_PDPHandle,
}

impl PowerDistributionPanel {
    /// Creates a PDP with the default ID of 0.
    pub fn new() -> HalResult<Self> {
        Self::with_id(0)
    }

    /// Creates a new PDP with a specified ID.
    pub fn with_id(module: i32) -> HalResult<Self> {
        let handle = hal_call!(HAL_InitializePDP(module))?;
        usage::report(usage::resource_types::PDP, module as _);
        Ok(PowerDistributionPanel { handle })
    }

    /// Get the voltage going into the PDP.
    pub fn voltage(&self) -> HalMaybe<f64> {
        maybe_hal_call!(HAL_GetPDPVoltage(self.handle))
    }

    /// Get the PDP's temperature, in degrees Celsius.
    /// # Errors
    /// The `HalMaybe` returned will have an error most commonly
    /// in the case of a CAN timeout. (In Fact, this is the only
    /// error WPILib will ever report!).
    pub fn temperature(&self) -> HalMaybe<f64> {
        maybe_hal_call!(HAL_GetPDPTemperature(self.handle))
    }

    /// Get the current on a specific channel on the PDP, in amps.
    ///
    /// # Errrors
    /// If `channel` is invalid, the return value will contain
    /// `NAN` and `HalError(0).
    ///
    /// The `HalMaybe` returned will have an error most commonly
    /// in the case of a CAN timeout. (In Fact, this is the only
    /// error WPILib will ever report!).
    pub fn current(&self, channel: i32) -> HalMaybe<f64> {
        maybe_hal_call!(HAL_GetPDPChannelCurrent(self.handle, channel))
    }

    /// Get the total current drawn from the PDP, in amps.
    /// # Errors
    /// The `HalMaybe` returned will have an error most commonly
    /// in the case of a CAN timeout. (In Fact, this is the only
    /// error WPILib will ever report!).
    pub fn total_current(&self) -> HalMaybe<f64> {
        maybe_hal_call!(HAL_GetPDPTotalCurrent(self.handle))
    }

    /// Get the total power drawn from the PDP, in watts.
    /// # Errors
    /// The `HalMaybe` returned will have an error most commonly
    /// in the case of a CAN timeout. (In Fact, this is the only
    /// error WPILib will ever report!).
    pub fn total_power(&self) -> HalMaybe<f64> {
        maybe_hal_call!(HAL_GetPDPTotalPower(self.handle))
    }

    /// Get the total energy expended by the PDP, in joules.
    /// # Errors
    /// The `HalMaybe` returned will have an error most commonly
    /// in the case of a CAN timeout. (In Fact, this is the only
    /// error WPILib will ever report!).
    pub fn total_energy(&self) -> HalMaybe<f64> {
        maybe_hal_call!(HAL_GetPDPTotalEnergy(self.handle))
    }

    /// Reset the total energy count so far to zero.
    ///
    /// # Errors
    /// Errors in the case of a CAN timeout. (In Fact, this
    /// is the only error WPILib will ever report!).
    pub fn reset_total_energy(&self) -> HalResult<()> {
        hal_call!(HAL_ResetPDPTotalEnergy(self.handle))
    }

    /// Clear sticky faults in the PDP.
    /// # Errors
    /// Errors in the case of a CAN timeout. (In Fact, this
    /// is the only error WPILib will ever report!).
    pub fn clear_sticky_faults(&self) -> HalResult<()> {
        hal_call!(HAL_ClearPDPStickyFaults(self.handle))
    }
}
