// This file is part of "first-rust-competition", which is free software: you
// can redistribute it and/or modify it under the terms of the GNU General
// Public License version 3 as published by the Free Software Foundation. See
// <https://www.gnu.org/licenses/> for a copy.

use super::sensor_util;
use hal::*;
use std::ptr;

/// Corresponds to WPILibC's SolenoidBase, and is responsible for
/// Getting info about a solenoid module, (conceptually a PCM).
/// Even though each Solenoid will have a different instance, they all will
/// probably refer to the same piece of hardware.
pub struct SolenoidModule {
    module: i32,
}

/// All of these methods are equivalent to their WPILib counterparts.
impl SolenoidModule {
    /// Gets the state of each solenoid on the module with the given number.
    /// Returns a bit mask.
    pub fn get_all_with_module(module: i32) -> HalResult<i32> {
        hal_call!(HAL_GetAllSolenoids(module))
    }

    /// Is the same as `get_all_with_module`, but on the module this instance
    /// refers to.
    pub fn get_all(&self) -> HalResult<i32> {
        Self::get_all_with_module(self.module)
    }

    pub fn get_pcm_solenoid_blacklist_with_module(module: i32) -> i32 {
        maybe_hal_call!(HAL_GetPCMSolenoidBlackList(module)).ok()
    }

    pub fn get_pcm_solenoid_blacklist(&self) -> i32 {
        Self::get_pcm_solenoid_blacklist_with_module(self.module)
    }

    pub fn get_pcm_solenoid_voltage_sticky_fault_with_module(module: i32) -> bool {
        maybe_hal_call!(HAL_GetPCMSolenoidVoltageStickyFault(module)).ok() != 0
    }

    pub fn get_pcm_solenoid_voltage_sticky_fault(&self) -> bool {
        Self::get_pcm_solenoid_voltage_sticky_fault_with_module(self.module)
    }

    pub fn get_pcm_solenoid_voltage_fault_with_module(module: i32) -> bool {
        maybe_hal_call!(HAL_GetPCMSolenoidVoltageFault(module)).ok() != 0
    }

    pub fn get_pcm_solenoid_voltage_fault(&self) -> bool {
        Self::get_pcm_solenoid_voltage_fault_with_module(self.module)
    }

    pub fn clear_all_pcm_sticky_faults_with_module(module: i32) {
        maybe_hal_call!(HAL_ClearAllPCMStickyFaults(module)).ok();
    }

    pub fn clear_all_pcm_sticky_faults(&self) {
        Self::clear_all_pcm_sticky_faults_with_module(self.module);
    }
}

pub struct Solenoid {
    solenoid_handle: HAL_SolenoidHandle,
    channel: i32,
    module: SolenoidModule,
}

impl Solenoid {
    /// Make a new solenoid with the given channel.
    pub fn new(channel: i32) -> HalResult<Solenoid> {
        Self::new_with_module(sensor_util::default_solenoid_module(), channel)
    }

    /// If for some reason the Pneumatic Control Module is not on CAN module 0,
    /// you can use this constructor. Most people will never need this.
    pub fn new_with_module(module_number: i32, channel: i32) -> HalResult<Solenoid> {
        if !sensor_util::check_solenoid_module(module_number) {
            return Err(HalError(0));
        };

        if !sensor_util::check_solenoid_channel(channel) {
            return Err(HalError(0));
        };

        let handle = hal_call!(HAL_InitializeSolenoidPort(HAL_GetPortWithModule(
            module_number,
            channel
        )))?;

        report_usage_extras(
            resource_type!(Solenoid),
            channel as UsageResourceType,
            module_number,
            ptr::null(),
        );

        Ok(Solenoid {
            solenoid_handle: handle,
            channel,
            module: SolenoidModule {
                module: module_number,
            },
        })
    }

    /// Sets the solenoid to on or off
    pub fn set(&self, on: bool) -> HalResult<()> {
        hal_call!(HAL_SetSolenoid(self.solenoid_handle, on as i32))
    }

    /// Gets the state of the solenoid by calling out to the hardware through an FFI.
    /// If you need speed, consider caching the value you set yourself!
    pub fn get(&self) -> HalResult<bool> {
        Ok(hal_call!(HAL_GetSolenoid(self.solenoid_handle))? != 0)
    }

    pub fn is_blacklisted(&self) -> bool {
        return (self.module.get_pcm_solenoid_blacklist() & (1 << self.channel)) != 0;
    }

    pub fn set_pulse_duration(&self, seconds: f64) -> HalResult<()> {
        let duration_ms: i32 = (seconds * 1000.0) as i32;
        hal_call!(HAL_SetOneShotDuration(self.solenoid_handle, duration_ms))
    }

    pub fn start_pulse(&self) -> HalResult<()> {
        hal_call!(HAL_FireOneShot(self.solenoid_handle))
    }

    pub fn module(&self) -> &SolenoidModule {
        &self.module
    }
}

impl Drop for Solenoid {
    fn drop(&mut self) {
        unsafe { HAL_FreeSolenoidPort(self.solenoid_handle) }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Action {
    Forward,
    Reverse,
    Off,
}

pub struct DoubleSolenoid {
    forward: Solenoid,
    reverse: Solenoid,
}

impl DoubleSolenoid {
    pub fn new(forward_channel: i32, reverse_channel: i32) -> HalResult<DoubleSolenoid> {
        Self::new_with_module(
            sensor_util::default_solenoid_module(),
            forward_channel,
            reverse_channel,
        )
    }

    pub fn new_with_module(
        module_number: i32,
        forward_channel: i32,
        reverse_channel: i32,
    ) -> HalResult<DoubleSolenoid> {
        Ok(DoubleSolenoid {
            forward: Solenoid::new_with_module(module_number, forward_channel)?,
            reverse: Solenoid::new_with_module(module_number, reverse_channel)?,
        })
    }

    pub fn set(&self, action: Action) -> HalResult<()> {
        let forward;
        let reverse;
        match action {
            Action::Forward => {
                forward = true;
                reverse = false;
            }
            Action::Reverse => {
                forward = false;
                reverse = true;
            }
            Action::Off => {
                forward = false;
                reverse = false;
            }
        };
        self.forward.set(forward)?;
        self.reverse.set(reverse)?;
        Ok(())
    }

    pub fn get(&self) -> HalResult<Action> {
        if self.forward.get()? {
            return Ok(Action::Forward);
        };
        if self.reverse.get()? {
            return Ok(Action::Reverse);
        };
        Ok(Action::Off)
    }

    pub fn is_fwd_blacklisted(&self) -> bool {
        self.forward.is_blacklisted()
    }

    pub fn is_rev_blacklisted(&self) -> bool {
        self.reverse.is_blacklisted()
    }

    pub fn module(&self) -> &SolenoidModule {
        self.forward.module()
    }
}

pub struct Compressor {
    compressor_handle: HAL_CompressorHandle,
    module: i32,
}

impl Compressor {
    pub fn new() -> HalResult<Self> {
        Self::new_with_module(sensor_util::default_solenoid_module())
    }

    pub fn new_with_module(module: i32) -> HalResult<Self> {
        let compressor_handle = hal_call!(HAL_InitializeCompressor(module))?;
        Ok(Self {
            compressor_handle,
            module,
        })
    }

    pub fn set_closed_loop_control(&self, on: bool) {
        hal_call!(HAL_SetCompressorClosedLoopControl(
            self.compressor_handle,
            on as i32
        ))
        .ok();
    }

    pub fn start(&self) {
        self.set_closed_loop_control(true);
    }

    pub fn stop(&self) {
        self.set_closed_loop_control(false);
    }

    pub fn enabled(&self) -> bool {
        maybe_hal_call!(HAL_GetCompressor(self.compressor_handle)).ok() != 0
    }

    pub fn get_pressure_switch_value(&self) -> bool {
        maybe_hal_call!(HAL_GetCompressorPressureSwitch(self.compressor_handle)).ok() != 0
    }

    pub fn get_compressor_current(&self) -> f64 {
        maybe_hal_call!(HAL_GetCompressorCurrent(self.compressor_handle)).ok()
    }

    pub fn get_closed_loop_control(&self) -> bool {
        maybe_hal_call!(HAL_GetCompressorClosedLoopControl(self.compressor_handle)).ok() != 0
    }

    pub fn get_compressor_current_too_high_fault(&self) -> bool {
        maybe_hal_call!(HAL_GetCompressorCurrentTooHighStickyFault(
            self.compressor_handle
        ))
        .ok()
            != 0
    }

    pub fn get_compressor_current_too_high_sticky_fault(&self) -> bool {
        maybe_hal_call!(HAL_GetCompressorCurrentTooHighStickyFault(
            self.compressor_handle
        ))
        .ok()
            != 0
    }

    pub fn get_compressor_shorted_sticky_fault(&self) -> bool {
        maybe_hal_call!(HAL_GetCompressorShortedStickyFault(self.compressor_handle)).ok() != 0
    }

    pub fn get_compressor_shorted_fault(&self) -> bool {
        maybe_hal_call!(HAL_GetCompressorShortedFault(self.compressor_handle)).ok() != 0
    }

    pub fn get_compressor_not_connected_sticky_fault(&self) -> bool {
        maybe_hal_call!(HAL_GetCompressorNotConnectedStickyFault(
            self.compressor_handle
        ))
        .ok()
            != 0
    }

    pub fn get_compressor_not_connected_fault(&self) -> bool {
        maybe_hal_call!(HAL_GetCompressorNotConnectedFault(self.compressor_handle)).ok() != 0
    }

    pub fn clear_all_pcm_sticky_faults(&self) {
        hal_call!(HAL_ClearAllPCMStickyFaults(self.module)).ok();
    }
}
