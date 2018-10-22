// Copyright 2018 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use sensor_util;
use wpilib_sys::*;

pub struct PwmMotorController {
    channel: i32,
    handle: HAL_DigitalHandle,
}

pub enum PeriodMultiplier {
    Multiplier1x,
    Multiplier2x,
    Multiplier4x,
}

impl PwmMotorController {

    #[allow(clippy::new_ret_no_self)]
    pub fn new(channel: i32) -> HalResult<Self> {
        if !sensor_util::check_pwm_channel(channel) {
            return Err(HalError(0));
        }

        let handle = hal_call!(HAL_InitializePWMPort(HAL_GetPort(channel)))?;

        hal_call!(HAL_SetPWMDisabled(handle));
        hal_call!(HAL_SetPWMEliminateDeadband(handle, false as i32));

        report_usage(resource_type!(PWM), channel as UsageResourceInstance);

        Ok(PwmMotorController { channel, handle })
    }

    pub fn set_raw(&mut self, value: i32) -> HalResult<()> {
        hal_call!(HAL_SetPWMRaw(self.handle, value))
    }

    pub fn get_raw(&self) -> HalResult<i32> {
        hal_call!(HAL_GetPWMRaw(self.handle))
    }

    pub fn set_position(&mut self, pos: f64) -> HalResult<()> {
        hal_call!(HAL_SetPWMPosition(self.handle, pos))
    }

    pub fn get_position(&self) -> HalResult<f64> {
        hal_call!(HAL_GetPWMPosition(self.handle))
    }

    pub fn set_speed(&mut self, speed: f64) -> HalResult<()> {
        hal_call!(HAL_SetPWMSpeed(self.handle, speed))
    }

    pub fn get_speed(&self) -> HalResult<f64> {
        hal_call!(HAL_GetPWMSpeed(self.handle))
    }

    pub fn set_disabled(&mut self) -> HalResult<()> {
        hal_call!(HAL_SetPWMDisabled(self.handle))
    }

    pub fn set_period_multiplier(&mut self, mult: &PeriodMultiplier) -> HalResult<()> {
        let squelch_num = match mult {
            PeriodMultiplier::Multiplier1x => 0,
            PeriodMultiplier::Multiplier2x => 1,
            PeriodMultiplier::Multiplier4x => 3,
        };

        hal_call!(HAL_SetPWMPeriodScale(self.handle, squelch_num))
    }

    pub fn set_zero_latch(&mut self) -> HalResult<()> {
        hal_call!(HAL_LatchPWMZero(self.handle))
    }


    pub fn enable_deadband_elimination(&mut self, eliminate_deadband: bool) -> HalResult<()> {
        hal_call!(HAL_SetPWMEliminateDeadband(self.handle, eliminate_deadband as i32))
    }

    pub fn set_bounds(&mut self, max: f64, deadband_max: f64, center: f64, deadband_min: f64, min: f64) -> HalResult<()> {
        hal_call!(HAL_SetPWMConfig(self.handle, max, deadband_max, center, deadband_min, min))
    }

    pub fn set_raw_bounds(&mut self, max: i32, deadband_max: i32, center: i32, deadband_min: i32, min: i32) -> HalResult<()> {
        hal_call!(HAL_SetPWMConfigRaw(self.handle, max, deadband_max, center, deadband_min, min))
    }

    pub fn get_raw_bounds(&self, max: &mut i32, deadband_max: &mut i32, center: &mut i32, deadband_min: &mut i32, min: &mut i32) -> HalResult<()> {
        hal_call!(HAL_GetPWMConfigRaw(self.handle, max, deadband_max, center, deadband_min, min))
    }

    pub fn get_channel(&self) -> i32 {
        self.channel
    }
}