use thiserror::Error;

#[derive(Error, Debug)]
pub enum RobotError {
    #[error("HAL error.")]
    HALError(#[from] crate::hal::error::HALError),
}
