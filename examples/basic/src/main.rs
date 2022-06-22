//! Starts the robot, prints "Hello, world!", and stays in a loop forever.

use wpilib::{
    hal::{UninitializedHAL, HAL},
    robot::error::RobotError,
};

fn main() -> Result<(), RobotError> {
    #[cfg(target_arch = "arm")]
    let _hal: HAL = UninitializedHAL::default().initialize()?;

    // TODO: Add HAL simulation.
    // #[cfg(not(target_arch = "arm"))]
    // let _hal: HAL = UninitializedHAL::default().initialize()?;

    println!("Hello, world!");

    loop {}
}
