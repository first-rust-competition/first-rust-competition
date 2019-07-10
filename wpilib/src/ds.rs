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

/// A type representing a valid Joystick axis
#[derive(Copy, Clone, Debug)]
pub struct JoystickAxis(usize);
impl JoystickAxis {
    /// Axis 0, commonly the X axis on a joystick.
    pub const X: Self = Self(0);
    /// Axis 1, commonly the Y axis on a joystick.
    pub const Y: Self = Self(1);
    /// Axis 2, commonly the Z axis or twist on a joystick.
    pub const Z: Self = Self(2);
    /// Axis 2, commonly the Z axis or twist on a joystick.
    pub const TWIST: Self = Self::Z;
    /// Axis 3, commonly the throttle on a joystick.
    pub const THROTTLE: Self = Self(3);

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

/// A type representing a valid Joystick axis
#[derive(Copy, Clone, Debug)]
pub struct JoystickPOV(usize);
impl JoystickPOV {
    /// Creates a new POV without checking the value.
    pub const unsafe fn new_unchecked(pov: u8) -> Self {
        JoystickPOV(pov as usize)
    }

    /// Creates a new POV hat from a port number
    pub fn new(pov: u8) -> Option<Self> {
        if u32::from(pov) >= HAL_kMaxJoystickPOVs {
            None
        } else {
            Some(JoystickPOV(usize::from(pov)))
        }
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

    /// Whether the 0-indexed button `button` is held on the controller on `port`
    /// # Errors
    /// Returns None if the requested button does not exist on the controller.
    /// This may mean it is unplugged.
    #[inline]
    pub fn stick_button(&self, port: JoystickPort, button: u8) -> Option<bool> {
        let buttons = stick_buttons(port);

        if button >= buttons.count {
            return None;
        }
        Some(buttons.buttons & (1 << button) != 0)
    }

    /// The value of `axis` on the controller on `port`
    /// # Errors
    /// Returns None if the requested axis does not exist on the controller.
    /// This may mean it is unplugged.
    #[inline]
    pub fn stick_axis(&self, port: JoystickPort, axis: JoystickAxis) -> Option<f32> {
        let axes = stick_axes(port);

        if axis.0 > axes.count as usize {
            return None;
        }
        Some(axes.axes[axis.0])
    }

    /// The value of `pov` on the controller on `port`
    /// # Errors
    /// Returns None if the requested hat does not exist on the controller.
    /// This may mean it is unplugged.
    #[inline]
    pub fn stick_pov(&self, port: JoystickPort, pov: JoystickPOV) -> Option<i16> {
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
