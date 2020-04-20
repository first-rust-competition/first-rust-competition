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

//! Digital I/O.

use wpilib_sys::usage::{instances, resource_types};
use wpilib_sys::*;

#[cfg(feature = "embedded-hal")]
use embedded_hal::digital::v2::OutputPin;

/// The number of DIOs on the RoboRIO.
fn num_digital_channels() -> i32 {
    unsafe { HAL_GetNumDigitalChannels() }
}

#[derive(Debug)]
/// A DIO pin.
pub struct DigitalPin<MODE> {
    channel: i32,
    handle: HAL_DigitalHandle,
    mode: MODE,
}

#[derive(Debug)]
pub struct Input;
#[derive(Debug)]
pub struct Output;

const IS_INPUT: bool = true;

/**
 * Class to read a digital input.
 *
 * This class will read digital inputs and return the current value on the
 * channel. Other devices such as encoders, gear tooth sensors, etc. that are
 * implemented elsewhere will automatically allocate digital inputs and outputs
 * as required. This class is only for devices like switches etc. that aren't
 * implemented anywhere else.
 */
pub type DigitalInput = DigitalPin<Input>;
/// A digital output used to control lights, etc from the RoboRIO.
pub type DigitalOutput = DigitalPin<Output>;

// TODO: implement the rest of the methods
impl DigitalInput {
    /// Creates a new DIO pin in input mode.
    pub fn new(channel: i32) -> HalResult<Self> {
        let handle = hal_call!(HAL_InitializeDIOPort(
            HAL_GetPort(channel),
            IS_INPUT as HAL_Bool
        ))?;

        usage::report(resource_types::DigitalInput, channel as instances::Type);

        Ok(DigitalPin {
            channel,
            handle,
            mode: Input,
        })
    }
}

impl DigitalOutput {
    /// Creates a new DIO pin in output mode.
    pub fn new(channel: i32) -> HalResult<Self> {
        let handle = hal_call!(HAL_InitializeDIOPort(
            HAL_GetPort(channel),
            !IS_INPUT as HAL_Bool
        ))?;

        usage::report(resource_types::DigitalOutput, channel as instances::Type);

        Ok(DigitalPin {
            channel,
            handle,
            mode: Output,
        })
    }

    /// Set the value to output.
    pub fn set(&mut self, value: bool) -> HalResult<()> {
        hal_call!(HAL_SetDIO(self.handle, value as HAL_Bool))
    }
}

/// Methods common to both input and output modes.
impl<MODE> DigitalPin<MODE> {
    /// Get the current value of the pin.
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
}

/// Output mode methods.
impl DigitalOutput {
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

impl<MODE> Drop for DigitalPin<MODE> {
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
            num_digital_channels(),
        ));
        let _ = hal_call!(HAL_FreeDigitalPWM(self.0));
    }
}
