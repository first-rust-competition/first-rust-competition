/*
This file is part of "first-rust-competition", which is free software: you can
redistribute it and/or modify it under the terms of the GNU General Public
License version 3 as published by the Free Software Foundation. See
<https://www.gnu.org/licenses/> for a copy.
*/

use hal::*;

// all of these enums use magic numbers from wpilibc SerialPort.h

#[derive(Debug, Copy, Clone)]
pub enum Port {
    Onboard = 0,
    MXP = 1,
    USB1 = 2,
    USB2 = 3,
}

#[derive(Debug, Copy, Clone)]
pub enum Parity {
    None = 0,
    Odd = 1,
    Even = 2,
    Mark = 3,
    Space = 4,
}

#[derive(Debug, Copy, Clone)]
pub enum StopBits {
    One = 10,
    OnePointFive = 15,
    Two = 20,
}

#[derive(Debug, Copy, Clone)]
pub enum WriteBufferMode {
    FlushOnAcces = 1,
    FlushWhenFull = 2,
}

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
        hal_call!(HAL_SetSerialParity(port as HAL_SerialPort, databits as i32))?;
        hal_call!(HAL_SetSerialStopBits(
            port as HAL_SerialPort,
            stopbits as i32
        ))?;

        let serial_port = SerialPort { port };
        serial_port.set_timeout(5.0);
        serial_port.set_write_buf_mode(WriteBufferMode::FlushOnAcces);
        serial_port.enable_termination('\n' as i8);
        report_usage(resource_type!(SerialPort), 0);
        Ok(serial_port)
    }

    pub fn set_flow_control(&mut self, flow_control: FlowControl) -> HalResult<()> {
        hal_call!(HAL_SetSerialFlowControl(
            self.port as HAL_SerialPort,
            flow_control as i32
        ))
    }

    pub fn enable_termination(&mut self, terminator: i8) -> HalResult<()> {
        hal_call!(HAL_EnableSerialTermination(
            self.port as HAL_SerialPort,
            terminator
        ))
    }

    pub fn disable_termination(&mut self) -> HalResult<()> {
        hal_call!(HAL_DisableSerialTermination(self.port as HAL_SerialPort))
    }

    pub fn get_bytes_received(&mut self) -> HalResult<u32> {
        hal_call!(HAL_GetSerialBytesReceived(self.port as HAL_SerialPort))
    }

    pub fn read(&mut self, buf: &mut [i8]) -> HalResult<()> {
        hal_call!(HAL_ReadSerial(
            self.port as HAL_SerialPort,
            buf.as_mut_ptr(),
            buf.len() as i32
        ))
    }

    pub fn write(&mut self, buf: &[i8]) -> HalResult<()> {
        hal_call!(HAL_WriteSerial(
            self.port as HAL_SerialPort,
            buf.as_ptr(),
            buf.len() as i32
        ))
    }

    pub fn set_timeout(&mut self, seconds: f64) -> HalResult<()> {
        hal_call!(HAL_DisableSerialTermination(
            self.port as HAL_SerialPort,
            seconds
        ))
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
