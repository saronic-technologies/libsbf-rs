use super::common::{ConnectionPort, ExtSensorModel};
use crate::util::binrw::{map_u1_enum, map_u2, map_u4, unmap_u1_enum, unmap_u2, unmap_u4};
use alloc::vec::Vec;
use binrw::binrw;
use serde::Serialize;

// ExtSensorStatus Block 4223
#[binrw]
#[derive(Clone, Debug, Serialize)]
pub struct ExtSensorStatus {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    #[br(map = map_u1_enum)]
    #[bw(map = unmap_u1_enum)]
    pub source: ConnectionPort,
    #[br(map = map_u1_enum)]
    #[bw(map = unmap_u1_enum)]
    pub sensor_model: ExtSensorModel,
    #[br(parse_with = binrw::helpers::until_eof)]
    pub data: Vec<u8>,
}

impl ExtSensorStatus {
    // ADIS1650x status flags (byte 0)
    pub const ADIS_ERROR: u8 = 0x01;
    pub const ADIS_SENSOR_FAILURE: u8 = 0x02;
    pub const ADIS_CLOCK_ERROR: u8 = 0x04;
    pub const ADIS_CRC_ERROR: u8 = 0x08;
    pub const ADIS_NO_DATA_WRONG_STATE: u8 = 0x10;
    pub const ADIS_TIMESTAMP_ERROR: u8 = 0x20;
    pub const ADIS_DATA_GAP: u8 = 0x40;
    pub const ADIS_UNEXPECTED_SAMPLES: u8 = 0x80;

    // ADIS1650x info type (byte 1)
    pub const ADIS_INFO_NONE: u8 = 0;
    pub const ADIS_INFO_DIAG_STAT: u8 = 1;
    pub const ADIS_INFO_CRC_FAIL_COUNT: u8 = 2;
    pub const ADIS_INFO_NO_DATA_SECONDS: u8 = 3;
    pub const ADIS_INFO_INTERNAL_STATE: u8 = 4;
    pub const ADIS_INFO_SAMPLES_LAST_PPS: u8 = 5;
    pub const ADIS_INFO_DRIFTING_SAMPLES: u8 = 6;
    pub const ADIS_INFO_CLOCK_ERROR_SAMPLES: u8 = 7;
}
