// Copyright 2019 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Contains objects for sending and receiving frames from a device on the CAN bus.

use wpilib_sys::*;

pub const TEAM_MANUFACTURER: HAL_CANManufacturer::Type = HAL_CANManufacturer::HAL_CAN_Man_kTeamUse;
pub const TEAM_DEVICE_TYPE: HAL_CANDeviceType::Type = HAL_CANDeviceType::HAL_CAN_Dev_kMiscellaneous;

#[derive(Debug, Clone)]
/// The inner return type of the various `Can` read methods.
pub struct CanData {
    data: [u8; 8],
    length: i32,
    timestamp: u64,
}

impl CanData {
    /// A slice of the actual data read.
    pub fn data(&self) -> &[u8] {
        &self.data[..self.length as usize]
    }

    /// A slice of the entire data array.
    pub fn raw_data(&self) -> &[u8; 8] {
        &self.data
    }

    /// Returns the underlying data array, taking ownership.
    pub fn into_raw_data(self) -> [u8; 8] {
        self.data
    }

    /// Returns the actual number of bytes read.
    pub fn length(&self) -> i32 {
        self.length
    }

    /// A timestamp of when the packet was received, based off `CLOCK_MONOTONIC`.
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
}

/// A device on the CAN bus that conforms to the standard CAN spec
/// and uses the FRC CAN bus addressing.
///
/// No packets that can be sent gets blocked by the roboRIO,
/// so all methods work identically in all robot modes.
#[derive(Debug)]
pub struct Can {
    handle: HAL_CANHandle,
}

impl Can {
    pub const TEAM_MANUFACTURER: HAL_CANManufacturer::Type = TEAM_MANUFACTURER;
    pub const TEAM_DEVICE_TYPE: HAL_CANDeviceType::Type = TEAM_DEVICE_TYPE;

    /// Create an interface to a CAN device with the specified device ID,
    /// manufacturer, and device type.
    ///
    /// The device ID is 6 bits, the manufacturer is 8 bits, and the device type is 5 bits.
    pub fn new(
        device_id: u8,
        device_manufacturer: HAL_CANManufacturer::Type,
        device_type: HAL_CANDeviceType::Type,
    ) -> HalResult<Self> {
        let handle = hal_call!(HAL_InitializeCAN(
            device_manufacturer,
            device_id.into(),
            device_type,
        ))?;
        usage::report(usage::resource_types::CAN, device_id.into());
        Ok(Can { handle })
    }

    /// Create an interface to a CAN device with the specified device ID.
    /// This uses the team manufacturer and device types.
    ///
    /// The device ID is 6 bits (0-63).
    pub fn with_team_device(device_id: u8) -> HalResult<Self> {
        Can::new(device_id, TEAM_MANUFACTURER, TEAM_DEVICE_TYPE)
    }

    /// Write a packet to the CAN device with a specific ID.
    /// This ID is 10 bits.
    pub fn write_packet(&self, data: &[u8], api_id: i32) -> HalResult<()> {
        hal_call!(HAL_WriteCANPacket(
            self.handle,
            data.as_ptr(),
            data.len() as _,
            api_id,
        ))
    }

    /// Write a repeating packet to the CAN device with a specific ID.
    /// This ID is 10 bits.
    ///
    /// The roboRIO will automatically repeat the packet at the specified interval.
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

    /// Stop a repeating packet with a specific ID.
    /// This ID is 10 bits.
    pub fn stop_packet_repeating(&mut self, api_id: i32) -> HalResult<()> {
        hal_call!(HAL_StopCANPacketRepeating(self.handle, api_id))
    }

    /// Read a new CAN packet.
    ///
    /// This will only return `Some` once per packet received.
    ///
    /// Any calls made before receiving another packet will return `None`.
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

    /// Read a CAN packet.
    ///
    /// This will always return the last packet received, without accounting for packet age.
    ///
    /// Returns `None` if no packet has ever been received with the given ID.
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

    /// Read a CAN packet.
    ///
    /// This will return the last packet received
    /// until the packet is older than the requested timeout.
    ///
    /// Returns `None` if no packet has been received within the timeout period.
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

    /// Read a CAN packet.
    ///
    /// This will return the last packet received
    /// until the packet is older than the requested timeout.
    /// Returns `None` if no packet has been received within the timeout period.
    ///
    /// Packets are cached until the specified period has passed.
    /// This is useful when you know the packet is sent at a regular interval.
    /// Any intermediary calls will not result in a read from FRC NetComm.
    /// It is not recommended to use this unless you understand the implications.
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
