// Copyright 2018 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use wpilib_sys::*;

// This is kind of a hack so that IDEs don't bug devs.
// bindgen rightfully generates a serial interface that uses std::os::raw::c_char
// but that's a crappy user interface. The problem is that wheter its u8 or i8
// depends on the platform.
// Of course, this could change under us at the mere flip of a compiler option.
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
#[allow(non_camel_case_types)]
type byte = u8;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[allow(non_camel_case_types)]
type byte = i8;

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
        hal_call!(HAL_InitializeSerialPort(port as HAL_SerialPort))?;

        hal_call!(HAL_SetSerialBaudRate(
            port as HAL_SerialPort,
            baud_rate as i32
        ))?;
        hal_call!(HAL_SetSerialDataBits(
            port as HAL_SerialPort,
            databits as i32
        ))?;
        hal_call!(HAL_SetSerialParity(port as HAL_SerialPort, parity as i32))?;
        hal_call!(HAL_SetSerialStopBits(
            port as HAL_SerialPort,
            stopbits as i32
        ))?;

        let mut serial_port = SerialPort { port };
        serial_port.set_timeout(5.0)?;
        serial_port.set_write_buf_mode(WriteBufferMode::FlushOnAcces)?;

        #[allow(clippy::unnecessary_cast)]
        // silence clippy when casting to byte is already u8
        serial_port.enable_termination(b'\n' as byte)?;
        report_usage(resource_type!(SerialPort), 0);
        Ok(serial_port)
    }

    pub fn set_flow_control(&mut self, flow_control: FlowControl) -> HalResult<()> {
        hal_call!(HAL_SetSerialFlowControl(
            self.port as HAL_SerialPort,
            flow_control as i32
        ))
    }

    pub fn enable_termination(&mut self, terminator: byte) -> HalResult<()> {
        hal_call!(HAL_EnableSerialTermination(
            self.port as HAL_SerialPort,
            terminator
        ))
    }

    pub fn disable_termination(&mut self) -> HalResult<()> {
        hal_call!(HAL_DisableSerialTermination(self.port as HAL_SerialPort))
    }

    pub fn get_bytes_received(&mut self) -> HalResult<i32> {
        hal_call!(HAL_GetSerialBytesReceived(self.port as HAL_SerialPort))
    }

    pub fn read(&mut self, buf: &mut [byte]) -> HalResult<i32> {
        hal_call!(HAL_ReadSerial(
            self.port as HAL_SerialPort,
            buf.as_mut_ptr(),
            buf.len() as i32
        ))
    }

    pub fn read_len(&mut self, buf: &mut [byte], len: usize) -> HalResult<i32> {
        let len = ::std::cmp::max(len, buf.len());
        hal_call!(HAL_ReadSerial(
            self.port as HAL_SerialPort,
            buf.as_mut_ptr(),
            len as i32
        ))
    }

    /// # Returns
    /// Then number of bytes actually written to the buffer.
    pub fn write(&mut self, buf: &[byte]) -> HalResult<i32> {
        hal_call!(HAL_WriteSerial(
            self.port as HAL_SerialPort,
            buf.as_ptr(),
            buf.len() as i32
        ))
    }

    pub fn set_timeout(&mut self, seconds: f64) -> HalResult<()> {
        hal_call!(HAL_SetSerialTimeout(self.port as HAL_SerialPort, seconds))
    }

    pub fn set_read_buf_size(&mut self, size: u32) -> HalResult<()> {
        hal_call!(HAL_SetSerialReadBufferSize(
            self.port as HAL_SerialPort,
            size as i32
        ))
    }

    pub fn set_write_buf_size(&mut self, size: u32) -> HalResult<()> {
        hal_call!(HAL_SetSerialWriteBufferSize(
            self.port as HAL_SerialPort,
            size as i32
        ))
    }

    pub fn set_write_buf_mode(&mut self, mode: WriteBufferMode) -> HalResult<()> {
        hal_call!(HAL_SetSerialWriteMode(
            self.port as HAL_SerialPort,
            mode as i32
        ))
    }

    pub fn flush(&mut self) -> HalResult<()> {
        hal_call!(HAL_FlushSerial(self.port as HAL_SerialPort))
    }

    pub fn reset(&mut self) -> HalResult<()> {
        hal_call!(HAL_ClearSerial(self.port as HAL_SerialPort))
    }
}

impl Drop for SerialPort {
    fn drop(&mut self) {
        hal_call!(HAL_CloseSerial(self.port as HAL_SerialPort)).ok();
    }
}
