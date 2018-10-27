// Copyright 2018 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use wpilib_sys::*;

#[derive(Debug, Copy, Clone)]
pub enum Port {
    OnboardCS0 = 0,
    OnboardCS1 = 1,
    OnboardCS2 = 2,
    OnboardCS3 = 3,
    MXP = 4,
}

/// Spi for driver writers. Currenltly does not include an accumulator.
pub struct Spi {
    port: HAL_SPIPort,
    msb_first: bool,
    sample_on_trailing: bool,
    clk_idle_high: bool,
}

impl Spi {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(port: Port) -> HalResult<Self> {
        hal_call!(HAL_InitializeSPI(port as HAL_SPIPort))?;
        report_usage(resource_type!(SPI), 1);
        Ok(Spi {
            port: port as HAL_SPIPort,
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

    pub fn write(&mut self, buf: &[u8]) -> i32 {
        unsafe { HAL_WriteSPI(self.port, buf.as_ptr(), buf.len() as i32) }
    }

    pub fn read(&mut self, initiate: bool, buf: &mut [u8]) -> i32 {
        if initiate {
            let send_data: Vec<u8> = vec![0; buf.len()];
            return unsafe {
                HAL_TransactionSPI(
                    self.port,
                    send_data.as_ptr(),
                    buf.as_mut_ptr(),
                    buf.len() as i32,
                )
            };
        }
        unsafe { HAL_ReadSPI(self.port, buf.as_mut_ptr(), buf.len() as i32) }
    }

    /// Caller must ensure that the slices size is greater than the size parameter
    pub unsafe fn transaction(&mut self, to_send: &[u8], to_receive: &mut [u8], size: i32) -> i32 {
        HAL_TransactionSPI(self.port, to_send.as_ptr(), to_receive.as_mut_ptr(), size)
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

    pub fn read_auto_recieved_data(&mut self, to_read: &mut [u8], timeout: f64) -> HalResult<i32> {
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
