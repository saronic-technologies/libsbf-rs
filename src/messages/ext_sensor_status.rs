use binrw::binrw;
use alloc::vec::Vec;
use crate::do_not_use::{map_u2, map_u4, unmap_u2, unmap_u4, write_vec};

// ExtSensorStatus Block 4223
#[binrw]
#[derive(Debug, Clone)]
pub struct ExtSensorStatus {
    #[br(map = map_u4)]
    #[bw(map = unmap_u4)]
    pub tow: Option<u32>,
    #[br(map = map_u2)]
    #[bw(map = unmap_u2)]
    pub wnc: Option<u16>,
    pub source: u8,
    pub sensor_model: u8,
    #[br(parse_with = binrw::helpers::until_eof)]
    #[bw(write_with = write_vec)]
    pub data: Vec<u8>,
}

impl ExtSensorStatus {
    // Source constants
    pub const SOURCE_COM1: u8 = 0;
    pub const SOURCE_COM2: u8 = 1;
    pub const SOURCE_COM3: u8 = 2;
    pub const SOURCE_COM4: u8 = 3;
    pub const SOURCE_GPIO: u8 = 4;
    pub const SOURCE_USB1: u8 = 5;
    pub const SOURCE_USB2: u8 = 6;
    pub const SOURCE_IP10: u8 = 7;
    pub const SOURCE_IP11: u8 = 8;
    pub const SOURCE_IP12: u8 = 9;
    pub const SOURCE_IP13: u8 = 10;
    pub const SOURCE_IP14: u8 = 11;
    pub const SOURCE_IP15: u8 = 12;
    pub const SOURCE_IP16: u8 = 13;
    pub const SOURCE_IP17: u8 = 14;
    pub const SOURCE_IPS1: u8 = 15;
    pub const SOURCE_IPS2: u8 = 16;
    pub const SOURCE_IPS3: u8 = 17;
    pub const SOURCE_IPS4: u8 = 18;
    pub const SOURCE_IPS5: u8 = 19;
    pub const SOURCE_IPR1: u8 = 20;
    pub const SOURCE_IPR2: u8 = 21;
    pub const SOURCE_IPR3: u8 = 22;
    pub const SOURCE_IPR4: u8 = 23;
    pub const SOURCE_IPR5: u8 = 24;
    pub const SOURCE_INTERNAL_SPI: u8 = 32;
    
    // Sensor model constants
    pub const MODEL_SBG_ELLIPSE: u8 = 2;
    pub const MODEL_SBG_ELLIPSE2: u8 = 5;
    pub const MODEL_VN100: u8 = 7;
    pub const MODEL_ADIS1650X: u8 = 10;
    pub const MODEL_ZERO_VELOCITY: u8 = 20;
    pub const MODEL_VELOCITY_INPUT: u8 = 21;
    
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