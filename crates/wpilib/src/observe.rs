// Copyright 2020 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Functions to report the robot status to the Driver Station.
//!
//! These functions are called by the IterativeRobot starter functions for you.
//!
//! Calling these functions will generate a corresponding event in the DS log.
//!
//! The mode functions must be called at least ever 50ms,
//! otherwise the DS will disable the robot.

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
