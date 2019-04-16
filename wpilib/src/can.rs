use wpilib_sys::*;

pub const TEAM_MANUFACTURER: HAL_CANManufacturer::Type = HAL_CANManufacturer::HAL_CAN_Man_kTeamUse;
pub const TEAM_DEVICE_TYPE: HAL_CANDeviceType::Type = HAL_CANDeviceType::HAL_CAN_Dev_kMiscellaneous;

#[derive(Debug, Clone)]
pub struct CanData {
    data: [u8; 8],
    length: i32,
    timestamp: u64,
}

impl CanData {
    pub fn data(&self) -> &[u8] {
        &self.data[..self.length as usize]
    }

    pub fn raw_data(&self) -> &[u8; 8] {
        &self.data
    }

    pub fn into_raw_data(self) -> [u8; 8] {
        self.data
    }

    pub fn length(&self) -> i32 {
        self.length
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
}

/// A device on the CAN bus that conforms to the standard CAN spec
/// and uses the FRC CAN bus addressing.
#[derive(Debug)]
pub struct Can {
    handle: HAL_CANHandle,
}

impl Can {
    pub const TEAM_MANUFACTURER: HAL_CANManufacturer::Type = TEAM_MANUFACTURER;
    pub const TEAM_DEVICE_TYPE: HAL_CANDeviceType::Type = TEAM_DEVICE_TYPE;

    pub fn new(
        device_id: i32,
        device_manufacturer: HAL_CANManufacturer::Type,
        device_type: HAL_CANDeviceType::Type,
    ) -> HalResult<Self> {
        let handle = hal_call!(HAL_InitializeCAN(
            device_manufacturer,
            device_id,
            device_type,
        ))?;
        usage::report(usage::resource_types::CAN, device_id as _);
        Ok(Can { handle })
    }

    pub fn with_team_device(device_id: i32) -> HalResult<Self> {
        Can::new(device_id, TEAM_MANUFACTURER, TEAM_DEVICE_TYPE)
    }

    pub fn write_packet(&self, data: &[u8], api_id: i32) -> HalResult<()> {
        hal_call!(HAL_WriteCANPacket(
            self.handle,
            data.as_ptr(),
            data.len() as _,
            api_id,
        ))
    }

    pub fn write_packet_repeating(
        &mut self,
        data: &[u8],
        api_id: i32,
        repeat_ms: i32,
    ) -> HalResult<()> {
        hal_call!(HAL_WriteCANPacketRepeating(
            self.handle,
            data.as_ptr(),
            data.len() as _,
            api_id,
            repeat_ms,
        ))
    }

    pub fn stop_packet_repeating(&mut self, api_id: i32) -> HalResult<()> {
        hal_call!(HAL_StopCANPacketRepeating(self.handle, api_id))
    }

    pub fn read_packet_new(&self, api_id: i32) -> HalResult<Option<CanData>> {
        let mut data = [0; 8];
        let mut length = 0;
        let mut timestamp = 0;
        let status = hal_call!(HAL_ReadCANPacketNew(
            self.handle,
            api_id,
            data.as_mut_ptr(),
            &mut length,
            &mut timestamp,
        ));
        match status {
            Err(HalError(code)) if code == HAL_ERR_CANSessionMux_MessageNotFound => Ok(None),
            Err(error) => Err(error),
            Ok(()) => Ok(Some(CanData {
                data,
                length,
                timestamp,
            })),
        }
    }

    pub fn read_packet_latest(&self, api_id: i32) -> HalResult<Option<CanData>> {
        let mut data = [0; 8];
        let mut length = 0;
        let mut timestamp = 0;
        let status = hal_call!(HAL_ReadCANPacketLatest(
            self.handle,
            api_id,
            data.as_mut_ptr(),
            &mut length,
            &mut timestamp,
        ));
        match status {
            Err(HalError(code)) if code == HAL_ERR_CANSessionMux_MessageNotFound => Ok(None),
            Err(error) => Err(error),
            Ok(()) => Ok(Some(CanData {
                data,
                length,
                timestamp,
            })),
        }
    }

    pub fn read_packet_timeout(&self, api_id: i32, timeout_ms: i32) -> HalResult<Option<CanData>> {
        let mut data = [0; 8];
        let mut length = 0;
        let mut timestamp = 0;
        let status = hal_call!(HAL_ReadCANPacketTimeout(
            self.handle,
            api_id,
            data.as_mut_ptr(),
            &mut length,
            &mut timestamp,
            timeout_ms,
        ));
        match status {
            Err(HalError(code))
                if code == HAL_CAN_TIMEOUT || code == HAL_ERR_CANSessionMux_MessageNotFound =>
            {
                Ok(None)
            }
            Err(error) => Err(error),
            Ok(()) => Ok(Some(CanData {
                data,
                length,
                timestamp,
            })),
        }
    }

    pub fn read_periodic_packet(
        &self,
        api_id: i32,
        timeout_ms: i32,
        period_ms: i32,
    ) -> HalResult<Option<CanData>> {
        let mut data = [0; 8];
        let mut length = 0;
        let mut timestamp = 0;
        let status = hal_call!(HAL_ReadCANPeriodicPacket(
            self.handle,
            api_id,
            data.as_mut_ptr(),
            &mut length,
            &mut timestamp,
            timeout_ms,
            period_ms,
        ));
        match status {
            Err(HalError(code))
                if code == HAL_CAN_TIMEOUT || code == HAL_ERR_CANSessionMux_MessageNotFound =>
            {
                Ok(None)
            }
            Err(error) => Err(error),
            Ok(()) => Ok(Some(CanData {
                data,
                length,
                timestamp,
            })),
        }
    }
}

impl Drop for Can {
    fn drop(&mut self) {
        unsafe { HAL_CleanCAN(self.handle) }
    }
}
