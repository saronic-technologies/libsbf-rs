use alloc::vec::Vec;
use binrw::binrw;
use core::fmt;

/// Connection port / external sensor source.
///
/// Used by ExtSensorStatus, ExtSensorInfo, and VelSensorSetup to identify the
/// receiver port an external sensor is connected to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum ConnectionPort {
    #[default]
    Com1 = 0,
    Com2 = 1,
    Com3 = 2,
    Com4 = 3,
    Gpio = 4,
    Usb1 = 5,
    Usb2 = 6,
    Ip10 = 7,
    Ip11 = 8,
    Ip12 = 9,
    Ip13 = 10,
    Ip14 = 11,
    Ip15 = 12,
    Ip16 = 13,
    Ip17 = 14,
    Ips1 = 15,
    Ips2 = 16,
    Ips3 = 17,
    Ips4 = 18,
    Ips5 = 19,
    Ipr1 = 20,
    Ipr2 = 21,
    Ipr3 = 22,
    Ipr4 = 23,
    Ipr5 = 24,
    InternalSpi = 32,
    Unknown,
}

impl From<u8> for ConnectionPort {
    fn from(value: u8) -> Self {
        match value {
            0 => ConnectionPort::Com1,
            1 => ConnectionPort::Com2,
            2 => ConnectionPort::Com3,
            3 => ConnectionPort::Com4,
            4 => ConnectionPort::Gpio,
            5 => ConnectionPort::Usb1,
            6 => ConnectionPort::Usb2,
            7 => ConnectionPort::Ip10,
            8 => ConnectionPort::Ip11,
            9 => ConnectionPort::Ip12,
            10 => ConnectionPort::Ip13,
            11 => ConnectionPort::Ip14,
            12 => ConnectionPort::Ip15,
            13 => ConnectionPort::Ip16,
            14 => ConnectionPort::Ip17,
            15 => ConnectionPort::Ips1,
            16 => ConnectionPort::Ips2,
            17 => ConnectionPort::Ips3,
            18 => ConnectionPort::Ips4,
            19 => ConnectionPort::Ips5,
            20 => ConnectionPort::Ipr1,
            21 => ConnectionPort::Ipr2,
            22 => ConnectionPort::Ipr3,
            23 => ConnectionPort::Ipr4,
            24 => ConnectionPort::Ipr5,
            32 => ConnectionPort::InternalSpi,
            _ => ConnectionPort::Unknown,
        }
    }
}

impl fmt::Display for ConnectionPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionPort::Com1 => write!(f, "COM1"),
            ConnectionPort::Com2 => write!(f, "COM2"),
            ConnectionPort::Com3 => write!(f, "COM3"),
            ConnectionPort::Com4 => write!(f, "COM4"),
            ConnectionPort::Gpio => write!(f, "GPIO"),
            ConnectionPort::Usb1 => write!(f, "USB1"),
            ConnectionPort::Usb2 => write!(f, "USB2"),
            ConnectionPort::Ip10 => write!(f, "IP10"),
            ConnectionPort::Ip11 => write!(f, "IP11"),
            ConnectionPort::Ip12 => write!(f, "IP12"),
            ConnectionPort::Ip13 => write!(f, "IP13"),
            ConnectionPort::Ip14 => write!(f, "IP14"),
            ConnectionPort::Ip15 => write!(f, "IP15"),
            ConnectionPort::Ip16 => write!(f, "IP16"),
            ConnectionPort::Ip17 => write!(f, "IP17"),
            ConnectionPort::Ips1 => write!(f, "IPS1"),
            ConnectionPort::Ips2 => write!(f, "IPS2"),
            ConnectionPort::Ips3 => write!(f, "IPS3"),
            ConnectionPort::Ips4 => write!(f, "IPS4"),
            ConnectionPort::Ips5 => write!(f, "IPS5"),
            ConnectionPort::Ipr1 => write!(f, "IPR1"),
            ConnectionPort::Ipr2 => write!(f, "IPR2"),
            ConnectionPort::Ipr3 => write!(f, "IPR3"),
            ConnectionPort::Ipr4 => write!(f, "IPR4"),
            ConnectionPort::Ipr5 => write!(f, "IPR5"),
            ConnectionPort::InternalSpi => write!(f, "Internal SPI"),
            ConnectionPort::Unknown => write!(f, "Unknown"),
        }
    }
}

/// External sensor model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum ExtSensorModel {
    #[default]
    Unknown = 0,
    SbgEllipse = 2,
    SbgEllipse2 = 5,
    Vn100 = 7,
    Adis1650x = 10,
    ZeroVelocity = 20,
    VelocityInput = 21,
}

impl From<u8> for ExtSensorModel {
    fn from(value: u8) -> Self {
        match value {
            2 => ExtSensorModel::SbgEllipse,
            5 => ExtSensorModel::SbgEllipse2,
            7 => ExtSensorModel::Vn100,
            10 => ExtSensorModel::Adis1650x,
            20 => ExtSensorModel::ZeroVelocity,
            21 => ExtSensorModel::VelocityInput,
            _ => ExtSensorModel::Unknown,
        }
    }
}

impl fmt::Display for ExtSensorModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExtSensorModel::Unknown => write!(f, "Unknown"),
            ExtSensorModel::SbgEllipse => write!(f, "SBG Ellipse"),
            ExtSensorModel::SbgEllipse2 => write!(f, "SBG Ellipse 2"),
            ExtSensorModel::Vn100 => write!(f, "VN-100"),
            ExtSensorModel::Adis1650x => write!(f, "ADIS1650x"),
            ExtSensorModel::ZeroVelocity => write!(f, "Zero Velocity"),
            ExtSensorModel::VelocityInput => write!(f, "Velocity Input"),
        }
    }
}

// ExtSensorStatus Block 4223
#[binrw]
#[derive(Debug)]
pub struct ExtSensorStatus {
    #[br(map = |x: u32| if x == crate::DO_NOT_USE_U4 { None } else { Some(x) })]
    pub tow: Option<u32>,
    #[br(map = |x: u16| if x == crate::DO_NOT_USE_U2 { None } else { Some(x) })]
    pub wnc: Option<u16>,
    pub source: u8,
    pub sensor_model: u8,
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
