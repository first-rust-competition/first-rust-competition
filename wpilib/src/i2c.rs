use wpilib_sys::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Port {
    Onboard = HAL_I2CPort::HAL_I2C_kOnboard,
    MXP = HAL_I2CPort::HAL_I2C_kMXP,
}

pub struct I2C {
    port: Port,
    device_address: i32,
}

impl I2C {
    pub fn new(port: Port, device_address: i32) -> HalResult<Self> {
        hal_call!(HAL_InitializeI2C(port as HAL_I2CPort::Type))?;
        usage::report(usage::resource_types::I2C, 0);
        Ok(I2C {
            port,
            device_address,
        })
    }

    pub fn transaction(&self, data_to_send: &[u8], data_received: &mut [u8]) -> HalResult<()> {
        let status = unsafe {
            HAL_TransactionI2C(
                self.port as HAL_I2CPort::Type,
                self.device_address,
                data_to_send.as_ptr(),
                data_to_send.len() as i32,
                data_received.as_mut_ptr(),
                data_received.len() as i32,
            )
        };

        if status < 0 {
            Ok(())
        } else {
            Err(HalError::from(status))
        }
    }

    pub fn address_only(&self) -> HalResult<()> {
        self.transaction(&[], &mut [])
    }

    pub fn write(&self, register_address: i32, data: u8) -> HalResult<()> {
        let mut buf = [0u8; 2];
        buf[0] = register_address as u8; // TODO: Is this valid?
        buf[1] = data;

        let status = unsafe {
            HAL_WriteI2C(
                self.port as HAL_I2CPort::Type,
                self.device_address,
                buf.as_ptr(),
                buf.len() as i32,
            )
        };

        if status < 0 {
            Ok(())
        } else {
            Err(HalError::from(status))
        }
    }

    pub fn write_bulk(&self, data: Vec<u8>) -> HalResult<()> {
        let status = unsafe {
            HAL_WriteI2C(
                self.port as HAL_I2CPort::Type,
                self.device_address,
                data.as_ptr(),
                data.len() as i32,
            )
        };

        if status < 0 {
            Ok(())
        } else {
            Err(HalError::from(status))
        }
    }

    pub fn read(&self, register_address: i32, buf: &mut [u8]) -> HalResult<()> {
        if buf.is_empty() {
            return Ok(());
        }

        self.transaction(&[register_address as u8], buf)
    }

    pub fn read_only(&self, buf: &mut [u8]) -> HalResult<()> {
        let status = unsafe {
            HAL_ReadI2C(
                self.port as HAL_I2CPort::Type,
                self.device_address,
                buf.as_mut_ptr(),
                buf.len() as i32,
            )
        };

        if status < 0 {
            Ok(())
        } else {
            Err(HalError::from(status))
        }
    }

    pub fn verify_sensor(&self, register_address: i32, expected: &[u8]) -> bool {
        let mut i = 0;
        let mut cur_register_address = register_address;

        loop {
            if i >= expected.len() {
                break;
            }

            let to_read = if expected.len() - i < 4 {
                expected.len() - i
            } else {
                4
            };

            let mut buf = vec![0; to_read];

            if self.read(cur_register_address, &mut buf[..]).is_err() {
                return false;
            }

            for j in 0..to_read {
                if buf[j] != expected[i + j] {
                    return false;
                }
            }

            i += 4;
            cur_register_address += 4;
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
