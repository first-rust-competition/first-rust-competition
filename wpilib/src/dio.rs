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

use lazy_static::lazy_static;
use wpilib_sys::usage::{instances, resource_types};
use wpilib_sys::*;

lazy_static! {
    /// The number of DIOs on the RoboRIO.
    static ref NUM_DIGITAL_CHANNELS: i32 = unsafe { HAL_GetNumDigitalChannels() };
}
#[cfg(feature = "embedded-hal")]
use embedded_hal::digital::v2::OutputPin;

/// A digital output used to control lights, etc from the RoboRIO.
#[derive(Debug)]
pub struct DigitalOutput {
    channel: i32,
    handle: HAL_DigitalHandle,
}

impl DigitalOutput {
    /// Create a new digital output on the specificed channel, returning an error if initialization
    /// fails.
    pub fn new(channel: i32) -> HalResult<Self> {
        let handle = hal_call!(HAL_InitializeDIOPort(
            HAL_GetPort(channel),
            false as HAL_Bool // false for output
        ))?;

        usage::report(resource_types::DigitalOutput, channel as instances::Type);

        Ok(DigitalOutput { channel, handle })
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
    pub fn channel(&self) -> i32 {
        self.channel
    }

    /// Get a handle to this DIO.
    pub fn handle(&self) -> HAL_DigitalHandle {
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
    pub fn enable_pwm(self, initial_duty_cycle: f64) -> HalResult<DigitalPwm> {
        let pwm = hal_call!(HAL_AllocateDigitalPWM())?;
        hal_call!(HAL_SetDigitalPWMDutyCycle(pwm, initial_duty_cycle))?;
        hal_call!(HAL_SetDigitalPWMOutputChannel(pwm, self.channel))?;
        Ok(DigitalPwm {
            handle: DigitalPwmHandle(pwm),
            pin: self,
        })
    }
}

impl Drop for DigitalOutput {
    fn drop(&mut self) {
        unsafe { HAL_FreeDIOPort(self.handle) }
    }
}

#[cfg(feature = "embedded-hal")]
impl OutputPin for DigitalOutput {
    type Error = HalError;

    fn set_low(&mut self) -> HalResult<()> {
        Ok(self.set(false)?)
    }

    fn set_high(&mut self) -> HalResult<()> {
        Ok(self.set(true)?)
    }
}

/// Digital PWM output.
///
/// PWM output will be disabled when this is dropped.
///
/// Use [`DigitalOutput::enable_pwm`] to get an instance of this.
///
/// [`DigitalOutput::enable_pwm`]: struct.DigitalOutput.html#method.enable_pwm
pub struct DigitalPwm {
    // this is ordered for drop order correctness
    handle: DigitalPwmHandle,
    pin: DigitalOutput,
}

/// Internal wrapper for a digital PWM handle with our Drop impl.
struct DigitalPwmHandle(HAL_DigitalPWMHandle);

impl DigitalPwm {
    /// Set a new duty cycle to use in PWM on this output.
    pub fn update_duty_cycle(&mut self, duty_cycle: f64) -> HalResult<()> {
        hal_call!(HAL_SetDigitalPWMDutyCycle(self.handle.0, duty_cycle))
    }

    /// Set the PWM rate for digital PWM outputs, from 0.6Hz to 19kHz.
    /// All digital channels will use the same PWM rate.
    /// Will return an error if PWM has not been enabled.
    pub fn set_pwm_rate(rate: f64) -> HalResult<()> {
        hal_call!(HAL_SetDigitalPWMRate(rate))
    }

    /// Get a reference to the underlying DigitalOutput.
    pub fn pin(&self) -> &DigitalOutput {
        &self.pin
    }

    /// Disables PWM output, returning the underlying DigitalOutput.
    pub fn disable(self) -> DigitalOutput {
        self.pin
    }
}

impl Drop for DigitalPwmHandle {
    fn drop(&mut self) {
        let _ = hal_call!(HAL_SetDigitalPWMOutputChannel(
            self.0,
            *NUM_DIGITAL_CHANNELS,
        ));
        let _ = hal_call!(HAL_FreeDigitalPWM(self.0));
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
#[derive(Debug)]
pub struct DigitalInput {
    channel: i32,
    handle: HAL_DigitalHandle,
}

// TODO: implement the rest of the methods
impl DigitalInput {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(channel: i32) -> HalResult<Self> {
        let handle = hal_call!(HAL_InitializeDIOPort(
            HAL_GetPort(channel as i32),
            true as HAL_Bool // true for input
        ))?;

        usage::report(resource_types::DigitalInput, channel as instances::Type);

        Ok(DigitalInput { channel, handle })
    }

    /// Get the value from the digital input channel from the FPGA.
    pub fn get(&self) -> HalResult<bool> {
        Ok(hal_call!(HAL_GetDIO(self.handle))? != 0)
    }

    pub fn handle(&self) -> HAL_DigitalHandle {
        self.handle
    }

    pub fn channel(&self) -> i32 {
        self.channel
    }
}

impl Drop for DigitalInput {
    fn drop(&mut self) {
        unsafe { HAL_FreeDIOPort(self.handle) }
    }
}
