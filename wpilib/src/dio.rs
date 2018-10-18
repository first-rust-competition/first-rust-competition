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
use wpilib_sys::*;

/// A digital output used to control lights, etc from the RoboRIO.
#[allow(dead_code)]
pub struct DigitalOutput {
    channel: i32,
    handle: HAL_DigitalHandle,
    pwm: Option<HAL_DigitalPWMHandle>,
}

impl DigitalOutput {
    /// Create a new digital output on the specificed channel, returning an error if initialization
    /// fails.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(channel: i32) -> HalResult<Self> {
        if !sensor_util::check_digital_channel(channel) {
            return Err(HalError(0));
        }

        let handle = hal_call!(HAL_InitializeDIOPort(
            HAL_GetPort(channel as i32),
            false as i32 // for input?
        ))?;

        report_usage(
            resource_type!(DigitalOutput),
            channel as UsageResourceInstance,
        );

        Ok(DigitalOutput {
            channel,
            handle,
            pwm: None,
        })
    }

    /// Set the PWM rate for this output, from 0.6Hz to 19kHz. Will return an error if PWM has not
    /// been enabled. All digital channels will use the same PWM rate.
    pub fn set_pwm_rate(rate: f64) -> HalResult<()> {
        hal_call!(HAL_SetDigitalPWMRate(rate))
    }

    /// Set the value to output.
    pub fn set(&mut self, value: bool) -> HalResult<()> {
        hal_call!(HAL_SetDIO(self.handle, value as i32))
    }

    /// Get the previously-written output.
    pub fn get(&self) -> HalResult<bool> {
        Ok(hal_call!(HAL_GetDIO(self.handle))? != 0)
    }

    /// Get the channel for this DIO.
    pub fn get_channel(&self) -> i32 {
        self.channel
    }

    /// Get a handle to this DIO.
    pub fn get_handle(&self) -> HAL_DigitalHandle {
        self.handle
    }

    /// Write a pulse to this output.
    pub fn pulse(&mut self, length: f64) -> HalResult<()> {
        hal_call!(HAL_Pulse(self.handle, length))
    }

    /// Is this output currently in the middle of a pulse?
    pub fn is_pulsing(&self) -> HalResult<bool> {
        Ok(hal_call!(HAL_IsPulsing(self.handle))? != 0)
    }

    /// Enable PWM for this output.
    pub fn enable_pwm(&mut self, initial_duty_cycle: f64) -> HalResult<()> {
        let pwm = hal_call!(HAL_AllocateDigitalPWM())?;
        hal_call!(HAL_SetDigitalPWMDutyCycle(pwm, initial_duty_cycle))?;
        hal_call!(HAL_SetDigitalPWMOutputChannel(pwm, self.channel))?;
        self.pwm = Some(pwm);
        Ok(())
    }

    /// Turn off PWM for this output.
    pub fn disable_pwm(&mut self) -> HalResult<()> {
        if let Some(pwm) = self.pwm {
            hal_call!(HAL_SetDigitalPWMOutputChannel(
                pwm,
                *sensor_util::NUM_DIGITAL_CHANNELS
            ))?;
            hal_call!(HAL_FreeDigitalPWM(pwm))?;
            self.pwm = None;
        }
        Ok(())
    }

    /// Set a new duty cycle to use in PWM on this output.
    pub fn update_duty_cycle(&mut self, duty_cycle: f64) -> HalResult<()> {
        if let Some(pwm) = self.pwm {
            hal_call!(HAL_SetDigitalPWMDutyCycle(pwm, duty_cycle))
        } else {
            Ok(())
        }
    }
}

impl Drop for DigitalOutput {
    fn drop(&mut self) {
        let _ = self.disable_pwm();
        unsafe {
            HAL_FreeDIOPort(self.handle);
        }
    }
}

/**
 * Class to read a digital input.
 *
 * This class will read digital inputs and return the current value on the
 * channel. Other devices such as encoders, gear tooth sensors, etc. that are
 * implemented elsewhere will automatically allocate digital inputs and outputs
 * as required. This class is only for devices like switches etc. that aren't
 * implemented anywhere else.
 */
#[allow(dead_code)]
pub struct DigitalInput {
    channel: i32,
    handle: HAL_DigitalHandle,
}

// TODO: implement the rest of the methods
impl DigitalInput {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(channel: i32) -> HalResult<Self> {
        if !sensor_util::check_digital_channel(channel) {
            return Err(HalError(0));
        }

        let handle = hal_call!(HAL_InitializeDIOPort(
            HAL_GetPort(channel as i32),
            true as i32 // for input?
        ))?;

        report_usage(
            resource_type!(DigitalInput),
            channel as UsageResourceInstance,
        );

        Ok(DigitalInput { channel, handle })
    }

    /// Get the value from the digital input channel from the FPGA.
    pub fn get(&self) -> HalResult<bool> {
        Ok(hal_call!(HAL_GetDIO(self.handle))? != 0)
    }

    pub fn get_channel(&self) -> i32 {
        self.channel
    }
}

impl Drop for DigitalInput {
    fn drop(&mut self) {
        unsafe {
            HAL_FreeDIOPort(self.handle);
        }
    }
}
