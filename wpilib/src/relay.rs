// Copyright 2019 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Interface to the relay pins.
//!
//! Relays are intended to be connected to Spikes or similar relays.
//! The relay channels control a pair of pins which can be independently
//! toggled on or off.  This can allow off, full forward, or full reverse
//! control of motors without variable speed.
//!
//! # Examples
//!
//! Using a bidirectional relay:
//!
//! ```
//! use wpilib::relay;
//!
//! # fn main() -> wpilib::HalResult<()> {
//! let mut relay = relay::Relay<relay::BothDirections>::new(1)?;
//!
//! relay.set(relay::Value::Forward)?;
//! assert_eq!(relay.get(), relay::Value::Forward);
//! # }
//! ```

use std::marker::PhantomData;
use wpilib_sys::*;

/// Possible values for a bidirectional relay.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    /// Both pins set to 0V.
    Off,
    /// All pins set to 12V.
    On,
    /// Forward pin only set to 12V. Reverse pin set to 0V.
    Forward,
    /// Reverse pin only set to 12V. Forward pin set to 0V.
    Reverse,
}

/*
/// Convenience for treating bidirectional and unidirectional relays the same.
impl From<bool> for Value {
    fn from(v: bool) -> Self {
        if v {
            Value::On
        } else {
            Value::Off
        }
    }
}
*/

#[derive(Debug)]
/// Marker specifying a Relay goes in both directions.
pub struct BothDirections;
#[derive(Debug)]
/// Marker specifying a Relay goes forward only.
pub struct ForwardOnly;
#[derive(Debug)]
/// Marker specifying a Relay goes reverse only.
pub struct ReverseOnly;

/// Marker trait for direction type parameter for Relay.
///
/// This trait is sealed and cannot be implemented outside of this crate.
pub trait Direction: private::Sealed {
    #[doc(hidden)]
    const FORWARD: bool = true;
    #[doc(hidden)]
    const REVERSE: bool = true;
}
impl Direction for BothDirections {}
impl Direction for ForwardOnly {
    #[doc(hidden)]
    const REVERSE: bool = false;
}
impl Direction for ReverseOnly {
    #[doc(hidden)]
    const FORWARD: bool = false;
}

/// Interface for Spike-style relay outputs.
///
/// See the module-level docs for more details.
#[derive(Debug)]
pub struct Relay<DIR: Direction> {
    forward_handle: HAL_RelayHandle,
    reverse_handle: HAL_RelayHandle,
    channel: i32,
    direction: PhantomData<DIR>,
}

impl<DIR: Direction> Relay<DIR> {
    /// Creates a Relay given a channel.
    ///
    /// The relay will be initialized such that both lines are initially 0V.
    pub fn new(channel: i32) -> HalResult<Self> {
        let port_handle = unsafe { HAL_GetPort(channel) };

        let forward_handle = if DIR::FORWARD {
            usage::report(usage::resource_types::Relay, channel as _);
            hal_call!(HAL_InitializeRelayPort(port_handle, true as HAL_Bool))?
        } else {
            HAL_kInvalidHandle
        };
        let reverse_handle = if DIR::REVERSE {
            usage::report(
                usage::resource_types::Relay,
                channel as usage::instances::Type + 128,
            );
            hal_call!(HAL_InitializeRelayPort(port_handle, false as HAL_Bool))?
        } else {
            HAL_kInvalidHandle
        };

        if forward_handle != HAL_kInvalidHandle {
            hal_call!(HAL_SetRelay(forward_handle, false as HAL_Bool))?
        }
        if reverse_handle != HAL_kInvalidHandle {
            hal_call!(HAL_SetRelay(reverse_handle, false as HAL_Bool))?
        }

        Ok(Self {
            forward_handle,
            reverse_handle,
            channel,
            direction: PhantomData,
        })
    }
}

impl Relay<BothDirections> {
    /// Set the relay state.
    pub fn set(&mut self, value: Value) -> HalResult<()> {
        self.set_forward(match value {
            Value::On | Value::Forward => true,
            _ => false,
        })?;
        self.set_reverse(match value {
            Value::On | Value::Reverse => true,
            _ => false,
        })
    }

    /// Get the relay state.
    pub fn get(&self) -> HalResult<Value> {
        match (
            hal_call!(HAL_GetRelay(self.forward_handle))? != 0,
            hal_call!(HAL_GetRelay(self.reverse_handle))? != 0,
        ) {
            (true, true) => Ok(Value::On),
            (true, false) => Ok(Value::Forward),
            (false, true) => Ok(Value::Reverse),
            (false, false) => Ok(Value::Off),
        }
    }
}

impl Relay<ForwardOnly> {
    pub fn set(&mut self, value: bool) -> HalResult<()> {
        self.set_forward(value)
    }

    pub fn get(&self) -> HalResult<bool> {
        Ok(hal_call!(HAL_GetRelay(self.forward_handle))? != 0)
    }
}

impl Relay<ReverseOnly> {
    pub fn set(&mut self, value: bool) -> HalResult<()> {
        self.set_reverse(value)
    }

    pub fn get(&self) -> HalResult<bool> {
        Ok(hal_call!(HAL_GetRelay(self.reverse_handle))? != 0)
    }
}

impl<DIR: Direction> Relay<DIR> {
    pub fn channel(&self) -> i32 {
        self.channel
    }

    fn set_forward(&mut self, value: bool) -> HalResult<()> {
        hal_call!(HAL_SetRelay(self.forward_handle, value as HAL_Bool))
    }

    fn set_reverse(&mut self, value: bool) -> HalResult<()> {
        hal_call!(HAL_SetRelay(self.reverse_handle, value as HAL_Bool))
    }
}

impl<DIR: Direction> Drop for Relay<DIR> {
    fn drop(&mut self) {
        // ignore errors, as we want to make sure a free happens
        if self.forward_handle != HAL_kInvalidHandle {
            let _ = self.set_forward(false);
            unsafe { HAL_FreeRelayPort(self.forward_handle) }
        }
        if self.reverse_handle != HAL_kInvalidHandle {
            let _ = self.set_reverse(false);
            unsafe { HAL_FreeRelayPort(self.reverse_handle) }
        }
    }
}

// Seal the Direction trait.
mod private {
    pub trait Sealed {}
    impl Sealed for super::BothDirections {}
    impl Sealed for super::ForwardOnly {}
    impl Sealed for super::ReverseOnly {}
}
