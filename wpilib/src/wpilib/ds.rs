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

This file is part of "first-rust-competition", which is free software: you can
redistribute it and/or modify it under the terms of the GNU General Public
License version 3 as published by the Free Software Foundation. See
<https://www.gnu.org/licenses/> for a copy.
*/

// TODO(Lytigas) re-architecht the Driverstation
#![allow(clippy::mutex_atomic)]

use super::robot_base::RobotBase;
use super::time::Throttler;
use hal::*;
use std::ffi;
use std::sync::*;
use std::thread;
use std::time;

const JOYSTICK_PORTS: usize = 6;
const JOYSTICK_AXES: usize = 12;
const JOYSTICK_POVS: usize = 12;

#[derive(Debug, Copy, Clone)]
pub enum Alliance {
    Red,
    Blue,
}

// #[derive(Debug, Copy, Clone)]
// enum MatchType {
//     None,
//     Practice,
//     Qualification,
//     Elimination,
// }

#[derive(Debug, Copy, Clone)]
pub enum RobotState {
    Disabled,
    Autonomous,
    Teleop,
    Test,
    EStop,
}

// TODO: implement matchinfo data
// struct MatchInfoData {
//     event_name: String,
//     game_specific_message: String,
//     match_number: u32,
//     replay_number: u32,
//     match_type: MatchType,
// }

#[derive(Default)]
struct Joysticks {
    axes: [HAL_JoystickAxes; JOYSTICK_PORTS],
    povs: [HAL_JoystickPOVs; JOYSTICK_PORTS],
    buttons: [HAL_JoystickButtons; JOYSTICK_PORTS],
    descriptor: [HAL_JoystickDescriptor; JOYSTICK_PORTS],
}

#[derive(Debug, Copy, Clone)]
pub enum JoystickError {
    JoystickDNE,
    ChannelUnplugged,
    ChannelDNE,
    DsUnreachable,
}

pub struct DriverStation {
    joysticks: Joysticks,
    control_word: HAL_ControlWord,
    state: RobotState,
    fms_attached: bool,
    ds_attached: bool,
    report_throttler: Throttler<u64, u64>,
    condvar: Arc<(Mutex<bool>, Condvar)>,
    join: Option<thread::JoinHandle<()>>,
}

pub type ThreadSafeDs = Arc<RwLock<DriverStation>>;

impl DriverStation {
    pub(crate) fn new() -> Self {
        DriverStation {
            joysticks: Joysticks::default(),
            control_word: HAL_ControlWord::default(),
            state: RobotState::Disabled,
            fms_attached: false,
            ds_attached: false,
            report_throttler: Throttler::new(RobotBase::fpga_time().unwrap(), 1_000_000),
            condvar: Arc::new((Mutex::new(false), Condvar::new())),
            join: None,
        }
    }
    /// Spawns a thread to read from the physical driver station and pass the data to the given
    /// virtual driverstation
    pub(crate) fn spawn_updater(ds: &mut Arc<RwLock<DriverStation>>) {
        let updater_pointer = ds.clone();
        let mut write_lock = ds.write().unwrap();
        if write_lock.join.is_some() {
            return;
        }
        write_lock.join = Some(thread::spawn(move || {
            loop {
                unsafe {
                    HAL_WaitForDSData();
                }

                // Update the joysticks and control word using the new data.
                let mut joysticks = Joysticks::default();
                for stick in 0..JOYSTICK_PORTS {
                    unsafe {
                        HAL_GetJoystickAxes(
                            stick as i32,
                            &mut joysticks.axes[stick] as *mut HAL_JoystickAxes,
                        );
                        HAL_GetJoystickPOVs(
                            stick as i32,
                            &mut joysticks.povs[stick] as *mut HAL_JoystickPOVs,
                        );
                        HAL_GetJoystickButtons(
                            stick as i32,
                            &mut joysticks.buttons[stick] as *mut HAL_JoystickButtons,
                        );
                        HAL_GetJoystickDescriptor(
                            stick as i32,
                            &mut joysticks.descriptor[stick] as *mut HAL_JoystickDescriptor,
                        );
                    }
                }
                let mut control_word: HAL_ControlWord = HAL_ControlWord::default();
                unsafe {
                    HAL_GetControlWord(&mut control_word as *mut HAL_ControlWord);
                }
                // copy data over
                {
                    let mut write_lock = updater_pointer.write().unwrap();
                    write_lock.joysticks = joysticks;
                    write_lock.update_data(control_word);
                }
                // notify threads
                {
                    let read_lock = updater_pointer.read().unwrap();
                    let mut guard = read_lock.condvar.0.lock().unwrap();
                    *guard = true;
                    read_lock.condvar.1.notify_all();
                }
            }
        }));
    }

    fn update_data(&mut self, control_word: HAL_ControlWord) {
        self.control_word = control_word;
        self.state = if self.control_word.enabled() != 0 {
            if self.control_word.autonomous() != 0 {
                RobotState::Autonomous
            } else {
                RobotState::Teleop
            }
        } else if self.control_word.eStop() != 0 {
            RobotState::EStop
        } else {
            RobotState::Disabled
        };
        self.fms_attached = self.control_word.fmsAttached() != 0;
        self.ds_attached = self.control_word.dsAttached() != 0;
    }

    /// Report an error to the driver station in its most general form. Don't use this directly,
    /// instead use it in other error reporting methods.
    fn report(&self, is_error: bool, code: i32, error: &str, location: &str, stack: &str) {
        unsafe {
            HAL_SendError(
                is_error as i32,
                code,
                false as i32,
                ffi::CString::new(error).unwrap().into_raw(),
                ffi::CString::new(location).unwrap().into_raw(),
                ffi::CString::new(stack).unwrap().into_raw(),
                true as i32,
            );
        }
    }

    pub fn report_error(&mut self, error: &str) {
        self.report(true, 1, error, "", "");
    }

    pub fn report_warning(&mut self, warning: &str) {
        self.report(false, 1, warning, "", "");
    }

    /// Report a message at a throttled rate
    pub fn report_throttled(&mut self, is_error: bool, message: &str) {
        // If the FPGA timer breaks, don't throttle
        if self
            .report_throttler
            .update(RobotBase::fpga_time().unwrap_or(0))
        {
            self.report(is_error, 1, message, "", "");
        }
    }

    /// Get an axis on a joystick, in the range of [-1, 1].
    pub fn get_joystick_axis(&self, stick: usize, axis: usize) -> Result<f32, JoystickError> {
        if stick >= JOYSTICK_PORTS {
            // self.report_throttled(true, "Bad joystick");
            Err(JoystickError::JoystickDNE)
        } else if axis >= JOYSTICK_AXES {
            // self.report_throttled(true, "Bad joystick axis");
            Err(JoystickError::ChannelDNE)
        } else if axis >= self.joysticks.axes[stick].count as usize {
            // self.report_throttled(
            // true,
            // "Joystick axis missing, check if all controllers are plugged in",
            // );
            Err(JoystickError::ChannelUnplugged)
        } else {
            Ok(self.joysticks.axes[stick].axes[axis])
        }
    }

    /// Get the position of a POV switch, in degrees.
    pub fn get_joystick_pov(&self, stick: usize, pov: usize) -> Result<i16, JoystickError> {
        if stick >= JOYSTICK_POVS {
            // self.report_throttled(true, "Bad joystick");
            Err(JoystickError::JoystickDNE)
        } else if pov >= JOYSTICK_AXES {
            // self.report_throttled(true, "Bad joystick pov");
            Err(JoystickError::ChannelDNE)
        } else if pov >= self.joysticks.povs[stick].count as usize {
            // self.report_throttled(
            //     true,
            //     "Joystick pov missing, check if all controllers are plugged in",
            // );
            Err(JoystickError::ChannelUnplugged)
        } else {
            Ok(self.joysticks.povs[stick].povs[pov])
        }
    }

    /// Get the state of a button on a joystick.
    pub fn get_joystick_button(&self, stick: usize, button: usize) -> Result<bool, JoystickError> {
        if stick >= JOYSTICK_POVS {
            // self.report_throttled(true, "Bad joystick");
            Err(JoystickError::JoystickDNE)
        } else if button == 0 {
            // self.report_throttled(true, "Bad joystick button (button IDs start from 1)");
            Err(JoystickError::ChannelDNE)
        } else if button >= self.joysticks.povs[stick].count as usize {
            // self.report_throttled(
            //     true,
            //     "Joystick button missing, check if all controllers are plugged \
            //      in",
            // );
            Err(JoystickError::ChannelUnplugged)
        } else {
            let mask = 1 << (button - 1);
            Ok(self.joysticks.buttons[stick].buttons & mask != 0)
        }
    }

    /// Get the alliance the robot is on.
    #[allow(non_upper_case_globals)]
    pub fn get_alliance(&self) -> HalResult<Alliance> {
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

    /// Get the id for the station the driver station is at, as an integer.
    #[allow(non_upper_case_globals)]
    pub fn get_station(&self) -> HalResult<u32> {
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

    /// Wait for a new driver station packet.
    pub fn wait_for_data(&self) {
        let &(ref wait_lock, ref wait_cond) = &*self.condvar;
        let mut has_data = wait_lock.lock().unwrap();
        while !*has_data {
            has_data = wait_cond.wait(has_data).unwrap();
        }
    }

    /// Waits for a new driver station packet and returns true, or returns false if timeout is
    /// exceeded.
    pub fn wait_for_data_or_timeout(&self, timeout: time::Duration) -> bool {
        let &(ref wait_lock, ref wait_cond) = &*self.condvar;
        let mut has_data = wait_lock.lock().unwrap();

        while !*has_data {
            let result = wait_cond.wait_timeout(has_data, timeout).unwrap();
            if result.1.timed_out() {
                return false;
            } else {
                has_data = result.0;
            }
        }
        true
    }

    /// Does the robot have connection to the FMS?
    pub fn is_fms_attached(&self) -> bool {
        self.fms_attached
    }

    /// Does the robot have connection to the driver station?
    pub fn is_ds_attached(&self) -> bool {
        self.ds_attached
    }

    /// Get the state of the robot.
    pub fn get_state(&self) -> RobotState {
        self.state
    }
}

impl Drop for DriverStation {
    fn drop(&mut self) {
        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
    }
}
