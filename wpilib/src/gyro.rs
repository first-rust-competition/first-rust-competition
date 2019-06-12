use wpilib_sys::*;

pub trait Gyro {
    fn calibrate(&self) -> HalResult<()>;

    fn reset(&self) -> HalResult<()>;

    fn angle(&self) -> HalResult<f64>;

    fn rate(&self) -> HalResult<f64>;
}

const DEFAULT_VOLTS_PER_DEGREE_PER_SECOND: f64 = 0.007;

pub struct AnalogGyro {
    handle: HAL_GyroHandle,
}

impl AnalogGyro {
    pub fn new(channel: i32) -> HalResult<Self> {
        let handle = hal_call!(HAL_InitializeAnalogGyro(channel))?;

        hal_call!(HAL_SetupAnalogGyro(handle))?;

        usage::report(usage::resource_types::Gyro, channel as u32);

        Ok(AnalogGyro { handle })
    }

    pub fn offset(&self) -> HalResult<f64> {
        hal_call!(HAL_GetAnalogGyroOffset(self.handle))
    }

    pub fn set_parameters(&self, center: i32, offset: f64) -> HalResult<()> {
        hal_call!(HAL_SetAnalogGyroParameters(
            self.handle,
            DEFAULT_VOLTS_PER_DEGREE_PER_SECOND,
            offset,
            center
        ))
    }

    pub fn set_sensitivity(&self, volts_per_degree_per_second: f64) -> HalResult<()> {
        hal_call!(HAL_SetAnalogGyroVoltsPerDegreePerSecond(
            self.handle,
            volts_per_degree_per_second
        ))
    }

    pub fn set_deadband(&self, volts: f64) -> HalResult<()> {
        hal_call!(HAL_SetAnalogGyroDeadband(self.handle, volts))
    }
}

impl Gyro for AnalogGyro {
    fn calibrate(&self) -> HalResult<()> {
        hal_call!(HAL_CalibrateAnalogGyro(self.handle))
    }

    fn reset(&self) -> HalResult<()> {
        hal_call!(HAL_ResetAnalogGyro(self.handle))
    }

    fn angle(&self) -> HalResult<f64> {
        hal_call!(HAL_GetAnalogGyroAngle(self.handle))
    }

    fn rate(&self) -> HalResult<f64> {
        hal_call!(HAL_GetAnalogGyroRate(self.handle))
    }
}

impl Drop for AnalogGyro {
    fn drop(&mut self) {
        unsafe { HAL_FreeAnalogGyro(self.handle) }
    }
}
