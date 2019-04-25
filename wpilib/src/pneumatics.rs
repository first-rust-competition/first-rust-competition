// Copyright 2018 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use lazy_static::lazy_static;
use wpilib_sys::*;

lazy_static! {
    /// The number of solenoid channels per PCM.
    pub static ref NUM_SOLENOID_CHANNELS: i32 = unsafe { HAL_GetNumSolenoidChannels() };

    /// The number of possible PCMs.
    pub static ref NUM_SOLENOID_MODULES: i32 = unsafe { HAL_GetNumPCMModules() };
}

/// Check if a solenoid module (PCM) is valid.
#[inline]
pub fn check_solenoid_module(module: i32) -> bool {
    unsafe { HAL_CheckSolenoidModule(module) != 0 }
}

/// Check if a solenoid channel is valid.
#[inline]
pub fn check_solenoid_channel(channel: i32) -> bool {
    unsafe { HAL_CheckSolenoidChannel(channel) != 0 }
}

/// Represents a CTRE Pneumatics Control Module.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PneumaticsControlModule(i32);

impl Default for PneumaticsControlModule {
    /// Returns a PCM with the default ID of 0.
    #[inline]
    fn default() -> Self {
        PneumaticsControlModule::DEFAULT
    }
}

impl PneumaticsControlModule {
    /// A PCM with the default ID of 0.
    // 0 is guaranteed to be valid.
    const DEFAULT: Self = PneumaticsControlModule(0);

    /// Creates a PCM object for a given ID.
    pub fn new(module_id: i32) -> Option<Self> {
        if check_solenoid_module(module_id) {
            Some(PneumaticsControlModule(module_id))
        } else {
            None
        }
    }

    /// Returns the module ID.
    #[inline]
    pub fn number(self) -> i32 {
        self.0
    }

    /// Gets the state of each solenoid on the PCM.
    /// Returns a bit mask.
    pub fn all(self) -> HalResult<i32> {
        hal_call!(HAL_GetAllSolenoids(self.0))
    }

    pub fn solenoid_blacklist(self) -> i32 {
        maybe_hal_call!(HAL_GetPCMSolenoidBlackList(self.0)).ok()
    }

    pub fn solenoid_voltage_sticky_fault(self) -> bool {
        maybe_hal_call!(HAL_GetPCMSolenoidVoltageStickyFault(self.0)).ok() != 0
    }

    pub fn solenoid_voltage_fault(self) -> bool {
        maybe_hal_call!(HAL_GetPCMSolenoidVoltageFault(self.0)).ok() != 0
    }

    pub fn clear_all_sticky_faults(&mut self) -> HalResult<()> {
        hal_call!(HAL_ClearAllPCMStickyFaults(self.0))
    }

    pub fn solenoid(self, channel: i32) -> HalResult<Solenoid> {
        Solenoid::with_module(self, channel)
    }

    pub fn double_solenoid(
        self,
        forward_channel: i32,
        reverse_channel: i32,
    ) -> HalResult<DoubleSolenoid> {
        DoubleSolenoid::with_module(self, forward_channel, reverse_channel)
    }

    pub fn compressor(self) -> Compressor {
        Compressor::with_module(self)
    }
}

#[derive(Debug)]
pub struct Solenoid {
    handle: HAL_SolenoidHandle,
    channel: i32,
    module: PneumaticsControlModule,
}

impl Solenoid {
    /// Create a new solenoid on the default PCM with the given channel.
    pub fn new(channel: i32) -> HalResult<Solenoid> {
        Self::with_module(PneumaticsControlModule::DEFAULT, channel)
    }

    /// If for some reason the Pneumatic Control Module is not on CAN module 0,
    /// you can use this constructor. Most people will never need this.
    pub fn with_module(module: PneumaticsControlModule, channel: i32) -> HalResult<Solenoid> {
        let module_number = module.number();

        let handle = hal_call!(HAL_InitializeSolenoidPort(HAL_GetPortWithModule(
            module_number,
            channel
        )))?;

        usage::report_context(
            usage::resource_types::Solenoid,
            channel as usage::instances::Type,
            module_number,
        );

        Ok(Solenoid {
            handle,
            channel,
            module,
        })
    }

    /// Sets the solenoid to on or off
    pub fn set(&mut self, on: bool) -> HalResult<()> {
        hal_call!(HAL_SetSolenoid(self.handle, on as HAL_Bool))
    }

    /// Gets the state of the solenoid by calling out to the hardware through an FFI.
    /// If you need speed, consider caching the value you set yourself!
    pub fn get(&self) -> HalResult<bool> {
        Ok(hal_call!(HAL_GetSolenoid(self.handle))? != 0)
    }

    pub fn is_blacklisted(&self) -> bool {
        (self.module.solenoid_blacklist() & (1 << self.channel)) != 0
    }

    pub fn set_pulse_duration(&mut self, seconds: f64) -> HalResult<()> {
        let duration_ms: i32 = (seconds * 1000.0) as i32;
        hal_call!(HAL_SetOneShotDuration(self.handle, duration_ms))
    }

    pub fn start_pulse(&mut self) -> HalResult<()> {
        hal_call!(HAL_FireOneShot(self.handle))
    }

    pub fn module(&self) -> &PneumaticsControlModule {
        &self.module
    }
}

impl Drop for Solenoid {
    fn drop(&mut self) {
        unsafe { HAL_FreeSolenoidPort(self.handle) }
    }
}

/// Possible values for a DoubleSolenoid.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    Forward,
    Reverse,
    Off,
}

impl Default for Action {
    #[inline]
    fn default() -> Self {
        Action::Off
    }
}

#[derive(Debug)]
pub struct DoubleSolenoid {
    forward: Solenoid,
    reverse: Solenoid,
}

impl DoubleSolenoid {
    pub fn new(forward: Solenoid, reverse: Solenoid) -> Self {
        DoubleSolenoid { forward, reverse }
    }

    pub fn with_channels(forward_channel: i32, reverse_channel: i32) -> HalResult<DoubleSolenoid> {
        Self::with_module(
            PneumaticsControlModule::DEFAULT,
            forward_channel,
            reverse_channel,
        )
    }

    pub fn with_module(
        module: PneumaticsControlModule,
        forward_channel: i32,
        reverse_channel: i32,
    ) -> HalResult<DoubleSolenoid> {
        Ok(DoubleSolenoid {
            forward: Solenoid::with_module(module, forward_channel)?,
            reverse: Solenoid::with_module(module, reverse_channel)?,
        })
    }

    pub fn set(&mut self, action: Action) -> HalResult<()> {
        self.forward.set(action == Action::Forward)?;
        self.reverse.set(action == Action::Reverse)?;
        Ok(())
    }

    pub fn get(&self) -> HalResult<Action> {
        if self.forward.get()? {
            Ok(Action::Forward)
        } else if self.reverse.get()? {
            Ok(Action::Reverse)
        } else {
            Ok(Action::Off)
        }
    }

    pub fn is_fwd_blacklisted(&self) -> bool {
        self.forward.is_blacklisted()
    }

    pub fn is_rev_blacklisted(&self) -> bool {
        self.reverse.is_blacklisted()
    }
}

#[derive(Debug)]
pub struct Compressor {
    handle: HAL_CompressorHandle,
}

impl Default for Compressor {
    #[inline]
    fn default() -> Self {
        Compressor::new()
    }
}

impl Compressor {
    pub fn new() -> Self {
        Self::with_module(PneumaticsControlModule::DEFAULT)
    }

    pub fn with_module(module: PneumaticsControlModule) -> Self {
        // HAL_InitializeCompressor returns an error iff the module number is
        // invalid, but PneumaticsControlModule already guarantees it's valid.
        let handle = hal_call!(HAL_InitializeCompressor(module.number())).unwrap();
        Self { handle }
    }

    /// Allow or disallow the compressor to run.
    pub fn set_closed_loop_control(&mut self, on: bool) -> HalResult<()> {
        hal_call!(HAL_SetCompressorClosedLoopControl(
            self.handle,
            on as HAL_Bool
        ))
    }

    /// Allow the compressor to start.
    pub fn start(&mut self) -> HalResult<()> {
        self.set_closed_loop_control(true)
    }

    /// Forcibly stop the compressor.
    pub fn stop(&mut self) -> HalResult<()> {
        self.set_closed_loop_control(false)
    }

    pub fn enabled(&self) -> bool {
        maybe_hal_call!(HAL_GetCompressor(self.handle)).ok() != 0
    }

    pub fn pressure_switch_value(&self) -> bool {
        maybe_hal_call!(HAL_GetCompressorPressureSwitch(self.handle)).ok() != 0
    }

    pub fn current(&self) -> f64 {
        maybe_hal_call!(HAL_GetCompressorCurrent(self.handle)).ok()
    }

    pub fn closed_loop_control(&self) -> bool {
        maybe_hal_call!(HAL_GetCompressorClosedLoopControl(self.handle)).ok() != 0
    }

    pub fn current_too_high_fault(&self) -> bool {
        maybe_hal_call!(HAL_GetCompressorCurrentTooHighFault(self.handle)).ok() != 0
    }

    pub fn current_too_high_sticky_fault(&self) -> bool {
        maybe_hal_call!(HAL_GetCompressorCurrentTooHighStickyFault(self.handle)).ok() != 0
    }

    pub fn shorted_sticky_fault(&self) -> bool {
        maybe_hal_call!(HAL_GetCompressorShortedStickyFault(self.handle)).ok() != 0
    }

    pub fn shorted_fault(&self) -> bool {
        maybe_hal_call!(HAL_GetCompressorShortedFault(self.handle)).ok() != 0
    }

    pub fn not_connected_sticky_fault(&self) -> bool {
        maybe_hal_call!(HAL_GetCompressorNotConnectedStickyFault(self.handle)).ok() != 0
    }

    pub fn not_connected_fault(&self) -> bool {
        maybe_hal_call!(HAL_GetCompressorNotConnectedFault(self.handle)).ok() != 0
    }
}
