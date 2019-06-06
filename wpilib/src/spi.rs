// Copyright 2018 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;
use wpilib_sys::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Port {
    OnboardCS0 = HAL_SPIPort::HAL_SPI_kOnboardCS0,
    OnboardCS1 = HAL_SPIPort::HAL_SPI_kOnboardCS1,
    OnboardCS2 = HAL_SPIPort::HAL_SPI_kOnboardCS2,
    OnboardCS3 = HAL_SPIPort::HAL_SPI_kOnboardCS3,
    MXP = HAL_SPIPort::HAL_SPI_kMXP,
}

/// Spi for driver writers. Currenltly does not include an accumulator.
#[derive(Debug)]
pub struct Spi {
    port: HAL_SPIPort::Type,
    msb_first: bool,
    sample_on_trailing: bool,
    clk_idle_high: bool,
}

impl Spi {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(port: Port) -> HalResult<Self> {
        hal_call!(HAL_InitializeSPI(port as HAL_SPIPort::Type))?;
        usage::report(usage::resource_types::SPI, 1);
        Ok(Spi {
            port: port as HAL_SPIPort::Type,
            msb_first: false,
            sample_on_trailing: false,
            clk_idle_high: false,
        })
    }

    pub fn set_clock_rate(&mut self, hz: f64) {
        unsafe {
            HAL_SetSPISpeed(self.port, hz as i32); // all of my what but its honestly what they do
        }
    }

    #[inline]
    fn update_spi_opts(&mut self) {
        unsafe {
            HAL_SetSPIOpts(
                self.port,
                if self.msb_first { 1 } else { 0 },
                if self.sample_on_trailing { 1 } else { 0 },
                if self.clk_idle_high { 1 } else { 0 },
            );
        }
    }

    pub fn set_msb_first(&mut self) {
        self.msb_first = true;
        self.update_spi_opts();
    }

    pub fn set_lsb_first(&mut self) {
        self.msb_first = false;
        self.update_spi_opts();
    }

    pub fn set_sample_data_on_leading_edge(&mut self) {
        self.sample_on_trailing = false;
        self.update_spi_opts();
    }

    pub fn set_sample_data_on_trailing_edge(&mut self) {
        self.sample_on_trailing = true;
        self.update_spi_opts();
    }

    pub fn set_clock_active_low(&mut self) {
        self.clk_idle_high = true;
        self.update_spi_opts();
    }

    pub fn set_clock_active_high(&mut self) {
        self.clk_idle_high = false;
        self.update_spi_opts();
    }

    pub fn set_chip_select_active_high(&mut self) -> HalResult<()> {
        hal_call!(HAL_SetSPIChipSelectActiveHigh(self.port))
    }

    pub fn set_chip_select_active_low(&mut self) -> HalResult<()> {
        hal_call!(HAL_SetSPIChipSelectActiveLow(self.port))
    }

    pub fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        io_result(unsafe { HAL_WriteSPI(self.port, data.as_ptr(), data.len() as _) })
    }

    pub fn read(&mut self, initiate: bool, buf: &mut [u8]) -> io::Result<usize> {
        if initiate {
            let send_data: Vec<u8> = vec![0; buf.len()];
            return unsafe { self.transaction_into(&send_data, buf.as_mut_ptr()) };
        }

        io_result(unsafe { HAL_ReadSPI(self.port, buf.as_mut_ptr(), buf.len() as _) })
    }

    /// Performs an SPI send/receive transaction.
    pub fn transaction(&mut self, to_send: &[u8]) -> io::Result<Vec<u8>> {
        let size = to_send.len();
        let mut receive_buf = Vec::with_capacity(size);
        let read = unsafe { self.transaction_into(to_send, receive_buf.as_mut_ptr()) }?;
        unsafe { receive_buf.set_len(read) }
        Ok(receive_buf)
    }

    pub unsafe fn transaction_into(
        &mut self,
        to_send: &[u8],
        receive_buf: *mut u8,
    ) -> io::Result<usize> {
        let size = to_send.len();
        io_result(HAL_TransactionSPI(
            self.port,
            to_send.as_ptr(),
            receive_buf,
            size as _,
        ))
    }

    pub fn init_auto(&mut self, buffer_size: i32) -> HalResult<()> {
        hal_call!(HAL_InitSPIAuto(self.port, buffer_size))
    }

    pub fn free_auto(&mut self) -> HalResult<()> {
        hal_call!(HAL_FreeSPIAuto(self.port))
    }

    pub fn set_auto_transmit_data(&mut self, to_send: &[u8], zero_size: i32) -> HalResult<()> {
        hal_call!(HAL_SetSPIAutoTransmitData(
            self.port,
            to_send.as_ptr(),
            to_send.len() as i32,
            zero_size
        ))
    }

    pub fn start_auto_rate(&mut self, period: f64) -> HalResult<()> {
        hal_call!(HAL_StartSPIAutoRate(self.port, period))
    }

    pub fn stop_auto(&mut self) -> HalResult<()> {
        hal_call!(HAL_StopSPIAuto(self.port))
    }

    pub fn force_auto_read(&mut self) -> HalResult<()> {
        hal_call!(HAL_ForceSPIAutoRead(self.port))
    }

    pub fn read_auto_received_data(&mut self, to_read: &mut [u32], timeout: f64) -> HalResult<i32> {
        hal_call!(HAL_ReadSPIAutoReceivedData(
            self.port,
            to_read.as_mut_ptr(),
            to_read.len() as i32,
            timeout
        ))
    }

    pub fn auto_dropped_count(&mut self) -> HalResult<i32> {
        hal_call!(HAL_GetSPIAutoDroppedCount(self.port))
    }
}

impl Drop for Spi {
    fn drop(&mut self) {
        unsafe { HAL_CloseSPI(self.port) }
    }
}

/// Convert the return value of HAL SPI read/write/transaction to an io::Result.
fn io_result(rv: i32) -> io::Result<usize> {
    if rv < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(rv as usize)
    }
}
