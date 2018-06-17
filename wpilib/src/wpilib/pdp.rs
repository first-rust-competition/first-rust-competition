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

use super::sensor_util;
use hal::*;

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
    pub fn get_voltage(&self) -> HalResult<f64> {
        hal_call!(HAL_GetPDPVoltage(self.handle))
    }

    /// Get the PDP's temperature, in degrees Celsius.
    pub fn get_temperature(&self) -> HalResult<f64> {
        hal_call!(HAL_GetPDPTemperature(self.handle))
    }

    /// Get the current on a specific channel on the PDP, in amps.
    ///
    /// Returns `Err(HalError(0))` if `channel` is not a valid channel.
    pub fn get_current(&self, channel: i32) -> HalResult<f64> {
        if !sensor_util::check_pdp_channel(channel) {
            return Err(HalError(0));
        }

        hal_call!(HAL_GetPDPChannelCurrent(self.handle, channel))
    }

    /// Get the total current drawn from the PDP, in amps.
    pub fn get_total_current(&self) -> HalResult<f64> {
        hal_call!(HAL_GetPDPTotalCurrent(self.handle))
    }

    /// Get the total power drawn from the PDP, in watts.
    pub fn get_total_power(&self) -> HalResult<f64> {
        hal_call!(HAL_GetPDPTotalPower(self.handle))
    }

    /// Get the total energy expended by the PDP, in joules.
    pub fn get_total_energy(&self) -> HalResult<f64> {
        hal_call!(HAL_GetPDPTotalEnergy(self.handle))
    }

    /// Reset the total energy count so far to zero.
    pub fn reset_total_energy(&mut self) -> HalResult<()> {
        hal_call!(HAL_ResetPDPTotalEnergy(self.handle))
    }

    /// Clear sticky faults in the PDP.
    pub fn clear_sticky_faults(&mut self) -> HalResult<()> {
        hal_call!(HAL_ClearPDPStickyFaults(self.handle))
    }
}
