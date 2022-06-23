//! Access to the RoboRIO's I2C port.

use std::cmp;
use std::io;
use wpilib_sys::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Port {
    Onboard = HAL_I2CPort::HAL_I2C_kOnboard,
    MXP = HAL_I2CPort::HAL_I2C_kMXP,
}

pub struct I2C {
    port: Port,
    device_address: u16,
}

impl I2C {
    /// Constructs a new I2C
    ///
    /// `port` is the I2C port to which the device is connected, and `device_address` is the address of the device on the bus
    pub fn new(port: Port, device_address: u16) -> HalResult<Self> {
        hal_call!(HAL_InitializeI2C(port as HAL_I2CPort::Type))?;
        usage::report(usage::resource_types::I2C, 0);
        Ok(I2C {
            port,
            device_address,
        })
    }

    /// Generic transaction.
    ///
    /// This is a lower-level interface to the I2C hardware giving you more control
    /// over each transaction.
    ///
    /// This function will send all the bytes in `data_to_send` and will read data into `data_received`. Callers should make sure these buffers are sized appropriately
    ///
    /// Returns a result based on whether the transaction was successful
    pub fn transaction(&self, data_to_send: &[u8], data_received: &mut [u8]) -> io::Result<usize> {
        let status = unsafe {
            HAL_TransactionI2C(
                self.port as HAL_I2CPort::Type,
                i32::from(self.device_address),
                data_to_send.as_ptr(),
                data_to_send.len() as i32,
                data_received.as_mut_ptr(),
                data_received.len() as i32,
            )
        };

        io_result(status)
    }

    /// Attempt to address a device on the I2C bus.
    ///
    /// This allows you to figure out if there is a device on the I2C bus that
    /// responds to the address specified in the constructor.
    ///
    /// Returns a result based on whether the transaction was successful
    pub fn address_only(&self) -> io::Result<usize> {
        self.transaction(&[], &mut [])
    }

    /// Execute a write transaction with the device.
    ///
    /// Write a single byte to a register on a device and wait until the
    ///   transaction is complete.
    ///
    /// Returns a result based on whether the transaction was successful
    pub fn write(&self, register_address: u8, data: u8) -> io::Result<usize> {
        let buf = [register_address, data];

        let status = unsafe {
            HAL_WriteI2C(
                self.port as HAL_I2CPort::Type,
                i32::from(self.device_address),
                buf.as_ptr(),
                buf.len() as i32,
            )
        };

        io_result(status)
    }

    /// Execute a bulk write transaction with the device.
    ///
    /// Write multiple bytes to a device and wait until the
    ///   transaction is complete.
    ///
    /// Returns a result based on whether the transaction was successful
    pub fn write_bulk(&self, data: &[u8]) -> io::Result<usize> {
        let status = unsafe {
            HAL_WriteI2C(
                self.port as HAL_I2CPort::Type,
                i32::from(self.device_address),
                data.as_ptr(),
                data.len() as i32,
            )
        };

        io_result(status)
    }

    /// Execute a read transaction with the device.
    ///
    /// Read bytes from a device.
    /// Most I2C devices will auto-increment the register pointer internally
    /// allowing you to read consecutive registers on a device in a single
    /// transaction.
    ///
    /// Returns a result based on whether the transaction was successful
    pub fn read(&self, register_address: i32, buf: &mut [u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Write buffer length < 1",
            ));
        }

        self.transaction(&[register_address as u8], buf)
    }

    /// Execute a read only transaction with the device.
    ///
    /// Read bytes from a device. This method does not write any data to prompt the
    /// device.
    ///
    /// Returns a result based on whether the transaction was successful
    pub fn read_only(&self, buf: &mut [u8]) -> io::Result<usize> {
        let status = unsafe {
            HAL_ReadI2C(
                self.port as HAL_I2CPort::Type,
                i32::from(self.device_address),
                buf.as_mut_ptr(),
                buf.len() as i32,
            )
        };

        io_result(status)
    }

    /// Verify that a device's registers contain expected values.
    ///
    /// Most devices will have a set of registers that contain a known value that
    /// can be used to identify them.  This allows an I2C device driver to easily
    /// verify that the device contains the expected value.
    ///
    /// The device must support and be configured to use register
    /// auto-increment.
    pub fn verify_sensor(&self, register_address: i32, expected: &[u8]) -> bool {
        // (register_address..).step_by(4) gets truncated to the length of the first iter when we zip
        for (i, cur_register_address) in (0..expected.len())
            .step_by(4)
            .zip((register_address..).step_by(4))
        {
            let to_read = cmp::min(4, expected.len() - i);

            let mut buf = vec![0; to_read];

            if self.read(cur_register_address, &mut buf[..]).is_err() {
                return false;
            }

            for j in 0..to_read {
                if buf[j] != expected[i + j] {
                    return false;
                }
            }
        }

        true
    }
}

impl Drop for I2C {
    fn drop(&mut self) {
        unsafe {
            HAL_CloseI2C(self.port as HAL_I2CPort::Type);
        }
    }
}

fn io_result(rv: i32) -> io::Result<usize> {
    if rv < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(rv as usize)
    }
}
