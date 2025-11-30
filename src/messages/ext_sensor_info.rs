use binrw::binrw;
use alloc::vec::Vec;

// ExtSensorInfo Block 4222
#[binrw]
#[derive(Debug, Clone)]
pub struct ExtSensorInfo {
    #[br(map = crate::do_not_use::map_u4)]
    #[bw(map = |x| crate::do_not_use::unmap_u4(x))]
    pub tow: Option<u32>,
    #[br(map = crate::do_not_use::map_u2)]
    #[bw(map = |x| crate::do_not_use::unmap_u2(x))]
    pub wnc: Option<u16>,
    pub source: u8,
    pub sensor_model: u8,
    #[br(parse_with = binrw::helpers::until_eof)]
    #[bw(write_with = crate::do_not_use::write_vec)]
    pub data: Vec<u8>,
}

impl ExtSensorInfo {
    // Source constants (same as ExtSensorStatus)
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
    
    // Sensor model constants (same as ExtSensorStatus)
    pub const MODEL_SBG_ELLIPSE: u8 = 2;
    pub const MODEL_SBG_ELLIPSE2: u8 = 5;
    pub const MODEL_VN100: u8 = 7;
    pub const MODEL_ADIS1650X: u8 = 10;
    pub const MODEL_ZERO_VELOCITY: u8 = 20;
    pub const MODEL_VELOCITY_INPUT: u8 = 21;
    
    // Data structure sizes
    pub const SBG_DATA_SIZE: usize = 52;
    pub const VN100_DATA_SIZE: usize = 36;
    pub const ADIS1650X_DATA_SIZE: usize = 44;
}