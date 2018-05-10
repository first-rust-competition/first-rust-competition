// MIT License

// Copyright (c) 2017 Rust for Robotics Developers

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#![allow(dead_code)] // for now
use super::hal::*;

/// The number of DIOs on the RoboRIO.
pub fn num_digital_channels() -> i32 {
    unsafe { HAL_GetNumDigitalChannels() }
}

/// The number of analog inputs on the RoboRIO.
pub fn num_analog_inputs() -> i32 {
    unsafe { HAL_GetNumAnalogInputs() }
}

/// The number of solenoid channels per PCM.
pub fn num_solenoid_channels() -> i32 {
    unsafe { HAL_GetNumSolenoidChannels() }
}

/// The number of PCMs.
pub fn num_solenoid_modules() -> i32 {
    unsafe { HAL_GetNumPCMModules() }
}

/// The number of hardware PWM channels on the RoboRIO.
pub fn num_pwm_channels() -> i32 {
    unsafe { HAL_GetNumPWMChannels() }
}

/// The number of relay headers on the RoboRIO.
pub fn num_relay_headers() -> i32 {
    unsafe { HAL_GetNumRelayHeaders() }
}

/// Check if a solenoid module is valid.
pub fn check_solenoid_module(module: i32) -> bool {
    unsafe { HAL_CheckSolenoidModule(module) != 0 }
}

/// Check if a digital channel is valid.
pub fn check_digital_channel(channel: i32) -> bool {
    unsafe { HAL_CheckDIOChannel(channel) != 0 }
}

/// Check if a relay channel is valid.
pub fn check_relay_channel(channel: i32) -> bool {
    unsafe { HAL_CheckRelayChannel(channel) != 0 }
}

/// Check if a PWM channel is valid.
pub fn check_pwm_channel(channel: i32) -> bool {
    unsafe { HAL_CheckPWMChannel(channel) != 0 }
}

/// Check if an analog input channel is valid.
pub fn check_analog_input_channel(channel: i32) -> bool {
    unsafe { HAL_CheckAnalogInputChannel(channel) != 0 }
}

/// Check if an analog output channel is valid.
pub fn check_analog_output_channel(channel: i32) -> bool {
    unsafe { HAL_CheckAnalogOutputChannel(channel) != 0 }
}

/// Check if a solenoid channel is valid.
pub fn check_solenoid_channel(channel: i32) -> bool {
    unsafe { HAL_CheckSolenoidChannel(channel) != 0 }
}

/// Check if a PDP channel is valid.
pub fn check_pdp_channel(channel: i32) -> bool {
    unsafe { HAL_CheckPDPModule(channel) != 0 }
}
