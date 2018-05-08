use hal::*;

/// A digital output used to control lights, etc from the RoboRIO.
pub struct DigitalOutput {
    channel: i32,
    handle: HAL_DigitalHandle,
    pwm: Option<HAL_DigitalPWMHandle>,
}

impl DigitalOutput {
    /// Create a new digital output on the specificed channel, returning an error if initialization
    /// fails.
    pub fn new(channel: i32) -> HalResult<DigitalOutput> {
        if !sensor_base::check_digital_channel(channel) {
            return Err(HalError(0));
        }

        let handle = hal_call!(HAL_InitializeDIOPort(HAL_GetPort(channel), false as i32))?;

        report_usage(tResourceType_kResourceType_DigitalOutput, channel);

        Ok(DigitalOutput {
            channel: channel,
            handle: handle,
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
                sensor_base::num_digital_channels()
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
