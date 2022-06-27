//! Interface to WPILib's Hardware Abstraction Layer.

use self::error::{HALError::HALInitializationError, HALInitializationError::Unknown};

pub mod error;

/// A marker struct for an uninitialized HAL.
pub struct UninitializedHAL {
    timeout: i32,
    mode: i32,
}

/// A handle to WPILib's [Hardware Abstraction Layer](https://en.wikipedia.org/wiki/Hardware_abstraction).
///
/// In order to get a handle to this struct, initialyze [`UninitializedHAL`].
pub struct HAL {
    /// A marker that stops external users from instantiating HAL directly. If you want to get a HAL, use [`UninitializedHAL`] instead.
    pub(self) _private: (),
}

impl UninitializedHAL {
    pub fn new(timeout: i32, mode: i32) -> Self {
        Self { timeout, mode }
    }

    pub fn initialize(self) -> Result<HAL, error::HALError> {
        if unsafe { wpilib_sys::bindings::HAL_Initialize(self.timeout, self.mode) } == 0 {
            return Err(HALInitializationError(Unknown));
        }

        #[cfg(feature = "tracing")]
        tracing::trace!(action = "initialize HAL", ok = true);

        Ok(HAL::new())
    }
}

impl Default for UninitializedHAL {
    fn default() -> Self {
        Self {
            timeout: 500,
            mode: 0,
        }
    }
}

impl HAL {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}
