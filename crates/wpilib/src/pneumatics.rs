//! Access to the pneumatic HAL.

use wpilib_sys::*;

/// Check if a solenoid module is valid.
fn check_solenoid_module(channel: i32) -> bool {
    unsafe { HAL_CheckCTREPCMSolenoidChannel(channel) != 0 }
}

/// Represents a CTRE Pneumatics Control Module (PCM).
///
/// This struct serves two purposes. First, to check faults on any PCM.
/// Second, control pnuematic components connected to a PCM that isn't on CAN
/// module 0 (the default). If your Robot has only one PCM, you will most
/// likely never need this struct, as all Pneumatic components have
/// constructors that assume the default PCM.
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

    /// Returns a PCM with the default ID of 0.
    pub fn new() -> Self {
        Self::DEFAULT
    }

    /// Creates a PCM object for a given ID.
    pub fn with_id(module_id: i32) -> Option<Self> {
        if check_solenoid_module(module_id) {
            Some(PneumaticsControlModule(module_id))
        } else {
            None
        }
    }

    /// Returns the module ID.
    #[inline]
    pub fn id(self) -> i32 {
        self.0
    }

    /// Gets the state of each solenoid on the PCM.
    /// Returns a bit mask.
    pub fn all(self) -> HalResult<i32> {
        hal_call!(HAL_GetCTREPCMSolenoids(self.0))
    }

    pub fn solenoid_blacklist(self) -> i32 {
        maybe_hal_call!(HAL_GetCTREPCMSolenoidDisabledList(self.0)).ok()
    }

    pub fn solenoid_voltage_sticky_fault(self) -> bool {
        maybe_hal_call!(HAL_GetCTREPCMSolenoidVoltageStickyFault(self.0)).ok() != 0
    }

    pub fn solenoid_voltage_fault(self) -> bool {
        maybe_hal_call!(HAL_GetCTREPCMSolenoidVoltageFault(self.0)).ok() != 0
    }

    pub fn clear_all_sticky_faults(&mut self) -> HalResult<()> {
        hal_call!(HAL_ClearAllCTREPCMStickyFaults(self.0))
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

    // See `Compressor`.
    // pub fn compressor(self) -> Compressor {
    //    Compressor::with_module(self)
    // }
}

#[derive(Debug)]
pub struct Solenoid {
    handle: HAL_SolenoidHandle,
    channel: i32,
    module: PneumaticsControlModule,
}

impl Solenoid {
    /// Creates a new solenoid on the default PCM with the given channel.
    pub fn new(channel: i32) -> HalResult<Solenoid> {
        Self::with_module(PneumaticsControlModule::DEFAULT, channel)
    }

    /// If for some reason the Pneumatic Control Module is not on CAN module 0,
    /// you can use this constructor. Most people will never need this.
    pub fn with_module(module: PneumaticsControlModule, channel: i32) -> HalResult<Solenoid> {
        let module_id = module.id();

        let handle = hal_call!(HAL_InitializeCTREPCM(
            HAL_GetPortWithModule(module_id, channel),
            std::ptr::null()
        ))?;

        usage::report_context(
            usage::resource_types::Solenoid,
            channel as usage::instances::Type,
            module_id,
        );

        Ok(Solenoid {
            handle,
            channel,
            module,
        })
    }

    /// Sets the solenoid to on or off
    pub fn set(&mut self, on: bool) -> HalResult<()> {
        hal_call!(HAL_SetCTREPCMClosedLoopControl(self.handle, on as HAL_Bool))
    }

    /// Gets the state of the solenoid by calling out to the hardware through an FFI.
    /// If you need speed, consider caching the value you set yourself!
    pub fn get(&self) -> HalResult<bool> {
        Ok(hal_call!(HAL_GetCTREPCMClosedLoopControl(self.handle))? != 0)
    }

    pub fn is_blacklisted(&self) -> bool {
        (self.module.solenoid_blacklist() & (1 << self.channel)) != 0
    }

    // TODO: verify the role of the `index` argument in the next two methods (currently defaultint to zero).
    pub fn set_pulse_duration(&mut self, seconds: f64) -> HalResult<()> {
        let duration_ms: i32 = (seconds * 1000.0) as i32;
        hal_call!(HAL_SetCTREPCMOneShotDuration(self.handle, duration_ms, 0))
    }

    pub fn start_pulse(&mut self) -> HalResult<()> {
        hal_call!(HAL_FireCTREPCMOneShot(self.handle, 0))
    }

    pub fn module(&self) -> &PneumaticsControlModule {
        &self.module
    }
}

impl Drop for Solenoid {
    fn drop(&mut self) {
        unsafe { HAL_FreeCTREPCM(self.handle) }
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
    /// Creates a `DoubleSolenoid`from any two existing Solenoids.
    ///
    /// The primary use case for this constructor is the create a
    /// `DoubleSolenoid` from two solenoids on different PCMs. Otherwise,
    /// [`::with_channels`](#method.with_channels) or
    /// [`::with_module`](#method.with_module) should be used instead.
    pub fn from_solenoids(forward: Solenoid, reverse: Solenoid) -> Self {
        DoubleSolenoid { forward, reverse }
    }

    /// Creates a `DoubleSolenoid` on the default PCM with the given channels.
    /// This is the most common constructor.
    ///
    /// To create a `DoubleSolenoid` not on the default PCM, use
    /// [`::with_module`](#method.with_module).
    pub fn with_channels(forward_channel: i32, reverse_channel: i32) -> HalResult<DoubleSolenoid> {
        Self::with_module(
            PneumaticsControlModule::DEFAULT,
            forward_channel,
            reverse_channel,
        )
    }

    /// Creates a `DoubleSolenoid`on the given PCM using the given channels.
    ///
    /// If each solenoid is connected to a different PCM, create each
    /// `Solenoid` individually and use
    /// [`::from_solenoids`](#method.from_solenoids).
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

/*
 * TODO: find the HAL Compressor bindings.
/// Represents a compressor hooked up to a PCM.
/// Note that a Compressor object does not need to created for a compressor to function.
/// A Compressor object is only used alter the default closed loop behavior.
#[derive(Debug)]
pub struct Compressor {
    handle: HAL_CompressorHandle,
}

impl Default for Compressor {
    #[inline]
    /// Returns the `Compressor` on the default PCM.
    fn default() -> Self {
        Self::new()
    }
}

impl Compressor {
    /// Returns the `Compressor` on the default PCM.
    pub fn new() -> Self {
        Self::with_module(PneumaticsControlModule::DEFAULT)
    }

    /// Creates a `Compressor` on the given PCM.
    pub fn with_module(module: PneumaticsControlModule) -> Self {
        // HAL_InitializeCompressor returns an error if the module number is
        // invalid, but PneumaticsControlModule already guarantees it's valid.
        let handle = hal_call!(HAL_InitializeCompressor(module.id())).unwrap();
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
*/
