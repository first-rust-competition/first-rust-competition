// Copyright 2018 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os::raw::c_char;
use wpilib_sys::*;

// all of these enums use magic numbers from wpilibc SerialPort.h

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Port {
    Onboard = 0,
    MXP = 1,
    USB1 = 2,
    USB2 = 3,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Parity {
    None = 0,
    Odd = 1,
    Even = 2,
    Mark = 3,
    Space = 4,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum StopBits {
    One = 10,
    OnePointFive = 15,
    Two = 20,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum WriteBufferMode {
    FlushOnAcces = 1,
    FlushWhenFull = 2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum FlowControl {
    None = 0,
    XonXoff = 1,
    RtsCts = 2,
    DtrDsr = 4,
}

#[derive(Debug)]
pub struct SerialPort {
    port: Port,
}

impl SerialPort {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        baud_rate: u32,
        port: Port,
        databits: u32,
        parity: Parity,
        stopbits: StopBits,
    ) -> HalResult<Self> {
        hal_call!(HAL_InitializeSerialPort(port as HAL_SerialPort::Type))?;

        hal_call!(HAL_SetSerialBaudRate(
            port as HAL_SerialPort::Type,
            baud_rate as i32
        ))?;
        hal_call!(HAL_SetSerialDataBits(
            port as HAL_SerialPort::Type,
            databits as i32
        ))?;
        hal_call!(HAL_SetSerialParity(
            port as HAL_SerialPort::Type,
            parity as i32
        ))?;
        hal_call!(HAL_SetSerialStopBits(
            port as HAL_SerialPort::Type,
            stopbits as i32
        ))?;

        let mut serial_port = SerialPort { port };
        serial_port.set_timeout(5.0)?;
        serial_port.set_write_buf_mode(WriteBufferMode::FlushOnAcces)?;

        serial_port.enable_termination(b'\n')?;
        report_usage(resource_type!(SerialPort), 0);
        Ok(serial_port)
    }

    pub fn set_flow_control(&mut self, flow_control: FlowControl) -> HalResult<()> {
        hal_call!(HAL_SetSerialFlowControl(
            self.port as HAL_SerialPort::Type,
            flow_control as i32
        ))
    }

    #[allow(clippy::unnecessary_cast)]
    pub fn enable_termination(&mut self, terminator: u8) -> HalResult<()> {
        hal_call!(HAL_EnableSerialTermination(
            self.port as HAL_SerialPort::Type,
            terminator as c_char
        ))
    }

    pub fn disable_termination(&mut self) -> HalResult<()> {
        hal_call!(HAL_DisableSerialTermination(
            self.port as HAL_SerialPort::Type
        ))
    }

    pub fn bytes_received(&mut self) -> HalResult<i32> {
        hal_call!(HAL_GetSerialBytesReceived(
            self.port as HAL_SerialPort::Type
        ))
    }

    #[allow(clippy::unnecessary_cast)]
    pub fn read(&mut self, buf: &mut [u8]) -> HalResult<i32> {
        hal_call!(HAL_ReadSerial(
            self.port as HAL_SerialPort::Type,
            buf.as_mut_ptr() as *mut c_char,
            buf.len() as i32
        ))
    }

    #[allow(clippy::unnecessary_cast)]
    pub fn read_len(&mut self, buf: &mut [u8], len: usize) -> HalResult<i32> {
        let len = len.min(buf.len());
        hal_call!(HAL_ReadSerial(
            self.port as HAL_SerialPort::Type,
            buf.as_mut_ptr() as *mut c_char,
            len as i32
        ))
    }

    /// # Returns
    /// Then number of bytes actually written to the buffer.
    pub fn write(&mut self, buf: &[u8]) -> HalResult<i32> {
        hal_call!(HAL_WriteSerial(
            self.port as HAL_SerialPort::Type,
            buf.as_ptr() as *const c_char,
            buf.len() as i32
        ))
    }

    pub fn set_timeout(&mut self, seconds: f64) -> HalResult<()> {
        hal_call!(HAL_SetSerialTimeout(
            self.port as HAL_SerialPort::Type,
            seconds
        ))
    }

    pub fn set_read_buf_size(&mut self, size: u32) -> HalResult<()> {
        hal_call!(HAL_SetSerialReadBufferSize(
            self.port as HAL_SerialPort::Type,
            size as i32
        ))
    }

    pub fn set_write_buf_size(&mut self, size: u32) -> HalResult<()> {
        hal_call!(HAL_SetSerialWriteBufferSize(
            self.port as HAL_SerialPort::Type,
            size as i32
        ))
    }

    pub fn set_write_buf_mode(&mut self, mode: WriteBufferMode) -> HalResult<()> {
        hal_call!(HAL_SetSerialWriteMode(
            self.port as HAL_SerialPort::Type,
            mode as i32
        ))
    }

    pub fn flush(&mut self) -> HalResult<()> {
        hal_call!(HAL_FlushSerial(self.port as HAL_SerialPort::Type))
    }

    pub fn reset(&mut self) -> HalResult<()> {
        hal_call!(HAL_ClearSerial(self.port as HAL_SerialPort::Type))
    }
}

use std::io;
/// Delegates to the [`SerialPort::write`] method and is affected in the same way by automatic terminators, buffering, and other behavior.
impl io::Write for SerialPort {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        hal_to_io_len(self.write(buf))
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        hal_to_io(self.flush())
    }
}

/// Delegates to the [`SerialPort::read`] method and is affected in the same way by buffering and other behavior.
impl io::Read for SerialPort {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        hal_to_io_len(self.read(buf))
    }
}

#[inline]
fn hal_to_io_len(r: HalResult<i32>) -> io::Result<usize> {
    match r {
        Ok(x) => Ok(x as usize),
        Err(t) => Err(io::Error::new(io::ErrorKind::Other, t)),
    }
}

#[inline]
fn hal_to_io<T>(r: HalResult<T>) -> io::Result<T> {
    match r {
        Ok(x) => Ok(x),
        Err(t) => Err(io::Error::new(io::ErrorKind::Other, t)),
    }
}

impl Drop for SerialPort {
    fn drop(&mut self) {
        hal_call!(HAL_CloseSerial(self.port as HAL_SerialPort::Type)).ok();
    }
}
