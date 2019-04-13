// Copyright 2018 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use sensor_util;
use wpilib_sys::usage::{instances, resource_types};
use wpilib_sys::*;

/// Represents the amount to multiply the minimum servo-pulse pwm period by.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PeriodMultiplier {
    /// Don't skip pulses. PWM pulses occur every 5.005 ms
    Multiplier1x = 0,
    /// Skip every other pulse. PWM pulses occur every 10.010 ms
    Multiplier2x = 1,
    /// Skip three out of four pulses. PWM pulses occur every 20.020 ms
    Multiplier4x = 3,
}

#[derive(Debug)]
pub struct PWM {
    channel: i32,
    handle: HAL_DigitalHandle,
}

impl PWM {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(channel: i32) -> HalResult<Self> {
        if !sensor_util::check_pwm_channel(channel) {
            return Err(HalError(0));
        }

        let handle = hal_call!(HAL_InitializePWMPort(HAL_GetPort(channel)))?;

        hal_call!(HAL_SetPWMDisabled(handle))?;
        hal_call!(HAL_SetPWMEliminateDeadband(handle, false as i32))?;

        usage::report(resource_types::PWM, channel as instances::Type);

        Ok(PWM { channel, handle })
    }

    /// Set the PWM value directly to the hardware.
    pub fn set_raw(&mut self, value: i32) -> HalResult<()> {
        hal_call!(HAL_SetPWMRaw(self.handle, value))
    }

    /// Get the PWM value directly from the hardware.
    pub fn raw(&self) -> HalResult<i32> {
        hal_call!(HAL_GetPWMRaw(self.handle))
    }

    /// Set the PWM value based on a position. `pos` must be between 0 and 1.
    pub fn set_position(&mut self, pos: f64) -> HalResult<()> {
        hal_call!(HAL_SetPWMPosition(self.handle, pos))
    }

    /// Get the PWM value in terms of a position.
    pub fn position(&self) -> HalResult<f64> {
        hal_call!(HAL_GetPWMPosition(self.handle))
    }

    /// Set the PWM value based on a speed between -1 and 1.
    pub fn set_speed(&mut self, speed: f64) -> HalResult<()> {
        hal_call!(HAL_SetPWMSpeed(self.handle, speed))
    }

    /// Get the PWM value in terms of speed.
    pub fn speed(&self) -> HalResult<f64> {
        hal_call!(HAL_GetPWMSpeed(self.handle))
    }

    /// Temporarily disables the PWM output. The next set call will re-enable.
    pub fn set_disabled(&mut self) -> HalResult<()> {
        hal_call!(HAL_SetPWMDisabled(self.handle))
    }

    /// Slow down the PWM signal for old devices.
    pub fn set_period_multiplier(&mut self, mult: PeriodMultiplier) -> HalResult<()> {
        hal_call!(HAL_SetPWMPeriodScale(self.handle, mult as i32))
    }

    /// Honestly, I have no idea what this does and it isn't documented in wpilib.
    pub fn set_zero_latch(&mut self) -> HalResult<()> {
        hal_call!(HAL_LatchPWMZero(self.handle))
    }

    /// Optionally eliminate the deadband from a speed controller.
    pub fn enable_deadband_elimination(&mut self, eliminate_deadband: bool) -> HalResult<()> {
        hal_call!(HAL_SetPWMEliminateDeadband(
            self.handle,
            eliminate_deadband as i32
        ))
    }

    /// Set the bounds on the PWM pulse widths. This sets the bounds on the PWM values for a
    /// particular type of controller. The values determine the upper and lower speeds as well as
    /// the deadband bracket.
    pub fn set_bounds(
        &mut self,
        max: f64,
        deadband_max: f64,
        center: f64,
        deadband_min: f64,
        min: f64,
    ) -> HalResult<()> {
        hal_call!(HAL_SetPWMConfig(
            self.handle,
            max,
            deadband_max,
            center,
            deadband_min,
            min
        ))
    }

    /// Set the bounds on the PWM values. This sets the bounds on the PWM values for a particular
    /// each type of controller. The values determine the upper and lower speeds as well as the
    /// deadband bracket.
    pub fn set_raw_bounds(
        &mut self,
        max: i32,
        deadband_max: i32,
        center: i32,
        deadband_min: i32,
        min: i32,
    ) -> HalResult<()> {
        hal_call!(HAL_SetPWMConfigRaw(
            self.handle,
            max,
            deadband_max,
            center,
            deadband_min,
            min
        ))
    }

    /// Get the bounds on the PWM values. This Gets the bounds on the PWM values for a particular
    /// each type of controller. The values determine the upper and lower speeds as well as the
    /// deadband bracket.
    pub fn raw_bounds(
        &self,
        max: &mut i32,
        deadband_max: &mut i32,
        center: &mut i32,
        deadband_min: &mut i32,
        min: &mut i32,
    ) -> HalResult<()> {
        hal_call!(HAL_GetPWMConfigRaw(
            self.handle,
            max,
            deadband_max,
            center,
            deadband_min,
            min
        ))
    }

    /// Get the channel of this device.
    pub fn channel(&self) -> i32 {
        self.channel
    }
}

impl Drop for PWM {
    fn drop(&mut self) {
        hal_call!(HAL_SetPWMDisabled(self.handle)).ok();
        hal_call!(HAL_FreePWMPort(self.handle)).ok();
    }
}

#[derive(Debug)]
pub struct PwmSpeedController {
    pwm: PWM,
    inverted: bool,
}

impl PwmSpeedController {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(channel: i32) -> HalResult<Self> {
        Ok(PwmSpeedController {
            pwm: PWM::new(channel)?,
            inverted: false,
        })
    }

    /// Creates a PwmMotorController which is configured as a talonSRX
    pub fn new_talon(channel: i32) -> HalResult<PwmSpeedController> {
        let mut pwm = PWM::new(channel)?;
        pwm.set_bounds(2.004, 1.52, 1.5, 1.48, 0.997)?;
        pwm.set_period_multiplier(PeriodMultiplier::Multiplier1x)?;
        pwm.set_speed(0.0)?;
        pwm.set_zero_latch()?;
        usage::report(resource_types::PWMTalonSRX, channel as instances::Type);
        Ok(PwmSpeedController {
            pwm,
            inverted: false,
        })
    }

    /// Set the PWM value. The PWM value is set using a range of -1.0 to 1.0, appropriately scaling
    /// the value for the FPGA.
    pub fn set(&mut self, speed: f64) -> HalResult<()> {
        self.pwm
            .set_speed(if self.inverted { -speed } else { speed })
    }

    /// Get the recently set value of the PWM.
    pub fn get(&self) -> HalResult<f64> {
        if self.inverted {
            Ok(-self.pwm.speed()?)
        } else {
            self.pwm.speed()
        }
    }

    /// Sets if the provided speed is inverted by default when calling set.
    pub fn set_inverted(&mut self, inverted: bool) {
        self.inverted = inverted;
    }

    /// Gets if the PWM is being inverted.
    pub fn inverted(&self) -> bool {
        self.inverted
    }

    /// Disabled the PWM until the next update.
    pub fn disable(&mut self) -> HalResult<()> {
        self.pwm.set_disabled()
    }
}
