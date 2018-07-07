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

use super::ds::*;
use super::robot_base::*;
use hal::*;

const RUMBLE_BASE: i32 = 65535;

/// Enum for accessing elements of XBox controller by side
#[derive(PartialEq)]
pub enum JoystickSide {
    /// left side of joystick while held upright
    LeftHand,
    /// right side of joystick while held upright
    RightHand,
}

/// public trait that lays down base methods for joysticks
pub trait JoystickBase {
    /// get raw axis value from driverstation
    fn get_raw_axis(&self, axis: usize) -> Result<f32, JoystickError>;
    /// get raw button value from driverstation
    fn get_raw_button(&self, button: usize) -> Result<bool, JoystickError>;
    /// get raw pov value from driverstation
    fn get_pov(&self, pov: usize) -> Result<i16, JoystickError>;
    /// set joystick output through hal
    fn set_output(&mut self, output_number: i32, value: bool);
    /// set joystick outputs through hal
    fn set_outputs(&mut self, value: i64);
    /// set joystick rumble on either side by a percentage from 0-100 through hal
    fn set_rumble(&mut self, side: JoystickSide, value: f32);
}

/// stuct for almost any FRC legal joystick
pub struct Joystick {
    port: usize,
    ds: ThreadSafeDs,
    outputs: i64,
    left_rumble: i32,
    right_rumble: i32,
}

impl Joystick {
    /// user creates a Joystick object here
    pub fn new(rbase: &RobotBase, port: usize) -> Joystick {
        Self::new_raw_ds(rbase.get_ds_instance(), port)
    }

    pub fn new_raw_ds(ds: ThreadSafeDs, port: usize) -> Joystick {
        Joystick {
            port: port,
            ds,
            outputs: 0i64,
            left_rumble: 0i32,
            right_rumble: 0i32,
        }
    }
}

impl JoystickBase for Joystick {
    fn get_raw_axis(&self, axis: usize) -> Result<f32, JoystickError> {
        let read_lock = self.ds.read().map_err(|_| JoystickError::DsUnreachable)?;
        read_lock.get_joystick_axis(self.port, axis)
    }

    fn get_raw_button(&self, button: usize) -> Result<bool, JoystickError> {
        let read_lock = self.ds.read().map_err(|_| JoystickError::DsUnreachable)?;
        read_lock.get_joystick_button(self.port, button)
    }

    fn get_pov(&self, pov: usize) -> Result<i16, JoystickError> {
        let read_lock = self.ds.read().map_err(|_| JoystickError::DsUnreachable)?;
        read_lock.get_joystick_pov(self.port, pov)
    }

    fn set_output(&mut self, output_number: i32, value: bool) {
        let o = output_number - 1i32;
        self.outputs = (self.outputs & (!(1i32 << o)) as i64) | ((value as i64) << o);
        unsafe {
            HAL_SetJoystickOutputs(
                self.port as i32,
                self.outputs,
                self.left_rumble,
                self.right_rumble,
            );
        }
    }

    fn set_outputs(&mut self, value: i64) {
        self.outputs = value;
        unsafe {
            HAL_SetJoystickOutputs(
                self.port as i32,
                self.outputs,
                self.left_rumble,
                self.right_rumble,
            );
        }
    }

    fn set_rumble(&mut self, side: JoystickSide, mut value: f32) {
        value = if value > 1f32 {
            1f32
        } else if value < 0f32 {
            0f32
        } else {
            value
        };
        match side {
            JoystickSide::LeftHand => self.left_rumble = (value * RUMBLE_BASE as f32) as i32,
            JoystickSide::RightHand => self.right_rumble = (value * RUMBLE_BASE as f32) as i32,
        }
        unsafe {
            HAL_SetJoystickOutputs(
                self.port as i32,
                self.outputs,
                self.left_rumble,
                self.right_rumble,
            )
        };
    }
}
