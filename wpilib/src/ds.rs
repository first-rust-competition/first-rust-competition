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

// TODO(Lytigas) re-architecht the Driverstation
#![allow(clippy::mutex_atomic)]

use wpilib_sys::*;

const JOYSTICK_PORTS: usize = 6;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum JoystickError {
    PortDNE,
    ButtonUnplugged,
    AxisUnplugged,
    AxisDNE,
    PovDNE,
    PovUnplugged,
}

/// A type representing a valid Joystick port
#[derive(Copy, Clone, Debug)]
pub struct JoystickPort(i32);
impl JoystickPort {
    /// Creates a new port from a port number
    /// # Errors
    /// `PortDNE` if `port` is greater than 6.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(port: u8) -> Result<Self, JoystickError> {
        if port as usize >= JOYSTICK_PORTS {
            Err(JoystickError::PortDNE)
        } else {
            Ok(Self(i32::from(port)))
        }
    }
}

/// A type representing a valid Joystick axis
#[derive(Copy, Clone, Debug)]
pub struct JoystickAxis(usize);
impl JoystickAxis {
    /// Creates a new axis from a port number
    /// # Errors
    /// `AxisDNE` if `axis` is an invalid index.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(axis: u8) -> Result<Self, JoystickError> {
        if u32::from(axis) >= HAL_kMaxJoystickAxes {
            Err(JoystickError::PortDNE)
        } else {
            Ok(Self(usize::from(axis)))
        }
    }
}

/// A type representing a valid Joystick axis
#[derive(Copy, Clone, Debug)]
pub struct JoystickPOV(usize);
impl JoystickPOV {
    /// Creates a new POV hat from a port number
    /// # Errors
    /// `PovDNE` if `pov` is an invalid index.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(pov: u8) -> Result<Self, JoystickError> {
        if u32::from(pov) >= HAL_kMaxJoystickPOVs {
            Err(JoystickError::PovDNE)
        } else {
            Ok(Self(usize::from(pov)))
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
                HAL_MatchType_HAL_kMatchType_practice => MatchType::Practice,
                HAL_MatchType_HAL_kMatchType_qualification => MatchType::Qualification,
                HAL_MatchType_HAL_kMatchType_elimination => MatchType::Elimination,
                _ => MatchType::None,
            },
        }
    }
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
            Ok(Self(base))
        }
    }

    /// Whether the 0-indexed button `button` is held on the controller on `port`
    /// # Errors
    /// `ButtonUnplugged` if the requested button does not exist on the controller. This may mean it is
    /// unplugged
    #[inline]
    pub fn stick_button(&self, port: JoystickPort, button: u8) -> Result<bool, JoystickError> {
        let mut buttons: HAL_JoystickButtons = Default::default();
        unsafe {
            HAL_GetJoystickButtons(port.0, &mut buttons);
        }

        if button >= buttons.count {
            return Err(JoystickError::ButtonUnplugged);
        }
        Ok(buttons.buttons & (1 << button) != 0)
    }

    /// The value of `axis` on the controller on `port`
    /// # Errors
    /// `AxisUnplugged` if the requested axis does not exist on the controller. This may mean it is
    /// unplugged
    #[inline]
    pub fn stick_axis(&self, port: JoystickPort, axis: JoystickAxis) -> Result<f32, JoystickError> {
        let mut axes: HAL_JoystickAxes = Default::default();
        unsafe {
            HAL_GetJoystickAxes(port.0, &mut axes);
        }

        if axis.0 > axes.count as usize {
            return Err(JoystickError::AxisUnplugged);
        }
        Ok(axes.axes[axis.0])
    }

    /// The value of `pov` on the controller on `port`
    /// # Errors
    /// `AxisUnplugged` if the requested axis does not exist on the controller. This may mean it is
    /// unplugged
    #[inline]
    pub fn stick_pov(&self, port: JoystickPort, pov: JoystickPOV) -> Result<i16, JoystickError> {
        let mut povs: HAL_JoystickPOVs = Default::default();
        unsafe {
            HAL_GetJoystickPOVs(port.0, &mut povs);
        }

        if pov.0 > povs.count as usize {
            return Err(JoystickError::PovUnplugged);
        }
        Ok(povs.povs[pov.0])
    }

    /// The alliance the robot is on.
    #[allow(non_upper_case_globals)]
    pub fn alliance(&self) -> HalResult<Alliance> {
        match hal_call!(HAL_GetAllianceStation())? {
            HAL_AllianceStationID_HAL_AllianceStationID_kRed1
            | HAL_AllianceStationID_HAL_AllianceStationID_kRed2
            | HAL_AllianceStationID_HAL_AllianceStationID_kRed3 => Ok(Alliance::Red),
            HAL_AllianceStationID_HAL_AllianceStationID_kBlue1
            | HAL_AllianceStationID_HAL_AllianceStationID_kBlue2
            | HAL_AllianceStationID_HAL_AllianceStationID_kBlue3 => Ok(Alliance::Blue),
            _ => Err(HalError(0)),
        }
    }

    /// The id for the station the driver station is at, as an integer.
    #[allow(non_upper_case_globals)]
    pub fn station(&self) -> HalResult<u32> {
        match hal_call!(HAL_GetAllianceStation())? {
            HAL_AllianceStationID_HAL_AllianceStationID_kRed1
            | HAL_AllianceStationID_HAL_AllianceStationID_kBlue1 => Ok(1),
            HAL_AllianceStationID_HAL_AllianceStationID_kRed2
            | HAL_AllianceStationID_HAL_AllianceStationID_kBlue2 => Ok(2),
            HAL_AllianceStationID_HAL_AllianceStationID_kRed3
            | HAL_AllianceStationID_HAL_AllianceStationID_kBlue3 => Ok(3),
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
