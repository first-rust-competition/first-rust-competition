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

use wpilib_sys::*;

#[repr(transparent)]
/// Wrapper for a relay handle.
#[derive(Debug)]
struct RelayHandle(HAL_RelayHandle);

impl RelayHandle {
    fn new(port_handle: HAL_PortHandle, forward: bool) -> HalResult<Self> {
        let mut relay = Self(hal_call!(HAL_InitializeRelayPort(
            port_handle,
            forward as HAL_Bool,
        ))?);
        relay.set(false)?;
        Ok(relay)
    }

    fn get(&self) -> HalResult<()> {
        Ok(hal_call!(HAL_GetRelay(self.0))? != 0)
    }

    fn set(&mut self, on: bool) -> HalResult<()> {
        hal_call!(HAL_SetRelay(self.0, on as HAL_Bool))
    }
}

impl Drop for RelayHandle {
    fn drop(&mut self) {
        // ignore errors as we want to make sure a free happens
        let _ = self.set(false);

        unsafe { HAL_FreeRelayPort(self.0) }
    }
}

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

/// Interface for bidirectional Spike-style relay outputs.
///
/// # Examples
///
/// ```
/// # fn main() -> wpilib::HalResult<()> {
/// let mut relay = BiRelay::new(1)?;
///
/// relay.set(Value::Forward)?;
/// assert_eq!(relay.get(), Value::Forward);
/// # }
/// ```
#[derive(Debug)]
pub struct BiRelay {
    forward: RelayHandle,
    reverse: RelayHandle,
    channel: i32,
}

impl BiRelay {
    /// Creates a BiRelay given a channel.
    ///
    /// The relay will be initialized such that both lines are initially 0V.
    pub fn new(channel: i32) -> HalResult<Self> {
        let port_handle = unsafe { HAL_GetPort(channel) };

        let forward = RelayHandle::new(port_handle, true)?;
        let reverse = RelayHandle::new(port_handle, false)?;

        usage::report(usage::resource_types::Relay, channel as _);
        usage::report(
            usage::resource_types::Relay,
            channel as usage::instances::Type + 128,
        );

        Ok(Self {
            forward,
            reverse,
            channel,
        })
    }

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

    /// Set the forward output only (independent of the reverse output).
    fn set_forward(&mut self, on: bool) -> HalResult<()> {
        self.forward.set(on)
    }

    /// Set the reverse output only (independent of the forward output).
    fn set_reverse(&mut self, on: bool) -> HalResult<()> {
        self.reverse.set(on)
    }

    /// Get the relay state.
    pub fn get(&self) -> HalResult<Value> {
        match (self.forward.get()?, self.reverse.get()?) {
            (true, true) => Ok(Value::On),
            (true, false) => Ok(Value::Forward),
            (false, true) => Ok(Value::Reverse),
            (false, false) => Ok(Value::Off),
        }
    }

    /// Get the channel number for this relay.
    pub fn channel(&self) -> i32 {
        self.channel
    }
}

/// Possible directions for a unidirectional relay.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Reverse,
    Forward,
}

/// Interface for unidirectional Spike-style relay outputs.
///
/// # Examples
///
/// ```
/// # fn main() -> wpilib::HalResult<()> {
/// let mut relay = Relay::new(1, Direction::Forward)?;
///
/// relay.set(true)?;
/// assert_eq!(relay.get(), true);
/// # }
/// ```
#[derive(Debug)]
pub struct Relay {
    handle: RelayHandle,
    channel: i32,
}

impl Relay {
    /// Creates a Relay given a channel.
    ///
    /// The relay will be initialized such that it is initially 0V.
    pub fn new(channel: i32, direction: Direction) -> HalResult<Self> {
        let handle = RelayHandle::new(
            unsafe { HAL_GetPort(channel) },
            direction == Direction::Forward,
        )?;

        usage::report(
            usage::resource_types::Relay,
            channel as usage::instances::Type
                + match direction {
                    Direction::Forward => 0,
                    Direction::Reverse => 128,
                },
        );

        Ok(Self { handle, channel })
    }

    #[inline]
    /// Set the relay state.
    pub fn set(&mut self, on: bool) -> HalResult<()> {
        self.handle.set(on)
    }

    #[inline]
    /// Get the relay state.
    pub fn get(&self) -> HalResult<bool> {
        self.handle.get()
    }

    /// Get the channel number for this relay.
    pub fn channel(&self) -> i32 {
        self.channel
    }
}
