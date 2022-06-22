use thiserror::Error;

#[derive(Error, Debug)]
pub enum HALError {
    #[error("Couldn't initialize HAL.")]
    HALInitializationError(#[from] HALInitializationError),
}

#[derive(Error, Debug)]
pub enum HALInitializationError {
    #[error("Unknown error.")]
    Unknown,
}
