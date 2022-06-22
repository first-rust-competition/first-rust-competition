pub mod error;

use self::error::RobotError;
use crate::hal::{UninitializedHAL, HAL};
use wpilib_sys::usage;

pub struct UninitializedRobot {
    hal: UninitializedHAL,
}

pub struct Robot {
    hal: HAL,
    pub(self) _private: (),
}

impl UninitializedRobot {
    pub fn new(hal: UninitializedHAL) -> Self {
        Self { hal }
    }

    pub fn initialize(self) -> Result<Robot, RobotError> {
        let hal = self.hal.initialize()?;

        usage::report(usage::resource_types::Language, unsafe {
            std::mem::transmute(*b"Rust")
        });

        Ok(Robot { hal, _private: () })
    }
}

impl Default for UninitializedRobot {
    fn default() -> Self {
        Self {
            hal: UninitializedHAL::default(),
        }
    }
}
