use wpilib_sys::*;

pub trait Gyro {
    type Error;

    /// Calibrate the gyro by running for a number of samples and computing the
    /// center value. Then use the center value as the Accumulator center value for
    /// subsequent measurements. It's important to make sure that the robot is not
    /// moving while the centering calculations are in progress, this is typically
    /// done when the robot is first turned on while it's sitting at rest before
    /// the competition starts.
    fn calibrate(&self) -> Result<(), Self::Error>;

    /// Reset the gyro. Resets the gyro to a heading of zero. This can be used if
    /// there is significant drift in the gyro and it needs to be recalibrated
    /// after it has been running.
    fn reset(&self) -> Result<(), Self::Error>;

    /// Return the actual angle in degrees that the robot is currently facing.
    ///
    /// The angle is based on the current accumulator value corrected by the
    /// oversampling rate, the gyro type and the A/D calibration values. The angle
    /// is continuous, that is it will continue from 360 to 361 degrees. This
    /// allows algorithms that wouldn't want to see a discontinuity in the gyro
    /// output as it sweeps past from 360 to 0 on the second time around.
    ///
    /// The angle is expected to increase as the gyro turns clockwise when looked
    /// at from the top. It needs to follow NED axis conventions in order to work
    /// properly with dependent control loops.
    ///
    /// Returns the current heading of the robot in degrees. This heading is based
    /// on integration of the returned rate from the gyro.
    fn angle(&self) -> Result<f64, Self::Error>;

    /// Return the rate of rotation of the gyro.
    ///
    /// The rate is based on the most recent reading of the gyro analog value.
    ///
    /// The rate is expected to be positive as the gyro turns clockwise when looked
    /// at from the top. It needs to follow NED axis conventions in order to work
    /// properly with dependent control loops.
    ///
    /// Returns the current rate in degrees per second
    fn rate(&self) -> Result<f64, Self::Error>;
}

const DEFAULT_VOLTS_PER_DEGREE_PER_SECOND: f64 = 0.007;

pub struct AnalogGyro {
    handle: HAL_GyroHandle,
}

impl AnalogGyro {
    /// Creates a new gyro attached to the given analog input channel
    ///
    /// Note that gyros can only be used on onboard analog inputs 0 and 1
    pub fn new(channel: i32) -> HalResult<Self> {
        let handle = hal_call!(HAL_InitializeAnalogGyro(channel))?;

        hal_call!(HAL_SetupAnalogGyro(handle))?;

        usage::report(usage::resource_types::Gyro, channel as usage::instances::Type);

        Ok(AnalogGyro { handle })
    }

    /// Return the gyro offset value. If run after calibration,
    /// the offset value can be used as a preset later.
    pub fn offset(&self) -> HalResult<f64> {
        hal_call!(HAL_GetAnalogGyroOffset(self.handle))
    }

    /// Return the gyro center value. If run after calibration,
    /// the center value can be used as a preset later.
    pub fn center(&self) -> HalResult<i32> {
        hal_call!(HAL_GetAnalogGyroCenter(self.handle))
    }

    /// Configures the gyro parameters for the center value of the accumulator, and the raw offset of the gyro
    pub fn set_parameters(&self, center: i32, offset: f64) -> HalResult<()> {
        hal_call!(HAL_SetAnalogGyroParameters(
            self.handle,
            DEFAULT_VOLTS_PER_DEGREE_PER_SECOND,
            offset,
            center
        ))
    }

    /// Set the gyro sensitivity.
    ///
    /// This takes the number of volts/degree/second sensitivity of the gyro and
    /// uses it in subsequent calculations to allow the code to work with multiple
    /// gyros. This value is typically found in the gyro datasheet.
    pub fn set_sensitivity(&self, volts_per_degree_per_second: f64) -> HalResult<()> {
        hal_call!(HAL_SetAnalogGyroVoltsPerDegreePerSecond(
            self.handle,
            volts_per_degree_per_second
        ))
    }

    /// Set the size of the neutral zone.
    ///
    /// Any voltage from the gyro less than this amount from the center is
    /// considered stationary.  Setting a deadband will decrease the amount of
    /// drift when the gyro isn't rotating, but will make it less accurate.
    pub fn set_deadband(&self, volts: f64) -> HalResult<()> {
        hal_call!(HAL_SetAnalogGyroDeadband(self.handle, volts))
    }
}

impl Gyro for AnalogGyro {
    type Error = HalError;

    fn calibrate(&self) -> Result<(), Self::Error> {
        hal_call!(HAL_CalibrateAnalogGyro(self.handle))
    }

    fn reset(&self) -> Result<(), Self::Error> {
        hal_call!(HAL_ResetAnalogGyro(self.handle))
    }

    fn angle(&self) -> Result<f64, Self::Error> {
        hal_call!(HAL_GetAnalogGyroAngle(self.handle))
    }

    fn rate(&self) -> Result<f64, Self::Error> {
        hal_call!(HAL_GetAnalogGyroRate(self.handle))
    }
}

impl Drop for AnalogGyro {
    fn drop(&mut self) {
        unsafe { HAL_FreeAnalogGyro(self.handle) }
    }
}
