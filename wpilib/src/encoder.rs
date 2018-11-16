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

// TODO: documentation

// TODO: fix variables
use wpilib_sys::*;

use crate::dio::DigitalInput;
use std::ptr;

/// The indexing type for an encoder
#[derive(Debug, Copy, Clone)]
pub enum IndexingType {
    ResetWhileHigh,
    ResetWhileLow,
    ResetOnFallingEdge,
    ResetOnRisingEdge,
}

impl IndexingType {
    #[allow(dead_code)]
    pub(crate) fn into_hal(self) -> HAL_EncoderIndexingType {
        use self::IndexingType::*;
        match self {
            ResetWhileHigh => HAL_EncoderIndexingType_HAL_kResetWhileHigh,
            ResetWhileLow => HAL_EncoderIndexingType_HAL_kResetWhileLow,
            ResetOnFallingEdge => HAL_EncoderIndexingType_HAL_kResetOnFallingEdge,
            ResetOnRisingEdge => HAL_EncoderIndexingType_HAL_kResetOnRisingEdge,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum EncodingType {
    K1X,
    K2X,
    K4X,
}

impl EncodingType {
    pub(crate) fn into_hal(self) -> HAL_EncoderEncodingType {
        use self::EncodingType::*;
        match self {
            K1X => HAL_EncoderEncodingType_HAL_Encoder_k1X,
            K2X => HAL_EncoderEncodingType_HAL_Encoder_k2X,
            K4X => HAL_EncoderEncodingType_HAL_Encoder_k4X,
        }
    }
}

/// An encoder.
///
/// Uses quadrature on two separate channels to read the distance and direction travelled by a
/// shaft. All integration is done by the FPGA.
pub struct Encoder {
    _source_a: DigitalInput,
    _source_b: DigitalInput,
    _source_index: Option<DigitalInput>,
    encoder: HAL_EncoderHandle,
}

impl Encoder {
    /// Create a new encoder given two channels and an encoding type, returning an error if
    /// initialization fails.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(channel_a: i32, channel_b: i32, encoding: EncodingType) -> HalResult<Encoder> {
        let source_a = DigitalInput::new(channel_a)?;
        let source_b = DigitalInput::new(channel_b)?;

        let handle = hal_call!(HAL_InitializeEncoder(
            source_a.handle(),
            0i32,
            source_b.handle(),
            0i32,
            false as i32,
            encoding.into_hal()
        ))?;
        let encoder = Encoder {
            _source_a: source_a,
            _source_b: source_b,
            _source_index: None,
            encoder: handle,
        };

        unsafe {
            report_usage_extras(
                resource_type!(Encoder),
                encoder.fpga_index()? as u32,
                encoding.into_hal(),
                ptr::null(),
            );
        }

        Ok(encoder)
    }

    /// Get the FPGA index of this encoder.
    pub fn fpga_index(&self) -> HalResult<i32> {
        hal_call!(HAL_GetEncoderFPGAIndex(self.encoder))
    }

    /// Get the current value read by this encoder, with any scaling factors applied.
    pub fn get(&self) -> HalResult<i32> {
        hal_call!(HAL_GetEncoder(self.encoder))
    }

    /// Get the raw value of this encoder, without any scaling factors.
    pub fn get_raw(&self) -> HalResult<i32> {
        hal_call!(HAL_GetEncoderRaw(self.encoder))
    }

    /// Get the current scaling factor for this encoder.
    pub fn encoding_scale(&self) -> HalResult<i32> {
        hal_call!(HAL_GetEncoderEncodingScale(self.encoder))
    }

    /// Get the current (estimated) speed this encoder is travelling at.
    pub fn rate(&self) -> HalResult<f64> {
        hal_call!(HAL_GetEncoderRate(self.encoder))
    }

    /// Set the minimum rate that this encoder must be moving at to be considered "moving".
    pub fn set_min_rate(&mut self, min_rate: f64) -> HalResult<()> {
        hal_call!(HAL_SetEncoderMinRate(self.encoder, min_rate))
    }

    /// Reset the count of this encoder.
    pub fn reset(&mut self) -> HalResult<()> {
        hal_call!(HAL_ResetEncoder(self.encoder))
    }
}

impl Drop for Encoder {
    fn drop(&mut self) {
        hal_call!(HAL_FreeEncoder(self.encoder)).ok();
        // .ok() because the status variable is unused in HAL_FreeEncoder
    }
}