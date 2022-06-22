pub mod error;

use self::error::RobotError;
use crate::{
    ds::{DriverStation, UninitializedDriverStation},
    hal::{UninitializedHAL, HAL},
};
use wpilib_sys::usage;

pub struct UninitializedRobot {
    hal: UninitializedHAL,
    ds: UninitializedDriverStation,
}

pub struct Robot {
    hal: HAL,
    ds: DriverStation,
    pub(self) _private: (),
}

impl UninitializedRobot {
    pub fn new(hal: UninitializedHAL, ds: UninitializedDriverStation) -> Self {
        Self { hal, ds }
    }

    pub fn initialize(self) -> Result<Robot, RobotError> {
        let hal = self.hal.initialize()?;
        let ds = self.ds.initialize(&hal);

        usage::report(usage::resource_types::Language, unsafe {
            std::mem::transmute(*b"Rust")
        });

        Ok(Robot {
            hal,
            ds,
            _private: (),
        })
    }
}

impl Default for UninitializedRobot {
    fn default() -> Self {
        Self {
            hal: UninitializedHAL::default(),
            ds: UninitializedDriverStation::default(),
        }
    }
}

impl Robot {
    pub fn get_ds(&self) -> &DriverStation {
        &self.ds
    }
}
