//! Observation utilities for the Driver Station. Calling these functions will
//! generate a corresponding event in the DS log.
//!
//! These functions are already called by [`crate::iterative_robot::IterativeRobot`] for you.
//!
//! The mode functions must be called at least ever 50ms, otherwise the DS will
//! disable the robot.

use wpilib_sys::{
    HAL_ObserveUserProgramAutonomous, HAL_ObserveUserProgramDisabled,
    HAL_ObserveUserProgramStarting, HAL_ObserveUserProgramTeleop, HAL_ObserveUserProgramTest,
};

/// Tell the Driver Station that the robot is ready to be enabled.
///
/// This should be called once all hardware is initialised.
/// This must be called to allow the Driver Station to enable the robot.
#[inline]
pub fn start() {
    unsafe { HAL_ObserveUserProgramStarting() }
}

/// Acknowledge to the Driver Station that we are running disabled.
#[inline]
pub fn disabled() {
    unsafe { HAL_ObserveUserProgramDisabled() }
}

/// Acknowledge to the Driver Station that we are running autonomous enabled.
#[inline]
pub fn autonomous() {
    unsafe { HAL_ObserveUserProgramAutonomous() }
}

/// Acknowledge to the Driver Station that we are running teleoperated enabled.
#[inline]
pub fn teleop() {
    unsafe { HAL_ObserveUserProgramTeleop() }
}

/// Acknowledge to the Driver Station that we are running test mode.
#[inline]
pub fn test() {
    unsafe { HAL_ObserveUserProgramTest() }
}
