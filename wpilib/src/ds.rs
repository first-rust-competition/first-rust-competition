/* PORTIONS OF THIS FILE WERE ORIGINALLY DISTRIBUTED WITH THE FOLLOWING LICENSE

"""
MIT License
Copyright (c) 2017 Rust for Robotics Developers
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"""

Copyright 2018 First Rust Competition Developers.
Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
option. This file may not be copied, modified, or distributed
except according to those terms.
*/

//! Interface to data from the driver station.

// TODO(Lytigas) re-architecht the Driverstation
#![allow(clippy::mutex_atomic)]

use wpilib_sys::*;

const JOYSTICK_PORTS: usize = 6;

/// A type representing a valid Joystick port
#[derive(Copy, Clone, Debug)]
pub struct JoystickPort(i32);
impl JoystickPort {
    /// Creates a new port without checking the value.
    pub const unsafe fn new_unchecked(port: u8) -> Self {
        JoystickPort(port as i32)
    }

    /// Creates a new port from a port number if it is valid.
    pub fn new(port: u8) -> Option<Self> {
        if port as usize >= JOYSTICK_PORTS {
            None
        } else {
            Some(JoystickPort(i32::from(port)))
        }
    }
}

/// A valid joystick axis index.
#[derive(Copy, Clone, Debug)]
pub struct JoystickAxis(usize);
impl JoystickAxis {
    /// Axis 0, commonly the X axis on a joystick.
    /// This is the left thumbstick X axis on an XInput controller.
    pub const X: Self = Self(0);
    /// Axis 1, commonly the Y axis on a joystick.
    /// This is the left thumbstick Y axis on an XInput controller.
    pub const Y: Self = Self(1);
    /// Axis 2, commonly the Z axis or twist on a joystick.
    pub const Z: Self = Self(2);
    /// Axis 2, commonly the Z axis or twist on a joystick.
    pub const TWIST: Self = Self::Z;
    /// Axis 3, commonly the throttle on a joystick.
    pub const THROTTLE: Self = Self(3);

    /// Axis 4, the right thumbstick X axis on an XInput controller
    /// (such as Xbox controllers).
    pub const RIGHT_X: Self = Self(4);
    /// Axis 5, the right thumbstick X axis on an XInput controller
    /// (such as Xbox controllers).
    pub const RIGHT_Y: Self = Self(5);
    /// Axis 2, the left trigger axis on an XInput controller
    /// (such as Xbox controllers).
    pub const LEFT_TRIGGER: Self = Self(2);
    /// Axis 3, the right trigger axis on an XInput controller
    /// (such as Xbox controllers).
    pub const RIGHT_TRIGGER: Self = Self(3);

    /// Creates a new axis without checking the value.
    pub const unsafe fn new_unchecked(port: u8) -> Self {
        JoystickAxis(port as usize)
    }

    /// Creates a new axis from an axis index if the index is valid.
    pub fn new(axis: u8) -> Option<Self> {
        if u32::from(axis) >= HAL_kMaxJoystickAxes {
            None
        } else {
            Some(JoystickAxis(usize::from(axis)))
        }
    }
}

/// A valid joystick POV hat index.
#[derive(Copy, Clone, Debug, Default)]
pub struct JoystickPov(usize);
impl JoystickPov {
    /// Creates a new POV without checking the value.
    pub const unsafe fn new_unchecked(pov: u8) -> Self {
        Self(pov as usize)
    }

    /// Creates a new POV hat from a port number
    pub fn new(pov: u8) -> Option<Self> {
        if u32::from(pov) >= HAL_kMaxJoystickPOVs {
            None
        } else {
            Some(Self(usize::from(pov)))
        }
    }
}

#[deprecated(since = "0.5.0", note = "renamed to `JoystickPov`")]
pub type JoystickPOV = JoystickPov;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
/// Buttons for XInput controllers (such as Xbox controllers).
pub enum XInputButton {
    A,
    B,
    X,
    Y,

    /// The left bumper/shoulder button.
    LeftBumper,
    /// The right bumper/shoulder button.
    RightBumper,

    Back,
    Start,

    /// The left thumbstick button.
    LeftThumb,
    /// The right thumbstick button.
    RightThumb,
}

impl From<XInputButton> for u8 {
    #[inline]
    fn from(button: XInputButton) -> u8 {
        button as u8
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct JoystickButtons(HAL_JoystickButtons);
impl JoystickButtons {
    /// Get the state of the 0-indexed button `button`.
    ///
    /// Returns None if the button doesn't exist.
    /// This may mean the controller is unplugged.
    ///
    /// This method may take a `u8` or an [`XInputButton`].
    ///
    /// [`XInputButton`]: enum.XInputButton.html
    pub fn get(self, button: impl Into<u8>) -> Option<bool> {
        let button = button.into();
        if button >= self.0.count {
            None
        } else {
            Some(self.0.buttons & (1 << button) != 0)
        }
    }

    /// Get the number of buttons read.
    /// Returns 0 if the controller is unplugged.
    pub fn count(self) -> u8 {
        self.0.count
    }

    /// Get the raw bits representing the current button states.
    pub fn bits(self) -> u32 {
        self.0.buttons
    }

    /// Get the set of buttons that are currently pressed,
    /// but were not pressed when `other` was read.
    pub fn pressed_since(self, other: Self) -> Self {
        Self(HAL_JoystickButtons {
            count: self.0.count,
            buttons: self.0.buttons & !other.0.buttons,
        })
    }

    /// Get the set of buttons that are currently unpressed,
    /// but were pressed when `other` was read.
    pub fn released_since(self, other: Self) -> Self {
        Self(HAL_JoystickButtons {
            count: self.0.count,
            buttons: !self.0.buttons & other.0.buttons,
        })
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct JoystickAxes(HAL_JoystickAxes);
impl JoystickAxes {
    /// Get the value of the given axis.
    ///
    /// Returns None if the axis doesn't exist.
    /// This may mean the controller is unplugged.
    pub fn get(&self, axis: JoystickAxis) -> Option<f32> {
        if axis.0 > self.0.count as usize {
            None
        } else {
            Some(self.0.axes[axis.0])
        }
    }

    /// Equivalent to `get(JoystickAxis::X)`.
    pub fn x(&self) -> Option<f32> {
        self.get(JoystickAxis::X)
    }

    /// Equivalent to `get(JoystickAxis::Y)`.
    pub fn y(&self) -> Option<f32> {
        self.get(JoystickAxis::Y)
    }

    /// Get the number of axes read.
    /// Returns 0 if the controller is unplugged.
    pub fn count(&self) -> usize {
        self.0.count as usize
    }

    /// Get all the axes read.
    pub fn all(&self) -> &[f32] {
        &self.0.axes[..self.0.count as usize]
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct JoystickPovs(HAL_JoystickPOVs);
impl JoystickPovs {
    /// Get the value of the given POV hat.
    ///
    /// Returns None if the POV hat doesn't exist.
    /// This may mean the controller is unplugged.
    pub fn get(&self, pov: JoystickPov) -> Option<i16> {
        if pov.0 > self.0.count as usize {
            None
        } else {
            Some(self.0.povs[pov.0])
        }
    }

    /// Get the value of POV hat 0, or `None` if there are no POV hats.
    pub fn first(&self) -> Option<i16> {
        self.get(JoystickPov(0))
    }

    /// Get the number of POV hats read.
    /// Returns 0 if the controller is unplugged.
    pub fn count(&self) -> usize {
        self.0.count as usize
    }

    /// Get all the axes read.
    pub fn all(&self) -> &[i16] {
        &self.0.povs[..self.0.count as usize]
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Alliance {
    Red,
    Blue,
}

#[derive(Debug, Copy, Clone)]
enum MatchType {
    None,
    Practice,
    Qualification,
    Elimination,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum RobotState {
    Disabled,
    Autonomous,
    Teleop,
    Test,
    EStop,
}

#[derive(Debug, Clone)]
struct MatchInfoData {
    event_name: String,
    game_specific_message: Vec<u8>,
    match_number: u16,
    replay_number: u8,
    match_type: MatchType,
}

use std::ffi::CStr;
use std::os::raw::c_char;
#[allow(non_upper_case_globals)]
impl From<HAL_MatchInfo> for MatchInfoData {
    fn from(info: HAL_MatchInfo) -> MatchInfoData {
        let mut cs = info.eventName;
        cs[cs.len() - 1] = 0;

        use self::HAL_MatchType::*;
        Self {
            event_name: unsafe { CStr::from_ptr(&cs as *const c_char) }
                .to_string_lossy()
                .into_owned(),
            game_specific_message: info.gameSpecificMessage
                [0..info.gameSpecificMessageSize as usize]
                .to_vec(),
            match_number: info.matchNumber,
            replay_number: info.replayNumber,
            match_type: match info.matchType {
                HAL_kMatchType_practice => MatchType::Practice,
                HAL_kMatchType_qualification => MatchType::Qualification,
                HAL_kMatchType_elimination => MatchType::Elimination,
                _ => MatchType::None,
            },
        }
    }
}

fn stick_buttons(port: JoystickPort) -> HAL_JoystickButtons {
    let mut buttons: HAL_JoystickButtons = Default::default();
    unsafe { HAL_GetJoystickButtons(port.0, &mut buttons) };
    buttons
}

fn stick_axes(port: JoystickPort) -> HAL_JoystickAxes {
    let mut axes: HAL_JoystickAxes = Default::default();
    unsafe { HAL_GetJoystickAxes(port.0, &mut axes) };
    axes
}

fn stick_povs(port: JoystickPort) -> HAL_JoystickPOVs {
    let mut povs: HAL_JoystickPOVs = Default::default();
    unsafe { HAL_GetJoystickPOVs(port.0, &mut povs) };
    povs
}

use super::robot_base::RobotBase;

#[derive(Clone, Debug)]
pub struct DriverStation<'a>(&'a RobotBase);

// All methods on here MUST require &self
// The ONLY thing allowed to create instances of DriverStation is RobotBase,
// which ensures the HAL has been initialized successfully.
impl<'a> DriverStation<'a> {
    #[inline]
    pub(crate) fn from_base(base: &'a RobotBase) -> Result<Self, ()> {
        if unsafe { HAL_Initialize(500, 0) } == 0 {
            Err(())
        } else {
            Ok(DriverStation(base))
        }
    }

    /// Read the current buttons state from the given controller.
    pub fn stick_buttons(&self, port: JoystickPort) -> JoystickButtons {
        JoystickButtons(stick_buttons(port))
    }

    /// Whether the 0-indexed button `button` is held on the controller on `port`
    /// # Errors
    /// Returns None if the requested button does not exist on the controller.
    /// This may mean it is unplugged.
    #[deprecated(since = "0.5.0", note = "use `stick_buttons` instead")]
    #[inline]
    pub fn stick_button(&self, port: JoystickPort, button: u8) -> Option<bool> {
        let buttons = stick_buttons(port);

        if button >= buttons.count {
            return None;
        }
        Some(buttons.buttons & (1 << button) != 0)
    }

    /// Read the current axes from the given controller.
    pub fn stick_axes(&self, port: JoystickPort) -> JoystickAxes {
        JoystickAxes(stick_axes(port))
    }

    /// The value of `axis` on the controller on `port`
    /// # Errors
    /// Returns None if the requested axis does not exist on the controller.
    /// This may mean it is unplugged.
    #[deprecated(since = "0.5.0", note = "use `stick_axes` instead")]
    #[inline]
    pub fn stick_axis(&self, port: JoystickPort, axis: JoystickAxis) -> Option<f32> {
        let axes = stick_axes(port);

        if axis.0 > axes.count as usize {
            return None;
        }
        Some(axes.axes[axis.0])
    }

    /// Read the current POV hat directions from the given controller.
    pub fn stick_povs(&self, port: JoystickPort) -> JoystickPovs {
        JoystickPovs(stick_povs(port))
    }

    /// The value of `pov` on the controller on `port`
    /// # Errors
    /// Returns None if the requested hat does not exist on the controller.
    /// This may mean it is unplugged.
    #[deprecated(since = "0.5.0", note = "use `stick_povs` instead")]
    #[inline]
    pub fn stick_pov(&self, port: JoystickPort, pov: JoystickPov) -> Option<i16> {
        let povs = stick_povs(port);

        if pov.0 > povs.count as usize {
            return None;
        }
        Some(povs.povs[pov.0])
    }

    /// The alliance the robot is on.
    #[allow(non_upper_case_globals)]
    pub fn alliance(&self) -> HalResult<Alliance> {
        use self::HAL_AllianceStationID::*;
        match hal_call!(HAL_GetAllianceStation())? {
            kRed1 | kRed2 | kRed3 => Ok(Alliance::Red),
            kBlue1 | kBlue2 | kBlue3 => Ok(Alliance::Blue),
            _ => Err(HalError(0)),
        }
    }

    /// The id for the station the driver station is at, as an integer.
    #[allow(non_upper_case_globals)]
    pub fn station(&self) -> HalResult<u32> {
        use self::HAL_AllianceStationID::*;
        match hal_call!(HAL_GetAllianceStation())? {
            kRed1 | kBlue1 => Ok(1),
            kRed2 | kBlue2 => Ok(2),
            kRed3 | kBlue3 => Ok(3),
            _ => Err(HalError(0)),
        }
    }

    pub fn robot_state(&self) -> RobotState {
        let mut control_word: HAL_ControlWord = Default::default();
        unsafe {
            HAL_GetControlWord(&mut control_word);
        }
        if control_word.enabled() != 0 {
            if control_word.autonomous() != 0 {
                RobotState::Autonomous
            } else {
                RobotState::Teleop
            }
        } else if control_word.eStop() != 0 {
            RobotState::EStop
        } else {
            RobotState::Disabled
        }
    }

    pub fn ds_attached(&self) -> bool {
        let mut control_word: HAL_ControlWord = Default::default();
        unsafe {
            HAL_GetControlWord(&mut control_word);
        }
        control_word.dsAttached() != 0
    }

    pub fn fms_attached(&self) -> bool {
        let mut control_word: HAL_ControlWord = Default::default();
        unsafe {
            HAL_GetControlWord(&mut control_word);
        }
        control_word.fmsAttached() != 0
    }

    pub fn game_specific_message(&self) -> Vec<u8> {
        let mut info: HAL_MatchInfo = Default::default();
        unsafe {
            HAL_GetMatchInfo(&mut info);
        }

        info.gameSpecificMessage[0..info.gameSpecificMessageSize as usize].to_vec()
    }

    /// Blocks until a new data packet arrives
    pub fn wait_for_data(&self) {
        unsafe {
            HAL_WaitForDSData();
        }
    }
}

// Drop requirements implemented on RobotBase::drop
