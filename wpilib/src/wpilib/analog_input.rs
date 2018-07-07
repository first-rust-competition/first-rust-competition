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

use hal::*;
use std::{thread, time};
use wpilib::sensor_util;

/// An analog input on the RoboRIO
pub struct AnalogInput {
    channel: i32,
    port: HAL_AnalogInputHandle,
    accumulator_offset: i64,
}

impl AnalogInput {
    /// Create a new analog input on the specified channel, returning an error if initialization
    /// fails.
    ///
    /// # Errors
    /// Returns `Err(HalError(0))` if the channel is invalid.
    pub fn new(channel: i32) -> HalResult<AnalogInput> {
        if !sensor_util::check_analog_input_channel(channel) {
            return Err(HalError(0));
        }

        let port = hal_call!(HAL_InitializeAnalogInputPort(HAL_GetPort(channel)))?;

        report_usage(resource_type!(AnalogChannel), channel as u32);

        Ok(AnalogInput {
            channel: channel,
            port,
            accumulator_offset: 0,
        })
    }

    /// Read a value from the analog input.
    pub fn get_value(&self) -> HalResult<i32> {
        hal_call!(HAL_GetAnalogValue(self.port))
    }

    /// Read the average value of the analog input over some defined time period.
    pub fn get_average_value(&self) -> HalResult<i32> {
        hal_call!(HAL_GetAnalogAverageValue(self.port))
    }

    /// Read the raw value of the analog input in volts.
    pub fn get_voltage(&self) -> HalResult<f64> {
        hal_call!(HAL_GetAnalogVoltage(self.port))
    }

    /// Read the average raw value of the analog input in volts over some defined time period.
    pub fn get_average_voltage(&self) -> HalResult<f64> {
        hal_call!(HAL_GetAnalogAverageVoltage(self.port))
    }

    /// Get the channel number for this analog input.
    pub fn get_channel(&self) -> i32 {
        self.channel
    }

    /// Set the number of bits to use in averaging. Averaging will sample 2^bits actual reads.
    pub fn set_average_bits(&mut self, bits: i32) -> HalResult<()> {
        hal_call!(HAL_SetAnalogAverageBits(self.port, bits))
    }

    /// Get the previously-set number of average bits.
    pub fn get_average_bits(&self) -> HalResult<i32> {
        hal_call!(HAL_GetAnalogAverageBits(self.port))
    }

    /// Set the number of bits to use in oversampling to improve resolution with a slower rate.
    /// Oversampling will use 2^bits actual reads.
    pub fn set_oversample_bits(&mut self, bits: i32) -> HalResult<()> {
        hal_call!(HAL_SetAnalogOversampleBits(self.port, bits))
    }

    /// Get the previously-set number of oversample bits.
    pub fn get_oversample_bits(&self) -> HalResult<i32> {
        hal_call!(HAL_GetAnalogOversampleBits(self.port))
    }

    /// Get the factory scaling LSB weight constant:
    /// voltage = ((lsb_weight * 1e-9) * raw) - (offset * 1e-9)
    pub fn get_lsb_weight(&self) -> HalResult<i32> {
        hal_call!(HAL_GetAnalogLSBWeight(self.port))
    }

    /// Get the factory scaling offset constant:
    /// voltage = ((lsb_weight * 1e-9) * raw) - (offset * 1e-9)
    pub fn get_offset(&self) -> HalResult<i32> {
        hal_call!(HAL_GetAnalogOffset(self.port))
    }

    /// Returns true if this analog input is attached to an accumulator
    pub fn is_accumulator_channel(&self) -> HalResult<bool> {
        Ok(hal_call!(HAL_IsAccumulatorChannel(self.port))? != 0)
    }

    /// Initialize an accumulator on this channel.
    pub fn init_accumulator(&mut self) -> HalResult<()> {
        hal_call!(HAL_InitAccumulator(self.port))
    }

    /// Set the offset for the accumulator.
    pub fn set_accumulator_offset(&mut self, value: i64) {
        self.accumulator_offset = value;
    }

    /// Reset the accumulator and wait for the next sample.
    /// This blocks until new values are potentially available.
    pub fn reset_accumulator(&mut self) -> HalResult<()> {
        hal_call!(HAL_ResetAccumulator(self.port))?;

        let sample_time = 1f64 / AnalogInput::get_sample_rate()?;
        let over_samples = 1 << self.get_oversample_bits()?;
        let average_samples = 1 << self.get_average_bits()?;
        thread::sleep(time::Duration::from_micros(
            1000 * 1000 * over_samples * average_samples * sample_time as u64,
        ));
        Ok(())
    }

    /// Set the center of the accumulator. This value will be subtracted from all accumulated
    /// reads.
    pub fn set_accumulator_center(&mut self, center: i32) -> HalResult<()> {
        hal_call!(HAL_SetAccumulatorCenter(self.port, center))
    }

    /// Set the deadband for the accumulator. Anything within `deadband` of the accumulator center
    /// will be ignored in the accumulator.
    pub fn set_accumulator_deadband(&self, deadband: i32) -> HalResult<()> {
        hal_call!(HAL_SetAccumulatorDeadband(self.port, deadband))
    }

    /// Get a value from the accumulator.
    pub fn get_accumulator_value(&self) -> HalResult<i64> {
        hal_call!(HAL_GetAccumulatorValue(self.port))
    }

    /// Get the number of accumulated values.
    pub fn get_accumulator_count(&self) -> HalResult<i64> {
        hal_call!(HAL_GetAccumulatorCount(self.port))
    }

    /// Read the accumulator's value and the count of samples at the same time.
    /// Returns a tuple of `(value, count)`.
    pub fn get_accumulator_output(&self) -> HalResult<(i64, i64)> {
        let value: i64 = 0;
        let count: i64 = 0;
        hal_call!(HAL_GetAccumulatorOutput(
            self.port,
            value as *mut i64,
            count as *mut i64
        ))?;
        Ok((value, count))
    }

    /// Set the sample rate for analog inputs.
    pub fn set_sample_rate(samples_per_second: f64) -> HalResult<()> {
        hal_call!(HAL_SetAnalogSampleRate(samples_per_second))
    }

    /// Get the sample rate for analog inputs.
    pub fn get_sample_rate() -> HalResult<f64> {
        hal_call!(HAL_GetAnalogSampleRate())
    }
}

impl Drop for AnalogInput {
    fn drop(&mut self) {
        unsafe {
            HAL_FreeAnalogInputPort(self.port);
        }
    }
}
