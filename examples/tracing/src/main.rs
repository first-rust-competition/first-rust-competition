use wpilib::{
    hal::{UninitializedHAL, HAL},
    robot::error::RobotError,
};

fn main() -> Result<(), RobotError> {
    // Set up `tracing`
    tracing_subscriber::fmt::init();

    let _hal: HAL = UninitializedHAL::default().initialize()?;

    Ok(())
}
