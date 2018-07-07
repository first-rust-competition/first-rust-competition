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

use super::sensor_util;
use hal::*;
use std::f64::NAN;

/// An interface to the PDP for getting information about robot power.
pub struct PowerDistributionPanel {
    handle: HAL_PDPHandle,
}

impl PowerDistributionPanel {
    /// Initalizes a PDP using the default module, which is 0, according to WPILibC.
    pub fn new() -> HalResult<PowerDistributionPanel> {
        Self::new_with_module(0)
    }

    /// Create a new PDP interface on the specified module.
    pub fn new_with_module(module: i32) -> HalResult<PowerDistributionPanel> {
        let handle = hal_call!(HAL_InitializePDP(module))?;
        report_usage(resource_type!(PDP), module as UsageResourceInstance);
        Ok(PowerDistributionPanel { handle })
    }

    /// Get the voltage going into the PDP.
    pub fn get_voltage(&self) -> HalMaybe<f64> {
        maybe_hal_call!(HAL_GetPDPVoltage(self.handle))
    }

    /// Get the PDP's temperature, in degrees Celsius.
    /// # Errors
    /// The `HalMaybe` returned will have an error most commonly
    /// in the case of a CAN timeout. (In Fact, this is the only
    /// error WPILib will ever report!).
    pub fn get_temperature(&self) -> HalMaybe<f64> {
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
    pub fn get_current(&self, channel: i32) -> HalMaybe<f64> {
        if !sensor_util::check_pdp_channel(channel) {
            return HalMaybe::new(NAN, Some(HalError(0)));
        }

        maybe_hal_call!(HAL_GetPDPChannelCurrent(self.handle, channel))
    }

    /// Get the total current drawn from the PDP, in amps.
    /// # Errors
    /// The `HalMaybe` returned will have an error most commonly
    /// in the case of a CAN timeout. (In Fact, this is the only
    /// error WPILib will ever report!).
    pub fn get_total_current(&self) -> HalMaybe<f64> {
        maybe_hal_call!(HAL_GetPDPTotalCurrent(self.handle))
    }

    /// Get the total power drawn from the PDP, in watts.
    /// # Errors
    /// The `HalMaybe` returned will have an error most commonly
    /// in the case of a CAN timeout. (In Fact, this is the only
    /// error WPILib will ever report!).
    pub fn get_total_power(&self) -> HalMaybe<f64> {
        maybe_hal_call!(HAL_GetPDPTotalPower(self.handle))
    }

    /// Get the total energy expended by the PDP, in joules.
    /// # Errors
    /// The `HalMaybe` returned will have an error most commonly
    /// in the case of a CAN timeout. (In Fact, this is the only
    /// error WPILib will ever report!).
    pub fn get_total_energy(&self) -> HalMaybe<f64> {
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
